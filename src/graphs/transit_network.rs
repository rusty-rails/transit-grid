use super::PhysicalGraph;
use crate::{
    core::{NodeId, TransitEdge, TransitNode},
    operations::TransitNetworkModifier,
};
use geo::CoordNum;

/// `TransitNetwork` represents a transit network as a graph with transit nodes and edges.
///
/// The struct holds a physical graph which is a lower-level representation of the network.
/// The `TransitNetwork` provides a higher-level interface to the physical graph.
///
/// The struct implements `TransitNetworkModifier` trait for modifying the underlying physical graph.
pub struct TransitNetwork<R, T: CoordNum> {
    pub physical_graph: PhysicalGraph<R, T>,
}

/// Implementation of `TransitNetworkModifier` trait for `TransitNetwork`.
///
/// This implementation delegates the operations to the underlying physical graph.
impl<R, T: CoordNum> TransitNetworkModifier<R, T> for TransitNetwork<R, T> {
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
        self.physical_graph.add_transit_node(node)
    }

    /// Adds a `TransitEdge` to the physical graph of the network.
    ///
    /// # Arguments
    ///
    /// * `edge` - The `TransitEdge` to be added to the network.
    fn add_edge(&mut self, edge: TransitEdge<T>) {
        self.physical_graph.add_transit_edge(edge);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use geo::{coord, point, LineString};

    #[test]
    fn test_transit_network() {
        // Create a new TransitNetwork
        let mut network = TransitNetwork {
            physical_graph: PhysicalGraph::new(),
        };

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
        assert_eq!(node1_id, 0);
        assert_eq!(node2_id, 1);

        // Define an edge
        let edge = TransitEdge {
            id: 1,
            from: node1_id,
            to: node2_id,
            path: LineString(vec![coord! {x: 0.0, y: 0.0}, coord! {x: 1.0, y: 1.0}]),
        };

        // Add edge to the network
        network.add_edge(edge);

        // Check that the edge was added successfully
        assert_eq!(network.physical_graph.graph.edge_count(), 1);
    }
}
