use futures_util::{SinkExt, StreamExt as _};
use log::{error, info};
use std::{io::{self, BufRead}, net::SocketAddr, sync::Arc};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{Mutex, Notify},
};
use tokio_tungstenite::{accept_async, connect_async, tungstenite::Message};
use uuid::Uuid;

use crate::{Peers, peer::Peer};

async fn accept_connection(stream: TcpStream, peers: Peers) {
    println!("handling connection");

    let ws_stream = match accept_async(stream).await {
        Ok(ws) => {
            info!("WebSocket handshake successful");
            ws
        }
        Err(e) => {
            error!("WebSocket handshake failed: {}", e);
            return;
        }
    };

    let ip_address = ws_stream.get_ref().peer_addr().unwrap().ip().to_string();
    let (mut sender, mut receiver) = ws_stream.split();

    // getting senders uuid
    let sender_uuid: Uuid = Uuid::new_v4();
    let sender_wrap = Arc::new(Mutex::new(sender));
    let receiver_wrap = Arc::new(Mutex::new(receiver));

    // creating new peer
    let new_peer = Arc::new(Peer {
        uuid: sender_uuid,
        sender: sender_wrap.clone(),
        receiver: receiver_wrap.clone(),
        receiver_address: ip_address,
    });

    {
        let mut peers_lock = peers.lock().await;
        peers_lock.push(new_peer.clone());
        info!("New connection added. Total peers: {}", peers_lock.len());
    }

    while let Some(msg) = receiver_wrap.lock().await.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                info!("Received from client: {}", text);

                // blocking mutex for sending messages
                let mut peers_lock = peers.lock().await;

                for v in peers_lock.iter_mut() {
                    if sender_uuid == v.uuid {
                        continue;
                    }

                    if let Err(e) = sender_wrap
                        .lock()
                        .await
                        .send(Message::Text(text.clone()))
                        .await
                    {
                        eprintln!("Error sending message: {}", e);
                    }
                }
            }
            Ok(Message::Close(_)) => {
                info!("Client closed the connection");
                {
                    let mut peers_lock = peers.lock().await;
                    let _ = peers_lock.retain(|p| p.uuid != sender_uuid);
                }
                break;
            }
            Err(e) => {
                error!("Connection error: {}", e);
                {
                    let mut peers_lock = peers.lock().await;
                    let _ = peers_lock.retain(|p| p.uuid != sender_uuid);
                }
                break;
            }
            _ => {}
        }
    }
}

async fn start_server(addr: SocketAddr, peers: Peers, notify: Arc<Notify>) {
    let listener = TcpListener::bind(&addr).await.expect("failder to bind");

    info!("Listening on: {}", addr);

    notify.notify_one();

    while let Ok((stream, _)) = listener.accept().await {
        info!("New connection accepted");
        tokio::spawn(accept_connection(stream, peers.clone()));
    }
}

async fn start_client(addr: &str, peers: Peers) {
    println!("------------------------ {addr}");

    let (ws_stream, _response) = match connect_async(addr).await {
        Ok((ws, response)) => {
            info!("WebSocket connected to {}", addr);
            info!("Response status: {}", response.status());
            (ws, response)
        }

        Err(e) => {
            error!("Failed to connect: {}", e);
            return;
        }
    };

    // let peers_lock = peers.lock().await;

    // for i in peers_lock.iter() {
    //     println!("{}", i.uuid);
    // }

    let stdin = io::stdin();
    let mut handle = stdin.lock();

    loop {
        let mut input = String::new();
        match handle.read_line(&mut input) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let message = input.trim();
                println!("{}", message);
            }
            Err(e) => {
                error!("Failed to read line: {}", e);
                break;
            }
        }
    }
}

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
