pub mod local {
    use crate::crypto::message::exchange::Party;
    use std::collections::HashSet;
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct PartyDTO {
        /// used to persist or transfer a Party object.
        pub name: String,
        // TODO: look at encrypting the following values before saving
        // prefer using platform specific keystores, Apple Keychain Android Keymint etc
        pub secret_key: [u8; 32],
        pub public_key: [u8; 32],
        pub known_contacts: HashSet<[u8; 32]>,
    }

    impl PartyDTO {
        pub fn new(name: &str, secret_key: [u8; 32], public_key: [u8; 32], known_contacts: HashSet<[u8; 32]>) -> Self {
            Self {
                name: name.to_string(),
                secret_key,
                public_key,
                known_contacts
            }
        }

        pub fn from_party(party: &Party) -> Self {
            Self::new(
                party.name.as_str(),
                party.secret_key_bytes(),
                party.public_key_bytes(),
            party.get_contacts_bytes(),
            )
        }

        pub fn to_party(&self) -> Party {
            Party::from_values(
                self.name.as_str(),
                self.secret_key,
                self.public_key,
                self.known_contacts.clone(),
            )
        }
    }
}