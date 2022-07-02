pub trait BarCodeQuery {
    fn query(&self, bar_code: BarCode) -> bool;
}

pub struct BarCode {
    pub code_string: String
}