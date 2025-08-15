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

pub mod aes256_gcm {
    use aes_gcm::{
        aead::{Aead, AeadCore, KeyInit, OsRng},
        Aes256Gcm, Key, Nonce,
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
        let ciphertext = cipher
            .encrypt(&nonce, plaintext)
            .map_err(|e| format!("Encryption failed: {e}"))?;

        // Return both ciphertext and nonce (nonce is needed for decryption)
        Ok((ciphertext, nonce.to_vec()))
    }

    /// Decrypt ciphertext using AES-256-GCM
    pub fn decrypt(key: &Key<Aes256Gcm>, ciphertext: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
        let cipher = Aes256Gcm::new(key);

        // Convert nonce back to the correct type
        let nonce = Nonce::from_slice(nonce);

        // Decrypt the ciphertext
        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| format!("Decryption failed: {e}"))?;

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

use crypto_box::{
    aead::{Aead, AeadCore, OsRng},
    ChaChaBox, PublicKey, SecretKey,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::persistence::database::Entity;

// Type alias for convenience
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Helper function to get current timestamp
fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Represents a contact with detailed information
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Contact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    pub public_key: [u8; 32],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub verified: bool,
    pub blocked: bool,
    pub created_at: u64,
    pub last_seen: Option<u64>,
}

impl Default for Contact {
    fn default() -> Self {
        Self {
            id: None,
            name: String::new(),
            public_key: [0; 32],
            nickname: None,
            email: None,
            verified: false,
            blocked: false,
            created_at: current_timestamp(),
            last_seen: None,
        }
    }
}

impl Contact {
    /// Create a new contact
    pub fn new(name: &str, public_key: &PublicKey) -> Self {
        Self {
            name: name.to_string(),
            public_key: public_key.to_bytes(),
            ..Default::default()
        }
    }

    /// Create a contact with required fields using Default
    pub fn with_key(name: &str, public_key: &PublicKey) -> Self {
        Self {
            name: name.to_string(),
            public_key: public_key.to_bytes(),
            ..Default::default()
        }
    }

    /// Get the public key as a PublicKey object
    pub fn public_key(&self) -> PublicKey {
        PublicKey::from(self.public_key)
    }

    /// Get the public key as bytes
    pub fn public_key_bytes(&self) -> [u8; 32] {
        self.public_key
    }

    /// Set nickname
    pub fn set_nickname(&mut self, nickname: Option<String>) {
        self.nickname = nickname;
    }

    /// Set email
    pub fn set_email(&mut self, email: Option<String>) {
        self.email = email;
    }

    /// Mark as verified
    pub fn set_verified(&mut self, verified: bool) {
        self.verified = verified;
    }

    /// Block/unblock contact
    pub fn set_blocked(&mut self, blocked: bool) {
        self.blocked = blocked;
    }

    /// Update last seen timestamp
    pub fn update_last_seen(&mut self) {
        self.last_seen = Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );
    }

    /// Serialize to JSON
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }

    /// Deserialize from JSON
    pub fn from_json(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json)?)
    }

    /// Find a contact by public key from a list of contacts
    pub fn find_by_public_key<'a>(
        contacts: &'a [Contact],
        public_key: &[u8; 32],
    ) -> Option<&'a Contact> {
        contacts
            .iter()
            .find(|contact| contact.public_key == *public_key)
    }

    /// Find contacts by public keys from a list of contacts  
    pub fn find_by_public_keys<'a>(
        contacts: &'a [Contact],
        public_keys: &HashSet<[u8; 32]>,
    ) -> Vec<&'a Contact> {
        contacts
            .iter()
            .filter(|contact| public_keys.contains(&contact.public_key))
            .collect()
    }

    /// Filter non-blocked contacts from a list
    pub fn filter_non_blocked(contacts: &[Contact]) -> Vec<&Contact> {
        contacts.iter().filter(|contact| !contact.blocked).collect()
    }

    /// Filter verified contacts from a list
    pub fn filter_verified(contacts: &[Contact]) -> Vec<&Contact> {
        contacts.iter().filter(|contact| contact.verified).collect()
    }

    /// Get contact name or fallback to public key hex
    pub fn display_name(&self) -> String {
        if let Some(nickname) = &self.nickname {
            nickname.clone()
        } else {
            self.name.clone()
        }
    }
}

impl Entity for Contact {
    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    fn key_prefix() -> &'static str {
        "contact"
    }
}

/// Represents a room in the messaging system
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Room {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    pub description: String,
    pub member_count: u32,
    pub secret_key: [u8; 32],
    pub public_key: [u8; 32],
    pub known_contacts: HashSet<[u8; 32]>,
}

impl Default for Room {
    fn default() -> Self {
        let secret_key = SecretKey::generate(&mut OsRng);
        let public_key = secret_key.public_key();

        Self {
            id: None,
            name: String::new(),
            description: String::new(),
            member_count: 0,
            secret_key: secret_key.to_bytes(),
            public_key: public_key.to_bytes(),
            known_contacts: HashSet::new(),
        }
    }
}

