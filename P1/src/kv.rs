use std::collections::HashMap;

/// KvStore stores key/value in memory
#[derive(Default)]
pub struct KvStore {
    inner: HashMap<String, String>,
}

impl KvStore {
    /// create a new KvStore
    pub fn new() -> KvStore {
        KvStore::default()
    }

    /// set key/value
    pub fn set(&mut self, k: String, v: String) {
        let _ = self.inner.insert(k, v);
    }

    /// get value from key
    pub fn get(&self, k: String) -> Option<String> {
        self.inner.get(&k).cloned()
    }

    /// remove the key/value
    pub fn remove(&mut self, k: String) {
        let _ = self.inner.remove(&k);
    }
}
