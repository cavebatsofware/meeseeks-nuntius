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

use std::sync::LazyLock;
use serde::{Deserialize, Serialize};
use sled::{Db};

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

static DATABASE: LazyLock<Db> = LazyLock::new(|| {
    sled::open("app.sled").expect("Failed to open database")
});

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
            format!("{}:{}", prefix, id.to_string())
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
                    eprintln!("Warning: ID mismatch - key: {}, stored: {}", key, stored_id);
                }
            }
            
            Ok(Some(entity))
        } else {
            Ok(None)
        }
    }

    // Update entity - maintains ID consistency
    pub fn update_entity<T: Entity>(
        &self,
        entity: &T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(id) = entity.id() {
            let json = serde_json::to_vec(entity)?;
            self.db.insert(id, json)?;
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

    println!("\n=== Direct Sled Test ===");
    
    // List ALL keys
    println!("All keys in database:");
    for item in self.db.iter() {
        if let Ok((key, _)) = item {
            println!("  Key: {:?} => String: {:?}", 
                key.as_ref(), 
                String::from_utf8_lossy(&key)
            );
        }
    }
    
    // Test scan_prefix
    println!("\nScanning for prefix 'party':");
    let mut count = 0;
    for item in self.db.scan_prefix(b"party") {
        if let Ok((key, value)) = item {
            count += 1;
            println!("  Found: {:?} => {:?}", 
                String::from_utf8_lossy(&key),
                String::from_utf8_lossy(&value)
            );
        }
    }
    println!("Total found with prefix 'party': {}", count);

        let mut results = Vec::new();
        println!("db length: {}", self.db.len());
        let prefix_bytes: &[u8] = prefix.as_bytes();
        self.db.scan_prefix(prefix_bytes).for_each(|row| {
            let (key, value) = row.unwrap();
            let key_str = String::from_utf8(key.to_vec()).unwrap();
            let entity: T = serde_json::from_slice(&value).unwrap();
            if let Some(stored_id) = entity.id() {
                if stored_id != key_str {
                    eprintln!("Warning: ID mismatch - key: {}, stored: {}", key_str, stored_id);
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
            Ok(result)
        } else {
            Err("No value found for provided key".into())
        }
    }
}
