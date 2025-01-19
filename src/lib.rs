use rand::rngs::OsRng;
use rand::RngCore;


mod language;
pub use language::Language;

const MIN_WORDS: usize = 12;
const MAX_WORDS: usize = 24;

pub struct Mnemonic {
    lang: Language,
    mnemonic_type: MnemonicType,
}

pub enum MnemonicType {
    Twelve, // 128 bits of entropy -> 16 bytes (128 bits / 8)
    TwentyFour // 256 bits of entropy -> 32 bytes (256 bits / 8)
}


impl Mnemonic {

    pub fn new(lang: Language, mnemonic_type: MnemonicType) -> Mnemonic {
        let words: usize = match mnemonic_type {
            MnemonicType::Twelve => MIN_WORDS,
            MnemonicType::TwentyFour => MAX_WORDS,
        };

        Mnemonic {
            lang,
            mnemonic_type,
        }
    }

    pub fn generate_entropy(&self) -> Vec<u8> {
        let mut rng = OsRng {};
        let entropy_size = match self.mnemonic_type {
            MnemonicType::Twelve => 16, // 12-word mnemonic needs 128 bits (16 bytes)
            MnemonicType::TwentyFour => 32, // 24-word mnemonic needs 256 bits (32 bytes)
        };

        let mut entropy = vec![0u8; entropy_size];// empty vector with [0, 0, 0, 0, 0 ... entropy_size]

        // Fill the vector with random bytes
        rng.fill_bytes(&mut entropy);
        entropy
    }
}