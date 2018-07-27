extern crate rand;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt::Write;

pub trait PathableGraph {
    fn get_neighbors(&self, String) -> Option<&HashSet<String>>;
    fn contains_node(&self, String) -> bool;
}

fn find_path<T: PathableGraph>(graph: T, start: String, end: String) -> VecDeque<String> {
    // nodes to process.
    let mut q: VecDeque<String> = VecDeque::new();

    // visited nodes, to prevent loops.
    let mut visited: HashSet<String> = HashSet::new();

    // track first parent of any node starting from start, to reconstruct path.
    let mut parents: HashMap<String, String> = HashMap::new();

    // path between start and end
    let mut path: VecDeque<String> = VecDeque::new();

    // start and end must be in graph, or we can't start.
    if !graph.contains_node(start.clone()) || !graph.contains_node(end.clone()) {
        return path;
    }

    // special-case start.
    q.push_back(start.clone());
    parents.insert(start.clone(), "".to_string());

    while !q.is_empty() {

        // get an node
        let mut node = q.pop_front().unwrap().clone();

        // end test
        if node == end {

            // construct path
            while node != "" {
                path.push_front(node.clone());
                match parents.get(&node) {
                    Some(n) => node = n.to_string(),
                    None => node = "".to_string(),
                }
            }

            return path;
        }

        visited.insert(node.clone());

        match graph.get_neighbors(node.clone()) {
            Some(neighbors) => {

                // process neighbors
                for neighbor in neighbors {

                    // don't visit nodes twice
                    if visited.contains(neighbor) {
                        continue;
                    }
                    visited.insert(neighbor.to_string());

                    // mark first parent of node
                    if !parents.contains_key(neighbor) {
                        parents.insert(neighbor.to_string(), node.clone());
                    }

                    q.push_back(neighbor.to_string());
                }
            },
            None => {
                continue;
            },
        }

    }



    return path;
}

pub fn stringify_path(path: VecDeque<String>) -> String {
    let mut res = String::new();

    for (i, node) in path.iter().enumerate() {
        if i > 0 {
            write!(&mut res, "->'{}'", node).unwrap();
        } else {
            write!(&mut res, "'{}'", node).unwrap();
        }
    }

    return res;
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

        fn contains_node(&self, node: String) -> bool {
            return self.graph.contains_key(&node);
        }
    }

    #[test]
    fn test_get_path() {
        // build a test graph.
        let edges = vec![
            ("a", "b"),
            ("a", "c"),
            ("b", "c"),
            ("b", "d"),
            ("c", "d"),
            ("d", "e"),
            ("e", "f"),
            ("e", "h"),
            ("f", "g"),
            ("f", "k"),
            ("g", "k"),
            ("g", "h"),
            ("g", "j"),
            ("h", "j"),
            ("h", "i"),
            ("i", "j"),
            ("j", "l"),
            ("k", "l"),
        ];

        let mut string_edges = Vec::new();
        for edge in edges {
            let string_edge = (String::from(edge.0), String::from(edge.1));
            string_edges.push(string_edge);
        }

        // create pathable graph and put our graph on it.
        let graph = graphpath::build_graph(string_edges);
        let path_graph = TestPathableGraph {
            graph: graph,
        };

        let path = graphpath::find_path(path_graph, "a".to_string(), "f".to_string());
        assert_eq!(path.len(), 5);

        assert_eq!(path.front().unwrap(), "a");
        assert_eq!(path.back().unwrap(), "f");
    }

    #[test]
    fn test_build_graph() {
        // build a test graph.
        let edges = vec![
            ("a", "b"),
            ("a", "c"),
            ("a", "d"),
            ("b", "c"),
        ];

        let mut string_edges = Vec::new();
        for edge in edges {
            let string_edge = (String::from(edge.0), String::from(edge.1));
            string_edges.push(string_edge);
        }

        // create pathable graph and put our graph on it.
        let graph = graphpath::build_graph(string_edges);
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
