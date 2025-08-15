/*  This file is part of a secure messaging project codename meeseeks-nuntius
 *  Copyright (C) 2025 Grant DeFayette
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

// When debugging tests for readable output run with "cargo test -- --nocapture".

#[cfg(test)]
mod test_utils {
    /*
     * Test utils for enhanced output when debugging tests.
     */

    // ANSI color codes for terminal output
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const BLUE: &str = "\x1b[34m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const CYAN: &str = "\x1b[36m";
    pub const WHITE: &str = "\x1b[37m";
    pub const RESET: &str = "\x1b[0m";
    pub const BOLD: &str = "\x1b[1m";

    pub fn print_separator() {
        println!("{CYAN}═══════════════════════════════════════════════════════════{RESET}");
    }

    pub fn print_test_header(test_name: &str, emoji: &str) {
        print_separator();
        println!("{BOLD}{emoji} {test_name} {emoji}{RESET}");
        print_separator();
    }

    pub fn print_str_data(label: &str, data: &str, color: &str) {
        println!("{color}{label}: {data}{RESET}");
        println!("{WHITE}Length: {} bytes{RESET}", data.len());
    }

    pub fn print_hex_data(label: &str, data: &[u8], color: &str) {
        let hex_str = hex::encode(data);
        println!("{color}{label}: {hex_str}{RESET}");
        println!("{WHITE}Length: {} bytes{RESET}", data.len());
    }

    pub fn print_success(message: &str) {
        println!("{GREEN}✅ {message}{RESET}");
    }

    pub fn print_info(message: &str) {
        println!("{BLUE}ℹ️  {message}{RESET}");
    }

    pub fn print_warning(message: &str) {
        println!("{YELLOW}⚠️  {message}{RESET}");
    }

    pub fn print_error(message: &str) {
        println!("{RED}❌ {message}{RESET}");
    }
}

#[cfg(test)]
mod test_aes {
    use crate::crypto::message::aes256_gcm::*;
    use crate::crypto::test_message::test_utils::*;
    use serial_test::serial;

    #[test]
    #[serial(aes256_gcm)]
    fn test_encrypt_decrypt_basic() {
        print_test_header("Basic Encrypt/Decrypt Test", "🔐");

        let key = generate_key();
        let plaintext = b"Hello, AES-GCM!";

        print_info("Generating random 256-bit key...");
        print_hex_data("Key", &key, MAGENTA);

        print_info("Original plaintext:");
        println!(
            "{GREEN}\"{}\" ({}){RESET}",
            String::from_utf8_lossy(plaintext),
            plaintext.len()
        );
        print_hex_data("Plaintext bytes", plaintext, GREEN);

        println!("\n{YELLOW}🔒 ENCRYPTION PHASE{RESET}");
        let (ciphertext, nonce) = encrypt(&key, plaintext).unwrap();
        print_success("Encryption successful!");

        print_hex_data("Ciphertext", &ciphertext, RED);
        print_hex_data("Nonce", &nonce, CYAN);

        // Verify ciphertext is different from plaintext
        assert_ne!(ciphertext, plaintext);
        print_success("✓ Ciphertext differs from plaintext (as expected)");

        println!("\n{YELLOW}🔓 DECRYPTION PHASE{RESET}");
        let decrypted = decrypt(&key, &ciphertext, &nonce).unwrap();
        print_success("Decryption successful!");

        print_hex_data("Decrypted bytes", &decrypted, GREEN);
        println!(
            "{GREEN}Decrypted text: \"{}\"{RESET}",
            String::from_utf8_lossy(&decrypted)
        );

        // Verify decrypted text matches original
        assert_eq!(decrypted, plaintext);
        print_success("✓ Decrypted text matches original plaintext!");

        println!("{BOLD}🎉 Basic encryption/decryption test PASSED!{RESET}\n");
    }

    #[test]
    #[serial(aes256_gcm)]
    fn test_encrypt_decrypt_string() {
        print_test_header("String Encryption Test", "📝");

        let key = generate_key();
        let plaintext = "The quick brown fox jumps over the lazy dog";

        print_info("Testing string convenience functions...");
        println!("{GREEN}Original message: \"{plaintext}\"{RESET}");
        println!(
            "{WHITE}Message length: {} characters{RESET}",
            plaintext.len()
        );

        println!("\n{YELLOW}🔒 ENCRYPTING STRING{RESET}");
        let (ciphertext, nonce) = encrypt_string(&key, plaintext).unwrap();
        print_success("String encryption successful!");

        print_hex_data("Encrypted string", &ciphertext, RED);
        print_hex_data("Nonce", &nonce, CYAN);

        println!("\n{YELLOW}🔓 DECRYPTING STRING{RESET}");
        let decrypted = decrypt_string(&key, &ciphertext, &nonce).unwrap();
        print_success("String decryption successful!");

        println!("{GREEN}Decrypted: \"{decrypted}\"{RESET}");

        // Verify
        assert_eq!(decrypted, plaintext);
        print_success("✓ String round-trip successful!");

        println!("{BOLD}🎉 String encryption test PASSED!{RESET}\n");
    }

