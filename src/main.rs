use std::{net::SocketAddr, sync::Arc};

use futures_util::stream::{SplitSink, SplitStream};
use peer::Peer;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

mod peer;
mod start_p2p_node;

pub type Sender = SplitSink<WebSocketStream<TcpStream>, Message>;
pub type Receiver = SplitStream<WebSocketStream<TcpStream>>;
pub type Peers = Arc<Mutex<Vec<Arc<Peer>>>>;

#[tokio::main]
async fn main() {
    env_logger::init();

    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap_or_else(|e| {
        panic!("Invalid address: {}. Error: {}", "127.0.0.1:8080", e);
    });

    let peers: Arc<Mutex<Vec<Arc<Peer>>>> = Arc::new(Mutex::new(Vec::new()));

    start_p2p_node::start(addr, peers.clone()).await;
}   
