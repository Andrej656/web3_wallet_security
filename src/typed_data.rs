use ethers::types::{transaction::eip712::Eip712, Address};
use ethers::core::k256::ecdsa::{SigningKey, signature::Signer};
use ethers::core::types::H256;
use serde::{Deserialize, Serialize};
use crate::errors::WalletError;

#[derive(Debug, Clone, Serialize, Deserialize, Eip712)]
#[eip712(
    name = "MyDApp",
    version = "1.0",
    chain_id = 1,
    verifying_contract = "0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC"
)]
pub struct ExampleData {
    pub from: Address,
    pub to: Address,
    pub amount: u64,
    pub nonce: u64,
}

/// Sign EIP-712 typed data.
pub fn sign_typed_data(private_key: &[u8], data: ExampleData) -> Result<Vec<u8>, WalletError> {
    let signing_key = SigningKey::from_slice(private_key).map_err(|_| WalletError::InvalidPrivateKey)?;
    let digest = data.encode_eip712().map_err(|_| WalletError::Eip712EncodingError)?;
    Ok(signing_key.sign_digest(H256::from(digest)).as_ref().to_vec())
}
