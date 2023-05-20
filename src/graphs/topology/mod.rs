mod topology_graph;

use petgraph::stable_graph::{EdgeIndex, NodeIndex};
pub use topology_graph::TopologyGraph;

use crate::core::{EdgeId, NodeId};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TopoNode {
    pub id: NodeIndex,
    pub node_id: NodeId,
}

#[derive(Clone, Eq, PartialEq)]
pub struct TopoEdge {
    pub id: EdgeIndex,
    pub from: NodeId,
    pub to: NodeId,
    pub edge_id: EdgeId,
}
