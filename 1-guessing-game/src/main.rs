use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let top = 100;
    let bot = 0;

    let mut rng = rand::thread_rng();
    let secret: u32 = rng.gen_range(bot..=top);

    let mut guesses: Vec<u32> = Vec::new();
    let mut amount_guesses: u32 = 0;
    let mut suggested_guess: u32 = top / 2;

    println!("Guess:");

    loop {
        println!("You should guess: {}", suggested_guess);

        /*let mut guess_string = String::new();
        stdin().read_line(&mut guess_string).unwrap();
        let guess = match guess_string.trim().parse::<u32>() {
            Ok(num) => num
            Err(err) => {
                println!("Error {} and the number is '{}'", err, guess_string);
                continue;
            }
        };*/
        let guess = suggested_guess;
        guesses.push(guess);
        amount_guesses += 1;

        print!("Previous guesses: ");
        for i in &guesses {
            print!("[{}] ", i);
        }

        match guess.cmp(&secret) {
            Ordering::Less => {
                suggested_guess = (suggested_guess as f32 * 1.5).floor() as u32;
                println!("\nToo small")
            }
            Ordering::Greater => {
                suggested_guess = (suggested_guess as f32 * 0.5).floor() as u32;
                println!("\nToo Big")
            }
            Ordering::Equal => {
                println!("\nYOU WIN! it only took {} guesses", amount_guesses);
                break;
            }
        }
    }
}
