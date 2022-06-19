use crate::barcode_query::model::BarCodeQuery;
use crate::barcode_reader::reader::BarcodeReader;

pub trait BarcodeQueryApp {
    fn run(&self);
}


pub struct BarcodeQueryAppImpl {
    pub reader: Box<dyn BarcodeReader>,
    pub query: Box<dyn BarCodeQuery>,
}

impl BarcodeQueryApp for BarcodeQueryAppImpl {
    fn run(&self) {
        let mut exist = false;
        while !exist {
            let query = self.reader.read();
            if query == "exit" {
                exist = true
            } else {
                println!("query: {} {}", query, self.query.query(query.to_string()));
            }
        }
    }
}