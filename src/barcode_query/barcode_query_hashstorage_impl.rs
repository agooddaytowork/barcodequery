use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use crate::barcode_query::model::{BarCode, BarCodeQuery};
use crate::barcode_query::storage::BarCodeStorage;

pub struct BarCodeFileHashStorageImpl {
    file: File,
    storage: HashSet<String>,
}

impl BarCodeQuery for BarCodeFileHashStorageImpl {
    fn query(&self, bar_code: BarCode) -> bool {
        self.storage.contains(&bar_code.code_string)
    }
}

impl BarCodeFileHashStorageImpl {
    pub fn new(file: File) -> BarCodeFileHashStorageImpl {
        BarCodeFileHashStorageImpl {
            file,
            storage: HashSet::new(),
        }
    }
}

impl BarCodeStorage for BarCodeFileHashStorageImpl {
    fn load(&mut self) -> bool {
        // bar code file storage much smaller than mem size
        let f = BufReader::new(&self.file);
        for line in f.lines() {
            self.storage.insert(line.unwrap());
        }
        true
    }

    fn dump(&mut self) -> bool {
        for line in &self.storage {
            let _ = &self
                .file
                .write_fmt(format_args!("{}\n", line))
                .expect("could not write to file");
        }
        true
    }

    fn insert(&mut self, input: String) {
        self.storage.insert(input);
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, Write};

    use tempfile::tempfile;

    use crate::barcode_query::barcode_query_hashstorage_impl::BarCodeFileHashStorageImpl;
    use crate::barcode_query::model::{BarCode, BarCodeQuery};
    use crate::barcode_query::storage::BarCodeStorage;

    #[test]
    fn can_query() {
        let mut temp_file = tempfile().unwrap();
        write!(temp_file, "123\n456\n").unwrap();
        temp_file.rewind().unwrap();
        let mut storage = BarCodeFileHashStorageImpl::new(temp_file);
        storage.load();

        assert_eq!(
            storage.query(BarCode {
                code_string: "123".to_string(),
            }),
            true
        );
        assert_eq!(
            storage.query(BarCode {
                code_string: "456".to_string(),
            }),
            true
        );
        assert_eq!(
            storage.query(BarCode {
                code_string: "567".to_string(),
            }),
            false
        );
    }
}
