use regex::Regex;

use crate::utils::data::Stem;
use crate::utils::key_translator::translate_part_of_speech;
use crate::{latin_to_english::Word, Language, Translation, TranslationType};

use super::data::{EnglishWordInfo, Form, Inflection, LatinWordInfo, LongForm, WordInfo, UniqueLatinWordInfo};
use super::key_translator::{
    translate_age, translate_area, translate_declension, translate_frequency, translate_gender,
    translate_mood, translate_noun, translate_number, translate_pronoun, translate_source,
    translate_tense, translate_verb, translate_voice,
};
use super::principle_part_generator::{generate_for_nouns, generate_for_verbs};

pub fn format_output(
    mut translation_output: Vec<Translation>,
    language: Language,
    clean: bool,
) -> Vec<Translation> {
    for translation in &mut translation_output {
        if language == Language::English {
            if let TranslationType::English(info) = &mut translation.definitions {
                for english_word_info in info.iter_mut() {
                    *english_word_info = format_english_word(english_word_info.clone(), clean);
                }
            } else {
                panic!("Invalid TranslationType for English language.");
            }
        } else if language == Language::Latin {
            if let TranslationType::Latin(info) = &mut translation.definitions {
                for latin_word_info in info.iter_mut() {
                    if let Word::LatinWordInfo(latin_word_info) = &mut latin_word_info.word {
                        *latin_word_info = format_latin_word_info(latin_word_info.clone(), clean);
                    } else if let Word::UniqueLatinWordInfo(unique_latin_word_info) =
                        &mut latin_word_info.word
                    {
                        *unique_latin_word_info = format_unique_latin_word_info(unique_latin_word_info.clone(), clean);
                    } else {
                        panic!("Invalid Word type for Latin language.");
                    }
                    latin_word_info.stem = format_latin_stem(latin_word_info.stem.clone(), clean);
                    let pos = latin_word_info.stem.pos.clone();
                    latin_word_info.inflections =
                        format_latin_inflections(latin_word_info.inflections.clone(), pos, clean);
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

fn format_english_word(english_word: EnglishWordInfo, clean: bool) -> EnglishWordInfo {
    let mut clean_english_word: EnglishWordInfo = english_word;

    clean_english_word.pos = translate_part_of_speech(&clean_english_word.pos[..]).to_string();
    clean_english_word.frequency_type =
        translate_frequency(&clean_english_word.frequency_type[..]).to_string();
    clean_english_word.latin_entry = clean_english_word
        .latin_entry
        .as_ref()
        .and_then(|latin_word_info| Some(format_latin_word_info(latin_word_info.clone(), clean)));

    clean_english_word
}

fn format_latin_word_info(latin_word_info: LatinWordInfo, clean: bool) -> LatinWordInfo {
    let mut clean_latin_word_info: LatinWordInfo = latin_word_info;

    clean_latin_word_info.pos =
        translate_part_of_speech(&clean_latin_word_info.pos[..]).to_string();
    clean_latin_word_info.info = format_word_info(clean_latin_word_info.info);
    clean_latin_word_info.form = Form::LongForm(translate_latin_word_info_form(
        match clean_latin_word_info.form.clone() {
            Form::LongForm(_form) => "".to_string(),
            Form::StrForm(form) => form,
        },
        clean_latin_word_info.pos.clone(),
        clean,
    ));

    if clean_latin_word_info.pos == "noun" {
        clean_latin_word_info.parts = generate_for_nouns(
            clean_latin_word_info.n.clone(),
            match clean_latin_word_info.form.clone() {
                Form::LongForm(_form) => "".to_string(),
                Form::StrForm(form) => form,
            },
            clean_latin_word_info.parts,
        )
    } else if clean_latin_word_info.pos == "adjective" {
        println!("that")
    } else if clean_latin_word_info.pos == "verb" || clean_latin_word_info.pos == "participle" {
        clean_latin_word_info.parts =
            generate_for_verbs(clean_latin_word_info.n.clone(), clean_latin_word_info.parts)
    }

    clean_latin_word_info.orth = clean_latin_word_info.parts[0].clone();

    clean_latin_word_info
}

fn format_unique_latin_word_info(unique_latin_word_info: UniqueLatinWordInfo, clean: bool) -> UniqueLatinWordInfo {
    let mut clean_unique_latin_word_info: UniqueLatinWordInfo = unique_latin_word_info;

    clean_unique_latin_word_info.pos =
        translate_part_of_speech(&clean_unique_latin_word_info.pos[..]).to_string();
    clean_unique_latin_word_info.info = format_word_info(clean_unique_latin_word_info.info);
    clean_unique_latin_word_info.form = Form::LongForm(translate_latin_word_info_form(
        match clean_unique_latin_word_info.form.clone() {
            Form::LongForm(_form) => "".to_string(),
            Form::StrForm(form) => form,
        },
        clean_unique_latin_word_info.pos.clone(),
        clean,
    ));

    clean_unique_latin_word_info
}


fn translate_latin_word_info_form(form: String, pos: String, clean: bool) -> LongForm {
    let form_array = form.split_whitespace().collect::<Vec<&str>>();
    let mut clean_form: LongForm = LongForm::new();

    if pos == "unknown" {
        return clean_form;
    }

    if pos == "numeral" {
        clean_form.kind = Some("numeral".to_string());
        return clean_form;
    }

    //???: should be other stuff here too i think
    if form_array.len() < 2 {
        clean_form.kind = Some("part of speech".to_string());
        return clean_form;
    }

    let word_type: String = form_array[2].to_string();

    if pos == "noun" {
        clean_form.gender = Some(translate_gender(&word_type[..]).to_string());
        clean_form.kind = Some(translate_noun(&form_array[3]).to_string());
    } else if pos == "verb" || pos == "participle" {
        clean_form.verb = Some(translate_verb(&word_type[..]).to_string());
    } else if pos == "pronoun" || pos == "packon" {
        clean_form.voice = Some(translate_pronoun(&word_type[..]).to_string());
    }

    if !clean {
        clean_form = fill_in_form_blank(clean_form);
    }

    return clean_form;
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

fn format_latin_stem(latin_stem: Stem, clean: bool) -> Stem {
    let mut clean_latin_stem: Stem = latin_stem;

    clean_latin_stem.pos = translate_part_of_speech(&clean_latin_stem.pos[..]).to_string();
    clean_latin_stem.form = Form::LongForm(translate_latin_word_info_form(
        match clean_latin_stem.form.clone() {
            Form::LongForm(_form) => "".to_string(),
            Form::StrForm(form) => form,
        },
        clean_latin_stem.pos.clone(),
        clean,
    ));

    clean_latin_stem
}

fn format_latin_inflections(
    inflections: Vec<Inflection>,
    pos: String,
    clean: bool,
) -> Vec<Inflection> {
    let mut clean_inflections: Vec<Inflection> = Vec::new();
    let cleaned_inflections = remove_inflections_without_endings(inflections);

    for inflection in &cleaned_inflections {
        let mut clean_inflection: Inflection = inflection.clone();

        clean_inflection.pos = translate_part_of_speech(&clean_inflection.pos[..]).to_string();
        clean_inflection.ending = clean_inflection.ending.trim().to_string();
        clean_inflection.form = Form::LongForm(format_form(
            match clean_inflection.form.clone() {
                Form::LongForm(_form) => "".to_string(),
                Form::StrForm(form) => form,
            },
            clean_inflection.pos.clone(),
            clean,
        ));

        if clean {
            clean_inflection.note = clean_inflection.note.filter(|note| !note.is_empty());
        }

        clean_inflections.push(clean_inflection);
    }

    clean_inflections = remove_inflections_with_wrong_pos(clean_inflections, pos);

    if clean {
        clean_inflections = remove_vague_inflections(clean_inflections);
    }

    clean_inflections
}
fn remove_inflections_without_endings(inflections: Vec<Inflection>) -> Vec<Inflection> {
    let mut clean_inflections: Vec<Inflection> = Vec::new();

    for inflection in inflections {
        if inflection.ending != "" {
            clean_inflections.push(inflection);
        }
    }

    clean_inflections
}

// Canis generates with a pos of "verb", but is a noun. This removes those.
fn remove_inflections_with_wrong_pos(inflections: Vec<Inflection>, pos: String) -> Vec<Inflection> {
    let mut clean_inflections: Vec<Inflection> = Vec::new();

    for inflection in inflections {
        if inflection.pos == pos {
            clean_inflections.push(inflection);
        }
    }

    clean_inflections
}

fn remove_vague_inflections(inflections: Vec<Inflection>) -> Vec<Inflection> {
    let mut clean_inflections: Vec<Inflection> = Vec::new();

    for inflection in inflections {
        let clean_form: LongForm = match inflection.form.clone() {
            Form::LongForm(clean_form) => clean_form,
            Form::StrForm(_) => LongForm::new(),
        };
        if clean_form.gender.as_deref() != Some(&"unknown".to_string()) {
            clean_inflections.push(inflection);
        }
    }

    clean_inflections
}

fn format_form(form: String, pos: String, clean: bool) -> LongForm {
    let mut clean_form = LongForm::new();
    let form_array = form.split_whitespace().collect::<Vec<&str>>();

    if pos == "noun" || pos == "pronoun" || pos == "adjective" || pos == "numeral" {
        // Ex: "FUT   ACTIVE  IND  3 S" -> "future active indicative third singular"
        if form_array.len() == 3 {
            clean_form.declension = Some(translate_declension(form_array[0]).to_string());
            clean_form.number = Some(translate_number(form_array[1]).to_string());
            clean_form.gender = Some(translate_gender(form_array[2]).to_string());
        }
    } else if pos == "verb" {
        // Ex. "FUT   ACTIVE  IND  3 S" -> "future active indicative third singular"
        if form_array.len() == 5 {
            clean_form.tense = Some(translate_tense(form_array[0]).to_string());
            clean_form.voice = Some(translate_voice(form_array[1]).to_string());
            clean_form.mood = Some(translate_mood(form_array[2]).to_string());
            clean_form.person = Some(form_array[3].parse::<i8>().unwrap_or(0));
            clean_form.number = Some(translate_number(form_array[4]).to_string());
        }
    } else if pos == "participle" {
        // Ex: "VOC P N PRES ACTIVE  PPL" -> "vocative plural neuter present active participle"
        if form_array.len() == 5 {
            clean_form.declension = Some(translate_declension(form_array[0]).to_string());
            clean_form.number = Some(translate_number(form_array[1]).to_string());
            clean_form.gender = Some(translate_gender(form_array[2]).to_string());
            clean_form.tense = Some(translate_tense(form_array[3]).to_string());
            clean_form.voice = Some(translate_voice(form_array[4]).to_string());
        }
    }

    if !clean {
        clean_form = fill_in_form_blank(clean_form);
    }

    clean_form
}

fn fill_in_form_blank(mut clean_form: LongForm) -> LongForm {
    clean_form.declension.get_or_insert("".to_string());
    clean_form.number.get_or_insert("".to_string());
    clean_form.gender.get_or_insert("".to_string());
    clean_form.tense.get_or_insert("".to_string());
    clean_form.voice.get_or_insert("".to_string());
    clean_form.mood.get_or_insert("".to_string());
    clean_form.verb.get_or_insert("".to_string());
    clean_form.kind.get_or_insert("".to_string());
    clean_form.person.get_or_insert(0);
    clean_form
}

pub fn sanitize_word(word: &str) -> String {
    let mut word = word.to_owned();
    word = word.trim().to_lowercase();
    let re = Regex::new(r"[^a-z ]|\d|\s+").unwrap();
    word = re.replace_all(&word, " ").to_string();
    word
}
