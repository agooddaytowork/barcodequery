use crate::barcode_query::storage::BarCodeStorage;

pub trait BarCodeQuery {
    fn query(&self, input: String) -> bool;
}

pub trait BarCodeQueryFactory {
    fn new(file_path: String) -> Self;
}
