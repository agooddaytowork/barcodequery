use std::collections::HashSet;

#[derive(PartialEq, Clone, Copy)]
pub enum BarcodeStorageEnum {
    ExistingStorage,
    NewStorage,
}
pub trait BarCodeStorage {
    fn load(&mut self, input_file: String, storage_type: BarcodeStorageEnum) -> bool;
    fn dump(&mut self, output_file: String, storage_type: BarcodeStorageEnum) -> bool;
    fn get_mut_existing_storage(&mut self) -> &mut HashSet<String>;
}