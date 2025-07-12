#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use tokio::test;
    use crate::pgp_mgr::generate::*;
    
    // Test data constants
    const TEST_NAME: &str = "Test User";
    const TEST_EMAIL: &str = "test@example.com";
    const TEST_PASSPHRASE: &str = "test_passphrase_123";
    const INVALID_EMAIL: &str = "invalid-email";

    #[tokio::test]
    async fn test_generate_keys_with_valid_input() {
        let request = GenerateKeysRequest {
            name: TEST_NAME.to_string(),
            email: TEST_EMAIL.to_string(),
            passphrase: Some(TEST_PASSPHRASE.to_string()),
            key_type: Some(KeyAlgorithm::Ed25519),
        };

        let response = generate_openpgp_keys(request).await.unwrap();
        
        assert!(response.success);
        assert!(response.keys.is_some());
        assert!(response.error.is_none());

        let keys = response.keys.unwrap();
        assert_eq!(keys.user_id, format!("{} <{}>", TEST_NAME, TEST_EMAIL));
        assert!(!keys.primary_key.public_key.is_empty());
        assert!(!keys.primary_key.private_key.is_empty());
        assert!(!keys.primary_key.key_id.is_empty());
    }

    #[tokio::test]
    async fn test_generate_keys_without_passphrase() {
        let request = GenerateKeysRequest {
            name: TEST_NAME.to_string(),
            email: TEST_EMAIL.to_string(),
            passphrase: None,
            key_type: Some(KeyAlgorithm::Ed25519),
        };

        let response = generate_openpgp_keys(request).await.unwrap();
        
        assert!(response.success);
        assert!(response.keys.is_some());
        assert!(response.error.is_none());
    }

    #[tokio::test]
    async fn test_generate_keys_with_empty_name() {
        let request = GenerateKeysRequest {
            name: "".to_string(),
            email: TEST_EMAIL.to_string(),
            passphrase: Some(TEST_PASSPHRASE.to_string()),
            key_type: Some(KeyAlgorithm::Ed25519),
        };

        let response = generate_openpgp_keys(request).await.unwrap();
        
        assert!(!response.success);
        assert!(response.keys.is_none());
        assert!(response.error.is_some());
        assert_eq!(response.error.unwrap(), "Name and email are required");
    }

    #[tokio::test]
    async fn test_generate_keys_with_empty_email() {
        let request = GenerateKeysRequest {
            name: TEST_NAME.to_string(),
            email: "".to_string(),
            passphrase: Some(TEST_PASSPHRASE.to_string()),
            key_type: Some(KeyAlgorithm::Ed25519),
        };

        let response = generate_openpgp_keys(request).await.unwrap();
        
        assert!(!response.success);
        assert!(response.keys.is_none());
        assert!(response.error.is_some());
        assert_eq!(response.error.unwrap(), "Name and email are required");
    }

    #[tokio::test]
    async fn test_generate_keys_with_whitespace_only_inputs() {
        let request = GenerateKeysRequest {
            name: "   ".to_string(),
            email: "   ".to_string(),
            passphrase: Some(TEST_PASSPHRASE.to_string()),
            key_type: Some(KeyAlgorithm::Ed25519),
        };

        let response = generate_openpgp_keys(request).await.unwrap();
        
        assert!(!response.success);
        assert!(response.keys.is_none());
        assert!(response.error.is_some());
        assert_eq!(response.error.unwrap(), "Name and email are required");
    }

    #[tokio::test]
    async fn test_generate_keys_rsa4096() {
        let request = GenerateKeysRequest {
            name: TEST_NAME.to_string(),
            email: TEST_EMAIL.to_string(),
            passphrase: Some(TEST_PASSPHRASE.to_string()),
            key_type: Some(KeyAlgorithm::RSA4096),
        };

        let response = generate_openpgp_keys(request).await.unwrap();
        
        assert!(response.success);
        let keys = response.keys.unwrap();
        assert!(keys.encryption_subkey); // RSA should have encryption subkey
    }

    #[tokio::test]
    async fn test_generate_keys_ecdsa() {
        let request = GenerateKeysRequest {
            name: TEST_NAME.to_string(),
            email: TEST_EMAIL.to_string(),
            passphrase: Some(TEST_PASSPHRASE.to_string()),
            key_type: Some(KeyAlgorithm::ECDSA),
        };

        let response = generate_openpgp_keys(request).await.unwrap();
        
        assert!(response.success);
        let keys = response.keys.unwrap();
        assert!(keys.encryption_subkey); // ECDSA should have encryption subkey
    }

    #[tokio::test]
    async fn test_generate_keys_ed25519() {
        let request = GenerateKeysRequest {
            name: TEST_NAME.to_string(),
            email: TEST_EMAIL.to_string(),
            passphrase: Some(TEST_PASSPHRASE.to_string()),
            key_type: Some(KeyAlgorithm::Ed25519),
        };

        let response = generate_openpgp_keys(request).await.unwrap();
        
        assert!(response.success);
        let keys = response.keys.unwrap();
        assert!(!keys.encryption_subkey); // Ed25519 should not have encryption subkey
    }

    #[tokio::test]
    async fn test_generate_keys_default_algorithm() {
        let request = GenerateKeysRequest {
            name: TEST_NAME.to_string(),
            email: TEST_EMAIL.to_string(),
            passphrase: Some(TEST_PASSPHRASE.to_string()),
            key_type: None, // Should default to Ed25519
        };

        let response = generate_openpgp_keys(request).await.unwrap();
        
        assert!(response.success);
        let keys = response.keys.unwrap();
        assert!(!keys.encryption_subkey); // Default Ed25519 should not have encryption subkey
    }

    #[test]
    async fn test_key_algorithm_default() {
        let default_algo = KeyAlgorithm::default();
        assert!(matches!(default_algo, KeyAlgorithm::Ed25519));
    }

    #[tokio::test]
    async fn test_load_public_key_roundtrip() {
        let request = GenerateKeysRequest {
            name: TEST_NAME.to_string(),
            email: TEST_EMAIL.to_string(),
            passphrase: None,
            key_type: Some(KeyAlgorithm::Ed25519),
        };

        let response = generate_openpgp_keys(request).await.unwrap();
        let keys = response.keys.unwrap();

        // Test loading the public key back
        let loaded_public_key = load_public_key(&keys.primary_key.public_key);
        assert!(loaded_public_key.is_ok());
        
        // let public_key = loaded_public_key.unwrap();
    }

    #[tokio::test]
    async fn test_load_secret_key_without_passphrase() {
        let request = GenerateKeysRequest {
            name: TEST_NAME.to_string(),
            email: TEST_EMAIL.to_string(),
            passphrase: None,
            key_type: Some(KeyAlgorithm::Ed25519),
        };

        let response = generate_openpgp_keys(request).await.unwrap();
        let keys = response.keys.unwrap();

        // Test loading the secret key back (no passphrase)
        let loaded_secret_key = load_secret_key(&keys.primary_key.private_key, None);
        assert!(loaded_secret_key.is_ok());
        
        // TODO: Add assertion for key.
        // let secret_key = loaded_secret_key.unwrap();
    }

    #[tokio::test]
    async fn test_load_secret_key_with_passphrase() {
        let request = GenerateKeysRequest {
            name: TEST_NAME.to_string(),
            email: TEST_EMAIL.to_string(),
            passphrase: Some(TEST_PASSPHRASE.to_string()),
            key_type: Some(KeyAlgorithm::Ed25519),
        };

        let response = generate_openpgp_keys(request).await.unwrap();
        let keys = response.keys.unwrap();

        // Test loading the secret key back with correct passphrase
        let loaded_secret_key = load_secret_key(&keys.primary_key.private_key, Some(TEST_PASSPHRASE));
        assert!(loaded_secret_key.is_ok());
        
        // let secret_key = loaded_secret_key.unwrap();
    }

    #[tokio::test]
    async fn test_load_secret_key_with_wrong_passphrase() {
        let request = GenerateKeysRequest {
            name: TEST_NAME.to_string(),
            email: TEST_EMAIL.to_string(),
            passphrase: Some(TEST_PASSPHRASE.to_string()),
            key_type: Some(KeyAlgorithm::Ed25519),
        };

        let response = generate_openpgp_keys(request).await.unwrap();
        let keys = response.keys.unwrap();

        // Test loading the secret key back with wrong passphrase
        let loaded_secret_key = load_secret_key(&keys.primary_key.private_key, Some("wrong_passphrase"));
        assert!(loaded_secret_key.is_err());
    }

    #[test]
    async fn test_load_public_key_invalid_armor() {
        let result = load_public_key("invalid armor text");
        assert!(result.is_err());
    }

    #[test]
    async fn test_load_secret_key_invalid_armor() {
        let result = load_secret_key("invalid armor text", None);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_key_fingerprint_and_id_format() {
        let request = GenerateKeysRequest {
            name: TEST_NAME.to_string(),
            email: TEST_EMAIL.to_string(),
            passphrase: None,
            key_type: Some(KeyAlgorithm::Ed25519),
        };

        let response = generate_openpgp_keys(request).await.unwrap();
        let keys = response.keys.unwrap();
        
        // Test key ID format (should be hex string)
        assert!(keys.primary_key.key_id.len() > 0);
        assert!(keys.primary_key.key_id.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[tokio::test]
    async fn test_armored_keys_format() {
        let request = GenerateKeysRequest {
            name: TEST_NAME.to_string(),
            email: TEST_EMAIL.to_string(),
            passphrase: None,
            key_type: Some(KeyAlgorithm::Ed25519),
        };

        let response = generate_openpgp_keys(request).await.unwrap();
        let keys = response.keys.unwrap();

        // Test public key armor format
        assert!(keys.primary_key.public_key.starts_with("-----BEGIN PGP PUBLIC KEY BLOCK-----"));
        assert!(keys.primary_key.public_key.ends_with("-----END PGP PUBLIC KEY BLOCK-----\n"));
        
        // Test private key armor format
        assert!(keys.primary_key.private_key.starts_with("-----BEGIN PGP PRIVATE KEY BLOCK-----"));
        assert!(keys.primary_key.private_key.ends_with("-----END PGP PRIVATE KEY BLOCK-----\n"));
    }

    #[tokio::test]
    async fn test_user_id_format() {
        let name_with_spaces = "John Doe";
        let email = "john.doe@example.com";
        
        let request = GenerateKeysRequest {
            name: name_with_spaces.to_string(),
            email: email.to_string(),
            passphrase: None,
            key_type: Some(KeyAlgorithm::Ed25519),
        };

        let response = generate_openpgp_keys(request).await.unwrap();
        let keys = response.keys.unwrap();

        assert_eq!(keys.user_id, format!("{} <{}>", name_with_spaces, email));
    }

    #[tokio::test]
    async fn test_key_generation_error_display() {
        use std::error::Error;

        let error = KeyGenerationError::InvalidInput("test error".to_string());
        assert!(error.to_string().contains("Invalid input: test error"));
        
        // Test that it implements Error trait
        assert!(error.source().is_none());
    }

    #[tokio::test]
    async fn test_multiple_key_generation_uniqueness() {
        let request1 = GenerateKeysRequest {
            name: TEST_NAME.to_string(),
            email: TEST_EMAIL.to_string(),
            passphrase: None,
            key_type: Some(KeyAlgorithm::Ed25519),
        };

        let request2 = GenerateKeysRequest {
            name: TEST_NAME.to_string(),
            email: TEST_EMAIL.to_string(),
            passphrase: None,
            key_type: Some(KeyAlgorithm::Ed25519),
        };

        let response1 = generate_openpgp_keys(request1).await.unwrap();
        let response2 = generate_openpgp_keys(request2).await.unwrap();

        let keys1 = response1.keys.unwrap();
        let keys2 = response2.keys.unwrap();

        // Keys should be unique even with same input
        assert_ne!(keys1.primary_key.key_id, keys2.primary_key.key_id);
        assert_ne!(keys1.primary_key.public_key, keys2.primary_key.public_key);
        assert_ne!(keys1.primary_key.private_key, keys2.primary_key.private_key);
    }

    #[tokio::test]
    async fn test_serialization_deserialization() {
        let request = GenerateKeysRequest {
            name: TEST_NAME.to_string(),
            email: TEST_EMAIL.to_string(),
            passphrase: Some(TEST_PASSPHRASE.to_string()),
            key_type: Some(KeyAlgorithm::RSA4096),
        };

        // Test request serialization
        let serialized_request = serde_json::to_string(&request).unwrap();
        let deserialized_request: GenerateKeysRequest = serde_json::from_str(&serialized_request).unwrap();
        assert_eq!(request.name, deserialized_request.name);
        assert_eq!(request.email, deserialized_request.email);

        // Test response serialization
        let response = generate_openpgp_keys(request).await.unwrap();
        let serialized_response = serde_json::to_string(&response).unwrap();
        let deserialized_response: GenerateKeysResponse = serde_json::from_str(&serialized_response).unwrap();
        assert_eq!(response.success, deserialized_response.success);
    }
}
