use geo::CoordNum;

use crate::{core::NodeId, prelude::TransitNetworkRepairer};

use super::TransitNetwork;

impl<R: Copy + geo::EuclideanDistance<T, geo::Coord<T>>, T: CoordNum> TransitNetworkRepairer<R, T>
    for TransitNetwork<R, T>
{
    fn repair_edge(&mut self, node1: crate::core::NodeId, node2: crate::core::NodeId) {
        self.physical_graph.repair_edge(node1, node2);
        self.topology_graph.repair_edge(node1, node2);
    }

    fn repair(&mut self) {
        let edges: Vec<(NodeId, NodeId)> = self
            .physical_graph
            .graph
            .edge_weights()
            .map(|edge| (edge.source, edge.target))
            .collect();

        for edge in edges {
            self.repair_edge(edge.0, edge.1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::{TransitEdge, TransitNode},
        operations::TransitNetworkModifier,
    };
    use geo::{coord, LineString};
    use petgraph::visit::IntoEdgeReferences;

    #[test]
    fn test_transit_network_edge_addition() {
        // Create a new TransitNetwork
        let mut network = TransitNetwork::new();

        // Define some nodes
        let node1 = TransitNode {
            id: 1,
            location: coord!(x: 0.0, y: 0.0),
        };

        let node2 = TransitNode {
            id: 2,
            location: coord!(x: 1.0, y: 1.0),
        };

        let node3 = TransitNode {
            id: 3,
            location: coord!(x: 2.0, y: 2.0),
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
            length: 1.0,
            path: LineString(vec![coord! {x: 0.0, y: 0.0}, coord! {x: 1.0, y: 1.0}]),
        };

        let edge2 = TransitEdge {
            id: 2,
            source: 2,
            target: 3,
            length: 1.0,
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

        network.repair();
    }
}
