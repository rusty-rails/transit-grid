use crate::core::{EdgeId, NodeId, TransitEdge, TransitNode};
use geo::CoordNum;
use petgraph::graph::{NodeIndex, UnGraph};

/// `PhysicalGraph` represents the physical layout of the transit network.
///
/// It is an undirected graph where each node represents a transit node
/// (a point in the transit network where a vehicle can stop) and each edge
/// represents a transit edge (a path between two transit nodes).
///
/// The graph is implemented using `petgraph`'s `Csr` structure.
pub struct PhysicalGraph<R, T: CoordNum> {
    pub graph: UnGraph<TransitNode<R>, TransitEdge<T>, u32>,
}

impl<R, T: CoordNum> PhysicalGraph<R, T> {
    /// Creates a new, empty `PhysicalGraph`.
    pub fn new() -> Self {
        PhysicalGraph {
            graph: UnGraph::<TransitNode<R>, TransitEdge<T>, u32>::new_undirected(),
        }
    }

    /// Adds a `TransitNode` to the `PhysicalGraph`.
    ///
    /// # Example
    /// ```
    /// use transit_grid::prelude::PhysicalGraph;
    /// use transit_grid::core::TransitNode;
    /// use geo::{coord, Coord};
    ///
    /// let mut graph: PhysicalGraph<Coord, f64> = PhysicalGraph::new();
    /// let node = TransitNode { id: 1, location: coord! { x:0.0, y:0.0 } };
    /// graph.add_transit_node(node);
    /// ```
    pub fn add_transit_node(&mut self, node: TransitNode<R>) -> NodeId {
        self.graph.add_node(node).index()
    }

    /// Adds a `TransitEdge` to the `PhysicalGraph`.
    ///
    /// # Example
    /// ```
    /// use transit_grid::prelude::PhysicalGraph;
    /// use transit_grid::core::{TransitNode, TransitEdge};
    /// use geo::{coord, Coord, LineString};
    /// use petgraph::csr::IndexType;
    ///
    /// let mut graph: PhysicalGraph<Coord, f64> = PhysicalGraph::new();
    /// let node1 = TransitNode { id: 1, location: coord! { x:0.0, y:0.0 } };
    /// let node2 = TransitNode { id: 2, location: coord! { x:1.0, y:1.0 } };
    ///
    /// let node1_id = graph.add_transit_node(node1);
    /// let node2_id = graph.add_transit_node(node2);
    ///
    /// let edge = TransitEdge {
    ///     id: 1,
    ///     from: node1_id.index(),
    ///     to: node2_id.index(),
    ///     path: LineString(vec![coord! { x:0.0, y:0.0 }, coord! { x:1.0, y:1.0 }]),
    /// };
    ///
    /// graph.add_transit_edge(edge);
    /// ```
    pub fn add_transit_edge(&mut self, edge: TransitEdge<T>) -> EdgeId {
        self.graph
            .add_edge(NodeIndex::new(edge.from), NodeIndex::new(edge.to), edge)
            .index()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use geo::{coord, LineString};
    use petgraph::stable_graph::IndexType;

    #[test]
    fn test_graph() {
        let mut graph = PhysicalGraph::new();

        let node1 = TransitNode {
            id: 1,
            location: coord! { x:0.0, y:0.0 },
        };

        let node2 = TransitNode {
            id: 2,
            location: coord! { x:1.0, y:1.0 },
        };

        let node1_id = graph.add_transit_node(node1);
        let node2_id = graph.add_transit_node(node2);

        let edge = TransitEdge {
            id: 1,
            from: node1_id.index(),
            to: node2_id.index(),
            path: LineString(vec![coord! { x:0.0, y:0.0 }, coord! { x:1.0, y:1.0 }]),
        };

        let _ = graph.add_transit_edge(edge);

        assert_eq!(graph.graph.node_count(), 2);
        assert_eq!(graph.graph.edge_count(), 1);
    }
}
