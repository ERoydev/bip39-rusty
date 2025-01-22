use std::fmt;

mod chinese_simplified;
mod chinese_traditional;
mod czech;
mod english;
mod french;
mod italian;
mod japanese;
mod korean;
mod portuguese;
mod spanish;


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

impl Language {
    pub const fn get_predefined_word_list(language: &Language) -> [&'static str; 2048] {
        match language {
            Language::ChineseSimplified => chinese_simplified::WORDS,
            Language::ChineseTraditional => chinese_traditional::WORDS,
            Language::Czech => czech::WORDS,
            Language::English => english::WORDS,
            Language::French => french::WORDS,
            Language::Italian => italian::WORDS,
            Language::Japanese => japanese::WORDS,
            Language::Portuguese => portuguese::WORDS,
            Language::Spanish => spanish::WORDS,
            Language::Korean => korean::WORDS,
        }
    }
}