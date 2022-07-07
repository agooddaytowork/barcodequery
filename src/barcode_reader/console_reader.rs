use crate::barcode_reader::reader::BarcodeReader;
use std::io::{stdin, stdout, Write};

pub struct ConsoleBarcodeReader {}

impl BarcodeReader for ConsoleBarcodeReader {
    fn read(&self) -> String {
        let mut s = String::new();
        print!("Please enter some barcode: ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        println!("You typed: {}", s);
        s
    }
}
