use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn file_to_vec(filename: String) -> Vec<String> {
    let file = File::open(filename).expect("no such file exist");
    let buffer = BufReader::new(file);

    buffer
        .lines()
        .map(|l| l.expect("cant parse line"))
        .collect()
}

pub fn make_guess(
    game_state: &mut Vec<char>,
    secret_word: &String,
    guess: char,
    guesses_count: &mut usize,
    guesses: &mut Vec<char>,
) -> bool {
    let mut did_find = false;

    if guesses.contains(&guess) {
        return false;
    }

    for (i, c) in secret_word.chars().enumerate() {
        if c == guess {
            did_find = true;
            game_state[i] = c;
        }
    }

    if !did_find {
        guesses.insert(*guesses_count, guess);
        *guesses_count += 1;
    }

    true
}
