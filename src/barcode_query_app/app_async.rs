use crate::barcode_query::model;
use crate::web::state::SharedState;
use std::ops::Deref;
use std::time::Duration;

pub struct BarcodeAppAsync;

impl BarcodeAppAsync {
    pub async fn run_until_stopped(shared_state: SharedState) -> anyhow::Result<()> {
        let shared_state = shared_state.deref().read().await;
        let sender = shared_state.tx.clone();
        loop {
            sender.send(model::Message {
                message_type: 0,
                message_str: "sent from app".to_string(),
            }).expect("unable to sent to channel");
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    }
}
