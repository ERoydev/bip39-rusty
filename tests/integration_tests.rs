use bip39_rusty::{Mnemonic, MnemonicType, Language};
use super::*;

#[test]
fn test_valid_mnemonic_creation_with_new_method() {
    let language = Language::English;
    let mnemonic_type = MnemonicType::Bits256;

    let mnemonic = Mnemonic::new(language, mnemonic_type);
    assert!(mnemonic.is_valid())


}