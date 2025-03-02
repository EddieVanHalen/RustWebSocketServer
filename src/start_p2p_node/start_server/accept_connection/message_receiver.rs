use crate::{
    Peers,
    peer::Peer,
    start_p2p_node::start_server::accept_connection::{remove_peer, send_peers::send_peers},
};
use futures_util::{SinkExt, StreamExt};
use log::{error, info};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::{Message, Utf8Bytes};
use uuid::Uuid;

// just mini func for deep copy of Vector
async fn to_vector(peers: Arc<Mutex<Peers>>) -> Vec<Peer> {
    let mut result: Vec<Peer> = Vec::new();

    let peers_lock = peers.lock().await;

    for i in peers_lock.iter() {
        result.push(i.clone());
    }

    result
}

pub async fn message_receiver(peer: Arc<Peer>, peers: Arc<Mutex<Peers>>, sender_uuid: Uuid) {
    // waiting for message then handle this message
    while let Some(msg) = peer.receiver.lock().await.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                info!("Received from client {}: {}", sender_uuid, text);

                //removing peer if he sent "close" word
                if text.trim().eq_ignore_ascii_case("close") {
                    remove_peer(peers, sender_uuid).await;
                    break;
                }

                // sending all peers to client if he sent "ips" word
                if text.trim().eq_ignore_ascii_case("ips") {
                    send_peers(peer.sender.clone(), to_vector(peers.clone()).await).await;
                }

                // broadcasting message to connected peers
                for i in peers.lock().await.iter() {
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
            _ => {
                let message = msg.unwrap();
                info!("{}", &message);
            }
        }
    }
}
