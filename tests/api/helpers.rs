use barcodequery::web::server_axum::WebSocketServer;

pub async fn spawn_app() -> TestApp {
    let _ = tokio::spawn(WebSocketServer::run_until_stopped());
    TestApp {
        address: "127.0.0.1:3000".to_string(),
    }
}

pub struct TestApp {
    pub address: String,
}
