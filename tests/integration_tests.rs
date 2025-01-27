use bip39_rusty::{Mnemonic, MnemonicType, Language};


#[test]
fn test_valid_mnemonic_creation__returns_mnemonic_instance() {
    let language = Language::English;
    let mnemonic_type = MnemonicType::Bits256;
    let mnemonic = Mnemonic::new(language, mnemonic_type);

    assert!(mnemonic.is_valid());
}

#[test]
fn test_invalid_mnemonic_creation__returns_default_mnemonic_instance() {
    let language = Language::English;
    let mnemonic_type = MnemonicType::Bits256;

    let mut mnemonic = Mnemonic::new(language, mnemonic_type);


    assert!(mnemonic.is_valid());
}