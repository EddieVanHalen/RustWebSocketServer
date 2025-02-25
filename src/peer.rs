use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use crate::{Sender, Receiver};

pub struct Peer {
    pub uuid: Uuid,
    pub sender: Arc<Mutex<Sender>>,
    pub receiver: Arc<Mutex<Receiver>>,
    pub receiver_address: String,
}
