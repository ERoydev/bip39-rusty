use bip39_rusty::{Mnemonic, Language, MnemonicType};

fn main() {
    /*
        Demonstrating the use of the bip39-rusty library to generate a BIP39 mnemonic phrase.

        The `Mnemonic` struct expects:
            - Language (e.g., Language::English)
            - MnemonicType (e.g., Bits128 or Bits256)

        Once created, you can use the following getter methods:
            - .checksum()        => Returns the checksum used for verification.
            - .mnemonic_phrase() => Returns the generated mnemonic phrase as a Vec<String>.

        Note: If any internal error occurs during mnemonic generation,
              the library will return a default Mnemonic with 256 bits and Language::English type.
    */

    // Create a new mnemonic
    let mnemonic = Mnemonic::new(Language::English, MnemonicType::Bits256);

    // Display the getters()
    println!("Generated Mnemonic Phrase: {:?}", mnemonic.mnemonic_phrase());
    println!("Checksum: {}", mnemonic.checksum());
}
