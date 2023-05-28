//! # operations
//!
//! This module provides abstractions and implementations for modifying a transit network. A transit network is represented as a graph,
//! where each node is a `TransitNode` (a point in the transit network where a vehicle can stop) and each edge represents a path
//! (`TransitEdge`) between two transit nodes. The main trait provided by this module is `TransitNetworkModifier`.
//!
//! ## `TransitNetworkModifier`
//!
//! The `TransitNetworkModifier` trait provides an interface for modifying the network. This includes adding nodes and edges to the network.
//! Implementors of this trait can be used to add `TransitNode` and `TransitEdge` instances to a network.
//!
//! For instance, an implementor might add a `TransitNode` to an internal data structure upon invocation of the `add_node` method.
//! Similarly, the `add_edge` and `add_edge_with_accessibility` methods are used to add `TransitEdge` instances to the network.
//! The `add_edge_with_accessibility` method also allows specifying the accessibility of the edge, represented by the `Accessability` enum.
//!

use crate::core::{Accessability, NodeId, TransitEdge, TransitNode};
use geo::CoordNum;

/// Trait providing methods for modifying a transit network.
///
/// This trait provides an abstraction for modifying a transit network, which is represented as a graph with `TransitNode` instances as nodes and `TransitEdge` instances as edges.
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
    /// * `accessibility` - The `Accessability` of the edge.
    fn add_edge_with_accessibility(&mut self, edge: TransitEdge<T>, accessibility: Accessability);
}
