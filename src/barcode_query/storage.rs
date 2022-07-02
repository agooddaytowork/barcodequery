#[derive(PartialEq, Clone, Copy)]
pub enum BarcodeStorageEnum {
    ExistingStorage,
    NewStorage,
}

pub trait BarCodeStorage {
    fn load(&mut self) -> bool;
    fn dump(&mut self) -> bool;
    fn insert(&mut self, input: String);
    // fn get_mut_storage(&mut self) -> &mut HashSet<String>;
}

