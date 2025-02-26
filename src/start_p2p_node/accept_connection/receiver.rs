use futures_util::{SinkExt, StreamExt};
use futures_util::stream::{SplitSink, SplitStream};
use log::{error, info};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;
use crate::Peers;

pub async fn message_receiver(
    // sender_wrap: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>,
    receiver_wrap: Arc<Mutex<SplitStream<WebSocketStream<TcpStream>>>>,
    peers : Peers,
    sender_uuid : Uuid,
) {
    // receiving message
    while let Some(msg) = receiver_wrap.lock().await.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                info!("Received from client {}", text);

                if text.trim().to_lowercase() == "close" {
                    let mut peers_lock = peers.lock().await;
                    peers_lock.retain(|p| p.uuid != sender_uuid);
                    info!("Peer removed: {:?}", &sender_uuid);
                    info!("Connection removed. Total peers: {}", peers_lock.len());
                    break;
                }

                // blocking mutex for sending messages
                let mut peers_lock = peers.lock().await;

                // broadcast
                for v in peers_lock.iter_mut() {
                    if sender_uuid == v.uuid {
                        continue;
                    }

                    let message = text.clone();

                    println!("message = {}", &message);

                    //sending messages
                    if let Err(e) = v.sender.lock().await.send(Message::Text(message)).await {
                        eprintln!("Error sending message: {}", e);
                    }
                }
            }
            Ok(Message::Close(_)) => {
                info!("Client closed the connection");
                {
                    let mut peers_lock = peers.lock().await;
                    let _ = peers_lock.retain(|p| p.uuid != sender_uuid);
                }
                break;
            }
            Err(e) => {
                error!("Connection error: {}", e);
                {
                    let mut peers_lock = peers.lock().await;
                    let _ = peers_lock.retain(|p| p.uuid != sender_uuid);
                }
                break;
            }
            _ => {}
        }
    }
}
