use crate::core::{Accessability, NodeId, TransitEdge, TransitNode};
use geo::CoordNum;

/// `TransitNetworkModifier` is a trait for modifying a transit network.
///
/// The transit network is represented as a graph, where each node is a transit node
/// (a point in the transit network where a vehicle can stop) and each edge represents
/// a path between two transit nodes.
///
/// This trait provides an abstraction for modifying the network, i.e., adding nodes
/// and edges to the network.
pub trait TransitNetworkModifier<R, T: CoordNum> {
    /// Adds a `TransitNode` to the network.
    ///
    /// # Arguments
    ///
    /// * `node` - The `TransitNode` to be added to the network.
    ///
    /// # Returns
    ///
    /// * `NodeId` - The ID of the added node.
    fn add_node(&mut self, node: TransitNode<R>) -> NodeId;

    /// Adds a `TransitEdge` to the network.
    ///
    /// # Arguments
    ///
    /// * `edge` - The `TransitEdge` to be added to the network.
    fn add_edge(&mut self, edge: TransitEdge<T>);

    /// Adds a `TransitEdge` to the network with a given accessibility.
    ///
    /// # Arguments
    ///
    /// * `edge` - The `TransitEdge` to be added to the network.
    /// * `accessibility` - The accessibility of the edge.
    fn add_edge_with_accessibility(&mut self, edge: TransitEdge<T>, accessibility: Accessability);
}
