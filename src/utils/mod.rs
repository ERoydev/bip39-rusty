use crate::{MnemonicType, Mnemonic};


pub fn prepare_data_for_mnemonic_struct_initialization(mnemonic_type: MnemonicType) -> (Vec<u8>, u8) {
    let mut raw_entropy = Mnemonic::generate_entropy(mnemonic_type);
    let checksum_decimal = Mnemonic::generate_checksum(&raw_entropy, mnemonic_type);

    (raw_entropy, checksum_decimal) //Return
}