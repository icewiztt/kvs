pub struct KvStore {}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

impl KvStore {
    pub fn new() -> KvStore {
        panic!()
    }
    pub fn set(&mut self, _a: String, _b: String) {
        panic!();
    }
    pub fn get(&self, _a: String) -> Option<String> {
        panic!();
    }
    pub fn remove(&mut self, _a: String) {
        panic!();
    }
}
