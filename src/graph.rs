pub trait Graph<'a, K, V>
where
    K: Copy + Eq + 'a,
    V: PartialEq + 'a
{
    /// An iterator that iterates over the edges of a node in the graph.
    type EdgeIterator: DoubleEndedIterator<Item = &'a K>;

    /// Inserts a node into the graph, returning the key it was inserted by.
    /// # Arguments
    /// * `node` - the node to insert into the graph.
    fn insert(&mut self, node: V) -> K;

    /// Removes a node from the graph, returning the value if it was found, `None` otherwise.
    /// # Arguments
    /// * `key` - the key of the node to remove from the graph.
    fn remove(&mut self, key: &K) -> Option<V>;

    /// Adds a connection between two nodes in the graph.
    /// # Arguments
    /// * `source` - the key of the source node for the connection.
    /// * `destination` - the key of the destination node for the connection.
    fn add_connection(&mut self, source: &K, destination: &K) -> bool;
 
    /// Removes a connection between two nodes in the graph.
    /// # Arguments
    /// * `source` - the key of the source node for the connection.
    /// * `destination` - the key of the destination node for the connection.
    fn remove_connection(&mut self, source: &K, destination: &K) -> bool;

    /// Returns the node and its edges in the graph, given its key, if it exists, otherwise `None`.
    /// # Arguments
    /// * `key` - the key to return the node and edges for
    fn get(&'a self, key: &K) -> Option<(&V, Self::EdgeIterator)>;

    /// Returns the value of a node in the graph, given its key, if it exists, otherwise `None`.
    /// # Arguments
    /// * `key` - the key of the node to return the value for
    fn get_value(&self, key: &K) -> Option<&V>;

    /// Returns the edges of a node in the graph, given its key, if it exists, otherwise `None`.
    /// # Arguments
    /// * `key` - the key of the node to return the edges for
    fn get_edges(&'a self, key: &K) -> Option<Self::EdgeIterator>;
}
