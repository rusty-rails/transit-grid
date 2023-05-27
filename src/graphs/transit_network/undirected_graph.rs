use crate::core::{EdgeId, NodeId, TransitEdge, TransitNode};
use crate::prelude::PhysicalGraph;
use geo::CoordNum;
use petgraph::Undirected;

use petgraph::graph::{EdgeIndex, NodeIndex, NodeIndices, UnGraph};
use petgraph::visit::{
    Data, EdgeCount, GraphBase, GraphRef, IntoEdgeReferences, IntoEdges, IntoNeighbors,
    IntoNodeIdentifiers, NodeCount, NodeIndexable, Visitable,
};

use super::TransitNetwork;

/// The `UndirectedGraph` struct represents an undirected graph for a transit network.
///
/// # Types
///
/// * `'a` - Lifetime associated with the transit network reference.
/// * `R` - Copyable type associated with the TransitNode.
/// * `T` - CoordNum type associated with the TransitEdge.
///
/// # Fields
///
/// * `network` - A reference to the transit network.
#[derive(Copy, Clone)]
pub struct UndirectedGraph<'a, R: std::marker::Copy, T: CoordNum> {
    network: &'a TransitNetwork<R, T>,
}

/// This implementation block contains functions associated with the `UndirectedGraph` struct.
impl<'a, R: std::marker::Copy, T: CoordNum> UndirectedGraph<'a, R, T> {
    /// Constructs a new `UndirectedGraph`.
    ///
    /// # Arguments
    ///
    /// * `network` - A reference to the transit network.
    ///
    /// # Returns
    ///
    /// A new `UndirectedGraph`.
    pub fn new(network: &'a TransitNetwork<R, T>) -> Self {
        UndirectedGraph { network }
    }
}

impl<'a, R: std::marker::Copy, T: CoordNum> GraphBase for UndirectedGraph<'a, R, T> {
    type NodeId = NodeIndex<NodeId>;
    type EdgeId = EdgeIndex<EdgeId>;
}

impl<'a, R: std::marker::Copy, T: CoordNum> GraphRef for UndirectedGraph<'a, R, T> {}

impl<'a, R: std::marker::Copy, T: CoordNum> Data for UndirectedGraph<'a, R, T> {
    type NodeWeight = TransitNode<R>;
    type EdgeWeight = TransitEdge<T>;
}

pub struct NodeIdentifierIterator<'a, R: Copy, T: CoordNum> {
    inner: NodeIndices<u32>,
    graph: &'a PhysicalGraph<R, T>,
}

impl<'a, R: Copy, T: CoordNum> Iterator for NodeIdentifierIterator<'a, R, T> {
    type Item = NodeIndex<NodeId>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(node_index) => Some(NodeIndex::new(
                self.graph.index_to_id(node_index).try_into().unwrap(),
            )),
            None => None,
        }
    }
}

impl<'a, R: std::marker::Copy, T: CoordNum> IntoNodeIdentifiers for UndirectedGraph<'a, R, T> {
    type NodeIdentifiers = NodeIdentifierIterator<'a, R, T>;

    fn node_identifiers(self) -> Self::NodeIdentifiers {
        NodeIdentifierIterator {
            inner: self.network.physical_graph.graph.node_indices(),
            graph: &self.network.physical_graph,
        }
    }
}

impl<'a, R: std::marker::Copy, T: CoordNum> IntoEdgeReferences for UndirectedGraph<'a, R, T> {
    type EdgeRef = petgraph::graph::EdgeReference<'a, TransitEdge<T>>;
    type EdgeReferences = petgraph::graph::EdgeReferences<'a, TransitEdge<T>>;

    fn edge_references(self) -> Self::EdgeReferences {
        (self.network.physical_graph.graph).edge_references()
    }
}

impl<'a, R: std::marker::Copy, T: CoordNum> Visitable for UndirectedGraph<'a, R, T> {
    type Map = <UnGraph<TransitNode<R>, TransitEdge<T>, u32> as Visitable>::Map;

    fn visit_map(&self) -> Self::Map {
        self.network.physical_graph.graph.visit_map()
    }

    fn reset_map(&self, map: &mut Self::Map) {
        self.network.physical_graph.graph.reset_map(map)
    }
}

impl<'a, R: std::marker::Copy, T: CoordNum> NodeIndexable for UndirectedGraph<'a, R, T> {
    fn node_bound(&self) -> usize {
        self.network.physical_graph.graph.node_bound()
    }

    fn to_index(&self, a: Self::NodeId) -> usize {
        let index = self
            .network
            .physical_graph
            .id_to_index(a.index().try_into().unwrap());
        self.network.physical_graph.graph.to_index(index)
    }

    fn from_index(&self, i: usize) -> Self::NodeId {
        let index = self.network.physical_graph.graph.from_index(i);
        NodeIndex::new(
            self.network
                .physical_graph
                .index_to_id(index)
                .try_into()
                .unwrap(),
        )
    }
}

impl<'a, R: std::marker::Copy, T: CoordNum> IntoEdges for UndirectedGraph<'a, R, T> {
    type Edges = petgraph::graph::Edges<'a, TransitEdge<T>, Undirected, u32>;

    fn edges(self, a: Self::NodeId) -> Self::Edges {
        let index = self
            .network
            .physical_graph
            .id_to_index(a.index().try_into().unwrap());
        self.network.physical_graph.graph.edges(index)
    }
}

impl<'a, R: std::marker::Copy, T: CoordNum> IntoNeighbors for UndirectedGraph<'a, R, T> {
    type Neighbors = petgraph::graph::Neighbors<'a, TransitEdge<T>>;
    //type Neighbors = petgraph::graph::Neighbors<'a, TopoEdge>;

    fn neighbors(self, n: Self::NodeId) -> Self::Neighbors {
        let index = self
            .network
            .physical_graph
            .id_to_index(n.index().try_into().unwrap());
        self.network.physical_graph.graph.neighbors(index)
    }
}

impl<'a, R: std::marker::Copy, T: CoordNum> NodeCount for UndirectedGraph<'a, R, T> {
    fn node_count(&self) -> usize {
        self.network.physical_graph.graph.node_count()
    }
}

impl<'a, R: std::marker::Copy, T: CoordNum> EdgeCount for UndirectedGraph<'a, R, T> {
    fn edge_count(&self) -> usize {
        self.network.physical_graph.graph.edge_count()
    }
}

#[cfg(test)]
mod tests {
    use geo::{coord, point, LineString};

    use crate::prelude::TransitNetworkModifier;

    use super::*;

    #[test]
    fn test_dijkstra() {
        let mut network = TransitNetwork::new();

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

        network.add_node(node1);
        network.add_node(node2);
        network.add_node(node3);

        let edge1 = TransitEdge {
            id: 1,
            from: 0,
            to: 1,
            path: LineString(vec![coord! {x: 0.0, y: 0.0}, coord! {x: 1.0, y: 1.0}]),
        };

        let edge2 = TransitEdge {
            id: 2,
            from: 1,
            to: 2,
            path: LineString(vec![coord! {x: 0.0, y: 0.0}, coord! {x: 2.0, y: 2.0}]),
        };

        network.add_edge(edge1);
        network.add_edge(edge2);
        let undirected_graph = UndirectedGraph::new(&network);

        let target_node = NodeIndex::new(2);
        let path = petgraph::algo::dijkstra(
            &undirected_graph,
            NodeIndex::new(0),
            Some(target_node),
            |_e| 1,
        );
        println!("{:?}", path);
        assert_eq!(path[&target_node], 3);
    }
}
