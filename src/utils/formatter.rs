use regex::Regex;

use crate::utils::data::Stem;
use crate::utils::key_translator::translate_part_of_speech;
use crate::{Language, Translation, TranslationType, latin_to_english::Word};

use super::data::{EnglishWordInfo, LatinWordInfo, WordInfo};
use super::key_translator::{translate_age, translate_area, translate_frequency, translate_source};

pub fn format_output(
    mut translation_output: Vec<Translation>,
    language: Language,
) -> Vec<Translation> {
    for translation in &mut translation_output {
        if language == Language::English {
            if let TranslationType::English(info) = &mut translation.definitions {
                for english_word_info in info.iter_mut() {
                    *english_word_info = format_english_word(english_word_info.clone());
                }
            } else {
                panic!("Invalid TranslationType for English language.");
            }
        } else if language == Language::Latin {
            if let TranslationType::Latin(info) = &mut translation.definitions {
                for latin_word_info in info.iter_mut() {
                    if let Word::LatinWordInfo(latin_word_info) = &mut latin_word_info.word {
                        *latin_word_info = format_latin_word_info(latin_word_info.clone());
                    } else if let Word::UniqueLatinWordInfo(unique_latin_word_info) =
                        &mut latin_word_info.word
                    {
                        //*unique_latin_word_info = format_latin_word_info(unique_latin_word_info.clone());
                        println!("unique_latin_word_info: {:?}", unique_latin_word_info);
                    } else {
                        panic!("Invalid Word type for Latin language.");
                    }

                    latin_word_info.stem = format_latin_stem(latin_word_info.stem.clone());
                }
            } else {
                panic!("Invalid TranslationType for Latin language.");
            }
        } else {
            panic!("Language not supported");
        }
    }

    translation_output
}


fn format_english_word(english_word: EnglishWordInfo) -> EnglishWordInfo {
    let mut clean_english_word: EnglishWordInfo = english_word;

    clean_english_word.pos = translate_part_of_speech(&clean_english_word.pos[..]).to_string();
    clean_english_word.frequency_type =
        translate_frequency(&clean_english_word.frequency_type[..]).to_string();
    clean_english_word.latin_entry = clean_english_word
        .latin_entry
        .as_ref()
        .and_then(|latin_word_info| Some(format_latin_word_info(latin_word_info.clone())));

    clean_english_word
}

fn format_latin_word_info(latin_word_info: LatinWordInfo) -> LatinWordInfo {
    let mut clean_latin_word_info: LatinWordInfo = latin_word_info;

    clean_latin_word_info.pos =
        translate_part_of_speech(&clean_latin_word_info.pos[..]).to_string();
    clean_latin_word_info.info = format_word_info(clean_latin_word_info.info);

    clean_latin_word_info
}

fn format_word_info(word_info: WordInfo) -> WordInfo {
    let mut clean_word_info: WordInfo = word_info;

    clean_word_info.age = translate_age(&clean_word_info.age[..]).to_string();
    clean_word_info.area = translate_area(&clean_word_info.area[..]).to_string();
    clean_word_info.geo = translate_area(&clean_word_info.geo[..]).to_string();
    clean_word_info.freq = translate_frequency(&clean_word_info.freq[..]).to_string();
    clean_word_info.source = translate_source(&clean_word_info.source[..]).to_string();

    clean_word_info
}

fn format_latin_stem(latin_stem: Stem) -> Stem {
    let mut clean_latin_stem: Stem = latin_stem;

    clean_latin_stem.pos = translate_part_of_speech(&clean_latin_stem.pos[..]).to_string();

    clean_latin_stem
}

fn format_latin_inflections() {}

fn remove_inflections_without_endings() {}

pub fn sanitize_word(word: &str) -> String {
    let mut word = word.to_owned();
    word = word.trim().to_lowercase();
    let re = Regex::new(r"[^a-z ]|\d|\s+").unwrap();
    word = re.replace_all(&word, " ").to_string();
    word
}
