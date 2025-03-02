use crate::{Receiver, Sender};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Clone)]
pub struct Peer {
    pub uuid: Uuid,
    pub sender: Arc<Mutex<Sender>>,
    pub receiver: Arc<Mutex<Receiver>>,
    pub receiver_address: String,
}
