use std::sync::Arc;

use futures_util::{SinkExt, StreamExt};
use log::{error, info};
use tokio::sync::{Mutex, Notify};
use tokio_tungstenite::tungstenite::{Message, Utf8Bytes};
use uuid::Uuid;

use crate::{
    Peers,
    peer::Peer,
    start_p2p_node::start_server::accept_connection::{remove_peer, send_peers::send_peers},
};

async fn to_vector(peers: Arc<Mutex<Peers>>) -> Vec<Peer> {
    let mut result: Vec<Peer> = Vec::new();

    let peers_lock = peers.lock().await;

    for i in peers_lock.iter() {
        result.push(i.clone());
    }

    result
}

pub async fn message_receiver(
    peer: Arc<Peer>,
    peers: Arc<Mutex<Peers>>,
    sender_uuid: Uuid,
    notify: Arc<Notify>,
) {
    notify.notify_one();
    while let Some(msg) = peer.receiver.lock().await.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                info!("Received from client {}: {}", sender_uuid, text);

                if text.trim().eq_ignore_ascii_case("close") {
                    remove_peer(peers, sender_uuid).await;
                    break;
                }

                if text.trim().eq_ignore_ascii_case("ips") {
                    send_peers(peer.sender.clone(), to_vector(peers.clone()).await).await;
                }

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
