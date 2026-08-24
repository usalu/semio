//! 🕸️ Meshing: 2D constrained Delaunay triangulation with holes (`PlanarDomain` → `TriMesh2`),
//! structured quad grids, quadratic promotion, and 3D extrusion (wedge/hex) with tet splitting.
//! The deterministic constrained Bowyer-Watson kernel is first-party; every public type is plain data
//! composed of `f64` and `u32`, with no geometry implementation leaking through the API.

use semio_framework_job::{Checkpoint, CommitCandidate, InteractiveJob, JobFault, JobPayloadAdmissionFault, JobPayloadStream, Operation, RetainedJobPayload, RetainedJobPayloadWriter, StepContext, StepOutcome};
use std::collections::{BTreeMap, BTreeSet, HashMap};

fn close_vec_owner_step<T>(owner: &mut Vec<T>, maximum_bytes: usize) -> Result<Option<(usize, usize)>, ()> {
    if owner.pop().is_some() {
        return Ok(Some((1, 0)));
    }
    let bytes = owner.capacity().checked_mul(std::mem::size_of::<T>()).ok_or(())?;
    if bytes == 0 {
        return Ok(None);
    }
    if bytes > maximum_bytes {
        return Err(());
    }
    *owner = Vec::new();
    Ok(Some((1, bytes)))
}

// #region 🔖️PlanarDomain
/// 📐️ A planar region to mesh: an outer boundary loop and zero or more hole loops, each a closed
/// polygon (points NOT repeating the first point at the end), in consistent (either) winding order.
#[derive(Clone, Debug, PartialEq)]
pub struct PlanarDomain {
    pub outer: Vec<[f64; 2]>,
    pub holes: Vec<Vec<[f64; 2]>>,
}

pub const MOUNTED_DOMAIN_POINT_SLOTS: usize = 128;
pub const MOUNTED_DOMAIN_HOLE_SLOTS: usize = 32;

/// 🧩 Fixed mounted polygon whose admission and copy advance one slot at a time.
pub struct MountedPlanarPolygon {
    points: [[f64; 2]; MOUNTED_DOMAIN_POINT_SLOTS],
    admitted: usize,
    len: usize,
}

impl MountedPlanarPolygon {
    fn new() -> Self {
        Self { points: [[0.0; 2]; MOUNTED_DOMAIN_POINT_SLOTS], admitted: 0, len: 0 }
    }

    pub fn admit_one(&mut self, target: usize) -> Result<bool, ()> {
        if target > MOUNTED_DOMAIN_POINT_SLOTS {
            return Err(());
        }
        if self.admitted < target {
            self.admitted += 1;
            return Ok(false);
        }
        Ok(true)
    }

    pub fn push(&mut self, point: [f64; 2]) -> Result<(), [f64; 2]> {
        if self.len == self.admitted {
            return Err(point);
        }
        self.points[self.len] = point;
        self.len += 1;
        Ok(())
    }

    fn as_slice(&self) -> &[[f64; 2]] {
        &self.points[..self.len]
    }

    pub fn close_step(&mut self) -> bool {
        if self.len != 0 {
            self.len -= 1;
            return false;
        }
        if self.admitted != 0 {
            self.admitted -= 1;
            return false;
        }
        true
    }
}

/// 🗺️ Fixed mounted planar-domain owner with one admitted polygon or point per opportunity.
pub struct MountedPlanarDomain {
    outer: MountedPlanarPolygon,
    holes: [MountedPlanarPolygon; MOUNTED_DOMAIN_HOLE_SLOTS],
    admitted_holes: usize,
    hole_count: usize,
    close_hole: usize,
}

impl MountedPlanarDomain {
    pub fn new() -> Self {
        Self { outer: MountedPlanarPolygon::new(), holes: std::array::from_fn(|_| MountedPlanarPolygon::new()), admitted_holes: 0, hole_count: 0, close_hole: 0 }
    }

    pub fn admit_outer_one(&mut self, target: usize) -> Result<bool, ()> {
        self.outer.admit_one(target)
    }

    pub fn push_outer(&mut self, point: [f64; 2]) -> Result<(), [f64; 2]> {
        self.outer.push(point)
    }

    pub fn admit_hole_one(&mut self, target: usize) -> Result<bool, ()> {
        if target > MOUNTED_DOMAIN_HOLE_SLOTS {
            return Err(());
        }
        if self.admitted_holes < target {
            self.admitted_holes += 1;
            return Ok(false);
        }
        Ok(true)
    }

    pub fn begin_hole(&mut self) -> Result<usize, ()> {
        if self.hole_count == self.admitted_holes {
            return Err(());
        }
        let index = self.hole_count;
        self.hole_count += 1;
        Ok(index)
    }

    pub fn admit_hole_point_one(&mut self, hole: usize, target: usize) -> Result<bool, ()> {
        self.holes.get_mut(hole).ok_or(())?.admit_one(target)
    }

    pub fn push_hole_point(&mut self, hole: usize, point: [f64; 2]) -> Result<(), [f64; 2]> {
        self.holes.get_mut(hole).ok_or(point)?.push(point)
    }

    fn outer(&self) -> &[[f64; 2]] {
        self.outer.as_slice()
    }

    fn holes_len(&self) -> usize {
        self.hole_count
    }

    fn hole(&self, index: usize) -> Option<&[[f64; 2]]> {
        (index < self.hole_count).then(|| self.holes[index].as_slice())
    }

    pub fn close_step(&mut self) -> bool {
        while self.close_hole < self.hole_count {
            if !self.holes[self.close_hole].close_step() {
                return false;
            }
            self.close_hole += 1;
            return false;
        }
        if self.hole_count != 0 {
            self.hole_count -= 1;
            return false;
        }
        if self.admitted_holes != 0 {
            self.admitted_holes -= 1;
            return false;
        }
        self.outer.close_step()
    }
}

impl Default for MountedPlanarDomain {
    fn default() -> Self {
        Self::new()
    }
}

enum MeshDomainOwner {
    Dynamic(PlanarDomain),
    Mounted(MountedPlanarDomain),
}

impl MeshDomainOwner {
    fn outer(&self) -> &[[f64; 2]] {
        match self {
            Self::Dynamic(domain) => &domain.outer,
            Self::Mounted(domain) => domain.outer(),
        }
    }

    fn holes_len(&self) -> usize {
        match self {
            Self::Dynamic(domain) => domain.holes.len(),
            Self::Mounted(domain) => domain.holes_len(),
        }
    }

    fn hole(&self, index: usize) -> Option<&[[f64; 2]]> {
        match self {
            Self::Dynamic(domain) => domain.holes.get(index).map(Vec::as_slice),
            Self::Mounted(domain) => domain.hole(index),
        }
    }
}

/// 🕸️ A triangulated mesh: shared node positions plus triangles as index triples into `points`.
#[derive(Clone, Debug, PartialEq)]
pub struct TriMesh2 {
    pub points: Vec<[f64; 2]>,
    pub tris: Vec<[u32; 3]>,
}

/// ⚙️ Refinement targets for [`triangulate`] — either left at `0.0` to disable that constraint.
#[derive(Clone, Copy, Debug)]
pub struct MeshOpts {
    pub max_edge: f64,
    pub min_angle_deg: f64,
}

/// ⚠️ Everything that can go wrong building a mesh.
#[derive(Debug)]
pub enum MeshError {
    DegenerateDomain,
    TriangulationFailed(String),
}

impl std::fmt::Display for MeshError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DegenerateDomain => formatter.write_str("domain has a degenerate outer boundary (fewer than 3 points)"),
            Self::TriangulationFailed(detail) => write!(formatter, "triangulation failed: {detail}"),
        }
    }
}

impl std::error::Error for MeshError {}

/// 🎯️ Ray-casting point-in-polygon test (standard even-odd rule; polygon need not be convex).
fn point_in_polygon(point: [f64; 2], polygon: &[[f64; 2]]) -> bool {
    if polygon.len() < 3 {
        return false;
    }
    let mut inside = false;
    let mut j = polygon.len() - 1;
    for i in 0..polygon.len() {
        let (xi, yi) = (polygon[i][0], polygon[i][1]);
        let (xj, yj) = (polygon[j][0], polygon[j][1]);
        let crosses = (yi > point[1]) != (yj > point[1]);
        if crosses && point[0] < (xj - xi) * (point[1] - yi) / (yj - yi) + xi {
            inside = !inside;
        }
        j = i;
    }
    inside
}

// #region 🧭️OwnedTriangulation
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Edge(usize, usize);

impl Edge {
    fn new(a: usize, b: usize) -> Self {
        if a < b {
            Self(a, b)
        } else {
            Self(b, a)
        }
    }
}

