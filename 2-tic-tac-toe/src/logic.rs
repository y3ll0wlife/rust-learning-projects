pub fn did_user_win(board: [i8; 9], player: i8) -> bool {
    // top row
    if board[0] == player && board[1] == player && board[2] == player {
        return true;
    }

    // middle row
    if board[3] == player && board[4] == player && board[5] == player {
        return true;
    }

    // last row
    if board[6] == player && board[7] == player && board[8] == player {
        return true;
    }

    // first column
    if board[0] == player && board[3] == player && board[6] == player {
        return true;
    }

    // seconds column
    if board[1] == player && board[4] == player && board[7] == player {
        return true;
    }

    // third column
    if board[2] == player && board[5] == player && board[8] == player {
        return true;
    }

    // diagonal from left
    if board[0] == player && board[4] == player && board[8] == player {
        return true;
    }

    // diagonal from right
    if board[2] == player && board[4] == player && board[6] == player {
        return true;
    }

    return false;
}

pub fn valid_move(board: [i8; 9], placement: usize) -> bool {
    if placement == 0 || placement > 9 {
        return false;
    }

    if board[placement - 1] > 0 {
        return false;
    }

    return true;
}
