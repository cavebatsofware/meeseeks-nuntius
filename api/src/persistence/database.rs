
pub mod sled {
    use serde::{Deserialize, Serialize};
    use sled::{Db, Result as SledResult};

    pub struct Database {
        db: Db,
    }

    impl Database {
        pub fn new(path: &str) -> sled::Result<Self> {
            Ok(Database {
                db: sled::open(path)?,
            })
        }

        // Generic save function for any serializable type
        pub fn save<T: Serialize>(&self, key: &str, value: &T) -> Result<(), Box<dyn std::error::Error>> {
            let json = serde_json::to_vec(value)?;
            self.db.insert(key, json)?;
            Ok(())
        }

        // Generic load function for any deserializable type
        pub fn load<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<Option<T>, Box<dyn std::error::Error>> {
            if let Some(bytes) = self.db.get(key)? {
                let value: T = serde_json::from_slice(&bytes)?;
                Ok(Some(value))
            } else {
                Ok(None)
            }
        }

        // Delete a key
        pub fn delete(&self, key: &str) -> SledResult<()> {
            self.db.remove(key)?;
            Ok(())
        }

        // List all keys with a prefix
        pub fn list_keys_with_prefix(&self, prefix: &str) -> Vec<String> {
            self.db
                .scan_prefix(prefix)
                .filter_map(|result| {
                    if let Ok((key, _)) = result {
                        String::from_utf8(key.to_vec()).ok()
                    } else {
                        None
                    }
                })
                .collect()
        }
    }   
}
