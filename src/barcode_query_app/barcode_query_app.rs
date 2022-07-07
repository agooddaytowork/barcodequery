use crate::barcode_query::barcode_query_hashstorage_impl::BarCodeFileHashStorageImpl;
use crate::barcode_query::model::{BarCode, BarCodeQuery};
use crate::barcode_query::storage::BarCodeStorage;
use crate::barcode_reader::reader::BarcodeReader;

// TODO - logging

pub struct BarcodeQueryApp {
    pub reader: Box<dyn BarcodeReader>,
    pub existing_storage: BarCodeFileHashStorageImpl,
    pub error_storage: BarCodeFileHashStorageImpl,
}

impl BarcodeQueryApp {
    pub fn run(&mut self) {
        let mut exist = false;
        while !exist {
            let query_string = self.reader.read();
            // TODO - many clones of query_string, is it expensive?
            if query_string == "exit" {
                exist = true;
                self.error_storage.dump();
            } else {
                let bar_code = BarCode {
                    code_string: query_string.clone(),
                };
                let exists_in_storage = self.existing_storage.query(bar_code);
                println!(
                    "query existing storage: {} {}",
                    query_string.clone(),
                    exists_in_storage
                );
                if !exists_in_storage {
                    println!("insert unknown query to error storage");
                    self.error_storage.insert(query_string.clone())
                }
            }
        }
    }
}
