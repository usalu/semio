//! 📐️ Puzzle 3d play app — the precompute geometry layer: the `nalgebra`/`parry3d` adapter (the ONE
//! interface boundary this module depends on), the plain `[f64; 3]`/`[f64; 4]` vector and
//! quaternion math the placement solver builds on, the brush placement pose solver itself, and the
//! collision-body/AABB/overlap primitives the brush and fill lanes gate placements with. Rehomed from
//! the former `⚙️engine/📐️geometry` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): this
//! is interactive brush/fill tool behaviour, so it lives with the app, not the artifact.

use crate::artifacts::puzzle3d::schema::{Quat, Vec3, WorldVolumeProps};
use std::collections::{BTreeMap, BTreeSet};

//#region 🔒️GeometryAdapter
/// 🔒️ Thin wrappers over `nalgebra`/`parry3d` — the one interface boundary this artifact depends on.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Vec3d(nalgebra::Vector3<f32>);

impl Vec3d {
    pub(crate) fn new(x: f32, y: f32, z: f32) -> Self {
        Self(nalgebra::Vector3::new(x, y, z))
    }
    pub(crate) fn x(&self) -> f32 {
        self.0.x
    }
    pub(crate) fn y(&self) -> f32 {
        self.0.y
    }
    pub(crate) fn z(&self) -> f32 {
        self.0.z
    }
    pub(crate) fn amax(&self) -> f32 {
        self.0.amax()
    }
}

impl std::ops::Add for Vec3d {
    type Output = Vec3d;
    fn add(self, rhs: Self) -> Self {
        Vec3d(self.0 + rhs.0)
    }
}

