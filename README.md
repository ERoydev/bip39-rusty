# BIP-39 Custom Implementation in Rust

This repository contains a custom implementation of the BIP-39 standard in Rust. The implementation allows for the generation of mnemonic phrases based on entropy, checksum, and wordlists. It supports multiple languages and mnemonic types (128-bit and 256-bit entropy).
I Developed this to simplify using the bip39 more easily in rust than the current bip39 that i found. The code is easy to understand and well documented, feel free to suggest improvements :)
## Features

- **Entropy Generation**: Generate random entropy with secure randomness.
- **Checksum Calculation**: Append a checksum based on the entropy.
- **Mnemonic Phrase Generation**: Convert entropy to mnemonic phrases using predefined wordlists.
- **Language Support**: Extendable to multiple languages.

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/bip39-rust.git
   cd bip39-rust
   ```

2. Ensure you have Rust installed. If not, install it using [rustup](https://rustup.rs/):


3. Build the project:
   ```bash
   cargo build
   ```

4. Run the tests:
   ```bash
   cargo test
   ```

## Usage

Here is an example of how to use the library to generate a mnemonic phrase:

```rust
use bip39_rusty::{Mnemonic, Language, MnemonicType};

fn main() {
    /*
        Demonstrating the use of the bip39-rusty library to generate a BIP39 mnemonic phrase.

        The `Mnemonic` struct expects as parameters:
            - Language (e.g., Language::English,)
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
```

## Library Structure

### Mnemonic
- Represents a BIP-39 mnemonic phrase.
- Fields:
  - `lang`: Language for the wordlist.
  - `mnemonic_type`: Type of mnemonic.
  - `entropy`: Generated entropy bytes. 
  - `checksum`: Checksum appended to entropy. 
  - `mnemonic_phrase`: List of words representing the mnemonic phrase. 

### MnemonicType
- Enum representing the type of mnemonic:
  - `Bits128`: 128-bit entropy (12 words).
  - `Bits256`: 256-bit entropy (24 words).

### Language
- Represents the wordlist language. You can add custom wordlists by extending this module.
```rust
pub enum Language {
    ChineseSimplified,
    ChineseTraditional,
    Czech,
    English,
    French,
    Italian,
    Japanese,
    Korean,
    Portuguese,
    Spanish
}
```
## Wordlist Support

The `Language` module provides predefined wordlists. Currently supported:
- English
- Chinese Simplified
- Chinese Traditional
- Korean
- Japanese
- French
- Czech
- Italian
- Portuguese
- Spanish

To add more languages, implement the wordlist in the `Language` module.

## Contributing

Contributions are welcome! If you have a feature request, bug report, or want to contribute code, please open an issue or a pull request.

### Steps to Contribute
1. Fork the repository.
2. Create a feature branch:
   ```bash
   git checkout -b feature-name
   ```
3. Commit your changes:
   ```bash
   git commit -m "Add feature-name"
   ```
4. Push to your branch:
   ```bash
   git push origin feature-name
   ```
5. Open a pull request.

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.

---

Feel free to explore, modify, and use this library as per your requirements. Happy coding!

