use std::collections::HashMap;

use petgraph::stable_graph::{EdgeIndex, NodeIndex, StableDiGraph};

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

/// Represents the topological graph of the transit network.
///
/// Topological graph is directed and each node in the topological graph maps to a node in the physical graph.
/// This is particularly useful for scenarios such as rail switches where the directionality of edges matters.
pub struct TopologyGraph {
    pub graph: StableDiGraph<TopoNode, TopoEdge, u32>,
    node_to_toponode: HashMap<NodeId, (NodeIndex, NodeIndex)>,
    toponode_to_node: HashMap<NodeIndex, NodeId>,
}

impl TopologyGraph {
    /// Creates a new instance of `TopologyGraph`.
    pub fn new() -> Self {
        TopologyGraph {
            graph: StableDiGraph::<TopoNode, TopoEdge, u32>::new(),
            node_to_toponode: HashMap::new(),
            toponode_to_node: HashMap::new(),
        }
    }

    /// Adds a Node with a `NodeId` to the topological graph. This internally adds two `TopoNode`s to the graph.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The `NodeId` to be added to the graph.
    ///
    /// # Returns
    ///
    /// * A tuple of two `TopoNodeId`s corresponding to the two `TopoNode`s added for the input `NodeId`.
    pub fn add_node(&mut self, node_id: NodeId) -> (NodeIndex, NodeIndex) {
        let topo_node1 = TopoNode {
            id: NodeIndex::default(), // Temporary value; will be updated
            node_id: node_id,
        };
        let topo_node1_id = self.graph.add_node(topo_node1);
        self.graph.node_weight_mut(topo_node1_id).unwrap().id = topo_node1_id;

        let topo_node2 = TopoNode {
            id: NodeIndex::default(), // Temporary value; will be updated
            node_id: node_id,
        };
        let topo_node2_id = self.graph.add_node(topo_node2);
        self.graph.node_weight_mut(topo_node2_id).unwrap().id = topo_node2_id;

        self.node_to_toponode
            .insert(node_id, (topo_node1_id, topo_node2_id));

        self.toponode_to_node.insert(topo_node1_id, node_id);
        self.toponode_to_node.insert(topo_node2_id, node_id);
        (topo_node1_id, topo_node2_id)
    }

    /// Adds a `TopoEdge` to the topological graph.
    ///
    /// # Arguments
    ///
    /// * `edge_id` - The `EdgeId` to be added to the graph.
    /// * `from_node_id` - The `NodeId` from which the edge is originating.
    /// * `to_node_id` - The `NodeId` to which the edge is pointing.
    ///
    /// # Returns
    ///
    /// * `TopoEdgeId` - The ID of the added edge.
    pub fn add_edge(
        &mut self,
        edge_id: EdgeId,
        from_node_id: NodeId,
        to_node_id: NodeId,
    ) -> (EdgeIndex, EdgeIndex) {
        let (from_topo_node_id1, from_topo_node_id2) =
            *self.node_to_toponode.get(&from_node_id).unwrap();
        let (to_topo_node_id1, to_topo_node_id2) = *self.node_to_toponode.get(&to_node_id).unwrap();

        let from_topo_node_id = if self.has_incoming(from_topo_node_id1) {
            from_topo_node_id2
        } else {
            from_topo_node_id1
        };

        let to_topo_node_id = if self.has_incoming(to_topo_node_id1) {
            to_topo_node_id2
        } else {
            to_topo_node_id1
        };

        let from_node_id: NodeId = self
            .toponode_to_node
            .get(&from_topo_node_id)
            .unwrap()
            .clone();
        let to_node_id: NodeId = self.toponode_to_node.get(&to_topo_node_id).unwrap().clone();

        let topo_edge1 = TopoEdge {
            id: EdgeIndex::new(0), // Temporary value; will be updated
            from: from_node_id,
            to: to_node_id,
            edge_id: edge_id,
        };
        let topo_edge1_id = self
            .graph
            .add_edge(from_topo_node_id, to_topo_node_id, topo_edge1);
        self.graph.edge_weight_mut(topo_edge1_id).unwrap().id = topo_edge1_id;

        let topo_edge2 = TopoEdge {
            id: EdgeIndex::new(0), // Temporary value; will be updated
            from: to_node_id,
            to: from_node_id,
            edge_id: edge_id,
        };
        let topo_edge2_id = self
            .graph
            .add_edge(to_topo_node_id, from_topo_node_id, topo_edge2);
        self.graph.edge_weight_mut(topo_edge2_id).unwrap().id = topo_edge2_id;

        (topo_edge1_id, topo_edge2_id)
    }

    /// Checks if a node has an incoming edge in the topological graph.
    ///
    /// # Arguments
    ///
    /// * `node` - The ID of the node to check.
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if the node has at least one incoming edge, `false` otherwise.
    pub fn has_incoming(&self, node: NodeIndex) -> bool {
        self.graph
            .neighbors_directed(node, petgraph::Incoming)
            .next()
            .is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topology_graph() {
        let mut topo_graph = TopologyGraph::new();

        let node_id1 = 1;
        let node_id2 = 2;

        let (added_node_id1_1, added_node_id1_2) = topo_graph.add_node(node_id1);
        let (added_node_id2_1, added_node_id2_2) = topo_graph.add_node(node_id2);

        // Each call to add_node() adds two nodes, so the total node count should be 4.
        assert_eq!(topo_graph.graph.node_count(), 4);

        let edge_id1 = 1;
        topo_graph.add_edge(edge_id1, node_id1, node_id2);

        assert_eq!(topo_graph.graph.edge_count(), 2);

        // Test if has_incoming works as expected
        assert_eq!(topo_graph.has_incoming(added_node_id1_1), true);
        assert_eq!(topo_graph.has_incoming(added_node_id1_2), false);

        assert_eq!(topo_graph.has_incoming(added_node_id2_1), true);
        assert_eq!(topo_graph.has_incoming(added_node_id2_2), false);
    }
}
