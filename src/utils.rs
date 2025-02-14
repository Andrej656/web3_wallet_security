use rand::Rng;
use sha2::{Digest, Sha256};
use hex::{encode as hex_encode, decode as hex_decode};
use secrecy::{ExposeSecret, Secret};
use std::time::{SystemTime, UNIX_EPOCH};
use ethers::types::Address;

/// Generate a random 32-byte hex string (used for private keys, seeds, etc.)
pub fn generate_random_hex() -> String {
    let mut rng = rand::thread_rng();
    let random_bytes: [u8; 32] = rng.gen();
    hex_encode(random_bytes)
}

/// Hash a given input with SHA-256
pub fn sha256_hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex_encode(hasher.finalize())
}

/// Convert an Ethereum address to a checksummed format
pub fn to_checksum_address(address: &str) -> Result<String, hex::FromHexError> {
    let address_bytes = hex_decode(&address[2..])?; // Strip the "0x" prefix
    let hash = sha256_hash(&address.to_lowercase());
    let mut checksum_address = "0x".to_string();

    for (i, char) in address.chars().skip(2).enumerate() {
        let hash_char = hash.chars().nth(i).unwrap();
        if hash_char.to_digit(16).unwrap() >= 8 {
            checksum_address.push(char.to_ascii_uppercase());
        } else {
            checksum_address.push(char);
        }
    }

    Ok(checksum_address)
}

/// Validate if a string is a valid Ethereum address
pub fn is_valid_address(address: &str) -> bool {
    if !address.starts_with("0x") || address.len() != 42 {
        return false;
    }

    hex_decode(&address[2..]).is_ok()
}

/// Derive a unique identifier for a transaction based on timestamp and input hash
pub fn derive_tx_id(input: &str) -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    sha256_hash(&format!("{}{}", timestamp, input))
}

/// Safely convert a `Secret`-wrapped private key into an Ethereum address
pub fn private_key_to_address(private_key: &Secret<String>) -> Result<Address, hex::FromHexError> {
    let decoded = hex_decode(private_key.expose_secret())?;
    let public_key = secp256k1::PublicKey::from_secret_key(&secp256k1::Secp256k1::new(), &decoded);
    Ok(Address::from_slice(&Sha256::digest(&public_key.serialize_uncompressed()[1..])[12..]))
}

/// Check if two Ethereum addresses are equal (case insensitive)
pub fn addresses_are_equal(addr1: &str, addr2: &str) -> bool {
    addr1.to_lowercase() == addr2.to_lowercase()
}
