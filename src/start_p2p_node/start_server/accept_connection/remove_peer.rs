use uuid::Uuid;

use crate::Peers;

pub async fn remove_peer(peers: &mut Peers, uuid: Uuid) {
    peers.retain(|i| i.uuid != uuid);
}
