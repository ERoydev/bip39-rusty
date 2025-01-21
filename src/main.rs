#![allow(unused)] // For beginning only

use crate::prelude::*;
use bip39_rusty::{Mnemonic, MnemonicType, Language};

mod error;
mod prelude;
mod utils;

fn main() -> Result<()> {
    let mnemonic = Mnemonic::new(Language::English, MnemonicType::Bits256);


    mnemonic.print_mnemonic_data();

    Ok(())
}
