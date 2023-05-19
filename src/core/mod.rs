use geo::{LineString, CoordNum};

type NodeId = u64;
type EdgeId = u64;

/// `TransitNode` represents a node in the transit network. 
/// 
/// Each node is identified by a unique ID and has a location.
/// 
/// # Example
/// ```
/// use geo::coord;
/// use transit_grid::core::TransitNode;
/// 
/// let node = TransitNode {
///     id: 1,
///     location: coord! { x: 0.0, y: 0.0 },
/// };
/// assert_eq!(node.id, 1);
/// assert_eq!(node.location, coord! { x: 0.0, y: 0.0 });
/// ```
pub struct TransitNode<T> {
    pub id: NodeId,
    pub location: T,
}

/// `TransitEdge` represents a connection between two `TransitNode` instances.
/// 
/// Each edge is identified by a unique ID and has a path which is a `LineString`.
/// 
/// # Example
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
pub struct TransitEdge<T: CoordNum> {
    pub id: EdgeId,
    pub from: NodeId,
    pub to: NodeId,
    pub path: LineString<T>,
}

#[cfg(test)]
mod tests {
    use geo::coord;
    use super::*;

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
        assert_eq!(edge.path, LineString(vec![coord! { x:0.0, y:0.0}, coord! { x:1.0, y:1.0}]));
    }
}
