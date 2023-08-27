// based on: https://github.com/mk270/whitakers-words/blob/9b11477e53f4adfb17d6f6aa563669dc71e0a680/src/support_utils/support_utils-dictionary_form.adb
use std::vec;

use crate::data::data::NValue;

pub enum Comparison {
    POS,
    COMP,
    SUPER,
    X,
}

pub fn generate_for_nouns(
    number_types: Vec<NValue>,
    gender: String,
    parts: Vec<String>,
) -> Vec<String> {
    let (num_type_1, num_type_2) = translate_number_types(number_types);

    match (num_type_1, num_type_2) {
        // first declension
        (1, 1) => set_principle_parts(parts, vec!["a", "ae"], None),
        // greek nouns
        (1, 6) => set_principle_parts(parts, vec!["e", "es"], None),
        (1, 7) => set_principle_parts(parts, vec!["es", "ae"], None),
        (1, 8) => set_principle_parts(parts, vec!["as", "ae"], None),
        // second declension
        (2, 1) => set_principle_parts(parts, vec!["us", "i"], None),
        (2, 2) => set_principle_parts(parts, vec!["um", "i"], None),
        (2, 3) => set_principle_parts(parts, vec!["", "i"], None),
        (2, 4) => {
            if gender == "M" {
                set_principle_parts(parts, vec!["us", "(i)"], None)
            } else if gender == "N" {
                set_principle_parts(parts, vec!["um", "(i)"], None)
            } else {
                parts
            }
        }
        (2, 5) => set_principle_parts(parts, vec!["us", ""], None),
        (2, 6) | (2, 7) => set_principle_parts(parts, vec!["os", "i"], None),
        (2, 8) => set_principle_parts(parts, vec!["on", "i"], None),
        (2, 9) => set_principle_parts(parts, vec!["us", "i"], None),
        // third declension
        (3, 1) | (3, 2) | (3, 3) | (3, 4) => set_principle_parts(parts, vec!["", "is"], None),
        (3, 7) | (3, 9) => set_principle_parts(parts, vec!["", "os/is"], None),
        // fourth declension
        (4, 1) => set_principle_parts(parts, vec!["us", "us"], None),
        (4, 2) => set_principle_parts(parts, vec!["u", "us"], None),
        (4, 3) => set_principle_parts(parts, vec!["us", "u"], None),
        // fifth declension
        (5, 1) => set_principle_parts(parts, vec!["es", "ei"], None),
        // special
        (9, 8) => set_principle_parts(parts, vec!["", ""], Some("abbreviation")),
        (9, 9) => set_principle_parts(parts, vec!["", ""], Some("undeclined")),
        _ => parts,
    }
}

pub fn generate_for_pronouns(number_types: Vec<NValue>, parts: Vec<String>) -> Vec<String> {
    let (num_type_1, num_type_2) = translate_number_types(number_types);

    match (num_type_1, num_type_2) {
        // proximal demonstrative pronouns (hic, haec hoc)
        (3, 1) => set_principle_parts(parts, vec!["ic", "aec", "oc"], None),
        (3, 2) => set_principle_parts(parts, vec!["ic", "aec", "uc"], None),

        (4, 1) => set_principle_parts(parts, vec!["s", "a", "d"], None),
        (4, 2) => set_principle_parts(parts, vec!["dem", "adem", "dem"], None),
        // Distal (ille, illa, illud) and medial (iste, ista, istud)
        // demonstrative pronouns
        (6, 1) => set_principle_parts(parts, vec!["e", "a", "ud"], None),
        (6, 2) => set_principle_parts(parts, vec!["e", "a", "um"], None),
        // special
        (9, 8) => set_principle_parts(parts, vec!["", "", ""], Some("abbreviation")),
        (9, 9) => set_principle_parts(parts, vec!["", "", ""], Some("undeclined")),
        _ => parts,
    }
}

