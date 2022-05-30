mod logic;
mod visual;

use rand::{thread_rng, Rng};
use std::io::stdin;

fn main() {
    let mut rng = thread_rng();

    // player 1 = O
    // player 2 = X
    let mut current_player: i8 = rng.gen_range(1..=2);
    let mut board: [i8; 9] = [0; 9];

    println!(
        "Welcome to Tic-Tac-Toe, player to start is {}",
        visual::get_symbol(current_player)
    );

    visual::print_board(board);

    loop {
        println!("Player {} turn ", visual::get_symbol(current_player));

        loop {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            match input.trim().parse() {
                Ok(num) => {
                    if logic::valid_move(board, num) {
                        board[num - 1] = current_player;
                        break;
                    } else {
                        println!("Invalid move, please choice another one")
                    }
                }
                Err(_) => {
                    println!("Invalid move, please choice another one")
                }
            };
        }

        visual::print_board(board);

        if logic::did_user_win(board, current_player) {
            println!("Player {} won!!!", visual::get_symbol(current_player));
            break;
        }

        current_player = if current_player == 1 { 2 } else { 1 };
    }
}
