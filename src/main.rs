use std::net::SocketAddr;

use futures_util::stream::{SplitSink, SplitStream};
use peer::Peer;
use start_p2p_node::start_p2p_node;
use tokio::net::TcpStream;
use tokio_tungstenite::{WebSocketStream, tungstenite::Message};

mod peer;
mod start_p2p_node;

pub type Peers = Vec<Peer>;
pub type Sender = SplitSink<WebSocketStream<TcpStream>, Message>;
pub type Receiver = SplitStream<WebSocketStream<TcpStream>>;

#[tokio::main]
async fn main() {
    env_logger::init();

    let addr: SocketAddr = "192.168.1.71:8080".parse().unwrap_or_else(|e| {
        panic!("Invalid address: {}. Error: {}", "localhost:8080", e);
    });

    let peers: Peers = Vec::new();

    start_p2p_node(addr, peers).await;
}
