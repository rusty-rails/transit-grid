use std::collections::{HashMap, HashSet};

use petgraph::{
    stable_graph::{EdgeIndex, NodeIndex, StableDiGraph},
    visit::EdgeRef,
    Direction,
};

use crate::core::{Accessability, EdgeId, NodeId};

use super::{TopoEdge, TopoNode};

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
            node_id,
        };
        let topo_node1_id = self.graph.add_node(topo_node1);
        self.graph.node_weight_mut(topo_node1_id).unwrap().id = topo_node1_id;

        let topo_node2 = TopoNode {
            id: NodeIndex::default(), // Temporary value; will be updated
            node_id,
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

        let from_node_id: NodeId = *self.toponode_to_node.get(&from_topo_node_id).unwrap();
        let to_node_id: NodeId = *self.toponode_to_node.get(&to_topo_node_id).unwrap();

        let topo_edge1 = TopoEdge {
            id: EdgeIndex::new(0), // Temporary value; will be updated
            from: from_node_id,
            to: to_node_id,
            edge_id,
        };
        let topo_edge1_id = self
            .graph
            .add_edge(from_topo_node_id, to_topo_node_id, topo_edge1);
        self.graph.edge_weight_mut(topo_edge1_id).unwrap().id = topo_edge1_id;

        let from_topo_node_id = self.get_other_toponode(from_topo_node_id).unwrap();
        let to_topo_node_id = self.get_other_toponode(to_topo_node_id).unwrap();

        let topo_edge2 = TopoEdge {
            id: EdgeIndex::new(0), // Temporary value; will be updated
            from: to_node_id,
            to: from_node_id,
            edge_id,
        };
        let topo_edge2_id = self
            .graph
            .add_edge(to_topo_node_id, from_topo_node_id, topo_edge2);
        self.graph.edge_weight_mut(topo_edge2_id).unwrap().id = topo_edge2_id;

        (topo_edge1_id, topo_edge2_id)
    }

    /// Checks if there are no edges in the specified direction leading to any of the nodes in the neighbors list.
    ///
    /// # Arguments
    ///
    /// * `topo_node_id` - The `NodeIndex` of the node to check.
    /// * `neighbors` - A vector of `NodeId` that the node should not have edges towards in the given direction.
    /// * `dir` - The direction of the edges to check (Outgoing or Incoming).
    ///
    /// # Returns
    ///
    /// * `bool` - True if none of the neighbors have an edge in the given direction to the node, otherwise false.
    pub fn no_edges_in_direction(
        &self,
        topo_node_id: NodeIndex,
        neighbors: Vec<NodeId>,
        dir: Direction,
    ) -> bool {
        // Convert the neighbors Vec into a HashSet for faster lookup
        let neighbors_set: HashSet<_> = neighbors.into_iter().collect();

        // Check for each neighbor of the node
        for edge in self.graph.edges_directed(topo_node_id, dir) {
            if neighbors_set.contains(&self.graph[edge.target()].node_id) {
                // If any edge in the given direction leads to a node in the neighbors list, return false
                return false;
            }
        }

        // If we've gone through all edges and none lead to a node in the neighbors list, return true
        true
    }

    /// Returns the `NodeIndex` of the `NodeId` that does not have any edge in the opposite direction leading to any node in `neighbors`.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The ID of the node to check.
    /// * `neighbors` - A vector of `NodeId`s to check.
    /// * `dir` - The `Direction` in which to check the edges.
    ///
    /// # Returns
    ///
    /// * `Option<NodeIndex>` - The `NodeIndex` of the `NodeId` if it does not have any edge in the opposite direction leading to nodes in `neighbors`, otherwise `None`.
    pub fn find_node_index_with_edges(
        &self,
        node_id: NodeId,
        neighbors: Vec<NodeId>,
        dir: Direction,
    ) -> Option<NodeIndex> {
        let topo_node_ids = self.node_to_toponode.get(&node_id)?;
        if self.no_edges_in_direction(topo_node_ids.0, neighbors.clone(), dir.opposite()) {
            return Some(topo_node_ids.0);
        }
        if self.no_edges_in_direction(topo_node_ids.1, neighbors, dir.opposite()) {
            return Some(topo_node_ids.1);
        }
        None
    }

    /// Returns the `NodeIndex` of the other `TopoNode` for a given `TopoNode`.
    ///
    /// # Arguments
    ///
    /// * `topo_node_id` - The `NodeIndex` of the `TopoNode`.
    ///
    /// # Returns
    ///
    /// * `Option<NodeIndex>` - The `NodeIndex` of the other `TopoNode` for the given `TopoNode`, if it exists.
    pub fn get_other_toponode(&self, topo_node_id: NodeIndex) -> Option<NodeIndex> {
        let node_id = self.toponode_to_node.get(&topo_node_id)?;
        let topo_node_ids = self.node_to_toponode.get(node_id)?;
        if topo_node_ids.0 == topo_node_id {
            Some(topo_node_ids.1)
        } else if topo_node_ids.1 == topo_node_id {
            Some(topo_node_ids.0)
        } else {
            None
        }
    }

    /// Adds an edge with a certain accessibility into the graph.
    ///
    /// # Arguments
    ///
    /// * `edge_id` - The identifier of the edge that should be added.
    /// * `from_node_id` - The identifier of the node where the edge should start.
    /// * `to_node_id` - The identifier of the node where the edge should end.
    /// * `accessability` - The type of accessability of the edge. This can be either `ReachableNodes` or `UnreachableNodes`.
    ///
    /// # Returns
    ///
    /// A tuple of `EdgeIndex` values that were assigned to the newly created edges.
    ///
    /// # Panics
    ///
    /// The function will panic if it's unable to add an edge with the provided accessibility.
    pub fn add_edge_with_accessibility(
        &mut self,
        edge_id: EdgeId,
        from_node_id: NodeId,
        to_node_id: NodeId,
        accessability: Accessability,
    ) -> (EdgeIndex, EdgeIndex) {
        let direction = match &accessability {
            Accessability::ReachableNodes(_) => (Direction::Incoming, Direction::Outgoing),
            Accessability::UnreachableNodes(_) => (Direction::Outgoing, Direction::Incoming),
        };

        let nodes = match &accessability {
            Accessability::ReachableNodes(nodes) => nodes,
            Accessability::UnreachableNodes(nodes) => nodes,
        };

        let u1 = self.find_node_index_with_edges(from_node_id, nodes.clone(), direction.0);
        let v1 = self.find_node_index_with_edges(to_node_id, nodes.clone(), direction.1);

        if let (Some(u1), Some(v1)) = (u1, v1) {
            let u2 = self.get_other_toponode(u1);
            let v2 = self.get_other_toponode(v1);

            if let (Some(u2), Some(v2)) = (u2, v2) {
                let from_node_id = *self.toponode_to_node.get(&u1).unwrap();
                let to_node_id = *self.toponode_to_node.get(&v1).unwrap();

                let topo_edge1 = TopoEdge {
                    id: EdgeIndex::new(0), // Temporary value; will be updated
                    from: from_node_id,
                    to: to_node_id,
                    edge_id,
                };
                let topo_edge1_id = self.graph.add_edge(u1, v1, topo_edge1);
                self.graph.edge_weight_mut(topo_edge1_id).unwrap().id = topo_edge1_id;

                let topo_edge2 = TopoEdge {
                    id: EdgeIndex::new(0), // Temporary value; will be updated
                    from: to_node_id,
                    to: from_node_id,
                    edge_id,
                };
                let topo_edge2_id = self.graph.add_edge(v2, u2, topo_edge2);
                self.graph.edge_weight_mut(topo_edge2_id).unwrap().id = topo_edge2_id;
                return (topo_edge1_id, topo_edge2_id);
            }
        }

        unreachable!("Could not add edge with accessibility");
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

impl Default for TopologyGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use petgraph::dot::Dot;

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
            .add_edge(added_node_id2_2, added_node_id1_2, topo_edge);

        assert_eq!(topo_graph.graph.edge_count(), 2);

        // Test if has_incoming works as expected
        assert_eq!(topo_graph.has_incoming(added_node_id2_1), true);
        assert_eq!(topo_graph.has_incoming(added_node_id1_1), false);

        assert_eq!(topo_graph.has_incoming(added_node_id1_2), true);
        assert_eq!(topo_graph.has_incoming(added_node_id2_2), false);
    }

    #[test]
    fn test_no_edges_in_direction() {
        // Create a new TopologyGraph
        let mut graph = TopologyGraph::new();

        // Define some nodes
        let node1 = 1;
        let node2 = 2;
        let node3 = 3;

        // Add nodes to the graph
        let node1_id = graph.add_node(node1);
        let node2_id = graph.add_node(node2);
        let _node3_id = graph.add_node(node3);

        // Add edges to the graph
        let edge_id1 = 1;
        let edge_id2 = 2;
        graph.add_edge(edge_id1, node1, node2);
        graph.add_edge(edge_id2, node1, node3);

        // Check that there are outgoing edges from node1_id.0 to node2
        assert_eq!(
            graph.no_edges_in_direction(node1_id.0, vec![node2], Direction::Outgoing),
            false
        );

        // Check that there are outgoing edges from node1_id.0 to node2 and node3
        assert_eq!(
            graph.no_edges_in_direction(node1_id.0, vec![node2, node3], Direction::Outgoing),
            false
        );

        // Check that there are no outgoing edges from node2_id.1 to node1
        assert_eq!(
            graph.no_edges_in_direction(node2_id.0, vec![node1], Direction::Outgoing),
            true
        );
    }

    #[test]
    fn test_find_node_index_with_edges() {
        // Create a new TopologyGraph
        let mut graph = TopologyGraph::new();

        // Define some nodes and edges
        let node1 = 1;
        let node2 = 2;
        let node3 = 3;
        let node4 = 4;

        // Add nodes and edges to the graph
        let topo_node1 = graph.add_node(node1);
        let topo_node2 = graph.add_node(node2);
        let topo_node3 = graph.add_node(node3);
        let topo_node4 = graph.add_node(node4);

        graph.add_edge(1, node1, node2);
        graph.add_edge(2, node1, node3);
        graph.add_edge(3, node2, node3);
        graph.add_edge(4, node3, node4);

        // Check if the function works as expected
        assert_eq!(
            graph.find_node_index_with_edges(node1, vec![node2, node3], Direction::Outgoing),
            Some(topo_node1.0)
        );
        assert_eq!(
            graph.find_node_index_with_edges(node2, vec![node1, node3], Direction::Incoming),
            Some(topo_node2.0)
        );
        assert_eq!(
            graph.find_node_index_with_edges(node3, vec![node2, node4], Direction::Outgoing),
            Some(topo_node3.0)
        );
        assert_eq!(
            graph.find_node_index_with_edges(node4, vec![node3], Direction::Incoming),
            Some(topo_node4.0)
        );
    }

    #[test]
    fn test_get_other_toponode() {
        let mut topo_graph = TopologyGraph::new();

        // Add some nodes to the graph
        let node_id1: NodeId = 1;
        let node_id2: NodeId = 2;

        let (topo_node_id1_1, topo_node_id1_2) = topo_graph.add_node(node_id1);
        let (topo_node_id2_1, topo_node_id2_2) = topo_graph.add_node(node_id2);

        // Assert that get_other_toponode returns the correct other TopoNode
        assert_eq!(
            topo_graph.get_other_toponode(topo_node_id1_1),
            Some(topo_node_id1_2)
        );
        assert_eq!(
            topo_graph.get_other_toponode(topo_node_id1_2),
            Some(topo_node_id1_1)
        );

        assert_eq!(
            topo_graph.get_other_toponode(topo_node_id2_1),
            Some(topo_node_id2_2)
        );
        assert_eq!(
            topo_graph.get_other_toponode(topo_node_id2_2),
            Some(topo_node_id2_1)
        );

        // For non-existing NodeIndex, the function should return None
        assert_eq!(topo_graph.get_other_toponode(NodeIndex::new(100)), None);
    }

    #[test]
    fn test_add_edge_with_accessibility() {
        let mut topo_graph = TopologyGraph::new();

        // Add some nodes to the graph
        let node_id1 = 1;
        let node_id2 = 2;
        topo_graph.add_node(node_id1);
        topo_graph.add_node(node_id2);

        // Add an edge with accessibility
        let edge_id = 1;
        let accessability = Accessability::ReachableNodes(vec![node_id1, node_id2]);
        let (edge_index1, edge_index2) =
            topo_graph.add_edge_with_accessibility(edge_id, node_id1, node_id2, accessability);

        // Assert that the edge has been added correctly
        assert!(topo_graph.graph.edge_weight(edge_index1).is_some());
        assert!(topo_graph.graph.edge_weight(edge_index2).is_some());
    }

    #[test]
    fn test_add_edge_with_accessibility_scenario_reachable() {
        let mut topo_graph = TopologyGraph::new();

        // Add nodes to the graph
        let node_ids: Vec<NodeId> = (0..5).collect();
        for node_id in &node_ids {
            topo_graph.add_node(*node_id);
        }

        // Add edge from 4 to 0
        let edge_id = 1;
        let accessability = Accessability::ReachableNodes(vec![node_ids[0]]);
        topo_graph.add_edge_with_accessibility(edge_id, node_ids[4], node_ids[0], accessability);

        // Add edges from 1 to 2 and 1 to 3
        let edge_id = 2;
        let accessability = Accessability::ReachableNodes(vec![]);
        topo_graph.add_edge_with_accessibility(
            edge_id,
            node_ids[1],
            node_ids[2],
            accessability.clone(),
        );
        let edge_id = 3;
        topo_graph.add_edge_with_accessibility(edge_id, node_ids[1], node_ids[3], accessability);

        // Add edge from 0 to 1
        let edge_id = 4;
        let accessability =
            Accessability::ReachableNodes(vec![node_ids[4], node_ids[2], node_ids[3]]);

        topo_graph.add_edge_with_accessibility(edge_id, node_ids[0], node_ids[1], accessability);

        // Assert that all edges have been added correctly
        for i in 1..=4 {
            let edge_index1 = EdgeIndex::new(i * 2 - 2);
            let edge_index2 = EdgeIndex::new(i * 2 - 1);
            assert!(topo_graph.graph.edge_weight(edge_index1).is_some());
            assert!(topo_graph.graph.edge_weight(edge_index2).is_some());
        }
    }

    #[test]
    fn test_add_edge_with_accessibility_scenario_unreachable() {
        let mut topo_graph = TopologyGraph::new();

        // Add nodes to the graph
        let node_ids: Vec<NodeId> = (0..4).collect();
        for node_id in &node_ids {
            topo_graph.add_node(*node_id);
        }

        let node_idx0 = topo_graph
            .node_to_toponode
            .get(&node_ids[0])
            .unwrap()
            .clone();
        let node_idx1 = topo_graph
            .node_to_toponode
            .get(&node_ids[1])
            .unwrap()
            .clone();
        let node_idx2 = topo_graph
            .node_to_toponode
            .get(&node_ids[2])
            .unwrap()
            .clone();
        let node_idx3 = topo_graph
            .node_to_toponode
            .get(&node_ids[3])
            .unwrap()
            .clone();

        let topo_edge = TopoEdge {
            id: EdgeIndex::new(0),
            from: node_ids[0],
            to: node_ids[1],
            edge_id: 1,
        };

        // Add edge from 0 to 1
        topo_graph
            .graph
            .add_edge(node_idx0.0, node_idx1.0, topo_edge.clone());
        topo_graph
            .graph
            .add_edge(node_idx1.1, node_idx0.1, topo_edge.clone());

        let topo_edge = TopoEdge {
            id: EdgeIndex::new(0),
            from: node_ids[1],
            to: node_ids[2],
            edge_id: 2,
        };

        topo_graph
            .graph
            .add_edge(node_idx1.0, node_idx2.0, topo_edge.clone());
        topo_graph
            .graph
            .add_edge(node_idx2.1, node_idx1.1, topo_edge.clone());

        // Add edge from 1 to 3
        let edge_id = 3;
        let accessability = Accessability::UnreachableNodes(vec![node_ids[2]]);

        topo_graph.add_edge_with_accessibility(
            edge_id,
            node_ids[1].clone(),
            node_ids[3].clone(),
            accessability,
        );

        println!("{}", Dot::new(&topo_graph.graph));

        assert!(
            topo_graph
                .graph
                .find_edge(node_idx1.0, node_idx3.0)
                .is_some()
                || topo_graph
                    .graph
                    .find_edge(node_idx1.0, node_idx3.1)
                    .is_some()
        );

        // Assert that all edges have been added correctly
        for i in 1..=3 {
            let edge_index1 = EdgeIndex::new(i * 2 - 2);
            let edge_index2 = EdgeIndex::new(i * 2 - 1);
            assert!(topo_graph.graph.edge_weight(edge_index1).is_some());
            assert!(topo_graph.graph.edge_weight(edge_index2).is_some());
        }
    }
}
