use std::collections::HashMap;

pub struct Cache {
    data: HashMap<String, String>,
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
}
