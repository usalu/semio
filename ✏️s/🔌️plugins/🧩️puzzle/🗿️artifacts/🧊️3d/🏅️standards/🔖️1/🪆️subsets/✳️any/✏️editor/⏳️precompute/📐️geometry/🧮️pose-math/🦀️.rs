//! 🧮️ Pose math of the precompute geometry layer: the rigid vector, point, rotation and pose adapter, the collision mesh shape and the brush placement pose solver.

use super::*;

//#region 🔒️GeometryAdapter
/// 🔒️ Thin wrappers over `semio_framework_3d::{rigid, collision}` — the one interface boundary
/// this artifact depends on.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3d(pub(super) rigid::Vector3);

impl Vec3d {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(rigid::Vector3::new(x, y, z))
    }
    pub fn x(&self) -> f32 {
        self.0.x
    }
    pub fn y(&self) -> f32 {
        self.0.y
    }
    pub fn z(&self) -> f32 {
        self.0.z
    }
    pub fn amax(&self) -> f32 {
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
pub struct Point3d(pub(super) rigid::Point3);

semio_framework_value::artifact_retire_leaf!(Point3d);

impl Point3d {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(rigid::Point3::new(x, y, z))
    }
    pub fn x(&self) -> f32 {
        self.0.x
    }
    pub fn y(&self) -> f32 {
        self.0.y
    }
    pub fn z(&self) -> f32 {
        self.0.z
    }
    pub fn inf(&self, other: &Self) -> Self {
        Self(self.0.inf(other.0))
    }
    pub fn sup(&self, other: &Self) -> Self {
        Self(self.0.sup(other.0))
    }
    #[cfg(test)]
    pub fn coords(&self) -> Vec3d {
        Vec3d(self.0.coords())
    }
    #[cfg(test)]
    pub fn from_coords(v: Vec3d) -> Self {
        Self(rigid::Point3::from_coords(v.0))
    }
}

