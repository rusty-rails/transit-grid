mod topology_graph;

use petgraph::stable_graph::{EdgeIndex, NodeIndex};
use std::fmt;
pub use topology_graph::TopologyGraph;

use crate::core::{EdgeId, NodeId};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TopoNode {
    pub id: NodeIndex,
    pub node_id: NodeId,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TopoEdge {
    pub id: EdgeIndex,
    pub from: NodeId,
    pub to: NodeId,
    pub edge_id: EdgeId,
}

impl fmt::Display for TopoNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TopoNode: {{ id: {:?}, node_id: {:?} }}",
            self.id, self.node_id
        )
    }
}

impl fmt::Display for TopoEdge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TopoEdge: {{ id: {:?}, from: {:?}, to: {:?}, edge_id: {:?} }}",
            self.id, self.from, self.to, self.edge_id
        )
    }
}
