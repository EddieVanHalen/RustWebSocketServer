use std::sync::Arc;

use futures_util::{SinkExt, StreamExt};
use log::{error, info};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Message, Utf8Bytes},
};
use uuid::Uuid;

use crate::{Peers, peer::Peer};

mod remove_peer;

use remove_peer::remove_peer;

pub async fn accept_connection(stream: TcpStream, peers: &mut Peers) {
    info!("Handling Connection In accept_connection");

    let ws_stream = match accept_async(stream).await {
        Ok(ws) => {
            info!("WebSocket handshake successful");
            ws
        }

        Err(e) => {
            error!("WebSocket handshake failed {}", e);
            return;
        }
    };

    let ip_address = ws_stream.get_ref().peer_addr().unwrap().ip().to_string();
    let (sender, receiver) = ws_stream.split();

    let sender_uuid = Uuid::new_v4();
    let sender_wrap = Arc::new(Mutex::new(sender));
    let receiver_wrap = Arc::new(Mutex::new(receiver));

    let new_peer = Peer {
        uuid: sender_uuid,
        sender: sender_wrap.clone(),
        receiver: receiver_wrap.clone(),
        receiver_address: ip_address,
    };

    //adding new peer to peers list
    peers.push(new_peer);

    while let Some(msg) = receiver_wrap.lock().await.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                info!("Received from client {}: {}", sender_uuid, text);

                if text.trim().eq_ignore_ascii_case("close") {
                    remove_peer(peers, sender_uuid).await;
                    break;
                }

                for i in peers.iter() {
                    if i.uuid == sender_uuid {
                        continue;
                    }

                    let message = &text.to_string();

                    let _ = i
                        .sender
                        .lock()
                        .await
                        .send(Message::Text(Utf8Bytes::from(message)))
                        .await;
                }
            }
            Ok(Message::Close(_)) => {
                info!("Client {} closed the connection", sender_uuid);
                remove_peer(peers, sender_uuid).await;
                break;
            }
            Err(e) => {
                error!("Connection error for {}: {}", sender_uuid, e);
                remove_peer(peers, sender_uuid).await;
                break;
            }
            _ => {}
        }
    }
}