    #[test]
    #[serial(aes256_gcm)]
    fn test_different_keys_fail() {
        print_test_header("Wrong Key Security Test", "🔑");

        let key1 = generate_key();
        let key2 = generate_key();
        let plaintext = b"Secret message";

        print_info("Testing that different keys cannot decrypt each other's data...");
        println!(
            "{GREEN}Plaintext: \"{}\"{RESET}",
            String::from_utf8_lossy(plaintext)
        );

        print_hex_data("Key 1", &key1, MAGENTA);
        print_hex_data("Key 2", &key2, MAGENTA);

        print_warning("Keys are different (as expected for security)");

        println!("\n{YELLOW}🔒 ENCRYPTING WITH KEY 1{RESET}");
        let (ciphertext, nonce) = encrypt(&key1, plaintext).unwrap();
        print_success("Encryption with Key 1 successful!");

        print_hex_data("Ciphertext", &ciphertext, RED);
        print_hex_data("Nonce", &nonce, CYAN);

        println!("\n{YELLOW}🔓 ATTEMPTING DECRYPTION WITH KEY 2{RESET}");
        let result = decrypt(&key2, &ciphertext, &nonce);

        match result {
            Ok(_) => {
                print_error("SECURITY BREACH: Wrong key should not decrypt!");
                panic!("Security test failed - wrong key decrypted data!");
            }
            Err(e) => {
                print_success("✓ Decryption correctly failed with wrong key");
                print_info(&format!("Error message: {e}"));
            }
        }

        println!("{BOLD}🛡️  Security test PASSED - wrong keys cannot decrypt!{RESET}\n");
    }

    #[test]
    #[serial(aes256_gcm)]
    fn test_wrong_nonce_fails() {
        print_test_header("Wrong Nonce Security Test", "🎲");

        let key = generate_key();
        let plaintext = b"Another secret";

        print_info("Testing that wrong nonces prevent decryption...");
        println!(
            "{GREEN}Plaintext: \"{}\"{RESET}",
            String::from_utf8_lossy(plaintext)
        );

        println!("\n{YELLOW}🔒 ENCRYPTING WITH RANDOM NONCE{RESET}");
        let (ciphertext, correct_nonce) = encrypt(&key, plaintext).unwrap();
        print_success("Encryption successful!");

        print_hex_data("Ciphertext", &ciphertext, RED);
        print_hex_data("Correct nonce", &correct_nonce, CYAN);

        // Try with wrong nonce
        let wrong_nonce = vec![0u8; 12]; // GCM nonce is 12 bytes
        print_hex_data("Wrong nonce (all zeros)", &wrong_nonce, RED);

        println!("\n{YELLOW}🔓 ATTEMPTING DECRYPTION WITH WRONG NONCE{RESET}");
        let result = decrypt(&key, &ciphertext, &wrong_nonce);

        match result {
            Ok(_) => {
                print_error("SECURITY BREACH: Wrong nonce should not decrypt!");
                panic!("Security test failed - wrong nonce decrypted data!");
            }
            Err(e) => {
                print_success("✓ Decryption correctly failed with wrong nonce");
                print_info(&format!("Error message: {e}"));
            }
        }

        println!("{BOLD}🛡️  Nonce security test PASSED!{RESET}\n");
    }

    #[test]
    #[serial(aes256_gcm)]
    fn test_empty_plaintext() {
        print_test_header(" Empty Data Test", "🕳️");

        let key = generate_key();
        let plaintext = b"";

        print_info("Testing encryption/decryption of empty data...");
        println!("{YELLOW}Plaintext: <empty> (0 bytes){RESET}");

        println!("\n{YELLOW}🔒 ENCRYPTING EMPTY DATA{RESET}");
        let (ciphertext, nonce) = encrypt(&key, plaintext).unwrap();
        print_success("Empty data encryption successful!");

        print_hex_data("Ciphertext", &ciphertext, RED);
        print_hex_data("Nonce", &nonce, CYAN);
        print_info("Note: Even empty data produces authentication tag");

        println!("\n{YELLOW}🔓 DECRYPTING EMPTY DATA{RESET}");
        let decrypted = decrypt(&key, &ciphertext, &nonce).unwrap();
        print_success("Empty data decryption successful!");

        println!("{GREEN}Decrypted length: {} bytes{RESET}", decrypted.len());

        assert_eq!(decrypted, plaintext);
        print_success("✓ Empty data round-trip successful!");

        println!("{BOLD}🎉 Empty data test PASSED!{RESET}\n");
    }

    #[test]
    #[serial(aes256_gcm)]
    fn test_large_plaintext() {
        print_test_header("Large Data Test", "📦");

        let key = generate_key();
        let plaintext = vec![42u8; 1024]; // 1KB of data

        print_info("Testing encryption/decryption of large data (1KB)...");
        println!(
            "{YELLOW}Plaintext: {} bytes of value 42 (0x2A){RESET}",
            plaintext.len()
        );

        // Show a sample of the data
        let sample = &plaintext[..std::cmp::min(32, plaintext.len())];
        print_hex_data("First 32 bytes", sample, GREEN);

        println!("\n{YELLOW}🔒 ENCRYPTING LARGE DATA{RESET}");
        let start_time = std::time::Instant::now();
        let (ciphertext, nonce) = encrypt(&key, &plaintext).unwrap();
        let encrypt_time = start_time.elapsed();
        print_success(&format!(
            "Large data encryption successful in {encrypt_time:?}!"
        ));

        print_hex_data("Nonce", &nonce, CYAN);
        println!("{RED}Ciphertext length: {} bytes{RESET}", ciphertext.len());

        // Show first few bytes of ciphertext
        let cipher_sample = &ciphertext[..std::cmp::min(32, ciphertext.len())];
        print_hex_data("First 32 bytes of ciphertext", cipher_sample, RED);

        println!("\n{YELLOW}🔓 DECRYPTING LARGE DATA{RESET}");
        let start_time = std::time::Instant::now();
        let decrypted = decrypt(&key, &ciphertext, &nonce).unwrap();
        let decrypt_time = start_time.elapsed();
        print_success(&format!(
            "Large data decryption successful in {decrypt_time:?}!"
        ));

        println!("{GREEN}Decrypted length: {} bytes{RESET}", decrypted.len());

        assert_eq!(decrypted, plaintext);
        print_success("✓ Large data integrity verified!");

        println!("{BLUE}📊 Performance: Encrypt {encrypt_time:?}, Decrypt {decrypt_time:?}{RESET}");
        println!("{BOLD}🎉 Large data test PASSED!{RESET}\n");
    }

