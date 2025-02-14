pub mod key_management;
pub mod transaction;
pub mod multi_sig;
pub mod typed_data;

pub struct WalletSdk;

impl WalletSdk {
    pub fn generate_key(file_path: &str) -> Result<key_management::KeyManager, String> {
        key_management::KeyManager::new(file_path)
    }

    pub fn load_key(file_path: &str) -> Result<key_management::KeyManager, String> {
        key_management::KeyManager::load(file_path)
    }

    pub fn create_multi_sig_wallet(
        owners: Vec<ed25519_dalek::PublicKey>,
        threshold: usize
    ) -> Result<multi_sig::MultiSigWallet, String> {
        multi_sig::MultiSigWallet::new(owners, threshold)
    }

    pub fn sign_eip712(private_key: &[u8], data: typed_data::ExampleData) -> Result<Vec<u8>, String> {
        typed_data::sign_typed_data(private_key, data)
    }
}
