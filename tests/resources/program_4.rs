use std::env;

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(num) = args.get(1) {
        if let Ok(n) = num.parse::<u32>() {
            println!("{} is prime: {}", n, is_prime(n));
        } else {
            println!("Invalid number!");
        }
    } else {
        println!("Please provide a number as a command-line argument.");
    }
}
