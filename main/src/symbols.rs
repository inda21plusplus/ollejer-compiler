use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolMap<V> {
    symbols: HashMap<String, V>,
    parent: Option<Box<SymbolMap<V>>>,
}

impl<V> SymbolMap<V> {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::<String, V>::new(),
            parent: None,
        }
    }

    pub fn get(&self, key: String) -> Option<&V> {
        self.symbols.get(&key)
    }

    pub fn set(&mut self, key: String, value: V) -> Option<V> {
        self.symbols.insert(key, value)
    }

    pub fn remove(&mut self, key: String) -> Option<V> {
        self.symbols.remove(&key)
    }
}
