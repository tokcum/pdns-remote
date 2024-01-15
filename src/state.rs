use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct State(Arc<Mutex<HashMap<String, u32>>>);

impl State {
    pub fn new() -> Self {
        State(Arc::new(Mutex::new(HashMap::new())))
    }

    pub fn get(&self, key: String) -> Option<u32> {
        let map = self.0.lock().unwrap();
        map.get(&key).cloned()
    }

    pub fn incr(&self, key: String) {
        let mut map = self.0.lock().unwrap();
        let value = map.get(&key).cloned().unwrap();
        map.insert(key, value + 1);
    }

    pub fn test(&self) -> &Self {
        let mut map = self.0.lock().unwrap();
        map.insert("www.google.de".to_string(), 1);

        self
    }
}
