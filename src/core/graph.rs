use std::collections::HashMap;

pub struct AdGraph {
    pub adjacency_list: HashMap<String, Vec<String>>,
}

impl AdGraph {
    pub fn new() -> Self {
        Self {
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: String, to: String) {
        self.adjacency_list.entry(from).or_default().push(to);
    }
}
