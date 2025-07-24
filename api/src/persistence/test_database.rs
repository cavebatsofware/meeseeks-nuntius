#[cfg(test)]
mod tests {
    use crate::crypto::message::exchange::Party;
    use crate::persistence::database::sled::Database;
    use std::collections::HashSet;
    use crypto_box::PublicKey;
    use tempfile::TempDir;

    fn create_test_party() -> Party {
        let known_contacts = [
            &PublicKey::from_bytes(
                [11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11]
            ),
            &PublicKey::from_bytes(
                [22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22]
            ),
            &PublicKey::from_bytes(
                [33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33]
            )
        ];
        Party::new_with_contacts(
            "Alice",
            &known_contacts,
        )
    }

    #[test]
    fn test_save_and_load_party() -> Result<(), Box<dyn std::error::Error>> {
        // Create a temporary directory for the test database
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test_db");
        let db = Database::new(db_path.to_str().unwrap())?;

        // Create test data
        let party = create_test_party();
        let key = "party:alice";

        // Save the Party
        db.save(key, &party)?;

        // Load it back
        let loaded_party: Option<Party> = db.load(key)?;

        // Verify it was loaded correctly
        assert!(loaded_party.is_some());
        let loaded = loaded_party.unwrap();

        assert_eq!(loaded.name, party.name);
        assert_eq!(loaded.secret_key_bytes(), party.secret_key_bytes());
        assert_eq!(loaded.public_key_bytes(), party.public_key_bytes());
        assert_eq!(loaded.get_contacts_bytes(), party.get_contacts_bytes());

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
            
            let party_dto = Party::from_values(name, *secret_key, *public_key, contacts);
            db.save(&format!("party:{}", name), &party_dto)?;
        }

        // List all party keys
        let party_keys = db.list_keys_with_prefix("party:");
        assert_eq!(party_keys.len(), 3);

        // Verify we can load each one
        for (name, secret_key, public_key) in parties {
            let loaded: Option<Party> = db.load(&format!("party:{}", name))?;
            assert!(loaded.is_some());
            
            let party = loaded.unwrap();
            assert_eq!(party.name, name);
            assert_eq!(party.secret_key_bytes(), secret_key);
            assert_eq!(party.public_key_bytes(), public_key);
            assert!(party.get_contacts_bytes().contains(&[99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99]));
            assert!(party.get_contacts_bytes().contains(&[100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100]));
            assert_eq!(party.get_contacts().len(), 2);
        }

        Ok(())
    }

    #[test]
    fn test_update_party() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test_db");
        let db = Database::new(db_path.to_str().unwrap())?;

        let key = "party:alice";
        let mut party = create_test_party();

        // Save initial version
        db.save(key, &party)?;

        // Update the party (add a new contact)
        party.add_contact(
            &PublicKey::from_bytes(
                [100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100]
            )
        );
        db.save(key, &party)?;

        // Load and verify the update
        let loaded: Option<Party> = db.load(key)?;
        assert!(loaded.is_some());
        
        let loaded_party = loaded.unwrap();
        assert_eq!(loaded_party.get_contacts().len(), 4); // Original 3 + 1 new
        assert!(loaded_party.get_contacts().contains(
            &PublicKey::from_bytes(
                [100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100])
            )
        );

        Ok(())
    }

    #[test]
    fn test_delete_party() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test_db");
        let db = Database::new(db_path.to_str().unwrap())?;

        let key = "party:alice";
        let party_dto = create_test_party();

        // Save the party
        db.save(key, &party_dto)?;

        // Verify it exists
        let loaded: Option<Party> = db.load(key)?;
        assert!(loaded.is_some());

        // Delete it
        db.delete(key)?;

        // Verify it's gone
        let loaded_after_delete: Option<Party> = db.load(key)?;
        assert!(loaded_after_delete.is_none());

        Ok(())
    }

    #[test]
    fn test_load_nonexistent_party() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test_db");
        let db = Database::new(db_path.to_str().unwrap())?;

        // Try to load a party that doesn't exist
        let loaded: Option<Party> = db.load("party:nonexistent")?;
        assert!(loaded.is_none());

        Ok(())
    }

    #[test]
    fn test_party_dto_conversion() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test_db");
        let db = Database::new(db_path.to_str().unwrap())?;

        let original_party = create_test_party();
        
        // Test round-trip conversion: Party -> JSON -> Party
        let party = Party::from_json(
            original_party.to_json().unwrap().as_str()
        ).unwrap();
        assert_eq!(party.name, original_party.name);
        assert_eq!(party.secret_key_bytes(), original_party.secret_key_bytes());
        assert_eq!(party.public_key_bytes(), original_party.public_key_bytes());
        assert_eq!(party.get_contacts_bytes(), original_party.get_contacts_bytes());

        // Test saving the converted DTO
        db.save("party:converted", &party)?;
        let loaded: Option<Party> = db.load("party:converted")?;
        assert!(loaded.is_some());

        Ok(())
    }
}
