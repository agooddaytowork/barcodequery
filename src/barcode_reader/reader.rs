pub trait BarcodeReader {
    fn read(&self) -> String;
}
