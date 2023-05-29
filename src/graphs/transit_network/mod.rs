use super::{PhysicalGraph, TopologyGraph};
use crate::{
    core::{Accessability, NodeId, TransitEdge, TransitNode},
    operations::TransitNetworkModifier,
};
use geo::CoordNum;

pub mod repair;

/// Represents a transit network as a graph with transit nodes and edges.
///
/// The struct holds a physical graph and a topological graph which are lower-level representations of the network.
/// `TransitNetwork` provides a higher-level interface to the physical graph and topological graph.
///
/// The struct implements `TransitNetworkModifier` trait for modifying the underlying physical graph.
///
/// # Generics
///
/// `R`: Copyable trait bound. This represents the type of the data associated with the network's routes.
/// `T`: This represents the type of the coordinates used in the network. It's expected to implement `CoordNum` trait.
///
/// # Fields
///
/// * `physical_graph: PhysicalGraph<R, T>` - The physical graph representing the transit network.
/// * `topology_graph: TopologyGraph` - The topological graph representing the transit network.
pub struct TransitNetwork<R: Copy, T: CoordNum> {
    /// The physical graph representing the transit network.
    pub physical_graph: PhysicalGraph<R, T>,
    /// The topological graph representing the transit network.
    pub topology_graph: TopologyGraph,
}

impl<R: Copy, T: CoordNum> TransitNetwork<R, T> {
    /// Constructs a new `TransitNetwork` with an empty `PhysicalGraph` and `TopologyGraph`.
    ///
    /// # Returns
    ///
    /// A new `TransitNetwork` instance.
    pub fn new() -> Self {
        TransitNetwork {
            physical_graph: PhysicalGraph::new(),
            topology_graph: TopologyGraph::new(),
        }
    }
}

impl<R: Copy, T: CoordNum> Default for TransitNetwork<R, T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Implementation of `TransitNetworkModifier` trait for `TransitNetwork`.
///
/// This implementation delegates the operations to the underlying physical graph.
impl<R: Copy, T: CoordNum> TransitNetworkModifier<R, T> for TransitNetwork<R, T> {
    /// Adds a `TransitNode` to the physical graph of the network.
    ///
    /// # Arguments
    ///
    /// * `node` - The `TransitNode` to be added to the network.
    ///
    /// # Returns
    ///
    /// * `NodeId` - The ID of the added node.
    fn add_node(&mut self, node: TransitNode<R>) -> NodeId {
        let node_id = node.id;
        self.physical_graph.add_transit_node(node);
        self.topology_graph.add_node(node_id);
        node_id
    }

    /// Adds a `TransitEdge` to the physical graph of the network.
    ///
    /// # Arguments
    ///
    /// * `edge` - The `TransitEdge` to be added to the network.
    fn add_edge(&mut self, edge: TransitEdge<T>) {
        self.physical_graph.add_transit_edge(edge.clone());
        self.topology_graph
            .add_edge(edge.id, edge.source, edge.target);
    }

