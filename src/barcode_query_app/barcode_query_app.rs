use crate::barcode_query::barcode_query_hashstorage_impl::BarCodeQueryHashStorageImpl;
use crate::barcode_query::model::BarCodeQuery;
use crate::barcode_query::storage::BarCodeStorage;
use crate::barcode_reader::reader::BarcodeReader;

pub trait BarcodeQueryApp {
    fn run(&mut self);
}


pub struct BarcodeQueryAppImpl {
    pub reader: Box<dyn BarcodeReader>,
    pub existing_storage: BarCodeQueryHashStorageImpl,
    pub error_storage: BarCodeQueryHashStorageImpl
}

impl BarcodeQueryApp for BarcodeQueryAppImpl {
    fn run(&mut self) {
        let mut exist = false;
        while !exist {
            let query_string = self.reader.read();
            if query_string == "exit" {
                exist = true;
                self.error_storage.dump();
            } else {
                let query_existing_storage_result = self.existing_storage.query(query_string.clone());
                println!("query existing storage: {} {}", &query_string, query_existing_storage_result);
                if !query_existing_storage_result{
                    println!("insert unknown query to error storage");
                    self.error_storage.insert(query_string.clone())
                }
            }
        }
    }
}