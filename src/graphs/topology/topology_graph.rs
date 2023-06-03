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
#[derive(Debug, Clone)]
pub struct TopologyGraph {
    /// the inner graph
    pub graph: StableDiGraph<TopoNode, TopoEdge, u32>,
    id_to_index: HashMap<NodeId, (NodeIndex, NodeIndex)>,
    index_to_id: HashMap<NodeIndex, NodeId>,
}

impl TopologyGraph {
    /// Creates a new instance of `TopologyGraph`.
    pub fn new() -> Self {
        TopologyGraph {
            graph: StableDiGraph::<TopoNode, TopoEdge, u32>::new(),
            id_to_index: HashMap::new(),
            index_to_id: HashMap::new(),
        }
    }

    /// Returns the `NodeId` corresponding to a given `NodeIndex`.
    ///
    /// This method is useful when you have the index of a node in the graph and you want to retrieve its identifier.
    ///
    /// # Arguments
    ///
    /// * `index` - The `NodeIndex` of the node.
    ///
    /// # Returns
    ///
    /// * `NodeId` - The identifier of the node corresponding to the input index.
    ///
    /// # Panics
    ///
    /// This function will panic if the `NodeIndex` does not exist in the graph.
    pub fn index_to_id(&self, index: NodeIndex) -> NodeId {
        self.index_to_id[&index]
    }

