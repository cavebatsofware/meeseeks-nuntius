pub mod aes256_gcm {
    use aes_gcm::{
        aead::{Aead, AeadCore, KeyInit, OsRng},
        Aes256Gcm, Nonce, Key
    };

    // Type alias for convenience
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    /// Generate a random 256-bit key for AES-256-GCM
    pub fn generate_key() -> Key<Aes256Gcm> {
        Aes256Gcm::generate_key(OsRng)
    }

    /// Encrypt plaintext using AES-256-GCM
    pub fn encrypt(key: &Key<Aes256Gcm>, plaintext: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
        let cipher = Aes256Gcm::new(key);
        
        // Generate a random nonce (96-bit for GCM)
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        
        // Encrypt the plaintext
        let ciphertext = cipher.encrypt(&nonce, plaintext)
            .map_err(|e| format!("Encryption failed: {}", e))?;
        
        // Return both ciphertext and nonce (nonce is needed for decryption)
        Ok((ciphertext, nonce.to_vec()))
    }

    /// Decrypt ciphertext using AES-256-GCM
    pub fn decrypt(key: &Key<Aes256Gcm>, ciphertext: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
        let cipher = Aes256Gcm::new(key);
        
        // Convert nonce back to the correct type
        let nonce = Nonce::from_slice(nonce);
        
        // Decrypt the ciphertext
        let plaintext = cipher.decrypt(nonce, ciphertext)
            .map_err(|e| format!("Decryption failed: {}", e))?;
        
        Ok(plaintext)
    }

    /// Convenience function to encrypt a string
    pub fn encrypt_string(key: &Key<Aes256Gcm>, plaintext: &str) -> Result<(Vec<u8>, Vec<u8>)> {
        encrypt(key, plaintext.as_bytes())
    }

    /// Convenience function to decrypt to a string
    pub fn decrypt_string(key: &Key<Aes256Gcm>, ciphertext: &[u8], nonce: &[u8]) -> Result<String> {
        let plaintext = decrypt(key, ciphertext, nonce)?;
        String::from_utf8(plaintext).map_err(|e| e.into())
    }
}

pub mod exchange {
    use crypto_box::{
        aead::{Aead, AeadCore, OsRng},
        ChaChaBox, PublicKey, SecretKey
    };
    use std::collections::HashSet;
    use serde::{Deserialize, Serialize};

    use crate::persistence::database::sled::Entity;

    // Type alias for convenience
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    /// Represents a party in the messaging system
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Party {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        pub name: String,
        secret_key: [u8; 32],
        public_key: [u8; 32],
        known_contacts: HashSet<[u8; 32]>,
    }

    impl Party {
        /// Create a new party with a random keypair
        pub fn new(name: &str) -> Self {
            let secret_key = SecretKey::generate(&mut OsRng);
            let public_key = secret_key.public_key();
            
            Self {
                id: None,
                name: name.to_string(),
                secret_key: secret_key.to_bytes(),
                public_key: public_key.to_bytes(),
                known_contacts: HashSet::new(),
            }
        }

        /// Create a new party with given values
        pub fn from_values(
            id: Option<String>,
            name: &str,
            secret_bytes: [u8; 32],
            public_bytes: [u8; 32],
            known_contacts: HashSet<[u8; 32]>
        ) -> Self {
            Self {
                id,
                name: name.to_string(),
                secret_key: secret_bytes,
                public_key: public_bytes,
                known_contacts,
            }
        }

        /// Serialize to JSON - ID will be included if present
        pub fn to_json(&self) -> Result<String> {
            Ok(serde_json::to_string(&self)?)
        }

        /// Deserialize from JSON - ID will be restored if present
        pub fn from_json(json: &str) -> Result<Self> {
            Ok(serde_json::from_str(json)?)
        }

        /// Create a new party and immediately register known contacts
        pub fn new_with_contacts(name: &str, other_parties: &[&PublicKey]) -> Self {
            let mut party = Self::new(name);
            
            // Register all contacts
            for other_public in other_parties {
                party.add_contact(other_public);
            }
            
            party
        }

        /// Get the public key
        pub fn public_key(&self) -> PublicKey {
            PublicKey::from_bytes(self.public_key)
        }

        /// Get the public key as bytes
        pub fn public_key_bytes(&self) -> [u8; 32] {
            self.public_key
        }

        /// Get the secret key as bytes
        pub fn secret_key_bytes(&self) -> [u8; 32] {
            self.secret_key
        }

        /// Add a contact to the known contacts list
        pub fn add_contact(&mut self, other_public: &PublicKey) {
            let other_key_bytes = other_public.to_bytes();
            self.known_contacts.insert(other_key_bytes);
        }

