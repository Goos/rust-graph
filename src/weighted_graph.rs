use num::traits::Zero;
use std::cmp::Ordering;
use std::hash::Hash;
use std::ops::Add;

use crate::graph::Graph;

#[derive(Debug, Copy, Clone)]
pub struct NoWeight {}

impl PartialEq for NoWeight {
    #[inline]
    fn eq(&self, _other: &NoWeight) -> bool {
        true
    }
    #[inline]
    fn ne(&self, _other: &NoWeight) -> bool {
        false
    }
}

impl PartialOrd for NoWeight {
    #[inline]
    fn partial_cmp(&self, _: &NoWeight) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl Add for NoWeight {
    type Output = NoWeight;
    #[inline]
    fn add(self, _rhs: NoWeight) -> NoWeight {
        NoWeight {}
    }
}

impl Zero for NoWeight {
    #[inline]
    fn zero() -> Self {
        NoWeight {}
    }
    #[inline]
    fn is_zero(&self) -> bool {
        true
    }
}


pub trait WeightedGraph<'a, K, V, W>: Graph<'a, K, V> 
where
    K: Copy + Eq + 'a,
    V: PartialEq + 'a,
    W: 'a
{
    /// An iterator that iterates over the weighted edges of a node in the graph.
    type WeightedEdgeIterator: DoubleEndedIterator<Item = (&'a K, &'a W)>;

    /// Adds a connection between two nodes in the graph with a weight.
    /// # Arguments
    /// * `source` - the key of the source node for the connection.
    /// * `destination` - the key of the destination node for the connection.
    /// * `weight` - the weight of the connection.
    fn add_weighted_connection(
        &mut self, 
        source: &K, 
        destination: &K, 
        weight: W
    ) -> bool; 

    /// Gets a node and its weighted edges in the graph, given its key, if it exists, otherwise `None`.
    /// # Arguments
    /// * `key` - the key to return the node and edges for
    fn get_weighted(&'a self, key: &K) -> Option<(&V, Self::WeightedEdgeIterator)>;

    /// Gets the weighted edges of a node in the graph, given its key, if it exists, otherwise `None`.
    /// # Arguments
    /// * `key` - the key of the node to return the edges for
    fn get_weighted_edges(&'a self, key: &K) -> Option<Self::WeightedEdgeIterator>;
}

