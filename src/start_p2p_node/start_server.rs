use std::{net::SocketAddr, sync::Arc};

use accept_connection::accept_connection;
use log::info;
use tokio::net::TcpListener;
use tokio::sync::Notify;

use crate::Peers;

mod accept_connection;

pub async fn start_server(addr: SocketAddr, peers: Peers, notify: Arc<Notify>) {
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    info!("Listening on {}", addr);

    notify.notify_one();

    while let Ok((stream, _)) = listener.accept().await {
        info!("New Connection Accepted");
        let mut peers_clone = peers.clone();
        tokio::spawn(async move {
            accept_connection(stream, &mut peers_clone).await;
        });
    }
}
