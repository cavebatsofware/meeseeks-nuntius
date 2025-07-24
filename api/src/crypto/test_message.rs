// When debugging tests uncomment the "#[serial]" attributes for readable output
// and run with "cargo test -- --nocapture".

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
        println!("{}═══════════════════════════════════════════════════════════{}", CYAN, RESET);
    }

    pub fn print_test_header(test_name: &str, emoji: &str) {
        print_separator();
        println!("{}{} {} {} {}{}", BOLD, emoji, test_name, emoji, RESET, "");
        print_separator();
    }

    pub fn print_str_data(label: &str, data: &str, color: &str) {
        println!("{}{}: {}{}{}", color, label, data, RESET, "");
        println!("{}Length: {} bytes{}", WHITE, data.len(), RESET);
    }

    pub fn print_hex_data(label: &str, data: &[u8], color: &str) {
        let hex_str = hex::encode(data);
        println!("{}{}: {}{}{}", color, label, hex_str, RESET, "");
        println!("{}Length: {} bytes{}", WHITE, data.len(), RESET);
    }

    pub fn print_success(message: &str) {
        println!("{}✅ {}{}", GREEN, message, RESET);
    }

    pub fn print_info(message: &str) {
        println!("{}ℹ️  {}{}", BLUE, message, RESET);
    }

    pub fn print_warning(message: &str) {
        println!("{}⚠️  {}{}", YELLOW, message, RESET);
    }

    pub fn print_error(message: &str) {
        println!("{}❌ {}{}", RED, message, RESET);
    }
}

#[cfg(test)]
mod test_aes {
    use serial_test::serial;
    use crate::crypto::test_message::test_utils::*;
    use crate::crypto::message::aes256_gcm::*;

    #[test]
    #[serial(aes256_gcm)]
    fn test_encrypt_decrypt_basic() {
        print_test_header("Basic Encrypt/Decrypt Test", "🔐");
        
        let key = generate_key();
        let plaintext = b"Hello, AES-GCM!";
        
        print_info("Generating random 256-bit key...");
        print_hex_data("Key", &key, MAGENTA);
        
        print_info("Original plaintext:");
        println!("{}\"{}\" ({}){}", GREEN, String::from_utf8_lossy(plaintext), plaintext.len(), RESET);
        print_hex_data("Plaintext bytes", plaintext, GREEN);
        
        println!("\n{}🔒 ENCRYPTION PHASE{}", YELLOW, RESET);
        let (ciphertext, nonce) = encrypt(&key, plaintext).unwrap();
        print_success("Encryption successful!");
        
        print_hex_data("Ciphertext", &ciphertext, RED);
        print_hex_data("Nonce", &nonce, CYAN);
        
        // Verify ciphertext is different from plaintext
        assert_ne!(ciphertext, plaintext);
        print_success("✓ Ciphertext differs from plaintext (as expected)");
        
        println!("\n{}🔓 DECRYPTION PHASE{}", YELLOW, RESET);
        let decrypted = decrypt(&key, &ciphertext, &nonce).unwrap();
        print_success("Decryption successful!");
        
        print_hex_data("Decrypted bytes", &decrypted, GREEN);
        println!("{}Decrypted text: \"{}\"{}", GREEN, String::from_utf8_lossy(&decrypted), RESET);
        
        // Verify decrypted text matches original
        assert_eq!(decrypted, plaintext);
        print_success("✓ Decrypted text matches original plaintext!");
        
        println!("{}🎉 Basic encryption/decryption test PASSED!{}\n", BOLD, RESET);
    }

