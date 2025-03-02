use std::net::SocketAddr;

use accept_connection::accept_connection;
use log::info;
use tokio::net::TcpListener;

use crate::Peers;

mod accept_connection;

pub async fn start_server(addr: SocketAddr, peers: Peers) {
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    info!("Listening on {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        info!("New Connection Accepted");
        let mut peers_clone = peers.clone();
        tokio::spawn(async move {
            accept_connection(stream, &mut peers_clone).await;
        });
    }
}
