use clap::{App, Arg};
use latin_to_english::LatinTranslationInfo;
use serde::{Deserialize, Serialize};
use serde_json;
use utils::data::EnglishWordInfo;

mod english_to_latin;
mod latin_to_english;
pub mod utils {
    pub mod data;
    pub mod key_translator;
    pub mod principle_part_generator;
    pub mod tricks;
}

#[derive(Serialize, Deserialize)]
struct LatinTranslation {
    word: String,
    def: Vec<LatinTranslationInfo>,
}

#[derive(Serialize, Deserialize)]
struct EnglishTranslation {
    word: String,
    def: Vec<EnglishWordInfo>,
}

//TODO add a param -m for experimental feature of trying to predict where macrons should be in the word
fn main() {
    let matches = App::new("Translator CLI")
        .version("0.1.0")
        .author("cqb13")
        .about("A CLI for interacting with the Whitaker's Words Dictionary")
        .subcommand(
            App::new("transEng")
                .about("Translate Latin to English")
                .arg(
                    Arg::with_name("text")
                        .help("The Latin text to translate to English")
                        .required(true),
                )
                .arg(
                    Arg::with_name("formatted")
                        .short('f')
                        .long("formatted")
                        .help("Determines if the output should be formatted")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("predict macrons")
                        .short('m')
                        .long("predict macrons")
                        .help("Tries to predict where macrons should be in the word")
                        .takes_value(false),
                ),
        )
        .subcommand(
            App::new("transLat")
                .about("Translate English to Latin")
                .arg(
                    Arg::with_name("text")
                        .help("The English text to translate to Latin")
                        .required(true),
                )
                .arg(
                    Arg::with_name("formatted")
                        .short('f')
                        .long("formatted")
                        .help("Determines if the output should be formatted")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("predict macrons")
                        .short('m')
                        .long("predict macrons")
                        .help("Tries to predict where macrons should be in the word")
                        .takes_value(false),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("transEng", trans_eng_matches)) => {
            let text = trans_eng_matches.value_of("text").unwrap();
            let formatted_output = trans_eng_matches.is_present("formatted");
            translate_to_latin(text, formatted_output);
        }
        Some(("transLat", trans_lat_matches)) => {
            let text = trans_lat_matches.value_of("text").unwrap();
            translate_to_english(text);
        }
        _ => println!("Please provide a valid command: transEng or transLat"),
    }
}

fn translate_to_english(latin_text: &str) {
    let latin_words: Vec<&str> = latin_text.split(" ").collect();
    let mut translations = Vec::new();

    for latin_word in latin_words {
        let output = latin_to_english::translate_to_english(latin_word);
        if output.len() > 0 {
            translations.push(LatinTranslation {
                word: latin_word.to_string(),
                def: output,
            });
        }
    }

    let json_output = serde_json::to_string_pretty(&translations).unwrap();
    println!("{}", json_output);
}

fn translate_to_latin(english_text: &str, formatted_output: bool) {
    let english_words: Vec<&str> = english_text.split(" ").collect();
    let mut translations: Vec<EnglishTranslation> = Vec::new();

    for word in english_words {
        let output = english_to_latin::translate_to_latin(word);
        if output.len() > 0 {
            translations.push(EnglishTranslation {
                word: word.to_string(),
                def: output,
            });
        }
    }

    if formatted_output {
        // formatting code
    }
    let json_output = serde_json::to_string_pretty(&translations).unwrap();
    println!("{}", json_output);
}
