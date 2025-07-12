pub mod pgp_mgr {
    use dioxus::prelude::*;
    use pgp::composed::{
        ArmorOptions, Deserializable, KeyType, SecretKeyParamsBuilder, SignedPublicKey, SignedSecretKey, SubkeyParamsBuilder
    };
    use pgp::crypto::hash::HashAlgorithm;
    use pgp::crypto::sym::SymmetricKeyAlgorithm;
    use pgp::types::{KeyDetails, KeyVersion};
    use pgp::types::{CompressionAlgorithm, Password};
    use serde::{Deserialize, Serialize};
    use zeroize::Zeroizing;
    use rand::thread_rng;

    // Constants for preferred algorithms (following rpgpie pattern)
    const PREFERRED_SYMMETRIC_KEY_ALGORITHMS: &[SymmetricKeyAlgorithm] = &[
        SymmetricKeyAlgorithm::AES256,
        SymmetricKeyAlgorithm::AES192,
        SymmetricKeyAlgorithm::AES128,
    ];

    const PREFERRED_HASH_ALGORITHMS: &[HashAlgorithm] = &[
        HashAlgorithm::Sha512,
    ];

    const PREFERRED_COMPRESSION_ALGORITHMS: &[CompressionAlgorithm] = &[
        CompressionAlgorithm::ZLIB,
        CompressionAlgorithm::ZIP,
    ];

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct KeyPair {
        pub public_key: String,      // ASCII armored public key
        pub private_key: String,     // ASCII armored private key (possibly encrypted)
        pub key_id: String,          // Key ID (last 8 bytes of fingerprint)
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserKeys {
        pub primary_key: KeyPair,    // Primary key for certification and signing
        pub encryption_subkey: bool, // Whether an encryption subkey was added
        pub user_id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct GenerateKeysRequest {
        pub name: String,
        pub email: String,
        pub passphrase: Option<String>,
        pub key_type: Option<KeyAlgorithm>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum KeyAlgorithm {   
        RSA4096,
        Ed25519,
        ECDSA,
    }

    impl Default for KeyAlgorithm {
        fn default() -> Self {
            KeyAlgorithm::Ed25519
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct GenerateKeysResponse {
        pub success: bool,
        pub keys: Option<UserKeys>,
        pub error: Option<String>,
    }

    #[derive(Debug)]
    pub enum KeyGenerationError {
        InvalidInput(String),
        KeyGenerationFailed(pgp::errors::Error),
        BuilderError(Box<dyn std::error::Error + Send + Sync>),
        SerializationFailed(pgp::errors::Error),
        Utf8Error(std::string::FromUtf8Error),
    }

    impl std::fmt::Display for KeyGenerationError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                KeyGenerationError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
                KeyGenerationError::KeyGenerationFailed(e) => write!(f, "Key generation failed: {}", e),
                KeyGenerationError::BuilderError(e) => write!(f, "Key builder error: {}", e),
                KeyGenerationError::SerializationFailed(e) => write!(f, "Key serialization failed: {}", e),
                KeyGenerationError::Utf8Error(e) => write!(f, "UTF-8 conversion failed: {}", e),
            }
        }
    }

    impl std::error::Error for KeyGenerationError {}

    pub async fn generate_openpgp_keys(
        request: GenerateKeysRequest,
    ) -> Result<GenerateKeysResponse, ServerFnError> {
        // Validate input
        if request.name.trim().is_empty() || request.email.trim().is_empty() {
            return Ok(GenerateKeysResponse {
                success: false,
                keys: None,
                error: Some("Name and email are required".to_string()),
            });
        }

        // Create user ID string following OpenPGP convention
        let user_id = format!("{} <{}>", request.name.trim(), request.email.trim());
        
        // Use secure password handling with Zeroizing
        let password = request.passphrase
            .map(|p| Password::Static(Zeroizing::new(p.into())));

        match generate_key_pair(&user_id, password.as_ref(), request.key_type.unwrap_or_default()) {
            Ok(keys) => Ok(GenerateKeysResponse {
                success: true,
                keys: Some(keys),
                error: None,
            }),
            Err(e) => Ok(GenerateKeysResponse {
                success: false,
                keys: None,
                error: Some(e.to_string()),
            }),
        }
    }

    fn generate_key_pair(
        user_id: &str,
        password: Option<&Password>,
        algorithm: KeyAlgorithm,
    ) -> Result<UserKeys, KeyGenerationError> {
        let mut rng = thread_rng();

        // Determine key types
        let (key_type_primary, key_type_encrypt) = match algorithm {
            KeyAlgorithm::RSA4096 => (KeyType::Rsa(4096), Some(KeyType::Rsa(4096))),
            KeyAlgorithm::Ed25519 => (KeyType::Ed25519, None),
            KeyAlgorithm::ECDSA => (
                KeyType::ECDSA(pgp::crypto::ecc_curve::ECCCurve::P256),
                Some(KeyType::ECDH(pgp::crypto::ecc_curve::ECCCurve::Curve25519))
            ),
        };

        // Build subkeys if needed (following rpgpie pattern)
        let subkeys = if let Some(encrypt_key_type) = key_type_encrypt.clone().into() {
            vec![SubkeyParamsBuilder::default()
                .version(KeyVersion::V5)
                .key_type(encrypt_key_type)
                .can_encrypt(true)
                .build()
                .map_err(|e| KeyGenerationError::BuilderError(Box::new(e)))?]
        } else {
            vec![]
        };

        // Build primary key parameters (following rpgpie pattern)
        let mut key_params = SecretKeyParamsBuilder::default();
        key_params
            .version(KeyVersion::V5)
            .key_type(key_type_primary)
            .can_certify(true)
            .can_sign(true)
            .primary_user_id(user_id.to_string())
            .preferred_symmetric_algorithms(PREFERRED_SYMMETRIC_KEY_ALGORITHMS.into())
            .preferred_hash_algorithms(PREFERRED_HASH_ALGORITHMS.into())
            .preferred_compression_algorithms(PREFERRED_COMPRESSION_ALGORITHMS.into())
            .subkeys(subkeys);

        // Build the key parameters
        let secret_key_params = key_params
            .build()
            .map_err(|e| KeyGenerationError::BuilderError(Box::new(e)))?;

        // Generate the key
        let secret_key = secret_key_params
            .generate(&mut rng)
            .map_err(KeyGenerationError::KeyGenerationFailed)?;

        // Sign with empty password first (following rpgpie pattern)
        let mut signed_secret_key = secret_key
            .sign(&mut rng, &Password::empty())
            .map_err(KeyGenerationError::KeyGenerationFailed)?;

        // Set password if provided (following rpgpie pattern)
        if let Some(key_password) = password {
            signed_secret_key
                .primary_key
                .set_password(&mut rng, key_password)
                .map_err(KeyGenerationError::KeyGenerationFailed)?;

            // Set password for subkeys too
            for sk in &mut signed_secret_key.secret_subkeys {
                sk.key
                    .set_password(&mut rng, key_password)
                    .map_err(KeyGenerationError::KeyGenerationFailed)?;
            }
        }

        // Export the key pair
        let key_pair = export_key_pair(signed_secret_key)?;

        Ok(UserKeys {
            primary_key: key_pair,
            encryption_subkey: key_type_encrypt.is_some(),
            user_id: user_id.to_string(),
        })
    }

    fn export_key_pair(
        signed_secret_key: SignedSecretKey
    ) -> Result<KeyPair, KeyGenerationError> {
        // Extract key metadata
        let key_id = hex::encode(signed_secret_key.key_id());

        // Export public key
        let signed_public_key = signed_secret_key.signed_public_key();
        
        let armor_options = ArmorOptions {
            headers: None,
            include_checksum: true,
        };

        let mut public_key_buf = Vec::new();
        signed_public_key.to_armored_writer(&mut public_key_buf, armor_options.clone())
            .map_err(KeyGenerationError::SerializationFailed)?;
        let public_key_armor = String::from_utf8(public_key_buf)
            .map_err(KeyGenerationError::Utf8Error)?;

        // Export private key (it's already encrypted if password was provided)
        let mut private_key_buf = Vec::new();
        signed_secret_key.to_armored_writer(&mut private_key_buf, armor_options.clone())
            .map_err(KeyGenerationError::SerializationFailed)?;
        let private_key_armor = String::from_utf8(private_key_buf)
            .map_err(KeyGenerationError::Utf8Error)?;

        Ok(KeyPair {
            public_key: public_key_armor,
            private_key: private_key_armor,
            key_id,
        })
    }

    // Helper functions for loading keys back from armor format
    pub fn load_public_key(armor: &str) -> Result<SignedPublicKey, pgp::errors::Error> {
        let (key, _) = SignedPublicKey::from_string(armor)?;
        Ok(key)
    }

    pub fn load_secret_key(
        armor: &str,
        _passphrase: Option<&str>,
    ) -> Result<SignedSecretKey, pgp::errors::Error> {
        let (key, _) = SignedSecretKey::from_string(armor)?;
        Ok(key)
    }
}