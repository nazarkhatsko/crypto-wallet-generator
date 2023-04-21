use clap::ValueEnum;
use serde::Serialize;
use std::fmt;

#[derive(Serialize, ValueEnum, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Blockchain {
    Bitcoin,
    Ethereum,
}

impl fmt::Display for Blockchain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Blockchain::Bitcoin => write!(f, "Bitcoin"),
            Blockchain::Ethereum => write!(f, "Ethereum"),
        }
    }
}
