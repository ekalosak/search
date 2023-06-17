use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(url) = args.get(1) {
        match reqwest::blocking::get(url) {
            Ok(response) => {
                if response.status().is_success() {
                    let body = response.text().unwrap();
                    println!("{}", body);
                } else {
                    println!("Request failed with status code: {}", response.status());
                }
            }
            Err(err) => {
                println!("Failed to perform HTTP GET request: {}", err);
            }
        }
    } else {
        println!("Please provide a URL as a command-line argument.");
    }
}
