use rand::rngs::OsRng;
use rand::RngCore;
use sha256::digest;

mod language;
mod types;
mod utils;

pub use language::Language;
pub use crate::types::MnemonicType;

const MIN_WORDS: usize = 12;
const MAX_WORDS: usize = 24;
const DEFAULT_MNEMONIC_TYPE: MnemonicType = MnemonicType::Bits256; // Default Mnemonic Type when error occurs

#[derive(Debug)]
pub enum MnemonicError {
    InvalidChecksum,
    InvalidEntropy,
    GeneratorError,
}

impl std::fmt::Display for MnemonicError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MnemonicError::InvalidChecksum => write!(f, "Invalid checksum."),
            MnemonicError::InvalidEntropy => write!(f, "Invalid entropy."),
            MnemonicError::GeneratorError => write!(f, "Error when creating Mnemonic instance!")
        }
    }
}

pub struct EntropyInfo {
    pub bytes: usize,
    pub bits: usize,
}

pub struct Mnemonic {
    lang: Language,
    mnemonic_type: MnemonicType,
    entropy: Vec<u8>,
    checksum: u8,
    mnemonic_phrase: Vec<String>,
}


impl Mnemonic {

    pub fn is_valid(&self) -> bool {
        // Check if mnemonic phrase is not empty
        if self.mnemonic_phrase.is_empty() {
            return false;
        }

        // Check if mnemonic_phrase words count is in the required Ranges
        let word_count = self.mnemonic_phrase.len();
        if !matches!(word_count, MIN_WORDS | MAX_WORDS) {
            return false;
        }

        // Check if phrases exists in the wordlist
        let wordlist = Language::get_predefined_word_list(&self.lang);
        for word in &self.mnemonic_phrase {
            if !wordlist.contains(&word.as_str()) {
                return false;
            }
        }

        true
    }

    /// Wrapper for .generator() function, created to handle errors
    pub fn new(lang: Language, mnemonic_type: MnemonicType) -> Mnemonic {
        match Self::generator(lang, mnemonic_type) {
            Ok(mut mnemonic) => {
                mnemonic.mnemonic_phrase_generation();

                // Check if the generated mnemonic is valid before returning it
                if mnemonic.is_valid() {
                    mnemonic
                } else {
                    eprintln!("Generated mnemonic is invalid, using default fallback.");
                    Mnemonic::default()
                }
            }
            Err(e) => {
                eprintln!("Error creating mnemonic: {}, using default fallback", e);
                // Provide default Mnemonic as fallback if error happen
                Mnemonic::default()
            }
        }
    }

    /// Main function for creating an instance of Mnemonic Struct
    fn generator(lang: Language, mnemonic_type: MnemonicType) -> Result<Mnemonic, MnemonicError> {
        /*
        This is responsible to create Mnemonic instance and set initial values for entropy and checksum
        */
        let (mut raw_entropy, checksum_decimal) = utils::prepare_data_for_mnemonic_struct_initialization(mnemonic_type); // Derive entropy and checksum

        Self::validate_entropy(&raw_entropy).map_err(|_e| MnemonicError::InvalidEntropy)?;

        raw_entropy.push(checksum_decimal);

        Ok(Mnemonic {
            lang,
            mnemonic_type,
            entropy: raw_entropy, // I store it entropy + checksum
            checksum: checksum_decimal,
            mnemonic_phrase: Vec::new(),
        })
    }

    /// Handler if an error occurs inside .new() wrapper of .generator() to return default Mnemonic instance
    fn default() -> Mnemonic {
            let (mut raw_entropy, checksum_decimal) = utils::prepare_data_for_mnemonic_struct_initialization(DEFAULT_MNEMONIC_TYPE);
            raw_entropy.push(checksum_decimal);

            let mut mnemonic = Mnemonic {
                lang: Language::English,
                mnemonic_type: DEFAULT_MNEMONIC_TYPE,
                entropy: raw_entropy,
                checksum: checksum_decimal,
                mnemonic_phrase: Vec::new(),
            };

            mnemonic.mnemonic_phrase_generation();
            mnemonic
    }

    pub fn validate_checksum(&self) -> Result<bool, MnemonicError> {
        /*
            I use binary representation of entropy since i store entropy + checksum
            Then i calculate how many bits is my checksum and i retrieve it
            I convert it to decimal and compare it with my self.checksum to see if it is the same
        */
        let binary_entropy = Self::convert_entropy_to_binary(&self.entropy);
        let checksum_bits = self.mnemonic_type.bits() / 32;

        if binary_entropy.len() < checksum_bits {
            return Err(MnemonicError::InvalidChecksum);
        }

        let checksum_binary = &binary_entropy[binary_entropy.len() - checksum_bits..];
        let checksum_decimal = u8::from_str_radix(checksum_binary, 2)
            .map_err(|_| MnemonicError::InvalidChecksum)?;

        Ok(checksum_decimal == self.checksum)
    }

    /// Getter for the mnemonic phrase.
    pub fn mnemonic_phrase(&self) -> &Vec<String> {
        &self.mnemonic_phrase
    }