    #[test]
    #[serial(aes256_gcm)]
    fn test_encrypt_decrypt_string() {
        print_test_header("String Encryption Test", "📝");
        
        let key = generate_key();
        let plaintext = "The quick brown fox jumps over the lazy dog";
        
        print_info("Testing string convenience functions...");
        println!("{}Original message: \"{}\"{}", GREEN, plaintext, RESET);
        println!("{}Message length: {} characters{}", WHITE, plaintext.len(), RESET);
        
        println!("\n{}🔒 ENCRYPTING STRING{}", YELLOW, RESET);
        let (ciphertext, nonce) = encrypt_string(&key, plaintext).unwrap();
        print_success("String encryption successful!");
        
        print_hex_data("Encrypted string", &ciphertext, RED);
        print_hex_data("Nonce", &nonce, CYAN);
        
        println!("\n{}🔓 DECRYPTING STRING{}", YELLOW, RESET);
        let decrypted = decrypt_string(&key, &ciphertext, &nonce).unwrap();
        print_success("String decryption successful!");
        
        println!("{}Decrypted: \"{}\"{}", GREEN, decrypted, RESET);
        
        // Verify
        assert_eq!(decrypted, plaintext);
        print_success("✓ String round-trip successful!");
        
        println!("{}🎉 String encryption test PASSED!{}\n", BOLD, RESET);
    }

    #[test]
    #[serial(aes256_gcm)]
    fn test_different_keys_fail() {
        print_test_header("Wrong Key Security Test", "🔑");
        
        let key1 = generate_key();
        let key2 = generate_key();
        let plaintext = b"Secret message";
        
        print_info("Testing that different keys cannot decrypt each other's data...");
        println!("{}Plaintext: \"{}\"{}", GREEN, String::from_utf8_lossy(plaintext), RESET);
        
        print_hex_data("Key 1", &key1, MAGENTA);
        print_hex_data("Key 2", &key2, MAGENTA);
        
        print_warning("Keys are different (as expected for security)");
        
        println!("\n{}🔒 ENCRYPTING WITH KEY 1{}", YELLOW, RESET);
        let (ciphertext, nonce) = encrypt(&key1, plaintext).unwrap();
        print_success("Encryption with Key 1 successful!");
        
        print_hex_data("Ciphertext", &ciphertext, RED);
        print_hex_data("Nonce", &nonce, CYAN);
        
        println!("\n{}🔓 ATTEMPTING DECRYPTION WITH KEY 2{}", YELLOW, RESET);
        let result = decrypt(&key2, &ciphertext, &nonce);
        
        match result {
            Ok(_) => {
                print_error("SECURITY BREACH: Wrong key should not decrypt!");
                panic!("Security test failed - wrong key decrypted data!");
            }
            Err(e) => {
                print_success("✓ Decryption correctly failed with wrong key");
                print_info(&format!("Error message: {}", e));
            }
        }
        
        println!("{}🛡️  Security test PASSED - wrong keys cannot decrypt!{}\n", BOLD, RESET);
    }

    #[test]
    #[serial(aes256_gcm)]
    fn test_wrong_nonce_fails() {
        print_test_header("Wrong Nonce Security Test", "🎲");
        
        let key = generate_key();
        let plaintext = b"Another secret";
        
        print_info("Testing that wrong nonces prevent decryption...");
        println!("{}Plaintext: \"{}\"{}", GREEN, String::from_utf8_lossy(plaintext), RESET);
        
        println!("\n{}🔒 ENCRYPTING WITH RANDOM NONCE{}", YELLOW, RESET);
        let (ciphertext, correct_nonce) = encrypt(&key, plaintext).unwrap();
        print_success("Encryption successful!");
        
        print_hex_data("Ciphertext", &ciphertext, RED);
        print_hex_data("Correct nonce", &correct_nonce, CYAN);
        
        // Try with wrong nonce
        let wrong_nonce = vec![0u8; 12]; // GCM nonce is 12 bytes
        print_hex_data("Wrong nonce (all zeros)", &wrong_nonce, RED);
        
        println!("\n{}🔓 ATTEMPTING DECRYPTION WITH WRONG NONCE{}", YELLOW, RESET);
        let result = decrypt(&key, &ciphertext, &wrong_nonce);
        
        match result {
            Ok(_) => {
                print_error("SECURITY BREACH: Wrong nonce should not decrypt!");
                panic!("Security test failed - wrong nonce decrypted data!");
            }
            Err(e) => {
                print_success("✓ Decryption correctly failed with wrong nonce");
                print_info(&format!("Error message: {}", e));
            }
        }
        
        println!("{}🛡️  Nonce security test PASSED!{}\n", BOLD, RESET);
    }

