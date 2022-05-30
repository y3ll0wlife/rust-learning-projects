pub fn get_symbol(value: i8) -> char {
    match value {
        0 => ' ',
        1 => 'O',
        2 => 'X',
        _ => panic!(
            "Invalid value: '{}' can only value of '0', '1' or '2'",
            value
        ),
    }
}

pub fn print_board(board: [i8; 9]) {
    println!(
        "{} | {} | {}",
        get_symbol(board[0]),
        get_symbol(board[1]),
        get_symbol(board[2])
    );
    println!("-----------");
    println!(
        "{} | {} | {}",
        get_symbol(board[3]),
        get_symbol(board[4]),
        get_symbol(board[5])
    );
    println!("-----------");
    println!(
        "{} | {} | {}",
        get_symbol(board[6]),
        get_symbol(board[7]),
        get_symbol(board[8])
    );
}
