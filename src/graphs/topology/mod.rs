//! This module contains the `TopologyGraph` and the structures `TopoNode` and `TopoEdge` to represent the nodes and edges.
//!
//! `TopologyGraph` provides a way of maintaining the topology of a graph and mapping between `NodeId`s and `EdgeId`s
//! (custom identifiers) and `NodeIndex` and `EdgeIndex` (indices in the petgraph).
//!
//! `TopoNode` and `TopoEdge` are used to represent nodes and edges within the `TopologyGraph`.
mod topology_graph;

use petgraph::stable_graph::{EdgeIndex, NodeIndex};
use std::fmt;
pub use topology_graph::TopologyGraph;

use crate::core::{EdgeId, NodeId};

/// Represents a node in the `TopologyGraph`.
///
/// Each node is identified by a `NodeIndex` (which represents the node's position in the petgraph)
/// and a `NodeId` (a custom identifier).
///
/// # Fields
///
/// * `id: NodeIndex` - The index of the node in the petgraph.
/// * `node_id: NodeId` - The custom identifier of the node.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TopoNode {
    /// The index of the node in the petgraph.
    pub id: NodeIndex,
    /// The custom identifier of the node.
    pub node_id: NodeId,
}

/// Represents an edge in the `TopologyGraph`.
///
/// Each edge is identified by an `EdgeIndex` (which represents the edge's position in the petgraph),
/// a `from` and `to` `NodeId` (representing the nodes that the edge connects),
/// and an `EdgeId` (a custom identifier).
///
/// # Fields
///
/// * `id: EdgeIndex` - The index of the edge in the petgraph.
/// * `from: NodeId` - The custom identifier of the node where the edge originates.
/// * `to: NodeId` - The custom identifier of the node where the edge ends.
/// * `edge_id: EdgeId` - The custom identifier of the edge.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TopoEdge {
    /// The index of the edge in the petgraph.
    pub id: EdgeIndex,
    /// The custom identifier of the node where the edge originates.
    pub from: NodeId,
    /// The custom identifier of the node where the edge ends.
    pub to: NodeId,
    /// The custom identifier of the edge.
    pub edge_id: EdgeId,
}

/// Formats the `TopoNode` for display purposes.
impl fmt::Display for TopoNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TopoNode: {{ id: {:?}, node_id: {:?} }}",
            self.id, self.node_id
        )
    }
}

/// Formats the `TopoEdge` for display purposes.
impl fmt::Display for TopoEdge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TopoEdge: {{ id: {:?}, from: {:?}, to: {:?}, edge_id: {:?} }}",
            self.id, self.from, self.to, self.edge_id
        )
    }
}
