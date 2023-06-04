use crate::core::NodeId;

use super::TopologyGraph;

/// `TopologyGraphRepairer` provides functionality to manipulate and repair edges in a topological graph.
///
/// It provides methods for reversing dual edges and cross-linking dual edges.
pub trait TopologyGraphRepairer {
    /// Reverse the dual edge defined by the two given node IDs.
    ///
    /// Implementations should ensure that after this operation, the direction of the dual edge between the two nodes is reversed. This implies that if the edge was directed from `node1` to `node2`, it should be directed from `node2` to `node1` after this operation, and vice versa.
    ///
    /// # Arguments
    ///
    /// * `node1` - The ID of the first node defining the dual edge to be reversed.
    /// * `node2` - The ID of the second node defining the dual edge to be reversed.
    fn reverse_dual_edge(&mut self, node1: NodeId, node2: NodeId);

    /// Cross-link the dual edge defined by the two given node IDs.
    ///
    /// Implementations should ensure that after this operation, the dual edge between the two nodes is cross-linked. This implies that if there was a direct edge from `node1` to `node2`, there should now also be a direct edge from `node2` to `node1` after this operation, and vice versa.
    ///
    /// # Arguments
    ///
    /// * `node1` - The ID of the first node defining the dual edge to be cross-linked.
    /// * `node2` - The ID of the second node defining the dual edge to be cross-linked.
    fn cross_link_dual_edge(&mut self, node1: NodeId, node2: NodeId);
}

impl TopologyGraphRepairer for TopologyGraph {
    fn reverse_dual_edge(&mut self, node1: NodeId, node2: NodeId) {
        if let Some(edges) = self.find_edge_indices(node1, node2) {
            self.reverse_edge(edges.0);
            self.reverse_edge(edges.1);
        }
    }

    fn cross_link_dual_edge(&mut self, _node1: NodeId, _node2: NodeId) {
        unimplemented!("Cross-linking dual edges is not yet implemented")
    }
}

#[cfg(test)]
mod tests {
    use petgraph::stable_graph::EdgeIndex;

    use crate::prelude::TopoEdge;

    use super::*;

    #[test]
    fn test_topology_graph() {
        let mut topo_graph = TopologyGraph::new();

        let node_id1 = 1;
        let node_id2 = 2;

        let (added_node_id1_1, added_node_id1_2) = topo_graph.add_node(node_id1);
        let (added_node_id2_1, added_node_id2_2) = topo_graph.add_node(node_id2);

        assert_eq!(topo_graph.graph.node_count(), 4);

        let edge_id1 = 1;

        let topo_edge = TopoEdge {
            id: EdgeIndex::new(0),
            from: node_id1,
            to: node_id2,
            edge_id: edge_id1,
        };

        topo_graph
            .graph
            .add_edge(added_node_id1_1, added_node_id2_1, topo_edge.clone());
        topo_graph
            .graph
            .add_edge(added_node_id2_2, added_node_id1_2, topo_edge.clone());

        assert_eq!(topo_graph.graph.edge_count(), 2);

        assert_eq!(topo_graph.has_incoming(added_node_id2_1), true);
        assert_eq!(topo_graph.has_incoming(added_node_id1_1), false);

        assert_eq!(topo_graph.has_incoming(added_node_id1_2), true);
        assert_eq!(topo_graph.has_incoming(added_node_id2_2), false);

        topo_graph.reverse_dual_edge(node_id1, node_id2);

        assert_eq!(topo_graph.has_incoming(added_node_id1_1), true);
        assert_eq!(topo_graph.has_incoming(added_node_id2_1), false);

        assert_eq!(topo_graph.has_incoming(added_node_id2_2), true);
        assert_eq!(topo_graph.has_incoming(added_node_id1_2), false);
    }
}
