extern crate barcodequery;

use barcodequery::barcode_query::barcode_query_hashstorage_impl::BarCodeQueryHashStorageImpl;
use barcodequery::barcode_query::model::BarCodeQueryFactory;
use barcodequery::barcode_query::storage::BarCodeStorage;
use barcodequery::barcode_query::storage::BarcodeStorageEnum::ExistingStorage;
use barcodequery::barcode_query_app::barcode_query_app::{BarcodeQueryApp, BarcodeQueryAppImpl};
use barcodequery::barcode_reader::console_reader::ConsoleBarcodeReader;

fn main() {
    let mut barcode_query: BarCodeQueryHashStorageImpl = BarCodeQueryHashStorageImpl::new();
    barcode_query.load("test.txt".to_string(), ExistingStorage);
    let query_app = BarcodeQueryAppImpl {
        reader: Box::new( ConsoleBarcodeReader {}),
        query: Box::new(barcode_query)
    };
    query_app.run();
}
