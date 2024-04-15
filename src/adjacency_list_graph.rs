use num::traits::{PrimInt, Zero};

use crate::graph::Graph;
use crate::weighted_graph::{NoWeight, WeightedGraph};

#[derive(Debug, Copy, Clone)]
pub struct AdjacencyListEdge<K: Copy, W: Copy> {
    destination: K,
    weight: W
}

#[derive(Debug)]
pub struct AdjacencyListGraph<K, V, W = NoWeight>
where
    K: PrimInt + Copy,
    V: PartialEq,
    W: PartialOrd + Zero + Copy
{
    nodes: Vec<V>,
    edges: Vec<Vec<AdjacencyListEdge<K, W>>>
}

impl<K, V, W> AdjacencyListGraph<K, V, W>
where
    K: PrimInt + Copy,
    V: PartialEq,
    W: PartialOrd + Zero + Copy
{
    pub fn new(nodes: Vec<V>) -> AdjacencyListGraph<K, V, W> {
        let edges = vec![vec![]; nodes.len()];
        AdjacencyListGraph {
            nodes,
            edges
        }
    }
}

pub struct EdgeDestinationIterator<'a, K, W>
where
    K: Copy,
    W: Copy
{
    iter: std::slice::Iter<'a, AdjacencyListEdge<K, W>>,
}

impl<'a, K, W> Iterator for EdgeDestinationIterator<'a, K, W>
where 
    K: Copy,
    W: Copy + 'a
{
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|e| &e.destination)
    }
}

impl<'a, K, W> DoubleEndedIterator for EdgeDestinationIterator<'a, K, W>
where 
    K: Copy,
    W: Copy + 'a
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|e| &e.destination)
    }
}

impl<'a, K, V, W> Graph<'a, K, V> for AdjacencyListGraph<K, V, W>
where
    K: PrimInt + Copy + 'a,
    V: PartialEq + 'a,
    W: PartialOrd + Zero + Copy + 'a
{
    type EdgeIterator = EdgeDestinationIterator<'a, K, W>;

    fn insert(&mut self, value: V) -> K {
        self.nodes.push(value);
        self.edges.push(vec![]);
        K::from(self.nodes.len() - 1).unwrap()
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let index = key.to_usize()?;
        if index >= self.nodes.len() {
            return None;
        }

        self.edges.remove(index);
        Some(self.nodes.remove(index))
    }

    fn add_connection(
        &mut self, 
        source: &K,
        destination: &K
    ) -> bool {
        let index = source.to_usize().unwrap();
        let Some(edges) = self.edges.get_mut(index) else {
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
        source: &K,
        destination: &K
    ) -> bool {
        let index = source.to_usize().unwrap();
        let Some(edges) = self.edges.get_mut(index) else {
            return false;
        };
        if let Some(index) = edges.iter().position(|e| &e.destination == destination) {
            edges.remove(index);
            true
        } else {
            false
        }
    }

    fn get(&'a self, key: &K) -> Option<(&V, Self::EdgeIterator)> {
        let Some(node) = self.nodes.get(key.to_usize()?) else { return None; } ;
        let Some(edges) = self.edges.get(key.to_usize()?) else { return None; };
        let destination_iter = EdgeDestinationIterator {
            iter: edges.iter()
        };
        Some((node, destination_iter))
    }

    fn get_value(&self, key: &K) -> Option<&V> {
        self.nodes.get(key.to_usize()?)
    }

    fn get_edges(&'a self, key: &K) -> Option<Self::EdgeIterator> {
        let edges = self.edges.get(key.to_usize()?)?;
        let destination_iter = EdgeDestinationIterator {
            iter: edges.iter()
        };
        Some(destination_iter)
    }
}

pub struct WeightedEdgeIterator<'a, K, W>
where
    K: Copy,
    W: Copy
{
    iter: std::slice::Iter<'a, AdjacencyListEdge<K, W>>,
}

impl<'a, K, W> Iterator for WeightedEdgeIterator<'a, K, W>
where 
    K: Copy,
    W: Copy + 'a
{
    type Item = (&'a K, &'a W);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|e| (&e.destination, &e.weight))
    }
}

impl<'a, K, W> DoubleEndedIterator for WeightedEdgeIterator<'a, K, W>
where
    K: Copy,
    W: Copy + 'a
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|e| (&e.destination, &e.weight))
    }
}

impl<'a, K, V, W> WeightedGraph<'a, K, V, W> for AdjacencyListGraph<K, V, W> 
where
    K: PrimInt + Copy + 'a,
    V: PartialEq + 'a,
    W: PartialOrd + Zero + Copy + 'a
{
    type WeightedEdgeIterator = WeightedEdgeIterator<'a, K, W>;

    fn add_weighted_connection(
        &mut self, 
        source: &K, 
        destination: &K, 
        weight: W
    ) -> bool {
        let index = source.to_usize().unwrap();
        let Some(edges) = self.edges.get_mut(index) else {
            return false;
        };
        let edge = AdjacencyListEdge {
            destination: destination.clone(),
            weight
        };
        edges.push(edge);
        true
    }

    fn get_weighted(&'a self, key: &K) -> Option<(&V, Self::WeightedEdgeIterator)> {
        let Some(node) = self.nodes.get(key.to_usize()?) else { return None; } ;
        let Some(edges) = self.edges.get(key.to_usize()?) else { return None; };
        let edges_iter = WeightedEdgeIterator {
            iter: edges.iter()
        };
        Some((node, edges_iter))
    }

    fn get_weighted_edges(&'a self, key: &K) -> Option<Self::WeightedEdgeIterator> {
        let edges = self.edges.get(key.to_usize()?)?;
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
        let mut graph: AdjacencyListGraph<u16, String> = AdjacencyListGraph::new(
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
        let empty = vec![] as Vec<&u16>;
        assert!(edges_5.eq(empty));
    }

    #[test]
    fn test_dfs_search() {
        let mut graph: AdjacencyListGraph<u16, String> = AdjacencyListGraph::new(
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
        let mut graph: AdjacencyListGraph<u16, String> = AdjacencyListGraph::new(
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
        let mut graph: AdjacencyListGraph<u16, String, i32> = AdjacencyListGraph::new(
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
        let empty = vec![] as Vec<(&u16, &i32)>;
        assert!(edges_5.eq(empty));
    }

    #[test]
    fn test_adjacency_list_memory_layout() {
        // The memory size of unweighted edge structs is just the key size.
        let unweighted_edge_size = std::mem::size_of::<AdjacencyListEdge<u16, NoWeight>>();
        assert_eq!(unweighted_edge_size, 2);

        // The memory size of weighted edge structs is just the key size plus the weight size,
        // along with the alignment of the key (key = 2, weight = 4, alignment = 2).
        let weighted_edge_size = std::mem::size_of::<AdjacencyListEdge<u16, f32>>();
        assert_eq!(weighted_edge_size, 8);

        let noweight_size = std::mem::size_of::<NoWeight>();
        assert_eq!(noweight_size, 0);
    }
}
