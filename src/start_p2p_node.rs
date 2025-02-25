use std::{
    net::SocketAddr,
    sync::Arc,
};
use tokio::sync::Notify;

use crate::Peers;

mod accept_connection;
mod start_server;
mod start_client;

use start_server::*;
use start_client::*;
use accept_connection::*;

pub async fn start(addr: SocketAddr, peers: Peers) {
    let notify = Arc::new(Notify::new());

    let notify_clone = Arc::clone(&notify);

    let peers_clone = Arc::clone(&peers);

    tokio::spawn(async move {
        start_server(addr, peers_clone, notify_clone).await;
    });

    notify.notified().await;

    println!("Server Started Working har");

    start_client("ws://127.0.0.1:8080", peers.clone()).await;
}
