use std::{env, net::SocketAddr};
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

    let variables: Vec<String> = env::args().collect();

    let mode = &variables[1];
    let host_ip = &variables[2];

    let addr: SocketAddr = host_ip.parse().unwrap_or_else(|e| {
        panic!("Invalid address: {}. Error: {}", "localhost:8080", e);
    });

    let peers: Peers = Vec::new();

    start_p2p_node(addr, mode, peers).await;
}