    fn add_edge_with_accessibility(&mut self, edge: TransitEdge<T>, accessability: Accessability) {
        self.physical_graph.add_transit_edge(edge.clone());
        self.topology_graph.add_edge_with_accessibility(
            edge.id,
            edge.source,
            edge.target,
            accessability,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use geo::{coord, point, LineString};
    use petgraph::visit::IntoEdgeReferences;

    #[test]
    fn test_transit_network() {
        // Create a new TransitNetwork
        let mut network = TransitNetwork::new();

        // Define some nodes
        let node1 = TransitNode {
            id: 1,
            location: point!(x: 0.0, y: 0.0),
        };

        let node2 = TransitNode {
            id: 2,
            location: point!(x: 1.0, y: 1.0),
        };

        // Add nodes to the network
        let node1_id = network.add_node(node1);
        let node2_id = network.add_node(node2);

        // Check that the nodes were added successfully
        assert_eq!(node1_id, 1);
        assert_eq!(node2_id, 2);

        // Define an edge
        let edge = TransitEdge {
            id: 1,
            source: 1,
            target: 2,
            path: LineString(vec![coord! {x: 0.0, y: 0.0}, coord! {x: 1.0, y: 1.0}]),
        };

        // Add edge to the network
        network.add_edge(edge);

        // Check that the edge was added successfully
        assert_eq!(network.physical_graph.graph.edge_count(), 1);

        // Check that the topology graph was populated correctly
        assert_eq!(network.topology_graph.graph.node_count(), 4);
        assert_eq!(network.topology_graph.graph.edge_count(), 2);
    }

    #[test]
    fn test_transit_network_edge_addition() {
        // Create a new TransitNetwork
        let mut network = TransitNetwork::new();

        // Define some nodes
        let node1 = TransitNode {
            id: 1,
            location: point!(x: 0.0, y: 0.0),
        };

        let node2 = TransitNode {
            id: 2,
            location: point!(x: 1.0, y: 1.0),
        };

        let node3 = TransitNode {
            id: 3,
            location: point!(x: 2.0, y: 2.0),
        };

        // Add nodes to the network
        network.add_node(node1);
        network.add_node(node2);
        network.add_node(node3);

        // Define edges
        let edge1 = TransitEdge {
            id: 1,
            source: 1,
            target: 2,
            path: LineString(vec![coord! {x: 0.0, y: 0.0}, coord! {x: 1.0, y: 1.0}]),
        };

        let edge2 = TransitEdge {
            id: 2,
            source: 2,
            target: 3,
            path: LineString(vec![coord! {x: 0.0, y: 0.0}, coord! {x: 2.0, y: 2.0}]),
        };

        // Add edges to the network
        network.add_edge(edge1);
        network.add_edge(edge2);

        // Check that the edges were added successfully
        assert_eq!(network.physical_graph.graph.edge_count(), 2);

        // Check that the topology graph was populated correctly
        assert_eq!(network.topology_graph.graph.node_count(), 6);
        assert_eq!(network.topology_graph.graph.edge_count(), 4);

        // Check that the topology edges were computed correctly
        let edge_ids: Vec<_> = network
            .topology_graph
            .graph
            .edge_references()
            .map(|edge| edge.weight().edge_id)
            .collect();
        assert_eq!(edge_ids, vec![1, 1, 2, 2]);
    }

    #[test]
    fn test_add_edge_with_accessibility() {
        // Create a new TransitNetwork

        let mut network = TransitNetwork::new();

        // Define some nodes
        let node0 = TransitNode {
            id: 0,
            location: point!(x: 0.0, y: 0.0),
        };

        let node1 = TransitNode {
            id: 1,
            location: point!(x: 1.0, y: 1.0),
        };

        let node2 = TransitNode {
            id: 2,
            location: point!(x: 2.0, y: 2.0),
        };

        let node3 = TransitNode {
            id: 3,
            location: point!(x: 3.0, y: 3.0),
        };

        let node4 = TransitNode {
            id: 4,
            location: point!(x: 4.0, y: 4.0),
        };

        // Add nodes to the network
        network.add_node(node0);
        network.add_node(node1);
        network.add_node(node2);
        network.add_node(node3);
        network.add_node(node4);

        // Define edges
        let edge01 = TransitEdge {
            id: 1,
            source: 0,
            target: 1,
            path: LineString(vec![coord! {x: 0.0, y: 0.0}, coord! {x: 1.0, y: 1.0}]),
        };

        let edge14 = TransitEdge {
            id: 2,
            source: 1,
            target: 4,
            path: LineString(vec![coord! {x: 1.0, y: 1.0}, coord! {x: 4.0, y: 4.0}]),
        };

        let edge12 = TransitEdge {
            id: 3,
            source: 1,
            target: 2,
            path: LineString(vec![coord! {x: 1.0, y: 1.0}, coord! {x: 2.0, y: 2.0}]),
        };

        let edge13 = TransitEdge {
            id: 4,
            source: 1,
            target: 3,
            path: LineString(vec![coord! {x: 1.0, y: 1.0}, coord! {x: 3.0, y: 3.0}]),
        };

        // Add edges to the network
        network.add_edge(edge01);
        network.add_edge(edge14);
        network.add_edge(edge12);
        network.add_edge(edge13);

        // Add edge with accessibility
        let edge40 = TransitEdge {
            id: 5,
            source: 4,
            target: 0,
            path: LineString(vec![coord! {x: 4.0, y: 4.0}, coord! {x: 0.0, y: 0.0}]),
        };

        network.add_edge_with_accessibility(edge40, Accessability::ReachableNodes(vec![2, 3]));

        // Check that the edges were added successfully
        assert_eq!(network.physical_graph.graph.edge_count(), 5);

        // Check that the topology graph was populated correctly
        assert_eq!(network.topology_graph.graph.node_count(), 10);
        assert_eq!(network.topology_graph.graph.edge_count(), 10);

        // Check that the topology edges were computed correctly
        let edge_ids: Vec<_> = network
            .topology_graph
            .graph
            .edge_references()
            .map(|edge| edge.weight().edge_id)
            .collect();
        assert_eq!(edge_ids, vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);
    }

    #[test]
    fn test_default() {
        let network: TransitNetwork<u32, f64> = TransitNetwork::default();

        assert_eq!(network.physical_graph.graph.node_count(), 0);
        assert_eq!(network.physical_graph.graph.edge_count(), 0);
        assert_eq!(network.topology_graph.graph.node_count(), 0);
        assert_eq!(network.topology_graph.graph.edge_count(), 0);
    }
}
