#![allow(unused)] // For beginning only

use crate::prelude::*;
use bip39_rusty::{Mnemonic, MnemonicType, Language};

mod error;
mod prelude;
mod utils;

fn main() -> Result<()> {
    println!("Hello, world!");
    let mnemonic = Mnemonic::new(Language::English, MnemonicType::Twelve);
    let entropy = mnemonic.generate_entropy();

    println!("{:?}", entropy);
    Ok(())
}
