use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(name) = args.get(1) {
        println!("Hello, {}!", name);
    } else {
        println!("Hello, World!");
    }
}
