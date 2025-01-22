use rand::rngs::OsRng;
use rand::RngCore;
use sha256::digest;
use hex;

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

    /// Wrapper for .generator() function, created to handle errors
    pub fn new(lang: Language, mnemonic_type: MnemonicType) -> Mnemonic {
        match Self::generator(lang, mnemonic_type) {
            Ok(mut mnemonic) => {
                mnemonic.mnemonic_phrase_generation();
                mnemonic
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
        raw_entropy.push(checksum_decimal);

        Ok(Mnemonic {
            lang,
            mnemonic_type,
            entropy: raw_entropy,
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

    /// Getter for the checksum.
    pub fn checksum(&self) -> u8 {
        self.checksum
    }

    /// Getter for the mnemonic phrase.
    pub fn mnemonic_phrase(&self) -> &Vec<String> {
        &self.mnemonic_phrase
    }

    /// Bellow are functions that implement my bip39 cryptography
    fn generate_entropy(mnemonic_type: MnemonicType) -> Vec<u8> {
        let mut rng = OsRng {};
        let entropy_bytes_count = mnemonic_type.bytes();

        let mut entropy = vec![0u8; entropy_bytes_count]; // empty vector

        // Fill the vector with random bytes
        rng.fill_bytes(&mut entropy);
        entropy
    }

    fn generate_checksum(entropy: &Vec<u8>, mnemonic_type: MnemonicType) -> u8 {
        let hash = digest(entropy);

        if hash.len() < 2 {
            panic!("Hash must be at least 2 characters.");
        }

        let checksum_bits = mnemonic_type.bits() / 32;
        let checksum_index = if checksum_bits == 4 {1} else if checksum_bits == 8 {2} else {0};

        let checksum = &hash[..checksum_index]; // checksum in hexadecimal
        u8::from_str_radix(&checksum, 16).expect("Failed to parse checksum as u8") // I convert hexadecimal to decimal in order to append in my raw entropy
    }

    fn convert_entropy_to_binary(&self) -> String {
        let mut binary_entropy = String::new();
        for el in &self.entropy {
            // Ensure each byte is represented by exactly 8 bits
            let binary_repr = format!("{:08b}", el);
            binary_entropy += &binary_repr
        }
        binary_entropy
    }

    fn mnemonic_phrase_generation(&mut self) {
        let binary_entropy = self.convert_entropy_to_binary(); // Convert entropy to binary

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
            let decimal = usize::from_str_radix(chunk, 2).unwrap(); // Convert binary to decimal
            let phrase = wordlist[decimal];
            self.add_mnemonic_phrase(String::from(phrase));
        }
    }

    fn add_mnemonic_phrase(&mut self, word: String) {
        // Function to push words in mnemonic field in my Struct => Util function
        self.mnemonic_phrase.push(word);
    }
}