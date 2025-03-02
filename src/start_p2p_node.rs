use std::net::SocketAddr;
use std::sync::Arc;

use crate::Peers;

mod start_client;
mod start_server;

use log::debug;
use start_client::start_client;
use start_server::start_server;

use tokio::sync::Notify;

pub async fn start_p2p_node(addr: SocketAddr, peers: Peers) {
    let notify = Arc::new(Notify::new());

    let notify_clone = notify.clone();

    let peers_clone = peers.clone();

    tokio::spawn(async move {
        start_server(addr, peers_clone, notify_clone).await;
    });

    notify.notified().await;

    debug!("Server Started");

    start_client(&addr.to_string()).await;
}
