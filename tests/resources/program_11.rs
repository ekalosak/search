use std::env;
use std::fs::File;
use std::io::copy;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(url) = args.get(1) {
        let response = reqwest::blocking::get(url).unwrap();
        if response.status().is_success() {
            let mut file = File::create("downloaded_file.txt").unwrap();
            let content = response.bytes().unwrap();
            copy(&mut content.as_ref(), &mut file).unwrap();
            println!("File downloaded successfully!");
        } else {
            println!("Download failed with status code: {}", response.status());
        }
    } else {
        println!("Please provide a URL as a command-line argument.");
    }
}
