use std::io::{self, BufRead};
use std::sync::Arc;
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use log::{error, info};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tokio_tungstenite::tungstenite::{Error, Utf8Bytes};
use tokio_tungstenite::tungstenite::protocol::CloseFrame;
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

    // splitting sender and receiver
    let (mut sender, mut receiver) = ws_stream.split();

    // taking the io stream
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    // handling input
    loop {
        let mut input = String::new();
        match handle.read_line(&mut input) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let message = input.trim();

                if message == "close" {
                    let _ = sender.close().await;
                }

                let _ = sender.send(Message::Text(Utf8Bytes::from(message))).await;
            }
            Err(e) => {
                error!("Failed to read line: {}", e);
                break;
            }
        }
    }
}