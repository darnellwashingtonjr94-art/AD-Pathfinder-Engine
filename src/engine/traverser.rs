use crate::core::graph::AdGraph;
use std::collections::VecDeque;

pub fn shortest_path(graph: &AdGraph, start: &str, target: &str) -> Option<Vec<String>> {
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();

    queue.push_back(start.to_string());
    visited.insert(start.to_string(), None);

    while let Some(current) = queue.pop_front() {
        if current == target {
            let mut path = Vec::new();
            let mut curr = Some(target.to_string());
            while let Some(node) = curr {
                path.push(node.clone());
                curr = visited.get(&node).cloned().flatten();
            }
            path.reverse();
            return Some(path);
        }

        if let Some(neighbors) = graph.adjacency_list.get(&current) {
            for neighbor in neighbors {
                if !visited.contains_key(neighbor) {
                    visited.insert(neighbor.clone(), Some(current.clone()));
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }
    None
}
