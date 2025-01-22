use bip39_rusty::{Mnemonic, MnemonicType, Language};

fn main() {
    let mnemonic = Mnemonic::new(Language::English, MnemonicType::Bits256);
    println!("Generated Mnemonic: {:?}", mnemonic.mnemonic_phrase());
}