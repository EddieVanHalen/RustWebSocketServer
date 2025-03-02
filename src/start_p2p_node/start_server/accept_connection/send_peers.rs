use std::sync::Arc;

use futures_util::SinkExt;
use log::{debug, info};
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::{Message, Utf8Bytes};

use crate::{Peers, Sender};

pub async fn send_peers(sender: Arc<Mutex<Sender>>, peers: Peers) {
    info!("Sending Peers");

    let mut sender_lock = sender.lock().await;

    // sending all existing peers to new peer(sender)
    for i in peers.iter() {
        debug!("{}", i.receiver_address);
        let result = sender_lock
            .send(Message::Text(Utf8Bytes::from(
                i.receiver_address.to_string(),
            )))
            .await;

        debug!("{:#?}", result.unwrap());
    }
}
