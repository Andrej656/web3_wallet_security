use thiserror::Error;

#[derive(Debug, Error)]
pub enum WalletError {
    #[error("Failed to load or save key file.")]
    KeyFileError,
    #[error("Failed to deserialize the key.")]
    KeyDeserializationError,
    #[error("Invalid private key.")]
    InvalidPrivateKey,
    #[error("Invalid recipient address.")]
    InvalidAddress,
    #[error("Invalid threshold: must be less than or equal to the number of owners.")]
    InvalidThreshold,
    #[error("EIP-712 encoding failed.")]
    Eip712EncodingError,
}
