use std::io::{self, BufRead};

use futures_util::{SinkExt, StreamExt};
use log::{error, info};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::Peers;

pub async fn start_client(addr: &str, peers: Peers) {
    info!("------------------------ {addr}");

    let (ws_stream, _response) = match connect_async(addr).await {
        Ok((ws, response)) => {
            info!("WebSocket connected to {}", addr);
            info!("Response status: {}", response.status());
            (ws, response)
        }

        Err(e) => {
            error!("Failed to connect: {}", e);
            return;
        }
    };

    // let peers_lock = peers.lock().await;

    // for i in peers_lock.iter() {
    //     println!("{}", i.uuid);
    // }

    let (mut sender, mut receiver) = ws_stream.split();

    let stdin = io::stdin();
    let mut handle = stdin.lock();

    loop {
        let mut input = String::new();
        match handle.read_line(&mut input) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let message = input.trim();
                let _ = sender.send(Message::Text(message.to_string())).await;
            }
            Err(e) => {
                error!("Failed to read line: {}", e);
                break;
            }
        }
    }
}