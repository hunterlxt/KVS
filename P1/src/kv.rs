use std::collections::HashMap;

#[derive(Default)]
pub struct KvStore {
    inner: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore::default()
    }

    pub fn set(&mut self, key: String, value: String) {
        self.inner.insert(key, value);
    }

    pub fn get(&mut self, key: String) -> Option<String> {
        self.inner.get(&key).map(|v| v.clone())
    }

    pub fn remove(&mut self, key: String) {
        self.inner.remove(&key);
    }
}
