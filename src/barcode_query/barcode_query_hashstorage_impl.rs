use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::Write;
use crate::barcode_query::model::{BarCodeQuery, BarCodeQueryFactory};
use crate::barcode_query::storage::{BarCodeStorage, BarcodeStorageEnum};

impl BarCodeQuery for BarCodeQueryHashStorageImpl {
    fn query(&self, input: String) -> bool {
        self.storage.contains(&input)
    }
}


pub struct BarCodeQueryHashStorageImpl {
    file_path: String,
    storage: HashSet<String>,
}

impl BarCodeQueryFactory for BarCodeQueryHashStorageImpl {
    fn new(file_path: String) -> BarCodeQueryHashStorageImpl {
        BarCodeQueryHashStorageImpl {
            file_path,
            storage: HashSet::new(),
        }
    }
}

impl BarCodeStorage for BarCodeQueryHashStorageImpl {
    fn load(&mut self) -> bool {
        let content = fs::read_to_string(&self.file_path).expect("cannot read file");
        for line in content.split("\n") {
            self.storage.insert(line.parse().unwrap());
        }
        true
    }

    fn dump(&mut self) -> bool {
        let mut file = File::create(&self.file_path).expect("unable to create file");

        for line in &self.storage {
            file.write_fmt(format_args!("{}\n", line)).expect("could not write to file");
        }
        true
    }

    fn insert(&mut self, input: String) {
        self.storage.insert(input);
    }
}
