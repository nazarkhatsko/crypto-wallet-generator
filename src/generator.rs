use crate::wallet::Wallet;

pub trait Generator {
    fn generate_by_random() -> Result<Wallet, String>;
    fn generate_by_mnemonic(mnemonic: &String) -> Result<Wallet, String>;
    fn generate_by_file(file: &String) -> Result<Wallet, String>;
    fn generate_by_bytes(bytes: &Vec<u8>) -> Result<Wallet, String>;
}
