//! 🔀️ Bounded deterministic planar path boolean operations.

use crate::engine::{DrawingError, PathSegment};
use std::cmp::Ordering;
use std::collections::BTreeSet;

// #region 🔖️Model
const MAX_INPUT_EDGES: usize = 4_096;
const MAX_ATOMIC_EDGES: usize = 65_536;
const RELATIVE_EPSILON: f64 = 1.0e-10;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn from_array(value: [f64; 2]) -> Result<Self, DrawingError> {
        if !value[0].is_finite() || !value[1].is_finite() {
            return Err(DrawingError::InvalidInput("boolean coordinates must be finite".into()));
        }
        Ok(Self { x: value[0], y: value[1] })
    }

    fn to_array(self) -> [f64; 2] {
        [if self.x == 0.0 { 0.0 } else { self.x }, if self.y == 0.0 { 0.0 } else { self.y }]
    }
}

#[derive(Clone, Copy)]
struct Segment {
    from: Point,
    to: Point,
}

#[derive(Clone, Copy)]
struct DirectedEdge {
    from: usize,
    to: usize,
}

#[derive(Clone, Copy)]
enum Operation {
    Union,
    Difference,
    Intersection,
    Xor,
}

impl Operation {
    fn parse(value: &str) -> Result<Self, DrawingError> {
        match value {
            "union" => Ok(Self::Union),
            "difference" => Ok(Self::Difference),
            "intersection" => Ok(Self::Intersection),
            "xor" => Ok(Self::Xor),
            _ => Err(DrawingError::InvalidInput(format!("unknown boolean operation: {value}"))),
        }
    }

    fn apply(self, a: bool, b: bool) -> bool {
        match self {
            Self::Union => a || b,
            Self::Difference => a && !b,
            Self::Intersection => a && b,
            Self::Xor => a != b,
        }
    }
}

#[derive(Clone, Copy)]
enum FillRule {
    IndependentContours,
    OrientedBoundary,
}

#[derive(Clone)]
struct Region {
    rings: Vec<Vec<Point>>,
    fill_rule: FillRule,
}

impl Region {
    fn input(rings: Vec<Vec<Point>>) -> Self {
        Self { rings, fill_rule: FillRule::IndependentContours }
    }

    fn boundary(rings: Vec<Vec<Point>>) -> Self {
        Self { rings, fill_rule: FillRule::OrientedBoundary }
    }

    fn contains(&self, point: Point) -> bool {
        match self.fill_rule {
            FillRule::IndependentContours => self.rings.iter().any(|ring| point_in_ring(point, ring)),
            FillRule::OrientedBoundary => self.rings.iter().map(|ring| winding_number(point, ring)).sum::<i32>() != 0,
        }
    }

    fn segments(&self) -> Vec<Segment> {
        let mut result = Vec::new();
        for ring in &self.rings {
            for index in 0..ring.len() {
                let segment = Segment { from: ring[index], to: ring[(index + 1) % ring.len()] };
                if squared_distance(segment.from, segment.to) > 0.0 {
                    result.push(segment);
                }
            }
        }
        result
    }
}
// #endregion 🔖️Model

// #region 🛡️Input
fn close_ring(points: &mut Vec<Point>, rings: &mut Vec<Vec<Point>>) -> Result<(), DrawingError> {
    if points.len() < 3 {
        return Err(DrawingError::InvalidInput("boolean input needs a closed polygon".into()));
    }
    if points.first() == points.last() {
        points.pop();
    }
    rings.push(std::mem::take(points));
    Ok(())
}

fn segments_to_region(segments: &[PathSegment]) -> Result<Region, DrawingError> {
    let mut rings = Vec::new();
    let mut points = Vec::new();
    for segment in segments {
        match segment {
            PathSegment::Move { to } => {
                if !points.is_empty() {
                    close_ring(&mut points, &mut rings)?;
                }
                points.push(Point::from_array(*to)?);
            }
            PathSegment::Line { to } => points.push(Point::from_array(*to)?),
            PathSegment::Close if !points.is_empty() => close_ring(&mut points, &mut rings)?,
            _ => {}
        }
    }
    if !points.is_empty() {
        close_ring(&mut points, &mut rings)?;
    }
    if rings.is_empty() {
        return Err(DrawingError::InvalidInput("boolean input needs a closed polygon".into()));
    }
    Ok(Region::input(rings))
}
// #endregion 🛡️Input

