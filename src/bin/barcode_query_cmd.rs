extern crate barcodequery;

use barcodequery::barcode_query::barcode_query_hashstorage_impl::BarCodeQueryHashStorageImpl;
use barcodequery::barcode_query::model::BarCodeQueryFactory;
use barcodequery::barcode_query::storage::BarCodeStorage;
use barcodequery::barcode_query::storage::BarcodeStorageEnum::ExistingStorage;
use barcodequery::barcode_query_app::barcode_query_app::{BarcodeQueryApp, BarcodeQueryAppImpl};
use barcodequery::barcode_reader::console_reader::ConsoleBarcodeReader;

fn main() {
    let mut existing_storage: BarCodeQueryHashStorageImpl = BarCodeQueryHashStorageImpl::new("test.txt".to_string());
    let mut error_storage: BarCodeQueryHashStorageImpl = BarCodeQueryHashStorageImpl::new("error.txt".to_string());
    existing_storage.load();

    let mut query_app = BarcodeQueryAppImpl {
        reader: Box::new( ConsoleBarcodeReader {}),
        existing_storage,
        error_storage
    };
    query_app.run();
}