    /// Bellow are functions that implement my bip39 cryptography
    fn generate_entropy(mnemonic_type: MnemonicType) -> Vec<u8> {
        let mut rng = OsRng {};
        let entropy_bytes_count = mnemonic_type.bytes();

        let mut entropy = vec![0u8; entropy_bytes_count]; // empty vector [0, 0, 0, 0, 0...] with length of 16 or 32 depends of mnemonic_type

        // Fill the vector with random bytes
        rng.fill_bytes(&mut entropy); // [123, 23, 123, 23, 123...]
        entropy
    }

    fn generate_checksum(entropy: &Vec<u8>, mnemonic_type: MnemonicType) -> u8 {
        let hash = digest(entropy); // Hash the entropy using sha256 which returns it in hexadecimal

        if hash.len() < 2 {
            panic!("Hash must be at least 2 characters.");
        }

        let checksum_bits = mnemonic_type.bits() / 32;
        let checksum_index = if checksum_bits == 4 {1} else if checksum_bits == 8 {2} else {0}; // i take 4 bits or 8 bits

        let checksum = &hash[..checksum_index]; // checksum in hexadecimal
        u8::from_str_radix(checksum, 16).expect("Failed to parse checksum as u8") // I convert hexadecimal to decimal in order to append in my raw entropy
    }

    fn convert_entropy_to_binary(entropy: &Vec<u8>) -> String {
        // [123, 231 ,123 ,123 ,43 ,123, 231(checksum)] => 0011100111011001110011
        let mut binary_entropy = String::new();

        for el in entropy {
            // Ensure each byte is represented by exactly 8 bits
            let binary_repr = format!("{:08b}", el);
            binary_entropy += &binary_repr
        }
        binary_entropy // => 011011001110111
    }

    fn mnemonic_phrase_generation(&mut self) {
        // Convert my raw entropy + checksum into binary, divide it into chunks of 11-bit each with length of 24 (words) or 12 (words)
        let binary_entropy = Self::convert_entropy_to_binary(&self.entropy);

        let mut start_idx = 0;
        let mut chunks = Vec::new(); // ["01000110110", "11100010110" ...] each chunk of 11bits for 24 len if Bit256

        // Loop through the binary string and extract 11-bit chunks
        while start_idx + 11 <= binary_entropy.len() {
            // Extract the chunk of 11 bits starting at `start_idx`
            chunks.push(binary_entropy.get(start_idx..start_idx + 11).unwrap());
            start_idx += 11; // Move to the next chunk
        }

        let wordlist = Language::get_predefined_word_list(&self.lang); // I take wordlist from language based on chosen one

        for chunk in chunks {
            // I have some number calculated from my 11-bit binary from 0 to 2047 and i have wordlist with 2048
            // I use this decimal representation as index to take word from my predefined list
            let decimal = usize::from_str_radix(chunk, 2).unwrap(); // Convert binary to decimal
            let phrase = wordlist[decimal];
            self.add_mnemonic_phrase(String::from(phrase));
        }
    }

    fn add_mnemonic_phrase(&mut self, word: String) {
        // Function to push words in mnemonic field in my Struct => Util function
        self.mnemonic_phrase.push(word);
    }

    fn validate_entropy(entropy: &Vec<u8>) -> Result<(), String> {
        let entropy_bits = entropy.len() * 8;
        if entropy_bits != 128 && entropy_bits != 256 {
            return Err(format!(
                "Invalid entropy length: expected 128 or 256 bits, but got {} bits",
                entropy_bits
            ));
        }
        Ok(())  // Return Ok if the entropy length is valid
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_valid_entropy_vector() {
        // Example valid entropy in decimal (corresponding to 128 bits)
        let entropy128: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let entropy256 = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
        ];

        let entropy_binary128 = Mnemonic::convert_entropy_to_binary(&entropy128);
        let entropy_binary256 = Mnemonic::convert_entropy_to_binary(&entropy256);

        let validate128 = Mnemonic::validate_entropy(&entropy128);
        let validate256 = Mnemonic::validate_entropy(&entropy256);

        assert!(validate128.is_ok(), "Entropy should correct");
        assert!(validate256.is_ok(), "Entropy should be correct");

        assert_eq!(entropy_binary128.len(), 128, "Binary length for entropy128 should be 128 bits");
        assert_eq!(entropy_binary256.len(), 256, "Binary length for entropy256 should be 256 bits");
    }

    #[test]
    fn test_invalid_entropy_vector() {
        let entropy128: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let entropy256 = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30
        ];

        let entropy_binary128 = Mnemonic::convert_entropy_to_binary(&entropy128);
        let entropy_binary256 = Mnemonic::convert_entropy_to_binary(&entropy256);

        let validate128 = Mnemonic::validate_entropy(&entropy128);
        let validate256 = Mnemonic::validate_entropy(&entropy256);

        assert!(validate128.is_err(), "Entropy is invalid");
        assert!(validate256.is_err(), "Entropy is invalid");

        assert_ne!(entropy_binary128.len(), 128);
        assert_ne!(entropy_binary256.len(), 256);
    }


}