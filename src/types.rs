use crate::{MAX_WORDS, MIN_WORDS};



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