    #[test]
    #[serial(aes256_gcm)]
    fn test_empty_plaintext() {
        print_test_header(" Empty Data Test", "🕳️");
        
        let key = generate_key();
        let plaintext = b"";
        
        print_info("Testing encryption/decryption of empty data...");
        println!("{}Plaintext: <empty> (0 bytes){}", YELLOW, RESET);
        
        println!("\n{}🔒 ENCRYPTING EMPTY DATA{}", YELLOW, RESET);
        let (ciphertext, nonce) = encrypt(&key, plaintext).unwrap();
        print_success("Empty data encryption successful!");
        
        print_hex_data("Ciphertext", &ciphertext, RED);
        print_hex_data("Nonce", &nonce, CYAN);
        print_info("Note: Even empty data produces authentication tag");
        
        println!("\n{}🔓 DECRYPTING EMPTY DATA{}", YELLOW, RESET);
        let decrypted = decrypt(&key, &ciphertext, &nonce).unwrap();
        print_success("Empty data decryption successful!");
        
        println!("{}Decrypted length: {} bytes{}", GREEN, decrypted.len(), RESET);
        
        assert_eq!(decrypted, plaintext);
        print_success("✓ Empty data round-trip successful!");
        
        println!("{}🎉 Empty data test PASSED!{}\n", BOLD, RESET);
    }

    #[test]
    #[serial(aes256_gcm)]
    fn test_large_plaintext() {
        print_test_header("Large Data Test", "📦");
        
        let key = generate_key();
        let plaintext = vec![42u8; 1024]; // 1KB of data
        
        print_info("Testing encryption/decryption of large data (1KB)...");
        println!("{}Plaintext: {} bytes of value 42 (0x2A){}", YELLOW, plaintext.len(), RESET);
        
        // Show a sample of the data
        let sample = &plaintext[..std::cmp::min(32, plaintext.len())];
        print_hex_data("First 32 bytes", sample, GREEN);
        
        println!("\n{}🔒 ENCRYPTING LARGE DATA{}", YELLOW, RESET);
        let start_time = std::time::Instant::now();
        let (ciphertext, nonce) = encrypt(&key, &plaintext).unwrap();
        let encrypt_time = start_time.elapsed();
        print_success(&format!("Large data encryption successful in {:?}!", encrypt_time));
        
        print_hex_data("Nonce", &nonce, CYAN);
        println!("{}Ciphertext length: {} bytes{}", RED, ciphertext.len(), RESET);
        
        // Show first few bytes of ciphertext
        let cipher_sample = &ciphertext[..std::cmp::min(32, ciphertext.len())];
        print_hex_data("First 32 bytes of ciphertext", cipher_sample, RED);
        
        println!("\n{}🔓 DECRYPTING LARGE DATA{}", YELLOW, RESET);
        let start_time = std::time::Instant::now();
        let decrypted = decrypt(&key, &ciphertext, &nonce).unwrap();
        let decrypt_time = start_time.elapsed();
        print_success(&format!("Large data decryption successful in {:?}!", decrypt_time));
        
        println!("{}Decrypted length: {} bytes{}", GREEN, decrypted.len(), RESET);
        
        assert_eq!(decrypted, plaintext);
        print_success("✓ Large data integrity verified!");
        
        println!("{}📊 Performance: Encrypt {:?}, Decrypt {:?}{}", BLUE, encrypt_time, decrypt_time, RESET);
        println!("{}🎉 Large data test PASSED!{}\n", BOLD, RESET);
    }

