use crate::blockchain::Blockchain;
use crate::crypto::secp256k1::generate_keypair;
use crate::generator::Generator;
use crate::utils::random::get_random_vec;
use crate::wallet::{Wallet, WalletBuilder};

pub struct EthereumGenerator {}

impl Generator for EthereumGenerator {
    fn generate_by_random() -> Result<Wallet, String> {
        Self::generate_by_bytes(&get_random_vec(0, 255, 32))
    }

    fn generate_by_mnemonic(mnemonic: &String) -> Result<Wallet, String> {
        Self::generate_by_bytes(&mnemonic.as_bytes().to_vec())
    }

    fn generate_by_file(file: &String) -> Result<Wallet, String> {
        Self::generate_by_bytes(&file.as_bytes().to_vec())
    }

    fn generate_by_bytes(bytes: &Vec<u8>) -> Result<Wallet, String> {
        let (secret_key, public_key) = generate_keypair();

        let wallet = WalletBuilder::new()
            .set_blockchain(Blockchain::Ethereum)
            .set_address("address".to_string())
            .set_public_key(public_key.to_string())
            .set_secret_key(secret_key.to_string())
            .build();

        Ok(wallet)
    }
}
