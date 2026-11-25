//! 🔷️ Owned constrained Bowyer-Watson triangulation, input preparation and the one-shot `triangulate`.

use super::*;
use std::ops::Index;

// #region 🔖️PlanarDomain
// #region 🧭️OwnedTriangulation
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct Edge(pub(super) usize, pub(super) usize);

impl Edge {
    pub(super) fn new(a: usize, b: usize) -> Self {
        if a < b {
            Self(a, b)
        } else {
            Self(b, a)
        }
    }
}

#[derive(Clone, Debug)]
pub(super) struct OwnedTriangulation {
    pub(super) points: MeshRows<[f64; 2]>,
    pub(super) triangles: MeshRows<[usize; 3]>,
    pub(super) input_len: usize,
    pub(super) insert_cursor: usize,
    pub(super) insertion_order: MeshRows<usize>,
    pub(super) insertion: PointInsertion,
    pub(super) maximum_triangles: usize,
    pub(super) allocation_fault: bool,
    pub(super) mounted_initialization: MountedTriangulationInitialization,
    pub(super) finish: TriangulationFinishCursor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum MountedTriangulationStage {
    BoundsPoint,
    ValidateBounds,
    ReserveInsertionOrder,
    BuildInsertionOrder,
    OrderInsertion,
    ReserveSuperPoints,
    AppendSuperPoint,
    ReserveTriangles,
    SeedTriangle,
    Complete,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct MountedTriangulationInitialization {
    pub(super) stage: MountedTriangulationStage,
    pub(super) cursor: usize,
    pub(super) sort_outer: usize,
    pub(super) sort_inner: usize,
    pub(super) bounds: [f64; 4],
    pub(super) center: [f64; 2],
    pub(super) span: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum TriangulationFinishStage {
    Filter,
    TruncateTriangles,
    OrderTriangles,
    TruncatePoints,
    Complete,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct TriangulationFinishCursor {
    pub(super) stage: TriangulationFinishStage,
    pub(super) read: usize,
    pub(super) write: usize,
    pub(super) sort_outer: usize,
    pub(super) sort_inner: usize,
}

#[derive(Clone, Debug)]
pub(super) struct PointInsertion {
    pub(super) point_index: usize,
    phase: PointInsertionPhase,
    pub(super) cursor: usize,
    pub(super) bad: MeshRows<usize>,
    containing: Option<usize>,
    pub(super) boundary: MeshRows<(Edge, usize)>,
    bad_cursor: usize,
    kept: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PointInsertionPhase {
    Idle,
    Scan,
    Retain,
    Trim,
    Fan,
    Clear,
}

impl PointInsertion {
    pub(super) fn new(mounted: bool) -> Self {
        Self { point_index: 0, phase: PointInsertionPhase::Idle, cursor: 0, bad: MeshRows::new(mounted), containing: None, boundary: MeshRows::new(mounted), bad_cursor: 0, kept: 0 }
    }

    fn reserve_bad(&mut self, context: &mut Option<&mut StepContext<'_>>) -> Result<bool, ()> {
        if !self.bad.is_mounted() { return Ok(true); }
        let context = context.as_deref_mut().ok_or(())?;
        Ok(self.bad.reserve(self.bad.len() + 1, context)? && self.boundary.reserve(self.boundary.len() + 3, context)?)
    }

    fn add_bad(&mut self, index: usize, triangle: [usize; 3]) {
        self.bad.push(index);
        for edge in [Edge::new(triangle[0], triangle[1]), Edge::new(triangle[1], triangle[2]), Edge::new(triangle[2], triangle[0])] {
            let (mut left, mut right) = (0, self.boundary.len());
            while left < right {
                let middle = left + (right - left) / 2;
                if self.boundary[middle].0 < edge { left = middle + 1; } else { right = middle; }
            }
            if self.boundary.get(left).is_some_and(|entry| entry.0 == edge) {
                self.boundary[left].1 += 1;
            } else {
                self.boundary.push((edge, 1));
                for index in (left..self.boundary.len() - 1).rev() {
                    self.boundary.swap(index, index + 1);
                }
            }
        }
    }
}

fn orient(a: [f64; 2], b: [f64; 2], c: [f64; 2]) -> f64 {
    (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
}

pub(super) fn ccw_triangle(points: &(impl Index<usize, Output = [f64; 2]> + ?Sized), mut triangle: [usize; 3]) -> Option<[usize; 3]> {
    let signed = orient(points[triangle[0]], points[triangle[1]], points[triangle[2]]);
    if signed.abs() <= f64::EPSILON {
        return None;
    }
    if signed < 0.0 {
        triangle.swap(1, 2);
    }
    Some(triangle)
}

fn in_circumcircle(points: &MeshRows<[f64; 2]>, triangle: [usize; 3], point: [f64; 2]) -> bool {
    let a = points[triangle[0]];
    let b = points[triangle[1]];
    let c = points[triangle[2]];
    let ax = a[0] - point[0];
    let ay = a[1] - point[1];
    let bx = b[0] - point[0];
    let by = b[1] - point[1];
    let cx = c[0] - point[0];
    let cy = c[1] - point[1];
    let determinant = (ax * ax + ay * ay) * (bx * cy - by * cx) - (bx * bx + by * by) * (ax * cy - ay * cx) + (cx * cx + cy * cy) * (ax * by - ay * bx);
    let scale = ax.abs().max(ay.abs()).max(bx.abs()).max(by.abs()).max(cx.abs()).max(cy.abs()).max(1.0);
    determinant > f64::EPSILON * scale.powi(4) * 32.0
}

pub(super) fn segments_cross(a: [f64; 2], b: [f64; 2], c: [f64; 2], d: [f64; 2]) -> bool {
    let epsilon = f64::EPSILON * 64.0;
    let ab_c = orient(a, b, c);
    let ab_d = orient(a, b, d);
    let cd_a = orient(c, d, a);
    let cd_b = orient(c, d, b);
    ab_c * ab_d < -epsilon && cd_a * cd_b < -epsilon
}

impl OwnedTriangulation {
    pub(super) fn begin(points: Vec<[f64; 2]>) -> Result<Self, MeshError> {
        if points.len() < 3 || points.iter().flatten().any(|coordinate| !coordinate.is_finite()) {
            return Err(MeshError::DegenerateDomain);
        }
        let mut unique = BTreeMap::new();
        for point in points {
            unique.entry((point[0].to_bits(), point[1].to_bits())).or_insert(point);
        }
        let mut points: Vec<_> = unique.into_values().collect();
        points.sort_by(|a, b| a[0].total_cmp(&b[0]).then_with(|| a[1].total_cmp(&b[1])));
        if points.len() < 3 {
            return Err(MeshError::DegenerateDomain);
        }
        let input_len = points.len();
        let (mut min_x, mut max_x, mut min_y, mut max_y) = (points[0][0], points[0][0], points[0][1], points[0][1]);
        for point in &points[1..] {
            min_x = min_x.min(point[0]);
            max_x = max_x.max(point[0]);
            min_y = min_y.min(point[1]);
            max_y = max_y.max(point[1]);
        }
        let span = (max_x - min_x).max(max_y - min_y);
        if span <= f64::EPSILON {
            return Err(MeshError::DegenerateDomain);
        }
        let center = [(min_x + max_x) * 0.5, (min_y + max_y) * 0.5];
        let mut insertion_order: Vec<_> = (0..input_len).collect();
        insertion_order.sort_by_key(|index| {
            let x = points[*index][0].to_bits();
            let y = points[*index][1].to_bits();
            x.wrapping_mul(0x9e37_79b9_7f4a_7c15).rotate_left(17) ^ y.wrapping_mul(0xbf58_476d_1ce4_e5b9).rotate_left(41)
        });
        points.extend_from_slice(&[[center[0] - 32.0 * span, center[1] - 16.0 * span], [center[0] + 32.0 * span, center[1] - 16.0 * span], [center[0], center[1] + 32.0 * span]]);
        Ok(Self {
            points: points.into(),
            triangles: vec![[input_len, input_len + 1, input_len + 2]].into(),
            input_len,
            insert_cursor: 0,
            insertion_order: insertion_order.into(),
            insertion: PointInsertion::new(false),
            maximum_triangles: usize::MAX,
            allocation_fault: false,
            mounted_initialization: MountedTriangulationInitialization { stage: MountedTriangulationStage::Complete, cursor: 0, sort_outer: 0, sort_inner: 0, bounds: [0.0; 4], center: [0.0; 2], span: 0.0 },
            finish: TriangulationFinishCursor { stage: TriangulationFinishStage::Filter, read: 0, write: 0, sort_outer: 1, sort_inner: 1 },
        })
    }

    pub(super) fn begin_mounted(points: MeshRows<[f64; 2]>, maximum_triangles: usize) -> Self {
        let input_len = points.len();
        let mounted = points.is_mounted();
        Self {
            points,
            triangles: MeshRows::new(mounted),
            input_len,
            insert_cursor: 0,
            insertion_order: MeshRows::new(mounted),
            insertion: PointInsertion::new(mounted),
            maximum_triangles,
            allocation_fault: false,
            mounted_initialization: MountedTriangulationInitialization {
                stage: MountedTriangulationStage::BoundsPoint,
                cursor: 0,
                sort_outer: 1,
                sort_inner: 1,
                bounds: [f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY],
                center: [0.0; 2],
                span: 0.0,
            },
            finish: TriangulationFinishCursor { stage: TriangulationFinishStage::Filter, read: 0, write: 0, sort_outer: 1, sort_inner: 1 },
        }
    }

    fn insertion_key(&self, index: usize) -> u64 {
        let point = self.points[index];
        point[0].to_bits().wrapping_mul(0x9e37_79b9_7f4a_7c15).rotate_left(17) ^ point[1].to_bits().wrapping_mul(0xbf58_476d_1ce4_e5b9).rotate_left(41)
    }

    pub(super) fn advance_mounted_initialization(&mut self, context: &mut StepContext<'_>) -> Result<bool, MeshError> {
        match self.mounted_initialization.stage {
            MountedTriangulationStage::BoundsPoint => {
                if let Some(point) = self.points.get(self.mounted_initialization.cursor).copied() {
                    if !point[0].is_finite() || !point[1].is_finite() {
                        return Err(MeshError::DegenerateDomain);
                    }
                    self.mounted_initialization.bounds[0] = self.mounted_initialization.bounds[0].min(point[0]);
                    self.mounted_initialization.bounds[1] = self.mounted_initialization.bounds[1].max(point[0]);
                    self.mounted_initialization.bounds[2] = self.mounted_initialization.bounds[2].min(point[1]);
                    self.mounted_initialization.bounds[3] = self.mounted_initialization.bounds[3].max(point[1]);
                    self.mounted_initialization.cursor += 1;
                } else {
                    self.mounted_initialization.stage = MountedTriangulationStage::ValidateBounds;
                }
            }
            MountedTriangulationStage::ValidateBounds => {
                let bounds = self.mounted_initialization.bounds;
                let span = (bounds[1] - bounds[0]).max(bounds[3] - bounds[2]);
                if span <= f64::EPSILON {
                    return Err(MeshError::DegenerateDomain);
                }
                self.mounted_initialization.span = span;
                self.mounted_initialization.center = [(bounds[0] + bounds[1]) * 0.5, (bounds[2] + bounds[3]) * 0.5];
                self.mounted_initialization.stage = MountedTriangulationStage::ReserveInsertionOrder;
            }
            MountedTriangulationStage::ReserveInsertionOrder => {
                if !self.insertion_order.reserve(self.input_len, context).map_err(|_| MeshError::TriangulationFailed("mounted insertion-order backing rejected".into()))? {
                    return Ok(false);
                }
                self.mounted_initialization.cursor = 0;
                self.mounted_initialization.stage = MountedTriangulationStage::BuildInsertionOrder;
            }
            MountedTriangulationStage::BuildInsertionOrder => {
                if self.mounted_initialization.cursor < self.input_len {
                    self.insertion_order.push(self.mounted_initialization.cursor);
                    self.mounted_initialization.cursor += 1;
                } else {
                    self.mounted_initialization.stage = MountedTriangulationStage::OrderInsertion;
                }
            }
            MountedTriangulationStage::OrderInsertion => {
                if self.mounted_initialization.sort_outer >= self.insertion_order.len() {
                    self.mounted_initialization.stage = MountedTriangulationStage::ReserveSuperPoints;
                } else if self.mounted_initialization.sort_inner > 0 {
                    let right = self.mounted_initialization.sort_inner;
                    let left = right - 1;
                    if self.insertion_key(self.insertion_order[right]) < self.insertion_key(self.insertion_order[left]) {
                        self.insertion_order.swap(left, right);
                        self.mounted_initialization.sort_inner -= 1;
                    } else {
                        self.mounted_initialization.sort_outer += 1;
                        self.mounted_initialization.sort_inner = self.mounted_initialization.sort_outer;
                    }
                } else {
                    self.mounted_initialization.sort_outer += 1;
                    self.mounted_initialization.sort_inner = self.mounted_initialization.sort_outer;
                }
            }
            MountedTriangulationStage::ReserveSuperPoints => {
                if !self.points.reserve(self.input_len + 3, context).map_err(|_| MeshError::TriangulationFailed("mounted triangulation-point backing rejected".into()))? {
                    return Ok(false);
                }
                self.mounted_initialization.cursor = 0;
                self.mounted_initialization.stage = MountedTriangulationStage::AppendSuperPoint;
            }
            MountedTriangulationStage::AppendSuperPoint => {
                let center = self.mounted_initialization.center;
                let span = self.mounted_initialization.span;
                let points = [[center[0] - 32.0 * span, center[1] - 16.0 * span], [center[0] + 32.0 * span, center[1] - 16.0 * span], [center[0], center[1] + 32.0 * span]];
                if let Some(point) = points.get(self.mounted_initialization.cursor).copied() {
                    self.points.push(point);
                    self.mounted_initialization.cursor += 1;
                } else {
                    self.mounted_initialization.stage = MountedTriangulationStage::ReserveTriangles;
                }
            }
            MountedTriangulationStage::ReserveTriangles => {
                if !self.triangles.reserve(1, context).map_err(|_| MeshError::TriangulationFailed("mounted triangulation-face backing rejected".into()))? {
                    return Ok(false);
                }
                self.mounted_initialization.stage = MountedTriangulationStage::SeedTriangle;
            }
            MountedTriangulationStage::SeedTriangle => {
                self.triangles.push([self.input_len, self.input_len + 1, self.input_len + 2]);
                self.mounted_initialization.stage = MountedTriangulationStage::Complete;
            }
            MountedTriangulationStage::Complete => return Ok(true),
        }
        Ok(false)
    }

    pub(super) fn insert_next(&mut self, mut context: Option<&mut StepContext<'_>>) -> bool {
        if self.insert_cursor >= self.input_len {
            return false;
        }
        if self.advance_insertion(&mut context).is_err() {
            self.allocation_fault = true;
        }
        true
    }

    fn advance_insertion(&mut self, context: &mut Option<&mut StepContext<'_>>) -> Result<(), ()> {
        let insertion = &mut self.insertion;
        match insertion.phase {
            PointInsertionPhase::Idle => {
                insertion.point_index = self.insertion_order[self.insert_cursor];
                insertion.cursor = 0;
                insertion.containing = None;
                insertion.bad_cursor = 0;
                insertion.kept = 0;
                insertion.phase = PointInsertionPhase::Scan;
            }
            PointInsertionPhase::Scan => {
                if let Some(triangle) = self.triangles.get(insertion.cursor).copied() {
                    let point = self.points[insertion.point_index];
                    if in_circumcircle(&self.points, triangle, point) {
                        if !insertion.reserve_bad(context)? { return Ok(()); }
                        insertion.add_bad(insertion.cursor, triangle);
                    } else if insertion.containing.is_none()
                        && orient(self.points[triangle[0]], self.points[triangle[1]], point) >= 0.0
                        && orient(self.points[triangle[1]], self.points[triangle[2]], point) >= 0.0
                        && orient(self.points[triangle[2]], self.points[triangle[0]], point) >= 0.0
                    {
                        insertion.containing = Some(insertion.cursor);
                    }
                    insertion.cursor += 1;
                } else {
                    if insertion.bad.is_empty() {
                        if let Some(index) = insertion.containing {
                            if !insertion.reserve_bad(context)? { return Ok(()); }
                            insertion.add_bad(index, self.triangles[index]);
                        }
                    }
                    insertion.cursor = 0;
                    insertion.phase = PointInsertionPhase::Retain;
                }
            }
            PointInsertionPhase::Retain => {
                if let Some(triangle) = self.triangles.get(insertion.cursor).copied() {
                    if insertion.bad.get(insertion.bad_cursor) == Some(&insertion.cursor) {
                        insertion.bad_cursor += 1;
                    } else {
                        self.triangles[insertion.kept] = triangle;
                        insertion.kept += 1;
                    }
                    insertion.cursor += 1;
                } else {
                    insertion.phase = PointInsertionPhase::Trim;
                }
            }
            PointInsertionPhase::Trim => {
                if self.triangles.len() > insertion.kept {
                    self.triangles.pop();
                } else {
                    insertion.cursor = 0;
                    insertion.phase = PointInsertionPhase::Fan;
                }
            }
            PointInsertionPhase::Fan => {
                if let Some(&(Edge(a, b), count)) = insertion.boundary.get(insertion.cursor) {
                    if count == 1 {
                        if let Some(triangle) = ccw_triangle(&self.points, [a, b, insertion.point_index]) {
                            let target = self.triangles.len() + 1;
                            if target > self.maximum_triangles.saturating_mul(4).saturating_add(1) { return Err(()); }
                            if self.triangles.is_mounted() && !self.triangles.reserve(target, context.as_deref_mut().ok_or(())?)? { return Ok(()); }
                            self.triangles.push(triangle);
                        }
                    }
                    insertion.cursor += 1;
                } else {
                    insertion.phase = PointInsertionPhase::Clear;
                }
            }
            PointInsertionPhase::Clear => {
                if insertion.bad.pop().is_none() && insertion.boundary.pop().is_none() {
                    self.insert_cursor += 1;
                    insertion.phase = PointInsertionPhase::Idle;
                }
            }
        }
        Ok(())
    }

    pub(super) fn advance_finish_insertion(&mut self) -> bool {
        match self.finish.stage {
            TriangulationFinishStage::Filter => {
                if let Some(triangle) = self.triangles.get(self.finish.read).copied() {
                    if triangle[0] < self.input_len && triangle[1] < self.input_len && triangle[2] < self.input_len {
                        self.triangles[self.finish.write] = triangle;
                        self.finish.write += 1;
                    }
                    self.finish.read += 1;
                } else {
                    self.finish.stage = TriangulationFinishStage::TruncateTriangles;
                }
            }
            TriangulationFinishStage::TruncateTriangles => {
                if self.triangles.len() > self.finish.write {
                    self.triangles.pop();
                } else {
                    self.finish.stage = TriangulationFinishStage::OrderTriangles;
                }
            }
            TriangulationFinishStage::OrderTriangles => {
                if self.finish.sort_outer >= self.triangles.len() {
                    self.finish.stage = TriangulationFinishStage::TruncatePoints;
                } else if self.finish.sort_inner > 0 {
                    let right = self.finish.sort_inner;
                    let left = right - 1;
                    if self.triangles[right] < self.triangles[left] {
                        self.triangles.swap(left, right);
                        self.finish.sort_inner -= 1;
                    } else {
                        self.finish.sort_outer += 1;
                        self.finish.sort_inner = self.finish.sort_outer;
                    }
                } else {
                    self.finish.sort_outer += 1;
                    self.finish.sort_inner = self.finish.sort_outer;
                }
            }
            TriangulationFinishStage::TruncatePoints => {
                if self.points.len() > self.input_len {
                    self.points.pop();
                } else {
                    self.finish.stage = TriangulationFinishStage::Complete;
                }
            }
            TriangulationFinishStage::Complete => return true,
        }
        false
    }

    fn bowyer_watson(points: Vec<[f64; 2]>) -> Result<Self, MeshError> {
        let mut triangulation = Self::begin(points)?;
        while triangulation.insert_next(None) {
            if triangulation.allocation_fault { return Err(MeshError::TriangulationFailed("triangulation backing rejected".into())); }
        }
        while !triangulation.advance_finish_insertion() {}
        Ok(triangulation)
    }

    fn edges(&self) -> Vec<(Edge, [Option<usize>; 2])> {
        let mut edges = Vec::<(Edge, [Option<usize>; 2])>::with_capacity(self.triangles.len().saturating_mul(3));
        for (triangle_index, triangle) in self.triangles.as_cold().iter().enumerate() {
            for edge in [Edge::new(triangle[0], triangle[1]), Edge::new(triangle[1], triangle[2]), Edge::new(triangle[2], triangle[0])] {
                match edges.binary_search_by_key(&edge, |(current, _)| *current) {
                    Ok(index) => edges[index].1[1] = Some(triangle_index),
                    Err(index) => edges.insert(index, (edge, [Some(triangle_index), None])),
                }
            }
        }
        edges
    }

    fn recover_constraint(&mut self, constraint: Edge, fixed: &[Edge]) -> Result<(), MeshError> {
        let limit = self.triangles.len().saturating_mul(self.triangles.len()).max(1);
        for _ in 0..limit {
            let edges = self.edges();
            if edges.binary_search_by_key(&constraint, |(edge, _)| *edge).is_ok() {
                return Ok(());
            }
            let crossing = edges.iter().find_map(|(edge, adjacent)| {
                if adjacent[1].is_none() || fixed.binary_search(edge).is_ok() || edge.0 == constraint.0 || edge.0 == constraint.1 || edge.1 == constraint.0 || edge.1 == constraint.1 {
                    return None;
                }
                segments_cross(self.points[constraint.0], self.points[constraint.1], self.points[edge.0], self.points[edge.1]).then_some((*edge, [adjacent[0].expect("first adjacency"), adjacent[1].expect("second adjacency")]))
            });
            let Some((Edge(a, b), adjacent)) = crossing else {
                break;
            };
            let first = self.triangles[adjacent[0]];
            let second = self.triangles[adjacent[1]];
            let c = *first.iter().find(|index| **index != a && **index != b).expect("triangle edge has opposite vertex");
            let d = *second.iter().find(|index| **index != a && **index != b).expect("triangle edge has opposite vertex");
            if !segments_cross(self.points[a], self.points[b], self.points[c], self.points[d]) {
                break;
            }
            let Some(first) = ccw_triangle(&self.points, [c, d, a]) else { break };
            let Some(second) = ccw_triangle(&self.points, [d, c, b]) else { break };
            self.triangles[adjacent[0]] = first;
            self.triangles[adjacent[1]] = second;
        }
        if self.edges().binary_search_by_key(&constraint, |(edge, _)| *edge).is_ok() {
            Ok(())
        } else {
            Err(MeshError::TriangulationFailed(format!("could not recover boundary edge {}-{}", constraint.0, constraint.1)))
        }
    }
}

/// 📏️ Points closer than this fraction of the domain's extent are one vertex: a hole corner that
/// lands on the interior lattice, or a subdivision point of one loop meeting another's, must never
/// become two vertices an ulp apart (a sliver the Delaunay kernel cannot recover a constraint through).
const PREPARED_POINT_MERGE_RATIO: f64 = 1e-9;

/// 🔢️ How many equal segments a constrained edge of `length` is cut into at `spacing`: the smallest
/// count whose segments fit, read with a relative tolerance so an edge that is an exact multiple of
/// the spacing (up to floating-point noise) is not split one time too many — two solids sharing an
/// edge then subdivide it identically.
pub(crate) fn constrained_segments(length: f64, spacing: f64) -> usize {
    ((length / spacing) - 1e-9).ceil().max(1.0) as usize
}

/// 📐️ The `i`-th of `segments` equally spaced points from `a` towards `b` — `a + (b − a)·i/n` with
/// the integer product taken first, so a multiple of the spacing lands on its exact binary value and
/// the same edge walked from either end meets on the same coordinates.
pub(crate) fn subdivision_point(a: [f64; 2], b: [f64; 2], segment: usize, segments: usize) -> [f64; 2] {
    let n = segments as f64;
    let i = segment as f64;
    [a[0] + (b[0] - a[0]) * i / n, a[1] + (b[1] - a[1]) * i / n]
}

fn prepare_owned_input(domain: &PlanarDomain, opts: &MeshOpts) -> Result<(Vec<[f64; 2]>, Vec<Edge>), MeshError> {
    if domain.outer.len() < 3 || domain.holes.iter().any(|hole| hole.len() < 3) {
        return Err(MeshError::DegenerateDomain);
    }
    let extent = {
        let min_x = domain.outer.iter().map(|point| point[0]).fold(f64::INFINITY, f64::min);
        let max_x = domain.outer.iter().map(|point| point[0]).fold(f64::NEG_INFINITY, f64::max);
        let min_y = domain.outer.iter().map(|point| point[1]).fold(f64::INFINITY, f64::min);
        let max_y = domain.outer.iter().map(|point| point[1]).fold(f64::NEG_INFINITY, f64::max);
        (max_x - min_x).max(max_y - min_y).max(f64::MIN_POSITIVE)
    };
    let merge_tolerance = extent * PREPARED_POINT_MERGE_RATIO;
    let mut points: Vec<[f64; 2]> = Vec::new();
    let mut constraints = Vec::new();
    let mut point_cells = BTreeMap::<(i64, i64), Vec<usize>>::new();
    let cell_of = |point: [f64; 2]| ((point[0] / merge_tolerance).round() as i64, (point[1] / merge_tolerance).round() as i64);
    let mut insert = |point: [f64; 2]| {
        let cell = cell_of(point);
        for dx in -1..=1 {
            for dy in -1..=1 {
                let Some(candidates) = point_cells.get(&(cell.0 + dx, cell.1 + dy)) else { continue };
                for &index in candidates {
                    let existing = points[index];
                    if (existing[0] - point[0]).abs() <= merge_tolerance && (existing[1] - point[1]).abs() <= merge_tolerance {
                        return index;
                    }
                }
            }
        }
        let index = points.len();
        points.push(point);
        point_cells.entry(cell).or_default().push(index);
        index
    };
    for polygon in std::iter::once(&domain.outer).chain(domain.holes.iter()) {
        let mut loop_indices = Vec::new();
        for index in 0..polygon.len() {
            let a = polygon[index];
            let b = polygon[(index + 1) % polygon.len()];
            let length = ((b[0] - a[0]).powi(2) + (b[1] - a[1]).powi(2)).sqrt();
            let segments = if opts.max_edge > 0.0 { constrained_segments(length, opts.max_edge / 2f64.sqrt()) } else { 1 };
            for segment in 0..segments {
                loop_indices.push(insert(subdivision_point(a, b, segment, segments)));
            }
        }
        loop_indices.dedup();
        for index in 0..loop_indices.len() {
            let edge = Edge::new(loop_indices[index], loop_indices[(index + 1) % loop_indices.len()]);
            if edge.0 != edge.1 {
                constraints.push(edge);
            }
        }
    }
    if opts.max_edge > 0.0 {
        let spacing = opts.max_edge / 2f64.sqrt();
        let min_x = domain.outer.iter().map(|point| point[0]).fold(f64::INFINITY, f64::min);
        let max_x = domain.outer.iter().map(|point| point[0]).fold(f64::NEG_INFINITY, f64::max);
        let min_y = domain.outer.iter().map(|point| point[1]).fold(f64::INFINITY, f64::min);
        let max_y = domain.outer.iter().map(|point| point[1]).fold(f64::NEG_INFINITY, f64::max);
        let columns = constrained_segments(max_x - min_x, spacing);
        let rows = constrained_segments(max_y - min_y, spacing);
        if columns.saturating_mul(rows) > 1_000_000 {
            return Err(MeshError::TriangulationFailed("refinement grid exceeds one million points".to_string()));
        }
        for row in 1..rows {
            for column in 1..columns {
                let point = [subdivision_point([min_x, min_y], [max_x, min_y], column, columns)[0], subdivision_point([min_x, min_y], [min_x, max_y], row, rows)[1]];
                if point_in_polygon(point, &domain.outer) && !domain.holes.iter().any(|hole| point_in_polygon(point, hole)) {
                    insert(point);
                }
            }
        }
    }
    constraints.sort_unstable();
    constraints.dedup();
    Ok((points, constraints))
}

#[derive(Debug)]
pub(super) struct MeshInputPreparation {
    pub(super) maximum_points: usize,
    pub(super) maximum_constraints: usize,
    pub(super) points: MeshRows<[f64; 2]>,
    pub(super) constraints: MeshRows<Edge>,
    pub(super) point_indices: MeshRows<((u64, u64), usize)>,
    pub(super) polygon: usize,
    pub(super) edge: usize,
    pub(super) segment: usize,
    current_segments: usize,
    first: Option<usize>,
    previous: Option<usize>,
    pub(super) grid_row: usize,
    pub(super) grid_column: usize,
    grid_rows: usize,
    grid_columns: usize,
    pub(super) bounds: [f64; 4],
    boundary_complete: bool,
    pub(super) pending_point: Option<[f64; 2]>,
    pub(super) pending_index: Option<usize>,
    pending_boundary: bool,
    pub(super) point_lookup_cursor: usize,
    pub(super) grid_candidate: Option<[f64; 2]>,
    pub(super) grid_polygon: usize,
    pub(super) grid_edge: usize,
    grid_inside: bool,
}

impl MeshInputPreparation {
    pub(super) fn new(maximum_points: usize, maximum_constraints: usize, mounted: bool) -> Self {
        Self {
            maximum_points,
            maximum_constraints,
            points: MeshRows::new(mounted),
            constraints: MeshRows::new(mounted),
            point_indices: MeshRows::new(mounted),
            polygon: 0,
            edge: 0,
            segment: 0,
            current_segments: 0,
            first: None,
            previous: None,
            grid_row: 1,
            grid_column: 1,
            grid_rows: 0,
            grid_columns: 0,
            bounds: [f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY],
            boundary_complete: false,
            pending_point: None,
            pending_index: None,
            pending_boundary: false,
            point_lookup_cursor: 0,
            grid_candidate: None,
            grid_polygon: 0,
            grid_edge: 0,
            grid_inside: false,
        }
    }

    fn advance_pending_point(&mut self, context: &mut StepContext<'_>) -> Result<(), &'static str> {
        let point = self.pending_point.expect("pending preparation point retained");
        let key = (point[0].to_bits(), point[1].to_bits());
        if let Some((current, index)) = self.point_indices.get(self.point_lookup_cursor) {
            if *current == key {
                self.pending_index = Some(*index);
                self.pending_point = None;
                self.point_lookup_cursor = 0;
            } else {
                self.point_lookup_cursor += 1;
            }
            return Ok(());
        }
        if self.points.len() >= self.maximum_points || self.point_indices.len() >= self.maximum_points {
            return Err("mesh-fixed-input-capacity");
        }
        let index = self.points.len();
        if self.points.is_mounted() && !(self.points.reserve(index + 1, context).map_err(|_| "mesh-preparation-points")?
            && self.point_indices.reserve(index + 1, context).map_err(|_| "mesh-preparation-point-index")?) { return Ok(()); }
        self.points.push(point);
        self.point_indices.push((key, index));
        self.pending_index = Some(index);
        self.pending_point = None;
        self.point_lookup_cursor = 0;
        Ok(())
    }

    fn advance_grid_cell(&mut self) {
        self.grid_column += 1;
        if self.grid_column >= self.grid_columns {
            self.grid_column = 1;
            self.grid_row += 1;
        }
    }

    fn accept_pending_index(&mut self, context: &mut StepContext<'_>) -> Result<(), &'static str> {
        let index = self.pending_index.expect("resolved preparation index retained");
        if self.pending_boundary && self.previous.is_some_and(|previous| previous != index) && !self.reserve_constraint(context)? { return Ok(()); }
        self.pending_index = None;
        if self.pending_boundary {
            let point = self.points[index];
            if self.polygon == 0 {
                self.bounds[0] = self.bounds[0].min(point[0]);
                self.bounds[1] = self.bounds[1].max(point[0]);
                self.bounds[2] = self.bounds[2].min(point[1]);
                self.bounds[3] = self.bounds[3].max(point[1]);
            }
            if let Some(previous) = self.previous {
                if previous != index {
                    self.constraints.push(Edge::new(previous, index));
                }
            } else {
                self.first = Some(index);
            }
            self.previous = Some(index);
            let polygon_complete = self.segment + 1 == self.current_segments;
            if polygon_complete {
                self.segment = 0;
                self.edge += 1;
            } else {
                self.segment += 1;
            }
        } else {
            self.advance_grid_cell();
        }
        Ok(())
    }

    fn reserve_constraint(&mut self, context: &mut StepContext<'_>) -> Result<bool, &'static str> {
        if self.constraints.len() >= self.maximum_constraints { return Err("mesh-fixed-input-capacity"); }
        Ok(!self.constraints.is_mounted() || self.constraints.reserve(self.constraints.len() + 1, context).map_err(|_| "mesh-preparation-constraints")?)
    }

    fn advance_grid_classification(&mut self, domain: &MeshDomainOwner) {
        if self.grid_candidate.is_none() {
            self.grid_candidate = Some([
                subdivision_point([self.bounds[0], self.bounds[2]], [self.bounds[1], self.bounds[2]], self.grid_column, self.grid_columns)[0],
                subdivision_point([self.bounds[0], self.bounds[2]], [self.bounds[0], self.bounds[3]], self.grid_row, self.grid_rows)[1],
            ]);
            self.grid_polygon = 0;
            self.grid_edge = 0;
            self.grid_inside = false;
            return;
        }
        let point = self.grid_candidate.expect("grid candidate retained");
        let polygon = if self.grid_polygon == 0 {
            domain.outer()
        } else {
            let Some(polygon) = domain.hole(self.grid_polygon - 1) else {
                self.grid_candidate = None;
                return;
            };
            polygon
        };
        if self.grid_edge < polygon.len() {
            let previous = if self.grid_edge == 0 { polygon.len() - 1 } else { self.grid_edge - 1 };
            let a = polygon[self.grid_edge];
            let b = polygon[previous];
            let crosses = (a[1] > point[1]) != (b[1] > point[1]);
            if crosses && point[0] < (b[0] - a[0]) * (point[1] - a[1]) / (b[1] - a[1]) + a[0] {
                self.grid_inside = !self.grid_inside;
            }
            self.grid_edge += 1;
            return;
        }
        if (self.grid_polygon == 0 && !self.grid_inside) || (self.grid_polygon > 0 && self.grid_inside) {
            self.grid_candidate = None;
            self.advance_grid_cell();
        } else if self.grid_polygon < domain.holes_len() {
            self.grid_polygon += 1;
            self.grid_edge = 0;
            self.grid_inside = false;
        } else {
            self.pending_point = self.grid_candidate.take();
            self.pending_boundary = false;
            self.point_lookup_cursor = 0;
        }
    }

    pub(super) fn advance(&mut self, domain: &MeshDomainOwner, opts: &MeshOpts, context: &mut StepContext<'_>) -> Result<bool, &'static str> {
        if self.pending_index.is_some() {
            self.accept_pending_index(context)?;
            return Ok(false);
        }
        if self.pending_point.is_some() {
            self.advance_pending_point(context)?;
            return Ok(false);
        }
        let polygon_count = 1 + domain.holes_len();
        if !self.boundary_complete {
            if self.polygon >= polygon_count {
                self.boundary_complete = true;
                if opts.max_edge > 0.0 {
                    let spacing = opts.max_edge / 2f64.sqrt();
                    self.grid_columns = constrained_segments(self.bounds[1] - self.bounds[0], spacing);
                    self.grid_rows = constrained_segments(self.bounds[3] - self.bounds[2], spacing);
                    if self.grid_columns.saturating_mul(self.grid_rows) > 1_000_000 {
                        return Err("mesh-preparation-grid-capacity");
                    }
                }
                return Ok(false);
            }
            let polygon = if self.polygon == 0 { domain.outer() } else { domain.hole(self.polygon - 1).ok_or("mesh-preparation-degenerate-domain")? };
            if self.edge == polygon.len() {
                if let (Some(previous), Some(first)) = (self.previous, self.first) {
                    if previous != first {
                        if !self.reserve_constraint(context)? { return Ok(false); }
                        self.constraints.push(Edge::new(previous, first));
                    }
                }
                self.polygon += 1;
                self.edge = 0;
                self.segment = 0;
                self.first = None;
                self.previous = None;
                return Ok(false);
            }
            let a = polygon[self.edge];
            let b = polygon[(self.edge + 1) % polygon.len()];
            let length = ((b[0] - a[0]).powi(2) + (b[1] - a[1]).powi(2)).sqrt();
            let segments = if opts.max_edge > 0.0 { constrained_segments(length, opts.max_edge / 2f64.sqrt()) } else { 1 };
            self.pending_point = Some(subdivision_point(a, b, self.segment, segments));
            self.pending_boundary = true;
            self.current_segments = segments;
            self.point_lookup_cursor = 0;
            return Ok(false);
        }
        if opts.max_edge <= 0.0 || self.grid_row >= self.grid_rows {
            return Ok(true);
        }
        self.advance_grid_classification(domain);
        Ok(false)
    }
}

pub fn owned_triangulate(domain: &PlanarDomain, opts: &MeshOpts) -> Result<TriMesh2, MeshError> {
    let (input_points, input_constraints) = prepare_owned_input(domain, opts)?;
    let prepared_points = input_points.clone();
    let mut triangulation = OwnedTriangulation::bowyer_watson(input_points)?;
    let constraints = remap_constraints(&prepared_points, triangulation.points.as_cold(), input_constraints);
    let mut fixed = Vec::new();
    for constraint in constraints {
        triangulation.recover_constraint(constraint, &fixed)?;
        if let Err(index) = fixed.binary_search(&constraint) {
            fixed.insert(index, constraint);
        }
    }
    let mut triangles = Vec::new();
    for triangle in triangulation.triangles.into_vec() {
        let centroid = [
            (triangulation.points[triangle[0]][0] + triangulation.points[triangle[1]][0] + triangulation.points[triangle[2]][0]) / 3.0,
            (triangulation.points[triangle[0]][1] + triangulation.points[triangle[1]][1] + triangulation.points[triangle[2]][1]) / 3.0,
        ];
        if point_in_polygon(centroid, &domain.outer) && !domain.holes.iter().any(|hole| point_in_polygon(centroid, hole)) {
            triangles.push([triangle[0] as u32, triangle[1] as u32, triangle[2] as u32]);
        }
    }
    triangles.sort_unstable();
    Ok(TriMesh2 { points: triangulation.points.into_vec(), tris: triangles })
}

pub(super) fn remap_constraints(prepared_points: &[[f64; 2]], sorted_points: &[[f64; 2]], input_constraints: Vec<Edge>) -> Vec<Edge> {
    let point_lookup: BTreeMap<_, _> = sorted_points.iter().enumerate().map(|(index, point)| ((point[0].to_bits(), point[1].to_bits()), index)).collect();
    let remap: Vec<_> = prepared_points.iter().map(|point| point_lookup[&(point[0].to_bits(), point[1].to_bits())]).collect();
    input_constraints.into_iter().map(|edge| Edge::new(remap[edge.0], remap[edge.1])).collect()
}
// #endregion 🧭️OwnedTriangulation

/// 🕸️ Constrained Delaunay triangulation of `domain` honoring the outer boundary and hole boundaries
/// as constrained edges, with holes excluded from the output, optionally refined per `opts`.
///
/// Refinement deterministically subdivides constrained edges and seeds an interior lattice before
/// insertion. Triangle inside/outside classification happens after boundary recovery, by a local
/// point-in-polygon centroid test that handles every hole explicitly.
///
/// 🔁️ A lattice of one spacing can land a point an ulp-sliver away from a slanted boundary edge (a rectangle turned by
/// about 45°), where the boundary edge cannot be recovered; the same domain is then retried at the slightly different
/// spacings of [`LATTICE_RETRIES`], the last of which drops the lattice and meshes the boundary alone.
pub fn triangulate(domain: &PlanarDomain, opts: &MeshOpts) -> Result<TriMesh2, MeshError> {
    let mut first = None;
    for factor in LATTICE_RETRIES {
        match owned_triangulate(domain, &MeshOpts { max_edge: opts.max_edge * factor, ..*opts }) {
            Ok(mesh) => return Ok(mesh),
            Err(error) => first = first.or(Some(error)),
        }
    }
    Err(first.unwrap_or(MeshError::DegenerateDomain))
}

/// 🔁️ The factors of `max_edge` [`triangulate`] tries in order: the asked spacing first, a boundary-only mesh last.
const LATTICE_RETRIES: [f64; 7] = [1.0, 1.07, 0.93, 1.19, 0.81, 1.41, 0.0];
// #endregion 🔖️PlanarDomain