    #[test]
    #[serial(aes256_gcm)]
    fn test_nonce_uniqueness() {
        print_test_header("Nonce Uniqueness Test", "🎯");

        let key = generate_key();
        let plaintext = b"Same message";

        print_info("Testing that nonces are unique for identical messages...");
        println!(
            "{GREEN}Message: \"{}\"{RESET}",
            String::from_utf8_lossy(plaintext)
        );

        println!("\n{YELLOW}🔒 FIRST ENCRYPTION{RESET}");
        let (ciphertext1, nonce1) = encrypt(&key, plaintext).unwrap();
        print_success("First encryption successful!");
        print_hex_data("Ciphertext 1", &ciphertext1, RED);
        print_hex_data("Nonce 1", &nonce1, CYAN);

        println!("\n{YELLOW}🔒 SECOND ENCRYPTION (SAME MESSAGE){RESET}");
        let (ciphertext2, nonce2) = encrypt(&key, plaintext).unwrap();
        print_success("Second encryption successful!");
        print_hex_data("Ciphertext 2", &ciphertext2, RED);
        print_hex_data("Nonce 2", &nonce2, CYAN);

        // Nonces should be different even for same message
        assert_ne!(nonce1, nonce2);
        print_success("✓ Nonces are different (critical for security)!");

        // Ciphertexts should also be different due to different nonces
        assert_ne!(ciphertext1, ciphertext2);
        print_success("✓ Ciphertexts are different (due to unique nonces)!");

        println!("\n{YELLOW}🔓 VERIFYING BOTH DECRYPT CORRECTLY{RESET}");
        let decrypted1 = decrypt(&key, &ciphertext1, &nonce1).unwrap();
        let decrypted2 = decrypt(&key, &ciphertext2, &nonce2).unwrap();

        assert_eq!(decrypted1, plaintext);
        assert_eq!(decrypted2, plaintext);
        print_success("✓ Both ciphertexts decrypt to original message!");

        println!(
            "{BOLD}🎉 Nonce uniqueness test PASSED - cryptographic security maintained!{RESET}\n"
        );
    }
}

#[cfg(test)]
mod test_exchange {
    use crate::crypto::message::*;
    use crate::crypto::test_message::test_utils::*;
    use serial_test::serial;

    fn print_room_info(room: &Room) {
        println!("{YELLOW}👤 Room: {}{RESET}", room.name);
        print_hex_data(
            &format!("{}'s Public Key", room.name),
            &room.public_key_bytes(),
            MAGENTA,
        );
    }

    #[test]
    #[serial(exchange)]
    fn test_basic_messaging() {
        print_test_header("Basic crypto_box Messaging", "💬");

        // Create two rooms
        let mut alice = Room::new("Alice");
        let mut bob = Room::new("Bob");

        print_info("Created two rooms for secure communication");
        print_room_info(&alice);
        print_room_info(&bob);

        let message = "Hello Bob! This is a secret message from Alice.";
        println!("\n{GREEN}📝 Original message: \"{message}\"{RESET}");

        println!("\n{YELLOW}🔒 ALICE ENCRYPTING MESSAGE FOR BOB{RESET}");
        let encrypted = alice
            .encrypt_string_for(&bob.public_key(), message)
            .unwrap();
        print_success("Message encrypted successfully!");
        print_info(&format!(
            "Alice now has {} known contacts",
            alice.contact_count()
        ));

        print_hex_data(
            "Sender Public Key",
            &encrypted.sender_public_bytes(),
            MAGENTA,
        );
        print_hex_data("Nonce", &encrypted.nonce, CYAN);
        print_hex_data("Ciphertext", &encrypted.ciphertext, RED);

        println!("\n{YELLOW}🔓 BOB DECRYPTING MESSAGE FROM ALICE{RESET}");
        let decrypted = bob.decrypt_string_from(&encrypted).unwrap();
        print_success("Message decrypted successfully!");
        print_info(&format!(
            "Bob now has {} known contacts",
            bob.contact_count()
        ));

        println!("{GREEN}📖 Decrypted message: \"{decrypted}\"{RESET}");

        assert_eq!(decrypted, message);
        print_success("✓ Message integrity verified!");

        println!("{BOLD}🎉 Basic messaging test PASSED!{RESET}\n");
    }

