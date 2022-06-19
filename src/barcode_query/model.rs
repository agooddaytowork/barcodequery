pub trait BarCodeQuery {
    fn query(&self, input: String) -> bool;
    fn query_with_insert(&mut self, input: String) -> bool;
}

pub trait BarCodeQueryFactory {
    fn new() -> Self;
}
