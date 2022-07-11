use std::fmt::{Debug, Display};
use std::sync::Arc;

use tokio::sync::{broadcast, RwLock};
use tokio::task::JoinError;

use barcodequery::barcode_query::model::Message;
use barcodequery::barcode_query_app::app_async::BarcodeAppAsync;
use barcodequery::web::server_axum::WebSocketServer;
use barcodequery::web::state::State;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (sender1, receiver) = broadcast::channel::<Message>(20);
    let shared_state = Arc::new(RwLock::new(State {
        tx: sender1,
        rx: receiver,
    }));
    // TODO - can we use borrowing in 2 usages below by passing &shared_state instead of clone()?
    let server_task = tokio::spawn(WebSocketServer::run_until_stopped(shared_state.clone()));
    let app_task = tokio::spawn(BarcodeAppAsync::run_until_stopped(shared_state.clone()));

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
