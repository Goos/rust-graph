use num::traits::Zero;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::ops::Add;

use crate::graph::Graph;
use crate::weighted_graph::WeightedGraph;

#[derive(Debug, Clone)]
struct LinkedNode<T: Copy> {
    value: T,
    parent: Option<Box<LinkedNode<T>>>,
}

impl<T> LinkedNode<T>
where T: Copy
{
    fn new(value: T) -> LinkedNode<T> {
        LinkedNode { value, parent: None }
    }

    fn flatten(&self) -> Vec<T> {
        let mut list: Vec<T> = vec![];
        let mut current: Option<&LinkedNode<T>> = Some(self);
        while let Some(node) = current {
            list.push(node.value);
            current = if let Some(node) = &node.parent {
                Some(&**node)
            } else {
                None
            };
        }
        list.into_iter().rev().collect()
    }
}

pub trait SearchableGraph<'a, K, V>: Graph<'a, K, V>
where
    K: Copy + Hash + Eq + 'a,
    V: PartialEq + 'a
{
    /// Returns the first path found between two nodes in the graph,
    /// doing a depth-first search.
    /// # Arguments
    /// * `source` - the key of the source node for the connection.
    /// * `destination` - the key of the destination node for the connection.
    fn find_path_dfs(&'a self, source: &K, destination: &K) -> Option<Vec<K>> {
        let mut visited: Vec<K> = vec![];
        let mut stack: Vec<LinkedNode<K>> = vec![];
        stack.push(LinkedNode::new(source.clone()));

        while let Some(node) = stack.pop() {
            if !visited.contains(&node.value) {
                visited.push(node.value);
                if &node.value == destination {
                    return Some(node.flatten());
                } else if let Some(edges) = self.get_edges(&node.value) {
                    for edge in edges.rev() {
                        let mut edge_node = LinkedNode::new(edge.clone());
                        edge_node.parent = Some(Box::new(node.clone()));
                        stack.push(edge_node);
                    }
                }
            }
        }

        None
    }

    /// Returns the first path found between two nodes in the graph,
    /// doing a breadth-first search.
    /// # Arguments
    /// * `source` - the key of the source node for the connection.
    /// * `destination` - the key of the destination node for the connection.
    fn find_path_bfs(&'a self, source: &K, destination: &K) -> Option<Vec<K>> {
        let mut visited: HashSet<K> = HashSet::new();
        let mut queue: VecDeque<LinkedNode<K>> = VecDeque::new();
        queue.push_front(LinkedNode::new(source.clone()));

        while let Some(node) = queue.pop_front() {
            if &node.value == destination {
                return Some(node.flatten());
            } else if let Some(edges) = self.get_edges(&node.value) {
                for edge in edges {
                    if !visited.contains(edge) {
                        visited.insert(edge.clone());
                        let mut edge_node = LinkedNode::new(edge.clone());
                        edge_node.parent = Some(Box::new(node.clone()));
                        queue.push_back(edge_node);
                    }
                }
            }
        }

        None
    }
}

impl<'a, T, K, V> SearchableGraph<'a, K, V> for T
where
    T: Graph<'a, K, V>,
    K: Copy + Hash + Eq + 'a,
    V: PartialEq + 'a
{
}