// #region 📐️Predicates
fn cross(a: Point, b: Point, c: Point) -> f64 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

fn squared_distance(a: Point, b: Point) -> f64 {
    (a.x - b.x).mul_add(a.x - b.x, (a.y - b.y) * (a.y - b.y))
}

fn interpolate(segment: Segment, parameter: f64) -> Point {
    Point { x: (segment.to.x - segment.from.x).mul_add(parameter, segment.from.x), y: (segment.to.y - segment.from.y).mul_add(parameter, segment.from.y) }
}

fn parameter(segment: Segment, point: Point) -> f64 {
    let dx = segment.to.x - segment.from.x;
    let dy = segment.to.y - segment.from.y;
    if dx.abs() >= dy.abs() {
        if dx == 0.0 {
            0.0
        } else {
            (point.x - segment.from.x) / dx
        }
    } else if dy == 0.0 {
        0.0
    } else {
        (point.y - segment.from.y) / dy
    }
}

fn point_in_ring(point: Point, ring: &[Point]) -> bool {
    let mut inside = false;
    for index in 0..ring.len() {
        let a = ring[index];
        let b = ring[(index + 1) % ring.len()];
        if (a.y > point.y) != (b.y > point.y) {
            let crossing_x = (b.x - a.x).mul_add((point.y - a.y) / (b.y - a.y), a.x);
            if point.x < crossing_x {
                inside = !inside;
            }
        }
    }
    inside
}

fn winding_number(point: Point, ring: &[Point]) -> i32 {
    let mut winding = 0;
    for index in 0..ring.len() {
        let a = ring[index];
        let b = ring[(index + 1) % ring.len()];
        if a.y <= point.y {
            if b.y > point.y && cross(a, b, point) > 0.0 {
                winding += 1;
            }
        } else if b.y <= point.y && cross(a, b, point) < 0.0 {
            winding -= 1;
        }
    }
    winding
}

fn point_segment_distance(point: Point, segment: Segment) -> f64 {
    let length_squared = squared_distance(segment.from, segment.to);
    if length_squared == 0.0 {
        return squared_distance(point, segment.from).sqrt();
    }
    let dot = (point.x - segment.from.x).mul_add(segment.to.x - segment.from.x, (point.y - segment.from.y) * (segment.to.y - segment.from.y));
    let projection = (dot / length_squared).clamp(0.0, 1.0);
    squared_distance(point, interpolate(segment, projection)).sqrt()
}

fn signed_area(ring: &[Point]) -> f64 {
    if ring.len() < 3 {
        return 0.0;
    }
    let origin = ring[0];
    let mut twice_area = 0.0;
    for index in 1..ring.len() - 1 {
        twice_area += cross(origin, ring[index], ring[index + 1]);
    }
    twice_area * 0.5
}
// #endregion 📐️Predicates

// #region ✂️Arrangement
fn add_intersections(a: Segment, b: Segment, a_parameters: &mut Vec<f64>, b_parameters: &mut Vec<f64>, epsilon: f64) {
    let r = Point { x: a.to.x - a.from.x, y: a.to.y - a.from.y };
    let s = Point { x: b.to.x - b.from.x, y: b.to.y - b.from.y };
    let denominator = r.x * s.y - r.y * s.x;
    let offset = Point { x: b.from.x - a.from.x, y: b.from.y - a.from.y };
    let parallel_tolerance = epsilon * (squared_distance(a.from, a.to) * squared_distance(b.from, b.to)).sqrt().max(1.0);
    if denominator.abs() > parallel_tolerance {
        let a_parameter = (offset.x * s.y - offset.y * s.x) / denominator;
        let b_parameter = (offset.x * r.y - offset.y * r.x) / denominator;
        if (-epsilon..=1.0 + epsilon).contains(&a_parameter) && (-epsilon..=1.0 + epsilon).contains(&b_parameter) {
            a_parameters.push(a_parameter.clamp(0.0, 1.0));
            b_parameters.push(b_parameter.clamp(0.0, 1.0));
        }
        return;
    }
    if (offset.x * r.y - offset.y * r.x).abs() > parallel_tolerance {
        return;
    }
    for point in [b.from, b.to] {
        let value = parameter(a, point);
        if (-epsilon..=1.0 + epsilon).contains(&value) {
            a_parameters.push(value.clamp(0.0, 1.0));
        }
    }
    for point in [a.from, a.to] {
        let value = parameter(b, point);
        if (-epsilon..=1.0 + epsilon).contains(&value) {
            b_parameters.push(value.clamp(0.0, 1.0));
        }
    }
}

