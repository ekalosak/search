use std::io;

fn draw_board(board: &[&str]) {
    println!("---------");
    for row in board.chunks(3) {
        println!("| {} | {} | {} |", row[0], row[1], row[2]);
        println!("---------");
    }
}

fn is_board_full(board: &[&str]) -> bool {
    board.iter().all(|&cell| cell != "-")
}

fn check_winner(board: &[&str]) -> Option<&str> {
    let winning_combinations = [
        (0, 1, 2),
        (3, 4, 5),
        (6, 7, 8),
        (0, 3, 6),
        (1, 4, 7),
        (2, 5, 8),
        (0, 4, 8),
        (2, 4, 6),
    ];

    for &(a, b, c) in &winning_combinations {
        if board[a] != "-" && board[a] == board[b] && board[a] == board[c] {
            return Some(board[a]);
        }
    }

    None
}

fn main() {
    let mut board = ["-", "-", "-", "-", "-", "-", "-", "-", "-"];

    println!("Welcome to Tic-Tac-Toe!");

    let mut current_player = "X";

    loop {
        draw_board(&board);

        println!("Player {}, enter your move (0-8):", current_player);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let position: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        if position >= 9 {
            println!("Invalid position. Please enter a number between 0 and 8.");
            continue;
        }

        if board[position] != "-" {
            println!("Position already occupied. Please choose another position.");
            continue;
        }

        board[position] = current_player;

        if let Some(winner) = check_winner(&board) {
            draw_board(&board);
            println!("Player {} wins!", winner);
            break;
        }

        if is_board_full(&board) {
            draw_board(&board);
            println!("It's a tie!");
            break;
        }

        current_player = if current_player == "X" { "O" } else { "X" };
    }

    println!("Thanks for playing!");
}
