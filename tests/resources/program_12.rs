use std::io;
use rand::Rng;

fn get_user_choice() -> String {
    loop {
        println!("Choose one: (R)ock, (P)aper, (S)cissors");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read input");
        let user_choice = user_input.trim().to_uppercase();
        if user_choice == "R" || user_choice == "P" || user_choice == "S" {
            return user_choice;
        } else {
            println!("Invalid choice. Please try again.");
        }
    }
}

fn get_computer_choice() -> String {
    let choices = ["R", "P", "S"];
    let random_index = rand::thread_rng().gen_range(0..3);
    choices[random_index].to_string()
}

fn determine_winner(user_choice: &str, computer_choice: &str) -> String {
    if user_choice == computer_choice {
        "It's a tie!".to_string()
    } else if (user_choice == "R" && computer_choice == "S")
        || (user_choice == "P" && computer_choice == "R")
        || (user_choice == "S" && computer_choice == "P")
    {
        "You win!".to_string()
    } else {
        "Computer wins!".to_string()
    }
}

fn main() {
    println!("Welcome to Rock, Paper, Scissors!");

    loop {
        let user_choice = get_user_choice();
        let computer_choice = get_computer_choice();

        println!("You chose: {}", user_choice);
        println!("Computer chose: {}", computer_choice);

        let result = determine_winner(&user_choice, &computer_choice);
        println!("{}", result);

        println!("Do you want to play again? (Y/N)");
        let mut play_again = String::new();
        io::stdin().read_line(&mut play_again).expect("Failed to read input");
        if play_again.trim().to_uppercase() != "Y" {
            break;
        }
    }

    println!("Thanks for playing!");
}
