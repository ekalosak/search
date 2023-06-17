use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(sentence) = args.get(1) {
        let words: Vec<&str