impl Room {
    /// Create a new room with a random keypair
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    /// Create a new room with given values (for loading from storage)
    pub fn from_values(
        id: Option<String>,
        name: &str,
        description: &str,
        member_count: u32,
        secret_bytes: [u8; 32],
        public_bytes: [u8; 32],
        known_contacts: HashSet<[u8; 32]>,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: description.to_string(),
            member_count,
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

    /// Create a new room and immediately register known contacts
    pub fn new_with_contacts(name: &str, other_rooms: &[&PublicKey]) -> Self {
        let mut room = Self::new(name);

        // Register all contacts
        for other_public in other_rooms {
            room.add_contact(other_public);
        }

        room
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
        self.known_contacts
            .iter()
            .map(|bytes| PublicKey::from(*bytes))
            .collect()
    }

    /// List all known contacts (their public keys as byte arrays)
    pub fn known_contacts_bytes(&self) -> HashSet<[u8; 32]> {
        self.known_contacts.clone()
    }

    /// Get detailed contact information for known contacts
    pub fn get_known_contact_details<'a>(&self, all_contacts: &'a [Contact]) -> Vec<&'a Contact> {
        Contact::find_by_public_keys(all_contacts, &self.known_contacts)
    }

    /// Get a specific known contact's details
    pub fn get_contact_details<'a>(
        &self,
        all_contacts: &'a [Contact],
        public_key: &[u8; 32],
    ) -> Option<&'a Contact> {
        if self.known_contacts.contains(public_key) {
            Contact::find_by_public_key(all_contacts, public_key)
        } else {
            None
        }
    }

    /// Check if a public key belongs to a known and non-blocked contact
    pub fn is_trusted_contact(&self, all_contacts: &[Contact], public_key: &[u8; 32]) -> bool {
        if let Some(contact) = self.get_contact_details(all_contacts, public_key) {
            !contact.blocked
        } else {
            false
        }
    }

    /// Create a crypto box for communication with another contact
    fn create_crypto_box(&self, other_public: &PublicKey) -> ChaChaBox {
        // Use chacha20poly1305
        ChaChaBox::new(other_public, &SecretKey::from_bytes(self.secret_key))
    }

    /// Encrypt a message for another contact
    pub fn encrypt_for(
        &mut self,
        recipient_public: &PublicKey,
        plaintext: &[u8],
    ) -> Result<EncryptedMessage> {
        // Add contact if not already known
        if !self.is_known_contact(recipient_public) {
            self.add_contact(recipient_public);
        }

        // Create a fresh crypto box for this message
        let crypto_box = self.create_crypto_box(recipient_public);
        let nonce = ChaChaBox::generate_nonce(&mut OsRng);

        let ciphertext = crypto_box
            .encrypt(&nonce, plaintext)
            .map_err(|e| format!("Encryption failed: {e}"))?;

        Ok(EncryptedMessage {
            id: None,
            sender_public: self.public_key,
            ciphertext,
            nonce: nonce.to_vec(),
        })
    }

    /// Encrypt a string message for another contact
    pub fn encrypt_string_for(
        &mut self,
        recipient_public: &PublicKey,
        plaintext: &str,
    ) -> Result<EncryptedMessage> {
        self.encrypt_for(recipient_public, plaintext.as_bytes())
    }

    /// Decrypt a message from another contact
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
        let nonce_array: [u8; 24] = message
            .nonce
            .clone()
            .try_into()
            .map_err(|_| "Failed to convert nonce")?;

        let plaintext = crypto_box
            .decrypt(&nonce_array.into(), &*message.ciphertext)
            .map_err(|e| format!("Decryption failed: {e}"))?;

        Ok(plaintext)
    }

    /// Decrypt a message to a string
    pub fn decrypt_string_from(&mut self, message: &EncryptedMessage) -> Result<String> {
        let plaintext = self.decrypt_from(message)?;
        String::from_utf8(plaintext).map_err(|e| e.into())
    }
}

/// Entity implementation for Room, common boilerplate
impl Entity for Room {
    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    fn key_prefix() -> &'static str {
        "room"
    }
}

/// An encrypted message that can be sent between contacts
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EncryptedMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub sender_public: [u8; 32],
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
}

impl Default for EncryptedMessage {
    fn default() -> Self {
        Self {
            id: None,
            sender_public: [0; 32],
            ciphertext: Vec::new(),
            nonce: Vec::new(),
        }
    }
}

impl EncryptedMessage {
    /// Create a new encrypted message
    pub fn new(sender_public: PublicKey, ciphertext: Vec<u8>, nonce: Vec<u8>) -> Self {
        Self {
            sender_public: sender_public.to_bytes(),
            ciphertext,
            nonce,
            ..Default::default()
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
