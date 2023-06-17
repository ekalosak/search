use std::env;
use ping::ping;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(hostname) = args.get(1) {
        if let Ok(result) = ping(hostname, 5) {
            if result.packet_loss == 0 {
                println!("Ping statistics for {}: Avg RTT = {} ms", hostname, result.average_time);
            } else {
                println!("Ping failed: Packet loss = {}%", result.packet_loss);
            }
        } else {
            println!("Failed to ping {}.", hostname);
        }
    } else {
        println!("Please provide a hostname as a command-line argument.");
    }
}
