use std::sync::Arc;

use tokio::sync::Mutex;
use uuid::Uuid;

use crate::Peers;

//just removing peer by uuid
pub async fn remove_peer(peers: Arc<Mutex<Peers>>, uuid: Uuid) {
    peers.lock().await.retain(|i| i.uuid != uuid);
}
