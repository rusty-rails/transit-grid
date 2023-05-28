//! This module provides basic structures for representing a transit network.
//! It provides `TransitNode` and `TransitEdge` structures, along with ID types for them.
//! The `TransitNode` represents a node in the transit network, while the `TransitEdge` represents a connection between two nodes.
//! The module also provides `Accessability`, an enum for representing the accessibility of nodes in the network.

use geo::{CoordNum, LineString};

mod accessability;
/// Re-export of the `Accessability` enum from the `accessability` module.
pub use accessability::Accessability;

/// Type alias for an identifier.
pub type IdType = u32;

/// Type alias for a node identifier.
pub type NodeId = IdType;

/// Type alias for an edge identifier.
pub type EdgeId = IdType;

/// Structure representing a node in the transit network.
///
/// Each node has a unique identifier and a location.
/// The location type `T` is generic and can be any type that implements the `Copy` trait.
///
/// # Examples
///
/// ```
/// use geo::coord;
/// use transit_grid::core::TransitNode;
///
/// // GPS coordinates for London, UK: 51.5074 N, 0.1278 W
/// let node = TransitNode {
///     id: 1,
///     location: coord! { x: -0.1278, y: 51.5074 },
/// };
/// assert_eq!(node.id, 1);
/// assert_eq!(node.location, coord! { x: -0.1278, y: 51.5074 });
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TransitNode<T> {
    /// A unique identifier for the node.
    pub id: NodeId,

    /// The location of the node, represented by a generic type `T`.
    pub location: T,
}

/// Structure representing a connection between two `TransitNode` instances.
///
/// Each edge has a unique identifier and a path represented as a `LineString`.
/// The `LineString` type `T` is generic and can be any type that implements the `CoordNum` trait.
///
/// # Examples
///
/// ```
/// use geo::{coord, LineString};
/// use transit_grid::core::TransitEdge;
///
/// let edge = TransitEdge {
///     id: 1,
///     from: 1,
///     to: 2,
///     path: LineString(vec![coord! { x: 0.0, y: 0.0 }, coord! { x: 1.0, y: 1.0 }]),
/// };
/// assert_eq!(edge.id, 1);
/// assert_eq!(edge.from, 1);
/// assert_eq!(edge.to, 2);
/// assert_eq!(edge.path, LineString(vec![coord! { x: 0.0, y: 0.0 }, coord! { x: 1.0, y: 1.0 }]));
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TransitEdge<T: CoordNum> {
    /// A unique identifier for the edge.
    pub id: EdgeId,

    /// The identifier of the node where the edge starts.
    pub from: NodeId,

    /// The identifier of the node where the edge ends.
    pub to: NodeId,

    /// The path of the edge, represented as a `LineString`.
    pub path: LineString<T>,
}

impl<T: CoordNum> Default for TransitEdge<T> {
    fn default() -> Self {
        Self {
            id: 0,
            from: 0,
            to: 0,
            path: LineString(vec![]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use geo::coord;

    #[test]
    fn test_node() {
        let node = TransitNode {
            id: 1,
            location: coord! { x:0.0, y:0.0},
        };
        assert_eq!(node.id, 1);
        assert_eq!(node.location, coord! { x:0.0, y:0.0});
    }

    #[test]
    fn test_edge() {
        let edge = TransitEdge {
            id: 1,
            from: 1,
            to: 2,
            path: LineString(vec![coord! { x:0.0, y:0.0}, coord! { x:1.0, y:1.0}]),
        };
        assert_eq!(edge.id, 1);
        assert_eq!(edge.from, 1);
        assert_eq!(edge.to, 2);
        assert_eq!(
            edge.path,
            LineString(vec![coord! { x:0.0, y:0.0}, coord! { x:1.0, y:1.0}])
        );
    }

    #[test]
    fn test_edge_default() {
        let edge = TransitEdge::<f64>::default();
        assert_eq!(edge.id, 0);
        assert_eq!(edge.from, 0);
        assert_eq!(edge.to, 0);
        assert_eq!(edge.path, LineString::<f64>(vec![]));
    }
}