        /// Check if a contact is known
        pub fn is_known_contact(&self, other_public: &PublicKey) -> bool {
            let other_key_bytes = other_public.to_bytes();
            self.known_contacts.contains(&other_key_bytes)
        }

        /// Get the number of known contacts
        pub fn contact_count(&self) -> usize {
            self.known_contacts.len()
        }

        /// List all known contacts (their public keys)
        pub fn known_contacts(&self) -> Vec<PublicKey> {
            self.known_contacts.iter()
                .map(|bytes| PublicKey::from(*bytes))
                .collect()
        }

        /// List all known contacts (their public keys as byte arrays)
        pub fn known_contacts_bytes(&self) -> HashSet<[u8; 32]> {
            self.known_contacts.clone()
        }

        /// Create a crypto box for communication with another party
        fn create_crypto_box(&self, other_public: &PublicKey) -> ChaChaBox {
            // Use chacha20poly1305
            ChaChaBox::new(other_public, &SecretKey::from_bytes(self.secret_key))
        }

        /// Encrypt a message for another party
        pub fn encrypt_for(&mut self, recipient_public: &PublicKey, plaintext: &[u8]) -> Result<EncryptedMessage> {
            // Add contact if not already known
            if !self.is_known_contact(recipient_public) {
                self.add_contact(&recipient_public);
            }
            
            // Create a fresh crypto box for this message
            let crypto_box = self.create_crypto_box(recipient_public);
            let nonce = ChaChaBox::generate_nonce(&mut OsRng);
            
            let ciphertext = crypto_box.encrypt(&nonce, plaintext)
                .map_err(|e| format!("Encryption failed: {}", e))?;

            Ok(EncryptedMessage {
                id: None,
                sender_public: self.public_key,
                ciphertext,
                nonce: nonce.to_vec(),
            })
        }

        /// Encrypt a string message for another party
        pub fn encrypt_string_for(&mut self, recipient_public: &PublicKey, plaintext: &str) -> Result<EncryptedMessage> {
            self.encrypt_for(recipient_public, plaintext.as_bytes())
        }

        /// Decrypt a message from another party
        pub fn decrypt_from(&mut self, message: &EncryptedMessage) -> Result<Vec<u8>> {
            // Add contact if not already known\
            if !self.is_known_contact(&message.sender_public()) {
                self.add_contact(&message.sender_public());
            }
            
            // Create a fresh crypto box for this message
            let crypto_box = self.create_crypto_box(&message.sender_public());
            
            // Convert nonce back to the correct type
            if message.nonce.len() != 24 {
                return Err("Invalid nonce length".into());
            }
            let nonce_array: [u8; 24] = message.nonce.clone().try_into()
                .map_err(|_| "Failed to convert nonce")?;
            
            let plaintext = crypto_box.decrypt(&nonce_array.into(), &*message.ciphertext)
                .map_err(|e| format!("Decryption failed: {}", e))?;

            Ok(plaintext)
        }

        /// Decrypt a message to a string
        pub fn decrypt_string_from(&mut self, message: &EncryptedMessage) -> Result<String> {
            let plaintext = self.decrypt_from(message)?;
            String::from_utf8(plaintext).map_err(|e| e.into())
        }
    }

    /// Entity implementation for Party, common boilerplate
    impl Entity for Party {
        fn id(&self) -> Option<&str> {
            self.id.as_deref()
        }

        fn set_id(&mut self, id: String) {
            self.id = Some(id);
        }

        fn key_prefix() -> &'static str {
            "party"
        }
    }

    /// An encrypted message that can be sent between parties
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct EncryptedMessage {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        sender_public: [u8; 32],
        pub ciphertext: Vec<u8>,
        pub nonce: Vec<u8>,
    }

    impl EncryptedMessage {
        /// Create a new encrypted message
        pub fn new(sender_public: PublicKey, ciphertext: Vec<u8>, nonce: Vec<u8>) -> Self {
            Self {
                id: None,
                sender_public: sender_public.to_bytes(),
                ciphertext,
                nonce,
            }
        }

        /// Serialize to JSON - ID will be included if present
        pub fn to_json(&self) -> Result<String> {
            Ok(serde_json::to_string(&self)?)
        }

        /// Deserialize from JSON - ID will be restored if present
        pub fn from_json(json: &str) -> Result<Self> {
            Ok(serde_json::from_str(json)?)
        }

        pub fn sender_public(&self) -> PublicKey {
            PublicKey::from_bytes(self.sender_public)
        }

        pub fn sender_public_bytes(&self) -> [u8; 32] {
            self.sender_public
        }
    }

    impl Entity for EncryptedMessage {
        fn id(&self) -> Option<&str> {
            self.id.as_deref()
        }

        fn set_id(&mut self, id: String) {
            self.id = Some(id);
        }

        fn key_prefix() -> &'static str {
            "encrypted_message"
        }
    }
}
