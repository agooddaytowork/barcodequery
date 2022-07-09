use tokio::sync::oneshot::Sender;
use crate::barcode_query::model::Message;

pub struct BarcodeAppAsync;

impl BarcodeAppAsync {
    pub async fn run_until_stopped(sender: Sender<Message>) -> anyhow::Result<()> {
        anyhow::Ok(())
    }
}