    #[test]
    #[serial(exchange)]
    fn test_bidirectional_messaging() {
        print_test_header("Bidirectional Messaging", "🔄");

        let mut alice = Room::new("Alice");
        let mut bob = Room::new("Bob");

        print_info("Testing two-way communication between rooms");
        print_room_info(&alice);
        print_room_info(&bob);

        // Alice to Bob
        let alice_message = "Hi Bob, how are you?";
        println!("\n{GREEN}👩 Alice's message: \"{alice_message}\"{RESET}");

        let alice_encrypted = alice
            .encrypt_string_for(&bob.public_key(), alice_message)
            .unwrap();
        print_success("Alice's message encrypted");
        print_info(&format!("Alice known contacts: {}", alice.contact_count()));

        let bob_decrypted = bob.decrypt_string_from(&alice_encrypted).unwrap();
        println!("{BLUE}👨 Bob received: \"{bob_decrypted}\"{RESET}");
        print_info(&format!("Bob known contacts: {}", bob.contact_count()));
        assert_eq!(bob_decrypted, alice_message);

        // Bob to Alice
        let bob_message = "Hi Alice! I'm doing great, thanks for asking!";
        println!("\n{BLUE}👨 Bob's reply: \"{bob_message}\"{RESET}");

        let bob_encrypted = bob
            .encrypt_string_for(&alice.public_key(), bob_message)
            .unwrap();
        print_success("Bob's message encrypted");
        print_info(&format!("Bob known contacts: {}", bob.contact_count()));

        let alice_decrypted = alice.decrypt_string_from(&bob_encrypted).unwrap();
        println!("{GREEN}👩 Alice received: \"{alice_decrypted}\"{RESET}");
        print_info(&format!("Alice known contacts: {}", alice.contact_count()));
        assert_eq!(alice_decrypted, bob_message);

        print_success("✓ Bidirectional communication successful!");
        print_success("✓ Both rooms have registered each other as contacts!");
        println!("{BOLD}🎉 Bidirectional messaging test PASSED!{RESET}\n");
    }

    #[test]
    #[serial(exchange)]
    fn test_message_serialization() {
        print_test_header("Message Serialization", "📦");

        let mut alice = Room::new("Alice");
        let mut bob = Room::new("Bob");

        print_info("Testing message serialization for network transmission");

        let original_message = "This message will be serialized and deserialized!";
        println!("{GREEN}Original: \"{original_message}\"{RESET}");

        // Encrypt and serialize
        println!("\n{YELLOW}📤 SERIALIZATION PHASE{RESET}");
        let encrypted = alice
            .encrypt_string_for(&bob.public_key(), original_message)
            .unwrap();
        let serialized = encrypted.to_json().unwrap();
        print_success("Message encrypted and serialized");
        print_str_data("Serialized message", serialized.as_str(), CYAN);

        // Deserialize and decrypt
        println!("\n{YELLOW}📥 DESERIALIZATION PHASE{RESET}");
        let deserialized = EncryptedMessage::from_json(&serialized).unwrap();
        print_success("Message deserialized successfully");

        let decrypted = bob.decrypt_string_from(&deserialized).unwrap();
        print_success("Message decrypted successfully");
        println!("{GREEN}Decrypted: \"{decrypted}\"{RESET}");

        assert_eq!(decrypted, original_message);
        print_success("✓ Round-trip serialization successful!");

        println!("{BOLD}🎉 Serialization test PASSED!{RESET}\n");
    }

    #[test]
    #[serial(exchange)]
    fn test_wrong_recipient_fails() {
        print_test_header("Wrong Recipient Security", "🚫");

        let mut alice = Room::new("Alice");
        let mut bob = Room::new("Bob");
        let mut eve = Room::new("Eve"); // Eavesdropper

        print_info("Testing that only intended recipients can decrypt messages");
        print_room_info(&alice);
        print_room_info(&bob);
        print_room_info(&eve);

        let secret_message = "This is for Bob's eyes only!";
        println!("\n{GREEN}🤐 Secret message: \"{secret_message}\"{RESET}");

        println!("\n{YELLOW}🔒 ALICE ENCRYPTING FOR BOB{RESET}");
        let encrypted = alice
            .encrypt_string_for(&bob.public_key(), secret_message)
            .unwrap();
        print_success("Message encrypted for Bob");

        println!("\n{BLUE}👨 BOB ATTEMPTING TO DECRYPT{RESET}");
        let bob_result = bob.decrypt_string_from(&encrypted);
        match bob_result {
            Ok(decrypted) => {
                print_success("✓ Bob successfully decrypted the message");
                println!("{BLUE}Bob read: \"{decrypted}\"{RESET}");
                assert_eq!(decrypted, secret_message);
            }
            Err(_) => panic!("Bob should be able to decrypt the message!"),
        }

        println!("\n{RED}🕵️ EVE ATTEMPTING TO DECRYPT{RESET}");
        let eve_result = eve.decrypt_string_from(&encrypted);
        match eve_result {
            Ok(_) => {
                panic!("Security breach! Eve should not be able to decrypt Bob's message!");
            }
            Err(e) => {
                print_success("✓ Eve correctly failed to decrypt the message");
                print_info(&format!("Error: {e}"));
            }
        }

        println!("{BOLD}🛡️  Security test PASSED - only Bob can read his messages!{RESET}\n");
    }

