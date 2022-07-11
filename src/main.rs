use std::fmt::{Debug, Display};
use std::fs::File;

use tokio::task::JoinError;

use barcodequery::barcode_query::barcode_query_hashstorage_impl::BarCodeFileHashStorageImpl;
use barcodequery::barcode_query::storage::BarCodeStorage;
use barcodequery::barcode_query_app::app_async::BarcodeQueryAppAsync;
use barcodequery::barcode_reader::console_reader::ConsoleBarcodeReader;
use barcodequery::web::server_axum::WebSocketServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // TODO add barcode app: receives barcode and query part
    let server_task = tokio::spawn(WebSocketServer::run_until_stopped());

    tokio::select! {
        outcome = server_task => report_exit("Websocket server", outcome),
    }

    Ok(())
}

fn create_barcode_app() -> BarcodeQueryAppAsync {
    let bar_code_file = File::open("test.txt").unwrap();
    let error_file = File::create("error.txt").unwrap();

    let mut existing_storage = BarCodeFileHashStorageImpl::new(bar_code_file);
    let error_storage = BarCodeFileHashStorageImpl::new(error_file);
    existing_storage.load();

    let mut query_app = BarcodeQueryAppAsync {
        reader: Box::new(ConsoleBarcodeReader {}),
        existing_storage,
        error_storage,
    };
    query_app
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
