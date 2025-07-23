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

    // Type alias for convenience
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    /// Represents a party in the messaging system
    #[derive(Debug, Clone)]
    pub struct Party {
        pub name: String,
        secret_key: SecretKey,
        pub public_key: PublicKey,
        known_contacts: HashSet<[u8; 32]>,
    }

    impl Party {
        /// Create a new party with a random keypair
        pub fn new(name: &str) -> Self {
            let secret_key = SecretKey::generate(&mut OsRng);
            let public_key = secret_key.public_key();
            
            Self {
                name: name.to_string(),
                secret_key,
                public_key,
                known_contacts: HashSet::new(),
            }
        }

        /// Create a new party with given values
        pub fn from_values(
            name: &str,
            secret_bytes: [u8; 32],
            public_bytes: [u8; 32],
            known_contacts: HashSet<[u8; 32]>
        ) -> Self {
            Self {
                name: name.to_string(),
                secret_key: SecretKey::from_bytes(secret_bytes),
                public_key: PublicKey::from_bytes(public_bytes),
                known_contacts,
            }
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

        /// Get the public key as bytes for sharing
        pub fn public_key_bytes(&self) -> [u8; 32] {
            self.public_key.to_bytes()
        }

        /// Get the secret key as bytes for sharing
        pub fn secret_key_bytes(&self) -> [u8; 32] {
            self.secret_key.to_bytes()
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
        pub fn get_contacts(&self) -> Vec<PublicKey> {
            self.known_contacts.iter()
                .map(|bytes| PublicKey::from(*bytes))
                .collect()
        }

        /// List all known contacts (their public keys as byte arrays)
        pub fn get_contacts_bytes(&self) -> HashSet<[u8; 32]> {
            self.known_contacts.clone()
        }

        /// Create a crypto box for communication with another party
        fn create_crypto_box(&self, other_public: &PublicKey) -> ChaChaBox {
            // Use chacha20poly1305
            ChaChaBox::new(other_public, &self.secret_key)
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
                sender_public: self.public_key.clone(),
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
            if !self.is_known_contact(&message.sender_public) {
                self.add_contact(&message.sender_public);
            }
            
            // Create a fresh crypto box for this message
            let crypto_box = self.create_crypto_box(&message.sender_public);
            
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

    /// An encrypted message that can be sent between parties
    #[derive(Debug, Clone)]
    pub struct EncryptedMessage {
        pub sender_public: PublicKey,
        pub ciphertext: Vec<u8>,
        pub nonce: Vec<u8>,
    }

    impl EncryptedMessage {
        /// Serialize the message for transmission/storage
        pub fn to_bytes(&self) -> Vec<u8> {
            let mut result = Vec::new();
            
            // Add sender public key (32 bytes)
            result.extend_from_slice(&self.sender_public.to_bytes());
            
            // Add nonce length (4 bytes) and nonce
            result.extend_from_slice(&(self.nonce.len() as u32).to_be_bytes());
            result.extend_from_slice(&self.nonce);
            
            // Add ciphertext length (4 bytes) and ciphertext
            result.extend_from_slice(&(self.ciphertext.len() as u32).to_be_bytes());
            result.extend_from_slice(&self.ciphertext);
            
            result
        }

        /// Deserialize a message from bytes
        pub fn from_bytes(data: &[u8]) -> Result<Self> {
            if data.len() < 40 { // 32 + 4 + 4 minimum
                return Err("Invalid message format: too short".into());
            }

            let mut offset = 0;

            // Extract sender public key (32 bytes)
            let sender_bytes: [u8; 32] = data[offset..offset + 32].try_into()
                .map_err(|_| "Invalid sender public key")?;
            let sender_public = PublicKey::from(sender_bytes);
            offset += 32;

            // Extract nonce length and nonce
            let nonce_len = u32::from_be_bytes(data[offset..offset + 4].try_into()
                .map_err(|_| "Invalid nonce length")?) as usize;
            offset += 4;
            
            if data.len() < offset + nonce_len + 4 {
                return Err("Invalid message format: insufficient data for nonce".into());
            }
            
            let nonce = data[offset..offset + nonce_len].to_vec();
            offset += nonce_len;

            // Extract ciphertext length and ciphertext
            let ciphertext_len = u32::from_be_bytes(data[offset..offset + 4].try_into()
                .map_err(|_| "Invalid ciphertext length")?) as usize;
            offset += 4;
            
            if data.len() != offset + ciphertext_len {
                return Err("Invalid message format: incorrect ciphertext length".into());
            }
            
            let ciphertext = data[offset..offset + ciphertext_len].to_vec();

            Ok(EncryptedMessage {
                sender_public,
                ciphertext,
                nonce,
            })
        }
    }
}
