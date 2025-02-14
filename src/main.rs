use web3_wallet_security::WalletSdk;

fn main() {
    let sdk = WalletSdk;

    // Example: Generate a new key
    let key_manager = sdk.generate_key("wallet.key").unwrap();
    println!("Public Key: {:?}", key_manager.public_key());

    // Example: Sign a transaction
    let message = b"Hello, Web3!";
    let signature = key_manager.sign_message(message);
    println!("Signature: {:?}", signature.to_bytes());
}
