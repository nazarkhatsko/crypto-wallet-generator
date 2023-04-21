mod blockchain;
mod crypto;
mod generator;
mod generators;
mod utils;
mod wallet;

use crate::blockchain::Blockchain;
use crate::generator::Generator;
use crate::generators::bitcoin::BitcoinGenerator;
use crate::generators::ethereum::EthereumGenerator;
use crate::utils::filesystem::{read, write};
use crate::wallet::Wallet;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_enum)]
    blockchain: Blockchain,

    #[arg(short, long)]
    random: Option<bool>,

    #[arg(short, long)]
    mnemonic: Option<String>,

    #[arg(short, long)]
    file: Option<String>,

    #[arg(short, long)]
    output: Option<String>,
}

fn generate_manager<T: Generator>(
    random: Option<bool>,
    mnemonic: Option<String>,
    file: Option<String>,
) -> Result<Wallet, String> {
    if random.is_some() && mnemonic.is_none() && file.is_none() {
        return T::generate_by_random();
    } else if random.is_none() && mnemonic.is_some() && file.is_none() {
        return T::generate_by_mnemonic(&mnemonic.unwrap());
    } else if random.is_none() && mnemonic.is_none() && file.is_some() {
        return T::generate_by_file(&read(&file.unwrap()).unwrap());
    }

    Err("Invalid arguments".to_string())
}

fn main() {
    let args = Args::parse();

    let wallet = match args.blockchain {
        Blockchain::Bitcoin => {
            generate_manager::<BitcoinGenerator>(args.random, args.mnemonic, args.file)
        }
        Blockchain::Ethereum => {
            generate_manager::<EthereumGenerator>(args.random, args.mnemonic, args.file)
        }
    };
    let wallet = wallet.unwrap();

    if args.output.is_some() {
        write(&args.output.unwrap(), &wallet.to_json()).unwrap();
    } else {
        println!("{}", wallet);
    }
}
