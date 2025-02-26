mod receiver;
mod add_peer;

use std::sync::Arc;

use futures_util::{SinkExt, StreamExt};
use log::{error, info};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use uuid::Uuid;
use add_peer::*;

use crate::{peer::Peer, Peers, Sender};
use crate::start_p2p_node::accept_connection::receiver::message_receiver;

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
    let sender_wrap  = Arc::new(Mutex::new(sender));
    let receiver_wrap = Arc::new(Mutex::new(receiver));

    // creating new peer
    let new_peer = Arc::new(Peer {
        uuid: sender_uuid,
        sender: sender_wrap.clone(),
        receiver: receiver_wrap.clone(),
        receiver_address: ip_address,
    });

    // adding peer to peers list
    add_peer(new_peer, peers.clone()).await;

    // receiving message
    message_receiver(receiver_wrap, peers, sender_uuid).await;
}