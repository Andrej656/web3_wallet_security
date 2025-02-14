use ed25519_dalek::{PublicKey, Signature, Verifier};
use std::collections::HashMap;
use crate::errors::WalletError;

pub struct MultiSigWallet {
    pub owners: Vec<PublicKey>,
    pub threshold: usize,
}

impl MultiSigWallet {
    /// Create a new multi-signature wallet with a list of owners and a threshold.
    pub fn new(owners: Vec<PublicKey>, threshold: usize) -> Result<Self, WalletError> {
        if threshold > owners.len() {
            return Err(WalletError::InvalidThreshold);
        }
        Ok(Self { owners, threshold })
    }

    /// Verify if the provided signatures meet the threshold.
    pub fn verify_signatures(&self, message: &[u8], signatures: HashMap<PublicKey, Signature>) -> Result<bool, WalletError> {
        let mut valid_count = 0;

        for (public_key, signature) in signatures {
            if self.owners.contains(&public_key) && public_key.verify(message, &signature).is_ok() {
                valid_count += 1;
            }
            if valid_count >= self.threshold {
                return Ok(true);
            }
        }

        Ok(false)
    }
}
