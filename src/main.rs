mod tests;
use list::List;
use std::collections::HashMap;
use std::fmt::Debug;


pub struct Graph<T> {
    nodes: List<T>,
    edges: HashMap<usize, List<usize>>, 
}

impl<T> Graph<T>
where
    T: PartialEq<T>,
    T: Debug,
{
    pub fn new() -> Self {
        Graph {
            nodes: List::new(),
            edges: HashMap::new(), 
        }
    }

    pub fn add_node(&mut self, value: T) {
        let index = self.nodes.len(); 
        self.nodes.push(value);
        self.edges.insert(index, List::new()); 
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.nodes.len() && to < self.nodes.len() {
            // from-to
            if let Some(edges) = self.edges.get_mut(&from) {
                edges.push(to);
            }
            // to-from
            if let Some(edges) = self.edges.get_mut(&to) {
                edges.push(from);
            }
        } else {
            panic!("Node indices out of bounds.");
        }
    }

    pub fn contains(&self, value: T) -> bool {
        return self.nodes
        .iter()
        .any(|current| *current == value);
    }

    pub fn find_path(&self, start: usize, end: usize) -> Option<Vec<usize>> {
        let mut visited = vec![false; self.nodes.len()];
        let mut path = Vec::new();

        if self.find_path_recursive(start, end, &mut visited, &mut path) {
            return Some(path);
        } else {
            return None;
        }
    }

    fn find_path_recursive(
        &self, 
        current: usize, 
        end: usize, 
        visited: &mut Vec<bool>, 
        path: &mut Vec<usize>
    ) -> bool {
        visited[current] = true;
        path.push(current);

        if current == end {
            return true;
        }

        if let Some(edges) = self.edges.get(&current) {
            for &to_node in edges.iter() {
                if !visited[to_node] {
                    if self.find_path_recursive(to_node, end, visited, path) {
                        return true;
                    }
                }
            }
        }

        path.pop();
        return false;
    }
}

fn main() {
    let mut graph = Graph::new();

    graph.add_node("A");
    graph.add_node("B");
    graph.add_node("C");
    graph.add_node("D");

    let a = 0;
    let b = 1;
    let c = 2;
    let d = 3;

    graph.add_edge(a, b);
    graph.add_edge(b, c);
    graph.add_edge(c, d);

    println!("Graph contains 'C': {}", graph.contains("C"));
    println!("Graph contains 'E': {}", graph.contains("E"));

    if let Some(path) = graph.find_path(a, d) {
        println!("Path from A to D: {:?}", path);
    } else {
        println!("No path found from A to D.");
    }

    if let Some(path) = graph.find_path(d, a) {
        println!("Path from D to A: {:?}", path);
    } else {
        println!("No path found from D to A.");
    }

}
