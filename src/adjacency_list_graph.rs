use num::traits::Zero;

use crate::graph::Graph;
use crate::weighted_graph::{NoWeight, WeightedGraph};

#[derive(Debug, Copy, Clone)]
pub struct AdjacencyListEdge<W: Copy> {
    destination: usize,
    weight: W
}

#[derive(Debug)]
pub struct AdjacencyListGraph<V, W = NoWeight>
where
    V: PartialEq,
    W: PartialOrd + Zero + Copy
{
    nodes: Vec<V>,
    edges: Vec<Vec<AdjacencyListEdge<W>>>
}

impl<V, W> AdjacencyListGraph<V, W>
where 
    V: PartialEq,
    W: PartialOrd + Zero + Zero + Copy
{
    pub fn new(nodes: Vec<V>) -> AdjacencyListGraph<V, W> {
        let edges = vec![vec![]; nodes.len()];
        AdjacencyListGraph {
            nodes,
            edges
        }
    }
}

pub struct EdgeDestinationIterator<'a, W> 
where W: Copy
{
    iter: std::slice::Iter<'a, AdjacencyListEdge<W>>,
}

impl<'a, W> Iterator for EdgeDestinationIterator<'a, W> 
where W: Copy + 'a
{
    type Item = &'a usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|e| &e.destination)
    }
}

impl<'a, W> DoubleEndedIterator for EdgeDestinationIterator<'a, W> 
where W: Copy + 'a
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|e| &e.destination)
    }
}

impl<'a, V, W> Graph<'a, usize, V> for AdjacencyListGraph<V, W>
where
    V: PartialEq + 'a,
    W: PartialOrd + Zero + Copy + 'a
{
    type EdgeIterator = EdgeDestinationIterator<'a, W>;

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
        let edge = AdjacencyListEdge {
            destination: destination.clone(),
            weight: W::zero()
        };
        edges.push(edge);
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
        if let Some(index) = edges.iter().position(|e| &e.destination == destination) {
            edges.remove(index);
            true
        } else {
            false
        }
    }

    fn get(&'a self, key: &usize) -> Option<(&V, Self::EdgeIterator)> {
        let Some(node) = self.nodes.get(*key) else { return None; } ;
        let Some(edges) = self.edges.get(*key) else { return None; };
        let destination_iter = EdgeDestinationIterator {
            iter: edges.iter()
        };
        Some((node, destination_iter))
    }

    fn get_value(&self, key: &usize) -> Option<&V> {
        self.nodes.get(*key)
    }

    fn get_edges(&'a self, key: &usize) -> Option<Self::EdgeIterator> {
        let edges = self.edges.get(*key)?;
        let destination_iter = EdgeDestinationIterator {
            iter: edges.iter()
        };
        Some(destination_iter)
    }
}

pub struct WeightedEdgeIterator<'a, W> 
where W: Copy
{
    iter: std::slice::Iter<'a, AdjacencyListEdge<W>>,
}

impl<'a, W> Iterator for WeightedEdgeIterator<'a, W> 
where W: Copy + 'a
{
    type Item = (&'a usize, &'a W);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|e| (&e.destination, &e.weight))
    }
}

impl<'a, W> DoubleEndedIterator for WeightedEdgeIterator<'a, W> 
where W: Copy + 'a
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|e| (&e.destination, &e.weight))
    }
}

impl<'a, V, W> WeightedGraph<'a, usize, V, W> for AdjacencyListGraph<V, W> 
where
    V: PartialEq + 'a,
    W: PartialOrd + Zero + Copy + 'a
{
    type WeightedEdgeIterator = WeightedEdgeIterator<'a, W>;

    fn add_weighted_connection(
        &mut self, 
        source: &usize, 
        destination: &usize, 
        weight: W
    ) -> bool {
        let Some(edges) = self.edges.get_mut(source.clone()) else {
            return false;
        };
        let edge = AdjacencyListEdge {
            destination: destination.clone(),
            weight
        };
        edges.push(edge);
        true
    }

    fn get_weighted(&'a self, key: &usize) -> Option<(&V, Self::WeightedEdgeIterator)> {
        let Some(node) = self.nodes.get(*key) else { return None; } ;
        let Some(edges) = self.edges.get(*key) else { return None; };
        let edges_iter = WeightedEdgeIterator {
            iter: edges.iter()
        };
        Some((node, edges_iter))
    }

    fn get_weighted_edges(&'a self, key: &usize) -> Option<Self::WeightedEdgeIterator> {
        let edges = self.edges.get(*key)?;
        let edges_iter = WeightedEdgeIterator {
            iter: edges.iter()
        };
        Some(edges_iter)
    }
}

#[cfg(test)]
mod tests {
    use crate::searchable_graph::SearchableGraph;

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

        assert!(graph.get_edges(&0).unwrap().eq(vec![&1, &2, &3]));
        assert!(graph.get_edges(&1).unwrap().eq(vec![&2, &3, &4]));
        
        let (node_0, edges_0) = graph.get(&0).unwrap();
        assert_eq!(node_0, &String::from("node-1"));
        assert!(edges_0.eq(vec![&1, &2, &3]));
        let (node_5, edges_5) = graph.get(&5).unwrap();
        assert_eq!(node_5, &String::from("node-6"));
        let empty = vec![] as Vec<&usize>;
        assert!(edges_5.eq(empty));
    }

    #[test]
    fn test_dfs_search() {
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
    }

    #[test]
    fn test_bfs_search() {
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
        let path_bfs = graph.find_path_bfs(&0, &3);
        assert_eq!(path_bfs, Some(vec![0, 3]));
    }

    #[test]
    fn test_weighted_getters() {
        let mut graph: AdjacencyListGraph<String, i32> = AdjacencyListGraph::new(
            vec![
                String::from("node-1"),
                String::from("node-2"),
                String::from("node-3"),
                String::from("node-4"),
                String::from("node-5"),
                String::from("node-6"),
            ]
        );
        graph.add_weighted_connection(&0, &1, 1);
        graph.add_weighted_connection(&0, &2, 2);
        graph.add_weighted_connection(&0, &3, 3);
        graph.add_weighted_connection(&1, &2, 4);
        graph.add_weighted_connection(&1, &3, 5);
        graph.add_weighted_connection(&1, &4, 6);
        graph.add_weighted_connection(&2, &3, 7);
        assert!(graph.get_weighted_edges(&0).unwrap().eq(vec![(&1, &1), (&2, &2), (&3, &3)]));
        assert!(graph.get_weighted_edges(&1).unwrap().eq(vec![(&2, &4), (&3, &5), (&4, &6)]));
        
        let (node_0, edges_0) = graph.get_weighted(&0).unwrap();
        assert_eq!(node_0, &String::from("node-1"));
        assert!(edges_0.eq(vec![(&1, &1), (&2, &2), (&3, &3)]));
        let (node_5, edges_5) = graph.get_weighted(&5).unwrap();
        assert_eq!(node_5, &String::from("node-6"));
        let empty = vec![] as Vec<(&usize, &i32)>;
        assert!(edges_5.eq(empty));
    }
}
