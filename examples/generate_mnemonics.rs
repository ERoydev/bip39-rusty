use bip39_rusty::{Mnemonic, Language, MnemonicType};

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

}
