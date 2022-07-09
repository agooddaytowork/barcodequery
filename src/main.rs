use std::fmt::{Debug, Display};

use tokio::sync::oneshot;
use tokio::task::JoinError;

use barcodequery::barcode_query::model::Message;
use barcodequery::barcode_query_app::app_async::BarcodeAppAsync;
use barcodequery::web::server_axum::WebSocketServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // TODO add barcode app: receives barcode and query part
    let (sender, receiver) = oneshot::channel::<Message>();
    let server_task = tokio::spawn(WebSocketServer::run_until_stopped(receiver));
    let app_task = tokio::spawn(BarcodeAppAsync::run_until_stopped(sender));

    tokio::select! {
        outcome = server_task => report_exit("Websocket server", outcome),
        outcome = app_task => report_exit("App task", outcome),
    }

    Ok(())
}

fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} failed",
                task_name
            )
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{}' task failed to complete",
                task_name
            )
        }
    }
}
