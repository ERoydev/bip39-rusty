#![allow(unused)] // For beginning only

use crate::prelude::*;
use bip39_rusty::{Mnemonic, MnemonicType, Language};

mod error;
mod prelude;
mod utils;

fn main() -> Result<()> {
    println!("Hello, world!");
    let mnemonic = Mnemonic::new(Language::English, MnemonicType::Bits256);
    let entropy = mnemonic.generate_entropy();
    let checksum_hash = mnemonic.checksum(&entropy);

    println!("{}", checksum_hash);

    println!("{:?}", entropy);
    Ok(())
}
