use futures_util::SinkExt;
use futures_util::stream::SplitSink;
use log::{error, info};
use std::io;
use std::io::BufRead;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::error::ProtocolError;
use tokio_tungstenite::tungstenite::{Error, Message, Utf8Bytes};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

//handling user input
pub async fn handle_input(
    sender: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
) {
    info!("Handling Input");

    let stdin = io::stdin();
    let mut handle = stdin.lock();

    loop {
        let mut input = String::new();
        match handle.read_line(&mut input) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let message = input.trim();

                info!("{}", message);

                if message == "close" {
                    let _ = sender.close().await;
                }

                let result = sender.send(Message::Text(Utf8Bytes::from(message))).await;
                
                //handling errors for case when i catch them to return in process_listener()
                if let Err(e) = result {
                    match e {
                        Error::Protocol(ProtocolError::SendAfterClosing) => {
                            println!("Tried to send a message after the WebSocket was closed.");
                            return;
                        }
                        Error::AlreadyClosed => {
                            println!("WebSocket connection is already closed.");
                            return;
                        }
                        Error::ConnectionClosed => {
                            println!("WebSocket connection was closed.");
                            return;
                        }
                        _ => {
                            println!("Other error occurred: {:?}", e);
                            return;
                        }
                    }
                }
                // debug!("{:#?}", result.unwrap());
            }
            Err(e) => {
                error!("Failed to read line: {}", e);
                break;
            }
        }
    }
}
