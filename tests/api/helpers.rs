use std::sync::Arc;

use tokio::sync::RwLock;

use barcodequery::barcode_query::model::Message;
use barcodequery::web::server_axum::WebSocketServer;
use barcodequery::web::state::State;

pub async fn spawn_app() -> TestApp {
    let (sender, receiver) = tokio::sync::broadcast::channel::<Message>(1);
    let shared_state = Arc::new(RwLock::new(State {
        tx: sender,
        rx: receiver,
    }));
    let _ = tokio::spawn(WebSocketServer::run_until_stopped(shared_state.clone()));
    TestApp {
        address: "127.0.0.1:3000".to_string(),
    }
}

pub struct TestApp {
    pub address: String,
}
