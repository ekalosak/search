use std::env;

fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(num) = args.get(1) {
        if let Ok(n) = num.parse::<u32>() {
            println!("Factorial of {}: {}", n, factorial(n));
        } else {
            println!("Invalid number!");
        }
    } else {
        println!("Please provide a number as a command-line argument.");
    }
}
