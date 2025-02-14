
use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, SECRET_KEY_LENGTH};
use secrecy::{Secret, ExposeSecret};
use rand::rngs::OsRng;
use std::fs::{self, File};
use std::io::{Write, Read};
use crate::errors::WalletError;

pub struct KeyManager {
    keypair: Secret<Keypair>,
}

impl KeyManager {
    /// Generate a new keypair and optionally save it to disk.
    pub fn generate(file_path: Option<&str>) -> Result<Self, WalletError> {
        let mut csprng = OsRng {};
        let keypair = Keypair::generate(&mut csprng);

        if let Some(path) = file_path {
            Self::save_key(path, &keypair)?;
        }

        Ok(Self {
            keypair: Secret::new(keypair),
        })
    }

    /// Load an existing keypair from a file.
    pub fn load(file_path: &str) -> Result<Self, WalletError> {
        let mut file = File::open(file_path).map_err(|_| WalletError::KeyFileError)?;
        let mut key_data = Vec::new();
        file.read_to_end(&mut key_data).map_err(|_| WalletError::KeyFileError)?;

        let secret_key = ed25519_dalek::SecretKey::from_bytes(&key_data)
            .map_err(|_| WalletError::KeyDeserializationError)?;
        let public_key = PublicKey::from(&secret_key);
        let keypair = Keypair { secret: secret_key, public: public_key };

        Ok(Self {
            keypair: Secret::new(keypair),
        })
    }

    /// Get the public key as a hex-encoded string.
    pub fn public_key(&self) -> String {
        hex::encode(self.keypair.expose_secret().public.as_bytes())
    }

    /// Save a keypair to disk securely.
    fn save_key(file_path: &str, keypair: &Keypair) -> Result<(), WalletError> {
        let secret_key_bytes = keypair.secret.as_bytes();
        let mut file = File::create(file_path).map_err(|_| WalletError::KeyFileError)?;
        file.write_all(secret_key_bytes).map_err(|_| WalletError::KeyFileError)?;
        Ok(())
    }

    /// Sign a message with the private key.
    pub fn sign_message(&self, message: &[u8]) -> Result<Signature, WalletError> {
        Ok(self.keypair.expose_secret().sign(message))
    }
}
