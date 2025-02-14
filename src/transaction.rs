use ethers::types::{TransactionRequest, U256};
use ethers::core::utils::keccak256;
use crate::errors::WalletError;

/// Build an Ethereum-compatible transaction.
pub fn build_transaction(
    to: &str,
    value: u64,
    gas: u64,
    gas_price: u64,
    nonce: u64,
) -> Result<TransactionRequest, WalletError> {
    let to_address = to.parse().map_err(|_| WalletError::InvalidAddress)?;
    Ok(TransactionRequest {
        to: Some(to_address),
        value: Some(U256::from(value)),
        gas: Some(U256::from(gas)),
        gas_price: Some(U256::from(gas_price)),
        nonce: Some(U256::from(nonce)),
        ..Default::default()
    })
}

/// Hash a transaction for signing.
pub fn hash_transaction(tx: &TransactionRequest) -> Vec<u8> {
    let rlp_encoded = rlp::encode(tx);
    keccak256(rlp_encoded)
}

/// Sign the hash of a transaction with a private key.
pub fn sign_transaction_hash(hash: &[u8], private_key: &[u8]) -> Result<Vec<u8>, WalletError> {
    let hashed = keccak256(hash);
    let signing_key = ethers::core::k256::ecdsa::SigningKey::from_slice(private_key)
        .map_err(|_| WalletError::InvalidPrivateKey)?;
    let signature = signing_key.sign_digest(hashed);
    Ok(signature.as_ref().to_vec())
}
