use futures_util::stream::SplitSink;
use futures_util::{SinkExt, StreamExt};
use log::{debug, error, info};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::{Message, Utf8Bytes};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};
use handle_input::*;

mod handle_input;


async fn send_me_peers(
    sender: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
) {
    let result = sender.send(Message::Text(Utf8Bytes::from("ips"))).await;

    debug!("{:#?}", result);
}

pub async fn start_client(addr: String) {
    info!("connecting to {addr}");

    let connection_ip = format!("ws://{}", addr);

    let (ws_stream, _response) = match connect_async(connection_ip).await {
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
    let (mut sender, _receiver) = ws_stream.split();

    send_me_peers(&mut sender).await;

    handle_input(&mut sender).await;
}