    #[test]
    #[serial(exchange)]
    fn test_contact_management() {
        print_test_header("Contact Management", "📇");

        let mut alice = Room::new("Alice");
        let mut bob = Room::new("Bob");
        let mut charlie = Room::new("Charlie");

        print_info("Testing pre-established contact functionality");

        // Create a room with pre-established contacts
        let contacts = [&alice.public_key(), &bob.public_key()];
        let mut dave = Room::new_with_contacts("Dave", &contacts);

        print_info(&format!(
            "Dave created with {} pre-registered contacts",
            dave.contact_count()
        ));

        // IMPORTANT: For bidirectional communication, recipients also need Dave's key
        alice.add_contact(&dave.public_key());
        bob.add_contact(&dave.public_key());

        print_info(
            "Alice and Bob have been given Dave's public key for bidirectional communication",
        );

        // Verify Dave can communicate with pre-registered contacts
        let message = "Hello everyone!";

        let to_alice = dave
            .encrypt_string_for(&alice.public_key(), message)
            .unwrap();
        let to_bob = dave.encrypt_string_for(&bob.public_key(), message).unwrap();

        print_success("✓ Dave encrypted messages for pre-registered contacts");

        // Test actual decryption (this is the important part!)
        let alice_received = alice.decrypt_string_from(&to_alice).unwrap();
        let bob_received = bob.decrypt_string_from(&to_bob).unwrap();

        assert_eq!(alice_received, message);
        assert_eq!(bob_received, message);
        print_success("✓ Alice and Bob successfully decrypted Dave's messages");

        // Dave establishes contact with Charlie on-demand
        let to_charlie = dave
            .encrypt_string_for(&charlie.public_key(), message)
            .unwrap();
        print_success("✓ Dave encrypted message for new contact (Charlie)");

        // Charlie needs Dave's key to decrypt
        charlie.add_contact(&dave.public_key());
        let charlie_received = charlie.decrypt_string_from(&to_charlie).unwrap();
        assert_eq!(charlie_received, message);
        print_success("✓ Charlie successfully decrypted Dave's message");

        print_info(&format!(
            "Dave now has {} total contacts",
            dave.contact_count()
        ));

        // Verify contact list
        let dave_contacts = dave.known_contacts();
        assert_eq!(dave_contacts.len(), 3);
        print_success("✓ Contact list management working correctly");

        // Test bidirectional communication
        println!("\n{YELLOW}🔄 Testing bidirectional communication{RESET}");
        let reply_message = "Hi Dave! Nice to meet you!";
        let alice_reply = alice
            .encrypt_string_for(&dave.public_key(), reply_message)
            .unwrap();
        let dave_received = dave.decrypt_string_from(&alice_reply).unwrap();
        assert_eq!(dave_received, reply_message);
        print_success("✓ Bidirectional communication working correctly");

        println!("{BLUE}Dave's contacts:{RESET}");
        for (i, contact_key) in dave_contacts.iter().enumerate() {
            let key_hex = hex::encode(contact_key.to_bytes());
            println!("  {}. {}...", i + 1, &key_hex[..16]); // Show first 16 chars
        }

        println!("{BOLD}🎉 Contact management test PASSED!{RESET}\n");
    }

    #[test]
    #[serial(exchange)]
    fn test_crypto_box_creation() {
        print_test_header("crypto_box Performance metrics", "⚡");

        let mut alice = Room::new("Alice");
        let mut bob = Room::new("Bob");

        print_info("Testing that new crypto boxes are created for each message");

        let test_message = "This is a test message!";

        // Multiple encryptions - each creates a fresh crypto box
        println!("\n{YELLOW}🔒 MULTIPLE ENCRYPTIONS{RESET}");
        let start_time = std::time::Instant::now();
        let encrypted1 = alice
            .encrypt_string_for(&bob.public_key(), test_message)
            .unwrap();
        let first_encrypt_time = start_time.elapsed();
        print_success(&format!(
            "First encryption completed in {first_encrypt_time:?}"
        ));
        print_info(&format!("Alice known contacts: {}", alice.contact_count()));

        let start_time = std::time::Instant::now();
        for _n in 0..1000 {
            let encrypted2 = alice
                .encrypt_string_for(&bob.public_key(), "Second message")
                .unwrap();
            // Verify different nonces
            assert_ne!(encrypted1.nonce, encrypted2.nonce);
        }
        let second_encrypt_time = start_time.elapsed();
        assert!(second_encrypt_time.as_millis() < 1000);
        print_success(&format!(
            "1K encryptions completed in {second_encrypt_time:?}"
        ));
        print_info(&format!(
            "Contact count unchanged: {}",
            alice.contact_count()
        ));
        print_success("✓ Different nonces used (critical for security)!");

        // Decryption performance
        println!("\n{YELLOW}🔓 DECRYPTION PERFORMANCE{RESET}");
        let start_time = std::time::Instant::now();
        for _n in 0..1000 {
            let decrypted1 = bob.decrypt_string_from(&encrypted1).unwrap();
            assert_eq!(decrypted1, test_message);
        }
        let decrypt_time = start_time.elapsed();
        assert!(decrypt_time.as_millis() < 1000);
        print_success(&format!("Decryption completed in {decrypt_time:?}"));
        print_success("✓ Message integrity verified!");

        println!("\n{BLUE}📊 Performance Summary:{RESET}");
        println!("  First encryption: {first_encrypt_time:?}");
        println!("  1k encryptions: {second_encrypt_time:?}");
        println!("  1k decryption: {decrypt_time:?}");
        print_info("Note: Each encryption creates a fresh ChaChaBox (correct pattern)");

        println!("{BOLD}🎉 crypto_box creation pattern test PASSED!{RESET}\n");
    }

    #[test]
    #[serial(exchange)]
    fn test_contact_registration() {
        print_test_header("Contact Registration", "📋");

        let mut alice = Room::new("Alice");
        let bob = Room::new("Bob");
        let charlie = Room::new("Charlie");

        print_info("Testing contact registration without communication");

        // Initially no contacts
        assert_eq!(alice.contact_count(), 0);
        print_info("Alice starts with 0 contacts");

        // Add contacts manually
        alice.add_contact(&bob.public_key());
        alice.add_contact(&charlie.public_key());

        print_success(&format!(
            "Alice manually registered {} contacts",
            alice.contact_count()
        ));

        // Check if contacts are known
        assert!(alice.is_known_contact(&bob.public_key()));
        assert!(alice.is_known_contact(&charlie.public_key()));
        print_success("✓ Contact recognition working correctly");

        // Adding same contact twice shouldn't increase count
        alice.add_contact(&bob.public_key());
        assert_eq!(alice.contact_count(), 2);
        print_success("✓ Duplicate contact addition handled correctly");

        // List contacts
        let contacts = alice.known_contacts();
        assert_eq!(contacts.len(), 2);
        print_success("✓ Contact listing working correctly");

        println!("{BLUE}Alice's registered contacts:{RESET}");
        for (i, contact_key) in contacts.iter().enumerate() {
            let key_hex = hex::encode(contact_key.to_bytes());
            println!("  {}. {}...", i + 1, &key_hex[..16]);
        }

        println!("{BOLD}🎉 Contact registration test PASSED!{RESET}\n");
    }
}

