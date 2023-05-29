//! Edge length functions for `TransitEdge`.
use geo::{CoordFloat, EuclideanLength, HaversineLength};
use num_traits::FromPrimitive;
use std::iter::Sum;

use crate::core::TransitEdge;

/// EdgeLength trait provides the length of an element.
/// It is designed to work with types that implement the `CoordFloat`, `FromPrimitive`, and `Sum` traits.
pub trait EdgeLength<T: CoordFloat + FromPrimitive + Sum> {
    /// Returns the Euclidean length of the element.
    fn length(&self) -> T;
}

/// EdgeLength trait implementation for `TransitEdge`.
/// Returns the Euclidean length of the `TransitEdge`.
impl<T: CoordFloat + FromPrimitive + Sum> EdgeLength<T> for TransitEdge<T> {
    fn length(&self) -> T {
        self.euclidean_length()
    }
}

/// EuclideanLength trait implementation for `TransitEdge`.
/// Returns the Euclidean length of the `TransitEdge`.
impl<T: CoordFloat + FromPrimitive + Sum> EuclideanLength<T> for TransitEdge<T> {
    fn euclidean_length(&self) -> T {
        self.path.euclidean_length()
    }
}

/// HaversineLength trait implementation for `TransitEdge`.
/// Returns the Haversine (great-circle) length of the `TransitEdge`.
impl<T: CoordFloat + FromPrimitive + Sum> HaversineLength<T> for TransitEdge<T> {
    fn haversine_length(&self) -> T {
        self.path.haversine_length()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::NodeId;
    use geo::{LineString, Point};

    #[test]
    fn test_edge_length() {
        let line = LineString::from(vec![(0.0, 0.0), (1.0, 1.0)]);
        let edge = TransitEdge {
            id: 1,
            source: 1,
            target: 2,
            path: line.clone(),
        };

        assert_eq!(edge.length(), (2f64).sqrt());
    }

    #[test]
    fn test_euclidean_length() {
        let line = LineString::from(vec![(0.0, 0.0), (1.0, 1.0)]);
        let edge = TransitEdge {
            id: 1,
            source: 1,
            target: 2,
            path: line.clone(),
        };

        assert_eq!(edge.euclidean_length(), (2f64).sqrt());
    }

    #[test]
    fn test_haversine_length() {
        let line = LineString::from(vec![(-179.9, 0.0), (179.9, 0.0)]);
        let edge = TransitEdge {
            id: 1,
            source: 1,
            target: 2,
            path: line.clone(),
        };

        let approx_circumference = 2.0 * std::f64::consts::PI * 6371.0 * 1000.0; // Approx. Earth radius in m
        let expected_length = approx_circumference * (0.2 / 360.0); // 0.2 degrees out of 360 degrees
        assert!((edge.haversine_length() - expected_length).abs() < 1.0); // Allow 1 km error
    }
}
