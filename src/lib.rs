use std::env::var;
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
    mnemonic_phrase: String,
}

#[derive(Debug)]
pub enum MnemonicError {
    InvalidChecksum,
    InvalidEntropy
}

impl std::fmt::Display for MnemonicError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MnemonicError::InvalidChecksum => write!(f, "Invalid checksum."),
            MnemonicError::InvalidEntropy => write!(f, "Invalid entropy."),
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
        let mut raw_entropy = Mnemonic::generate_entropy(mnemonic_type);
        let checksum_decimal = Mnemonic::checksum(&raw_entropy, mnemonic_type)?;
        let binary_entropy = Mnemonic::convert_entropy_to_binary(&raw_entropy);

        println!("binary entropy: {}", binary_entropy);

        raw_entropy.push(checksum_decimal);

        Ok(Mnemonic {
            lang,
            mnemonic_type,
            entropy: raw_entropy,
            checksum: checksum_decimal,
            mnemonic_phrase: String::new(),
        })
    }

    pub fn print_mnemonic_data(&self) {
        println!("Raw Entropy: {:?}, Checksum_decimal: {}", self.entropy, self.checksum);
    }

    fn generate_entropy(mnemonic_type: MnemonicType) -> Vec<u8> {
        let mut rng = OsRng {};
        let entropy_bytes_count = mnemonic_type.bytes();

        let mut entropy = vec![0u8; entropy_bytes_count]; // empty vector

        // Fill the vector with random bytes
        rng.fill_bytes(&mut entropy);
        entropy
    }


    fn checksum(entropy: &Vec<u8>, mnemonic_type: MnemonicType) -> Result<u8, MnemonicError> {
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

    fn convert_entropy_to_binary(entropy: &Vec<u8>) -> String {
        let mut binary_entropy = String::new();
        for el in entropy {
            // Ensure each byte is represented by exactly 8 bits
            let binary_repr = format!("{:08b}", el);
            binary_entropy += &binary_repr
        }
        binary_entropy
    }
}