#[cfg(test)]
mod test_contact {
    use crate::crypto::message::*;
    use crate::crypto::test_message::test_utils::*;
    use crate::persistence::database::Entity;
    use crypto_box::aead::OsRng;
    use crypto_box::{PublicKey, SecretKey};
    use serial_test::serial;
    use std::collections::HashSet;

    fn create_test_public_key() -> PublicKey {
        let secret_key = SecretKey::generate(&mut OsRng);
        secret_key.public_key()
    }

    #[test]
    #[serial(contact)]
    fn test_contact_creation_new() {
        print_test_header("Contact Creation (new)", "👤");

        let public_key = create_test_public_key();
        let name = "Alice Smith";

        print_info("Testing basic contact creation");
        println!("{GREEN}Name: \"{name}\"{RESET}");
        print_hex_data("Public Key", &public_key.to_bytes(), MAGENTA);

        let contact = Contact::new(name, &public_key);

        assert_eq!(contact.name, name);
        assert_eq!(contact.public_key, public_key.to_bytes());
        assert_eq!(contact.nickname, None);
        assert_eq!(contact.email, None);
        assert!(!contact.verified);
        assert!(!contact.blocked);
        assert_eq!(contact.last_seen, None);
        assert!(contact.id().is_none());
        assert!(contact.created_at > 0);

        print_success("✓ Contact created with correct default values");
        print_info(&format!("Created at timestamp: {}", contact.created_at));

        println!("{BOLD}🎉 Contact creation test PASSED!{RESET}\n");
    }

    #[test]
    #[serial(contact)]
    fn test_contact_creation_builder_pattern() {
        print_test_header("Contact Creation (Struct Literal)", "🏗️");

        let public_key = create_test_public_key();
        let id = Some("contact_123".to_string());
        let name = "Bob Jones".to_string();
        let nickname = Some("Bobby".to_string());
        let email = Some("bob@example.com".to_string());
        let verified = true;
        let blocked = false;
        let created_at = 1234567890u64;
        let last_seen = Some(1234567900u64);

        print_info("Testing contact creation using struct literals with Default");

        let contact = Contact {
            id: id.clone(),
            name: name.clone(),
            public_key: public_key.to_bytes(),
            nickname: nickname.clone(),
            email: email.clone(),
            verified,
            blocked,
            created_at,
            last_seen,
        };

        assert_eq!(contact.id(), id.as_deref());
        assert_eq!(contact.name, name);
        assert_eq!(contact.public_key, public_key.to_bytes());
        assert_eq!(contact.nickname, nickname);
        assert_eq!(contact.email, email);
        assert_eq!(contact.verified, verified);
        assert_eq!(contact.blocked, blocked);
        assert_eq!(contact.created_at, created_at);
        assert_eq!(contact.last_seen, last_seen);

        print_success("✓ Contact created with all specified values using struct literal");

        println!("{BOLD}🎉 Struct literal creation test PASSED!{RESET}\n");
    }

    #[test]
    #[serial(contact)]
    fn test_contact_public_key_methods() {
        print_test_header("Public Key Methods", "🔑");

        let public_key = create_test_public_key();
        let contact = Contact::new("Test User", &public_key);

        print_info("Testing public key accessor methods");
        print_hex_data("Original Key", &public_key.to_bytes(), MAGENTA);

        let retrieved_key = contact.public_key();
        let retrieved_bytes = contact.public_key_bytes();

        assert_eq!(retrieved_key.to_bytes(), public_key.to_bytes());
        assert_eq!(retrieved_bytes, public_key.to_bytes());

        print_hex_data("Retrieved Key", &retrieved_key.to_bytes(), CYAN);
        print_hex_data("Retrieved Bytes", &retrieved_bytes, CYAN);

        print_success("✓ Public key methods return correct values");

        println!("{BOLD}🎉 Public key methods test PASSED!{RESET}\n");
    }

    #[test]
    #[serial(contact)]
    fn test_contact_setters() {
        print_test_header("Contact Setters", "✏️");

        let public_key = create_test_public_key();
        let mut contact = Contact::new("Test User", &public_key);

        print_info("Testing all setter methods");

        // Test nickname setter
        contact.set_nickname(Some("Tester".to_string()));
        assert_eq!(contact.nickname, Some("Tester".to_string()));
        print_success("✓ Nickname setter working");

        contact.set_nickname(None);
        assert_eq!(contact.nickname, None);
        print_success("✓ Nickname can be cleared");

        // Test email setter
        contact.set_email(Some("test@example.com".to_string()));
        assert_eq!(contact.email, Some("test@example.com".to_string()));
        print_success("✓ Email setter working");

        contact.set_email(None);
        assert_eq!(contact.email, None);
        print_success("✓ Email can be cleared");

        // Test verified setter
        contact.set_verified(true);
        assert!(contact.verified);
        print_success("✓ Verified setter working (true)");

        contact.set_verified(false);
        assert!(!contact.verified);
        print_success("✓ Verified setter working (false)");

        // Test blocked setter
        contact.set_blocked(true);
        assert!(contact.blocked);
        print_success("✓ Blocked setter working (true)");

        contact.set_blocked(false);
        assert!(!contact.blocked);
        print_success("✓ Blocked setter working (false)");

        println!("{BOLD}🎉 Contact setters test PASSED!{RESET}\n");
    }

