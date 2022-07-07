pub trait BarCodeQuery {
    fn query(&self, bar_code: BarCode) -> bool;
}

pub struct BarCode {
    pub code_string: String,
}

pub struct ResultError {
    error: String,
}

pub struct ResultExisting {
    bar_code: BarCode,
}

pub struct ResultDuplicated {
    bar_code: BarCode,
}
