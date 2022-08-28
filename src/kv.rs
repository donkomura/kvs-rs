use std::collections::HashMap;

/// The `KvStore` stores string key/value pairs.
///
/// Key/value pairs are sotre in `HashMap` in memory and not persisted to disk.
///
/// Example:
///
/// ```rust
/// let mut sotre = KvStore::new();
/// store.set("key".to_owned(), "value".to_owned());
/// let val = store.get("key".to_owned());
/// assert_eq!(val, Some("value".to_owned()));
/// ```

#[derive(Default)]
pub struct KvStore {
    kvs: HashMap<String, String>,
}

impl KvStore {
    /// Creates a `KvStore`
    pub fn new() -> KvStore {
        KvStore {
            kvs: HashMap::new(),
        }
    }
    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be orverwritten.
    pub fn set(&mut self, key: String, value: String) {
        self.kvs.insert(key, value);
    }
    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exists.
    pub fn get(&self, key: String) -> Option<String> {
        self.kvs.get(&key).cloned()
    }
    /// Remove a given key.
    pub fn remove(&mut self, key: String) {
        self.kvs.remove(&key);
    }
}
