use sha256::digest;
use bip39_rusty::{Mnemonic, Language, MnemonicType, hex_to_binary};

fn main() {
    /*
        Demonstrating the use of the bip39-rusty library to generate a BIP39 mnemonic phrase.

        The `Mnemonic` struct expects:
            - Language (e.g., Language::English)
            - MnemonicType (e.g., Bits128 or Bits256)

        Once created, you can use the following getter method:
            - .mnemonic_phrase() => Returns the generated mnemonic phrase as a Vec<String>.

        Note: If any internal error occurs during mnemonic generation,
              the library will return a default Mnemonic with 256 bits and Language::English type.
    */

    // Create a new mnemonic
    let mnemonic = Mnemonic::new(Language::English, MnemonicType::Bits256);

    // Display the mnemonic phrases
    println!("Generated Mnemonic Phrase: {:?}", mnemonic.mnemonic_phrase());

    // validate the checksum
    let validation_result = mnemonic.validate_checksum();

    match validation_result {
        Ok(_value) => {
            println!("Its valid")
        }
        Err(_e) => {
            println!("Not valid")
        }
    }

    let lang = Language::English;
    let mnemonic_type = MnemonicType::Bits128;

    let mut mnemonic = Mnemonic::new(lang, mnemonic_type);

    let test_data = [
        "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f",
        "legal winner thank year wave sausage worth useful legal winner thank yellow",
    ];

    let mut binary_hex = hex_to_binary(&test_data[0]);
    let entropy_bytes = hex::decode(test_data[0]).expect("Invalid hex string");
    let hash = digest(entropy_bytes);

    println!("{}", hash);

    // Decode the hash into bytes
    let hash_bytes = hex::decode(hash).expect("Invalid hash string");

    // Extract the first 4 bits from the first byte of the hash
    let checksum_binary = format!("{:08b}", hash_bytes[0]); // Convert first byte to binary
    let checksum_binary = &checksum_binary[0..4]; // Take only the first 4 bits

    binary_hex += checksum_binary;
}