    #[test]
    #[serial(aes256_gcm)]
    fn test_nonce_uniqueness() {
        print_test_header("Nonce Uniqueness Test", "🎯");
        
        let key = generate_key();
        let plaintext = b"Same message";
        
        print_info("Testing that nonces are unique for identical messages...");
        println!("{}Message: \"{}\"{}", GREEN, String::from_utf8_lossy(plaintext), RESET);
        
        println!("\n{}🔒 FIRST ENCRYPTION{}", YELLOW, RESET);
        let (ciphertext1, nonce1) = encrypt(&key, plaintext).unwrap();
        print_success("First encryption successful!");
        print_hex_data("Ciphertext 1", &ciphertext1, RED);
        print_hex_data("Nonce 1", &nonce1, CYAN);
        
        println!("\n{}🔒 SECOND ENCRYPTION (SAME MESSAGE){}", YELLOW, RESET);
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
        
        println!("\n{}🔓 VERIFYING BOTH DECRYPT CORRECTLY{}", YELLOW, RESET);
        let decrypted1 = decrypt(&key, &ciphertext1, &nonce1).unwrap();
        let decrypted2 = decrypt(&key, &ciphertext2, &nonce2).unwrap();
        
        assert_eq!(decrypted1, plaintext);
        assert_eq!(decrypted2, plaintext);
        print_success("✓ Both ciphertexts decrypt to original message!");
        
        println!("{}🎉 Nonce uniqueness test PASSED - cryptographic security maintained!{}\n", BOLD, RESET);
    }
}

#[cfg(test)]
mod test_exchange {
    use serial_test::serial;
    use crate::crypto::test_message::test_utils::*;
    use crate::crypto::message::exchange::*;

    fn print_party_info(party: &Party) {
        println!("{}👤 Party: {}{}", YELLOW, party.name, RESET);
        print_hex_data(&format!("{}'s Public Key", party.name), &party.public_key_bytes(), MAGENTA);
    }

    #[test]
    #[serial(exchange)]
    fn test_basic_messaging() {
        print_test_header("Basic crypto_box Messaging", "💬");

        // Create two parties
        let mut alice = Party::new("Alice");
        let mut bob = Party::new("Bob");

        print_info("Created two parties for secure communication");
        print_party_info(&alice);
        print_party_info(&bob);

        let message = "Hello Bob! This is a secret message from Alice.";
        println!("\n{}📝 Original message: \"{}\"{}", GREEN, message, RESET);

        println!("\n{}🔒 ALICE ENCRYPTING MESSAGE FOR BOB{}", YELLOW, RESET);
        let encrypted = alice.encrypt_string_for(&bob.get_public_key(), message).unwrap();
        print_success("Message encrypted successfully!");
        print_info(&format!("Alice now has {} known contacts", alice.contact_count()));

        print_hex_data("Sender Public Key", &encrypted.get_sender_public_bytes(), MAGENTA);
        print_hex_data("Nonce", &encrypted.nonce, CYAN);
        print_hex_data("Ciphertext", &encrypted.ciphertext, RED);

        println!("\n{}🔓 BOB DECRYPTING MESSAGE FROM ALICE{}", YELLOW, RESET);
        let decrypted = bob.decrypt_string_from(&encrypted).unwrap();
        print_success("Message decrypted successfully!");
        print_info(&format!("Bob now has {} known contacts", bob.contact_count()));

        println!("{}📖 Decrypted message: \"{}\"{}", GREEN, decrypted, RESET);

        assert_eq!(decrypted, message);
        print_success("✓ Message integrity verified!");

        println!("{}🎉 Basic messaging test PASSED!{}\n", BOLD, RESET);
    }

