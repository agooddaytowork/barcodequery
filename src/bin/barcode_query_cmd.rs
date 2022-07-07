extern crate barcodequery;

use std::fs::File;

use barcodequery::barcode_query::barcode_query_hashstorage_impl::BarCodeFileHashStorageImpl;
use barcodequery::barcode_query::storage::BarCodeStorage;
use barcodequery::barcode_query_app::barcode_query_app::BarcodeQueryApp;
use barcodequery::barcode_reader::console_reader::ConsoleBarcodeReader;

fn main() {
    // TODO - read file path from config?
    let bar_code_file = File::open("test.txt").unwrap();
    let error_file = File::create("error.txt").unwrap();

    let mut existing_storage = BarCodeFileHashStorageImpl::new(bar_code_file);
    let error_storage = BarCodeFileHashStorageImpl::new(error_file);
    existing_storage.load();

    let mut query_app = BarcodeQueryApp {
        reader: Box::new(ConsoleBarcodeReader {}),
        existing_storage,
        error_storage,
    };
    query_app.run();
}