#[derive(Clone, Debug)]
struct OwnedTriangulation {
    points: Vec<[f64; 2]>,
    triangles: Vec<[usize; 3]>,
    input_len: usize,
    insert_cursor: usize,
    insertion_order: Vec<usize>,
    insertion: Option<PointInsertion>,
    maximum_triangles: usize,
    allocation_fault: bool,
    mounted_initialization: MountedTriangulationInitialization,
    finish: TriangulationFinishCursor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum MountedTriangulationStage {
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
struct MountedTriangulationInitialization {
    stage: MountedTriangulationStage,
    cursor: usize,
    sort_outer: usize,
    sort_inner: usize,
    bounds: [f64; 4],
    center: [f64; 2],
    span: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum TriangulationFinishStage {
    Filter,
    TruncateTriangles,
    OrderTriangles,
    TruncatePoints,
    Complete,
}

#[derive(Clone, Copy, Debug)]
struct TriangulationFinishCursor {
    stage: TriangulationFinishStage,
    read: usize,
    write: usize,
    sort_outer: usize,
    sort_inner: usize,
}

#[derive(Clone, Debug)]
struct PointInsertion {
    point_index: usize,
    phase: PointInsertionPhase,
    cursor: usize,
    bad: Vec<usize>,
    containing: Option<usize>,
    boundary: Vec<(Edge, usize)>,
    retained: Vec<[usize; 3]>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PointInsertionPhase {
    Scan,
    Retain,
    Fan,
}

fn orient(a: [f64; 2], b: [f64; 2], c: [f64; 2]) -> f64 {
    (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
}

fn ccw_triangle(points: &[[f64; 2]], mut triangle: [usize; 3]) -> Option<[usize; 3]> {
    let signed = orient(points[triangle[0]], points[triangle[1]], points[triangle[2]]);
    if signed.abs() <= f64::EPSILON {
        return None;
    }
    if signed < 0.0 {
        triangle.swap(1, 2);
    }
    Some(triangle)
}

fn in_circumcircle(points: &[[f64; 2]], triangle: [usize; 3], point: [f64; 2]) -> bool {
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

fn segments_cross(a: [f64; 2], b: [f64; 2], c: [f64; 2], d: [f64; 2]) -> bool {
    let epsilon = f64::EPSILON * 64.0;
    let ab_c = orient(a, b, c);
    let ab_d = orient(a, b, d);
    let cd_a = orient(c, d, a);
    let cd_b = orient(c, d, b);
    ab_c * ab_d < -epsilon && cd_a * cd_b < -epsilon
}

impl OwnedTriangulation {
    fn begin(points: Vec<[f64; 2]>) -> Result<Self, MeshError> {
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
            points,
            triangles: vec![[input_len, input_len + 1, input_len + 2]],
            input_len,
            insert_cursor: 0,
            insertion_order,
            insertion: None,
            maximum_triangles: usize::MAX,
            allocation_fault: false,
            mounted_initialization: MountedTriangulationInitialization { stage: MountedTriangulationStage::Complete, cursor: 0, sort_outer: 0, sort_inner: 0, bounds: [0.0; 4], center: [0.0; 2], span: 0.0 },
            finish: TriangulationFinishCursor { stage: TriangulationFinishStage::Filter, read: 0, write: 0, sort_outer: 1, sort_inner: 1 },
        })
    }

    fn begin_mounted(points: Vec<[f64; 2]>, maximum_triangles: usize) -> Result<Self, MeshError> {
        if points.len() < 3 {
            return Err(MeshError::DegenerateDomain);
        }
        let input_len = points.len();
        Ok(Self {
            points,
            triangles: Vec::new(),
            input_len,
            insert_cursor: 0,
            insertion_order: Vec::new(),
            insertion: None,
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
        })
    }

    fn insertion_key(&self, index: usize) -> u64 {
        let point = self.points[index];
        point[0].to_bits().wrapping_mul(0x9e37_79b9_7f4a_7c15).rotate_left(17) ^ point[1].to_bits().wrapping_mul(0xbf58_476d_1ce4_e5b9).rotate_left(41)
    }

    fn advance_mounted_initialization(&mut self) -> Result<bool, MeshError> {
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
                self.insertion_order.try_reserve_exact(self.input_len).map_err(|_| MeshError::TriangulationFailed("mounted insertion-order backing rejected".into()))?;
                if self.insertion_order.capacity().checked_mul(std::mem::size_of::<usize>()).is_none_or(|bytes| bytes > 4_096) {
                    return Err(MeshError::TriangulationFailed("mounted insertion-order backing exceeded page".into()));
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
                self.points.try_reserve_exact(3).map_err(|_| MeshError::TriangulationFailed("mounted triangulation-point backing rejected".into()))?;
                if self.points.capacity().checked_mul(std::mem::size_of::<[f64; 2]>()).is_none_or(|bytes| bytes > 4_096) {
                    return Err(MeshError::TriangulationFailed("mounted triangulation-point backing exceeded page".into()));
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
                self.triangles.try_reserve_exact(self.maximum_triangles.saturating_mul(4).saturating_add(1)).map_err(|_| MeshError::TriangulationFailed("mounted triangulation-face backing rejected".into()))?;
                if self.triangles.capacity().checked_mul(std::mem::size_of::<[usize; 3]>()).is_none_or(|bytes| bytes > 4_096) {
                    return Err(MeshError::TriangulationFailed("mounted triangulation-face backing exceeded page".into()));
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

    fn insert_next(&mut self) -> bool {
        if self.insert_cursor >= self.input_len {
            return false;
        }
        if self.insertion.is_none() {
            let maximum = if self.maximum_triangles == usize::MAX { self.triangles.len().saturating_mul(3).max(1) } else { self.maximum_triangles.saturating_mul(12).saturating_add(3) };
            let mut bad = Vec::new();
            let mut boundary = Vec::new();
            let mut retained = Vec::new();
            if bad.try_reserve_exact(maximum).is_err() || boundary.try_reserve_exact(maximum).is_err() || retained.try_reserve_exact(maximum).is_err() {
                self.allocation_fault = true;
                return true;
            }
            self.insertion = Some(PointInsertion { point_index: self.insertion_order[self.insert_cursor], phase: PointInsertionPhase::Scan, cursor: 0, bad, containing: None, boundary, retained });
            return true;
        }
        let insertion = self.insertion.as_mut().expect("point insertion initialized");
        match insertion.phase {
            PointInsertionPhase::Scan => {
                if insertion.cursor < self.triangles.len() {
                    let triangle_index = insertion.cursor;
                    let triangle = self.triangles[triangle_index];
                    let point = self.points[insertion.point_index];
                    if in_circumcircle(&self.points, triangle, point) {
                        insertion.bad.push(triangle_index);
                        for edge in [Edge::new(triangle[0], triangle[1]), Edge::new(triangle[1], triangle[2]), Edge::new(triangle[2], triangle[0])] {
                            match insertion.boundary.binary_search_by_key(&edge, |(current, _)| *current) {
                                Ok(index) => insertion.boundary[index].1 += 1,
                                Err(index) => insertion.boundary.insert(index, (edge, 1)),
                            }
                        }
                    } else if insertion.containing.is_none()
                        && orient(self.points[triangle[0]], self.points[triangle[1]], point) >= 0.0
                        && orient(self.points[triangle[1]], self.points[triangle[2]], point) >= 0.0
                        && orient(self.points[triangle[2]], self.points[triangle[0]], point) >= 0.0
                    {
                        insertion.containing = Some(triangle_index);
                    }
                    insertion.cursor += 1;
                }
                if insertion.cursor == self.triangles.len() {
                    if insertion.bad.is_empty() {
                        if let Some(triangle_index) = insertion.containing {
                            insertion.bad.push(triangle_index);
                            let triangle = self.triangles[triangle_index];
                            for edge in [Edge::new(triangle[0], triangle[1]), Edge::new(triangle[1], triangle[2]), Edge::new(triangle[2], triangle[0])] {
                                match insertion.boundary.binary_search_by_key(&edge, |(current, _)| *current) {
                                    Ok(index) => insertion.boundary[index].1 += 1,
                                    Err(index) => insertion.boundary.insert(index, (edge, 1)),
                                }
                            }
                        }
                    }
                    insertion.cursor = 0;
                    insertion.phase = PointInsertionPhase::Retain;
                }
            }
            PointInsertionPhase::Retain => {
                if insertion.cursor < self.triangles.len() {
                    if !insertion.bad.contains(&insertion.cursor) {
                        insertion.retained.push(self.triangles[insertion.cursor]);
                    }
                    insertion.cursor += 1;
                }
                if insertion.cursor == self.triangles.len() {
                    self.triangles = std::mem::take(&mut insertion.retained);
                    insertion.phase = PointInsertionPhase::Fan;
                }
            }
            PointInsertionPhase::Fan => {
                if !insertion.boundary.is_empty() {
                    let (Edge(a, b), count) = insertion.boundary.remove(0);
                    if count == 1 {
                        if let Some(triangle) = ccw_triangle(&self.points, [a, b, insertion.point_index]) {
                            self.triangles.push(triangle);
                        }
                    }
                }
                if insertion.boundary.is_empty() {
                    self.insert_cursor += 1;
                    self.insertion = None;
                }
            }
        }
        true
    }

    fn advance_finish_insertion(&mut self) -> bool {
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
                self.triangles.truncate(self.finish.write);
                self.finish.stage = TriangulationFinishStage::OrderTriangles;
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
                self.points.truncate(self.input_len);
                self.finish.stage = TriangulationFinishStage::Complete;
            }
            TriangulationFinishStage::Complete => return true,
        }
        false
    }

    fn bowyer_watson(points: Vec<[f64; 2]>) -> Result<Self, MeshError> {
        let mut triangulation = Self::begin(points)?;
        while triangulation.insert_next() {}
        while !triangulation.advance_finish_insertion() {}
        Ok(triangulation)
    }

    fn edges(&self) -> Vec<(Edge, [Option<usize>; 2])> {
        let mut edges = Vec::<(Edge, [Option<usize>; 2])>::with_capacity(self.triangles.len().saturating_mul(3));
        for (triangle_index, triangle) in self.triangles.iter().enumerate() {
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

fn prepare_owned_input(domain: &PlanarDomain, opts: &MeshOpts) -> Result<(Vec<[f64; 2]>, Vec<Edge>), MeshError> {
    if domain.outer.len() < 3 || domain.holes.iter().any(|hole| hole.len() < 3) {
        return Err(MeshError::DegenerateDomain);
    }
    let mut points = Vec::new();
    let mut constraints = Vec::new();
    let mut point_indices = BTreeMap::<(u64, u64), usize>::new();
    let mut insert = |point: [f64; 2]| {
        let key = (point[0].to_bits(), point[1].to_bits());
        *point_indices.entry(key).or_insert_with(|| {
            let index = points.len();
            points.push(point);
            index
        })
    };
    for polygon in std::iter::once(&domain.outer).chain(domain.holes.iter()) {
        let mut loop_indices = Vec::new();
        for index in 0..polygon.len() {
            let a = polygon[index];
            let b = polygon[(index + 1) % polygon.len()];
            let length = ((b[0] - a[0]).powi(2) + (b[1] - a[1]).powi(2)).sqrt();
            let segments = if opts.max_edge > 0.0 { (length / (opts.max_edge / 2f64.sqrt())).ceil().max(1.0) as usize } else { 1 };
            for segment in 0..segments {
                let t = segment as f64 / segments as f64;
                loop_indices.push(insert([a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t]));
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
        let columns = ((max_x - min_x) / spacing).ceil() as usize;
        let rows = ((max_y - min_y) / spacing).ceil() as usize;
        if columns.saturating_mul(rows) > 1_000_000 {
            return Err(MeshError::TriangulationFailed("refinement grid exceeds one million points".to_string()));
        }
        let column_step = (max_x - min_x) / columns as f64;
        let row_step = (max_y - min_y) / rows as f64;
        for row in 1..rows {
            for column in 1..columns {
                let point = [min_x + column as f64 * column_step, min_y + row as f64 * row_step];
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
struct MeshInputPreparation {
    points: Vec<[f64; 2]>,
    constraints: Vec<Edge>,
    point_indices: Vec<((u64, u64), usize)>,
    polygon: usize,
    edge: usize,
    segment: usize,
    current_segments: usize,
    first: Option<usize>,
    previous: Option<usize>,
    grid_row: usize,
    grid_column: usize,
    grid_rows: usize,
    grid_columns: usize,
    grid_step: [f64; 2],
    bounds: [f64; 4],
    boundary_complete: bool,
    pending_point: Option<[f64; 2]>,
    pending_index: Option<usize>,
    pending_boundary: bool,
    point_lookup_cursor: usize,
    grid_candidate: Option<[f64; 2]>,
    grid_polygon: usize,
    grid_edge: usize,
    grid_inside: bool,
}

impl MeshInputPreparation {
    fn new() -> Self {
        Self {
            points: Vec::new(),
            constraints: Vec::new(),
            point_indices: Vec::new(),
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
            grid_step: [0.0; 2],
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

    fn advance_pending_point(&mut self) {
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
            return;
        }
        let index = self.points.len();
        self.points.push(point);
        self.point_indices.push((key, index));
        self.pending_index = Some(index);
        self.pending_point = None;
        self.point_lookup_cursor = 0;
    }

    fn advance_grid_cell(&mut self) {
        self.grid_column += 1;
        if self.grid_column >= self.grid_columns {
            self.grid_column = 1;
            self.grid_row += 1;
        }
    }

    fn accept_pending_index(&mut self) {
        let index = self.pending_index.take().expect("resolved preparation index retained");
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
    }

    fn advance_grid_classification(&mut self, domain: &MeshDomainOwner) {
        if self.grid_candidate.is_none() {
            self.grid_candidate = Some([self.bounds[0] + self.grid_column as f64 * self.grid_step[0], self.bounds[2] + self.grid_row as f64 * self.grid_step[1]]);
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
        if self.grid_polygon == 0 && !self.grid_inside {
            self.grid_candidate = None;
            self.advance_grid_cell();
        } else if self.grid_polygon > 0 && self.grid_inside {
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

    fn advance(&mut self, domain: &MeshDomainOwner, opts: &MeshOpts) -> Result<bool, MeshError> {
        if self.pending_index.is_some() {
            self.accept_pending_index();
            return Ok(false);
        }
        if self.pending_point.is_some() {
            self.advance_pending_point();
            return Ok(false);
        }
        let polygon_count = 1 + domain.holes_len();
        if !self.boundary_complete {
            if self.polygon >= polygon_count {
                self.boundary_complete = true;
                if opts.max_edge > 0.0 {
                    let spacing = opts.max_edge / 2f64.sqrt();
                    self.grid_columns = ((self.bounds[1] - self.bounds[0]) / spacing).ceil() as usize;
                    self.grid_rows = ((self.bounds[3] - self.bounds[2]) / spacing).ceil() as usize;
                    if self.grid_columns.saturating_mul(self.grid_rows) > 1_000_000 {
                        return Err(MeshError::TriangulationFailed("refinement grid exceeds one million points".to_string()));
                    }
                    if self.grid_columns > 0 && self.grid_rows > 0 {
                        self.grid_step = [(self.bounds[1] - self.bounds[0]) / self.grid_columns as f64, (self.bounds[3] - self.bounds[2]) / self.grid_rows as f64];
                    }
                }
                return Ok(false);
            }
            let polygon = if self.polygon == 0 { domain.outer() } else { domain.hole(self.polygon - 1).ok_or(MeshError::DegenerateDomain)? };
            if self.edge == polygon.len() {
                if let (Some(previous), Some(first)) = (self.previous, self.first) {
                    if previous != first {
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
            let segments = if opts.max_edge > 0.0 { (length / (opts.max_edge / 2f64.sqrt())).ceil().max(1.0) as usize } else { 1 };
            let t = self.segment as f64 / segments as f64;
            self.pending_point = Some([a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t]);
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

fn owned_triangulate(domain: &PlanarDomain, opts: &MeshOpts) -> Result<TriMesh2, MeshError> {
    let (input_points, input_constraints) = prepare_owned_input(domain, opts)?;
    let prepared_points = input_points.clone();
    let mut triangulation = OwnedTriangulation::bowyer_watson(input_points)?;
    let constraints = remap_constraints(&prepared_points, &triangulation.points, input_constraints);
    let mut fixed = BTreeSet::new();
    for constraint in constraints {
        triangulation.recover_constraint(constraint, &fixed)?;
        fixed.insert(constraint);
    }
    let mut triangles = Vec::new();
    for triangle in triangulation.triangles {
        let centroid = [
            (triangulation.points[triangle[0]][0] + triangulation.points[triangle[1]][0] + triangulation.points[triangle[2]][0]) / 3.0,
            (triangulation.points[triangle[0]][1] + triangulation.points[triangle[1]][1] + triangulation.points[triangle[2]][1]) / 3.0,
        ];
        if point_in_polygon(centroid, &domain.outer) && !domain.holes.iter().any(|hole| point_in_polygon(centroid, hole)) {
            triangles.push([triangle[0] as u32, triangle[1] as u32, triangle[2] as u32]);
        }
    }
    triangles.sort_unstable();
    Ok(TriMesh2 { points: triangulation.points, tris: triangles })
}

fn remap_constraints(prepared_points: &[[f64; 2]], sorted_points: &[[f64; 2]], input_constraints: Vec<Edge>) -> Vec<Edge> {
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
pub fn triangulate(domain: &PlanarDomain, opts: &MeshOpts) -> Result<TriMesh2, MeshError> {
    owned_triangulate(domain, opts)
}
// #endregion 🔖️PlanarDomain

// #region 🧵️IncrementalMeshJob
/// 🎚️ The fidelity of a [`MeshJobPreview`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MeshQualityTier {
    Coarse,
    Refined,
    Final,
}

/// 🗺️ A replaceable, non-authoritative mesh overlay published while [`MeshJob`] is running.
#[derive(Clone, Debug, PartialEq)]
pub struct MeshJobPreview {
    pub sequence: u64,
    pub tier: MeshQualityTier,
    pub refinement_steps: usize,
    pub mesh: TriMesh2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MeshJobStage {
    Validate,
    ReservePreparation,
    PrepareInput,
    CountInput,
    Initialize,
    InsertBoundary,
    ReserveEdgeAuthorities,
    IndexEdges,
    ConstrainBoundary,
    ReservePointIndex,
    ReserveMeshPoints,
    ReserveMeshTriangles,
    Classify,
    PublishPreview,
    Finalize,
    PublishCheckpoint,
    Complete,
    Published,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum ConstraintRecoveryStage {
    ReserveConstraintWorkspace,
    IndexTriangleEdge,
    SearchConstraintEdge,
    ClassifyIntersection,
    SelectDeterministicFlip,
    ValidateFlip,
    ApplyFlip,
    RetireFormerEdge,
    PublishConstraintProgress,
    ConstraintComplete,
}

#[derive(Clone, Copy)]
struct IndexedConstraintEdge {
    edge: Edge,
    adjacent: [Option<usize>; 2],
    active: bool,
    fixed: bool,
}

#[derive(Clone, Copy)]
struct ConstraintFlipCandidate {
    edge: Edge,
    adjacent: [usize; 2],
    replacement: [[usize; 3]; 2],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FaceClassificationStage {
    Begin,
    OuterEdge,
    HoleEdge,
    PointLookup,
    PointInsert,
    TrianglePublish,
}

#[derive(Clone, Copy)]
struct FaceClassificationCursor {
    stage: FaceClassificationStage,
    positions: [[f64; 2]; 3],
    centroid: [f64; 2],
    hole: usize,
    edge: usize,
    inside: bool,
    slot: usize,
    lookup: usize,
    indices: [u32; 3],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MeshPublicationKind {
    Preview,
    Checkpoint,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum MeshPayloadStage {
    Magic,
    Tier,
    Complete,
    Sequence,
    Refinement,
    PointCount,
    TriangleCount,
    PointCoordinate,
    TriangleIndex,
    CommitPage,
    Done,
}

#[derive(Clone, Copy, Debug)]
struct MeshPayloadCursor {
    stage: MeshPayloadStage,
    point: usize,
    coordinate: usize,
    triangle: usize,
    index: usize,
}

/// 🧵️ Persistent constrained-mesh state machine. Bowyer-Watson insertion retains scan, cavity
/// compaction, and fan cursors; boundary recovery and face classification retain independent cursors.
/// Only the completed deterministic payload is authoritative.
pub struct MeshJob {
    operation: Operation,
    domain: MeshDomainOwner,
    options: MeshOpts,
    preparation: Option<MeshInputPreparation>,
    prepared_input: Option<(Vec<[f64; 2]>, Vec<Edge>)>,
    triangulation: Option<OwnedTriangulation>,
    constraints: Vec<Edge>,
    fixed_constraints: Vec<Edge>,
    indexed_edges: Vec<Edge>,
    indexed_constraint_edges: Vec<IndexedConstraintEdge>,
    edge_index_cursor: usize,
    edge_index_local_cursor: usize,
    edge_index_lookup_cursor: usize,
    edge_index_vacancy: Option<usize>,
    edge_index_candidate: Option<(Edge, usize)>,
    edge_index_valid: bool,
    stage: MeshJobStage,
    constraint_cursor: usize,
    constraint_stage: ConstraintRecoveryStage,
    constraint_search_cursor: usize,
    constraint_adjacency_cursor: usize,
    constraint_candidate: Option<(Edge, usize)>,
    constraint_flip: Option<ConstraintFlipCandidate>,
    constraint_flip_count: usize,
    constraint_flip_limit: usize,
    constraint_apply_cursor: usize,
    constraint_retire_cursor: usize,
    constraint_retire_adjacency_cursor: usize,
    constraint_reindex_cursor: usize,
    face_cursor: usize,
    face_classification: Option<FaceClassificationCursor>,
    input_count: usize,
    input_count_hole: usize,
    preview_tier: MeshQualityTier,
    mesh: TriMesh2,
    point_index: Vec<((u64, u64), u32)>,
    refinement_steps: usize,
    validation_hole_cursor: usize,
    publication_writer: Option<RetainedJobPayloadWriter>,
    publication_kind: Option<MeshPublicationKind>,
    publication_cursor: MeshPayloadCursor,
    publication_sequence: u64,
    close_lane: u8,
    maximum_points: usize,
    maximum_triangles: usize,
}

impl MeshJob {
    /// 🌱️ Creates a deterministic mesh operation from an immutable domain snapshot.
    pub fn new(domain: PlanarDomain, options: MeshOpts, operation: Operation) -> Self {
        Self::from_domain(MeshDomainOwner::Dynamic(domain), options, operation)
    }

    fn from_domain(domain: MeshDomainOwner, options: MeshOpts, operation: Operation) -> Self {
        Self {
            operation,
            domain,
            options,
            preparation: None,
            prepared_input: None,
            triangulation: None,
            constraints: Vec::new(),
            fixed_constraints: Vec::new(),
            indexed_edges: Vec::new(),
            indexed_constraint_edges: Vec::new(),
            edge_index_cursor: 0,
            edge_index_local_cursor: 0,
            edge_index_lookup_cursor: 0,
            edge_index_vacancy: None,
            edge_index_candidate: None,
            edge_index_valid: false,
            stage: MeshJobStage::Validate,
            constraint_cursor: 0,
            constraint_stage: ConstraintRecoveryStage::ReserveConstraintWorkspace,
            constraint_search_cursor: 0,
            constraint_adjacency_cursor: 0,
            constraint_candidate: None,
            constraint_flip: None,
            constraint_flip_count: 0,
            constraint_flip_limit: 0,
            constraint_apply_cursor: 0,
            constraint_retire_cursor: 0,
            constraint_retire_adjacency_cursor: 0,
            constraint_reindex_cursor: 0,
            face_cursor: 0,
            face_classification: None,
            input_count: 0,
            input_count_hole: 0,
            preview_tier: MeshQualityTier::Coarse,
            mesh: TriMesh2 { points: Vec::new(), tris: Vec::new() },
            point_index: Vec::new(),
            refinement_steps: 0,
            validation_hole_cursor: 0,
            publication_writer: None,
            publication_kind: None,
            publication_cursor: MeshPayloadCursor { stage: MeshPayloadStage::Magic, point: 0, coordinate: 0, triangle: 0, index: 0 },
            publication_sequence: 0,
            close_lane: 0,
            maximum_points: usize::MAX,
            maximum_triangles: usize::MAX,
        }
    }

    /// 🔒️ Creates a mesh operation whose retained point/triangle owners cannot grow past
    /// the caller's already-admitted fixed process credits.
    pub fn new_bounded(domain: PlanarDomain, options: MeshOpts, operation: Operation, maximum_points: usize, maximum_triangles: usize) -> Self {
        let mut job = Self::new(domain, options, operation);
        job.maximum_points = maximum_points;
        job.maximum_triangles = maximum_triangles;
        job
    }

    /// 🧱 Creates a bounded mesh operation by transferring an already-admitted fixed domain owner.
    pub fn new_mounted_bounded(domain: MountedPlanarDomain, options: MeshOpts, operation: Operation, maximum_points: usize, maximum_triangles: usize) -> Self {
        let mut job = Self::from_domain(MeshDomainOwner::Mounted(domain), options, operation);
        job.maximum_points = maximum_points;
        job.maximum_triangles = maximum_triangles;
        job
    }

    /// 🔭 Borrows one completed point without transferring the mesh backing.
    pub fn completed_point(&self, index: usize) -> Option<[f64; 2]> {
        (self.stage == MeshJobStage::Published).then(|| self.mesh.points.get(index).copied()).flatten()
    }

    /// 🔭 Borrows one completed triangle without transferring the mesh backing.
    pub fn completed_triangle(&self, index: usize) -> Option<[u32; 3]> {
        (self.stage == MeshJobStage::Published).then(|| self.mesh.tris.get(index).copied()).flatten()
    }

    pub fn completed_counts(&self) -> Option<(usize, usize)> {
        (self.stage == MeshJobStage::Published).then_some((self.mesh.points.len(), self.mesh.tris.len()))
    }

    /// 🧹️ Retires one exact mesh/domain owner per governed close opportunity.
    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if let Some(writer) = self.publication_writer.as_mut() {
            return match writer.close_step(1, maximum_bytes) {
                semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => (false, released_items, released_bytes),
                semio_framework_job::JobPayloadCloseStep::Complete => {
                    self.publication_writer = None;
                    self.publication_kind = None;
                    (false, 1, 0)
                }
            };
        }
        loop {
            let released = match self.close_lane {
                0 => match &mut self.domain {
                    MeshDomainOwner::Dynamic(domain) => match close_vec_owner_step(&mut domain.outer, maximum_bytes) {
                        Ok(Some(step)) => step,
                        Err(()) => return (false, 0, 0),
                        Ok(None) => {
                            self.close_lane += 1;
                            continue;
                        }
                    },
                    MeshDomainOwner::Mounted(domain) => {
                        if domain.close_step() {
                            self.close_lane += 2;
                            continue;
                        }
                        (1, 0)
                    }
                },
                1 => {
                    let MeshDomainOwner::Dynamic(domain) = &mut self.domain else {
                        self.close_lane += 1;
                        continue;
                    };
                    if let Some(hole) = domain.holes.last_mut() {
                        match close_vec_owner_step(hole, maximum_bytes) {
                            Ok(Some(step)) => return (false, step.0, step.1),
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {}
                        }
                        domain.holes.pop();
                        (1, 0)
                    } else {
                        match close_vec_owner_step(&mut domain.holes, maximum_bytes) {
                            Ok(Some(step)) => step,
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {
                                self.close_lane += 1;
                                continue;
                            }
                        }
                    }
                }
                2 => {
                    if let Some(preparation) = self.preparation.as_mut() {
                        match close_vec_owner_step(&mut preparation.points, maximum_bytes) {
                            Ok(Some(step)) => return (false, step.0, step.1),
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {}
                        }
                        match close_vec_owner_step(&mut preparation.constraints, maximum_bytes) {
                            Ok(Some(step)) => return (false, step.0, step.1),
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {}
                        }
                        match close_vec_owner_step(&mut preparation.point_indices, maximum_bytes) {
                            Ok(Some(step)) => return (false, step.0, step.1),
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {}
                        }
                        self.preparation = None;
                        (1, 0)
                    } else {
                        self.close_lane += 1;
                        continue;
                    }
                }
                3 => {
                    if let Some((points, edges)) = self.prepared_input.as_mut() {
                        match close_vec_owner_step(points, maximum_bytes) {
                            Ok(Some(step)) => return (false, step.0, step.1),
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {}
                        }
                        match close_vec_owner_step(edges, maximum_bytes) {
                            Ok(Some(step)) => return (false, step.0, step.1),
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {}
                        }
                        self.prepared_input = None;
                        (1, 0)
                    } else {
                        self.close_lane += 1;
                        continue;
                    }
                }
                4 => {
                    if let Some(triangulation) = self.triangulation.as_mut() {
                        if let Some(insertion) = triangulation.insertion.as_mut() {
                            match close_vec_owner_step(&mut insertion.bad, maximum_bytes) {
                                Ok(Some(step)) => return (false, step.0, step.1),
                                Err(()) => return (false, 0, 0),
                                Ok(None) => {}
                            }
                            match close_vec_owner_step(&mut insertion.boundary, maximum_bytes) {
                                Ok(Some(step)) => return (false, step.0, step.1),
                                Err(()) => return (false, 0, 0),
                                Ok(None) => {}
                            }
                            match close_vec_owner_step(&mut insertion.retained, maximum_bytes) {
                                Ok(Some(step)) => return (false, step.0, step.1),
                                Err(()) => return (false, 0, 0),
                                Ok(None) => {}
                            }
                            triangulation.insertion = None;
                            (1, 0)
                        } else {
                            match close_vec_owner_step(&mut triangulation.points, maximum_bytes) {
                                Ok(Some(step)) => return (false, step.0, step.1),
                                Err(()) => return (false, 0, 0),
                                Ok(None) => {}
                            }
                            match close_vec_owner_step(&mut triangulation.triangles, maximum_bytes) {
                                Ok(Some(step)) => return (false, step.0, step.1),
                                Err(()) => return (false, 0, 0),
                                Ok(None) => {}
                            }
                            match close_vec_owner_step(&mut triangulation.insertion_order, maximum_bytes) {
                                Ok(Some(step)) => return (false, step.0, step.1),
                                Err(()) => return (false, 0, 0),
                                Ok(None) => {}
                            }
                            self.triangulation = None;
                            (1, 0)
                        }
                    } else {
                        self.close_lane += 1;
                        continue;
                    }
                }
                5 => match close_vec_owner_step(&mut self.constraints, maximum_bytes) {
                    Ok(Some(step)) => step,
                    Err(()) => return (false, 0, 0),
                    Ok(None) => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                6 => match close_vec_owner_step(&mut self.fixed_constraints, maximum_bytes) {
                    Ok(Some(step)) => step,
                    Err(()) => return (false, 0, 0),
                    Ok(None) => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                7 => match close_vec_owner_step(&mut self.indexed_edges, maximum_bytes) {
                    Ok(Some(step)) => step,
                    Err(()) => return (false, 0, 0),
                    Ok(None) => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                8 => match close_vec_owner_step(&mut self.mesh.points, maximum_bytes) {
                    Ok(Some(step)) => step,
                    Err(()) => return (false, 0, 0),
                    Ok(None) => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                9 => match close_vec_owner_step(&mut self.mesh.tris, maximum_bytes) {
                    Ok(Some(step)) => step,
                    Err(()) => return (false, 0, 0),
                    Ok(None) => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                10 => match close_vec_owner_step(&mut self.point_index, maximum_bytes) {
                    Ok(Some(step)) => step,
                    Err(()) => return (false, 0, 0),
                    Ok(None) => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                11 => match close_vec_owner_step(&mut self.indexed_constraint_edges, maximum_bytes) {
                    Ok(Some(step)) => step,
                    Err(()) => return (false, 0, 0),
                    Ok(None) => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                _ => return (true, 0, 0),
            };
            return (false, released.0, released.1);
        }
    }

    /// 🗺️ The latest complete replaceable overlay; authoritative callers commit only the final outcome.
    pub fn preview(&self) -> MeshJobPreview {
        MeshJobPreview { sequence: self.operation.preview_sequence, tier: self.preview_tier, refinement_steps: self.refinement_steps, mesh: self.mesh.clone() }
    }

    /// 🧵️ Moves the completed mesh into its retained model-construction child. A false
    /// terminal leaves the exact owner in this job.
    pub fn take_completed_mesh(&mut self) -> Option<TriMesh2> {
        (self.stage == MeshJobStage::Published).then(|| std::mem::replace(&mut self.mesh, TriMesh2 { points: Vec::new(), tris: Vec::new() }))
    }

    fn begin_classification(&mut self, tier: MeshQualityTier) {
        self.face_cursor = 0;
        self.face_classification = None;
        self.preview_tier = tier;
        self.stage = MeshJobStage::Classify;
    }

    fn advance_polygon_edge(point: [f64; 2], polygon: &[[f64; 2]], edge: usize, inside: &mut bool) -> bool {
        if edge == polygon.len() {
            return true;
        }
        let current = polygon[edge];
        let previous = polygon[(edge + polygon.len() - 1) % polygon.len()];
        if ((current[1] > point[1]) != (previous[1] > point[1])) && point[0] < (previous[0] - current[0]) * (point[1] - current[1]) / (previous[1] - current[1]) + current[0] {
            *inside = !*inside;
        }
        false
    }

    fn advance_face_classification(&mut self) -> Result<bool, ()> {
        if self.face_classification.is_none() {
            let triangulation = self.triangulation.as_ref().ok_or(())?;
            let Some(triangle) = triangulation.triangles.get(self.face_cursor) else { return Ok(true) };
            let positions = std::array::from_fn(|index| triangulation.points[triangle[index]]);
            let centroid = [(positions[0][0] + positions[1][0] + positions[2][0]) / 3.0, (positions[0][1] + positions[1][1] + positions[2][1]) / 3.0];
            self.face_classification = Some(FaceClassificationCursor { stage: FaceClassificationStage::Begin, positions, centroid, hole: 0, edge: 0, inside: false, slot: 0, lookup: 0, indices: [0; 3] });
            return Ok(false);
        }
        let cursor = self.face_classification.as_mut().ok_or(())?;
        match cursor.stage {
            FaceClassificationStage::Begin => cursor.stage = FaceClassificationStage::OuterEdge,
            FaceClassificationStage::OuterEdge => {
                if Self::advance_polygon_edge(cursor.centroid, self.domain.outer(), cursor.edge, &mut cursor.inside) {
                    if !cursor.inside {
                        self.face_classification = None;
                        self.face_cursor += 1;
                        return Ok(false);
                    }
                    cursor.edge = 0;
                    cursor.inside = false;
                    cursor.stage = FaceClassificationStage::HoleEdge;
                } else {
                    cursor.edge += 1;
                }
            }
            FaceClassificationStage::HoleEdge => {
                if let Some(hole) = self.domain.hole(cursor.hole) {
                    if Self::advance_polygon_edge(cursor.centroid, hole, cursor.edge, &mut cursor.inside) {
                        if cursor.inside {
                            self.face_classification = None;
                            self.face_cursor += 1;
                            return Ok(false);
                        }
                        cursor.hole += 1;
                        cursor.edge = 0;
                        cursor.inside = false;
                    } else {
                        cursor.edge += 1;
                    }
                } else {
                    cursor.stage = FaceClassificationStage::PointLookup;
                }
            }
            FaceClassificationStage::PointLookup => {
                let position = cursor.positions[cursor.slot];
                let key = (position[0].to_bits(), position[1].to_bits());
                if let Some((current, index)) = self.point_index.get(cursor.lookup) {
                    if *current == key {
                        cursor.indices[cursor.slot] = *index;
                        cursor.slot += 1;
                        cursor.lookup = 0;
                    } else {
                        cursor.lookup += 1;
                    }
                } else {
                    cursor.stage = FaceClassificationStage::PointInsert;
                }
                if cursor.slot == 3 {
                    cursor.stage = FaceClassificationStage::TrianglePublish;
                }
            }
            FaceClassificationStage::PointInsert => {
                if self.mesh.points.len() == self.maximum_points {
                    return Err(());
                }
                let position = cursor.positions[cursor.slot];
                let key = (position[0].to_bits(), position[1].to_bits());
                self.mesh.points.push(position);
                let index = (self.mesh.points.len() - 1) as u32;
                self.point_index.push((key, index));
                cursor.indices[cursor.slot] = index;
                cursor.slot += 1;
                cursor.lookup = 0;
                cursor.stage = if cursor.slot == 3 { FaceClassificationStage::TrianglePublish } else { FaceClassificationStage::PointLookup };
            }
            FaceClassificationStage::TrianglePublish => {
                if self.mesh.tris.len() == self.maximum_triangles {
                    return Err(());
                }
                self.mesh.tris.push(cursor.indices);
                self.face_classification = None;
                self.face_cursor += 1;
            }
        }
        Ok(false)
    }

    fn advance_mesh_payload(&mut self) -> Result<bool, JobPayloadAdmissionFault> {
        let writer = self.publication_writer.as_mut().ok_or(JobPayloadAdmissionFault::WriterSealed)?;
        let mut scalar = [0; 8];
        let length = match self.publication_cursor.stage {
            MeshPayloadStage::Magic => {
                scalar.copy_from_slice(b"FEMMESH1");
                Some(8)
            }
            MeshPayloadStage::Tier => {
                scalar[0] = match self.preview_tier {
                    MeshQualityTier::Coarse => 0,
                    MeshQualityTier::Refined => 1,
                    MeshQualityTier::Final => 2,
                };
                Some(1)
            }
            MeshPayloadStage::Complete => {
                scalar[0] = u8::from(self.publication_kind != Some(MeshPublicationKind::Preview));
                Some(1)
            }
            MeshPayloadStage::Sequence => {
                scalar = self.publication_sequence.to_le_bytes();
                Some(8)
            }
            MeshPayloadStage::Refinement => {
                scalar = (self.refinement_steps as u64).to_le_bytes();
                Some(8)
            }
            MeshPayloadStage::PointCount => {
                scalar = (self.mesh.points.len() as u64).to_le_bytes();
                Some(8)
            }
            MeshPayloadStage::TriangleCount => {
                scalar = (self.mesh.tris.len() as u64).to_le_bytes();
                Some(8)
            }
            MeshPayloadStage::PointCoordinate => self.mesh.points.get(self.publication_cursor.point).map(|point| {
                scalar = point[self.publication_cursor.coordinate].to_bits().to_le_bytes();
                8
            }),
            MeshPayloadStage::TriangleIndex => self.mesh.tris.get(self.publication_cursor.triangle).map(|triangle| {
                scalar[..4].copy_from_slice(&triangle[self.publication_cursor.index].to_le_bytes());
                4
            }),
            MeshPayloadStage::CommitPage | MeshPayloadStage::Done => None,
        };
        if let Some(length) = length {
            if writer.staged_page_remaining() < length {
                writer.commit_staged_page()?;
                return Ok(false);
            }
            writer.write_staged(&scalar[..length])?;
        }
        self.publication_cursor.stage = match self.publication_cursor.stage {
            MeshPayloadStage::Magic => MeshPayloadStage::Tier,
            MeshPayloadStage::Tier => MeshPayloadStage::Complete,
            MeshPayloadStage::Complete => MeshPayloadStage::Sequence,
            MeshPayloadStage::Sequence => MeshPayloadStage::Refinement,
            MeshPayloadStage::Refinement => MeshPayloadStage::PointCount,
            MeshPayloadStage::PointCount => MeshPayloadStage::TriangleCount,
            MeshPayloadStage::TriangleCount if self.mesh.points.is_empty() => MeshPayloadStage::TriangleIndex,
            MeshPayloadStage::TriangleCount => MeshPayloadStage::PointCoordinate,
            MeshPayloadStage::PointCoordinate => {
                self.publication_cursor.coordinate += 1;
                if self.publication_cursor.coordinate == 2 {
                    self.publication_cursor.coordinate = 0;
                    self.publication_cursor.point += 1;
                }
                if self.publication_cursor.point == self.mesh.points.len() {
                    MeshPayloadStage::TriangleIndex
                } else {
                    MeshPayloadStage::PointCoordinate
                }
            }
            MeshPayloadStage::TriangleIndex => {
                if self.publication_cursor.triangle == self.mesh.tris.len() {
                    MeshPayloadStage::CommitPage
                } else {
                    self.publication_cursor.index += 1;
                    if self.publication_cursor.index == 3 {
                        self.publication_cursor.index = 0;
                        self.publication_cursor.triangle += 1;
                    }
                    if self.publication_cursor.triangle == self.mesh.tris.len() {
                        MeshPayloadStage::CommitPage
                    } else {
                        MeshPayloadStage::TriangleIndex
                    }
                }
            }
            MeshPayloadStage::CommitPage => {
                writer.commit_staged_page()?;
                MeshPayloadStage::Done
            }
            MeshPayloadStage::Done => MeshPayloadStage::Done,
        };
        Ok(self.publication_cursor.stage == MeshPayloadStage::Done)
    }

    fn advance_publication(&mut self, context: &mut StepContext<'_>, kind: MeshPublicationKind) -> Result<Option<RetainedJobPayload>, ()> {
        if self.publication_kind.is_none() {
            let sequence = if kind == MeshPublicationKind::Preview {
                let sequence = context.next_preview_sequence().map_err(|_| ())?;
                self.operation.preview_sequence = sequence + 1;
                sequence
            } else {
                self.operation.preview_sequence
            };
            self.publication_kind = Some(kind);
            self.publication_sequence = sequence;
            self.publication_cursor = MeshPayloadCursor { stage: MeshPayloadStage::Magic, point: 0, coordinate: 0, triangle: 0, index: 0 };
            return Ok(None);
        }
        if self.publication_kind != Some(kind) {
            return Err(());
        }
        if self.publication_writer.is_none() {
            let stream = match kind {
                MeshPublicationKind::Preview => JobPayloadStream::Preview,
                MeshPublicationKind::Checkpoint => JobPayloadStream::CheckpointState,
                MeshPublicationKind::Complete => JobPayloadStream::CommitOutput,
            };
            self.publication_writer = Some(RetainedJobPayloadWriter::new(stream));
            return Ok(None);
        }
        let writer = self.publication_writer.as_mut().ok_or(())?;
        if writer.staged_page_len().is_none() && self.publication_cursor.stage != MeshPayloadStage::Done {
            writer.begin_staged_page(context).map_err(|_| ())?;
            return Ok(None);
        }
        if !self.advance_mesh_payload().map_err(|_| ())? {
            return Ok(None);
        }
        let writer = self.publication_writer.take().ok_or(())?;
        let payload = writer.finish().map_err(|writer| {
            self.publication_writer = Some(writer);
        })?;
        self.publication_kind = None;
        Ok(Some(payload))
    }

    fn fail(message: impl Into<Vec<u8>>) -> StepOutcome {
        StepOutcome::Fault(JobFault { detail: message.into() })
    }

    fn begin_edge_index_candidate(&mut self, triangle: usize, local: usize) -> bool {
        let Some(indices) = self.triangulation.as_ref().and_then(|triangulation| triangulation.triangles.get(triangle)).copied() else {
            return false;
        };
        let edge = match local {
            0 => Edge::new(indices[0], indices[1]),
            1 => Edge::new(indices[1], indices[2]),
            _ => Edge::new(indices[2], indices[0]),
        };
        self.edge_index_candidate = Some((edge, triangle));
        self.edge_index_lookup_cursor = 0;
        self.edge_index_vacancy = None;
        true
    }

    fn advance_edge_index_candidate(&mut self) -> Result<bool, ()> {
        let Some((edge, triangle)) = self.edge_index_candidate else {
            return Ok(true);
        };
        if self.edge_index_lookup_cursor < self.indexed_constraint_edges.len() {
            let index = self.edge_index_lookup_cursor;
            let slot = &mut self.indexed_constraint_edges[index];
            self.edge_index_lookup_cursor += 1;
            if !slot.active {
                self.edge_index_vacancy.get_or_insert(index);
            } else if slot.edge == edge {
                if slot.adjacent[0] != Some(triangle) && slot.adjacent[1] != Some(triangle) {
                    if slot.adjacent[0].is_none() {
                        slot.adjacent[0] = Some(triangle);
                    } else if slot.adjacent[1].is_none() {
                        slot.adjacent[1] = Some(triangle);
                    } else {
                        return Err(());
                    }
                }
                self.edge_index_candidate = None;
                return Ok(true);
            }
            return Ok(false);
        }
        let entry = IndexedConstraintEdge { edge, adjacent: [Some(triangle), None], active: true, fixed: false };
        if let Some(index) = self.edge_index_vacancy {
            self.indexed_constraint_edges[index] = entry;
        } else if self.indexed_constraint_edges.len() < self.indexed_constraint_edges.capacity() {
            self.indexed_constraint_edges.push(entry);
        } else {
            return Err(());
        }
        self.edge_index_candidate = None;
        Ok(true)
    }

    fn advance_constraint_recovery(&mut self) -> Result<bool, MeshError> {
        let constraint = self.constraints[self.constraint_cursor];
        match self.constraint_stage {
            ConstraintRecoveryStage::ReserveConstraintWorkspace => {
                self.constraint_flip_limit = self.triangulation.as_ref().map_or(0, |triangulation| triangulation.triangles.len().saturating_mul(triangulation.triangles.len()).max(1));
                self.constraint_flip_count = 0;
                self.constraint_search_cursor = 0;
                self.constraint_stage = ConstraintRecoveryStage::SearchConstraintEdge;
            }
            ConstraintRecoveryStage::IndexTriangleEdge => {
                let adjacent = self.constraint_flip.map(|candidate| candidate.adjacent).ok_or_else(|| MeshError::TriangulationFailed("missing retained flip owner".into()))?;
                if self.edge_index_candidate.is_none() {
                    if self.constraint_reindex_cursor == 6 {
                        self.constraint_search_cursor = 0;
                        self.constraint_stage = ConstraintRecoveryStage::PublishConstraintProgress;
                        return Ok(false);
                    }
                    let triangle = adjacent[self.constraint_reindex_cursor / 3];
                    let local = self.constraint_reindex_cursor % 3;
                    if !self.begin_edge_index_candidate(triangle, local) {
                        return Err(MeshError::TriangulationFailed("missing flipped triangle".into()));
                    }
                } else if self.advance_edge_index_candidate().map_err(|_| MeshError::TriangulationFailed("constraint edge index capacity".into()))? {
                    self.constraint_reindex_cursor += 1;
                }
            }
            ConstraintRecoveryStage::SearchConstraintEdge => {
                if self.constraint_search_cursor >= self.indexed_constraint_edges.len() {
                    return Err(MeshError::TriangulationFailed(format!("could not recover boundary edge {}-{}", constraint.0, constraint.1)));
                }
                let index = self.constraint_search_cursor;
                let slot = self.indexed_constraint_edges[index];
                self.constraint_search_cursor += 1;
                if slot.active && slot.edge == constraint {
                    self.indexed_constraint_edges[index].fixed = true;
                    self.constraint_stage = ConstraintRecoveryStage::ConstraintComplete;
                } else if slot.active && slot.adjacent[1].is_some() && !slot.fixed && slot.edge.0 != constraint.0 && slot.edge.0 != constraint.1 && slot.edge.1 != constraint.0 && slot.edge.1 != constraint.1 {
                    self.constraint_candidate = Some((slot.edge, self.constraint_search_cursor - 1));
                    self.constraint_adjacency_cursor = 0;
                    self.constraint_stage = ConstraintRecoveryStage::ClassifyIntersection;
                }
            }
            ConstraintRecoveryStage::ClassifyIntersection => {
                let (edge, index) = self.constraint_candidate.ok_or_else(|| MeshError::TriangulationFailed("missing constraint candidate".into()))?;
                let points = &self.triangulation.as_ref().ok_or_else(|| MeshError::TriangulationFailed("missing triangulation".into()))?.points;
                if segments_cross(points[constraint.0], points[constraint.1], points[edge.0], points[edge.1]) {
                    self.constraint_adjacency_cursor = index;
                    self.constraint_stage = ConstraintRecoveryStage::SelectDeterministicFlip;
                } else {
                    self.constraint_candidate = None;
                    self.constraint_stage = ConstraintRecoveryStage::SearchConstraintEdge;
                }
            }
            ConstraintRecoveryStage::SelectDeterministicFlip => {
                let (edge, index) = self.constraint_candidate.ok_or_else(|| MeshError::TriangulationFailed("missing selected constraint edge".into()))?;
                let slot = self.indexed_constraint_edges[index];
                let adjacent = match slot.adjacent {
                    [Some(first), Some(second)] => [first, second],
                    _ => return Err(MeshError::TriangulationFailed("constraint edge adjacency".into())),
                };
                self.constraint_flip = Some(ConstraintFlipCandidate { edge, adjacent, replacement: [[0; 3]; 2] });
                self.constraint_stage = ConstraintRecoveryStage::ValidateFlip;
            }
            ConstraintRecoveryStage::ValidateFlip => {
                let mut candidate = self.constraint_flip.ok_or_else(|| MeshError::TriangulationFailed("missing flip candidate".into()))?;
                let triangulation = self.triangulation.as_ref().ok_or_else(|| MeshError::TriangulationFailed("missing triangulation".into()))?;
                let [a, b] = [candidate.edge.0, candidate.edge.1];
                let first = triangulation.triangles[candidate.adjacent[0]];
                let second = triangulation.triangles[candidate.adjacent[1]];
                let c = first.into_iter().find(|index| *index != a && *index != b).ok_or_else(|| MeshError::TriangulationFailed("first opposite vertex".into()))?;
                let d = second.into_iter().find(|index| *index != a && *index != b).ok_or_else(|| MeshError::TriangulationFailed("second opposite vertex".into()))?;
                if !segments_cross(triangulation.points[a], triangulation.points[b], triangulation.points[c], triangulation.points[d]) {
                    return Err(MeshError::TriangulationFailed("unflippable constraint".into()));
                }
                candidate.replacement[0] = ccw_triangle(&triangulation.points, [c, d, a]).ok_or_else(|| MeshError::TriangulationFailed("invalid first flip".into()))?;
                candidate.replacement[1] = ccw_triangle(&triangulation.points, [d, c, b]).ok_or_else(|| MeshError::TriangulationFailed("invalid second flip".into()))?;
                self.constraint_flip = Some(candidate);
                self.constraint_apply_cursor = 0;
                self.constraint_stage = ConstraintRecoveryStage::ApplyFlip;
            }
            ConstraintRecoveryStage::ApplyFlip => {
                let candidate = self.constraint_flip.ok_or_else(|| MeshError::TriangulationFailed("missing validated flip".into()))?;
                let index = candidate.adjacent[self.constraint_apply_cursor];
                self.triangulation.as_mut().ok_or_else(|| MeshError::TriangulationFailed("missing triangulation".into()))?.triangles[index] = candidate.replacement[self.constraint_apply_cursor];
                self.constraint_apply_cursor += 1;
                if self.constraint_apply_cursor == 2 {
                    self.constraint_flip_count += 1;
                    if self.constraint_flip_count > self.constraint_flip_limit {
                        return Err(MeshError::TriangulationFailed("constraint flip limit".into()));
                    }
                    self.constraint_retire_cursor = 0;
                    self.constraint_retire_adjacency_cursor = 0;
                    self.constraint_stage = ConstraintRecoveryStage::RetireFormerEdge;
                }
            }
            ConstraintRecoveryStage::RetireFormerEdge => {
                if self.constraint_retire_cursor < self.indexed_constraint_edges.len() {
                    let adjacent = self.constraint_flip.map(|candidate| candidate.adjacent).ok_or_else(|| MeshError::TriangulationFailed("missing retire authority".into()))?;
                    let slot = &mut self.indexed_constraint_edges[self.constraint_retire_cursor];
                    if self.constraint_retire_adjacency_cursor < 2 {
                        let owner = &mut slot.adjacent[self.constraint_retire_adjacency_cursor];
                        if owner.is_some_and(|triangle| adjacent[0] == triangle || adjacent[1] == triangle) {
                            *owner = None;
                        }
                        self.constraint_retire_adjacency_cursor += 1;
                    } else {
                        slot.active = slot.adjacent[0].is_some() || slot.adjacent[1].is_some();
                        self.constraint_retire_cursor += 1;
                        self.constraint_retire_adjacency_cursor = 0;
                    }
                } else {
                    self.constraint_reindex_cursor = 0;
                    self.edge_index_candidate = None;
                    self.constraint_stage = ConstraintRecoveryStage::IndexTriangleEdge;
                }
            }
            ConstraintRecoveryStage::PublishConstraintProgress => {
                self.constraint_candidate = None;
                self.constraint_search_cursor = 0;
                self.constraint_stage = ConstraintRecoveryStage::SearchConstraintEdge;
            }
            ConstraintRecoveryStage::ConstraintComplete => return Ok(true),
        }
        Ok(false)
    }
}

impl InteractiveJob for MeshJob {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return Self::fail(b"stale-mesh-operation".to_vec());
        }
        context.set_stage(match self.stage {
            MeshJobStage::Validate => "validate-references",
            MeshJobStage::ReservePreparation => "reserve-preparation",
            MeshJobStage::PrepareInput => "prepare-input",
            MeshJobStage::CountInput => "count-input",
            MeshJobStage::Initialize => "initialize-triangulation",
            MeshJobStage::ReserveEdgeAuthorities => "reserve-edge-authorities",
            MeshJobStage::InsertBoundary => "insert-boundary",
            MeshJobStage::IndexEdges => "index-edges",
            MeshJobStage::ConstrainBoundary => match self.constraint_stage {
                ConstraintRecoveryStage::ReserveConstraintWorkspace => "reserve-constraint-workspace",
                ConstraintRecoveryStage::IndexTriangleEdge => "index-triangle-edge",
                ConstraintRecoveryStage::SearchConstraintEdge => "search-constraint-edge",
                ConstraintRecoveryStage::ClassifyIntersection => "classify-intersection",
                ConstraintRecoveryStage::SelectDeterministicFlip => "select-deterministic-flip",
                ConstraintRecoveryStage::ValidateFlip => "validate-flip",
                ConstraintRecoveryStage::ApplyFlip => "apply-flip",
                ConstraintRecoveryStage::RetireFormerEdge => "retire-former-edge",
                ConstraintRecoveryStage::PublishConstraintProgress => "publish-constraint-progress",
                ConstraintRecoveryStage::ConstraintComplete => "constraint-complete",
            },
            MeshJobStage::ReservePointIndex => "reserve-point-index",
            MeshJobStage::ReserveMeshPoints => "reserve-mesh-points",
            MeshJobStage::ReserveMeshTriangles => "reserve-mesh-triangles",
            MeshJobStage::Classify => "classify-elements",
            MeshJobStage::PublishPreview => "publish-preview-entry",
            MeshJobStage::Finalize => "finalize-mesh",
            MeshJobStage::PublishCheckpoint => "publish-checkpoint-entry",
            MeshJobStage::Complete => "publish-complete-entry",
            MeshJobStage::Published => "complete",
        });
        if context.should_yield() {
            return StepOutcome::Yield;
        }
        context.consume_fuel(1);
        match self.stage {
            MeshJobStage::Validate => {
                if self.domain.outer().len() < 3 {
                    return Self::fail(MeshError::DegenerateDomain.to_string().into_bytes());
                }
                if let Some(hole) = self.domain.hole(self.validation_hole_cursor) {
                    if hole.len() < 3 {
                        return Self::fail(MeshError::DegenerateDomain.to_string().into_bytes());
                    }
                    self.validation_hole_cursor += 1;
                    return StepOutcome::Yield;
                }
                self.preparation = Some(MeshInputPreparation::new());
                self.stage = MeshJobStage::ReservePreparation;
                StepOutcome::Yield
            }
            MeshJobStage::ReservePreparation => {
                let preparation = self.preparation.as_mut().expect("input preparation initialized");
                if self.maximum_points != usize::MAX {
                    let maximum_constraints = self.maximum_triangles.saturating_mul(3);
                    if preparation.points.try_reserve_exact(self.maximum_points).is_err()
                        || preparation.point_indices.try_reserve_exact(self.maximum_points).is_err()
                        || preparation.constraints.try_reserve_exact(maximum_constraints).is_err()
                        || preparation.points.capacity().checked_mul(std::mem::size_of::<[f64; 2]>()).is_none_or(|bytes| bytes > 4_096)
                        || preparation.point_indices.capacity().checked_mul(std::mem::size_of::<((u64, u64), usize)>()).is_none_or(|bytes| bytes > 4_096)
                        || preparation.constraints.capacity().checked_mul(std::mem::size_of::<Edge>()).is_none_or(|bytes| bytes > 4_096)
                    {
                        return Self::fail(b"mesh-fixed-preparation-backing".to_vec());
                    }
                }
                self.stage = MeshJobStage::PrepareInput;
                StepOutcome::Yield
            }
            MeshJobStage::PrepareInput => {
                let complete = match self.preparation.as_mut().expect("input preparation initialized").advance(&self.domain, &self.options) {
                    Ok(complete) => complete,
                    Err(error) => return Self::fail(error.to_string().into_bytes()),
                };
                let preparation = self.preparation.as_ref().expect("input preparation retained");
                if preparation.points.len() > self.maximum_points || preparation.constraints.len() > self.maximum_triangles.saturating_mul(3) {
                    return Self::fail(b"mesh-fixed-input-capacity".to_vec());
                }
                if complete {
                    let preparation = self.preparation.take().expect("input preparation complete");
                    self.prepared_input = Some((preparation.points, preparation.constraints));
                    self.input_count = self.domain.outer().len();
                    self.input_count_hole = 0;
                    self.stage = MeshJobStage::CountInput;
                }
                StepOutcome::Yield
            }
            MeshJobStage::CountInput => {
                if let Some(hole) = self.domain.hole(self.input_count_hole) {
                    self.input_count = match self.input_count.checked_add(hole.len()) {
                        Some(count) => count,
                        None => return Self::fail(b"mesh-input-count-overflow".to_vec()),
                    };
                    self.input_count_hole += 1;
                } else {
                    self.stage = MeshJobStage::Initialize;
                }
                StepOutcome::Yield
            }
            MeshJobStage::Initialize => {
                if self.triangulation.is_none() {
                    let (prepared_points, input_constraints) = self.prepared_input.take().expect("prepared input");
                    if prepared_points.len() > self.maximum_points {
                        self.prepared_input = Some((prepared_points, input_constraints));
                        return Self::fail(b"mesh-fixed-point-capacity".to_vec());
                    }
                    if self.maximum_points == usize::MAX {
                        let triangulation = match OwnedTriangulation::begin(prepared_points.clone()) {
                            Ok(triangulation) => triangulation,
                            Err(error) => return Self::fail(error.to_string().into_bytes()),
                        };
                        self.constraints = remap_constraints(&prepared_points, &triangulation.points[..triangulation.input_len], input_constraints);
                        self.refinement_steps = triangulation.input_len.saturating_sub(self.input_count);
                        self.triangulation = Some(triangulation);
                        self.stage = MeshJobStage::ReserveEdgeAuthorities;
                        return StepOutcome::Yield;
                    }
                    let triangulation = match OwnedTriangulation::begin_mounted(prepared_points, self.maximum_triangles) {
                        Ok(triangulation) => triangulation,
                        Err(error) => return Self::fail(error.to_string().into_bytes()),
                    };
                    self.constraints = input_constraints;
                    self.refinement_steps = triangulation.input_len.saturating_sub(self.input_count);
                    self.triangulation = Some(triangulation);
                    return StepOutcome::Yield;
                }
                let initialized = match self.triangulation.as_mut().expect("mounted triangulation shell retained").advance_mounted_initialization() {
                    Ok(initialized) => initialized,
                    Err(error) => return Self::fail(error.to_string().into_bytes()),
                };
                if initialized {
                    self.stage = MeshJobStage::ReserveEdgeAuthorities;
                }
                StepOutcome::Yield
            }
            MeshJobStage::ReserveEdgeAuthorities => {
                if self.maximum_triangles == usize::MAX {
                    self.stage = MeshJobStage::InsertBoundary;
                    return StepOutcome::Yield;
                }
                let edge_capacity = self.maximum_triangles.saturating_mul(12).saturating_add(3);
                if self.fixed_constraints.try_reserve_exact(self.maximum_triangles.saturating_mul(3)).is_err()
                    || self.indexed_edges.try_reserve_exact(edge_capacity).is_err()
                    || self.indexed_constraint_edges.try_reserve_exact(edge_capacity).is_err()
                    || self.fixed_constraints.capacity().checked_mul(std::mem::size_of::<Edge>()).is_none_or(|bytes| bytes > 4_096)
                    || self.indexed_edges.capacity().checked_mul(std::mem::size_of::<Edge>()).is_none_or(|bytes| bytes > 4_096)
                    || self.indexed_constraint_edges.capacity().checked_mul(std::mem::size_of::<IndexedConstraintEdge>()).is_none_or(|bytes| bytes > 16_384)
                {
                    return Self::fail(b"mesh-fixed-edge-authority-backing".to_vec());
                }
                self.stage = MeshJobStage::InsertBoundary;
                StepOutcome::Yield
            }
            MeshJobStage::InsertBoundary => {
                let triangulation = self.triangulation.as_mut().expect("validated triangulation");
                if triangulation.insert_cursor < triangulation.input_len {
                    triangulation.insert_next();
                } else if triangulation.advance_finish_insertion() {
                    self.stage = MeshJobStage::IndexEdges;
                }
                if triangulation.allocation_fault {
                    return Self::fail(b"mesh-fixed-triangulation-workspace-backing".to_vec());
                }
                if triangulation.triangles.len() > self.maximum_triangles.saturating_mul(4).saturating_add(1) {
                    return Self::fail(b"mesh-fixed-triangulation-capacity".to_vec());
                }
                StepOutcome::Yield
            }
            MeshJobStage::IndexEdges => {
                let face_count = self.triangulation.as_ref().map_or(0, |triangulation| triangulation.triangles.len());
                if self.edge_index_candidate.is_none() {
                    if self.edge_index_cursor < face_count {
                        if !self.begin_edge_index_candidate(self.edge_index_cursor, self.edge_index_local_cursor) {
                            return Self::fail(b"mesh-edge-index-triangle".to_vec());
                        }
                    }
                } else {
                    match self.advance_edge_index_candidate() {
                        Ok(true) => {
                            self.edge_index_local_cursor += 1;
                            if self.edge_index_local_cursor == 3 {
                                self.edge_index_local_cursor = 0;
                                self.edge_index_cursor += 1;
                            }
                        }
                        Ok(false) => {}
                        Err(()) => return Self::fail(b"mesh-edge-index-capacity".to_vec()),
                    }
                }
                if self.edge_index_cursor == face_count {
                    self.edge_index_valid = true;
                    self.constraint_stage = ConstraintRecoveryStage::ReserveConstraintWorkspace;
                    self.stage = MeshJobStage::ConstrainBoundary;
                }
                StepOutcome::Yield
            }
            MeshJobStage::ConstrainBoundary => {
                if self.constraint_cursor < self.constraints.len() {
                    match self.advance_constraint_recovery() {
                        Ok(true) => {
                            self.constraint_cursor += 1;
                            self.constraint_stage = ConstraintRecoveryStage::ReserveConstraintWorkspace;
                        }
                        Ok(false) => {}
                        Err(error) => return Self::fail(error.to_string().into_bytes()),
                    }
                }
                if self.constraint_cursor == self.constraints.len() {
                    if self.maximum_points == usize::MAX {
                        self.begin_classification(if self.refinement_steps == 0 { MeshQualityTier::Coarse } else { MeshQualityTier::Final });
                    } else {
                        self.stage = MeshJobStage::ReservePointIndex;
                    }
                }
                StepOutcome::Yield
            }
            MeshJobStage::ReservePointIndex => {
                if self.point_index.try_reserve_exact(self.maximum_points).is_err() || self.point_index.capacity().checked_mul(std::mem::size_of::<((u64, u64), u32)>()).is_none_or(|bytes| bytes > 4_096) {
                    return Self::fail(b"mesh-fixed-point-index-backing".to_vec());
                }
                self.stage = MeshJobStage::ReserveMeshPoints;
                StepOutcome::Yield
            }
            MeshJobStage::ReserveMeshPoints => {
                if self.mesh.points.try_reserve_exact(self.maximum_points).is_err() || self.mesh.points.capacity().checked_mul(std::mem::size_of::<[f64; 2]>()).is_none_or(|bytes| bytes > 4_096) {
                    return Self::fail(b"mesh-fixed-point-backing".to_vec());
                }
                self.stage = MeshJobStage::ReserveMeshTriangles;
                StepOutcome::Yield
            }
            MeshJobStage::ReserveMeshTriangles => {
                if self.mesh.tris.try_reserve_exact(self.maximum_triangles).is_err() || self.mesh.tris.capacity().checked_mul(std::mem::size_of::<[u32; 3]>()).is_none_or(|bytes| bytes > 4_096) {
                    return Self::fail(b"mesh-fixed-triangle-backing".to_vec());
                }
                self.begin_classification(if self.refinement_steps == 0 { MeshQualityTier::Coarse } else { MeshQualityTier::Final });
                StepOutcome::Yield
            }
            MeshJobStage::Classify => {
                let face_count = self.triangulation.as_ref().expect("constrained triangulation").triangles.len();
                if self.face_cursor >= face_count {
                    self.stage = MeshJobStage::PublishPreview;
                    return StepOutcome::Yield;
                }
                if self.advance_face_classification().is_err() {
                    return Self::fail(b"mesh-fixed-output-capacity".to_vec());
                }
                StepOutcome::Yield
            }
            MeshJobStage::PublishPreview => match self.advance_publication(context, MeshPublicationKind::Preview) {
                Ok(Some(preview)) => {
                    self.stage = MeshJobStage::Finalize;
                    StepOutcome::PreviewReady(preview)
                }
                Ok(None) => StepOutcome::Yield,
                Err(()) => Self::fail(b"mesh-preview-publication".to_vec()),
            },
            MeshJobStage::Finalize => {
                self.preview_tier = MeshQualityTier::Final;
                self.stage = MeshJobStage::PublishCheckpoint;
                StepOutcome::Yield
            }
            MeshJobStage::PublishCheckpoint => match self.advance_publication(context, MeshPublicationKind::Checkpoint) {
                Ok(Some(state)) => {
                    self.stage = MeshJobStage::Complete;
                    StepOutcome::CheckpointReady(Checkpoint { applied_progress: self.mesh.tris.len() as u64, state })
                }
                Ok(None) => StepOutcome::Yield,
                Err(()) => Self::fail(b"mesh-checkpoint-publication".to_vec()),
            },
            MeshJobStage::Complete => match self.advance_publication(context, MeshPublicationKind::Complete) {
                Ok(Some(output)) => {
                    self.stage = MeshJobStage::Published;
                    StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output })
                }
                Ok(None) => StepOutcome::Yield,
                Err(()) => Self::fail(b"mesh-output-publication".to_vec()),
            },
            MeshJobStage::Published => StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output: RetainedJobPayload::empty(JobPayloadStream::CommitOutput) }),
        }
    }

    fn begin_close(&mut self) {
        if let Some(writer) = self.publication_writer.as_mut() {
            writer.begin_close();
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        let (complete, released_items, released_bytes) = MeshJob::close_step(self, maximum_bytes);
        if complete {
            semio_framework_job::InteractiveJobCloseStep::Complete
        } else {
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes }
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.publication_writer.is_none() && self.close_lane > 11
    }
}
// #endregion 🧵️IncrementalMeshJob

// #region 🔖️QuadMesh2
/// 🔲️ A structured quad mesh: shared node positions plus quads as index quadruples into `points`.
#[derive(Clone, Debug, PartialEq)]
pub struct QuadMesh2 {
    pub points: Vec<[f64; 2]>,
    pub quads: Vec<[u32; 4]>,
}

/// 🔲️ An `nx` x `ny` structured grid of quads over an axis-aligned rectangle `[x0,x1] x [y0,y1]`,
/// row-major point numbering, each quad wound `[bottom-left, bottom-right, top-right, top-left]`.
pub fn quad_grid(x0: f64, y0: f64, x1: f64, y1: f64, nx: usize, ny: usize) -> QuadMesh2 {
    let mut points = Vec::with_capacity((nx + 1) * (ny + 1));
    for j in 0..=ny {
        for i in 0..=nx {
            let x = x0 + (x1 - x0) * (i as f64) / (nx as f64);
            let y = y0 + (y1 - y0) * (j as f64) / (ny as f64);
            points.push([x, y]);
        }
    }
    let index = |i: usize, j: usize| (j * (nx + 1) + i) as u32;
    let mut quads = Vec::with_capacity(nx * ny);
    for j in 0..ny {
        for i in 0..nx {
            quads.push([index(i, j), index(i + 1, j), index(i + 1, j + 1), index(i, j + 1)]);
        }
    }
    QuadMesh2 { points, quads }
}
// #endregion 🔖️QuadMesh2

// #region 🔖️Quadratic
/// 🔺️ A quadratic-promoted triangle mesh: shared node positions (originals first, then appended
/// mid-edge points) plus 6-node triangles.
#[derive(Clone, Debug, PartialEq)]
pub struct TriMesh2Quadratic {
    pub points: Vec<[f64; 2]>,
    pub tris6: Vec<[u32; 6]>,
}

/// 🔗️ Looks up (or creates, welding shared edges to exactly one mid-node) the mid-edge point index
/// for edge `(a,b)`, keyed by the sorted `(min,max)` index pair.
fn mid_index(a: u32, b: u32, points: &mut Vec<[f64; 2]>, edge_mid: &mut HashMap<(u32, u32), u32>) -> u32 {
    let key = if a < b { (a, b) } else { (b, a) };
    if let Some(&idx) = edge_mid.get(&key) {
        return idx;
    }
    let pa = points[a as usize];
    let pb = points[b as usize];
    let idx = points.len() as u32;
    points.push([(pa[0] + pb[0]) * 0.5, (pa[1] + pb[1]) * 0.5]);
    edge_mid.insert(key, idx);
    idx
}

/// 🔺️ Promotes a linear `TriMesh2` to quadratic by inserting a mid-edge node per unique edge (shared
/// edges between adjacent triangles get exactly ONE mid-node, deduped by sorted `(min,max)` edge key).
/// Original points keep their indices unchanged; mid-edge points are appended after them. Each
/// triangle's 6 node indices follow `[n0,n1,n2, mid(n0,n1), mid(n1,n2), mid(n2,n0)]` — the standard
/// Tri6 convention (matches `elements2d.rs`'s `shape_tri6` node ordering, documented here since that
/// function may land concurrently with this module).
pub fn to_quadratic(mesh: &TriMesh2) -> TriMesh2Quadratic {
    let mut points = mesh.points.clone();
    let mut edge_mid: HashMap<(u32, u32), u32> = HashMap::new();
    let mut tris6 = Vec::with_capacity(mesh.tris.len());

    for tri in &mesh.tris {
        let [n0, n1, n2] = *tri;
        let m01 = mid_index(n0, n1, &mut points, &mut edge_mid);
        let m12 = mid_index(n1, n2, &mut points, &mut edge_mid);
        let m20 = mid_index(n2, n0, &mut points, &mut edge_mid);
        tris6.push([n0, n1, n2, m01, m12, m20]);
    }

    TriMesh2Quadratic { points, tris6 }
}
// #endregion 🔖️Quadratic

// #region 🔖️VolumeMesh
/// 🧱️ One volumetric cell — a linear wedge/hex prism or a tet, as index tuples into `VolumeMesh::points`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    Wedge6([u32; 6]),
    Hex8([u32; 8]),
    Tet4([u32; 4]),
}

/// 🧱️ A volumetric mesh: shared node positions plus cells.
#[derive(Clone, Debug, PartialEq)]
pub struct VolumeMesh {
    pub points: Vec<[f64; 3]>,
    pub cells: Vec<Cell>,
}

/// 🧱️ Extrudes a flat `TriMesh2` (lying in the z=0 plane) along +z by `height`, split into `layers`
/// equal-height layers, producing one `Wedge6` per (triangle, layer) — node order
/// `[bottom0,bottom1,bottom2, top0,top1,top2]` (bottom face matches the triangle's own `[n0,n1,n2]`
/// winding, top face directly above).
pub fn extrude_tri_mesh(mesh: &TriMesh2, height: f64, layers: usize) -> VolumeMesh {
    let layers = layers.max(1);
    let n = mesh.points.len();
    let mut points = Vec::with_capacity(n * (layers + 1));
    for l in 0..=layers {
        let z = height * (l as f64) / (layers as f64);
        for p in &mesh.points {
            points.push([p[0], p[1], z]);
        }
    }
    let mut cells = Vec::with_capacity(mesh.tris.len() * layers);
    for l in 0..layers {
        let bottom_off = (l * n) as u32;
        let top_off = ((l + 1) * n) as u32;
        for tri in &mesh.tris {
            let [a, b, c] = *tri;
            cells.push(Cell::Wedge6([bottom_off + a, bottom_off + b, bottom_off + c, top_off + a, top_off + b, top_off + c]));
        }
    }
    VolumeMesh { points, cells }
}

/// 🧱️ Extrudes a flat `QuadMesh2` along +z by `height` into `layers` layers of `Hex8` cells — node
/// order `[bottom0,bottom1,bottom2,bottom3, top0,top1,top2,top3]` matching the quad's own winding.
pub fn extrude_quad_mesh(mesh: &QuadMesh2, height: f64, layers: usize) -> VolumeMesh {
    let layers = layers.max(1);
    let n = mesh.points.len();
    let mut points = Vec::with_capacity(n * (layers + 1));
    for l in 0..=layers {
        let z = height * (l as f64) / (layers as f64);
        for p in &mesh.points {
            points.push([p[0], p[1], z]);
        }
    }
    let mut cells = Vec::with_capacity(mesh.quads.len() * layers);
    for l in 0..layers {
        let bottom_off = (l * n) as u32;
        let top_off = ((l + 1) * n) as u32;
        for quad in &mesh.quads {
            let [a, b, c, d] = *quad;
            cells.push(Cell::Hex8([bottom_off + a, bottom_off + b, bottom_off + c, bottom_off + d, top_off + a, top_off + b, top_off + c, top_off + d]));
        }
    }
    VolumeMesh { points, cells }
}

/// ✂️ Splits a quad face `[a,b,c,d]` (in winding order, so `a`-`c` and `b`-`d` are the two diagonals)
/// into 2 triangles, choosing the diagonal FROM the corner with the smallest global point index. This
/// depends only on the face's own 4 global indices (not on cell/apex choice), so two cells sharing a
/// quad face always agree — the parity-consistency guarantee `split_to_tets` relies on.
fn split_quad_face(a: u32, b: u32, c: u32, d: u32) -> [[u32; 3]; 2] {
    let min = a.min(b).min(c).min(d);
    if min == a || min == c {
        [[a, b, c], [a, c, d]]
    } else {
        [[a, b, d], [b, c, d]]
    }
}

/// 🔺️ Fan-triangulates a convex cell's boundary from `apex` (one of the cell's own vertices): every
/// quad face is split via [`split_quad_face`], every triangular face passes through as-is, and every
/// resulting boundary triangle that does NOT already contain `apex` becomes a tet `(apex, t0, t1, t2)`
/// — the standard star/cone decomposition of a convex polyhedron, valid since convexity guarantees no
/// overlap/gaps. Faces touching `apex` need no explicit tet: their volume is degenerate (zero) from
/// `apex`'s own cone and is instead captured as internal faces of tets from adjacent, non-apex faces.
fn split_cell_to_tets(quad_faces: &[[u32; 4]], tri_faces: &[[u32; 3]], apex: u32) -> Vec<[u32; 4]> {
    let mut tets = Vec::new();
    for &[a, b, c, d] in quad_faces {
        for tri in split_quad_face(a, b, c, d) {
            if !tri.contains(&apex) {
                tets.push([apex, tri[0], tri[1], tri[2]]);
            }
        }
    }
    for &tri in tri_faces {
        if !tri.contains(&apex) {
            tets.push([apex, tri[0], tri[1], tri[2]]);
        }
    }
    tets
}

/// 🔪️ Splits every `Wedge6`/`Hex8` cell into `Tet4` cells (`Wedge6` → 3 tets, `Hex8` → 6 tets), using
/// the minimum-global-node-index apex + face-diagonal rule so adjacent cells split their SHARED quad
/// faces identically (see [`split_quad_face`]). `Tet4` cells in the input pass through unchanged.
pub fn split_to_tets(mesh: &VolumeMesh) -> VolumeMesh {
    let mut cells = Vec::with_capacity(mesh.cells.len());
    for cell in &mesh.cells {
        match cell {
            Cell::Tet4(t) => cells.push(Cell::Tet4(*t)),
            Cell::Wedge6(w) => {
                let [n0, n1, n2, n3, n4, n5] = *w;
                let apex = w.iter().copied().min().unwrap();
                let quads = [[n0, n1, n4, n3], [n1, n2, n5, n4], [n2, n0, n3, n5]];
                let tris = [[n0, n1, n2], [n3, n4, n5]];
                for tet in split_cell_to_tets(&quads, &tris, apex) {
                    cells.push(Cell::Tet4(tet));
                }
            }
            Cell::Hex8(h) => {
                let [n0, n1, n2, n3, n4, n5, n6, n7] = *h;
                let apex = h.iter().copied().min().unwrap();
                let quads = [[n0, n1, n2, n3], [n4, n5, n6, n7], [n0, n1, n5, n4], [n1, n2, n6, n5], [n2, n3, n7, n6], [n3, n0, n4, n7]];
                for tet in split_cell_to_tets(&quads, &[], apex) {
                    cells.push(Cell::Tet4(tet));
                }
            }
        }
    }
    VolumeMesh { points: mesh.points.clone(), cells }
}

/// 🧭️ The average of `mesh.points` at `idxs` — shared by `boundary_faces`'s per-tet and per-face
/// centroid computations.
fn point_centroid(mesh: &VolumeMesh, idxs: &[u32]) -> [f64; 3] {
    let mut c = [0.0; 3];
    for &i in idxs {
        let p = mesh.points[i as usize];
        for k in 0..3 {
            c[k] += p[k];
        }
    }
    let n = idxs.len() as f64;
    [c[0] / n, c[1] / n, c[2] / n]
}

/// 🧱️ Every triangular face belonging to EXACTLY ONE `Tet4` cell — the mesh's outer surface (call
/// [`split_to_tets`] first if `mesh` still has `Wedge6`/`Hex8` cells; those contribute no faces here).
/// Each returned triangle is independently wound so its `cross(edge0,edge1)` normal points AWAY from
/// its own tet's centroid (outward) — determined per-tet via a centroid side-test, so the result
/// doesn't depend on any input node-order convention. Used by `fem_3d`'s solid mesh preview/rendering.
pub fn boundary_faces(mesh: &VolumeMesh) -> Vec<[u32; 3]> {
    let mut counts: HashMap<[u32; 3], usize> = HashMap::new();
    let mut oriented: HashMap<[u32; 3], [u32; 3]> = HashMap::new();

    for cell in &mesh.cells {
        let Cell::Tet4(t) = cell else { continue };
        let [n0, n1, n2, n3] = *t;
        let tet_centroid = point_centroid(mesh, &[n0, n1, n2, n3]);

        for face in [[n0, n1, n2], [n0, n1, n3], [n0, n2, n3], [n1, n2, n3]] {
            let mut key = face;
            key.sort_unstable();
            *counts.entry(key).or_insert(0) += 1;

            let p = |i: u32| mesh.points[i as usize];
            let (a, b, c) = (p(face[0]), p(face[1]), p(face[2]));
            let e0 = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
            let e1 = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
            let normal = [e0[1] * e1[2] - e0[2] * e1[1], e0[2] * e1[0] - e0[0] * e1[2], e0[0] * e1[1] - e0[1] * e1[0]];
            let face_centroid = point_centroid(mesh, &face);
            let to_tet = [tet_centroid[0] - face_centroid[0], tet_centroid[1] - face_centroid[1], tet_centroid[2] - face_centroid[2]];
            let dot = normal[0] * to_tet[0] + normal[1] * to_tet[1] + normal[2] * to_tet[2];
            let outward = if dot > 0.0 { [face[0], face[2], face[1]] } else { face };
            oriented.insert(key, outward);
        }
    }

    counts.into_iter().filter(|(_, count)| *count == 1).map(|(key, _)| oriented[&key]).collect()
}
// #endregion 🔖️VolumeMesh

// #region 🔖️Quality
/// 📊️ Cheap mesh sanity report — interior angle bounds (2D) and inverted-cell detection (3D).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QualityReport {
    pub min_angle_deg: f64,
    pub max_angle_deg: f64,
    pub min_jacobian_sign_positive: bool,
    pub element_count: usize,
}

/// 📐️ Interior angle at `p`, between the edges to `prev` and `next`, in degrees.
fn angle_at(prev: [f64; 2], p: [f64; 2], next: [f64; 2]) -> f64 {
    let v1 = [prev[0] - p[0], prev[1] - p[1]];
    let v2 = [next[0] - p[0], next[1] - p[1]];
    let dot = v1[0] * v2[0] + v1[1] * v2[1];
    let n1 = (v1[0] * v1[0] + v1[1] * v1[1]).sqrt();
    let n2 = (v2[0] * v2[0] + v2[1] * v2[1]).sqrt();
    let cos_a = (dot / (n1 * n2)).clamp(-1.0, 1.0);
    cos_a.acos().to_degrees()
}

/// 📊️ Min/max interior angle across all triangles; `min_jacobian_sign_positive` mirrors the 2D
/// analogue of the 3D check — true iff every triangle's signed area (shoelace, `[n0,n1,n2]` order) is
/// positive, i.e. consistently wound.
pub fn tri_mesh_quality(mesh: &TriMesh2) -> QualityReport {
    let mut min_angle = f64::INFINITY;
    let mut max_angle = f64::NEG_INFINITY;
    let mut all_positive = true;
    for tri in &mesh.tris {
        let p = [mesh.points[tri[0] as usize], mesh.points[tri[1] as usize], mesh.points[tri[2] as usize]];
        for i in 0..3 {
            let a = angle_at(p[(i + 2) % 3], p[i], p[(i + 1) % 3]);
            min_angle = min_angle.min(a);
            max_angle = max_angle.max(a);
        }
        let signed_area = 0.5 * ((p[1][0] - p[0][0]) * (p[2][1] - p[0][1]) - (p[2][0] - p[0][0]) * (p[1][1] - p[0][1]));
        if signed_area <= 0.0 {
            all_positive = false;
        }
    }
    if mesh.tris.is_empty() {
        min_angle = 0.0;
        max_angle = 0.0;
    }
    QualityReport { min_angle_deg: min_angle, max_angle_deg: max_angle, min_jacobian_sign_positive: all_positive, element_count: mesh.tris.len() }
}

/// 🧮️ Signed tet volume via the scalar triple product of edge vectors from `p0`.
fn tet_signed_volume(p0: [f64; 3], p1: [f64; 3], p2: [f64; 3], p3: [f64; 3]) -> f64 {
    let a = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
    let b = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
    let c = [p3[0] - p0[0], p3[1] - p0[1], p3[2] - p0[2]];
    let cross = [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
    (cross[0] * c[0] + cross[1] * c[1] + cross[2] * c[2]) / 6.0
}

/// 🧮️ Signed volume of one cell, via a FIXED (purely local-index-based, not global-min-based like
/// [`split_to_tets`]) tet decomposition — so the sign faithfully reflects whether the cell's own
/// documented local node order (`extrude_tri_mesh`/`extrude_quad_mesh`'s convention) is right-handed,
/// independent of the cell's global point indices. Verified by hand against a unit right prism and a
/// unit cube (both give the expected positive volume for correctly-ordered nodes).
fn cell_signed_volume(points: &[[f64; 3]], cell: &Cell) -> f64 {
    let p = |i: u32| points[i as usize];
    match cell {
        Cell::Tet4([a, b, c, d]) => tet_signed_volume(p(*a), p(*b), p(*c), p(*d)),
        Cell::Wedge6(n) => tet_signed_volume(p(n[0]), p(n[1]), p(n[2]), p(n[3])) + tet_signed_volume(p(n[1]), p(n[2]), p(n[3]), p(n[4])) + tet_signed_volume(p(n[2]), p(n[3]), p(n[4]), p(n[5])),
        Cell::Hex8(n) => {
            tet_signed_volume(p(n[0]), p(n[4]), p(n[5]), p(n[6]))
                + tet_signed_volume(p(n[0]), p(n[4]), p(n[6]), p(n[7]))
                + tet_signed_volume(p(n[0]), p(n[1]), p(n[2]), p(n[6]))
                + tet_signed_volume(p(n[0]), p(n[1]), p(n[6]), p(n[5]))
                + tet_signed_volume(p(n[0]), p(n[2]), p(n[3]), p(n[7]))
                + tet_signed_volume(p(n[0]), p(n[2]), p(n[7]), p(n[6]))
        }
    }
}

/// 📊️ `min_jacobian_sign_positive` is true iff every cell's signed volume is positive — a negative
/// signed volume flags inverted/degenerate connectivity. Angle bounds are a 2D-only concept and are
/// left at `0.0` here.
pub fn volume_mesh_quality(mesh: &VolumeMesh) -> QualityReport {
    let all_positive = mesh.cells.iter().all(|cell| cell_signed_volume(&mesh.points, cell) > 0.0);
    QualityReport { min_angle_deg: 0.0, max_angle_deg: 0.0, min_jacobian_sign_positive: all_positive, element_count: mesh.cells.len() }
}
// #endregion 🔖️Quality

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_job::{root_cancel_token, Generation, OperationId, RevisionId, StepBudget};
    use std::collections::HashSet;
    use std::time::{Duration, Instant};

    fn mesh_operation() -> Operation {
        Operation::new(OperationId(700), RevisionId(11), Generation(3), 19)
    }

    fn take_payload_bytes(mut payload: RetainedJobPayload) -> Vec<u8> {
        let mut bytes = Vec::new();
        for page in 0..payload.page_count() {
            bytes.extend_from_slice(payload.page(page).expect("retained mesh payload page"));
        }
        while !payload.terminal_is_empty() {
            let _ = payload.close_step(1, usize::MAX);
        }
        bytes
    }

    fn drive_mesh_job(mut job: MeshJob) -> (Vec<u8>, usize, Duration) {
        fn now() -> u64 {
            0
        }
        let cancel = root_cancel_token();
        let mut sequence = 0;
        let mut previews = 0;
        let mut worst = Duration::ZERO;
        for _ in 0..10_000_000 {
            let mut context = StepContext::new(job.operation.operation, job.operation.generation, StepBudget::new(64, 10), cancel.clone(), now, &mut sequence);
            let started = Instant::now();
            let outcome = job.step(&mut context);
            let elapsed = started.elapsed();
            worst = worst.max(elapsed);
            match outcome {
                StepOutcome::PreviewReady(preview) => {
                    previews += 1;
                    take_payload_bytes(preview);
                }
                StepOutcome::Complete(candidate) => {
                    take_payload_bytes(candidate.state);
                    return (take_payload_bytes(candidate.output), previews, worst);
                }
                StepOutcome::CheckpointReady(checkpoint) => {
                    take_payload_bytes(checkpoint.state);
                }
                StepOutcome::Yield => {}
                other => panic!("mesh job failed: {other:?}"),
            }
        }
        panic!("mesh job did not complete")
    }

    fn shoelace_area(points: &[[f64; 2]]) -> f64 {
        let mut sum = 0.0;
        for i in 0..points.len() {
            let a = points[i];
            let b = points[(i + 1) % points.len()];
            sum += a[0] * b[1] - b[0] * a[1];
        }
        (sum * 0.5).abs()
    }

    fn tri_area(mesh: &TriMesh2, tri: &[u32; 3]) -> f64 {
        shoelace_area(&[mesh.points[tri[0] as usize], mesh.points[tri[1] as usize], mesh.points[tri[2] as usize]])
    }

    fn total_area(mesh: &TriMesh2) -> f64 {
        mesh.tris.iter().map(|t| tri_area(mesh, t)).sum()
    }

    fn no_refine() -> MeshOpts {
        MeshOpts { max_edge: 0.0, min_angle_deg: 0.0 }
    }

    #[test]
    fn owned_bowyer_watson_replays_domain_invariants() {
        let fixtures = [
            (PlanarDomain { outer: square(10.0), holes: vec![] }, 100.0),
            (PlanarDomain { outer: vec![[0.0, 0.0], [10.0, 0.0], [10.0, 5.0], [5.0, 5.0], [5.0, 10.0], [0.0, 10.0]], holes: vec![] }, 75.0),
            (PlanarDomain { outer: square(10.0), holes: vec![vec![[3.0, 3.0], [7.0, 3.0], [7.0, 7.0], [3.0, 7.0]]] }, 84.0),
        ];
        for (domain, expected_area) in fixtures {
            let first = owned_triangulate(&domain, &no_refine()).expect("owned triangulates");
            let second = owned_triangulate(&domain, &no_refine()).expect("owned replay triangulates");
            assert_eq!(first, second);
            assert!((total_area(&first) - expected_area).abs() < 1e-9);
            assert!(tri_mesh_quality(&first).min_jacobian_sign_positive);
        }
    }

    fn square(side: f64) -> Vec<[f64; 2]> {
        vec![[0.0, 0.0], [side, 0.0], [side, side], [0.0, side]]
    }

    #[test]
    fn triangulate_square_area_matches_input() {
        let outer = square(10.0);
        let expected = shoelace_area(&outer);
        let domain = PlanarDomain { outer, holes: vec![] };
        let mesh = triangulate(&domain, &no_refine()).expect("triangulates");
        assert!(!mesh.tris.is_empty());
        assert!((total_area(&mesh) - expected).abs() < 1e-9);
    }

    #[test]
    fn triangulate_respects_hole_area() {
        let outer = square(10.0);
        let hole = vec![[3.0, 3.0], [7.0, 3.0], [7.0, 7.0], [3.0, 7.0]];
        let domain = PlanarDomain { outer, holes: vec![hole.clone()] };
        let mesh = triangulate(&domain, &no_refine()).expect("triangulates");
        let expected = 100.0 - 16.0;
        assert!((total_area(&mesh) - expected).abs() < 1e-6, "area={}", total_area(&mesh));
        for tri in &mesh.tris {
            let p0 = mesh.points[tri[0] as usize];
            let p1 = mesh.points[tri[1] as usize];
            let p2 = mesh.points[tri[2] as usize];
            let centroid = [(p0[0] + p1[0] + p2[0]) / 3.0, (p0[1] + p1[1] + p2[1]) / 3.0];
            assert!(!point_in_polygon(centroid, &hole));
        }
    }

    #[test]
    fn triangulate_honors_constrained_boundary_edges() {
        // L-shape: non-convex outer boundary.
        let outer = vec![[0.0, 0.0], [10.0, 0.0], [10.0, 5.0], [5.0, 5.0], [5.0, 10.0], [0.0, 10.0]];
        let domain = PlanarDomain { outer: outer.clone(), holes: vec![] };
        let mesh = triangulate(&domain, &no_refine()).expect("triangulates");

        let key = |p: [f64; 2]| (p[0].to_bits(), p[1].to_bits());
        let mut edge_set: HashSet<((u64, u64), (u64, u64))> = HashSet::new();
        for tri in &mesh.tris {
            let p = [mesh.points[tri[0] as usize], mesh.points[tri[1] as usize], mesh.points[tri[2] as usize]];
            for i in 0..3 {
                let a = key(p[i]);
                let b = key(p[(i + 1) % 3]);
                let edge = if a <= b { (a, b) } else { (b, a) };
                edge_set.insert(edge);
            }
        }

        for i in 0..outer.len() {
            let a = key(outer[i]);
            let b = key(outer[(i + 1) % outer.len()]);
            let edge = if a <= b { (a, b) } else { (b, a) };
            assert!(edge_set.contains(&edge), "boundary edge {i} missing from triangulation");
        }
    }

    #[test]
    fn refined_mesh_respects_min_angle() {
        // A long thin rectangle: all input corners are 90 degrees, while an unrefined diagonal would
        // produce slivers. Deterministic boundary subdivision plus the interior lattice removes them.
        let outer = vec![[0.0, 0.0], [20.0, 0.0], [20.0, 1.0], [0.0, 1.0]];
        let domain = PlanarDomain { outer, holes: vec![] };
        let opts = MeshOpts { max_edge: 1.0, min_angle_deg: 25.0 };
        let mesh = triangulate(&domain, &opts).expect("triangulates");
        let quality = tri_mesh_quality(&mesh);
        let epsilon = 2.0;
        assert!(quality.min_angle_deg >= opts.min_angle_deg - epsilon, "min_angle={}", quality.min_angle_deg);
    }

    #[test]
    fn quad_grid_has_expected_topology() {
        let mesh = quad_grid(0.0, 0.0, 3.0, 2.0, 3, 2);
        assert_eq!(mesh.quads.len(), 6);
        assert_eq!(mesh.points.len(), 12);
        assert_eq!(mesh.points[0], [0.0, 0.0]);
        assert_eq!(mesh.points[3], [3.0, 0.0]);
        assert_eq!(mesh.points[11], [3.0, 2.0]);
        assert_eq!(mesh.quads[0], [0, 1, 5, 4]);
        assert_eq!(mesh.quads[5], [6, 7, 11, 10]);
    }

    #[test]
    fn to_quadratic_welds_shared_edges() {
        let domain = PlanarDomain { outer: square(4.0), holes: vec![] };
        let mesh = triangulate(&domain, &no_refine()).expect("triangulates");
        assert!(mesh.tris.len() >= 2);

        let mut unique_edges: HashSet<(u32, u32)> = HashSet::new();
        for tri in &mesh.tris {
            for &(a, b) in &[(tri[0], tri[1]), (tri[1], tri[2]), (tri[2], tri[0])] {
                unique_edges.insert(if a < b { (a, b) } else { (b, a) });
            }
        }

        let quadratic = to_quadratic(&mesh);
        let new_points = quadratic.points.len() - mesh.points.len();
        assert_eq!(new_points, unique_edges.len());
    }

    #[test]
    fn extrude_tri_mesh_volume_matches_area_times_height() {
        let domain = PlanarDomain { outer: square(4.0), holes: vec![] };
        let mesh = triangulate(&domain, &no_refine()).expect("triangulates");
        let area = total_area(&mesh);
        let height = 3.0;
        let volume_mesh = extrude_tri_mesh(&mesh, height, 2);
        let tets = split_to_tets(&volume_mesh);
        let total: f64 = tets.cells.iter().map(|c| cell_signed_volume(&tets.points, c).abs()).sum();
        assert!((total - area * height).abs() < 1e-6, "total={} expected={}", total, area * height);
    }

    #[test]
    fn extrude_quad_mesh_volume_matches_area_times_height() {
        let mesh = quad_grid(0.0, 0.0, 4.0, 3.0, 4, 3);
        let area = 12.0;
        let height = 2.5;
        let volume_mesh = extrude_quad_mesh(&mesh, height, 3);
        let tets = split_to_tets(&volume_mesh);
        let total: f64 = tets.cells.iter().map(|c| cell_signed_volume(&tets.points, c).abs()).sum();
        assert!((total - area * height).abs() < 1e-6, "total={} expected={}", total, area * height);
    }

    #[test]
    fn split_to_tets_preserves_volume() {
        let domain = PlanarDomain { outer: square(4.0), holes: vec![] };
        let mesh = triangulate(&domain, &no_refine()).expect("triangulates");
        let wedge_mesh = extrude_tri_mesh(&mesh, 2.0, 2);
        let pre_wedge: f64 = wedge_mesh.cells.iter().map(|c| cell_signed_volume(&wedge_mesh.points, c).abs()).sum();
        let post_wedge = split_to_tets(&wedge_mesh);
        let post_wedge_total: f64 = post_wedge.cells.iter().map(|c| cell_signed_volume(&post_wedge.points, c).abs()).sum();
        assert!((pre_wedge - post_wedge_total).abs() < 1e-9);

        let quad_mesh = quad_grid(0.0, 0.0, 4.0, 4.0, 2, 2);
        let hex_mesh = extrude_quad_mesh(&quad_mesh, 2.0, 2);
        let pre_hex: f64 = hex_mesh.cells.iter().map(|c| cell_signed_volume(&hex_mesh.points, c).abs()).sum();
        let post_hex = split_to_tets(&hex_mesh);
        let post_hex_total: f64 = post_hex.cells.iter().map(|c| cell_signed_volume(&post_hex.points, c).abs()).sum();
        assert!((pre_hex - post_hex_total).abs() < 1e-9);
    }

    #[test]
    fn split_to_tets_shared_faces_are_parity_consistent() {
        // Two Hex8 cells sharing the quad face [1,2,6,5] (cell A's +x face / cell B's -x face).
        let points = vec![
            [0.0, 0.0, 0.0], // 0
            [1.0, 0.0, 0.0], // 1
            [1.0, 1.0, 0.0], // 2
            [0.0, 1.0, 0.0], // 3
            [0.0, 0.0, 1.0], // 4
            [1.0, 0.0, 1.0], // 5
            [1.0, 1.0, 1.0], // 6
            [0.0, 1.0, 1.0], // 7
            [2.0, 0.0, 0.0], // 8
            [2.0, 1.0, 0.0], // 9
            [2.0, 0.0, 1.0], // 10
            [2.0, 1.0, 1.0], // 11
        ];
        let cell_a = Cell::Hex8([0, 1, 2, 3, 4, 5, 6, 7]);
        let cell_b = Cell::Hex8([1, 8, 9, 2, 5, 10, 11, 6]);
        let shared_face: HashSet<u32> = [1, 2, 6, 5].into_iter().collect();

        let mesh_a = VolumeMesh { points: points.clone(), cells: vec![cell_a] };
        let mesh_b = VolumeMesh { points: points.clone(), cells: vec![cell_b] };
        let tets_a = split_to_tets(&mesh_a);
        let tets_b = split_to_tets(&mesh_b);

        let face_triangles = |vm: &VolumeMesh| -> HashSet<[u32; 3]> {
            let mut out = HashSet::new();
            for cell in &vm.cells {
                if let Cell::Tet4(t) = cell {
                    let faces = [[t[0], t[1], t[2]], [t[0], t[1], t[3]], [t[0], t[2], t[3]], [t[1], t[2], t[3]]];
                    for mut f in faces {
                        if f.iter().all(|v| shared_face.contains(v)) {
                            f.sort_unstable();
                            out.insert(f);
                        }
                    }
                }
            }
            out
        };

        let from_a = face_triangles(&tets_a);
        let from_b = face_triangles(&tets_b);
        assert_eq!(from_a.len(), 2, "expected the shared quad face split into 2 triangles from cell A");
        assert_eq!(from_a, from_b, "shared face must split identically from both cells");
    }

    #[test]
    fn volume_mesh_quality_detects_inverted_cell() {
        let points = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        let good = VolumeMesh { points: points.clone(), cells: vec![Cell::Tet4([0, 1, 2, 3])] };
        assert!(volume_mesh_quality(&good).min_jacobian_sign_positive);

        // Swap two nodes to invert the signed volume.
        let inverted = VolumeMesh { points, cells: vec![Cell::Tet4([1, 0, 2, 3])] };
        assert!(!volume_mesh_quality(&inverted).min_jacobian_sign_positive);
    }

    /// 🧱️ A `side`x`side` square extruded `height` tall, 1 layer, split to tets — `boundary_faces`'s
    /// total triangle area must equal the analytic box surface `2*side² + 4*side*height` (top + bottom
    /// + 4 sides), which also confirms every internal (shared, appears-twice) face was excluded.
    #[test]
    fn boundary_faces_area_matches_extruded_box_surface() {
        let side = 4.0;
        let height = 3.0;
        let domain = PlanarDomain { outer: square(side), holes: vec![] };
        let mesh = triangulate(&domain, &no_refine()).expect("triangulates");
        let volume_mesh = extrude_tri_mesh(&mesh, height, 1);
        let tets = split_to_tets(&volume_mesh);

        let faces = boundary_faces(&tets);
        let tri_area = |f: &[u32; 3]| -> f64 {
            let (a, b, c) = (tets.points[f[0] as usize], tets.points[f[1] as usize], tets.points[f[2] as usize]);
            let e0 = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
            let e1 = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
            let cross = [e0[1] * e1[2] - e0[2] * e1[1], e0[2] * e1[0] - e0[0] * e1[2], e0[0] * e1[1] - e0[1] * e1[0]];
            0.5 * (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt()
        };
        let total_area: f64 = faces.iter().map(tri_area).sum();
        let expected = 2.0 * side * side + 4.0 * side * height;
        assert!((total_area - expected).abs() < 1e-6, "total={total_area} expected={expected}");

        // Every boundary face must be wound so its normal points away from its own tet's centroid —
        // spot-checked here on the bottom face (z=0, outward normal must have negative z).
        for f in &faces {
            if tets.points[f[0] as usize][2] < 1e-9 && tets.points[f[1] as usize][2] < 1e-9 && tets.points[f[2] as usize][2] < 1e-9 {
                let (a, b, c) = (tets.points[f[0] as usize], tets.points[f[1] as usize], tets.points[f[2] as usize]);
                let e0 = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
                let e1 = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
                let normal_z = e0[0] * e1[1] - e0[1] * e1[0];
                assert!(normal_z < 0.0, "bottom face normal should point outward (-z), got normal_z={normal_z}");
            }
        }
    }

    #[test]
    fn triangulate_rejects_degenerate_outer_boundary() {
        let domain = PlanarDomain { outer: vec![[0.0, 0.0], [1.0, 0.0]], holes: vec![] };
        match triangulate(&domain, &no_refine()) {
            Err(MeshError::DegenerateDomain) => {}
            other => panic!("expected DegenerateDomain, got {other:?}"),
        }
    }

    #[test]
    fn triangulate_rejects_degenerate_hole() {
        let domain = PlanarDomain { outer: square(10.0), holes: vec![vec![[3.0, 3.0], [4.0, 4.0]]] };
        match triangulate(&domain, &no_refine()) {
            Err(MeshError::DegenerateDomain) => {}
            other => panic!("expected DegenerateDomain, got {other:?}"),
        }
    }

    #[test]
    fn point_in_polygon_returns_false_for_degenerate_polygon() {
        assert!(!point_in_polygon([0.0, 0.0], &[]));
        assert!(!point_in_polygon([0.0, 0.0], &[[0.0, 0.0], [1.0, 0.0]]));
    }

    /// 📊️ `tri_mesh_quality` flags a clockwise-wound (negative signed area) triangle via
    /// `min_jacobian_sign_positive`, and reports `0.0` angle bounds for an empty mesh instead of the
    /// unhelpful `f64::INFINITY`/`NEG_INFINITY` an empty min/max fold would otherwise leave behind.
    #[test]
    fn tri_mesh_quality_detects_inverted_winding_and_handles_empty_mesh() {
        let ccw = TriMesh2 { points: vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]], tris: vec![[0, 1, 2]] };
        assert!(tri_mesh_quality(&ccw).min_jacobian_sign_positive);

        let cw = TriMesh2 { points: vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]], tris: vec![[0, 2, 1]] };
        assert!(!tri_mesh_quality(&cw).min_jacobian_sign_positive);

        let empty = TriMesh2 { points: vec![], tris: vec![] };
        let quality = tri_mesh_quality(&empty);
        assert_eq!(quality.min_angle_deg, 0.0);
        assert_eq!(quality.max_angle_deg, 0.0);
        assert_eq!(quality.element_count, 0);
    }

    /// 🔺️ A `Cell::Tet4` already present in the input `VolumeMesh` passes through `split_to_tets`
    /// completely unchanged — the only cell kind besides `Wedge6`/`Hex8` `split_to_tets` accepts.
    #[test]
    fn split_to_tets_passes_through_existing_tet4_cells() {
        let points = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        let tet = Cell::Tet4([0, 1, 2, 3]);
        let mesh = VolumeMesh { points, cells: vec![tet] };
        let result = split_to_tets(&mesh);
        assert_eq!(result.cells.len(), 1);
        match result.cells[0] {
            Cell::Tet4(nodes) => assert_eq!(nodes, [0, 1, 2, 3]),
            _ => panic!("expected the Tet4 cell to pass through unchanged"),
        }
    }

    #[test]
    fn mesh_job_is_previewing_deterministic_and_step_bounded() {
        let domain = PlanarDomain { outer: square(8.0), holes: vec![square(2.0).into_iter().map(|point| [point[0] + 3.0, point[1] + 3.0]).collect()] };
        let options = MeshOpts { max_edge: 0.0, min_angle_deg: 0.0 };
        let first = drive_mesh_job(MeshJob::new(domain.clone(), options, mesh_operation()));
        let second = drive_mesh_job(MeshJob::new(domain, options, mesh_operation()));
        assert_eq!(first.0, second.0);
        assert!(first.1 > 0);
        assert!(first.2 < Duration::from_millis(8), "worst mesh job step was {:?}", first.2);
    }

    #[test]
    fn mesh_job_observes_cancellation_before_mutating() {
        fn now() -> u64 {
            0
        }
        let operation = mesh_operation();
        let mut job = MeshJob::new(PlanarDomain { outer: square(2.0), holes: vec![] }, MeshOpts { max_edge: 0.0, min_angle_deg: 0.0 }, operation);
        let cancel = root_cancel_token();
        cancel.cancel_now();
        let mut sequence = 0;
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(64, 10), cancel, now, &mut sequence);
        assert_eq!(job.step(&mut context), StepOutcome::Cancelled);
        assert_eq!(job.stage, MeshJobStage::Validate);
        assert!(job.triangulation.is_none());
    }

    #[test]
    fn mesh_job_large_boundary_never_runs_to_completion_in_one_step() {
        let boundary = (0..1_024)
            .map(|index| {
                let angle = index as f64 * std::f64::consts::TAU / 1_024.0;
                [angle.cos() * 100.0, angle.sin() * 100.0]
            })
            .collect();
        let (_, previews, worst) = drive_mesh_job(MeshJob::new(PlanarDomain { outer: boundary, holes: vec![] }, MeshOpts { max_edge: 0.0, min_angle_deg: 0.0 }, mesh_operation()));
        assert!(previews > 0);
        assert!(worst < Duration::from_millis(8), "worst large-boundary mesh job step was {worst:?}");
    }

    #[test]
    fn bounded_mesh_plus_one_fault_retains_the_exact_domain_for_cursor_close() {
        let operation = mesh_operation();
        let domain = PlanarDomain { outer: (0..65).map(|index| [index as f64, 0.0]).collect(), holes: Vec::new() };
        let mut job = MeshJob::new_bounded(domain, MeshOpts { max_edge: 0.0, min_angle_deg: 0.0 }, operation, 64, 128);
        let mut sequence = 0;
        let mut faulted = false;
        for _ in 0..256 {
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), root_cancel_token(), || 0, &mut sequence);
            if matches!(job.step(&mut context), StepOutcome::Fault(_)) {
                faulted = true;
                break;
            }
        }
        assert!(faulted);
        assert_eq!(job.domain.outer().len(), 65, "fault retains every rejected point owner");
        let (terminal, items, _) = job.close_step(4_096);
        assert!(!terminal);
        assert_eq!(items, 1);
        assert_eq!(job.domain.outer().len(), 64, "one close grant releases one point owner");
    }

    #[test]
    fn mesh_mounted_classification_indexes_admit_maximum_reject_plus_one_and_close_exactly() {
        let operation = Operation::new(OperationId(41), RevisionId(43), Generation(47), 53);
        let mut job = MeshJob::new_bounded(PlanarDomain { outer: square(1.0), holes: Vec::new() }, MeshOpts { max_edge: 0.0, min_angle_deg: 0.0 }, operation, 8, 2);
        let edge_capacity = job.maximum_triangles * 12 + 3;
        job.indexed_edges.try_reserve_exact(edge_capacity).expect("fixed edge index backing");
        assert!(job.indexed_edges.capacity() * std::mem::size_of::<Edge>() <= 4_096);
        job.indexed_edges.extend((0..edge_capacity).map(|index| Edge(index, index + 1)));
        let before = (job.indexed_edges.as_ptr(), job.indexed_edges.len(), job.indexed_edges.capacity());
        assert_eq!(job.indexed_edges.binary_search(&Edge(edge_capacity + 1, edge_capacity + 2)), Err(edge_capacity));
        assert_eq!((job.indexed_edges.as_ptr(), job.indexed_edges.len(), job.indexed_edges.capacity()), before, "plus-one preflight returns the exact index authority without insertion");
        job.stage = MeshJobStage::Complete;
        job.close_lane = 7;
        assert!(!job.close_step(4_096).0, "one grant retires one exact indexed edge");
        assert_eq!(job.indexed_edges.len(), edge_capacity - 1);
    }

    #[test]
    fn p6h_constraint_flip_interrupts_after_every_edge_phase_and_updates_only_affected_adjacencies() {
        let operation = mesh_operation();
        let points = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        let triangulation = OwnedTriangulation {
            points: points.clone(),
            triangles: vec![[0, 1, 2], [0, 2, 3]],
            input_len: 4,
            insert_cursor: 4,
            insertion_order: vec![0, 1, 2, 3],
            insertion: None,
            maximum_triangles: 4,
            allocation_fault: false,
            mounted_initialization: MountedTriangulationInitialization { stage: MountedTriangulationStage::Complete, cursor: 0, sort_outer: 4, sort_inner: 4, bounds: [0.0, 1.0, 0.0, 1.0], center: [0.5, 0.5], span: 1.0 },
            finish: TriangulationFinishCursor { stage: TriangulationFinishStage::Complete, read: 2, write: 2, sort_outer: 2, sort_inner: 2 },
        };
        let mut job = MeshJob::new_bounded(PlanarDomain { outer: points, holes: Vec::new() }, no_refine(), operation, 4, 4);
        job.triangulation = Some(triangulation);
        job.constraints = vec![Edge::new(1, 3)];
        job.stage = MeshJobStage::ConstrainBoundary;
        job.indexed_constraint_edges.try_reserve_exact(12).expect("fixed edge authority");
        for triangle in 0..2 {
            for local in 0..3 {
                assert!(job.begin_edge_index_candidate(triangle, local));
                while !job.advance_edge_index_candidate().expect("index edge") {}
            }
        }
        let mut seen = HashSet::new();
        let mut sequence = 0;
        let mut maximum_micros = 0;
        for _ in 0..512 {
            let stage = job.constraint_stage;
            seen.insert(stage);
            let before = job.triangulation.as_ref().expect("triangulation retained").triangles.clone();
            let before_cursor = (job.constraint_cursor, job.constraint_stage, job.constraint_search_cursor, job.constraint_apply_cursor, job.constraint_retire_cursor, job.constraint_retire_adjacency_cursor);
            let mut deadline = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, 0), root_cancel_token(), || 0, &mut sequence);
            assert_eq!(job.step(&mut deadline), StepOutcome::Yield);
            assert_eq!((job.constraint_cursor, job.constraint_stage, job.constraint_search_cursor, job.constraint_apply_cursor, job.constraint_retire_cursor, job.constraint_retire_adjacency_cursor), before_cursor);
            assert_eq!(job.triangulation.as_ref().expect("triangulation retained").triangles, before);

            let mut stale = StepContext::new(operation.operation, Generation(operation.generation.0 + 1), StepBudget::new(1, u64::MAX), root_cancel_token(), || 0, &mut sequence);
            assert!(matches!(job.step(&mut stale), StepOutcome::Fault(_)));
            assert_eq!((job.constraint_cursor, job.constraint_stage, job.constraint_search_cursor, job.constraint_apply_cursor, job.constraint_retire_cursor, job.constraint_retire_adjacency_cursor), before_cursor);

            let token = root_cancel_token();
            semio_framework_async::block_on(token.cancel());
            let mut cancelled = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), token, || 0, &mut sequence);
            assert_eq!(job.step(&mut cancelled), StepOutcome::Cancelled);
            assert_eq!((job.constraint_cursor, job.constraint_stage, job.constraint_search_cursor, job.constraint_apply_cursor, job.constraint_retire_cursor, job.constraint_retire_adjacency_cursor), before_cursor);

            let started = std::time::Instant::now();
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), root_cancel_token(), || 0, &mut sequence);
            assert_eq!(job.step(&mut context), StepOutcome::Yield);
            maximum_micros = maximum_micros.max(started.elapsed().as_micros());
            if job.constraint_cursor == job.constraints.len() {
                break;
            }
        }
        for stage in [
            ConstraintRecoveryStage::ReserveConstraintWorkspace,
            ConstraintRecoveryStage::IndexTriangleEdge,
            ConstraintRecoveryStage::SearchConstraintEdge,
            ConstraintRecoveryStage::ClassifyIntersection,
            ConstraintRecoveryStage::SelectDeterministicFlip,
            ConstraintRecoveryStage::ValidateFlip,
            ConstraintRecoveryStage::ApplyFlip,
            ConstraintRecoveryStage::RetireFormerEdge,
            ConstraintRecoveryStage::PublishConstraintProgress,
            ConstraintRecoveryStage::ConstraintComplete,
        ] {
            assert!(seen.contains(&stage), "missing interrupted phase {stage:?}");
        }
        assert!(job.indexed_constraint_edges.iter().any(|slot| slot.active && slot.edge == Edge::new(1, 3)));
        assert!(!job.indexed_constraint_edges.iter().any(|slot| slot.active && slot.edge == Edge::new(0, 2)));
        assert!(maximum_micros < 8_000, "constraint recovery step exceeded timing ceiling: {maximum_micros} us");

        let mut close_turns = 0;
        loop {
            close_turns += 1;
            let (terminal, released_items, _) = job.close_step(usize::MAX);
            assert!(released_items <= 1);
            if terminal {
                break;
            }
            assert!(close_turns < 20_000);
        }
        assert!(job.close_lane > 11);
    }

    /// 📦️ Mounted preparation, construction, finish, and payload cursors interrupt and replay exactly.
    #[test]
    fn p6h_mounted_mesh_preparation_initialization_finish_publication_interrupt_replay_timing_and_close() {
        fn domain() -> PlanarDomain {
            PlanarDomain { outer: square(2.0), holes: vec![square(0.5).into_iter().map(|point| [point[0] + 0.75, point[1] + 0.75]).collect()] }
        }

        fn snapshot(job: &MeshJob) -> impl PartialEq + std::fmt::Debug {
            (
                job.stage,
                job.preparation.as_ref().map(|cursor| {
                    (
                        (cursor.polygon, cursor.edge, cursor.segment, cursor.point_lookup_cursor, cursor.grid_row, cursor.grid_column, cursor.grid_polygon),
                        (cursor.grid_edge, cursor.pending_point.is_some(), cursor.pending_index, cursor.grid_candidate.is_some(), cursor.points.len(), cursor.constraints.len()),
                    )
                }),
                job.triangulation.as_ref().map(|cursor| {
                    (
                        cursor.mounted_initialization.stage,
                        cursor.mounted_initialization.cursor,
                        cursor.mounted_initialization.sort_outer,
                        cursor.mounted_initialization.sort_inner,
                        cursor.finish.stage,
                        cursor.finish.read,
                        cursor.finish.write,
                        cursor.finish.sort_outer,
                        cursor.finish.sort_inner,
                        cursor.insert_cursor,
                        cursor.points.len(),
                        cursor.triangles.len(),
                    )
                }),
                job.publication_kind,
                job.publication_cursor.stage,
                job.publication_cursor.point,
                job.publication_cursor.coordinate,
                job.publication_cursor.triangle,
                job.publication_cursor.index,
                job.publication_writer.as_ref().and_then(RetainedJobPayloadWriter::staged_page_len),
                job.mesh.points.len(),
                job.mesh.tris.len(),
            )
        }

        fn run(operation: Operation) -> (Vec<u8>, u128) {
            let mut job = MeshJob::new_bounded(domain(), MeshOpts { max_edge: 2.0, min_angle_deg: 0.0 }, operation, 128, 20);
            let mut sequence = 0;
            let mut maximum_micros = 0;
            let mut initialization_seen = HashSet::new();
            let mut finish_seen = HashSet::new();
            let mut payload_seen = HashSet::new();
            let mut preparation_lookup_seen = false;
            let mut preparation_polygon_seen = false;
            for _ in 0..1_000_000 {
                if let Some(preparation) = job.preparation.as_ref() {
                    preparation_lookup_seen |= preparation.point_lookup_cursor > 0;
                    preparation_polygon_seen |= preparation.grid_candidate.is_some() && preparation.grid_edge > 0;
                }
                if let Some(triangulation) = job.triangulation.as_ref() {
                    initialization_seen.insert(triangulation.mounted_initialization.stage);
                    finish_seen.insert(triangulation.finish.stage);
                }
                if job.publication_kind.is_some() {
                    payload_seen.insert(job.publication_cursor.stage);
                }
                if matches!(job.stage, MeshJobStage::PrepareInput | MeshJobStage::Initialize | MeshJobStage::InsertBoundary | MeshJobStage::PublishPreview | MeshJobStage::PublishCheckpoint | MeshJobStage::Complete) {
                    let before = snapshot(&job);
                    let mut deadline = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, 0), root_cancel_token(), || 0, &mut sequence);
                    assert_eq!(job.step(&mut deadline), StepOutcome::Yield);
                    assert_eq!(snapshot(&job), before);
                    let mut stale = StepContext::new(operation.operation, Generation(operation.generation.0 + 1), StepBudget::new(1, u64::MAX), root_cancel_token(), || 0, &mut sequence);
                    assert!(matches!(job.step(&mut stale), StepOutcome::Fault(_)));
                    assert_eq!(snapshot(&job), before);
                    let token = root_cancel_token();
                    semio_framework_async::block_on(token.cancel());
                    let mut cancelled = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), token, || 0, &mut sequence);
                    assert_eq!(job.step(&mut cancelled), StepOutcome::Cancelled);
                    assert_eq!(snapshot(&job), before);
                }
                let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), root_cancel_token(), || 0, &mut sequence);
                let started = Instant::now();
                let outcome = job.step(&mut context);
                maximum_micros = maximum_micros.max(started.elapsed().as_micros());
                match outcome {
                    StepOutcome::PreviewReady(preview) => {
                        take_payload_bytes(preview);
                    }
                    StepOutcome::CheckpointReady(checkpoint) => {
                        take_payload_bytes(checkpoint.state);
                    }
                    StepOutcome::Complete(candidate) => {
                        take_payload_bytes(candidate.state);
                        for stage in [
                            MountedTriangulationStage::BoundsPoint,
                            MountedTriangulationStage::ValidateBounds,
                            MountedTriangulationStage::ReserveInsertionOrder,
                            MountedTriangulationStage::BuildInsertionOrder,
                            MountedTriangulationStage::OrderInsertion,
                            MountedTriangulationStage::ReserveSuperPoints,
                            MountedTriangulationStage::AppendSuperPoint,
                            MountedTriangulationStage::ReserveTriangles,
                            MountedTriangulationStage::SeedTriangle,
                            MountedTriangulationStage::Complete,
                        ] {
                            assert!(initialization_seen.contains(&stage), "missing mounted initialization stage {stage:?}");
                        }
                        for stage in [TriangulationFinishStage::Filter, TriangulationFinishStage::TruncateTriangles, TriangulationFinishStage::OrderTriangles, TriangulationFinishStage::TruncatePoints, TriangulationFinishStage::Complete] {
                            assert!(finish_seen.contains(&stage), "missing finish stage {stage:?}");
                        }
                        for stage in [
                            MeshPayloadStage::Magic,
                            MeshPayloadStage::Tier,
                            MeshPayloadStage::Complete,
                            MeshPayloadStage::Sequence,
                            MeshPayloadStage::Refinement,
                            MeshPayloadStage::PointCount,
                            MeshPayloadStage::TriangleCount,
                            MeshPayloadStage::PointCoordinate,
                            MeshPayloadStage::TriangleIndex,
                            MeshPayloadStage::CommitPage,
                        ] {
                            assert!(payload_seen.contains(&stage), "missing payload stage {stage:?}");
                        }
                        assert!(preparation_lookup_seen && preparation_polygon_seen);
                        return (take_payload_bytes(candidate.output), maximum_micros);
                    }
                    StepOutcome::Yield => {}
                    outcome => panic!("mounted mesh cursor law failed: {outcome:?}"),
                }
            }
            panic!("mounted mesh cursor law did not complete")
        }

        let operation = mesh_operation();
        let first = run(operation);
        let second = run(operation);
        assert_eq!(first.0, second.0);
        assert!(first.1.max(second.1) < 8_000);

        let mut interrupted = MeshJob::new_bounded(domain(), MeshOpts { max_edge: 2.0, min_angle_deg: 0.0 }, operation, 128, 20);
        let mut sequence = 0;
        for _ in 0..1_000_000 {
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), root_cancel_token(), || 0, &mut sequence);
            match interrupted.step(&mut context) {
                StepOutcome::PreviewReady(preview) => {
                    take_payload_bytes(preview);
                }
                StepOutcome::CheckpointReady(checkpoint) => {
                    take_payload_bytes(checkpoint.state);
                }
                StepOutcome::Fault(fault) => panic!("interrupted mesh fixture fault: {:?}", fault.detail),
                _ => {}
            }
            if interrupted.publication_writer.as_ref().and_then(RetainedJobPayloadWriter::staged_page_len).is_some_and(|length| length > 0) {
                break;
            }
        }
        assert!(interrupted.publication_writer.is_some());
        InteractiveJob::begin_close(&mut interrupted);
        assert_eq!(InteractiveJob::close_step(&mut interrupted, 1, 0), semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 });
        for _ in 0..100_000 {
            match InteractiveJob::close_step(&mut interrupted, 1, usize::MAX) {
                semio_framework_job::InteractiveJobCloseStep::Pending { released_items, .. } => assert!(released_items <= 1),
                semio_framework_job::InteractiveJobCloseStep::Complete => break,
                semio_framework_job::InteractiveJobCloseStep::Blocked => panic!("mounted mesh close cannot block"),
            }
        }
        assert!(InteractiveJob::terminal_is_empty(&interrupted));
    }
}
// #endregion 🔖️Tests
