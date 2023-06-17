use std::env;

fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(input) = args.get(1) {
        println!("Reversed: {}", reverse_string(input));
    } else {
        println!("Please provide a string as a command-line argument.");
    }
}