    #[test]
    #[serial(contact)]
    fn test_update_last_seen() {
        print_test_header("Update Last Seen", "⏰");

        let public_key = create_test_public_key();
        let mut contact = Contact::new("Test User", &public_key);

        print_info("Testing last seen timestamp updates");

        // Initially no last seen
        assert_eq!(contact.last_seen, None);
        print_info("Initial last_seen: None");

        let before_update = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Small delay to ensure timestamp difference
        std::thread::sleep(std::time::Duration::from_millis(1));

        contact.update_last_seen();

        let after_update = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        assert!(contact.last_seen.is_some());
        let last_seen = contact.last_seen.unwrap();
        assert!(last_seen >= before_update && last_seen <= after_update);

        print_success("✓ Last seen timestamp updated correctly");
        print_info(&format!("Last seen: {last_seen}"));

        println!("{BOLD}🎉 Update last seen test PASSED!{RESET}\n");
    }

    #[test]
    #[serial(contact)]
    fn test_contact_serialization() {
        print_test_header("Contact Serialization", "📦");

        let public_key = create_test_public_key();
        let mut contact = Contact::new("Serialize Test", &public_key);
        contact.set_nickname(Some("Tester".to_string()));
        contact.set_email(Some("test@example.com".to_string()));
        contact.set_verified(true);
        contact.update_last_seen();

        print_info("Testing JSON serialization/deserialization");

        // Serialize
        let json = contact.to_json().unwrap();
        print_success("✓ Contact serialized to JSON");
        print_str_data("JSON", &json, CYAN);

        // Deserialize
        let deserialized = Contact::from_json(&json).unwrap();
        print_success("✓ Contact deserialized from JSON");

        // Verify all fields match
        assert_eq!(deserialized.name, contact.name);
        assert_eq!(deserialized.public_key, contact.public_key);
        assert_eq!(deserialized.nickname, contact.nickname);
        assert_eq!(deserialized.email, contact.email);
        assert_eq!(deserialized.verified, contact.verified);
        assert_eq!(deserialized.blocked, contact.blocked);
        assert_eq!(deserialized.created_at, contact.created_at);
        assert_eq!(deserialized.last_seen, contact.last_seen);

        print_success("✓ All fields preserved through serialization");

        println!("{BOLD}🎉 Contact serialization test PASSED!{RESET}\n");
    }

    #[test]
    #[serial(contact)]
    fn test_display_name() {
        print_test_header("Display Name", "🏷️");

        let public_key = create_test_public_key();

        print_info("Testing display name logic");

        // Without nickname, should return name
        let contact1 = Contact::new("John Doe", &public_key);
        assert_eq!(contact1.display_name(), "John Doe");
        print_success("✓ Display name without nickname uses name");

        // With nickname, should return nickname
        let mut contact2 = Contact::new("Jane Smith", &public_key);
        contact2.set_nickname(Some("Janey".to_string()));
        assert_eq!(contact2.display_name(), "Janey");
        print_success("✓ Display name with nickname uses nickname");

        println!(
            "{GREEN}Display name (no nickname): \"{}\"{RESET}",
            contact1.display_name()
        );
        println!(
            "{CYAN}Display name (with nickname): \"{}\"{RESET}",
            contact2.display_name()
        );

        println!("{BOLD}🎉 Display name test PASSED!{RESET}\n");
    }

    #[test]
    #[serial(contact)]
    fn test_find_by_public_key() {
        print_test_header("Find by Public Key", "🔍");

        let public_key1 = create_test_public_key();
        let public_key2 = create_test_public_key();
        let public_key3 = create_test_public_key();

        let contact1 = Contact::new("Alice", &public_key1);
        let contact2 = Contact::new("Bob", &public_key2);
        let contacts = vec![contact1, contact2];

        print_info("Testing contact search by public key");
        print_info(&format!("Contact list has {} contacts", contacts.len()));

        // Find existing contact
        let found = Contact::find_by_public_key(&contacts, &public_key1.to_bytes());
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Alice");
        print_success("✓ Found existing contact by public key");

        // Try to find non-existing contact
        let not_found = Contact::find_by_public_key(&contacts, &public_key3.to_bytes());
        assert!(not_found.is_none());
        print_success("✓ Correctly returned None for non-existing key");

        println!("{BOLD}🎉 Find by public key test PASSED!{RESET}\n");
    }

    #[test]
    #[serial(contact)]
    fn test_find_by_public_keys() {
        print_test_header("Find by Public Keys", "🔍");

        let public_key1 = create_test_public_key();
        let public_key2 = create_test_public_key();
        let public_key3 = create_test_public_key();
        let public_key4 = create_test_public_key();

        let contact1 = Contact::new("Alice", &public_key1);
        let contact2 = Contact::new("Bob", &public_key2);
        let contact3 = Contact::new("Charlie", &public_key3);
        let contacts = vec![contact1, contact2, contact3];

        print_info("Testing batch contact search by public keys");

        // Create set of keys to search for
        let mut search_keys = HashSet::new();
        search_keys.insert(public_key1.to_bytes());
        search_keys.insert(public_key3.to_bytes());
        search_keys.insert(public_key4.to_bytes()); // This one won't be found

        let found_contacts = Contact::find_by_public_keys(&contacts, &search_keys);

        assert_eq!(found_contacts.len(), 2); // Only Alice and Charlie should be found
        let names: Vec<&str> = found_contacts.iter().map(|c| c.name.as_str()).collect();
        assert!(names.contains(&"Alice"));
        assert!(names.contains(&"Charlie"));
        assert!(!names.contains(&"Bob")); // Bob's key wasn't in search set

        print_success(&format!(
            "✓ Found {} contacts from {} search keys",
            found_contacts.len(),
            search_keys.len()
        ));
        print_info("Found contacts: Alice, Charlie (Bob excluded as expected)");

        println!("{BOLD}🎉 Find by public keys test PASSED!{RESET}\n");
    }

