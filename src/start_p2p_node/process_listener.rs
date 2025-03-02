use log::{error, info};
use regex::Regex;
use std::io::{self};
use crate::start_p2p_node::start_client;

pub async fn process_listener() {
    let stdin = io::stdin();
    let re = Regex::new(r"^connect\s+(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}):(\d{1,5})$").unwrap();

    loop {
        let mut input = String::new();

        match stdin.read_line(&mut input) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let message = input.trim();

                if message.eq_ignore_ascii_case("exit") {
                    info!("Exiting process_listener...");
                    break;
                }

                if let Some(caps) = re.captures(message) {
                    let ip = &caps[1];
                    let port = &caps[2];
                    info!("Connecting to {}:{}", ip, port);
                    start_client(format!("{}:{}", ip, port)).await;
                    // break;
                }
            }
            Err(e) => {
                error!("Failed to read line: {}", e);
                break;
            }
        }
    }
}
