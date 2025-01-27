use crate::{MnemonicType, Mnemonic};
use hex;

pub fn prepare_data_for_mnemonic_struct_initialization(mnemonic_type: MnemonicType) -> (Vec<u8>, u8) {
    let raw_entropy = Mnemonic::generate_entropy(mnemonic_type);
    let checksum_decimal = Mnemonic::generate_checksum(&raw_entropy, mnemonic_type);

    (raw_entropy, checksum_decimal) //Return
}

pub fn hex_to_binary(hex: &str) -> String {
    // Convert hex string to bytes
    let bytes = hex::decode(hex).expect("Invalid hex string");

    // Convert each byte to a binary string and collect them
    let binary_string: String = bytes.iter()
        .map(|byte| format!("{:08b}", byte))  // Format each byte as 8-bit binary
        .collect();

    binary_string.to_string()
}