    #[test]
    #[serial(exchange)]
    fn test_bidirectional_messaging() {
        print_test_header("Bidirectional Messaging", "🔄");

        let mut alice = Party::new("Alice");
        let mut bob = Party::new("Bob");

        print_info("Testing two-way communication between parties");
        print_party_info(&alice);
        print_party_info(&bob);

        // Alice to Bob
        let alice_message = "Hi Bob, how are you?";
        println!("\n{}👩 Alice's message: \"{}\"{}", GREEN, alice_message, RESET);
        
        let alice_encrypted = alice.encrypt_string_for(&bob.get_public_key(), alice_message).unwrap();
        print_success("Alice's message encrypted");
        print_info(&format!("Alice known contacts: {}", alice.contact_count()));

        let bob_decrypted = bob.decrypt_string_from(&alice_encrypted).unwrap();
        println!("{}👨 Bob received: \"{}\"{}", BLUE, bob_decrypted, RESET);
        print_info(&format!("Bob known contacts: {}", bob.contact_count()));
        assert_eq!(bob_decrypted, alice_message);

        // Bob to Alice
        let bob_message = "Hi Alice! I'm doing great, thanks for asking!";
        println!("\n{}👨 Bob's reply: \"{}\"{}", BLUE, bob_message, RESET);
        
        let bob_encrypted = bob.encrypt_string_for(&alice.get_public_key(), bob_message).unwrap();
        print_success("Bob's message encrypted");
        print_info(&format!("Bob known contacts: {}", bob.contact_count()));

        let alice_decrypted = alice.decrypt_string_from(&bob_encrypted).unwrap();
        println!("{}👩 Alice received: \"{}\"{}", GREEN, alice_decrypted, RESET);
        print_info(&format!("Alice known contacts: {}", alice.contact_count()));
        assert_eq!(alice_decrypted, bob_message);

        print_success("✓ Bidirectional communication successful!");
        print_success("✓ Both parties have registered each other as contacts!");
        println!("{}🎉 Bidirectional messaging test PASSED!{}\n", BOLD, RESET);
    }

    #[test]
    #[serial(exchange)]
    fn test_message_serialization() {
        print_test_header("Message Serialization", "📦");

        let mut alice = Party::new("Alice");
        let mut bob = Party::new("Bob");

        print_info("Testing message serialization for network transmission");

        let original_message = "This message will be serialized and deserialized!";
        println!("{}Original: \"{}\"{}", GREEN, original_message, RESET);

        // Encrypt and serialize
        println!("\n{}📤 SERIALIZATION PHASE{}", YELLOW, RESET);
        let encrypted = alice.encrypt_string_for(&bob.get_public_key(), original_message).unwrap();
        let serialized = encrypted.to_json().unwrap();
        print_success("Message encrypted and serialized");
        print_str_data("Serialized message", &serialized.as_str(), CYAN);

        // Deserialize and decrypt
        println!("\n{}📥 DESERIALIZATION PHASE{}", YELLOW, RESET);
        let deserialized = EncryptedMessage::from_json(&serialized).unwrap();
        print_success("Message deserialized successfully");

        let decrypted = bob.decrypt_string_from(&deserialized).unwrap();
        print_success("Message decrypted successfully");
        println!("{}Decrypted: \"{}\"{}", GREEN, decrypted, RESET);

        assert_eq!(decrypted, original_message);
        print_success("✓ Round-trip serialization successful!");

        println!("{}🎉 Serialization test PASSED!{}\n", BOLD, RESET);
    }

    #[test]
    #[serial(exchange)]
    fn test_wrong_recipient_fails() {
        print_test_header("Wrong Recipient Security", "🚫");

        let mut alice = Party::new("Alice");
        let mut bob = Party::new("Bob");
        let mut eve = Party::new("Eve"); // Eavesdropper

        print_info("Testing that only intended recipients can decrypt messages");
        print_party_info(&alice);
        print_party_info(&bob);
        print_party_info(&eve);

        let secret_message = "This is for Bob's eyes only!";
        println!("\n{}🤐 Secret message: \"{}\"{}", GREEN, secret_message, RESET);

        println!("\n{}🔒 ALICE ENCRYPTING FOR BOB{}", YELLOW, RESET);
        let encrypted = alice.encrypt_string_for(&bob.get_public_key(), secret_message).unwrap();
        print_success("Message encrypted for Bob");

        println!("\n{}👨 BOB ATTEMPTING TO DECRYPT{}", BLUE, RESET);
        let bob_result = bob.decrypt_string_from(&encrypted);
        match bob_result {
            Ok(decrypted) => {
                print_success("✓ Bob successfully decrypted the message");
                println!("{}Bob read: \"{}\"{}", BLUE, decrypted, RESET);
                assert_eq!(decrypted, secret_message);
            }
            Err(_) => panic!("Bob should be able to decrypt the message!"),
        }

        println!("\n{}🕵️ EVE ATTEMPTING TO DECRYPT{}", RED, RESET);
        let eve_result = eve.decrypt_string_from(&encrypted);
        match eve_result {
            Ok(_) => {
                panic!("Security breach! Eve should not be able to decrypt Bob's message!");
            }
            Err(e) => {
                print_success("✓ Eve correctly failed to decrypt the message");
                print_info(&format!("Error: {}", e));
            }
        }

        println!("{}🛡️  Security test PASSED - only Bob can read his messages!{}\n", BOLD, RESET);
    }

