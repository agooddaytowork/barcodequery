use barcodequery::barcode_query::model::Message;
use barcodequery::web::server_axum::WebSocketServer;
use tokio::sync::oneshot;

pub async fn spawn_app() -> TestApp {
    let (sender, receiver) = oneshot::channel::<Message>();
    let _ = tokio::spawn(WebSocketServer::run_until_stopped(receiver));
    TestApp {
        address: "127.0.0.1:3000".to_string(),
    }
}

pub struct TestApp {
    pub address: String,
}
