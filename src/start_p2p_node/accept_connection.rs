use std::sync::Arc;

use futures_util::{SinkExt, StreamExt};
use log::{error, info};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use uuid::Uuid;

use crate::{peer::Peer, Peers};

pub async fn accept_connection(stream: TcpStream, peers: Peers) {
    info!("Handling Connection In accept_connection");

    let ws_stream = match accept_async(stream).await {
        Ok(ws) => {
            info!("WebSocket handshake successful");
            ws
        }
        Err(e) => {
            error!("WebSocket handshake failed: {}", e);
            return;
        }
    };

    let ip_address = ws_stream.get_ref().peer_addr().unwrap().ip().to_string();
    let (sender, receiver) = ws_stream.split();

    // getting senders uuid
    let sender_uuid: Uuid = Uuid::new_v4();
    let sender_wrap = Arc::new(Mutex::new(sender));
    let receiver_wrap = Arc::new(Mutex::new(receiver));

    // creating new peer
    let new_peer = Arc::new(Peer {
        uuid: sender_uuid,
        sender: sender_wrap.clone(),
        receiver: receiver_wrap.clone(),
        receiver_address: ip_address,
    });

    // adding peer to peers list
    {
        let mut peers_lock = peers.lock().await;
        peers_lock.push(new_peer.clone());
        info!("New connection added. Total peers: {}", peers_lock.len());
    }

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
                    if let Err(e) = v.sender.lock().await.send(Message::Text(message)).await
                    {
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