pub fn generate_for_adjectives(
    number_types: Vec<NValue>,
    parts: Vec<String>,
    comparison: Comparison,
) -> Vec<String> {
    let (num_type_1, num_type_2) = translate_number_types(number_types);

    match comparison {
        Comparison::COMP => set_principle_parts(parts, vec!["or", "or", "us"], None),
        Comparison::SUPER => set_principle_parts(parts, vec!["mus", "ma", "mum"], None),
        Comparison::POS => {
            match (num_type_1, num_type_2) {
                // first declension
                (1, 1) => set_principle_parts(parts, vec!["us", "a", "um"], None),
                (1, 2) | (1, 4) => set_principle_parts(parts, vec!["", "a", "um"], None),
                (1, 3) => set_principle_parts(parts, vec!["us", "a", "um (gen -ius)"], None),
                (1, 5) => set_principle_parts(parts, vec!["us", "a", "ud"], None),
                // second declension
                (2, 1) => set_principle_parts(parts, vec!["", "e", ""], None),
                (2, 2) => set_principle_parts(parts, vec!["", "a", ""], None),
                (2, 3) => set_principle_parts(parts, vec!["es", "es", "es"], None),
                (2, 6) => set_principle_parts(parts, vec!["os", "os", ""], None),
                (2, 7) => set_principle_parts(parts, vec!["os", "", ""], None),
                (2, 8) => set_principle_parts(parts, vec!["", "", "on"], None),
                // third declension
                (3, 1) => set_principle_parts(parts, vec!["", "", "is"], None),
                (3, 2) => set_principle_parts(parts, vec!["is", "is", "e"], None),
                (3, 3) => set_principle_parts(parts, vec!["", "is", "e"], None),
                (3, 6) => set_principle_parts(parts, vec!["", "", "os"], None),
                // special
                (9, 8) => set_principle_parts(parts, vec!["", "", ""], Some("abbreviation")),
                (9, 9) => set_principle_parts(parts, vec!["", "", ""], Some("undeclined")),
                _ => parts,
            }
        }
        Comparison::X => {
            match (num_type_1, num_type_2) {
                // unknown first declension
                (1, 1) => set_principle_parts(
                    parts,
                    vec!["us", "a -um", "or -or -us", "mus -a -um"],
                    None,
                ),
                (1, 2) => {
                    set_principle_parts(parts, vec!["", "a -um", "or -or -us", "mus -a -um"], None)
                }
                // unknown third declension
                (3, 1) => set_principle_parts(
                    parts,
                    vec!["", "is (gen .)", "or -or -us", "mus -a -um"],
                    None,
                ),
                (3, 2) => {
                    set_principle_parts(parts, vec!["is", "e", "or -or -us", "mus -a -um"], None)
                }
                (3, 3) => {
                    set_principle_parts(parts, vec!["", "is -e", "or -or -us", "mus -a -um"], None)
                }
                // special
                (9, 8) => set_principle_parts(parts, vec!["", "", ""], Some("abbreviation")),
                (9, 9) => set_principle_parts(parts, vec!["", "", ""], Some("undeclined")),
                _ => parts,
            }
        }
    }
}

pub fn generate_for_verbs(number_types: Vec<NValue>, parts: Vec<String>) -> Vec<String> {
    let (num_type_1, num_type_2) = translate_number_types(number_types);

    //TODO account for 0 in num2

    match (num_type_1, num_type_2) {
        (1, 1) => set_principle_parts(parts, vec!["o", "are", "i", "us"], None),
        (2, 1) => set_principle_parts(parts, vec!["eo", "ere", "i", "us"], None),
        (3, 1) => set_principle_parts(parts, vec!["o", "ere", "i", "us"], None),
        (3, 2) => set_principle_parts(parts, vec!["o", "re", "i", "us"], None),
        (3, 3) => set_principle_parts(parts, vec!["o", "eri", "i", "us"], None),
        (3, 4) => set_principle_parts(parts, vec!["o", "ire", "i", "us"], None),
        (5, 2) => set_principle_parts(parts, vec!["um", "esse", "i", ""], None),
        (6, 1) => set_principle_parts(parts, vec!["o", "re", "i", "us"], None),
        (6, 2) => set_principle_parts(parts, vec!["o", "le", "i", ""], None),
        (7, 1) => set_principle_parts(parts, vec!["o", "", "", ""], None),
        (7, 2) => set_principle_parts(parts, vec!["am", "iam", "", ""], None),
        (7, 3) => set_principle_parts(parts, vec!["o", "se", "", ""], None),
        (8, 1) => set_principle_parts(parts, vec!["o", "are", "i", ""], None),
        (8, 2) => set_principle_parts(parts, vec!["o", "ere", "", ""], None),
        (8, 3) => set_principle_parts(parts, vec!["o", "ere", "i", ""], None),
        (9, 9) => set_principle_parts(parts, vec!["", "", "", ""], Some("undeclined")),
        _ => parts,
    }
}

fn set_principle_parts(
    parts: Vec<String>,
    endings: Vec<&str>,
    special_case: Option<&str>,
) -> Vec<String> {
    let mut principle_parts = Vec::new();

    if endings.iter().all(|x| *x == "") {
        return vec![parts[0].clone() + " | " + special_case.unwrap_or("")];
    }

    for (i, part) in parts.iter().enumerate() {
        if part == "" || part == "zzz" {
            principle_parts.push("---".to_string());
        } else {
            principle_parts.push(part.clone() + endings[i]);
        }
    }

    principle_parts
}

fn translate_number_types(number_types: Vec<NValue>) -> (i8, i8) {
    let num_type_1 = match &number_types.get(0) {
        Some(NValue::Integer(num)) => *num,
        _ => 0,
    };

    let num_type_2 = match &number_types.get(1) {
        Some(NValue::Integer(num)) => *num,
        _ => 0,
    };

    (num_type_1, num_type_2)
}
