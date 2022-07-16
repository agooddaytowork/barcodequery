use std::sync::Arc;
use tokio::sync::broadcast::Receiver;
use tokio::sync::broadcast::Sender;
use tokio::sync::RwLock;

use crate::barcode_query::model;

pub struct State {
    pub tx: Sender<model::Message>,
    pub rx: Receiver<model::Message>
}

pub type SharedState = Arc<RwLock<State>>;
