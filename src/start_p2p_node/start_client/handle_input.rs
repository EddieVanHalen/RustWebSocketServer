use std::io;
use std::io::BufRead;
use futures_util::SinkExt;
use futures_util::stream::{SplitSink, SplitStream};
use log::error;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::{Message, Utf8Bytes};

//handling user input
pub async fn handle_input(sender: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>){
    // taking the io stream
    let stdin = io::stdin();
    let mut handle = stdin.lock();

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