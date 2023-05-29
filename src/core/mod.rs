//! This module provides basic structures for representing a transit network.
//! It provides `TransitNode` and `TransitEdge` structures, along with ID types for them.
//! The `TransitNode` represents a node in the transit network, while the `TransitEdge` represents a connection between two nodes.
//! The module also provides `Accessability`, an enum for representing the accessibility of nodes in the network.

mod edge;
pub use edge::{EdgeId, PathCoordinates, TransitEdge};

mod accessability;
/// Re-export of the `Accessability` enum from the `accessability` module.
pub use accessability::Accessability;

/// Type alias for an identifier.
pub type IdType = u64;

/// Type alias for a node identifier.
pub type NodeId = IdType;

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
}