fn intern_point(nodes: &mut Vec<Point>, point: Point, epsilon: f64) -> usize {
    if let Some(index) = nodes.iter().position(|candidate| squared_distance(*candidate, point) <= epsilon * epsilon) {
        index
    } else {
        nodes.push(point);
        nodes.len() - 1
    }
}

fn arrangement(regions: [&Region; 2], operation: Operation) -> Result<Region, DrawingError> {
    let source_segments = regions.into_iter().flat_map(Region::segments).collect::<Vec<_>>();
    if source_segments.len() > MAX_INPUT_EDGES {
        return Err(DrawingError::InvalidInput(format!("boolean input exceeds {MAX_INPUT_EDGES} linear edges")));
    }
    if source_segments.is_empty() {
        return Ok(Region::boundary(Vec::new()));
    }
    let coordinates = source_segments.iter().flat_map(|segment| [segment.from.x, segment.from.y, segment.to.x, segment.to.y]).collect::<Vec<_>>();
    let minimum = coordinates.iter().copied().fold(f64::INFINITY, f64::min);
    let maximum = coordinates.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let scale = (maximum - minimum).max(1.0);
    let epsilon = scale * RELATIVE_EPSILON + minimum.abs().max(maximum.abs()) * f64::EPSILON * 16.0;
    let mut split_parameters = vec![vec![0.0, 1.0]; source_segments.len()];
    for left in 0..source_segments.len() {
        for right in left + 1..source_segments.len() {
            let (before, after) = split_parameters.split_at_mut(right);
            add_intersections(source_segments[left], source_segments[right], &mut before[left], &mut after[0], RELATIVE_EPSILON);
        }
    }
    let mut nodes = Vec::new();
    let mut atomic = BTreeSet::new();
    for (segment, parameters) in source_segments.iter().zip(split_parameters.iter_mut()) {
        parameters.sort_by(|a, b| a.total_cmp(b));
        parameters.dedup_by(|a, b| (*a - *b).abs() <= RELATIVE_EPSILON);
        for pair in parameters.windows(2) {
            let from = intern_point(&mut nodes, interpolate(*segment, pair[0]), epsilon);
            let to = intern_point(&mut nodes, interpolate(*segment, pair[1]), epsilon);
            if from != to {
                atomic.insert((from.min(to), from.max(to)));
            }
        }
    }
    if atomic.len() > MAX_ATOMIC_EDGES {
        return Err(DrawingError::InvalidInput(format!("boolean arrangement exceeds {MAX_ATOMIC_EDGES} atomic edges")));
    }
    let arrangement_segments = atomic.iter().map(|(from, to)| Segment { from: nodes[*from], to: nodes[*to] }).collect::<Vec<_>>();
    let mut boundary = Vec::new();
    for ((from, to), segment) in atomic.into_iter().zip(arrangement_segments.iter().copied()) {
        let midpoint = interpolate(segment, 0.5);
        let length = squared_distance(segment.from, segment.to).sqrt();
        let nearest = source_segments.iter().map(|candidate| point_segment_distance(midpoint, *candidate)).filter(|distance| *distance > epsilon).fold(f64::INFINITY, f64::min);
        let offset = (length * 1.0e-6).min(scale * 1.0e-7).min(nearest * 0.2).max(epsilon * 4.0);
        let normal = Point { x: -(segment.to.y - segment.from.y) / length, y: (segment.to.x - segment.from.x) / length };
        let left = Point { x: midpoint.x + normal.x * offset, y: midpoint.y + normal.y * offset };
        let right = Point { x: midpoint.x - normal.x * offset, y: midpoint.y - normal.y * offset };
        let left_filled = operation.apply(regions[0].contains(left), regions[1].contains(left));
        let right_filled = operation.apply(regions[0].contains(right), regions[1].contains(right));
        if left_filled != right_filled {
            boundary.push(if left_filled { DirectedEdge { from, to } } else { DirectedEdge { from: to, to: from } });
        }
    }
    Ok(Region::boundary(trace_rings(&nodes, &boundary, epsilon)?))
}
// #endregion ✂️Arrangement