    #[test]
    #[serial(contact)]
    fn test_filter_non_blocked() {
        print_test_header("Filter Non-Blocked", "🚫");

        let public_key1 = create_test_public_key();
        let public_key2 = create_test_public_key();
        let public_key3 = create_test_public_key();

        let mut contact1 = Contact::new("Alice", &public_key1);
        let mut contact2 = Contact::new("Bob", &public_key2);
        let contact3 = Contact::new("Charlie", &public_key3);

        // Block Alice and Bob
        contact1.set_blocked(true);
        contact2.set_blocked(true);
        // Charlie remains unblocked

        let contacts = vec![contact1, contact2, contact3];

        print_info("Testing non-blocked contact filtering");
        print_info("Alice: blocked, Bob: blocked, Charlie: not blocked");

        let non_blocked = Contact::filter_non_blocked(&contacts);

        assert_eq!(non_blocked.len(), 1);
        assert_eq!(non_blocked[0].name, "Charlie");

        print_success("✓ Only non-blocked contact returned");
        print_info(&format!("Non-blocked contacts: {}", non_blocked[0].name));

        println!("{BOLD}🎉 Filter non-blocked test PASSED!{RESET}\n");
    }

    #[test]
    #[serial(contact)]
    fn test_filter_verified() {
        print_test_header("Filter Verified", "✅");

        let public_key1 = create_test_public_key();
        let public_key2 = create_test_public_key();
        let public_key3 = create_test_public_key();

        let mut contact1 = Contact::new("Alice", &public_key1);
        let contact2 = Contact::new("Bob", &public_key2);
        let mut contact3 = Contact::new("Charlie", &public_key3);

        // Verify Alice and Charlie
        contact1.set_verified(true);
        contact3.set_verified(true);
        // Bob remains unverified

        let contacts = vec![contact1, contact2, contact3];

        print_info("Testing verified contact filtering");
        print_info("Alice: verified, Bob: not verified, Charlie: verified");

        let verified = Contact::filter_verified(&contacts);

        assert_eq!(verified.len(), 2);
        let names: Vec<&str> = verified.iter().map(|c| c.name.as_str()).collect();
        assert!(names.contains(&"Alice"));
        assert!(names.contains(&"Charlie"));
        assert!(!names.contains(&"Bob"));

        print_success("✓ Only verified contacts returned");
        print_info("Verified contacts: Alice, Charlie");

        println!("{BOLD}🎉 Filter verified test PASSED!{RESET}\n");
    }

    #[test]
    #[serial(contact)]
    fn test_entity_trait() {
        print_test_header("Entity Trait Implementation", "🏢");

        let public_key = create_test_public_key();
        let mut contact = Contact::new("Entity Test", &public_key);

        print_info("Testing Entity trait implementation");

        // Initially no ID
        assert!(contact.id().is_none());
        print_success("✓ New contact has no ID");

        // Set ID
        let test_id = "contact_12345".to_string();
        contact.set_id(test_id.clone());
        assert_eq!(contact.id(), Some(test_id.as_str()));
        print_success("✓ ID set correctly");

        // Check key prefix
        assert_eq!(Contact::key_prefix(), "contact");
        print_success("✓ Key prefix is correct");

        print_info(&format!("Contact ID: {:?}", contact.id()));
        print_info(&format!("Key prefix: {}", Contact::key_prefix()));

        println!("{BOLD}🎉 Entity trait test PASSED!{RESET}\n");
    }

    #[test]
    #[serial(contact)]
    fn test_contact_edge_cases() {
        print_test_header("Contact Edge Cases", "⚠️");

        let public_key = create_test_public_key();

        print_info("Testing edge cases and boundary conditions");

        // Empty name
        let contact_empty_name = Contact::new("", &public_key);
        assert_eq!(contact_empty_name.name, "");
        print_success("✓ Empty name handled correctly");

        // Very long name
        let long_name = "A".repeat(1000);
        let contact_long_name = Contact::new(&long_name, &public_key);
        assert_eq!(contact_long_name.name.len(), 1000);
        print_success("✓ Long name handled correctly");

        // Unicode name
        let unicode_name = "Alice 👤 测试 🎉";
        let contact_unicode = Contact::new(unicode_name, &public_key);
        assert_eq!(contact_unicode.name, unicode_name);
        print_success("✓ Unicode name handled correctly");

        // Empty contacts list
        let empty_contacts: Vec<Contact> = vec![];
        let found = Contact::find_by_public_key(&empty_contacts, &public_key.to_bytes());
        assert!(found.is_none());
        print_success("✓ Search in empty list handled correctly");

        let non_blocked = Contact::filter_non_blocked(&empty_contacts);
        assert_eq!(non_blocked.len(), 0);
        print_success("✓ Filter on empty list handled correctly");

        println!("{BOLD}🎉 Edge cases test PASSED!{RESET}\n");
    }
}
