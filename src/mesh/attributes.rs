//! Entity attribute maps

use std::collections::HashMap;

/// Generic attribute map for mesh entities
pub struct AttributeMap<K, V> {
    pub map: HashMap<K, V>,
}

impl<K: std::hash::Hash + Eq, V> AttributeMap<K, V> {
    /// Create an empty attribute map
    pub fn new() -> Self {
        AttributeMap { map: HashMap::new() }
    }

    /// Insert a value for an entity
    pub fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }

    /// Get a reference to the value
    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }
}