    /// Returns the `NodeIndex` corresponding to a given `NodeId`.
    ///
    /// This method is useful when you have the identifier of a node and you want to retrieve its index in the graph.
    /// As each `NodeId` maps to two `TopoNode`s in the graph, this function returns a tuple of `NodeIndex`.
    ///
    /// # Arguments
    ///
    /// * `id` - The `NodeId` of the node.
    ///
    /// # Returns
    ///
    /// * A tuple of two `NodeIndex` values corresponding to the two `TopoNode`s for the input `NodeId`.
    ///
    /// # Panics
    ///
    /// This function will panic if the `NodeId` does not exist in the graph.
    pub fn id_to_index(&self, id: NodeId) -> (NodeIndex, NodeIndex) {
        self.id_to_index[&id]
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

        self.id_to_index
            .insert(node_id, (topo_node1_id, topo_node2_id));

        self.index_to_id.insert(topo_node1_id, node_id);
        self.index_to_id.insert(topo_node2_id, node_id);
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
            *self.id_to_index.get(&from_node_id).unwrap();
        let (to_topo_node_id1, to_topo_node_id2) = *self.id_to_index.get(&to_node_id).unwrap();

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

        let from_node_id: NodeId = *self.index_to_id.get(&from_topo_node_id).unwrap();
        let to_node_id: NodeId = *self.index_to_id.get(&to_topo_node_id).unwrap();

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
        let topo_node_ids = self.id_to_index.get(&node_id)?;
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
        let node_id = self.index_to_id.get(&topo_node_id)?;
        let topo_node_ids = self.id_to_index.get(node_id)?;
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
    /// The function will panic if it's unable to add an edge with the provided accessibility. This might occur if it cannot find nodes with the desired edge accessability or if the respective `TopoNode`s for the given nodes cannot be found.
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
                let from_node_id = *self.index_to_id.get(&u1).unwrap();
                let to_node_id = *self.index_to_id.get(&v1).unwrap();

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
    pub fn repair_edge(&mut self, node1: NodeId, node2: NodeId) {
        if let Some((edge_index1, edge_index2)) = self.find_edge_indices(node1, node2) {
            if !self.edge_is_in_neighbors_direction(edge_index1)
                && !self.edge_is_in_neighbors_direction(edge_index2)
            {
                self.reverse_edge(edge_index1);
                self.reverse_edge(edge_index2);
            }
        }
    }

    /// Reverse the direction of a given edge.
    ///
    /// # Arguments
    ///
    /// * `edge_index` - The index of the edge to reverse.
    ///
    /// # Panics
    ///
    /// This function will panic if the edge does not exist in the graph.
    pub fn reverse_edge(&mut self, edge_index: EdgeIndex) {
        let (source, target) = self.graph.edge_endpoints(edge_index).unwrap();
        let weight = self.graph.edge_weight(edge_index).unwrap().clone();
        self.graph.remove_edge(edge_index);
        self.graph.add_edge(target, source, weight);
    }

    /// Returns the indices of edges between two nodes in all directions.
    ///
    /// # Arguments
    ///
    /// * `node1_id` - The ID of the first node.
    /// * `node2_id` - The ID of the second node.
    ///
    /// # Returns
    ///
    /// * `Option<(EdgeIndex, EdgeIndex)>` - The indices of the two edges between the nodes, if they exist.
    ///
    /// # Panics
    ///
    /// This function will panic if the nodes do not exist in the graph.
    pub fn find_edge_indices(
        &self,
        node1_id: NodeId,
        node2_id: NodeId,
    ) -> Option<(EdgeIndex, EdgeIndex)> {
        let (node1_index1, node1_index2) = self.id_to_index(node1_id);
        let (node2_index1, node2_index2) = self.id_to_index(node2_id);

        let mut edges = Vec::new();

        for &source_index in &[node1_index1, node1_index2] {
            for &target_index in &[node2_index1, node2_index2] {
                if let Some(edge) = self.graph.find_edge(source_index, target_index) {
                    edges.push(edge);
                }
                if let Some(edge) = self.graph.find_edge(target_index, source_index) {
                    edges.push(edge);
                }
            }
        }

        if edges.len() == 2 {
            Some((edges[0], edges[1]))
        } else {
            None
        }
    }

    /// Checks if an edge is in the same direction as its neighboring edges.
    ///
    /// # Arguments
    ///
    /// * `edge_index` - The `EdgeIndex` of the edge to check.
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if the edge is in the same direction as its neighboring edges, `false` otherwise.
    ///
    /// # Panics
    ///
    /// This function will panic if the `EdgeIndex` does not exist in the graph.
    pub fn edge_is_in_neighbors_direction(&self, edge_index: EdgeIndex) -> bool {
        let (source, target) = self.graph.edge_endpoints(edge_index).unwrap();

        // Check if the source's neighbors are in the same direction
        let source_has_same_direction = self
            .graph
            .neighbors_directed(source, petgraph::Direction::Incoming)
            .any(|neighbor| neighbor != target);

        let target_has_same_direction = self
            .graph
            .neighbors_directed(target, petgraph::Direction::Outgoing)
            .any(|neighbor| neighbor != source);

        source_has_same_direction && target_has_same_direction
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
        println!("{}", Dot::new(&topo_graph.graph));

        let edge_indices = topo_graph.find_edge_indices(1, 2);
        assert_eq!(edge_indices.is_some(), true);
        assert_eq!(edge_indices.unwrap().0, EdgeIndex::new(0));
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

        let node_idx0 = topo_graph.id_to_index.get(&node_ids[0]).unwrap().clone();
        let node_idx1 = topo_graph.id_to_index.get(&node_ids[1]).unwrap().clone();
        let node_idx2 = topo_graph.id_to_index.get(&node_ids[2]).unwrap().clone();
        let node_idx3 = topo_graph.id_to_index.get(&node_ids[3]).unwrap().clone();

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

    #[test]
    fn test_default() {
        let topo_graph = TopologyGraph::default();

        // Ensure that the graph is empty
        assert_eq!(topo_graph.graph.node_count(), 0);
        assert_eq!(topo_graph.graph.edge_count(), 0);
        assert_eq!(topo_graph.id_to_index.len(), 0);
        assert_eq!(topo_graph.index_to_id.len(), 0);
    }

    #[test]
    fn test_reverse_edge() {
        let mut topo_graph = TopologyGraph::default();
        let node1 = topo_graph.add_node(1);
        let node2 = topo_graph.add_node(2);
        let edge_index = topo_graph.add_edge(33, 1, 2);

        // Ensure that the edge is initially from node1 to node2
        assert_eq!(
            topo_graph.graph.edge_endpoints(edge_index.0),
            Some((node1.0, node2.0))
        );

        topo_graph.reverse_edge(edge_index.0);

        assert_eq!(
            topo_graph.graph.edge_endpoints(edge_index.0),
            Some((node2.0, node1.0))
        );
    }

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
}
