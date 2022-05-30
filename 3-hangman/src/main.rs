mod logic;

use rand::Rng;
use std::io::stdin;

fn main() {
    let words = logic::file_to_vec("./src/wordlist.txt".to_string());
    let mut rng = rand::thread_rng();
    let secret_word = &words[rng.gen_range(0..=words.len())];

    let mut game_state: Vec<char> = secret_word.chars().map(|_| '_').collect();
    let mut guesses: Vec<char> = Vec::new();
    let mut guesses_count: usize = 0;
    const MAX_GUESS: usize = 5;

    println!("secret word: {}", secret_word);

    loop {
        let mut input: String = String::new();
        stdin().read_line(&mut input).unwrap();

        let guess: char = input.chars().nth(0).expect("cant read input");

        let success = logic::make_guess(
            &mut game_state,
            secret_word,
            guess,
            &mut guesses_count,
            &mut guesses,
        );

        if success {
            let game_state_str: String = game_state.iter().cloned().collect();

            println!("{}", game_state_str);
            println!("Previous guesses: {:?}", guesses);
            println!("You guessed: {}", guess);

            if game_state_str == *secret_word {
                println!("You won!");
                break;
            } else {
                if guesses_count > MAX_GUESS {
                    println!("You loose :(");
                    break;
                }
            }
        } else {
            println!("Invalid guess")
        }
    }
}
