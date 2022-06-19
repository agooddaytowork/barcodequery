extern crate barcodequery;

use barcodequery::barcode_query::barcode_query_hashstorage_impl::BarCodeQueryHashStorageImpl;
use barcodequery::barcode_query::model::{BarCodeQuery, BarCodeQueryFactory};
use barcodequery::barcode_query::storage::BarCodeStorage;
use barcodequery::barcode_query::storage::BarcodeStorageEnum::ExistingStorage;
use barcodequery::barcode_reader::console_reader::ConsoleBarcodeReader;
use barcodequery::barcode_reader::reader::BarcodeReader;

fn main() {
    let mut barcode_query: BarCodeQueryHashStorageImpl = BarCodeQueryHashStorageImpl::new();
    barcode_query.load("../../../test.txt".to_string(), ExistingStorage);
    println!("query: 123456785897 {}", barcode_query.query("123456785897".to_string()));
    println!("query: 111 {}", barcode_query.query("111".to_string()));
    barcode_query.dump("1234.txt".to_string(), ExistingStorage);

    let query = ConsoleBarcodeReader::read();
    println!("query: {} {}",query, barcode_query.query(query.to_string()));
}