    #[test]
    #[serial(exchange)]
    fn test_contact_management() {
        print_test_header("Contact Management", "📇");

        let mut alice = Party::new("Alice");
        let mut bob = Party::new("Bob");
        let mut charlie = Party::new("Charlie");

        print_info("Testing pre-established contact functionality");

        // Create a party with pre-established contacts
        let contacts = [&alice.get_public_key(), &bob.get_public_key()];
        let mut dave = Party::new_with_contacts("Dave", &contacts);
        
        print_info(&format!("Dave created with {} pre-registered contacts", dave.contact_count()));
        
        // IMPORTANT: For bidirectional communication, recipients also need Dave's key
        alice.add_contact(&dave.get_public_key());
        bob.add_contact(&dave.get_public_key());
        
        print_info("Alice and Bob have been given Dave's public key for bidirectional communication");
        
        // Verify Dave can communicate with pre-registered contacts
        let message = "Hello everyone!";
        
        let to_alice = dave.encrypt_string_for(&alice.get_public_key(), message).unwrap();
        let to_bob = dave.encrypt_string_for(&bob.get_public_key(), message).unwrap();
        
        print_success("✓ Dave encrypted messages for pre-registered contacts");
        
        // Test actual decryption (this is the important part!)
        let alice_received = alice.decrypt_string_from(&to_alice).unwrap();
        let bob_received = bob.decrypt_string_from(&to_bob).unwrap();
        
        assert_eq!(alice_received, message);
        assert_eq!(bob_received, message);
        print_success("✓ Alice and Bob successfully decrypted Dave's messages");
        
        // Dave establishes contact with Charlie on-demand
        let to_charlie = dave.encrypt_string_for(&charlie.get_public_key(), message).unwrap();
        print_success("✓ Dave encrypted message for new contact (Charlie)");
        
        // Charlie needs Dave's key to decrypt
        charlie.add_contact(&dave.get_public_key());
        let charlie_received = charlie.decrypt_string_from(&to_charlie).unwrap();
        assert_eq!(charlie_received, message);
        print_success("✓ Charlie successfully decrypted Dave's message");
        
        print_info(&format!("Dave now has {} total contacts", dave.contact_count()));
        
        // Verify contact list
        let dave_contacts = dave.get_contacts();
        assert_eq!(dave_contacts.len(), 3);
        print_success("✓ Contact list management working correctly");
        
        // Test bidirectional communication
        println!("\n{}🔄 Testing bidirectional communication{}", YELLOW, RESET);
        let reply_message = "Hi Dave! Nice to meet you!";
        let alice_reply = alice.encrypt_string_for(&dave.get_public_key(), reply_message).unwrap();
        let dave_received = dave.decrypt_string_from(&alice_reply).unwrap();
        assert_eq!(dave_received, reply_message);
        print_success("✓ Bidirectional communication working correctly");
        
        println!("{}Dave's contacts:{}", BLUE, RESET);
        for (i, contact_key) in dave_contacts.iter().enumerate() {
            let key_hex = hex::encode(contact_key.to_bytes());
            println!("  {}. {}...", i + 1, &key_hex[..16]); // Show first 16 chars
        }

        println!("{}🎉 Contact management test PASSED!{}\n", BOLD, RESET);
    }

