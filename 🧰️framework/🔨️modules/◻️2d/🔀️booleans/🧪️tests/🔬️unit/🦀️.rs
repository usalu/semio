
use super::*;

fn square(origin: [f64; 2], size: f64) -> Vec<PathSegment> {
    vec![PathSegment::Move { to: origin }, PathSegment::Line { to: [origin[0] + size, origin[1]] }, PathSegment::Line { to: [origin[0] + size, origin[1] + size] }, PathSegment::Line { to: [origin[0], origin[1] + size] }, PathSegment::Close]
}

fn reversed(path: Vec<PathSegment>) -> Vec<PathSegment> {
    let mut points = path
        .into_iter()
        .filter_map(|segment| match segment {
            PathSegment::Move { to } | PathSegment::Line { to } => Some(to),
            _ => None,
        })
        .collect::<Vec<_>>();
    points.reverse();
    let first = points.remove(0);
    let mut result = vec![PathSegment::Move { to: first }];
    result.extend(points.into_iter().map(|to| PathSegment::Line { to }));
    result.push(PathSegment::Close);
    result
}

fn contours(path: &[PathSegment]) -> usize {
    path.iter().filter(|segment| matches!(segment, PathSegment::Move { .. })).count()
}

fn absolute_ring_area(path: &[PathSegment]) -> f64 {
    segments_to_region(path).expect("valid result").rings.iter().map(|ring| signed_area(ring).abs()).sum()
}

#[test]
fn oracle_operations_match_overlapping_rectangles() {
    let a = square([0.0, 0.0], 10.0);
    let b = square([5.0, 5.0], 10.0);
    let union = boolean_paths(&a, &b, "union").expect("union");
    assert_eq!(absolute_ring_area(&union), 175.0);
    assert_eq!(
        union,
        vec![
            PathSegment::Move { to: [0.0, 0.0] },
            PathSegment::Line { to: [0.0, 10.0] },
            PathSegment::Line { to: [5.0, 10.0] },
            PathSegment::Line { to: [5.0, 15.0] },
            PathSegment::Line { to: [15.0, 15.0] },
            PathSegment::Line { to: [15.0, 5.0] },
            PathSegment::Line { to: [10.0, 5.0] },
            PathSegment::Line { to: [10.0, 0.0] },
            PathSegment::Line { to: [0.0, 0.0] },
            PathSegment::Close,
        ]
    );
    assert_eq!(absolute_ring_area(&boolean_paths(&a, &b, "intersection").expect("intersection")), 25.0);
    assert_eq!(absolute_ring_area(&boolean_paths(&a, &b, "difference").expect("difference")), 75.0);
    assert_eq!(contours(&boolean_paths(&a, &b, "xor").expect("xor")), 2);
}

#[test]
fn oracle_preserves_disjoint_and_touching_topology() {
    let a = square([0.0, 0.0], 5.0);
    assert_eq!(contours(&boolean_paths(&a, &square([10.0, 0.0], 5.0), "union").expect("disjoint")), 2);
    assert_eq!(contours(&boolean_paths(&a, &square([5.0, 0.0], 5.0), "union").expect("edge touch")), 1);
    assert_eq!(contours(&boolean_paths(&a, &square([5.0, 5.0], 5.0), "union").expect("vertex touch")), 2);
    assert!(matches!(boolean_paths(&a, &square([10.0, 0.0], 5.0), "intersection"), Err(DrawingError::Operation(_))));
}

