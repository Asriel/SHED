use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Store<T> {
    pub map: HashMap<String, T>,
}

impl<T> Store<T> {
    pub fn new() -> Store<T> {
        Store {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: T) {
        self.map.insert(key, value);
    }

    pub fn remove(&mut self, key: &String) -> Option<T> {
        self.map.remove(key)
    }
}
