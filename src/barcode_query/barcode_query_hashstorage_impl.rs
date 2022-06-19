use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::Write;
use crate::barcode_query::model::{BarCodeQuery, BarCodeQueryFactory};
use crate::barcode_query::storage::{BarCodeStorage, BarcodeStorageEnum};

impl BarCodeQuery for BarCodeQueryHashStorageImpl {
    fn query(&self, input: String) -> bool {
        self.existing_storage.contains(&input)
    }

    fn query_with_insert(&mut self, input: String) -> bool {
        if self.existing_storage.contains(&input) {
            return true;
        }
        self.new_storage.insert(input);

        return false;
    }
}


pub struct BarCodeQueryHashStorageImpl {
    existing_storage: HashSet<String>,
    new_storage: HashSet<String>,
}

impl BarCodeQueryFactory for BarCodeQueryHashStorageImpl {
    fn new() -> BarCodeQueryHashStorageImpl {
        BarCodeQueryHashStorageImpl {
            existing_storage: HashSet::new(),
            new_storage: HashSet::new(),
        }
    }
}

impl BarCodeStorage for BarCodeQueryHashStorageImpl {
    fn load(&mut self, input_file: String, storage_type: BarcodeStorageEnum) -> bool {
        let content = fs::read_to_string(input_file).expect("cannot read file");
        for line in content.split("\n") {
            match storage_type {
                BarcodeStorageEnum::ExistingStorage => {
                    self.existing_storage.insert(line.parse().unwrap());
                }
                BarcodeStorageEnum::NewStorage => {
                    self.new_storage.insert(line.parse().unwrap());
                }
            }
        }
        true
    }

    fn dump(&mut self, output_file: String, storage_type: BarcodeStorageEnum) -> bool {
        let mut file = File::create(output_file).expect("unable to create file");

        match storage_type {
            BarcodeStorageEnum::ExistingStorage => {
                for line in &self.existing_storage {
                    file.write_fmt(format_args!("{}\n", line)).expect("could not write to file");
                }
            }
            BarcodeStorageEnum::NewStorage => {
                for line in &self.new_storage {
                    file.write_fmt(format_args!("{}\n", line)).expect("could not write to file");
                }
            }
        }

        true
    }

    fn get_mut_existing_storage(&mut self) -> &mut HashSet<String> {
        &mut self.existing_storage
    }
}