impl std::ops::Sub for Point3d {
    type Output = Vec3d;
    fn sub(self, rhs: Self) -> Vec3d {
        Vec3d(self.0 - rhs.0)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rotation3d(pub(super) rigid::UnitQuaternion);

impl Rotation3d {
    pub fn identity() -> Self {
        Self(rigid::UnitQuaternion::identity())
    }
    /// 🔓️ Builds from CAD's `[i, j, k, w]` quaternion convention.
    pub fn from_ijkw(i: f32, j: f32, k: f32, w: f32) -> Self {
        Self(rigid::UnitQuaternion::from_quaternion(rigid::Quaternion::new(w, i, j, k)))
    }
    pub fn to_ijkw(self) -> (f32, f32, f32, f32) {
        let q = self.0.quaternion();
        (q.i, q.j, q.k, q.w)
    }
    pub fn rotation_between(from: Vec3d, to: Vec3d) -> Option<Self> {
        rigid::UnitQuaternion::rotation_between(from.0, to.0).map(Self)
    }
    pub fn apply(&self, v: Vec3d) -> Vec3d {
        Vec3d(self.0.apply(v.0))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Pose3d(pub(super) rigid::Isometry3);

semio_framework_value::artifact_retire_leaf!(Pose3d);

impl Pose3d {
    pub fn identity() -> Self {
        Self(rigid::Isometry3::identity())
    }
    pub fn from_parts(translation: Vec3d, rotation: Rotation3d) -> Self {
        Self(rigid::Isometry3::from_parts(translation.0, rotation.0))
    }
    pub fn inverse(&self) -> Self {
        Self(self.0.inverse())
    }
    pub fn transform_point(&self, point: &Point3d) -> Point3d {
        Point3d(self.0.transform_point(point.0))
    }
    pub fn semio_compose_rs(&self, other: &Self) -> Self {
        Self(self.0.compose(other.0))
    }
}

/// 🔺️ The shared triangle mesh of one collision shape. Its retirement releases a last share buffer by buffer: the vertex array, the
/// triangle array, then the BVH one node per turn; a share that is not the last releases only its handle.
#[derive(Clone)]
pub struct CollisionMesh(std::sync::Arc<collision::TriMesh>);

impl std::ops::Deref for CollisionMesh {
    type Target = collision::TriMesh;

    fn deref(&self) -> &collision::TriMesh {
        &self.0
    }
}

const COLLISION_MESH_TREE_STACK: usize = 64;

struct CollisionMeshRetirement {
    shared: std::mem::ManuallyDrop<Option<std::sync::Arc<collision::TriMesh>>>,
    vertices: std::mem::ManuallyDrop<Vec<rigid::Point3>>,
    triangles: std::mem::ManuallyDrop<Vec<[u32; 3]>>,
    tree: std::mem::ManuallyDrop<Vec<collision::BvhTree>>,
}

impl CollisionMeshRetirement {
    fn birth_bytes() -> usize {
        size_of::<Self>() + COLLISION_MESH_TREE_STACK * size_of::<collision::BvhTree>()
    }
}

impl semio_framework_value::retirement::RetirementCursor for CollisionMeshRetirement {
    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_value::retirement::RetirementStep {
        use semio_framework_value::retirement::RetirementStep;
        if self.terminal_is_empty() {
            return RetirementStep::Complete;
        }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return RetirementStep::BudgetExhausted;
        }
        if let Some(shared) = self.shared.take() {
            match std::sync::Arc::try_unwrap(shared) {
                Ok(mesh) => {
                    let parts = mesh.into_parts();
                    *self.vertices = parts.vertices;
                    *self.triangles = parts.triangles;
                    self.tree.extend(parts.bvh);
                }
                Err(handle) => drop(handle),
            }
            return RetirementStep::Advanced;
        }
        let demand = self.next_close_byte_demand().unwrap_or(0);
        if demand > grant.maximum_release_bytes {
            return RetirementStep::BudgetExhausted;
        }
        if self.vertices.capacity() != 0 {
            drop(std::mem::take(&mut *self.vertices));
            return RetirementStep::Bytes(demand);
        }
        if self.triangles.capacity() != 0 {
            drop(std::mem::take(&mut *self.triangles));
            return RetirementStep::Bytes(demand);
        }
        let Some(node) = self.tree.pop() else { return RetirementStep::Complete };
        match node.into_step() {
            collision::BvhTreeStep::Leaf => {}
            collision::BvhTreeStep::Branch(left, right) => {
                if self.tree.len() + 2 > self.tree.capacity() {
                    return RetirementStep::Failure(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "collision mesh tree exceeds its retirement stack"));
                }
                self.tree.push(right);
                self.tree.push(left);
            }
        }
        RetirementStep::Bytes(demand)
    }

    fn terminal_is_empty(&self) -> bool {
        self.shared.is_none() && self.vertices.capacity() == 0 && self.triangles.capacity() == 0 && self.tree.is_empty()
    }

    fn next_close_byte_demand(&self) -> Option<usize> {
        Some(if self.shared.is_some() {
            0
        } else if self.vertices.capacity() != 0 {
            self.vertices.capacity() * size_of::<rigid::Point3>()
        } else if self.triangles.capacity() != 0 {
            self.triangles.capacity() * size_of::<[u32; 3]>()
        } else if self.tree.is_empty() {
            0
        } else {
            collision::BvhTree::NODE_BYTES
        })
    }

    fn next_birth_bytes(&self, _: usize) -> Option<usize> {
        Some(0)
    }

    fn terminal_release_bytes(&self) -> Option<usize> {
        Some(Self::birth_bytes())
    }
}

impl Drop for CollisionMeshRetirement {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || semio_framework_value::retirement::RetirementCursor::terminal_is_empty(self), "a collision mesh must retire before it drops");
        unsafe {
            std::mem::ManuallyDrop::drop(&mut self.shared);
            std::mem::ManuallyDrop::drop(&mut self.vertices);
            std::mem::ManuallyDrop::drop(&mut self.triangles);
            std::mem::ManuallyDrop::drop(&mut self.tree);
        }
    }
}

impl semio_framework_value::retirement::RetireOwned for CollisionMesh {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        Box::new(CollisionMeshRetirement {
            shared: std::mem::ManuallyDrop::new(Some(self.0)),
            vertices: std::mem::ManuallyDrop::new(Vec::new()),
            triangles: std::mem::ManuallyDrop::new(Vec::new()),
            tree: std::mem::ManuallyDrop::new(Vec::with_capacity(COLLISION_MESH_TREE_STACK)),
        })
    }

    fn retirement_birth_bytes(&self) -> Option<usize> {
        Some(CollisionMeshRetirement::birth_bytes())
    }

    fn controlled_retirement_supported() -> bool {
        true
    }
}

#[derive(Clone, semio_framework_value::RetireOwned)]
pub struct CollisionShape {
    pub(super) shape: CollisionMesh,
}

