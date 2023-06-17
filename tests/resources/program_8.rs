use std::env;
use std::net::ToSocketAddrs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(hostname) = args.get(1) {
        if let Ok(addresses) = hostname.to_socket_addrs() {
            for addr in addresses {
                println!("{}:{}", hostname, addr);
            }
        } else {
            println!("Failed to perform DNS lookup for {}.", hostname);
        }
    } else {
        println!("Please provide a hostname as a command-line argument.");
    }
}
