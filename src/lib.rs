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
}

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

    pub fn new(lang: Language, mnemonic_type: MnemonicType) -> Mnemonic {
        Mnemonic {
            lang,
            mnemonic_type,
        }
    }


    pub fn generate_entropy(&self) -> Vec<u8> {
        let mut rng = OsRng {};
        let entropy_bytes_count = self.mnemonic_type.bytes();

        let mut entropy = vec![0u8; entropy_bytes_count];// empty vector with [0, 0, 0, 0, 0 ... entropy_size] u8 mean unsigned 8 bit integer

        // Fill the vector with random bytes
        rng.fill_bytes(&mut entropy);
        entropy
    }

    pub fn checksum(&self, entropy: &Vec<u8>) -> String {
        let hash = digest(entropy);

        if hash.len() < 2 {
            panic!("Hash must be at least 2 characters.");
        }

        let checksum_bits = self.mnemonic_type.bits() / 32;
        let checksum_index = if checksum_bits == 4 {1} else if checksum_bits == 8 {2} else {0};

        let checksum = &hash[..checksum_index];
        String::from(checksum)
    }

}