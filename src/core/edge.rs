use geo::{Coord, CoordNum, LineString};
use serde::{Deserialize, Serialize};

use super::{IdType, NodeId};

/// Type alias for an edge identifier.
pub type EdgeId = IdType;

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
///     source: 1,
///     target: 2,
///     length: 1.0,
///     path: LineString(vec![coord! { x: 0.0, y: 0.0 }, coord! { x: 1.0, y: 1.0 }]),
/// };
/// assert_eq!(edge.id, 1);
/// assert_eq!(edge.source, 1);
/// assert_eq!(edge.target, 2);
/// assert_eq!(edge.path, LineString(vec![coord! { x: 0.0, y: 0.0 }, coord! { x: 1.0, y: 1.0 }]));
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TransitEdge<T: CoordNum> {
    /// A unique identifier for the edge.
    pub id: EdgeId,

    /// The identifier of the node where the edge starts.
    pub source: NodeId,

    /// The identifier of the node where the edge ends.
    pub target: NodeId,

    /// The length of the edge.
    pub length: T,

    /// The path of the edge, represented as a `LineString`.
    pub path: LineString<T>,
}

impl<T: CoordNum> Default for TransitEdge<T> {
    fn default() -> Self {
        Self {
            id: 0,
            source: 0,
            target: 0,
            length: T::zero(),
            path: LineString(vec![]),
        }
    }
}

/// Trait providing a way to get the coordinates of the source and target nodes of a path.
///
/// `PathCoordinates` can be implemented by any type that has a source and target coordinate.
/// This is useful in graph algorithms where you need to know the start and end point of an edge.
///
pub trait PathCoordinates<T: CoordNum> {
    /// Returns the source coordinate of the path.
    fn source_coordinate(&self) -> Coord<T>;
    /// Returns the target coordinate of the path.
    fn target_coordinate(&self) -> Coord<T>;
}

impl<T: CoordNum> PathCoordinates<T> for TransitEdge<T> {
    fn source_coordinate(&self) -> Coord<T> {
        self.path.points().next().unwrap().0
    }

    fn target_coordinate(&self) -> Coord<T> {
        self.path.points().last().unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use geo::coord;

    #[test]
    fn test_edge() {
        let edge = TransitEdge {
            id: 1,
            source: 1,
            target: 2,
            length: 1.0,
            path: LineString(vec![coord! { x:0.0, y:0.0}, coord! { x:1.0, y:1.0}]),
        };
        assert_eq!(edge.id, 1);
        assert_eq!(edge.source, 1);
        assert_eq!(edge.target, 2);
        assert_eq!(edge.length, 1.0);
        assert_eq!(
            edge.path,
            LineString(vec![coord! { x:0.0, y:0.0}, coord! { x:1.0, y:1.0}])
        );
    }

    #[test]
    fn test_edge_default() {
        let edge = TransitEdge::<f64>::default();
        assert_eq!(edge.id, 0);
        assert_eq!(edge.source, 0);
        assert_eq!(edge.target, 0);
        assert_eq!(edge.path, LineString::<f64>(vec![]));
    }

    #[test]
    fn test_edge_coordinates() {
        let edge = TransitEdge {
            id: 1,
            source: 1,
            target: 2,
            length: 1.0,
            path: LineString(vec![coord! { x: 0.0, y: 0.0 }, coord! { x: 1.0, y: 1.0 }]),
        };
        assert_eq!(edge.source_coordinate(), coord! { x: 0.0, y: 0.0 });
        assert_eq!(edge.target_coordinate(), coord! { x: 1.0, y: 1.0 });
    }
}
