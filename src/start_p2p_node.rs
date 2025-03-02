use crate::{Peers, peer::Peer};
use log::debug;
use process_listener::process_listener;
use start_client::start_client;
use start_server::start_server;
use std::net::SocketAddr;

mod process_listener;
mod start_client;
mod start_server;

pub async fn start_p2p_node(addr: SocketAddr, mode: &str, peers: Peers) {
    let peers_clone: Vec<Peer> = peers.clone();

    match mode {
        "server" => {
            tokio::spawn(async move {
                start_server(addr, peers_clone).await;
            });

            debug!("Server Started");

            process_listener().await;
        }

        _ => return,
    }
}
