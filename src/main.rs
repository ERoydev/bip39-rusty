#![allow(unused)] // For beginning only

use std::fmt::format;
use bip39_rusty::{Mnemonic, MnemonicType, Language};

mod utils;

fn main() {
    match Mnemonic::new(Language::English, MnemonicType::Bits256) {
        Ok(mnemonic) => {
            println!("Mnemonic created");
            mnemonic.print_mnemonic_data();
        }
        Err(e) => {
            eprintln!("Failed to create mnemonic");
        }
    }

}