impl std::ops::Mul<f32> for Vec3d {
    type Output = Vec3d;
    fn mul(self, rhs: f32) -> Self {
        Vec3d(self.0 * rhs)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Point3d(nalgebra::Point3<f32>);

impl Point3d {
    pub(crate) fn new(x: f32, y: f32, z: f32) -> Self {
        Self(nalgebra::Point3::new(x, y, z))
    }
    pub(crate) fn x(&self) -> f32 {
        self.0.x
    }
    pub(crate) fn y(&self) -> f32 {
        self.0.y
    }
    pub(crate) fn z(&self) -> f32 {
        self.0.z
    }
    pub(crate) fn inf(&self, other: &Self) -> Self {
        Self(self.0.inf(&other.0))
    }
    pub(crate) fn sup(&self, other: &Self) -> Self {
        Self(self.0.sup(&other.0))
    }
    pub(crate) fn coords(&self) -> Vec3d {
        Vec3d(self.0.coords)
    }
    pub(crate) fn from_coords(v: Vec3d) -> Self {
        Self(nalgebra::Point3::from(v.0))
    }
}

impl std::ops::Sub for Point3d {
    type Output = Vec3d;
    fn sub(self, rhs: Self) -> Vec3d {
        Vec3d(self.0 - rhs.0)
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Rotation3d(nalgebra::UnitQuaternion<f32>);

impl Rotation3d {
    pub(crate) fn identity() -> Self {
        Self(nalgebra::UnitQuaternion::identity())
    }
    /// 🔓️ Builds from CAD's `[i, j, k, w]` quaternion convention.
    pub(crate) fn from_ijkw(i: f32, j: f32, k: f32, w: f32) -> Self {
        Self(nalgebra::UnitQuaternion::from_quaternion(nalgebra::Quaternion::new(w, i, j, k)))
    }
    pub(crate) fn to_ijkw(self) -> (f32, f32, f32, f32) {
        let q = self.0.quaternion();
        (q.i, q.j, q.k, q.w)
    }
    pub(crate) fn rotation_between(from: Vec3d, to: Vec3d) -> Option<Self> {
        nalgebra::UnitQuaternion::rotation_between(&from.0, &to.0).map(Self)
    }
    pub(crate) fn apply(&self, v: Vec3d) -> Vec3d {
        Vec3d(self.0 * v.0)
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Pose3d(nalgebra::Isometry3<f32>);

impl Pose3d {
    pub(crate) fn identity() -> Self {
        Self(nalgebra::Isometry3::identity())
    }
    pub(crate) fn from_parts(translation: Vec3d, rotation: Rotation3d) -> Self {
        Self(nalgebra::Isometry3::from_parts(translation.0.into(), rotation.0))
    }
    pub(crate) fn inverse(&self) -> Self {
        Self(self.0.inverse())
    }
    pub(crate) fn transform_point(&self, point: &Point3d) -> Point3d {
        Point3d(self.0 * point.0)
    }
    pub(crate) fn semio_compose_rs(&self, other: &Self) -> Self {
        Self(self.0 * other.0)
    }
}

#[derive(Clone)]
pub(crate) struct CollisionShape(parry3d::shape::SharedShape);

impl CollisionShape {
    pub(crate) fn from_triangle_mesh(vertices: &[Point3d], indices: Vec<[u32; 3]>) -> Self {
        let verts: Vec<nalgebra::Point3<f32>> = vertices.iter().map(|p| p.0).collect();
        let mesh = parry3d::shape::TriMesh::with_flags(verts, indices, parry3d::shape::TriMeshFlags::ORIENTED | parry3d::shape::TriMeshFlags::MERGE_DUPLICATE_VERTICES);
        Self(parry3d::shape::SharedShape::new(mesh))
    }
    pub(crate) fn contains_point(&self, pose: &Pose3d, point: &Point3d) -> bool {
        self.0.contains_point(&pose.0, &point.0)
    }
}

fn shapes_intersect(pose_a: &Pose3d, a: &CollisionShape, pose_b: &Pose3d, b: &CollisionShape) -> bool {
    parry3d::query::intersection_test(&pose_a.0, &*a.0, &pose_b.0, &*b.0).unwrap_or(false)
}
//#endregion 🔒️GeometryAdapter

//#region 🔖️Constants
const SURFACE_CONTACT_MAX_AABB_VOLUME: f64 = 1e-4;
const BRUSH_COLLISION_MESH_MIN_EXTENT: f64 = 2.0;
pub(crate) const BRUSH_PLACEMENT_PARALLEL_TOLERANCE: f64 = 1e-6;
//#endregion 🔖️Constants

//#region 🔖️Vectors
pub(crate) fn normalize_vec3(v: Vec3) -> Vec3 {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len < 1e-9 {
        return [0.0, 0.0, -1.0];
    }
    [v[0] / len, v[1] / len, v[2] / len]
}

pub(crate) fn vec3_dot(a: Vec3, b: Vec3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub(crate) fn vec3_cross(a: Vec3, b: Vec3) -> Vec3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

pub(crate) fn vec3_add(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

pub(crate) fn vec3_sub(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

pub(crate) fn negate_vec3(v: Vec3) -> Vec3 {
    [-v[0], -v[1], -v[2]]
}

pub(crate) fn vec3_scale(v: Vec3, scale: &Option<dsl::DslValue>) -> Vec3 {
    match scale {
        None => v,
        Some(dsl::DslValue::Number(n)) => {
            let s = *n;
            [v[0] * s, v[1] * s, v[2] * s]
        }
        Some(dsl::DslValue::Array(arr)) if arr.len() >= 3 => {
            let sx = arr[0].as_f64().unwrap_or(1.0);
            let sy = arr[1].as_f64().unwrap_or(1.0);
            let sz = arr[2].as_f64().unwrap_or(1.0);
            [v[0] * sx, v[1] * sy, v[2] * sz]
        }
        _ => v,
    }
}

pub(crate) fn unit_quat_from_cad(q: Quat) -> Rotation3d {
    Rotation3d::from_ijkw(q[0] as f32, q[1] as f32, q[2] as f32, q[3] as f32)
}

pub(crate) fn quat_rotate_vec(q: Quat, v: Vec3) -> Vec3 {
    let uq = unit_quat_from_cad(q);
    let rotated = uq.apply(Vec3d::new(v[0] as f32, v[1] as f32, v[2] as f32));
    [rotated.x() as f64, rotated.y() as f64, rotated.z() as f64]
}

pub(crate) fn quaternion_from_180_degree_axis(axis: Vec3) -> Quat {
    let unit = normalize_vec3(axis);
    [unit[0], unit[1], unit[2], 0.0]
}

pub(crate) fn anti_parallel_brush_orientation(target_dir: Vec3) -> Quat {
    let z_axis: Vec3 = [0.0, 0.0, 1.0];
    if target_dir[2].abs() < BRUSH_PLACEMENT_PARALLEL_TOLERANCE {
        return quaternion_from_180_degree_axis(z_axis);
    }
    let axis = vec3_cross(z_axis, target_dir);
    if (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt() < BRUSH_PLACEMENT_PARALLEL_TOLERANCE {
        return quaternion_from_180_degree_axis([1.0, 0.0, 0.0]);
    }
    quaternion_from_180_degree_axis(axis)
}

pub(crate) fn pose_isometry(origin: Vec3, orientation: Quat, _scale: &Option<dsl::DslValue>) -> Pose3d {
    let q = unit_quat_from_cad(orientation);
    let t = Vec3d::new(origin[0] as f32, origin[1] as f32, origin[2] as f32);
    Pose3d::from_parts(t, q)
}

pub(crate) fn compute_brush_placement_pose(
    source_local_position: Vec3,
    source_local_direction: Vec3,
    scale: &Option<dsl::DslValue>,
    target_world_position: Vec3,
    target_world_direction: Vec3,
    reference_orientation: Option<Quat>,
    use_host_orientation: bool,
) -> (Vec3, Quat) {
    let scaled_local = vec3_scale(source_local_position, scale);
    let local_dir = normalize_vec3(source_local_direction);
    let target_dir = normalize_vec3(target_world_direction);
    if use_host_orientation {
        if let Some(host_orientation) = reference_orientation {
            let world_source_dir = normalize_vec3(quat_rotate_vec(host_orientation, local_dir));
            if vec3_dot(world_source_dir, target_dir) < -BRUSH_PLACEMENT_PARALLEL_TOLERANCE {
                let origin = vec3_sub(target_world_position, quat_rotate_vec(host_orientation, scaled_local));
                return (origin, host_orientation);
            }
        }
    }
    let desired_world_dir = negate_vec3(target_dir);
    let orientation = if vec3_dot(local_dir, desired_world_dir) < -1.0 + BRUSH_PLACEMENT_PARALLEL_TOLERANCE {
        anti_parallel_brush_orientation(target_dir)
    } else {
        let from = Vec3d::new(local_dir[0] as f32, local_dir[1] as f32, local_dir[2] as f32);
        let to = Vec3d::new(desired_world_dir[0] as f32, desired_world_dir[1] as f32, desired_world_dir[2] as f32);
        let q = Rotation3d::rotation_between(from, to).unwrap_or(Rotation3d::identity());
        let (i, j, k, w) = q.to_ijkw();
        [i as f64, j as f64, k as f64, w as f64]
    };
    let origin = vec3_sub(target_world_position, quat_rotate_vec(orientation, scaled_local));
    (origin, orientation)
}
//#endregion 🔖️Vectors

//#region 🔖️Collision
#[derive(Clone)]
pub(crate) struct CollisionMeshPart {
    pub(crate) shape: CollisionShape,
    pub(crate) local_pose: Pose3d,
}

#[derive(Clone)]
pub(crate) struct CollisionBody {
    pub(crate) parts: Vec<CollisionMeshPart>,
    pub(crate) local_bounds_min: Point3d,
    pub(crate) local_bounds_max: Point3d,
}

pub(crate) fn collision_body_from_buffers(positions: &[f32], indices: &[u32]) -> Option<CollisionBody> {
    if positions.len() < 9 || indices.len() < 3 {
        return None;
    }
    let mut verts: Vec<Point3d> = Vec::with_capacity(positions.len() / 3);
    let mut min = Point3d::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
    let mut max = Point3d::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
    for chunk in positions.as_chunks::<3>().0 {
        let rp = Point3d::new(chunk[0], chunk[1], chunk[2]);
        verts.push(rp);
        min = min.inf(&rp);
        max = max.sup(&rp);
    }
    let extent = (max - min).amax();
    if !extent.is_finite() || extent < BRUSH_COLLISION_MESH_MIN_EXTENT as f32 {
        return None;
    }
    let mut tris: Vec<[u32; 3]> = Vec::with_capacity(indices.len() / 3);
    for chunk in indices.as_chunks::<3>().0 {
        tris.push(*chunk);
    }
    let shape = CollisionShape::from_triangle_mesh(&verts, tris);
    Some(CollisionBody { parts: vec![CollisionMeshPart { shape, local_pose: Pose3d::identity() }], local_bounds_min: min, local_bounds_max: max })
}

pub(crate) fn world_bounds(body: &CollisionBody, world: &Pose3d) -> (Point3d, Point3d) {
    let corners = [
        Point3d::new(body.local_bounds_min.x(), body.local_bounds_min.y(), body.local_bounds_min.z()),
        Point3d::new(body.local_bounds_max.x(), body.local_bounds_min.y(), body.local_bounds_min.z()),
        Point3d::new(body.local_bounds_min.x(), body.local_bounds_max.y(), body.local_bounds_min.z()),
        Point3d::new(body.local_bounds_max.x(), body.local_bounds_max.y(), body.local_bounds_min.z()),
        Point3d::new(body.local_bounds_min.x(), body.local_bounds_min.y(), body.local_bounds_max.z()),
        Point3d::new(body.local_bounds_max.x(), body.local_bounds_min.y(), body.local_bounds_max.z()),
        Point3d::new(body.local_bounds_min.x(), body.local_bounds_max.y(), body.local_bounds_max.z()),
        Point3d::new(body.local_bounds_max.x(), body.local_bounds_max.y(), body.local_bounds_max.z()),
    ];
    let mut min = Point3d::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
    let mut max = Point3d::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
    for corner in corners {
        let w = world.transform_point(&corner);
        min = min.inf(&w);
        max = max.sup(&w);
    }
    (min, max)
}

pub(crate) fn volume_scale_vec(scale: &Option<dsl::DslValue>) -> [f32; 3] {
    match scale {
        Some(dsl::DslValue::Number(n)) => {
            let s = *n as f32;
            [s, s, s]
        }
        Some(dsl::DslValue::Array(values)) if values.len() == 3 => {
            let read = |index: usize| values.get(index).and_then(|v| v.as_f64()).unwrap_or(1.0) as f32;
            [read(0), read(1), read(2)]
        }
        _ => [1.0, 1.0, 1.0],
    }
}

pub(crate) fn world_volumes_contain_aabb(volumes: &[WorldVolumeProps], min: Point3d, max: Point3d) -> bool {
    if volumes.is_empty() {
        return true;
    }
    let corners = [
        min,
        Point3d::new(max.x(), min.y(), min.z()),
        Point3d::new(min.x(), max.y(), min.z()),
        Point3d::new(max.x(), max.y(), min.z()),
        Point3d::new(min.x(), min.y(), max.z()),
        Point3d::new(max.x(), min.y(), max.z()),
        Point3d::new(min.x(), max.y(), max.z()),
        max,
    ];
    for volume in volumes {
        let scale = volume_scale_vec(&volume.scale);
        let world = pose_isometry(volume.origin, volume.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &None);
        let inv = world.inverse();
        let hx = 0.5 + 1e-3;
        let hy = 0.5 + 1e-3;
        let hz = 0.5 + 1e-3;
        let mut inside = true;
        for corner in corners {
            let relative = inv.transform_point(&corner);
            let local = Point3d::new(relative.x() / scale[0], relative.y() / scale[1], relative.z() / scale[2]);
            if local.x().abs() > hx || local.y().abs() > hy || local.z().abs() > hz {
                inside = false;
                break;
            }
        }
        if inside {
            return true;
        }
    }
    false
}

pub(crate) fn point_inside_body(body: &CollisionBody, world: &Pose3d, point: Point3d) -> bool {
    let local = world.inverse().transform_point(&point);
    for part in &body.parts {
        let part_local = part.local_pose.inverse().transform_point(&local);
        if part.shape.contains_point(&part.local_pose, &part_local) {
            return true;
        }
    }
    false
}

pub(crate) fn bodies_intersect(a: &CollisionBody, world_a: &Pose3d, b: &CollisionBody, world_b: &Pose3d) -> bool {
    let (amin, amax) = world_bounds(a, world_a);
    let (bmin, bmax) = world_bounds(b, world_b);
    if amax.x() < bmin.x() || bmax.x() < amin.x() || amax.y() < bmin.y() || bmax.y() < amin.y() || amax.z() < bmin.z() || bmax.z() < amin.z() {
        return false;
    }
    for part_a in &a.parts {
        let pose_a = world_a.semio_compose_rs(&part_a.local_pose);
        for part_b in &b.parts {
            let pose_b = world_b.semio_compose_rs(&part_b.local_pose);
            if shapes_intersect(&pose_a, &part_a.shape, &pose_b, &part_b.shape) {
                return true;
            }
        }
    }
    let center = Point3d::from_coords((amin.coords() + amax.coords() + bmin.coords() + bmax.coords()) * 0.25);
    point_inside_body(a, world_a, center) && point_inside_body(b, world_b, center)
}

//#region 🗺️BroadPhase
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct CollisionAabb {
    pub(crate) min: [f32; 3],
    pub(crate) max: [f32; 3],
}

impl CollisionAabb {
    pub(crate) fn from_body(body: &CollisionBody, world: &Pose3d) -> Self {
        let (min, max) = world_bounds(body, world);
        Self { min: [min.x(), min.y(), min.z()], max: [max.x(), max.y(), max.z()] }
    }

    fn intersects(&self, other: &Self) -> bool {
        self.max[0] >= other.min[0] && other.max[0] >= self.min[0] && self.max[1] >= other.min[1] && other.max[1] >= self.min[1] && self.max[2] >= other.min[2] && other.max[2] >= self.min[2]
    }
}

#[derive(Clone, Debug)]
pub(crate) struct CollisionSpatialIndex {
    cell_size: f32,
    entries: BTreeMap<String, CollisionAabb>,
    cells: BTreeMap<(i32, i32, i32), Vec<String>>,
    oversized: BTreeSet<String>,
}

impl CollisionSpatialIndex {
    const MAX_CELLS_PER_ENTRY: u64 = 4_096;

    pub(crate) fn new(cell_size: f32) -> Self {
        assert!(cell_size.is_finite() && cell_size > 0.0);
        Self { cell_size, entries: BTreeMap::new(), cells: BTreeMap::new(), oversized: BTreeSet::new() }
    }

    pub(crate) fn upsert(&mut self, id: String, bounds: CollisionAabb) {
        self.remove(&id);
        if let Some(cells) = self.covered_cells(bounds) {
            for cell in cells {
                let ids = self.cells.entry(cell).or_default();
                match ids.binary_search(&id) {
                    Ok(_) => {}
                    Err(index) => ids.insert(index, id.clone()),
                }
            }
        } else {
            self.oversized.insert(id.clone());
        }
        self.entries.insert(id, bounds);
    }

    pub(crate) fn remove(&mut self, id: &str) -> bool {
        let Some(bounds) = self.entries.remove(id) else { return false };
        if let Some(cells) = self.covered_cells(bounds) {
            for cell in cells {
                let remove_cell = if let Some(ids) = self.cells.get_mut(&cell) {
                    if let Ok(index) = ids.binary_search_by(|candidate| candidate.as_str().cmp(id)) {
                        ids.remove(index);
                    }
                    ids.is_empty()
                } else {
                    false
                };
                if remove_cell {
                    self.cells.remove(&cell);
                }
            }
        } else {
            self.oversized.remove(id);
        }
        true
    }

    pub(crate) fn query(&self, bounds: CollisionAabb) -> Vec<String> {
        let mut candidates = BTreeSet::new();
        if let Some(cells) = self.covered_cells(bounds) {
            for cell in cells {
                if let Some(ids) = self.cells.get(&cell) {
                    candidates.extend(ids.iter().cloned());
                }
            }
            candidates.extend(self.oversized.iter().cloned());
        } else {
            candidates.extend(self.entries.keys().cloned());
        }
        candidates.into_iter().filter(|id| self.entries.get(id).is_some_and(|candidate| candidate.intersects(&bounds))).collect()
    }

    fn covered_cells(&self, bounds: CollisionAabb) -> Option<Vec<(i32, i32, i32)>> {
        let cell = |value: f32| (value / self.cell_size).floor().clamp(i32::MIN as f32, i32::MAX as f32) as i32;
        let min = [cell(bounds.min[0]), cell(bounds.min[1]), cell(bounds.min[2])];
        let max = [cell(bounds.max[0]), cell(bounds.max[1]), cell(bounds.max[2])];
        let spans = [0, 1, 2].map(|axis| u64::try_from(i64::from(max[axis]) - i64::from(min[axis]) + 1).unwrap_or(0));
        let count = spans.into_iter().try_fold(1_u64, u64::checked_mul).unwrap_or(u64::MAX);
        if count > Self::MAX_CELLS_PER_ENTRY {
            return None;
        }
        let mut cells = Vec::new();
        for x in min[0]..=max[0] {
            for y in min[1]..=max[1] {
                for z in min[2]..=max[2] {
                    cells.push((x, y, z));
                }
            }
        }
        Some(cells)
    }
}
//#endregion 🗺️BroadPhase

//#region ⏳️OverlapStateMachine
pub(crate) trait CollisionStepContext {
    fn is_cancelled(&self) -> bool;
    fn should_yield(&self) -> bool;
    fn consume_fuel(&mut self, units: u64);
}

impl CollisionStepContext for semio_framework_job::StepContext<'_> {
    fn is_cancelled(&self) -> bool {
        semio_framework_job::StepContext::is_cancelled(self)
    }

    fn should_yield(&self) -> bool {
        semio_framework_job::StepContext::should_yield(self)
    }

    fn consume_fuel(&mut self, units: u64) {
        semio_framework_job::StepContext::consume_fuel(self, units);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) enum CollisionOverlapStage {
    BroadPhaseInit,
    PartPairs,
    SampleInit,
    Sampling,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum CollisionStepResult {
    Pending,
    Cancelled,
    Complete { overlap: f64, rejected_early: bool },
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub(crate) struct CollisionOverlapState {
    pub(crate) stage: CollisionOverlapStage,
    pub(crate) part_a_cursor: usize,
    pub(crate) part_b_cursor: usize,
    pub(crate) sample_cursor: usize,
    pub(crate) inside_both: usize,
    pub(crate) last_sample: Option<[f32; 3]>,
    intersection_min: [f32; 3],
    intersection_max: [f32; 3],
    fallback_center: [f32; 3],
    intersection_volume: f64,
    sample_count: usize,
    sample_batch_size: usize,
    overlap_budget: f64,
    rng_state: u32,
    result: Option<f64>,
    rejected_early: bool,
}

impl CollisionOverlapState {
    pub(crate) fn new(sample_count: usize, sample_batch_size: usize, overlap_budget: f64) -> Self {
        assert!(sample_batch_size > 0);
        Self {
            stage: CollisionOverlapStage::BroadPhaseInit,
            part_a_cursor: 0,
            part_b_cursor: 0,
            sample_cursor: 0,
            inside_both: 0,
            last_sample: None,
            intersection_min: [0.0; 3],
            intersection_max: [0.0; 3],
            fallback_center: [0.0; 3],
            intersection_volume: 0.0,
            sample_count,
            sample_batch_size,
            overlap_budget,
            rng_state: 0x9e3779b9,
            result: None,
            rejected_early: false,
        }
    }

    pub(crate) fn checkpoint(&self) -> Self {
        self.clone()
    }

    pub(crate) fn resume(checkpoint: Self) -> Self {
        checkpoint
    }

    pub(crate) fn step<C: CollisionStepContext>(&mut self, context: &mut C, a: &CollisionBody, world_a: &Pose3d, b: &CollisionBody, world_b: &Pose3d) -> CollisionStepResult {
        if context.is_cancelled() {
            return CollisionStepResult::Cancelled;
        }
        if context.should_yield() {
            return CollisionStepResult::Pending;
        }
        let stage = self.stage;
        let result = match stage {
            CollisionOverlapStage::BroadPhaseInit => self.init_broad_phase(a, world_a, b, world_b),
            CollisionOverlapStage::PartPairs => self.step_part_pair(a, world_a, b, world_b),
            CollisionOverlapStage::SampleInit => self.init_samples(),
            CollisionOverlapStage::Sampling => self.step_samples(context, a, world_a, b, world_b),
            CollisionOverlapStage::Complete => self.complete_result(),
        };
        if stage != CollisionOverlapStage::Sampling {
            context.consume_fuel(1);
        }
        if context.is_cancelled() {
            CollisionStepResult::Cancelled
        } else {
            result
        }
    }

    fn init_broad_phase(&mut self, a: &CollisionBody, world_a: &Pose3d, b: &CollisionBody, world_b: &Pose3d) -> CollisionStepResult {
        let a = CollisionAabb::from_body(a, world_a);
        let b = CollisionAabb::from_body(b, world_b);
        self.intersection_min = [a.min[0].max(b.min[0]), a.min[1].max(b.min[1]), a.min[2].max(b.min[2])];
        self.intersection_max = [a.max[0].min(b.max[0]), a.max[1].min(b.max[1]), a.max[2].min(b.max[2])];
        self.fallback_center = [(a.min[0] + a.max[0] + b.min[0] + b.max[0]) * 0.25, (a.min[1] + a.max[1] + b.min[1] + b.max[1]) * 0.25, (a.min[2] + a.max[2] + b.min[2] + b.max[2]) * 0.25];
        if !a.intersects(&b) {
            return self.finish(0.0, false);
        }
        self.stage = CollisionOverlapStage::PartPairs;
        CollisionStepResult::Pending
    }

    fn step_part_pair(&mut self, a: &CollisionBody, world_a: &Pose3d, b: &CollisionBody, world_b: &Pose3d) -> CollisionStepResult {
        if self.part_a_cursor < a.parts.len() && self.part_b_cursor < b.parts.len() {
            let part_a = &a.parts[self.part_a_cursor];
            let part_b = &b.parts[self.part_b_cursor];
            let pose_a = world_a.semio_compose_rs(&part_a.local_pose);
            let pose_b = world_b.semio_compose_rs(&part_b.local_pose);
            self.advance_part_pair(b.parts.len());
            if shapes_intersect(&pose_a, &part_a.shape, &pose_b, &part_b.shape) {
                self.stage = CollisionOverlapStage::SampleInit;
            }
            return CollisionStepResult::Pending;
        }
        let center = Point3d::new(self.fallback_center[0], self.fallback_center[1], self.fallback_center[2]);
        if point_inside_body(a, world_a, center) && point_inside_body(b, world_b, center) {
            self.stage = CollisionOverlapStage::SampleInit;
            CollisionStepResult::Pending
        } else {
            self.finish(0.0, false)
        }
    }

    fn advance_part_pair(&mut self, b_part_count: usize) {
        self.part_b_cursor += 1;
        if self.part_b_cursor >= b_part_count {
            self.part_b_cursor = 0;
            self.part_a_cursor += 1;
        }
    }

    fn init_samples(&mut self) -> CollisionStepResult {
        let size = [self.intersection_max[0] - self.intersection_min[0], self.intersection_max[1] - self.intersection_min[1], self.intersection_max[2] - self.intersection_min[2]];
        self.intersection_volume = size[0] as f64 * size[1] as f64 * size[2] as f64;
        if self.intersection_volume <= SURFACE_CONTACT_MAX_AABB_VOLUME {
            return self.finish(0.0, false);
        }
        if self.sample_count == 0 {
            return self.finish(f64::NAN, false);
        }
        self.stage = CollisionOverlapStage::Sampling;
        CollisionStepResult::Pending
    }

    fn step_samples<C: CollisionStepContext>(&mut self, context: &mut C, a: &CollisionBody, world_a: &Pose3d, b: &CollisionBody, world_b: &Pose3d) -> CollisionStepResult {
        let batch_end = self.sample_cursor.saturating_add(self.sample_batch_size).min(self.sample_count);
        while self.sample_cursor < batch_end {
            if context.is_cancelled() {
                return CollisionStepResult::Cancelled;
            }
            if context.should_yield() {
                return CollisionStepResult::Pending;
            }
            let sample = [self.next_sample_axis(0), self.next_sample_axis(1), self.next_sample_axis(2)];
            self.last_sample = Some(sample);
            self.sample_cursor += 1;
            context.consume_fuel(1);
            let point = Point3d::new(sample[0], sample[1], sample[2]);
            if point_inside_body(a, world_a, point) && point_inside_body(b, world_b, point) {
                self.inside_both += 1;
                if self.estimated_overlap() > self.overlap_budget {
                    return self.finish(self.overlap_budget + 1.0, true);
                }
            }
        }
        if self.sample_cursor == self.sample_count {
            self.finish(self.estimated_overlap(), false)
        } else {
            CollisionStepResult::Pending
        }
    }

    fn next_sample_axis(&mut self, axis: usize) -> f32 {
        self.rng_state = self.rng_state.wrapping_mul(1664525).wrapping_add(1013904223);
        let ratio = self.rng_state as f64 / u32::MAX as f64;
        self.intersection_min[axis] + (self.intersection_max[axis] - self.intersection_min[axis]) * ratio as f32
    }

    fn estimated_overlap(&self) -> f64 {
        (self.inside_both as f64 / self.sample_count as f64) * self.intersection_volume
    }

    fn finish(&mut self, overlap: f64, rejected_early: bool) -> CollisionStepResult {
        self.stage = CollisionOverlapStage::Complete;
        self.result = Some(overlap);
        self.rejected_early = rejected_early;
        CollisionStepResult::Complete { overlap, rejected_early }
    }

    fn complete_result(&self) -> CollisionStepResult {
        CollisionStepResult::Complete { overlap: self.result.unwrap_or(0.0), rejected_early: self.rejected_early }
    }
}
//#endregion ⏳️OverlapStateMachine
//#endregion 🔖️Collision

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::puzzle3d::schema::testkit::*;
    use std::time::{Duration, Instant};

    struct TestStepContext {
        cancelled: bool,
        yield_now: bool,
        fuel: u64,
    }

    impl TestStepContext {
        fn unlimited() -> Self {
            Self { cancelled: false, yield_now: false, fuel: u64::MAX }
        }
    }

    impl CollisionStepContext for TestStepContext {
        fn is_cancelled(&self) -> bool {
            self.cancelled
        }

        fn should_yield(&self) -> bool {
            self.yield_now || self.fuel == 0
        }

        fn consume_fuel(&mut self, units: u64) {
            self.fuel = self.fuel.saturating_sub(units);
        }
    }

    fn overlap_body() -> CollisionBody {
        let (positions, indices) = outward_wound_unit_cube_mesh_buffers();
        collision_body_from_buffers(&positions, &indices).expect("body")
    }

    fn drive_overlap(mut state: CollisionOverlapState, a: &CollisionBody, world_a: &Pose3d, b: &CollisionBody, world_b: &Pose3d) -> (CollisionOverlapState, CollisionStepResult) {
        let mut context = TestStepContext::unlimited();
        for _ in 0..100_000 {
            let result = state.step(&mut context, a, world_a, b, world_b);
            if !matches!(result, CollisionStepResult::Pending) {
                return (state, result);
            }
        }
        panic!("collision overlap state did not terminate");
    }

    #[test]
    fn world_volumes_contain_aabb_respects_oriented_box() {
        let volumes = vec![WorldVolumeProps { id: "v1".to_string(), origin: [0.0, 0.0, 0.0], orientation: None, scale: Some(dsl::DslValue::Array(vec![dsl::DslValue::Number(4.0), dsl::DslValue::Number(4.0), dsl::DslValue::Number(4.0)])) }];
        let min = Point3d::new(-1.0, -1.0, -1.0);
        let max = Point3d::new(1.0, 1.0, 1.0);
        assert!(world_volumes_contain_aabb(&volumes, min, max));
        let outside_min = Point3d::new(-3.0, -3.0, -3.0);
        let outside_max = Point3d::new(3.0, 3.0, 3.0);
        assert!(!world_volumes_contain_aabb(&volumes, outside_min, outside_max));
    }

    #[test]
    fn vec3d_and_point3d_basic_ops() {
        let a = Vec3d::new(1.0, 2.0, 3.0);
        let b = Vec3d::new(4.0, -1.0, 0.5);
        let sum = a + b;
        assert_eq!((sum.x(), sum.y(), sum.z()), (5.0, 1.0, 3.5));
        let scaled = a * 2.0;
        assert_eq!((scaled.x(), scaled.y(), scaled.z()), (2.0, 4.0, 6.0));
        assert_eq!(Vec3d::new(-5.0, 3.0, -1.0).amax(), 5.0);

        let p1 = Point3d::new(1.0, 5.0, -2.0);
        let p2 = Point3d::new(4.0, 2.0, 3.0);
        let inf = p1.inf(&p2);
        let sup = p1.sup(&p2);
        assert_eq!((inf.x(), inf.y(), inf.z()), (1.0, 2.0, -2.0));
        assert_eq!((sup.x(), sup.y(), sup.z()), (4.0, 5.0, 3.0));
        let diff = p2 - p1;
        assert_eq!((diff.x(), diff.y(), diff.z()), (3.0, -3.0, 5.0));
        let back = Point3d::from_coords(diff);
        assert_eq!((back.x(), back.y(), back.z()), (3.0, -3.0, 5.0));
    }

    #[test]
    fn rotation3d_identity_ijkw_roundtrip_and_apply() {
        let identity = Rotation3d::identity();
        assert_eq!(identity.to_ijkw(), (0.0, 0.0, 0.0, 1.0));
        let q = Rotation3d::from_ijkw(0.0, 0.0, 0.0, 1.0);
        let v = Vec3d::new(1.0, 0.0, 0.0);
        let rotated = q.apply(v);
        assert_eq!((rotated.x(), rotated.y(), rotated.z()), (1.0, 0.0, 0.0));
    }

    #[test]
    fn rotation3d_rotation_between_none_for_antiparallel() {
        let from = Vec3d::new(1.0, 0.0, 0.0);
        let to = Vec3d::new(-1.0, 0.0, 0.0);
        assert!(Rotation3d::rotation_between(from, to).is_none(), "opposite vectors have no unique rotation axis");
        let to2 = Vec3d::new(0.0, 1.0, 0.0);
        assert!(Rotation3d::rotation_between(from, to2).is_some());
    }

    #[test]
    fn pose3d_compose_inverse_transform_point() {
        let rotation = Rotation3d::from_ijkw(0.0, 0.0, 0.0, 1.0);
        let translation = Vec3d::new(1.0, 2.0, 3.0);
        let pose = Pose3d::from_parts(translation, rotation);
        let point = Point3d::new(0.0, 0.0, 0.0);
        let transformed = pose.transform_point(&point);
        assert_eq!((transformed.x(), transformed.y(), transformed.z()), (1.0, 2.0, 3.0));
        let back = pose.inverse().transform_point(&transformed);
        assert!(back.x().abs() < 1e-6 && back.y().abs() < 1e-6 && back.z().abs() < 1e-6);
        let composed = pose.semio_compose_rs(&Pose3d::identity());
        let composed_point = composed.transform_point(&point);
        assert_eq!((composed_point.x(), composed_point.y(), composed_point.z()), (1.0, 2.0, 3.0));
    }

    #[test]
    fn normalize_vec3_handles_zero_length() {
        assert_eq!(normalize_vec3([0.0, 0.0, 0.0]), [0.0, 0.0, -1.0]);
        let n = normalize_vec3([3.0, 0.0, 4.0]);
        assert!((n[0] - 0.6).abs() < 1e-9 && (n[2] - 0.8).abs() < 1e-9);
    }

    #[test]
    fn vec3_math_helpers() {
        assert_eq!(vec3_dot([1.0, 2.0, 3.0], [4.0, 5.0, 6.0]), 32.0);
        assert_eq!(vec3_cross([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]), [0.0, 0.0, 1.0]);
        assert_eq!(vec3_add([1.0, 2.0, 3.0], [1.0, 1.0, 1.0]), [2.0, 3.0, 4.0]);
        assert_eq!(vec3_sub([1.0, 2.0, 3.0], [1.0, 1.0, 1.0]), [0.0, 1.0, 2.0]);
        assert_eq!(negate_vec3([1.0, -2.0, 3.0]), [-1.0, 2.0, -3.0]);
    }

    #[test]
    fn vec3_scale_variants() {
        assert_eq!(vec3_scale([1.0, 2.0, 3.0], &None), [1.0, 2.0, 3.0]);
        assert_eq!(vec3_scale([1.0, 2.0, 3.0], &Some(dsl::DslValue::Number(2.0))), [2.0, 4.0, 6.0]);
        assert_eq!(vec3_scale([1.0, 2.0, 3.0], &Some(dsl::DslValue::Array(vec![dsl::DslValue::Number(2.0), dsl::DslValue::Number(3.0), dsl::DslValue::Number(4.0)]))), [2.0, 6.0, 12.0]);
        assert_eq!(vec3_scale([1.0, 2.0, 3.0], &Some(dsl::DslValue::String("bogus".to_string()))), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn quat_rotate_vec_and_180_degree_axis() {
        let identity: Quat = [0.0, 0.0, 0.0, 1.0];
        let rotated = quat_rotate_vec(identity, [1.0, 2.0, 3.0]);
        assert!((rotated[0] - 1.0).abs() < 1e-9 && (rotated[1] - 2.0).abs() < 1e-9 && (rotated[2] - 3.0).abs() < 1e-9);
        let q = quaternion_from_180_degree_axis([0.0, 0.0, 2.0]);
        assert_eq!(q, [0.0, 0.0, 1.0, 0.0]);
    }

    #[test]
    fn anti_parallel_brush_orientation_branches() {
        let in_plane = anti_parallel_brush_orientation([1.0, 0.0, 0.0]);
        assert_eq!(in_plane, quaternion_from_180_degree_axis([0.0, 0.0, 1.0]), "near-planar target direction falls back to the z axis");
        let along_z = anti_parallel_brush_orientation([0.0, 0.0, 1.0]);
        assert_eq!(along_z, quaternion_from_180_degree_axis([1.0, 0.0, 0.0]), "target parallel to z has an undefined cross axis, falls back to x");
        let general = anti_parallel_brush_orientation([0.0, 1.0, 0.5]);
        assert_eq!(general, quaternion_from_180_degree_axis([-1.0, 0.0, 0.0]), "general case uses cross(z, target)");
    }

    #[test]
    fn compute_brush_placement_pose_host_orientation_branch() {
        let host_orientation: Quat = [0.0, 0.0, 0.0, 1.0];
        let (origin, orientation) = compute_brush_placement_pose([1.0, 0.0, 0.0], [0.0, 0.0, -1.0], &None, [10.0, 0.0, 0.0], [0.0, 0.0, 1.0], Some(host_orientation), true);
        assert_eq!(orientation, host_orientation, "an antiparallel host-source direction keeps the host's own orientation");
        assert_eq!(origin, [9.0, 0.0, 0.0]);
    }

    #[test]
    fn compute_brush_placement_pose_falls_back_without_reference_orientation() {
        let (_, orientation) = compute_brush_placement_pose([0.0, 0.0, 0.0], [0.0, 0.0, -1.0], &None, [0.0, 0.0, 0.0], [0.0, 0.0, -1.0], None, true);
        let anti = anti_parallel_brush_orientation([0.0, 0.0, -1.0]);
        assert_eq!(orientation, anti, "use_host_orientation with no reference orientation must fall through to the general path");
    }

    #[test]
    fn compute_brush_placement_pose_general_rotation_between() {
        let (origin, orientation) = compute_brush_placement_pose([0.0, 0.0, 0.0], [0.0, 0.0, -1.0], &None, [5.0, 0.0, 0.0], [1.0, 0.0, 0.0], None, false);
        let rotated = quat_rotate_vec(orientation, [0.0, 0.0, -1.0]);
        assert!((rotated[0] + 1.0).abs() < 1e-4, "local dir must rotate onto the desired world dir: {rotated:?}");
        assert!((origin[0] - 5.0).abs() < 1e-6);
    }

    #[test]
    fn collision_body_from_buffers_rejects_too_few_or_degenerate() {
        assert!(collision_body_from_buffers(&[0.0; 6], &[0, 1, 2]).is_none(), "fewer than 3 vertices must be rejected");
        assert!(collision_body_from_buffers(&[0.0; 9], &[0, 1]).is_none(), "fewer than one triangle's worth of indices must be rejected");
        let tiny_positions: Vec<f32> = vec![0.0, 0.0, 0.0, 0.1, 0.0, 0.0, 0.0, 0.1, 0.0];
        assert!(collision_body_from_buffers(&tiny_positions, &[0, 1, 2]).is_none(), "extent below the minimum collision mesh extent must be rejected");
    }

    #[test]
    fn collision_body_from_buffers_accepts_valid_mesh() {
        let (positions, indices) = unit_cube_mesh_buffers();
        let scaled: Vec<f32> = positions.iter().map(|c| c * 4.0).collect();
        let body = collision_body_from_buffers(&scaled, &indices).expect("valid mesh should build a body");
        assert_eq!(body.parts.len(), 1);
        assert_eq!((body.local_bounds_min.x(), body.local_bounds_max.x()), (-4.0, 4.0));
    }

    #[test]
    fn world_bounds_transforms_local_aabb_corners() {
        let (positions, indices) = unit_cube_mesh_buffers();
        let body = collision_body_from_buffers(&positions, &indices).expect("body");
        let pose = Pose3d::from_parts(Vec3d::new(10.0, 0.0, 0.0), Rotation3d::identity());
        let (min, max) = world_bounds(&body, &pose);
        assert_eq!((min.x(), max.x()), (9.0, 11.0));
    }

    #[test]
    fn world_volumes_contain_aabb_empty_and_multi_volume() {
        assert!(world_volumes_contain_aabb(&[], Point3d::new(-1.0, -1.0, -1.0), Point3d::new(1.0, 1.0, 1.0)), "no target volumes means unconstrained");
        let volumes = vec![WorldVolumeProps { id: "far".into(), origin: [100.0, 0.0, 0.0], orientation: None, scale: None }, WorldVolumeProps { id: "near".into(), origin: [0.0, 0.0, 0.0], orientation: None, scale: Some(dsl::DslValue::Number(4.0)) }];
        assert!(world_volumes_contain_aabb(&volumes, Point3d::new(-1.0, -1.0, -1.0), Point3d::new(1.0, 1.0, 1.0)), "any single containing volume is enough");
    }

    #[test]
    fn overlap_state_rejects_disjoint_aabbs() {
        let (positions, indices) = unit_cube_mesh_buffers();
        let body = collision_body_from_buffers(&positions, &indices).expect("body");
        let pose_a = Pose3d::identity();
        let pose_b = Pose3d::from_parts(Vec3d::new(100.0, 0.0, 0.0), Rotation3d::identity());
        assert!(!bodies_intersect(&body, &pose_a, &body, &pose_b));
        let (_, result) = drive_overlap(CollisionOverlapState::new(64, 8, 0.02), &body, &pose_a, &body, &pose_b);
        assert_eq!(result, CollisionStepResult::Complete { overlap: 0.0, rejected_early: false });
    }

    #[test]
    fn overlap_state_reports_positive_overlap_for_coincident_bodies() {
        let (positions, indices) = outward_wound_unit_cube_mesh_buffers();
        let scaled: Vec<f32> = positions.iter().map(|c| c * 4.0).collect();
        let body = collision_body_from_buffers(&scaled, &indices).expect("body");
        let pose = Pose3d::identity();
        assert!(point_inside_body(&body, &pose, Point3d::new(0.0, 0.0, 0.0)), "the box's own center must be inside itself");
        let (_, result) = drive_overlap(CollisionOverlapState::new(256, 16, f64::INFINITY), &body, &pose, &body, &pose);
        let CollisionStepResult::Complete { overlap, .. } = result else { panic!("overlap state must complete") };
        assert!(overlap > 0.0, "two fully coincident solid bodies must report a positive overlap: {overlap}");
    }

    #[test]
    fn overlap_is_deterministic_across_batch_sizes() {
        let body = overlap_body();
        let pose = Pose3d::identity();
        let runs = [1, 7, 64].map(|batch| drive_overlap(CollisionOverlapState::new(257, batch, f64::INFINITY), &body, &pose, &body, &pose));
        let overlaps = runs.map(|(_, result)| match result {
            CollisionStepResult::Complete { overlap, rejected_early: false } => overlap,
            other => panic!("unexpected result: {other:?}"),
        });
        assert_eq!(overlaps[0], overlaps[1]);
        assert_eq!(overlaps[1], overlaps[2]);
    }

    #[test]
    fn overlap_checkpoint_resumes_exact_rng_and_sample_cursor() {
        let body = overlap_body();
        let pose = Pose3d::identity();
        let mut state = CollisionOverlapState::new(257, 3, f64::INFINITY);
        let mut context = TestStepContext::unlimited();
        while state.sample_cursor < 12 {
            assert_eq!(state.step(&mut context, &body, &pose, &body, &pose), CollisionStepResult::Pending);
        }
        let checkpoint = state.checkpoint();
        let (finished, result) = drive_overlap(state, &body, &pose, &body, &pose);
        let (resumed, resumed_result) = drive_overlap(CollisionOverlapState::resume(checkpoint), &body, &pose, &body, &pose);
        assert_eq!(result, resumed_result);
        assert_eq!(finished, resumed);
    }

    #[test]
    fn overlap_touching_surfaces_are_not_solid_overlap() {
        let body = overlap_body();
        let pose_a = Pose3d::identity();
        let pose_b = Pose3d::from_parts(Vec3d::new(2.0, 0.0, 0.0), Rotation3d::identity());
        let (_, result) = drive_overlap(CollisionOverlapState::new(257, 8, f64::INFINITY), &body, &pose_a, &body, &pose_b);
        assert_eq!(result, CollisionStepResult::Complete { overlap: 0.0, rejected_early: false });
    }

    #[test]
    fn overlap_rejects_early_against_budget() {
        let body = overlap_body();
        let pose = Pose3d::identity();
        let (state, result) = drive_overlap(CollisionOverlapState::new(4096, 32, 0.0), &body, &pose, &body, &pose);
        assert_eq!(result, CollisionStepResult::Complete { overlap: 1.0, rejected_early: true });
        assert!(state.sample_cursor < 4096);
    }

    #[test]
    fn overlap_cancellation_and_yield_preserve_state() {
        let body = overlap_body();
        let pose = Pose3d::identity();
        let mut state = CollisionOverlapState::new(64, 8, f64::INFINITY);
        let before = state.checkpoint();
        let mut cancelled = TestStepContext { cancelled: true, yield_now: false, fuel: 100 };
        assert_eq!(state.step(&mut cancelled, &body, &pose, &body, &pose), CollisionStepResult::Cancelled);
        assert_eq!(state, before);
        let mut yielding = TestStepContext { cancelled: false, yield_now: true, fuel: 100 };
        assert_eq!(state.step(&mut yielding, &body, &pose, &body, &pose), CollisionStepResult::Pending);
        assert_eq!(state, before);
        let mut no_fuel = TestStepContext { cancelled: false, yield_now: false, fuel: 0 };
        assert_eq!(state.step(&mut no_fuel, &body, &pose, &body, &pose), CollisionStepResult::Pending);
        assert_eq!(state, before);
    }

    #[test]
    fn spatial_index_queries_are_exact_and_ordered() {
        let mut index = CollisionSpatialIndex::new(2.0);
        let near = CollisionAabb { min: [-1.0; 3], max: [1.0; 3] };
        let far = CollisionAabb { min: [10.0; 3], max: [12.0; 3] };
        index.upsert("zeta".into(), near);
        index.upsert("alpha".into(), near);
        index.upsert("far".into(), far);
        assert_eq!(index.query(near), vec!["alpha".to_string(), "zeta".to_string()]);
        index.upsert("zeta".into(), far);
        assert_eq!(index.query(near), vec!["alpha".to_string()]);
        assert!(index.remove("alpha"));
        assert!(!index.remove("missing"));
        assert!(index.query(near).is_empty());
    }

    #[test]
    fn spatial_index_bounds_adversarial_cell_spans() {
        let mut index = CollisionSpatialIndex::new(1.0);
        let world = CollisionAabb { min: [-1.0e9; 3], max: [1.0e9; 3] };
        let near = CollisionAabb { min: [-1.0; 3], max: [1.0; 3] };
        index.upsert("world".into(), world);
        index.upsert("near".into(), near);
        assert_eq!(index.query(near), vec!["near".to_string(), "world".to_string()]);
        assert_eq!(index.query(world), vec!["near".to_string(), "world".to_string()]);
        assert!(index.remove("world"));
    }

    #[test]
    fn overlap_sample_steps_stay_within_interaction_watchdog() {
        let body = overlap_body();
        let pose = Pose3d::identity();
        let mut state = CollisionOverlapState::new(128, 1, f64::INFINITY);
        let mut context = TestStepContext::unlimited();
        while state.stage != CollisionOverlapStage::Sampling {
            assert_eq!(state.step(&mut context, &body, &pose, &body, &pose), CollisionStepResult::Pending);
        }
        for _ in 0..32 {
            let started = Instant::now();
            let result = state.step(&mut context, &body, &pose, &body, &pose);
            assert!(started.elapsed() < Duration::from_millis(8), "one-sample collision step exceeded the 8 ms interaction ceiling");
            assert!(matches!(result, CollisionStepResult::Pending | CollisionStepResult::Complete { .. }));
        }
    }
}
//#endregion 🧪️Tests
