//! 🔀️ Planar path boolean operations (optional `booleans` feature).

use crate::engine::{DrawingError, PathSegment};
use geo::{BooleanOps, Coord, LineString, MultiPolygon, Polygon};

fn close_polygon(coords: &mut Vec<Coord<f64>>) -> Result<Polygon<f64>, DrawingError> {
    if coords.len() < 3 {
        return Err(DrawingError::InvalidInput("boolean input needs a closed polygon".into()));
    }
    if coords.first() != coords.last() {
        coords.push(coords[0]);
    }
    Ok(Polygon::new(LineString(std::mem::take(coords)), vec![]))
}

fn segments_to_multipolygon(segments: &[PathSegment]) -> Result<MultiPolygon<f64>, DrawingError> {
    let mut polygons = Vec::new();
    let mut coords = Vec::new();
    for segment in segments {
        match segment {
            PathSegment::Move { to } => {
                if !coords.is_empty() {
                    polygons.push(close_polygon(&mut coords)?);
                }
                coords.push(Coord { x: to[0], y: to[1] });
            }
            PathSegment::Line { to } => coords.push(Coord { x: to[0], y: to[1] }),
            PathSegment::Close if !coords.is_empty() => {
                polygons.push(close_polygon(&mut coords)?);
            }
            _ => {}
        }
    }
    if !coords.is_empty() {
        polygons.push(close_polygon(&mut coords)?);
    }
    if polygons.is_empty() {
        return Err(DrawingError::InvalidInput("boolean input needs a closed polygon".into()));
    }
    Ok(MultiPolygon::new(polygons))
}

fn ring_to_segments(ring: &LineString<f64>) -> Vec<PathSegment> {
    let mut segments = Vec::new();
    let mut first = true;
    for coord in &ring.0 {
        let point = [coord.x, coord.y];
        if first {
            segments.push(PathSegment::Move { to: point });
            first = false;
        } else {
            segments.push(PathSegment::Line { to: point });
        }
    }
    if !segments.is_empty() {
        segments.push(PathSegment::Close);
    }
    segments
}

fn polygon_to_segments(polygon: &Polygon<f64>) -> Vec<PathSegment> {
    let mut segments = ring_to_segments(polygon.exterior());
    for interior in polygon.interiors() {
        segments.extend(ring_to_segments(interior));
    }
    segments
}

pub fn boolean_paths(a: &[PathSegment], b: &[PathSegment], operation: &str) -> Result<Vec<PathSegment>, DrawingError> {
    let poly_a = segments_to_multipolygon(a)?;
    let poly_b = segments_to_multipolygon(b)?;
    let result = match operation {
        "union" => poly_a.union(&poly_b),
        "difference" => poly_a.difference(&poly_b),
        "intersection" => poly_a.intersection(&poly_b),
        "xor" => poly_a.xor(&poly_b),
        _ => return Err(DrawingError::InvalidInput(format!("unknown boolean operation: {operation}"))),
    };
    let mut segments = Vec::new();
    for polygon in result {
        segments.extend(polygon_to_segments(&polygon));
    }
    if segments.is_empty() {
        return Err(DrawingError::Operation("boolean produced empty path".into()));
    }
    Ok(segments)
}

pub fn boolean_paths_many(inputs: &[Vec<PathSegment>], operation: &str) -> Result<Vec<PathSegment>, DrawingError> {
    if inputs.is_empty() {
        return Err(DrawingError::InvalidInput("boolean operation needs at least one path".into()));
    }
    let mut acc = segments_to_multipolygon(&inputs[0])?;
    for next in inputs.iter().skip(1) {
        let poly_b = segments_to_multipolygon(next)?;
        acc = match operation {
            "union" => acc.union(&poly_b),
            "difference" => acc.difference(&poly_b),
            "intersection" => acc.intersection(&poly_b),
            "xor" => acc.xor(&poly_b),
            _ => return Err(DrawingError::InvalidInput(format!("unknown boolean operation: {operation}"))),
        };
    }
    let mut segments = Vec::new();
    for polygon in acc.into_iter() {
        segments.extend(polygon_to_segments(&polygon));
    }
    if segments.is_empty() {
        return Err(DrawingError::Operation("boolean produced empty path".into()));
    }
    Ok(segments)
}

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn square(origin: [f64; 2], size: f64) -> Vec<PathSegment> {
        vec![PathSegment::Move { to: origin }, PathSegment::Line { to: [origin[0] + size, origin[1]] }, PathSegment::Line { to: [origin[0] + size, origin[1] + size] }, PathSegment::Line { to: [origin[0], origin[1] + size] }, PathSegment::Close]
    }

    #[test]
    fn union_overlapping_rects() {
        let merged = boolean_paths(&square([0.0, 0.0], 10.0), &square([5.0, 5.0], 10.0), "union").expect("union");
        assert!(!merged.is_empty());
    }

    #[test]
    fn union_preserves_disconnected_input_contours() {
        let mut disconnected = square([20.0, 0.0], 5.0);
        disconnected.extend(square([30.0, 0.0], 5.0));
        let merged = boolean_paths(&square([0.0, 0.0], 5.0), &disconnected, "union").expect("union");
        assert_eq!(merged.iter().filter(|segment| matches!(segment, PathSegment::Move { .. })).count(), 3);
    }

    #[test]
    fn close_polygon_errors_on_too_few_points() {
        let open = vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Line { to: [1.0, 1.0] }];
        let err = boolean_paths(&open, &square([0.0, 0.0], 5.0), "union").unwrap_err();
        assert!(matches!(err, DrawingError::InvalidInput(_)));
    }

    #[test]
    fn boolean_paths_errors_on_unknown_operation() {
        let err = boolean_paths(&square([0.0, 0.0], 5.0), &square([1.0, 1.0], 5.0), "bogus").unwrap_err();
        assert!(matches!(err, DrawingError::InvalidInput(message) if message.contains("unknown boolean operation")));
    }

    #[test]
    fn boolean_paths_intersection_of_disjoint_shapes_errors_on_empty_result() {
        let err = boolean_paths(&square([0.0, 0.0], 5.0), &square([100.0, 100.0], 5.0), "intersection").unwrap_err();
        assert!(matches!(err, DrawingError::Operation(message) if message.contains("empty path")));
    }

    #[test]
    fn boolean_paths_many_errors_on_empty_inputs() {
        let err = boolean_paths_many(&[], "union").unwrap_err();
        assert!(matches!(err, DrawingError::InvalidInput(_)));
    }

    #[test]
    fn boolean_paths_many_computes_running_operation_across_three_inputs() {
        let inputs = vec![square([0.0, 0.0], 10.0), square([0.0, 0.0], 10.0), square([0.0, 0.0], 10.0)];
        let merged = boolean_paths_many(&inputs, "intersection").expect("intersection");
        assert!(!merged.is_empty());
    }
}
// #endregion 🧪️Tests
