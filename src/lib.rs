use std::env::var;
use std::thread::current;
use rand::rngs::OsRng;
use rand::RngCore;
use sha256::digest;
use hex;

mod language;
pub use language::Language;


const MIN_WORDS: usize = 12;
const MAX_WORDS: usize = 24;

pub struct Mnemonic {
    lang: Language,
    mnemonic_type: MnemonicType,
    entropy: Vec<u8>,
    checksum: u8,
    mnemonic_phrase: Vec<String>,
}

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

#[derive(Copy, Clone)]
pub enum MnemonicType {
    Bits128, // 128 bits of entropy -> 16 bytes (128 bits / 8)
    Bits256, // 256 bits of entropy -> 32 bytes (256 bits / 8)
}

impl MnemonicType {
    pub const fn bytes(&self) -> usize {
        match self {
            MnemonicType::Bits128 => 16,
            MnemonicType::Bits256 => 32,
        }
    }

    pub const fn bits(&self) -> usize {
        match self {
            MnemonicType::Bits128 => 128,
            MnemonicType::Bits256 => 256,
        }
    }

    pub const fn words_count(&self) -> usize {
        match self {
            MnemonicType::Bits128 => MIN_WORDS,
            MnemonicType::Bits256 => MAX_WORDS,
        }
    }
}

pub struct EntropyInfo {
    pub bytes: usize,
    pub bits: usize,
}


impl Mnemonic {

    pub fn new(lang: Language, mnemonic_type: MnemonicType) -> Result<Mnemonic, MnemonicError> {
        // Used to create instance and use this instance to create mnemonic_phrases
        match Self::generator(lang, mnemonic_type) {
            Ok(mut mnemonic) => {
                // After i have instance create i can generate phrase
                mnemonic.mnemonic_phrase_generation();
                Ok(mnemonic)
            }
            Err(e) => {
                eprintln!("Error creating mnemonic: {}", e);
                Err(MnemonicError::GeneratorError)
            }
        }
    }

    /// Getter for the language.
    pub fn language(&self) -> &Language {
        &self.lang
    }

    /// Getter for the mnemonic type.
    pub fn mnemonic_type(&self) -> MnemonicType {
        self.mnemonic_type
    }

    /// Getter for the checksum.
    pub fn checksum(&self) -> u8 {
        self.checksum
    }

    /// Getter for the mnemonic phrase.
    pub fn mnemonic_phrase(&self) -> &Vec<String> {
        &self.mnemonic_phrase
    }

    fn generator(lang: Language, mnemonic_type: MnemonicType) -> Result<Mnemonic, MnemonicError> {
        /*
        This is responsible to create Mneumonic instance and set initial values for checksum, entropy and so on
        */
        let mut raw_entropy = Mnemonic::generate_entropy(mnemonic_type);
        let checksum_decimal = Mnemonic::generate_checksum(&raw_entropy, mnemonic_type)?;
        raw_entropy.push(checksum_decimal);


        Ok(Mnemonic {
            lang,
            mnemonic_type,
            entropy: raw_entropy,
            checksum: checksum_decimal,
            mnemonic_phrase: Vec::new(),
        })
    }

    fn generate_entropy(mnemonic_type: MnemonicType) -> Vec<u8> {
        let mut rng = OsRng {};
        let entropy_bytes_count = mnemonic_type.bytes();

        let mut entropy = vec![0u8; entropy_bytes_count]; // empty vector

        // Fill the vector with random bytes
        rng.fill_bytes(&mut entropy);
        entropy
    }

    fn generate_checksum(entropy: &Vec<u8>, mnemonic_type: MnemonicType) -> Result<u8, MnemonicError> {
        let hash = digest(entropy);

        if hash.len() < 2 {
            panic!("Hash must be at least 2 characters.");
        }

        let checksum_bits = mnemonic_type.bits() / 32;
        let checksum_index = if checksum_bits == 4 {1} else if checksum_bits == 8 {2} else {0};

        let checksum = &hash[..checksum_index]; // checksum in hexadecimal
        u8::from_str_radix(&checksum, 16)// I convert hexadecimal to decimal in order to append in my raw entropy
            .map_err(|_| MnemonicError::InvalidChecksum)
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
        // Function to push words in mnemonic field in my Struct
        self.mnemonic_phrase.push(word);
    }
}