use petgraph::{
    csr::{Csr, IndexType, NodeIndex},
    Directed,
};

use crate::core::{IdType, TopoEdge, TopoNode, TopoNodeId};

/// Represents the topological graph of the transit network.
///
/// Topological graph is directed and each node in the topological graph maps to a node in the physical graph.
/// This is particularly useful for scenarios such as rail switches where the directionality of edges matters.
pub struct TopologyGraph {
    pub graph: Csr<TopoNode, TopoEdge, Directed, IdType>,
}

impl TopologyGraph {
    /// Creates a new instance of `TopologyGraph`.
    pub fn new() -> Self {
        TopologyGraph { graph: Csr::new() }
    }

    /// Adds a `TopoNode` to the topological graph.
    ///
    /// # Arguments
    ///
    /// * `node` - The `TopoNode` to be added to the graph.
    ///
    /// # Returns
    ///
    /// * `TopoNodeId` - The ID of the added node.
    pub fn add_node(&mut self, node: TopoNode) -> TopoNodeId {
        self.graph.add_node(node)
    }

    /// Adds a `TopoEdge` to the topological graph.
    ///
    /// # Arguments
    ///
    /// * `edge` - The `TopoEdge` to be added to the graph.
    pub fn add_edge(&mut self, edge: TopoEdge) {
        self.graph.add_edge(
            NodeIndex::new(edge.from as usize),
            NodeIndex::new(edge.to as usize),
            edge,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topology_graph() {
        let mut topo_graph = TopologyGraph::new();

        let topo_node1 = TopoNode { id: 1, node_id: 1 };
        let topo_node2 = TopoNode { id: 2, node_id: 2 };

        let added_node_id1 = topo_graph.add_node(topo_node1);
        let added_node_id2 = topo_graph.add_node(topo_node2);

        assert_eq!(added_node_id1, 0);
        assert_eq!(added_node_id2, 1);
        assert_eq!(topo_graph.graph.node_count(), 2);

        let topo_edge = TopoEdge {
            id: 1,
            from: 0,
            to: 1,
            edge_id: 1,
        };
        topo_graph.add_edge(topo_edge);

        assert_eq!(topo_graph.graph.edge_count(), 1);
    }
}