// #region 🧵️Contours
fn point_order(a: Point, b: Point) -> Ordering {
    a.x.total_cmp(&b.x).then_with(|| a.y.total_cmp(&b.y))
}

fn simplify_ring(mut ring: Vec<Point>, epsilon: f64) -> Vec<Point> {
    loop {
        let previous_len = ring.len();
        if previous_len < 3 {
            return ring;
        }
        ring = (0..previous_len)
            .filter_map(|index| {
                let previous = ring[(index + previous_len - 1) % previous_len];
                let current = ring[index];
                let next = ring[(index + 1) % previous_len];
                let collinear = cross(previous, current, next).abs() <= epsilon * (squared_distance(previous, current).sqrt() + squared_distance(current, next).sqrt()).max(1.0);
                (!collinear).then_some(current)
            })
            .collect();
        if ring.len() == previous_len {
            return ring;
        }
    }
}

fn rotate_canonical(ring: &mut [Point]) {
    let exterior = signed_area(ring) > 0.0;
    let index = (0..ring.len()).min_by(|left, right| ring[*left].x.total_cmp(&ring[*right].x).then_with(|| if exterior { ring[*left].y.total_cmp(&ring[*right].y) } else { ring[*right].y.total_cmp(&ring[*left].y) })).unwrap_or(0);
    ring.rotate_left(index);
}

fn trace_rings(nodes: &[Point], edges: &[DirectedEdge], epsilon: f64) -> Result<Vec<Vec<Point>>, DrawingError> {
    let mut outgoing = vec![Vec::new(); nodes.len()];
    for (index, edge) in edges.iter().enumerate() {
        outgoing[edge.from].push(index);
    }
    let mut order = (0..edges.len()).collect::<Vec<_>>();
    order.sort_by(|left, right| point_order(nodes[edges[*left].from], nodes[edges[*right].from]).then_with(|| point_order(nodes[edges[*left].to], nodes[edges[*right].to])));
    let mut used = vec![false; edges.len()];
    let mut rings = Vec::new();
    for start in order {
        if used[start] {
            continue;
        }
        let start_node = edges[start].from;
        let mut current = start;
        let mut ring = vec![nodes[start_node]];
        loop {
            if used[current] {
                return Err(DrawingError::Operation("boolean boundary topology is not manifold".into()));
            }
            used[current] = true;
            let edge = edges[current];
            if edge.to == start_node {
                break;
            }
            ring.push(nodes[edge.to]);
            let reverse_angle = (nodes[edge.from].y - nodes[edge.to].y).atan2(nodes[edge.from].x - nodes[edge.to].x);
            current = outgoing[edge.to]
                .iter()
                .copied()
                .filter(|candidate| !used[*candidate])
                .min_by(|left, right| {
                    let left_edge = edges[*left];
                    let right_edge = edges[*right];
                    let left_angle = (nodes[left_edge.to].y - nodes[left_edge.from].y).atan2(nodes[left_edge.to].x - nodes[left_edge.from].x);
                    let right_angle = (nodes[right_edge.to].y - nodes[right_edge.from].y).atan2(nodes[right_edge.to].x - nodes[right_edge.from].x);
                    (reverse_angle - left_angle).rem_euclid(std::f64::consts::TAU).total_cmp(&(reverse_angle - right_angle).rem_euclid(std::f64::consts::TAU))
                })
                .ok_or_else(|| DrawingError::Operation("boolean boundary topology is open".into()))?;
        }
        let mut ring = simplify_ring(ring, epsilon);
        if ring.len() >= 3 && signed_area(&ring).abs() > epsilon * epsilon {
            rotate_canonical(&mut ring);
            rings.push(ring);
        }
    }
    let (mut exteriors, holes): (Vec<_>, Vec<_>) = rings.into_iter().partition(|ring| signed_area(ring) > 0.0);
    exteriors.sort_by(|left, right| point_order(left[0], right[0]).then_with(|| signed_area(right).total_cmp(&signed_area(left))));
    let mut owned_holes = vec![Vec::new(); exteriors.len()];
    let mut unowned_holes = Vec::new();
    for hole in holes {
        let owner = exteriors.iter().enumerate().filter(|(_, exterior)| point_in_ring(hole[0], exterior)).min_by(|(_, left), (_, right)| signed_area(left).total_cmp(&signed_area(right))).map(|(index, _)| index);
        if let Some(owner) = owner {
            owned_holes[owner].push(hole);
        } else {
            unowned_holes.push(hole);
        }
    }
    let mut ordered = Vec::new();
    for (exterior, mut holes) in exteriors.into_iter().zip(owned_holes) {
        holes.sort_by(|left, right| point_order(left[0], right[0]).then_with(|| signed_area(left).abs().total_cmp(&signed_area(right).abs())));
        ordered.push(exterior);
        ordered.extend(holes);
    }
    unowned_holes.sort_by(|left, right| point_order(left[0], right[0]));
    ordered.extend(unowned_holes);
    Ok(ordered)
}

