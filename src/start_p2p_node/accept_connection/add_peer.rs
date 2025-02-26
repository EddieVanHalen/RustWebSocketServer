use std::sync::Arc;
use log::info;
use crate::peer::Peer;
use crate::Peers;

pub async fn add_peer(peer : Arc<Peer>, peers: Peers){
    let mut peers_lock = peers.lock().await;
    peers_lock.push(peer.clone());
    info!("New connection added. Total peers: {}", peers_lock.len());
}