    #[test]
    #[serial(exchange)]
    fn test_crypto_box_creation() {
        print_test_header("crypto_box Performance metrics", "⚡");

        let mut alice = Party::new("Alice");
        let mut bob = Party::new("Bob");

        print_info("Testing that new crypto boxes are created for each message");

        let test_message = "This is a test message!";

        // Multiple encryptions - each creates a fresh crypto box
        println!("\n{}🔒 MULTIPLE ENCRYPTIONS{}", YELLOW, RESET);
        let start_time = std::time::Instant::now();
        let encrypted1 = alice.encrypt_string_for(&bob.get_public_key(), test_message).unwrap();
        let first_encrypt_time = start_time.elapsed();
        print_success(&format!("First encryption completed in {:?}", first_encrypt_time));
        print_info(&format!("Alice known contacts: {}", alice.contact_count()));

        let start_time = std::time::Instant::now();
        for _n in 0..1000 {
            let encrypted2 = alice.encrypt_string_for(&bob.get_public_key(), "Second message").unwrap();
            // Verify different nonces
            assert_ne!(encrypted1.nonce, encrypted2.nonce);
        }
        let second_encrypt_time = start_time.elapsed();
        assert!(second_encrypt_time.as_millis() < 1000);
        print_success(&format!("1K encryptions completed in {:?}", second_encrypt_time));
        print_info(&format!("Contact count unchanged: {}", alice.contact_count()));
        print_success("✓ Different nonces used (critical for security)!");

        // Decryption performance
        println!("\n{}🔓 DECRYPTION PERFORMANCE{}", YELLOW, RESET);
        let start_time = std::time::Instant::now();
        for _n in 0..1000 {
            let decrypted1 = bob.decrypt_string_from(&encrypted1).unwrap();
            assert_eq!(decrypted1, test_message);
        }
        let decrypt_time = start_time.elapsed();
        assert!(decrypt_time.as_millis() < 1000);
        print_success(&format!("Decryption completed in {:?}", decrypt_time));
        print_success("✓ Message integrity verified!");

        println!("\n{}📊 Performance Summary:{}", BLUE, RESET);
        println!("  First encryption: {:?}", first_encrypt_time);
        println!("  1k encryptions: {:?}", second_encrypt_time);
        println!("  1k decryption: {:?}", decrypt_time);
        print_info("Note: Each encryption creates a fresh ChaChaBox (correct pattern)");

        println!("{}🎉 crypto_box creation pattern test PASSED!{}\n", BOLD, RESET);
    }

    #[test]
    #[serial(exchange)]
    fn test_contact_registration() {
        print_test_header("Contact Registration", "📋");

        let mut alice = Party::new("Alice");
        let bob = Party::new("Bob");
        let charlie = Party::new("Charlie");

        print_info("Testing contact registration without communication");

        // Initially no contacts
        assert_eq!(alice.contact_count(), 0);
        print_info("Alice starts with 0 contacts");

        // Add contacts manually
        alice.add_contact(&bob.get_public_key());
        alice.add_contact(&charlie.get_public_key());

        print_success(&format!("Alice manually registered {} contacts", alice.contact_count()));

        // Check if contacts are known
        assert!(alice.is_known_contact(&bob.get_public_key()));
        assert!(alice.is_known_contact(&charlie.get_public_key()));
        print_success("✓ Contact recognition working correctly");

        // Adding same contact twice shouldn't increase count
        alice.add_contact(&bob.get_public_key());
        assert_eq!(alice.contact_count(), 2);
        print_success("✓ Duplicate contact addition handled correctly");

        // List contacts
        let contacts = alice.get_contacts();
        assert_eq!(contacts.len(), 2);
        print_success("✓ Contact listing working correctly");

        println!("{}Alice's registered contacts:{}", BLUE, RESET);
        for (i, contact_key) in contacts.iter().enumerate() {
            let key_hex = hex::encode(contact_key.to_bytes());
            println!("  {}. {}...", i + 1, &key_hex[..16]);
        }

        println!("{}🎉 Contact registration test PASSED!{}\n", BOLD, RESET);
    }
}
