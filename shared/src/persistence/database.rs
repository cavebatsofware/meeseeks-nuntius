/*  This file is part of a secure messaging project codename meeseeks-nuntius
 *  Copyright (C) 2025  Grant DeFayette
 *
 *  meeseeks-nuntius is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  meeseeks-nuntius is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with meeseeks-nuntius.  If not, see <https://www.gnu.org/licenses/>.
 */

use serde::{Deserialize, Serialize};
use sled::Db;
use std::path::PathBuf;
use std::sync::LazyLock;

// Entity trait
// provides key prefix for database operations
pub trait Entity: Serialize + for<'de> Deserialize<'de> {
    fn id(&self) -> Option<&str>;
    fn set_id(&mut self, id: String);
    fn key_prefix() -> &'static str;
}

pub struct Database {
    db: &'static Db,
}

fn get_database_path() -> PathBuf {
    #[cfg(target_os = "ios")]
    {
        // Use iOS Application Support directory for persistent storage
        if let Some(home) = std::env::var_os("HOME") {
            let app_support = PathBuf::from(home)
                .join("Library")
                .join("Application Support");
            // Create directory if it doesn't exist
            let _ = std::fs::create_dir_all(&app_support);
            app_support.join("app.sled")
        } else {
            PathBuf::from("app.sled")
        }
    }

    #[cfg(target_os = "android")]
    {
        // Use Android's internal app files directory
        let app_files_dir = PathBuf::from("/data/data/com.cavebatsofware/files");

        // Create directory if it doesn't exist
        let _ = std::fs::create_dir_all(&app_files_dir);

        app_files_dir.join("app.sled")
    }

    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        PathBuf::from("app.sled")
    }
}

static DATABASE: LazyLock<Db> = LazyLock::new(|| {
    let path = get_database_path();
    sled::open(path).expect("Failed to open database")
});

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}

impl Database {
    pub fn new() -> Self {
        Database { db: &DATABASE }
    }

    pub fn clear(&self) -> std::result::Result<(), sled::Error> {
        self.db.clear()
    }

    fn generate_unique_key(&self, prefix: &str) -> Result<String, Box<dyn std::error::Error>> {
        let id = self.db.generate_id()?;

        let key = if prefix.is_empty() {
            id.to_string()
        } else {
            format!("{prefix}:{id}")
        };

        Ok(key)
    }

    pub fn flush(&self) -> Result<usize, sled::Error> {
        self.db.flush()
    }

    // Save entity - ensures ID consistency
    pub fn save_entity<T: Entity + Clone>(
        &self,
        entity: &mut T,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let key = if let Some(existing_id) = entity.id() {
            existing_id.to_string()
        } else {
            let new_id = self.generate_unique_key(T::key_prefix())?;
            entity.set_id(new_id.clone());
            new_id
        };

        let json = serde_json::to_vec(&entity)?;
        self.db.insert(&key, json)?;

        // Force flush to disk for mobile persistence
        self.db.flush()?;

        Ok(key)
    }

    // Load entity - validates ID consistency
    pub fn load_entity<T: Entity>(
        &self,
        key: &str,
    ) -> Result<Option<T>, Box<dyn std::error::Error>> {
        if let Some(bytes) = self.db.get(key)? {
            let entity: T = serde_json::from_slice(&bytes)?;

            // Optional: Validate that stored ID matches the key
            if let Some(stored_id) = entity.id() {
                if stored_id != key {
                    eprintln!("Warning: ID mismatch - key: {key}, stored: {stored_id}");
                }
            }

            Ok(Some(entity))
        } else {
            Ok(None)
        }
    }

    // Update entity - maintains ID consistency
    pub fn update_entity<T: Entity>(&self, entity: &T) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(id) = entity.id() {
            let json = serde_json::to_vec(entity)?;
            self.db.insert(id, json)?;

            // Force flush to disk for mobile persistence
            self.db.flush()?;

            Ok(())
        } else {
            Err("Cannot update entity without ID".into())
        }
    }

    // Load all entities with validation
    pub fn load_all_entities<T: Entity>(
        &self,
        prefix: &str,
    ) -> Result<Vec<T>, Box<dyn std::error::Error>> {
        let mut results = Vec::new();
        let prefix_bytes: &[u8] = prefix.as_bytes();
        self.db.scan_prefix(prefix_bytes).for_each(|row| {
            let (key, value) = row.unwrap();
            let key_str = String::from_utf8(key.to_vec()).unwrap();
            let entity: T = serde_json::from_slice(&value).unwrap();
            if let Some(stored_id) = entity.id() {
                if stored_id != key_str {
                    eprintln!("Warning: ID mismatch - key: {key_str}, stored: {stored_id}");
                }
            }

            results.push(entity);
        });

        Ok(results)
    }

    pub fn delete<T: Entity>(&self, key: &str) -> Result<T, Box<dyn std::error::Error>> {
        let raw = self.db.remove(key)?;
        if raw.is_some() {
            let result: T = serde_json::from_slice(&raw.unwrap())?;

            // Force flush to disk for mobile persistence
            self.db.flush()?;

            Ok(result)
        } else {
            Err("No value found for provided key".into())
        }
    }

    /// Find entity by a field value using a predicate function
    pub fn find_entity<T: Entity, F>(
        &self,
        prefix: &str,
        predicate: F,
    ) -> Result<Option<T>, Box<dyn std::error::Error>>
    where
        F: Fn(&T) -> bool,
    {
        let prefix_bytes: &[u8] = prefix.as_bytes();
        let mut result = None;

        self.db
            .scan_prefix(prefix_bytes)
            .try_for_each(|row| -> Result<(), Box<dyn std::error::Error>> {
                let (_, value) = row?;
                let entity: T = serde_json::from_slice(&value)?;

                if predicate(&entity) {
                    result = Some(entity);
                    return Err("found".into()); // Early termination
                }
                Ok(())
            })
            .ok(); // Ignore the "found" error

        Ok(result)
    }

    /// Find all entities matching a predicate
    pub fn find_entities<T: Entity, F>(
        &self,
        prefix: &str,
        predicate: F,
    ) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        F: Fn(&T) -> bool,
    {
        let mut results = Vec::new();
        let prefix_bytes: &[u8] = prefix.as_bytes();

        self.db.scan_prefix(prefix_bytes).try_for_each(
            |row| -> Result<(), Box<dyn std::error::Error>> {
                let (_, value) = row?;
                let entity: T = serde_json::from_slice(&value)?;

                if predicate(&entity) {
                    results.push(entity);
                }
                Ok(())
            },
        )?;

        Ok(results)
    }
}
