//! 📐️ Puzzle 3d artifact engine — the geometry layer: the `nalgebra`/`parry3d` adapter (the ONE
//! interface boundary this artifact depends on), the plain `[f64; 3]`/`[f64; 4]` vector and
//! quaternion math the placement solver builds on, the brush placement pose solver itself, and the
//! collision-body/AABB/overlap primitives the brush and fill lanes gate placements with.

use crate::artifacts::puzzle3d::engine::{Quat, Vec3, WorldVolumeProps};

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
pub(crate) struct CollisionMeshPart {
    pub(crate) shape: CollisionShape,
    pub(crate) local_pose: Pose3d,
}

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

pub(crate) fn solid_overlap_volume(a: &CollisionBody, world_a: &Pose3d, b: &CollisionBody, world_b: &Pose3d, sample_count: usize, overlap_budget: f64) -> f64 {
    let (amin, amax) = world_bounds(a, world_a);
    let (bmin, bmax) = world_bounds(b, world_b);
    let imin = Point3d::new(amin.x().max(bmin.x()), amin.y().max(bmin.y()), amin.z().max(bmin.z()));
    let imax = Point3d::new(amax.x().min(bmax.x()), amax.y().min(bmax.y()), amax.z().min(bmax.z()));
    if imax.x() < imin.x() || imax.y() < imin.y() || imax.z() < imin.z() {
        return 0.0;
    }
    if !bodies_intersect(a, world_a, b, world_b) {
        return 0.0;
    }
    let size = imax - imin;
    let box_vol = size.x() as f64 * size.y() as f64 * size.z() as f64;
    if box_vol <= SURFACE_CONTACT_MAX_AABB_VOLUME {
        return 0.0;
    }
    let mut inside_both = 0usize;
    let mut state = 0x9e3779b9u32;
    for _ in 0..sample_count {
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        let rx = (state as f64) / (u32::MAX as f64);
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        let ry = (state as f64) / (u32::MAX as f64);
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        let rz = (state as f64) / (u32::MAX as f64);
        let p = Point3d::new(imin.x() + (imax.x() - imin.x()) * rx as f32, imin.y() + (imax.y() - imin.y()) * ry as f32, imin.z() + (imax.z() - imin.z()) * rz as f32);
        if point_inside_body(a, world_a, p) && point_inside_body(b, world_b, p) {
            inside_both += 1;
            if (inside_both as f64 / sample_count as f64) * box_vol > overlap_budget {
                return overlap_budget + 1.0;
            }
        }
    }
    (inside_both as f64 / sample_count as f64) * box_vol
}
//#endregion 🔖️Collision

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::puzzle3d::engine::testkit::*;

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
    fn bodies_intersect_and_solid_overlap_volume_reject_disjoint_aabbs() {
        let (positions, indices) = unit_cube_mesh_buffers();
        let body = collision_body_from_buffers(&positions, &indices).expect("body");
        let pose_a = Pose3d::identity();
        let pose_b = Pose3d::from_parts(Vec3d::new(100.0, 0.0, 0.0), Rotation3d::identity());
        assert!(!bodies_intersect(&body, &pose_a, &body, &pose_b));
        assert_eq!(solid_overlap_volume(&body, &pose_a, &body, &pose_b, 64, 0.02), 0.0);
    }

    #[test]
    fn solid_overlap_volume_reports_positive_overlap_for_coincident_bodies() {
        let (positions, indices) = outward_wound_unit_cube_mesh_buffers();
        let scaled: Vec<f32> = positions.iter().map(|c| c * 4.0).collect();
        let body = collision_body_from_buffers(&scaled, &indices).expect("body");
        let pose = Pose3d::identity();
        assert!(point_inside_body(&body, &pose, Point3d::new(0.0, 0.0, 0.0)), "the box's own center must be inside itself");
        let overlap = solid_overlap_volume(&body, &pose, &body, &pose, 256, 0.0);
        assert!(overlap > 0.0, "two fully coincident solid bodies must report a positive overlap: {overlap}");
    }
}
//#endregion 🧪️Tests
