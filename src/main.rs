#![allow(unused)] // For beginning only

use std::fmt::format;
use bip39_rusty::{Mnemonic, MnemonicType, Language};

mod utils;

fn main() {
    match Mnemonic::new(Language::English, MnemonicType::Bits256) {
        Ok(mnemonic) => {
            println!("Mnemonic created successfully!");
            mnemonic.print_mnemonic_data();
        }
        Err(e) => {
            eprintln!("Error creating mnemonic: {}", e);
        }
    }

    let x = 42;
    let binary_x = format!("{:b}", x);
    println!("The binary representation of {} is {}", x, binary_x);
}