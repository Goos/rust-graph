use std::hash::Hash;

use crate::graph::Graph;

pub trait WeightedGraph<K, V, W>: Graph<K, V> 
where
    K: Copy + Hash + Eq,
    V: PartialEq
{
    fn add_connection(
        &mut self, 
        source: &K, 
        destination: &K, 
        weight: W
    ) -> bool;
}
