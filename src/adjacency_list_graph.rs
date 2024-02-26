use crate::graph::Graph;
use crate::algorithms::SearchableGraph; 

#[derive(Debug)]
pub struct AdjacencyListGraph<V>
where
    V: PartialEq
{
    nodes: Vec<V>,
    edges: Vec<Vec<usize>>
}

impl<V> AdjacencyListGraph<V>
where 
    V: PartialEq
{
    pub fn new(nodes: Vec<V>) -> AdjacencyListGraph<V> {
        let edges: Vec<Vec<usize>> = vec![vec![]; nodes.len()];
        AdjacencyListGraph {
            nodes,
            edges
        }
    }
}

impl<V> Graph<usize, V> for AdjacencyListGraph<V>
where
    V: PartialEq
{

    fn insert(&mut self, value: V) -> usize {
        self.nodes.push(value);
        self.edges.push(vec![]);
        self.nodes.len() - 1
    }

    fn remove(&mut self, key: &usize) -> Option<V> {
        let index = key.clone();
        if index >= self.nodes.len() {
            return None;
        }

        self.edges.remove(index);
        Some(self.nodes.remove(index))
    }

    fn add_connection(
        &mut self, 
        source: &usize,
        destination: &usize
    ) -> bool {
        let Some(edges) = self.edges.get_mut(source.clone()) else {
            return false;
        };
        edges.push(destination.clone());
        true
    }

    fn remove_connection(
        &mut self, 
        source: &usize, 
        destination: &usize
    ) -> bool {
        let Some(edges) = self.edges.get_mut(source.clone()) else {
            return false;
        };
        if let Some(index) = edges.iter().position(|e| e == destination) {
            edges.remove(index);
            true
        } else {
            false
        }
    }

    fn get(&self, key: &usize) -> Option<(&V, &Vec<usize>)> {
        let Some(node) = self.nodes.get(*key) else { return None; } ;
        let Some(edges) = self.edges.get(*key) else { return None; };
        Some((node, edges))
    }

    fn get_value(&self, key: &usize) -> Option<&V> {
        self.nodes.get(*key)
    }

    fn get_edges(&self, key: &usize) -> Option<&Vec<usize>> {
        self.edges.get(*key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getters() {
        let mut graph: AdjacencyListGraph<String> = AdjacencyListGraph::new(
            vec![
                String::from("node-1"),
                String::from("node-2"),
                String::from("node-3"),
                String::from("node-4"),
                String::from("node-5"),
                String::from("node-6"),
            ]
        );

        graph.add_connection(&0, &1);
        graph.add_connection(&0, &2);
        graph.add_connection(&0, &3);
        graph.add_connection(&1, &2);
        graph.add_connection(&1, &3);
        graph.add_connection(&1, &4);
        graph.add_connection(&2, &3);

        assert_eq!(graph.get_edges(&0), Some(&vec![1, 2, 3]));
        assert_eq!(graph.get_edges(&1), Some(&vec![2, 3, 4]));
        
        assert_eq!(graph.get(&0), Some((&String::from("node-1"), &vec![1, 2, 3])));
        assert_eq!(graph.get(&5), Some((&String::from("node-6"), &vec![])));
    }

    #[test]
    fn test_search_search_strategies() {
        let mut graph: AdjacencyListGraph<String> = AdjacencyListGraph::new(
            vec![
                String::from("node-1"),
                String::from("node-2"),
                String::from("node-3"),
                String::from("node-4"),
            ]
        );

        graph.add_connection(&0, &1);
        graph.add_connection(&0, &2);
        graph.add_connection(&0, &3);
        graph.add_connection(&1, &2);
        graph.add_connection(&2, &3);

        let path_dfs = graph.find_path_dfs(&0, &3);
        assert_eq!(path_dfs, Some(vec![0, 1, 2, 3]));

        let path_bfs = graph.find_path_bfs(&0, &3);
        assert_eq!(path_bfs, Some(vec![0, 3]));
    }
}
