use std::{net::SocketAddr, sync::Arc};

use log::info;
use tokio::{net::TcpListener, sync::Notify};

use crate::{start_p2p_node::accept_connection, Peers};

pub async fn start_server(addr: SocketAddr, peers: Peers, notify: Arc<Notify>) {
    let listener = TcpListener::bind(&addr).await.expect("failed to bind");

    info!("Listening on: {}", addr);

    notify.notify_one();

    while let Ok((stream, _)) = listener.accept().await {
        info!("New connection accepted");
        tokio::spawn(accept_connection(stream, peers.clone()));
    }
}