impl CollisionShape {
    pub fn from_triangle_mesh(vertices: &[Point3d], indices: Vec<[u32; 3]>) -> Self {
        let verts: Vec<rigid::Point3> = vertices.iter().map(|p| p.0).collect();
        Self { shape: CollisionMesh(std::sync::Arc::new(collision::TriMesh::new(verts, indices))) }
    }
    pub fn contains_point(&self, pose: &Pose3d, point: &Point3d) -> bool {
        collision::contains_point(pose.0, &self.shape, point.0)
    }
}

pub(super) fn shapes_intersect(pose_a: &Pose3d, a: &CollisionShape, pose_b: &Pose3d, b: &CollisionShape) -> bool {
    collision::intersection_test(pose_a.0, &a.shape, pose_b.0, &b.shape)
}
//#endregion 🔒️GeometryAdapter

//#region 🔖️Constants
pub(super) const BRUSH_COLLISION_MESH_MIN_EXTENT: f64 = 2.0;

/// 🕳️ Distance (m) below which a probe is contact, not penetration, whatever the tolerance: single-precision noise of
/// faces that meet exactly (1e-7 m measured on docked cubes) must never read as a collision.
pub(super) const COLLISION_CONTACT_NOISE_M: f64 = 1e-4;

/// 🕳️ Probes per edge of a clipped face polygon (its first corner included), see `CollisionPenetrationState::step_face`.
pub(super) const COLLISION_EDGE_PROBES: usize = 8;

/// 🕳️ Smallest and largest half inward offset of a coincident-surface probe (m): a zero tolerance still probes off the
/// surface, and an unbounded measure (an infinite tolerance) still probes a real point instead of one at infinity.
pub(super) const COLLISION_INSET_MIN_M: f64 = 1e-4;

pub(super) const COLLISION_INSET_MAX_M: f64 = 0.05;

pub const BRUSH_PLACEMENT_PARALLEL_TOLERANCE: f64 = 1e-6;
//#endregion 🔖️Constants

//#region 🔖️Vectors
pub fn normalize_vec3(v: Vec3) -> Vec3 {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len < 1e-9 {
        return [0.0, 0.0, -1.0];
    }
    [v[0] / len, v[1] / len, v[2] / len]
}

pub fn vec3_dot(a: Vec3, b: Vec3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub fn vec3_cross(a: Vec3, b: Vec3) -> Vec3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

pub fn vec3_add(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

pub fn vec3_sub(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

pub fn negate_vec3(v: Vec3) -> Vec3 {
    [-v[0], -v[1], -v[2]]
}

pub fn vec3_scale(v: Vec3, scale: &Option<semio_framework_value::DslValue>) -> Vec3 {
    match scale {
        None => v,
        Some(semio_framework_value::DslValue::Number(n)) => {
            let s = n.as_f64();
            [v[0] * s, v[1] * s, v[2] * s]
        }
        Some(semio_framework_value::DslValue::Array(arr)) if arr.len() >= 3 => {
            let sx = arr[0].as_f64().unwrap_or(1.0);
            let sy = arr[1].as_f64().unwrap_or(1.0);
            let sz = arr[2].as_f64().unwrap_or(1.0);
            [v[0] * sx, v[1] * sy, v[2] * sz]
        }
        _ => v,
    }
}

pub fn unit_quat_from_cad(q: Quat) -> Rotation3d {
    Rotation3d::from_ijkw(q[0] as f32, q[1] as f32, q[2] as f32, q[3] as f32)
}

pub fn quat_rotate_vec(q: Quat, v: Vec3) -> Vec3 {
    let uq = unit_quat_from_cad(q);
    let rotated = uq.apply(Vec3d::new(v[0] as f32, v[1] as f32, v[2] as f32));
    [rotated.x() as f64, rotated.y() as f64, rotated.z() as f64]
}

pub fn quaternion_from_180_degree_axis(axis: Vec3) -> Quat {
    let unit = normalize_vec3(axis);
    [unit[0], unit[1], unit[2], 0.0]
}

pub fn anti_parallel_brush_orientation(target_dir: Vec3) -> Quat {
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

pub fn pose_isometry(origin: Vec3, orientation: Quat, _scale: &Option<semio_framework_value::DslValue>) -> Pose3d {
    let q = unit_quat_from_cad(orientation);
    let t = Vec3d::new(origin[0] as f32, origin[1] as f32, origin[2] as f32);
    Pose3d::from_parts(t, q)
}

pub fn compute_brush_placement_pose(
    source_local_position: Vec3,
    source_local_direction: Vec3,
    scale: &Option<semio_framework_value::DslValue>,
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
