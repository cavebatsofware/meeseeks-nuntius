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

#[cfg(test)]
mod tests {
    use crate::crypto::message::{Party, EncryptedMessage};
    use crate::persistence::database::{Database, Entity};
    use std::collections::HashSet;
    use crypto_box::PublicKey;
    use serial_test::serial;

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
    #[serial(local_db)]
    fn test_save_and_load_party() -> Result<(), Box<dyn std::error::Error>> {
        let db = Database::new();
        let _ = db.clear();

        // Create test data
        let mut party = create_test_party();

        // Save the Party
        let key = db.save_entity(&mut party)?;

        // Load it back
        let loaded_party: Option<Party> = db.load_entity(&key)?;

        // Verify it was loaded correctly
        assert!(loaded_party.is_some());
        let loaded = loaded_party.unwrap();

        assert_eq!(loaded.name, party.name);
        assert_eq!(loaded.secret_key_bytes(), party.secret_key_bytes());
        assert_eq!(loaded.public_key_bytes(), party.public_key_bytes());
        assert_eq!(loaded.known_contacts_bytes(), party.known_contacts_bytes());

        Ok(())
    }

    #[test]
    #[serial(local_db)]
    fn test_multiple_parties() -> Result<(), Box<dyn std::error::Error>> {
        let db = Database::new();
        let _ = db.clear();

        // Create multiple parties with consistent key lengths
        let parties = vec![
            ("party:alice",
             "alice", 
             [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 
             [10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10]),
            ("party:bob",
             "bob",
             [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2], 
             [20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20]),
            ("party:charlie",
             "charlie", 
             [3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3], 
             [30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30]),
        ];

        let mut party_keys = Vec::new();
        // Save all parties
        for (id, name, secret_key, public_key) in &parties {
            let mut contacts = HashSet::new();
            contacts.insert([99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99]); // Common contact
            contacts.insert([100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100]); // Another contact

            let mut party = Party::from_values(Some(id.to_string()), name, *secret_key, *public_key, contacts);
            let id = db.save_entity(&mut party)?;
            party_keys.push(id);
        }
        let f= db.flush().unwrap();
        println!("flush: {}", f);

        // List all party keys
        let party_keys: Vec<Party> = db.load_all_entities(Party::key_prefix()).unwrap();
        assert_eq!(party_keys.len(), 3);

        // Verify we can load each one
        for (id, name, secret_key, public_key) in parties {
            let loaded: Option<Party> = db.load_entity(id)?;
            assert!(loaded.is_some());
            
            let party = loaded.unwrap();
            assert_eq!(party.name, name);
            assert_eq!(party.secret_key_bytes(), secret_key);
            assert_eq!(party.public_key_bytes(), public_key);
            assert!(party.known_contacts_bytes().contains(&[99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99]));
            assert!(party.known_contacts_bytes().contains(&[100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100]));
            assert_eq!(party.known_contacts().len(), 2);
        }

        Ok(())
    }

    #[test]
    #[serial(local_db)]
    fn test_update_party() -> Result<(), Box<dyn std::error::Error>> {
        let db = Database::new();
        let _ = db.clear();
        let mut party = create_test_party();

        // Save initial version
        db.save_entity(&mut party)?;

        // Update the party (add a new contact)
        party.add_contact(
            &PublicKey::from_bytes(
                [100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100]
            )
        );
        let key = db.save_entity(&mut party)?;

        // Load and verify the update
        let loaded: Option<Party> = db.load_entity(&key)?;
        assert!(loaded.is_some());
        
        let loaded_party = loaded.unwrap();
        assert_eq!(loaded_party.known_contacts().len(), 4); // Original 3 + 1 new
        assert!(loaded_party.known_contacts().contains(
            &PublicKey::from_bytes(
                [100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100])
            )
        );

        Ok(())
    }

    #[test]
    #[serial(local_db)]
    fn test_delete_party() -> Result<(), Box<dyn std::error::Error>> {
        let db = Database::new();
        let _ = db.clear();
        let mut party = create_test_party();

        // Save the party
        let key = db.save_entity(&mut party)?;

        // Verify it exists
        let loaded: Option<Party> = db.load_entity(&key)?;
        assert!(loaded.is_some());

        // Delete it
        let deleted: Party = db.delete(&key)?;
        assert!(deleted.name == "Alice");

        // Verify it's gone
        let loaded_after_delete: Option<Party> = db.load_entity(&key)?;
        assert!(loaded_after_delete.is_none());

        Ok(())
    }

    #[test]
    #[serial(local_db)]
    fn test_load_nonexistent_party() -> Result<(), Box<dyn std::error::Error>> {
        let db = Database::new();
        let _ = db.clear();

        // Try to load a party that doesn't exist
        let loaded: Option<Party> = db.load_entity("party:nonexistent")?;
        assert!(loaded.is_none());

        Ok(())
    }

    #[test]
    #[serial(local_db)]
    fn test_party_dto_conversion() -> Result<(), Box<dyn std::error::Error>> {
        let db = Database::new();
        let _ = db.clear();

        let original_party = create_test_party();
        
        // Test round-trip conversion: Party -> JSON -> Party
        let mut party = Party::from_json(
            original_party.to_json().unwrap().as_str()
        ).unwrap();
        assert_eq!(party.name, original_party.name);
        assert_eq!(party.secret_key_bytes(), original_party.secret_key_bytes());
        assert_eq!(party.public_key_bytes(), original_party.public_key_bytes());
        assert_eq!(party.known_contacts_bytes(), original_party.known_contacts_bytes());

        // Test saving the converted DTO
        let id = db.save_entity(&mut party)?;
        let loaded: Option<Party> = db.load_entity(&id)?;
        assert!(loaded.is_some());

        Ok(())
    }

    #[test]
    #[serial(local_db)]
    fn test_id_consistency() {
        let db = Database::new();
        let _ = db.clear();
        
        // Create and save party
        let mut party = Party::new("Alice");
        let id = db.save_entity(&mut party).unwrap();
        
        // Verify ID is set
        assert_eq!(party.id(), Some(id.as_str()));
        
        // Test JSON serialization includes ID
        let json = party.to_json().unwrap();
        assert!(json.contains(&format!("\"id\":\"{}\"", id)));
        
        // Load and verify
        let loaded = db.load_entity::<Party>(&id).unwrap().unwrap();
        assert_eq!(loaded.id(), Some(id.as_str()));
        
        // Update and verify
        db.update_entity(&loaded).unwrap();
    }

    // Helper function to create a test database
    fn create_test_db() -> Database{
        let db = Database::new();
        let _ = db.clear();
        db
    }
    
    // Helper function to create a test EncryptedMessage
    fn create_test_message() -> EncryptedMessage {
        let sender_public = [1u8; 32];
        let ciphertext = vec![1, 2, 3, 4, 5];
        let nonce = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        
        EncryptedMessage::new(
            PublicKey::from_bytes(sender_public),
            ciphertext,
            nonce,
        )
    }

    #[test]
    #[serial(local_db)]
    fn test_save_encrypted_message_generates_id() {
        let db = create_test_db();

        let mut message = create_test_message();
        
        // Initially no ID
        assert!(message.id().is_none());
        
        // Save should generate an ID
        let key = db.save_entity(&mut message).unwrap();
        
        // Should have ID after save
        assert!(message.id().is_some());
        assert_eq!(message.id().unwrap(), key);
        assert!(key.starts_with(EncryptedMessage::key_prefix()));
    }

    #[test]
    #[serial(local_db)]
    fn test_save_and_load_encrypted_message() {
        let db = create_test_db();

        let mut original_message = create_test_message();
        
        // Save the message
        let key = db.save_entity(&mut original_message).unwrap();
        
        // Load it back
        let loaded_message: Option<EncryptedMessage> = db.load_entity(&key).unwrap();
        
        assert!(loaded_message.is_some());
        let loaded = loaded_message.unwrap();
        
        // Verify all fields match
        assert_eq!(loaded.id(), original_message.id());
        assert_eq!(loaded.sender_public_bytes(), original_message.sender_public_bytes());
        assert_eq!(loaded.ciphertext, original_message.ciphertext);
        assert_eq!(loaded.nonce, original_message.nonce);
    }

    #[test]
    #[serial(local_db)]
    fn test_update_encrypted_message() {
        let db = create_test_db();
        
        let mut message = create_test_message();
        
        // Save initial message
        let key = db.save_entity(&mut message).unwrap();
        
        // Modify the message (update ciphertext)
        message.ciphertext = vec![10, 20, 30, 40, 50];
        
        // Update in database
        db.update_entity(&message).unwrap();
        
        // Load and verify the update
        let loaded: Option<EncryptedMessage> = db.load_entity(&key).unwrap();
        assert!(loaded.is_some());
        let loaded = loaded.unwrap();
        
        assert_eq!(loaded.ciphertext, vec![10, 20, 30, 40, 50]);
        assert_eq!(loaded.id(), message.id());
    }

    #[test]
    #[serial(local_db)]
    fn test_load_all_encrypted_messages() {
        let db = create_test_db();
        
        // Create and save multiple messages
        let mut message1 = create_test_message();
        let mut message2 = EncryptedMessage::new(
            PublicKey::from_bytes([1u8; 32]),
            vec![100, 200, 254],
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
        );
        let mut message3 = EncryptedMessage::new(
            PublicKey::from_bytes([2u8; 32]),
            vec![1, 2, 3, 4, 5],
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
        );
        
        db.save_entity(&mut message1).unwrap();
        db.save_entity(&mut message2).unwrap();
        db.save_entity(&mut message3).unwrap();
        
        // Load all encrypted messages
        let all_messages: Vec<EncryptedMessage> = db.load_all_entities(EncryptedMessage::key_prefix()).unwrap();
        
        assert_eq!(all_messages.len(), 3);
        
        // Verify all messages have IDs and correct prefix
        for msg in &all_messages {
            assert!(msg.id().is_some());
            assert!(msg.id().unwrap().starts_with(EncryptedMessage::key_prefix()));
        }
    }

    #[test]
    #[serial(local_db)]
    fn test_delete_encrypted_message() {
        let db = create_test_db();

        let mut message = create_test_message();
        
        // Save the message
        let key = db.save_entity(&mut message).unwrap();
        
        // Verify it exists
        let loaded: Option<EncryptedMessage> = db.load_entity(&key).unwrap();
        assert!(loaded.is_some());
        
        // Delete it
        let deleted: EncryptedMessage = db.delete(&key).unwrap();
        assert_eq!(deleted.id(), message.id());
        
        // Verify it's gone
        let loaded_after_delete: Option<EncryptedMessage> = db.load_entity(&key).unwrap();
        assert!(loaded_after_delete.is_none());
    }

    #[test]
    #[serial(local_db)]
    fn test_encrypted_message_json_serialization_with_id() {
        let db = create_test_db();

        let mut message = create_test_message();
        
        // Save to get an ID
        db.save_entity(&mut message).unwrap();
        
        // Serialize to JSON
        let json = message.to_json().unwrap();
        
        // Should contain the ID field
        assert!(json.contains("\"id\":"));
        
        // Deserialize back
        let restored = EncryptedMessage::from_json(&json).unwrap();
        
        // Should match original
        assert_eq!(restored.id(), message.id());
        assert_eq!(restored.sender_public_bytes(), message.sender_public_bytes());
        assert_eq!(restored.ciphertext, message.ciphertext);
        assert_eq!(restored.nonce, message.nonce);
    }
}
