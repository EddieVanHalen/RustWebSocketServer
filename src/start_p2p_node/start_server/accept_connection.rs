use futures_util::StreamExt;
use std::sync::Arc;
use tokio_tungstenite::accept_async;
use uuid::Uuid;
use log::{error, info};
use crate::{Peers, Sender, peer::Peer};
use tokio::{
    net::TcpStream,
    sync::Mutex,
};

mod message_receiver;
mod remove_peer;
mod send_peers;

use message_receiver::message_receiver;
use remove_peer::remove_peer;

pub async fn accept_connection(stream: TcpStream, peers: &mut Peers) {
    info!("Handling Connection In accept_connection");

    //getting WebSocket stream
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

    // splitting one stream into two streams
    let (sender, receiver) = ws_stream.split();

    //creating user and user's data
    let sender_uuid = Uuid::new_v4();
    let sender_wrap: Arc<Mutex<Sender>> = Arc::new(Mutex::new(sender));
    let receiver_wrap = Arc::new(Mutex::new(receiver));

    let new_peer = Peer {
        uuid: sender_uuid,
        sender: sender_wrap.clone(),
        receiver: receiver_wrap.clone(),
        receiver_address: ip_address,
    };

    //adding new peer to peers list
    peers.push(new_peer.clone());

    // making clones for sending in another thread
    let new_peer_wrap_clone = Arc::new(new_peer.clone()).clone();
    let sender_uuid_clone = sender_uuid.clone();
    let peers_clone_arc = Arc::new(Mutex::new(peers.clone()));

    // starting message receiver
    tokio::spawn(async move {
        message_receiver(
            new_peer_wrap_clone,
            peers_clone_arc,
            sender_uuid_clone,
        )
        .await;
    });
}
