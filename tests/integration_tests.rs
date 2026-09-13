use ad_pathfinder_engine::core::graph::AdGraph;
use ad_pathfinder_engine::engine::traverser::shortest_path;

#[test]
fn test_shortest_path_traversal() {
    let mut graph = AdGraph::new();
    graph.add_edge("UserA".to_string(), "GroupB".to_string());
    graph.add_edge("GroupB".to_string(), "ComputerC".to_string());

    let path = shortest_path(&graph, "UserA", "ComputerC");
    assert!(path.is_some());
    let unwrapped = path.unwrap();
    assert_eq!(unwrapped, vec!["UserA", "GroupB", "ComputerC"]);
}