fn rings_to_segments(rings: &[Vec<Point>]) -> Vec<PathSegment> {
    let mut segments = Vec::new();
    for ring in rings {
        if ring.is_empty() {
            continue;
        }
        segments.push(PathSegment::Move { to: ring[0].to_array() });
        for point in ring.iter().skip(1).rev() {
            segments.push(PathSegment::Line { to: point.to_array() });
        }
        segments.push(PathSegment::Line { to: ring[0].to_array() });
        segments.push(PathSegment::Close);
    }
    segments
}

fn input_rings_to_segments(rings: &[Vec<Point>]) -> Vec<PathSegment> {
    let mut segments = Vec::new();
    for ring in rings {
        if ring.is_empty() {
            continue;
        }
        segments.push(PathSegment::Move { to: ring[0].to_array() });
        segments.extend(ring.iter().skip(1).map(|point| PathSegment::Line { to: point.to_array() }));
        segments.push(PathSegment::Line { to: ring[0].to_array() });
        segments.push(PathSegment::Close);
    }
    segments
}

fn require_nonempty(region: Region) -> Result<Region, DrawingError> {
    if region.rings.is_empty() {
        Err(DrawingError::Operation("boolean produced empty path".into()))
    } else {
        Ok(region)
    }
}
// #endregion 🧵️Contours

// #region 🎯️API
/// 🔀️ Applies a regularized planar boolean operation to two finite linear path collections.
pub fn boolean_paths(a: &[PathSegment], b: &[PathSegment], operation: &str) -> Result<Vec<PathSegment>, DrawingError> {
    let operation = Operation::parse(operation)?;
    let a = segments_to_region(a)?;
    let b = segments_to_region(b)?;
    Ok(rings_to_segments(&require_nonempty(arrangement([&a, &b], operation)?)?.rings))
}

/// 🔀️ Applies a running regularized planar boolean operation to one or more path collections.
pub fn boolean_paths_many(inputs: &[Vec<PathSegment>], operation: &str) -> Result<Vec<PathSegment>, DrawingError> {
    if inputs.is_empty() {
        return Err(DrawingError::InvalidInput("boolean operation needs at least one path".into()));
    }
    let mut accumulator = segments_to_region(&inputs[0])?;
    if inputs.len() == 1 {
        return Ok(input_rings_to_segments(&accumulator.rings));
    }
    let operation = Operation::parse(operation)?;
    for input in inputs.iter().skip(1) {
        let next = segments_to_region(input)?;
        accumulator = arrangement([&accumulator, &next], operation)?;
    }
    Ok(rings_to_segments(&require_nonempty(accumulator)?.rings))
}
// #endregion 🎯️API

// #region 🧪️Tests
#[cfg(test)]
mod tests {
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
}
// #endregion 🧪️Tests
