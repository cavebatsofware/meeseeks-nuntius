#[cfg(test)]
mod tests {
    use crate::persistence::database::sled::Database;
    use crate::persistence::models::local::*;
    use std::collections::HashSet;
    use tempfile::TempDir;

    fn create_test_party_dto() -> PartyDTO {
        let mut known_contacts = HashSet::new();
        known_contacts.insert([1u8; 32]);
        known_contacts.insert([2u8; 32]);
        known_contacts.insert([3u8; 32]);

        PartyDTO::new(
            "Alice",
            [42u8; 32], // secret key
            [84u8; 32], // public key
            known_contacts,
        )
    }

    #[test]
    fn test_save_and_load_party_dto() -> Result<(), Box<dyn std::error::Error>> {
        // Create a temporary directory for the test database
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test_db");
        let db = Database::new(db_path.to_str().unwrap())?;

        // Create test data
        let party_dto = create_test_party_dto();
        let key = "party:alice";

        // Save the PartyDTO
        db.save(key, &party_dto)?;

        // Load it back
        let loaded_party_dto: Option<PartyDTO> = db.load(key)?;

        // Verify it was loaded correctly
        assert!(loaded_party_dto.is_some());
        let loaded = loaded_party_dto.unwrap();

        assert_eq!(loaded.name, party_dto.name);
        assert_eq!(loaded.secret_key, party_dto.secret_key);
        assert_eq!(loaded.public_key, party_dto.public_key);
        assert_eq!(loaded.known_contacts, party_dto.known_contacts);

        Ok(())
    }

    #[test]
    fn test_multiple_parties() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test_db");
        let db = Database::new(db_path.to_str().unwrap())?;

        // Create multiple parties with consistent key lengths
        let parties = vec![
            ("alice", 
             [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 
             [10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10]),
            ("bob", 
             [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2], 
             [20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20]),
            ("charlie", 
             [3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3], 
             [30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30]),
        ];

        // Save all parties
        for (name, secret_key, public_key) in &parties {
            let mut contacts = HashSet::new();
            contacts.insert([99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99]); // Common contact
            contacts.insert([100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100]); // Another contact
            
            let party_dto = PartyDTO::new(name, *secret_key, *public_key, contacts);
            db.save(&format!("party:{}", name), &party_dto)?;
        }

        // List all party keys
        let party_keys = db.list_keys_with_prefix("party:");
        assert_eq!(party_keys.len(), 3);

        // Verify we can load each one
        for (name, secret_key, public_key) in parties {
            let loaded: Option<PartyDTO> = db.load(&format!("party:{}", name))?;
            assert!(loaded.is_some());
            
            let party = loaded.unwrap();
            assert_eq!(party.name, name);
            assert_eq!(party.secret_key, secret_key);
            assert_eq!(party.public_key, public_key);
            assert!(party.known_contacts.contains(&[99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99]));
            assert!(party.known_contacts.contains(&[100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100]));
            assert_eq!(party.known_contacts.len(), 2);
        }

        Ok(())
    }

    #[test]
    fn test_update_party() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test_db");
        let db = Database::new(db_path.to_str().unwrap())?;

        let key = "party:alice";
        let mut party_dto = create_test_party_dto();

        // Save initial version
        db.save(key, &party_dto)?;

        // Update the party (add a new contact)
        party_dto.known_contacts.insert([99u8; 32]);
        db.save(key, &party_dto)?;

        // Load and verify the update
        let loaded: Option<PartyDTO> = db.load(key)?;
        assert!(loaded.is_some());
        
        let loaded_party = loaded.unwrap();
        assert_eq!(loaded_party.known_contacts.len(), 4); // Original 3 + 1 new
        assert!(loaded_party.known_contacts.contains(&[99u8; 32]));

        Ok(())
    }

    #[test]
    fn test_delete_party() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test_db");
        let db = Database::new(db_path.to_str().unwrap())?;

        let key = "party:alice";
        let party_dto = create_test_party_dto();

        // Save the party
        db.save(key, &party_dto)?;

        // Verify it exists
        let loaded: Option<PartyDTO> = db.load(key)?;
        assert!(loaded.is_some());

        // Delete it
        db.delete(key)?;

        // Verify it's gone
        let loaded_after_delete: Option<PartyDTO> = db.load(key)?;
        assert!(loaded_after_delete.is_none());

        Ok(())
    }

    #[test]
    fn test_load_nonexistent_party() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test_db");
        let db = Database::new(db_path.to_str().unwrap())?;

        // Try to load a party that doesn't exist
        let loaded: Option<PartyDTO> = db.load("party:nonexistent")?;
        assert!(loaded.is_none());

        Ok(())
    }

    #[test]
    fn test_party_dto_conversion() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test_db");
        let db = Database::new(db_path.to_str().unwrap())?;

        let party_dto = create_test_party_dto();
        
        // Test round-trip conversion: DTO -> Party -> DTO
        let party = party_dto.to_party();
        assert_eq!(party.name, party_dto.name);
        assert_eq!(party.secret_key_bytes(), party_dto.secret_key);
        assert_eq!(party.public_key_bytes(), party_dto.public_key);
        assert_eq!(party.get_contacts_bytes(), party_dto.known_contacts);
        let converted_back = PartyDTO::from_party(&party);

        assert_eq!(party_dto.name, converted_back.name);
        assert_eq!(party_dto.secret_key, converted_back.secret_key);
        assert_eq!(party_dto.public_key, converted_back.public_key);
        assert_eq!(party_dto.known_contacts, converted_back.known_contacts);

        // Test saving the converted DTO
        db.save("party:converted", &converted_back)?;
        let loaded: Option<PartyDTO> = db.load("party:converted")?;
        assert!(loaded.is_some());

        Ok(())
    }
}
