mod handle_input;

use std::io::{self, BufRead};
use std::sync::Arc;
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use log::{error, info};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tokio_tungstenite::tungstenite::{Error, Utf8Bytes};
use tokio_tungstenite::tungstenite::protocol::CloseFrame;
use crate::Peers;
use crate::start_p2p_node::start_client::handle_input::handle_input;

pub async fn start_client(addr: &str) {
    info!("------------------------ {addr}");

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

    // splitting sender and receiver
    let (mut sender, mut receiver) = ws_stream.split();

    // handling input
    handle_input(&mut sender).await;
}