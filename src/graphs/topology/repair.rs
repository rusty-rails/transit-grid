use crate::core::NodeId;

use super::TopologyGraph;

/// `TopologyGraphRepairer` provides functionality to manipulate and repair edges in a topological graph.
///
/// It provides methods for reversing dual edges and cross-linking dual edges.
pub trait TopologyGraphRepairer {
    /// Repairs the direction of edges in a graph if they are incorrectly directed.
    ///
    /// This function repairs edges between two nodes in the graph by examining their direction.
    /// If the edges have the same direction (either both outgoing or both incoming), the direction
    /// of the edges will be switched to ensure a consistent direction from `node1` to `node2`.
    ///
    /// # Examples
    /// Correct scenarios:
    /// a -> node1_indices.0 -> node2_indices.0 -> b
    /// a -> node1_indices.0 -> node2_indices.1 -> b
    /// a -> node1_indices.1 -> node2_indices.0 -> b
    ///
    /// Incorrect scenarios:
    /// a -> node1_indices.0 <- node2_indices.0 -> b
    /// a -> node1_indices.1 <- node2_indices.0 -> b
    /// a -> node1_indices.0 <- node2_indices.1 -> b
    ///
    /// In the incorrect scenarios, the function will correct the edge directions as:
    /// a -> node1_indices.0 -> node2_indices.0 -> b
    /// a -> node1_indices.1 -> node2_indices.0 -> b
    /// a -> node1_indices.0 -> node2_indices.1 -> b
    ///
    /// # Arguments
    /// * `node1`: The first node of the edge pair.
    /// * `node2`: The second node of the edge pair.
    ///
    /// # Panics
    /// This function will panic if either of the node indices is not present in the graph.
    ///
    /// # Note
    /// This function is mainly intended to be used for directed graphs. Using it for undirected graphs
    /// may not have the intended effect.
    ///
    /// This function should be used when a graph's edge directions are set manually and may be incorrect,
    /// and when it's important that the edges have a specific direction for the logic of the application.
    fn repair_edge(&mut self, node1: NodeId, node2: NodeId);

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
    fn repair_edge(&mut self, node1: NodeId, node2: NodeId) {
        if let Some((edge_index1, edge_index2)) = self.find_edge_indices(node1, node2) {
            if !self.edge_is_in_neighbors_direction(edge_index1)
                && !self.edge_is_in_neighbors_direction(edge_index2)
            {
                self.reverse_dual_edge(node1, node2);
            }
        }
        if let Some((edge_index1, edge_index2)) = self.find_edge_indices(node1, node2) {
            if !self.edge_is_in_neighbors_direction(edge_index1)
                && !self.edge_is_in_neighbors_direction(edge_index2)
            {
                self.cross_link_dual_edge(node1, node2);
            }
        }
    }

    fn reverse_dual_edge(&mut self, node1: NodeId, node2: NodeId) {
        if let Some(edges) = self.find_edge_indices(node1, node2) {
            self.reverse_edge(edges.0);
            self.reverse_edge(edges.1);
        }
    }

    fn cross_link_dual_edge(&mut self, node1: NodeId, node2: NodeId) {
        if let Some(edges) = self.find_edge_indices(node1, node2) {
            let (source1, target1) = self.graph.edge_endpoints(edges.0).unwrap();
            let (source2, target2) = self.graph.edge_endpoints(edges.1).unwrap();

            let weight1 = self.graph.edge_weight(edges.0).unwrap().clone();
            let weight2 = self.graph.edge_weight(edges.1).unwrap().clone();

            self.graph.remove_edge(edges.0);
            self.graph.remove_edge(edges.1);

            self.graph.add_edge(source1, source2, weight1);
            self.graph.add_edge(target1, target2, weight2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::TopoEdge;
    use petgraph::{dot::Dot, stable_graph::EdgeIndex};

    #[test]
    fn test_repair_edge() {
        let mut topo_graph = TopologyGraph::new();

        let node_id_a = 1;
        let node_id_b = 2;
        let node_id_c = 3;
        let node_id_d = 4;

        topo_graph.add_node(node_id_a);
        topo_graph.add_node(node_id_b);
        topo_graph.add_node(node_id_c);
        topo_graph.add_node(node_id_d);

        let _edge31 = topo_graph.add_edge(31, 1, 2);
        let edge32 = topo_graph.add_edge(32, 2, 3);
        let _edge33 = topo_graph.add_edge(33, 3, 4);

        println!("{:?}", Dot::new(&topo_graph.graph));

        assert_ne!(true, topo_graph.edge_is_in_neighbors_direction(edge32.0));
        assert_ne!(true, topo_graph.edge_is_in_neighbors_direction(edge32.1));

        topo_graph.repair_edge(node_id_b, node_id_c);

        println!("{:?}", Dot::new(&topo_graph.graph));

        assert!(topo_graph.edge_is_in_neighbors_direction(edge32.0));
        assert!(topo_graph.edge_is_in_neighbors_direction(edge32.1));
    }

    #[test]
    fn test_reverse_dual_edge() {
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

    #[test]
    fn test_cross_link_dual_edge() {
        let mut topo_graph = TopologyGraph::new();

        let node_id1 = 1;
        let node_id2 = 2;

        let (added_node_id1_1, added_node_id1_2) = topo_graph.add_node(node_id1);
        let (added_node_id2_1, added_node_id2_2) = topo_graph.add_node(node_id2);

        assert_eq!(topo_graph.graph.node_count(), 4);

        let edge_id1 = 1;

        topo_graph.add_edge(edge_id1, node_id1, node_id2);

        assert_eq!(topo_graph.graph.edge_count(), 2);

        assert_eq!(topo_graph.has_incoming(added_node_id1_1), false);
        assert_eq!(topo_graph.has_incoming(added_node_id2_1), true);

        assert_eq!(topo_graph.has_incoming(added_node_id1_2), true);
        assert_eq!(topo_graph.has_incoming(added_node_id2_2), false);

        assert!(topo_graph
            .graph
            .find_edge(added_node_id1_1, added_node_id2_1)
            .is_some());

        topo_graph.cross_link_dual_edge(node_id1, node_id2);

        assert_eq!(topo_graph.has_incoming(added_node_id1_1), false);
        assert_eq!(topo_graph.has_incoming(added_node_id2_1), false);

        assert_eq!(topo_graph.has_incoming(added_node_id1_2), true);
        assert_eq!(topo_graph.has_incoming(added_node_id2_2), true);

        assert!(topo_graph
            .graph
            .find_edge(added_node_id1_1, added_node_id2_1)
            .is_none());
        assert!(topo_graph
            .graph
            .find_edge(added_node_id2_1, added_node_id1_2)
            .is_some());
        assert!(topo_graph
            .graph
            .find_edge(added_node_id1_1, added_node_id2_2)
            .is_some());
    }
}
