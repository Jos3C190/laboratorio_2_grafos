#[cfg(test)]
mod tests {
    use crate::Graph;


    #[test]
    fn test_add_node() {
        let mut graph = Graph::new();
        graph.add_node("A");
        assert_eq!(graph.nodes.len(), 1);
        assert_eq!(graph.contains("A"), true);
        assert_eq!(graph.contains("B"), false);
    }

    #[test]
    fn test_add_edge() {
        let mut graph = Graph::new();

        graph.add_node("A");
        graph.add_node("B");
        graph.add_node("C");

        graph.add_edge(0, 1); 
        graph.add_edge(1, 2);

        // Verificar cantidad de aristas
        assert_eq!(graph.edges.get(&0).unwrap().len(), 1);
        assert_eq!(graph.edges.get(&1).unwrap().len(), 2);
        assert_eq!(graph.edges.get(&2).unwrap().len(), 1);

        // Verificar cada relacion
        assert_eq!(graph.edges.get(&0).unwrap().iter().collect::<Vec<&usize>>(), vec![&1]);
        assert_eq!(graph.edges.get(&1).unwrap().iter().collect::<Vec<&usize>>(), vec![&0, &2]);
        assert_eq!(graph.edges.get(&2).unwrap().iter().collect::<Vec<&usize>>(), vec![&1]);

    }

    #[test]
    fn test_contains() {
        let mut graph = Graph::new();
        graph.add_node("A");
        assert_eq!(graph.contains("A"), true);
        assert_eq!(graph.contains("B"), false);
    }

    #[test]
    fn test_find_path() {
        let mut graph = Graph::new();
        graph.add_node("A");
        graph.add_node("B");
        graph.add_node("C");
        graph.add_node("D");
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);

        let path = graph.find_path(0, 3);
        assert_eq!(path, Some(vec![0, 1, 2, 3]));

        let path = graph.find_path(3, 0);
        assert_eq!(path, Some(vec![3, 2, 1, 0]));

        let path = graph.find_path(0, 2);
        assert_eq!(path, Some(vec![0, 1, 2]));

        let path = graph.find_path(2, 0);
        assert_eq!(path, Some(vec![2, 1, 0]));

        let path = graph.find_path(1, 3);
        assert_eq!(path, Some(vec![1, 2, 3]));

        let path = graph.find_path(3, 2);
        assert_eq!(path, Some(vec![3, 2]));

        let path = graph.find_path(1, 0);
        assert_eq!(path, Some(vec![1, 0]));
    }
}
