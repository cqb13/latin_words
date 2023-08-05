//TODO: add word mod, if word ends in e, try ae
use crate::utils::tricks::trick_list::match_tricks_list;
use crate::utils::tricks::trick_list::Trick;
use crate::utils::tricks::word_mods::{flip, flip_flop};

#[derive(Debug, Clone)]
pub enum Operation {
    FlipFlop,
    Flip,
    Internal,
    Slur,
}

pub fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

pub fn is_roman_digit(c: char) -> bool {
    match c.to_ascii_uppercase() {
        'I' | 'V' | 'X' | 'L' | 'C' | 'D' | 'M' => true,
        _ => false,
    }
}

pub fn is_roman_number(possible_roman_number: &str) -> bool {
    possible_roman_number.chars().all(is_roman_digit)
}

pub fn is_common_prefix(prefix: String) -> bool {
    let constant_prefixes = [
        "dis", "ex", "in", "per", "prae", "pro", "re", "si", "sub", "super", "trans",
    ];

    constant_prefixes.contains(&prefix.as_str())
}

pub fn translate_roman_digit_to_number(c: char) -> u32 {
    match c.to_ascii_uppercase() {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => panic!("Invalid roman digit: {}", c),
    }
}

pub fn evaluate_roman_numeral(roman_numeral: &str) -> u32 {
    let mut result = 0;
    let mut last_digit = 0;
    for c in roman_numeral.chars().rev() {
        let digit = translate_roman_digit_to_number(c);
        if digit < last_digit {
            result -= digit;
        } else {
            result += digit;
        }
        last_digit = digit;
    }
    result
}

pub fn try_tricks(mut word: String) -> String {
    let trick_chars = [
        'a', 'd', 'e', 'f', 'g', 'h', 'k', 'l', 'm', 'n', 'o', 'p', 's', 't', 'u', 'y', 'z',
    ];
    let _slur_trick_chars = ['a', 'c', 'i', 'n', 'o', 'q', 's'];

    let first_char = word.chars().next().unwrap();
    if trick_chars.contains(&first_char) {
        let trick_list = get_trick_lists(word.to_string());
        let mut max_attempts = 0;
        while max_attempts < 2 {
            word = iterate_over_tricks(trick_list.clone(), word.to_string());
            max_attempts += 1;
        }
    }

    word
}

fn get_trick_lists(word: String) -> Vec<Trick> {
    let first_char = word.chars().next().unwrap();

    let trick_list = match_tricks_list(first_char);

    trick_list
}

fn iterate_over_tricks(trick_list: Vec<Trick>, mut word: String) -> String {
    // word should be modified after each operation is applied.
    for trick in trick_list.iter() {
        word = match trick.operation {
            Operation::FlipFlop => flip_flop(trick.flip_flop1, trick.flip_flop2, &word),
            Operation::Flip => flip(trick.flip_flip3, trick.flip_flip4,& word),
            Operation::Internal => return word, //internal(trick.internal1, trick.internal2),
            Operation::Slur => return word,     // Assuming Slur causes an exception
        };
    }

    word
}
