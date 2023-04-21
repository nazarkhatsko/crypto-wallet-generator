use crate::blockchain::Blockchain;

use ansi_term::Colour::{Cyan, Green, Red, White, Yellow};
use ansi_term::Style;
use serde::Serialize;
use serde_json;
use std::fmt;

pub struct WalletBuilder {
    blockchain: Option<Blockchain>,
    address: Option<String>,
    public_key: Option<String>,
    secret_key: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct Wallet {
    blockchain: Blockchain,
    address: String,
    public_key: String,
    secret_key: String,
}

impl WalletBuilder {
    pub fn new() -> Self {
        Self {
            blockchain: None,
            address: None,
            public_key: None,
            secret_key: None,
        }
    }

    pub fn set_blockchain(&mut self, blockchain: Blockchain) -> &mut Self {
        self.blockchain = Some(blockchain);
        self
    }

    pub fn set_address(&mut self, address: String) -> &mut Self {
        self.address = Some(address);
        self
    }

    pub fn set_public_key(&mut self, public_key: String) -> &mut Self {
        self.public_key = Some(public_key);
        self
    }

    pub fn set_secret_key(&mut self, secret_key: String) -> &mut Self {
        self.secret_key = Some(secret_key);
        self
    }

    pub fn build(&self) -> Wallet {
        Wallet::new(
            self.blockchain.clone().unwrap(),
            self.address.clone().unwrap(),
            self.public_key.clone().unwrap(),
            self.secret_key.clone().unwrap(),
        )
    }
}

impl Wallet {
    pub fn new(
        blockchain: Blockchain,
        address: String,
        public_key: String,
        secret_key: String,
    ) -> Self {
        Self {
            blockchain,
            address,
            public_key,
            secret_key,
        }
    }

    pub fn blockchain(&self) -> &Blockchain {
        &self.blockchain
    }

    pub fn address(&self) -> &String {
        &self.address
    }

    pub fn public_key(&self) -> &String {
        &self.public_key
    }

    pub fn secret_key(&self) -> &String {
        &self.secret_key
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl fmt::Display for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\n\t{}\t{}\n\t{}\t{}\n\t{}\t{}\n\t{}\t{}\n",
            Style::new().fg(White).bold().paint("Blockchain:"),
            Style::new()
                .fg(Yellow)
                .bold()
                .paint(self.blockchain().to_string()),
            Style::new().fg(White).bold().paint("Address:"),
            Style::new().fg(Green).bold().paint(self.address()),
            Style::new().fg(White).bold().paint("Public key:"),
            Style::new().fg(Cyan).bold().paint(self.public_key()),
            Style::new().fg(White).bold().paint("Secret key:"),
            Style::new().fg(Red).bold().paint(self.secret_key()),
        )
    }
}
