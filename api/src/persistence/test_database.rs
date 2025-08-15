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
    use crate::crypto::message::{Contact, EncryptedMessage, Room};
    use crate::persistence::database::{Database, Entity};
    use crypto_box::PublicKey;
    use serial_test::serial;
    use std::collections::HashSet;

    fn create_test_room() -> Room {
        let known_contacts = [
            &PublicKey::from_bytes([
                11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
                11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
            ]),
            &PublicKey::from_bytes([
                22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
                22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
            ]),
            &PublicKey::from_bytes([
                33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33,
                33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33,
            ]),
        ];
        Room::new_with_contacts("Alice", &known_contacts)
    }

    #[test]
    #[serial(local_db)]
    fn test_save_and_load_room() -> Result<(), Box<dyn std::error::Error>> {
        let db = Database::new();
        let _ = db.clear();

        // Create test data
        let mut room = create_test_room();

        // Save the Room
        let key = db.save_entity(&mut room)?;

        // Load it back
        let loaded_room: Option<Room> = db.load_entity(&key)?;

        // Verify it was loaded correctly
        assert!(loaded_room.is_some());
        let loaded = loaded_room.unwrap();

        assert_eq!(loaded.name, room.name);
        assert_eq!(loaded.secret_key_bytes(), room.secret_key_bytes());
        assert_eq!(loaded.public_key_bytes(), room.public_key_bytes());
        assert_eq!(loaded.known_contacts_bytes(), room.known_contacts_bytes());

        Ok(())
    }

    #[test]
    #[serial(local_db)]
    fn test_multiple_parties() -> Result<(), Box<dyn std::error::Error>> {
        let db = Database::new();
        let _ = db.clear();

        // Create multiple parties with consistent key lengths
        let parties = vec![
            (
                "room:alice",
                "alice",
                [
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1,
                ],
                [
                    10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                    10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                ],
            ),
            (
                "room:bob",
                "bob",
                [
                    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                    2, 2, 2, 2, 2, 2,
                ],
                [
                    20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20,
                    20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20,
                ],
            ),
            (
                "room:charlie",
                "charlie",
                [
                    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
                    3, 3, 3, 3, 3, 3,
                ],
                [
                    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
                    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
                ],
            ),
        ];

        let mut room_keys = Vec::new();
        // Save all parties
        for (id, name, secret_key, public_key) in &parties {
            let mut contacts = HashSet::new();
            contacts.insert([
                99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99,
                99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99,
            ]); // Common contact
            contacts.insert([
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
            ]); // Another contact

            let mut room = Room::from_values(
                Some(id.to_string()),
                name,
                "Test room description",
                0,
                *secret_key,
                *public_key,
                contacts,
            );
            let id = db.save_entity(&mut room)?;
            room_keys.push(id);
        }

        // List all room keys
        let room_keys: Vec<Room> = db.load_all_entities(Room::key_prefix()).unwrap();
        assert_eq!(room_keys.len(), 3);

        // Verify we can load each one
        for (id, name, secret_key, public_key) in parties {
            let loaded: Option<Room> = db.load_entity(id)?;
            assert!(loaded.is_some());

            let room = loaded.unwrap();
            assert_eq!(room.name, name);
            assert_eq!(room.secret_key_bytes(), secret_key);
            assert_eq!(room.public_key_bytes(), public_key);
            assert!(room.known_contacts_bytes().contains(&[
                99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99,
                99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99
            ]));
            assert!(room.known_contacts_bytes().contains(&[
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100
            ]));
            assert_eq!(room.known_contacts().len(), 2);
        }

        Ok(())
    }

    #[test]
    #[serial(local_db)]
    fn test_update_room() -> Result<(), Box<dyn std::error::Error>> {
        let db = Database::new();
        let _ = db.clear();
        let mut room = create_test_room();

        // Save initial version
        db.save_entity(&mut room)?;

        // Update the room (add a new contact)
        room.add_contact(&PublicKey::from_bytes([
            100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
            100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        ]));
        let key = db.save_entity(&mut room)?;

        // Load and verify the update
        let loaded: Option<Room> = db.load_entity(&key)?;
        assert!(loaded.is_some());

        let loaded_room = loaded.unwrap();
        assert_eq!(loaded_room.known_contacts().len(), 4); // Original 3 + 1 new
        assert!(loaded_room
            .known_contacts()
            .contains(&PublicKey::from_bytes([
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100
            ])));

        Ok(())
    }

    #[test]
    #[serial(local_db)]
    fn test_delete_room() -> Result<(), Box<dyn std::error::Error>> {
        let db = Database::new();
        let _ = db.clear();
        let mut room = create_test_room();

        // Save the room
        let key = db.save_entity(&mut room)?;

        // Verify it exists
        let loaded: Option<Room> = db.load_entity(&key)?;
        assert!(loaded.is_some());

        // Delete it
        let deleted: Room = db.delete(&key)?;
        assert!(deleted.name == "Alice");

        // Verify it's gone
        let loaded_after_delete: Option<Room> = db.load_entity(&key)?;
        assert!(loaded_after_delete.is_none());

        Ok(())
    }

    #[test]
    #[serial(local_db)]
    fn test_load_nonexistent_room() -> Result<(), Box<dyn std::error::Error>> {
        let db = Database::new();
        let _ = db.clear();

        // Try to load a room that doesn't exist
        let loaded: Option<Room> = db.load_entity("room:nonexistent")?;
        assert!(loaded.is_none());

        Ok(())
    }

    #[test]
    #[serial(local_db)]
    fn test_room_dto_conversion() -> Result<(), Box<dyn std::error::Error>> {
        let db = Database::new();
        let _ = db.clear();

        let original_room = create_test_room();

        // Test round-trip conversion: Room -> JSON -> Room
        let mut room = Room::from_json(original_room.to_json().unwrap().as_str()).unwrap();
        assert_eq!(room.name, original_room.name);
        assert_eq!(room.secret_key_bytes(), original_room.secret_key_bytes());
        assert_eq!(room.public_key_bytes(), original_room.public_key_bytes());
        assert_eq!(
            room.known_contacts_bytes(),
            original_room.known_contacts_bytes()
        );

        // Test saving the converted DTO
        let id = db.save_entity(&mut room)?;
        let loaded: Option<Room> = db.load_entity(&id)?;
        assert!(loaded.is_some());

        Ok(())
    }

    #[test]
    #[serial(local_db)]
    fn test_id_consistency() {
        let db = Database::new();
        let _ = db.clear();

        // Create and save room
        let mut room = Room::new("Alice");
        let id = db.save_entity(&mut room).unwrap();

        // Verify ID is set
        assert_eq!(room.id(), Some(id.as_str()));

        // Test JSON serialization includes ID
        let json = room.to_json().unwrap();
        assert!(json.contains(&format!("\"id\":\"{id}\"")));

        // Load and verify
        let loaded = db.load_entity::<Room>(&id).unwrap().unwrap();
        assert_eq!(loaded.id(), Some(id.as_str()));

        // Update and verify
        db.update_entity(&loaded).unwrap();
    }

    // Helper function to create a test database
    fn create_test_db() -> Database {
        let db = Database::new();
        let _ = db.clear();
        db
    }

    // Helper function to create a test EncryptedMessage
    fn create_test_message() -> EncryptedMessage {
        let sender_public = [1u8; 32];
        let ciphertext = vec![1, 2, 3, 4, 5];
        let nonce = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];

        EncryptedMessage::new(PublicKey::from_bytes(sender_public), ciphertext, nonce)
    }

    fn create_test_contact() -> Contact {
        use crypto_box::SecretKey;
        let secret_key = SecretKey::generate(&mut crypto_box::aead::OsRng);
        let public_key = secret_key.public_key();
        Contact::new("Alice", &public_key)
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
        assert_eq!(
            loaded.sender_public_bytes(),
            original_message.sender_public_bytes()
        );
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
        let all_messages: Vec<EncryptedMessage> = db
            .load_all_entities(EncryptedMessage::key_prefix())
            .unwrap();

        assert_eq!(all_messages.len(), 3);

        // Verify all messages have IDs and correct prefix
        for msg in &all_messages {
            assert!(msg.id().is_some());
            assert!(msg
                .id()
                .unwrap()
                .starts_with(EncryptedMessage::key_prefix()));
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
        assert_eq!(
            restored.sender_public_bytes(),
            message.sender_public_bytes()
        );
        assert_eq!(restored.ciphertext, message.ciphertext);
        assert_eq!(restored.nonce, message.nonce);
    }

    #[test]
    #[serial(local_db)]
    fn test_save_and_load_contact() -> Result<(), Box<dyn std::error::Error>> {
        let db = Database::new();
        let _ = db.clear();

        let mut contact = create_test_contact();

        let key = db.save_entity(&mut contact)?;

        let loaded_contact: Option<Contact> = db.load_entity(&key)?;

        assert!(loaded_contact.is_some());
        let loaded = loaded_contact.unwrap();

        assert_eq!(loaded.name, contact.name);
        assert_eq!(loaded.public_key, contact.public_key);
        assert_eq!(loaded.nickname, contact.nickname);
        assert_eq!(loaded.email, contact.email);
        assert_eq!(loaded.verified, contact.verified);
        assert_eq!(loaded.blocked, contact.blocked);
        assert_eq!(loaded.created_at, contact.created_at);

        Ok(())
    }

    #[test]
    #[serial(local_db)]
    fn test_multiple_contacts() -> Result<(), Box<dyn std::error::Error>> {
        let db = Database::new();
        let _ = db.clear();

        use crypto_box::SecretKey;
        let contacts_data = vec![
            ("contact:alice", "Alice", "alice@example.com"),
            ("contact:bob", "Bob", "bob@example.com"),
            ("contact:charlie", "Charlie", "charlie@example.com"),
        ];

        let mut contact_keys = Vec::new();
        for (id, name, email) in &contacts_data {
            let secret_key = SecretKey::generate(&mut crypto_box::aead::OsRng);
            let public_key = secret_key.public_key();
            let mut contact = Contact {
                id: Some(id.to_string()),
                name: name.to_string(),
                public_key: public_key.to_bytes(),
                nickname: Some(format!("nickname_{}", name.to_lowercase())),
                email: Some(email.to_string()),
                verified: name == &"Alice",
                blocked: false,
                created_at: 1640995200,
                last_seen: Some(1640995300),
                ..Default::default()
            };
            let key = db.save_entity(&mut contact)?;
            contact_keys.push(key);
        }

        let all_contacts: Vec<Contact> = db.load_all_entities(Contact::key_prefix()).unwrap();
        assert_eq!(all_contacts.len(), 3);

        for (id, name, email) in contacts_data {
            let loaded: Option<Contact> = db.load_entity(id)?;
            assert!(loaded.is_some());

            let contact = loaded.unwrap();
            assert_eq!(contact.name, name);
            assert_eq!(contact.email, Some(email.to_string()));
            assert_eq!(
                contact.nickname,
                Some(format!("nickname_{}", name.to_lowercase()))
            );
            assert_eq!(contact.verified, name == "Alice");
            assert!(!contact.blocked);
        }

        Ok(())
    }

    #[test]
    #[serial(local_db)]
    fn test_update_contact() -> Result<(), Box<dyn std::error::Error>> {
        let db = Database::new();
        let _ = db.clear();
        let mut contact = create_test_contact();

        db.save_entity(&mut contact)?;

        contact.set_nickname(Some("Ally".to_string()));
        contact.set_email(Some("alice@newdomain.com".to_string()));
        contact.set_verified(true);
        contact.update_last_seen();
        let key = db.save_entity(&mut contact)?;

        let loaded: Option<Contact> = db.load_entity(&key)?;
        assert!(loaded.is_some());

        let loaded_contact = loaded.unwrap();
        assert_eq!(loaded_contact.nickname, Some("Ally".to_string()));
        assert_eq!(
            loaded_contact.email,
            Some("alice@newdomain.com".to_string())
        );
        assert!(loaded_contact.verified);
        assert!(loaded_contact.last_seen.is_some());

        Ok(())
    }

    #[test]
    #[serial(local_db)]
    fn test_delete_contact() -> Result<(), Box<dyn std::error::Error>> {
        let db = Database::new();
        let _ = db.clear();
        let mut contact = create_test_contact();

        let key = db.save_entity(&mut contact)?;

        let loaded: Option<Contact> = db.load_entity(&key)?;
        assert!(loaded.is_some());

        let deleted: Contact = db.delete(&key)?;
        assert!(deleted.name == "Alice");

        let loaded_after_delete: Option<Contact> = db.load_entity(&key)?;
        assert!(loaded_after_delete.is_none());

        Ok(())
    }

    #[test]
    #[serial(local_db)]
    fn test_load_nonexistent_contact() -> Result<(), Box<dyn std::error::Error>> {
        let db = Database::new();
        let _ = db.clear();

        let loaded: Option<Contact> = db.load_entity("contact:nonexistent")?;
        assert!(loaded.is_none());

        Ok(())
    }
}
