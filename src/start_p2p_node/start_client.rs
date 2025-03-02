use futures_util::StreamExt;
use log::{error, info};
use tokio_tungstenite::connect_async;

mod handle_input;

use handle_input::*;

pub async fn start_client(addr: &str) {
    info!("----------------------------------------------- {addr}");

    let connection_ip = format!("ws://{}", addr);

    let (ws_stream, response) = match connect_async(connection_ip).await {
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
    let (mut sender, receiver) = ws_stream.split();

    handle_input(&mut sender).await;
}