#[test]
fn oracle_emits_hole_for_contained_difference_and_xor() {
    let outer = square([0.0, 0.0], 10.0);
    let inner = square([2.0, 2.0], 4.0);
    assert_eq!(contours(&boolean_paths(&outer, &inner, "difference").expect("difference")), 2);
    assert_eq!(contours(&boolean_paths(&outer, &inner, "xor").expect("xor")), 2);
    let mut two_outers = square([0.0, 0.0], 10.0);
    two_outers.extend(square([20.0, 0.0], 10.0));
    let mut two_inners = square([2.0, 2.0], 4.0);
    two_inners.extend(square([22.0, 2.0], 4.0));
    let result = boolean_paths(&two_outers, &two_inners, "difference").expect("two polygons with holes");
    let moves = result
        .iter()
        .filter_map(|segment| match segment {
            PathSegment::Move { to } => Some(*to),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(moves, vec![[0.0, 0.0], [2.0, 6.0], [20.0, 0.0], [22.0, 6.0]]);
}

#[test]
fn oracle_is_winding_independent_and_deterministic() {
    let a = square([0.0, 0.0], 10.0);
    let b = square([5.0, 5.0], 10.0);
    let expected = boolean_paths(&a, &b, "union").expect("union");
    assert_eq!(boolean_paths(&reversed(a), &reversed(b), "union").expect("reversed"), expected);
    assert_eq!(boolean_paths(&square([0.0, 0.0], 10.0), &square([5.0, 5.0], 10.0), "union").expect("repeat"), expected);
}

#[test]
fn oracle_discards_degenerate_and_duplicate_edges() {
    let degenerate = vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Line { to: [5.0, 0.0] }, PathSegment::Line { to: [10.0, 0.0] }, PathSegment::Close];
    let result = boolean_paths(&degenerate, &square([20.0, 0.0], 2.0), "union").expect("union");
    assert_eq!(contours(&result), 1);
    assert_eq!(absolute_ring_area(&result), 4.0);
    let duplicate = vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Line { to: [5.0, 0.0] }, PathSegment::Line { to: [5.0, 0.0] }, PathSegment::Line { to: [5.0, 5.0] }, PathSegment::Line { to: [0.0, 5.0] }, PathSegment::Close];
    assert_eq!(absolute_ring_area(&boolean_paths(&duplicate, &square([10.0, 0.0], 2.0), "union").expect("duplicate")), 29.0);
    assert_eq!(absolute_ring_area(&boolean_paths(&square([0.0, 0.0], 10.0), &square([5.0, 0.0], 10.0), "intersection").expect("collinear overlap")), 50.0);
}

#[test]
fn translated_coordinates_preserve_topology() {
    let origin = 1.0e12;
    let union = boolean_paths(&square([origin, origin], 10.0), &square([origin + 5.0, origin + 5.0], 10.0), "union").expect("translated union");
    assert_eq!(contours(&union), 1);
    assert_eq!(union.len(), 10);
}

#[test]
fn self_crossing_even_odd_contour_is_regularized() {
    let bow_tie = vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Line { to: [10.0, 10.0] }, PathSegment::Line { to: [0.0, 10.0] }, PathSegment::Line { to: [10.0, 0.0] }, PathSegment::Close];
    let result = boolean_paths(&bow_tie, &square([20.0, 0.0], 2.0), "union").expect("self-crossing union");
    assert_eq!(contours(&result), 3);
    assert_eq!(absolute_ring_area(&result), 54.0);
}

#[test]
fn oracle_many_operations_keep_intermediate_holes() {
    let inputs = vec![square([0.0, 0.0], 10.0), square([2.0, 2.0], 6.0), square([4.0, -2.0], 2.0)];
    assert_eq!(contours(&boolean_paths_many(&inputs, "difference").expect("difference")), 2);
    assert!(matches!(boolean_paths_many(&inputs, "intersection"), Err(DrawingError::Operation(_))));
    let recovered = boolean_paths_many(&[square([0.0, 0.0], 10.0), square([0.0, 0.0], 10.0), square([20.0, 0.0], 2.0)], "xor").expect("xor recovers from an empty intermediate result");
    assert_eq!(absolute_ring_area(&recovered), 4.0);
}

#[test]
fn errors_match_public_contract() {
    let open = vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Line { to: [1.0, 1.0] }];
    assert!(matches!(boolean_paths(&open, &square([0.0, 0.0], 5.0), "union"), Err(DrawingError::InvalidInput(_))));
    assert!(matches!(boolean_paths(&square([0.0, 0.0], 5.0), &square([1.0, 1.0], 5.0), "bogus"), Err(DrawingError::InvalidInput(message)) if message.contains("unknown boolean operation")));
    assert!(matches!(boolean_paths_many(&[], "union"), Err(DrawingError::InvalidInput(_))));
    assert!(matches!(boolean_paths(&square([0.0, 0.0], 5.0), &square([100.0, 100.0], 5.0), "intersection"), Err(DrawingError::Operation(message)) if message.contains("empty path")));
}
