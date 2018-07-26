extern crate rand;

use std::collections::HashMap;
use std::collections::HashSet;

pub trait PathableGraph {
    fn get_neighbors(&self, String) -> Option<&HashSet<String>>;
}

// Take a list of edges and make a graph out of it.
fn build_graph(edges: Vec<(String, String)>) -> HashMap<String, HashSet<String>> {
    let mut graph_map: HashMap<String, HashSet<String>> = HashMap::new();

    for edge in edges {
        let first_node: String = edge.0;
        let second_node: String = edge.1;

        let new_connections_first = match graph_map.get(&first_node).as_mut() {
            Some(connections) => {
                let mut c: HashSet<String> = connections.iter().cloned().collect();
                c.insert(second_node.clone());
                c
            },
            None => {
                let mut c: HashSet<String> = HashSet::new();
                c.insert(second_node.clone());
                c
            }
        };
        graph_map.insert(first_node.clone(), new_connections_first);

        let new_connections_second = match graph_map.get(&second_node).as_mut() {
            Some(connections) => {
                let mut c: HashSet<String> = connections.iter().cloned().collect();
                c.insert(first_node.clone());
                c
            },
            None => {
                let mut c: HashSet<String> = HashSet::new();
                c.insert(first_node.clone());
                c
            }
        };
        graph_map.insert(second_node.clone(), new_connections_second);
    }

    return graph_map;
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::collections::HashSet;

    use graphpath;
    use graphpath::PathableGraph;

    struct TestPathableGraph {
        graph: HashMap<String, HashSet<String>>,
    }

    impl graphpath::PathableGraph for TestPathableGraph {
        fn get_neighbors(&self, node: String) -> Option<&HashSet<String>> {
            return self.graph.get(&node);
        }
    }

    #[test]
    fn test_build_graph() {
        // build a test graph.
        let connections = vec![
            ("a", "b"),
            ("a", "c"),
            ("a", "d"),
            ("b", "c"),
        ];

        let mut string_connections = Vec::new();
        for connection in connections {
            let string_connection = (String::from(connection.0), String::from(connection.1));
            string_connections.push(string_connection);
        }

        // create pathable graph and put our graph on it.
        let graph = graphpath::build_graph(string_connections);
        let path_graph = TestPathableGraph {
            graph: graph,
        };

        // gross
        assert_eq!(path_graph.get_neighbors("a".to_string()), Some(&["b".to_string(), "c".to_string(),
        "d".to_string()].iter().cloned().collect()));
        assert_eq!(path_graph.get_neighbors("b".to_string()), Some(&["a".to_string(), "c".to_string()].iter().cloned().collect()));
        assert_eq!(path_graph.get_neighbors("c".to_string()), Some(&["a".to_string(), "b".to_string()].iter().cloned().collect()));
        assert_eq!(path_graph.get_neighbors("d".to_string()), Some(&["a".to_string()].iter().cloned().collect()));
    }
}
