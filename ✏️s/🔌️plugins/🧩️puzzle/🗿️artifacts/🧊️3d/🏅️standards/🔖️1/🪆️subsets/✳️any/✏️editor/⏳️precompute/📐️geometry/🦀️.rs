//! 📐️ Puzzle 3d play app — the precompute geometry layer: the `semio_framework_3d::{rigid,
//! collision}` adapter (the ONE interface boundary this module depends on), the plain
//! `[f64; 3]`/`[f64; 4]` vector and quaternion math the placement solver builds on, the brush
//! placement pose solver itself, and the collision-body/AABB/overlap primitives the brush and
//! fill lanes gate placements with. Rehomed from the former `⚙️engine/📐️geometry` (ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): this is interactive brush/fill tool
//! behaviour, so it lives with the app, not the artifact. The `nalgebra`/`parry3d` third-party
//! surface this adapter used to wrap moved into the framework (ticket
//! 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS) — see
//! `semio_framework_3d::rigid` (vectors/points/quaternions/isometries) and
//! `semio_framework_3d::collision` (BVH triangle-mesh intersection + winding-number containment).

use crate::artifacts::puzzle3d::schema::{Quat, Vec3, WorldVolumeProps};
use semio_framework_3d::{collision, rigid};
use std::borrow::Borrow;
use std::mem::MaybeUninit;

pub(crate) const FIXED_OWNER_SLOTS: usize = 32;
pub(crate) const FIXED_OWNER_PAGE_BYTES: usize = 16 * 1024;

#[derive(Debug)]
pub(crate) struct FixedOwnerVec<T, const N: usize = FIXED_OWNER_SLOTS> {
    page: Option<Box<[MaybeUninit<T>; N]>>,
    len: usize,
}

impl<T, const N: usize> FixedOwnerVec<T, N> {
    pub(crate) fn new() -> Self {
        assert!(Self::page_bytes() <= FIXED_OWNER_PAGE_BYTES);
        Self { page: Some(Box::new(std::array::from_fn(|_| MaybeUninit::uninit()))), len: 0 }
    }

    pub(crate) const fn page_bytes() -> usize {
        std::mem::size_of::<[MaybeUninit<T>; N]>()
    }

    pub(crate) fn backing_credit(&self) -> Option<(usize, usize)> {
        self.page.as_ref().map(|_| (1, Self::page_bytes()))
    }

    pub(crate) fn backing_ptr(&self) -> Option<*const MaybeUninit<T>> {
        self.page.as_ref().map(|page| page.as_ptr())
    }

    pub(crate) fn len(&self) -> usize {
        self.len
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub(crate) fn as_slice(&self) -> &[T] {
        let Some(page) = self.page.as_ref() else { return &[] };
        unsafe { std::slice::from_raw_parts(page.as_ptr().cast::<T>(), self.len) }
    }

    pub(crate) fn as_mut_slice(&mut self) -> &mut [T] {
        let Some(page) = self.page.as_mut() else { return &mut [] };
        unsafe { std::slice::from_raw_parts_mut(page.as_mut_ptr().cast::<T>(), self.len) }
    }

    pub(crate) fn try_push(&mut self, value: T) -> Result<(), T> {
        if self.len == N {
            return Err(value);
        }
        self.page.as_mut().expect("live fixed owner page")[self.len].write(value);
        self.len += 1;
        Ok(())
    }

    pub(crate) fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        Some(unsafe { self.page.as_mut()?.get_unchecked_mut(self.len).assume_init_read() })
    }

    pub(crate) fn retire_backing(&mut self) -> bool {
        if self.len != 0 {
            return false;
        }
        self.page.take().is_some()
    }

    pub(crate) fn terminal_owners_empty(&self) -> bool {
        self.len == 0 && self.page.is_none()
    }
}

impl<T, const N: usize> std::ops::Deref for FixedOwnerVec<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T, const N: usize> std::ops::DerefMut for FixedOwnerVec<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<T, const N: usize> Drop for FixedOwnerVec<T, N> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

#[derive(Debug)]
pub(crate) struct FixedOwnerMap<K, V, const N: usize = FIXED_OWNER_SLOTS> {
    page: Option<Box<[Option<(K, V)>; N]>>,
    len: usize,
}

#[derive(Debug)]
pub(crate) enum FixedOwnerMapInsert<K, V> {
    Inserted,
    Occupied { input_key: K, input_value: V },
}

impl<K, V, const N: usize> FixedOwnerMap<K, V, N> {
    pub(crate) fn new() -> Self {
        assert!(Self::page_bytes() <= FIXED_OWNER_PAGE_BYTES);
        Self { page: Some(Box::new(std::array::from_fn(|_| None))), len: 0 }
    }

    pub(crate) const fn page_bytes() -> usize {
        std::mem::size_of::<[Option<(K, V)>; N]>()
    }

    pub(crate) fn backing_credit(&self) -> Option<(usize, usize)> {
        self.page.as_ref().map(|_| (1, Self::page_bytes()))
    }

    pub(crate) fn backing_ptr(&self) -> Option<*const Option<(K, V)>> {
        self.page.as_ref().map(|page| page.as_ptr())
    }

    pub(crate) fn len(&self) -> usize {
        self.len
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &(K, V)> {
        self.page.as_ref().into_iter().flat_map(move |page| page[..self.len].iter()).filter_map(Option::as_ref)
    }

    pub(crate) fn keys(&self) -> impl Iterator<Item = &K> {
        self.iter().map(|(key, _)| key)
    }

    pub(crate) fn values(&self) -> impl Iterator<Item = &V> {
        self.iter().map(|(_, value)| value)
    }

    fn index_of<Q>(&self, key: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.iter().position(|(candidate, _)| <K as Borrow<Q>>::borrow(candidate) == key)
    }

    pub(crate) fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.index_of(key).and_then(|index| self.page.as_ref()?.get(index)?.as_ref().map(|(_, value)| value))
    }

    pub(crate) fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let index = self.index_of(key)?;
        self.page.as_mut()?.get_mut(index)?.as_mut().map(|(_, value)| value)
    }

    pub(crate) fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.index_of(key).is_some()
    }

    pub(crate) fn try_insert(&mut self, key: K, value: V) -> Result<FixedOwnerMapInsert<K, V>, (K, V)>
    where
        K: Ord,
    {
        if self.index_of(&key).is_some() {
            return Ok(FixedOwnerMapInsert::Occupied { input_key: key, input_value: value });
        }
        if self.len == N {
            return Err((key, value));
        }
        let insert_at = self.iter().position(|(candidate, _)| candidate > &key).unwrap_or(self.len);
        let page = self.page.as_mut().expect("live fixed owner page");
        for index in (insert_at..self.len).rev() {
            page[index + 1] = page[index].take();
        }
        page[insert_at] = Some((key, value));
        self.len += 1;
        Ok(FixedOwnerMapInsert::Inserted)
    }

    pub(crate) fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let index = self.index_of(key)?;
        let page = self.page.as_mut()?;
        let entry = page[index].take()?;
        for cursor in index + 1..self.len {
            page[cursor - 1] = page[cursor].take();
        }
        self.len -= 1;
        Some(entry)
    }

    pub(crate) fn pop_first(&mut self) -> Option<(K, V)> {
        if self.len == 0 {
            return None;
        }
        let page = self.page.as_mut()?;
        let entry = page[0].take();
        for cursor in 1..self.len {
            page[cursor - 1] = page[cursor].take();
        }
        self.len -= 1;
        entry
    }

    pub(crate) fn retire_backing(&mut self) -> bool {
        if self.len != 0 {
            return false;
        }
        self.page.take().is_some()
    }

    pub(crate) fn terminal_owners_empty(&self) -> bool {
        self.len == 0 && self.page.is_none()
    }
}

#[derive(Debug)]
pub(crate) struct FixedOwnerSet<K, const N: usize = FIXED_OWNER_SLOTS> {
    values: FixedOwnerMap<K, (), N>,
}

#[derive(Debug)]
pub(crate) enum FixedOwnerSetInsert<K> {
    Inserted,
    Present { input: K },
}

impl<K, const N: usize> FixedOwnerSet<K, N> {
    pub(crate) fn new() -> Self {
        Self { values: FixedOwnerMap::new() }
    }

    pub(crate) fn backing_credit(&self) -> Option<(usize, usize)> {
        self.values.backing_credit()
    }

    pub(crate) fn backing_ptr(&self) -> Option<*const Option<(K, ())>> {
        self.values.backing_ptr()
    }

    pub(crate) fn len(&self) -> usize {
        self.values.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &K> {
        self.values.keys()
    }

    pub(crate) fn contains<Q>(&self, value: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.values.contains_key(value)
    }

    pub(crate) fn try_insert(&mut self, value: K) -> Result<FixedOwnerSetInsert<K>, K>
    where
        K: Ord,
    {
        self.values
            .try_insert(value, ())
            .map(|outcome| match outcome {
                FixedOwnerMapInsert::Inserted => FixedOwnerSetInsert::Inserted,
                FixedOwnerMapInsert::Occupied { input_key, input_value: () } => FixedOwnerSetInsert::Present { input: input_key },
            })
            .map_err(|(value, ())| value)
    }

    pub(crate) fn remove_entry<Q>(&mut self, value: &Q) -> Option<K>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.values.remove_entry(value).map(|(value, ())| value)
    }

    pub(crate) fn pop_first(&mut self) -> Option<K> {
        self.values.pop_first().map(|(value, ())| value)
    }

    pub(crate) fn retire_backing(&mut self) -> bool {
        self.values.retire_backing()
    }

    pub(crate) fn terminal_owners_empty(&self) -> bool {
        self.values.terminal_owners_empty()
    }
}

//#region 🔒️GeometryAdapter
/// 🔒️ Thin wrappers over `semio_framework_3d::{rigid, collision}` — the one interface boundary
/// this artifact depends on.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Vec3d(rigid::Vector3);

impl Vec3d {
    pub(crate) fn new(x: f32, y: f32, z: f32) -> Self {
        Self(rigid::Vector3::new(x, y, z))
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
pub(crate) struct Point3d(rigid::Point3);

impl Point3d {
    pub(crate) fn new(x: f32, y: f32, z: f32) -> Self {
        Self(rigid::Point3::new(x, y, z))
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
        Self(self.0.inf(other.0))
    }
    pub(crate) fn sup(&self, other: &Self) -> Self {
        Self(self.0.sup(other.0))
    }
    pub(crate) fn coords(&self) -> Vec3d {
        Vec3d(self.0.coords())
    }
    pub(crate) fn from_coords(v: Vec3d) -> Self {
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
pub(crate) struct Rotation3d(rigid::UnitQuaternion);

impl Rotation3d {
    pub(crate) fn identity() -> Self {
        Self(rigid::UnitQuaternion::identity())
    }
    /// 🔓️ Builds from CAD's `[i, j, k, w]` quaternion convention.
    pub(crate) fn from_ijkw(i: f32, j: f32, k: f32, w: f32) -> Self {
        Self(rigid::UnitQuaternion::from_quaternion(rigid::Quaternion::new(w, i, j, k)))
    }
    pub(crate) fn to_ijkw(self) -> (f32, f32, f32, f32) {
        let q = self.0.quaternion();
        (q.i, q.j, q.k, q.w)
    }
    pub(crate) fn rotation_between(from: Vec3d, to: Vec3d) -> Option<Self> {
        rigid::UnitQuaternion::rotation_between(from.0, to.0).map(Self)
    }
    pub(crate) fn apply(&self, v: Vec3d) -> Vec3d {
        Vec3d(self.0.apply(v.0))
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Pose3d(rigid::Isometry3);

impl Pose3d {
    pub(crate) fn identity() -> Self {
        Self(rigid::Isometry3::identity())
    }
    pub(crate) fn from_parts(translation: Vec3d, rotation: Rotation3d) -> Self {
        Self(rigid::Isometry3::from_parts(translation.0, rotation.0))
    }
    pub(crate) fn inverse(&self) -> Self {
        Self(self.0.inverse())
    }
    pub(crate) fn transform_point(&self, point: &Point3d) -> Point3d {
        Point3d(self.0.transform_point(point.0))
    }
    pub(crate) fn semio_compose_rs(&self, other: &Self) -> Self {
        Self(self.0.compose(other.0))
    }
}

#[derive(Clone)]
pub(crate) struct CollisionShape {
    shape: std::sync::Arc<collision::TriMesh>,
    retained_items: usize,
    retained_bytes: usize,
    page_bounded: bool,
}

impl CollisionShape {
    pub(crate) fn from_triangle_mesh(vertices: &[Point3d], indices: Vec<[u32; 3]>) -> Self {
        let verts: Vec<rigid::Point3> = vertices.iter().map(|p| p.0).collect();
        let vertex_bytes = verts.capacity().saturating_mul(std::mem::size_of::<rigid::Point3>());
        let index_bytes = indices.capacity().saturating_mul(std::mem::size_of::<[u32; 3]>());
        let retained_items = usize::from(vertex_bytes != 0) + usize::from(index_bytes != 0);
        let retained_bytes = vertex_bytes.saturating_add(index_bytes);
        let page_bounded = vertex_bytes <= 16 * 1024 && index_bytes <= 16 * 1024;
        let mesh = collision::TriMesh::new(verts, indices);
        Self { shape: std::sync::Arc::new(mesh), retained_items, retained_bytes, page_bounded }
    }
    pub(crate) fn contains_point(&self, pose: &Pose3d, point: &Point3d) -> bool {
        collision::contains_point(pose.0, &self.shape, point.0)
    }
}

fn shapes_intersect(pose_a: &Pose3d, a: &CollisionShape, pose_b: &Pose3d, b: &CollisionShape) -> bool {
    collision::intersection_test(pose_a.0, &a.shape, pose_b.0, &b.shape)
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
            let s = n.as_f64();
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

impl CollisionBody {
    pub(crate) fn retained_parts_backing_credit(&self) -> Option<(usize, usize)> {
        let bytes = self.parts.capacity().checked_mul(std::mem::size_of::<CollisionMeshPart>())?;
        (self.parts.capacity() <= 32 && bytes <= 16 * 1024).then_some((usize::from(bytes != 0), bytes))
    }

    pub(crate) fn retained_part_credit(&self, index: usize) -> Option<(usize, usize)> {
        let part = self.parts.get(index)?;
        part.shape.page_bounded.then_some((part.shape.retained_items, part.shape.retained_bytes))
    }
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
            let s = n.as_f64() as f32;
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
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug)]
pub(crate) struct CollisionSpatialIndex {
    cell_size: f32,
    entries: FixedOwnerMap<String, CollisionAabb>,
    cells: FixedOwnerMap<(i32, i32, i32), FixedOwnerSet<String>>,
    oversized: FixedOwnerSet<String>,
    retiring_key: Option<String>,
    retiring_bucket: Option<FixedOwnerSet<String>>,
}

#[derive(Clone, Debug)]
pub(crate) enum CollisionIndexRejectedOwner {
    Capacity(String, CollisionAabb),
}

impl CollisionIndexRejectedOwner {
    pub(crate) fn retire_one(&mut self) -> bool {
        match self {
            Self::Capacity(id, _) if id.capacity() != 0 => {
                drop(std::mem::take(id));
                false
            }
            Self::Capacity(_, _) => true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct CollisionIndexOwner {
    pub(crate) operation: u64,
    pub(crate) generation: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CollisionCellSpan {
    min: [i32; 3],
    max: [i32; 3],
    count: u64,
}

impl CollisionCellSpan {
    fn new(cell_size: f32, bounds: CollisionAabb) -> Option<Self> {
        let cell = |value: f32| (value / cell_size).floor().clamp(i32::MIN as f32, i32::MAX as f32) as i32;
        let min = [cell(bounds.min[0]), cell(bounds.min[1]), cell(bounds.min[2])];
        let max = [cell(bounds.max[0]), cell(bounds.max[1]), cell(bounds.max[2])];
        let spans = [0, 1, 2].map(|axis| u64::try_from(i64::from(max[axis]) - i64::from(min[axis]) + 1).ok());
        let count = spans.into_iter().try_fold(1_u64, |count, span| count.checked_mul(span?))?;
        (count <= CollisionSpatialIndex::MAX_CELLS_PER_ENTRY).then_some(Self { min, max, count })
    }

    fn cell(self, cursor: u64) -> Option<(i32, i32, i32)> {
        if cursor >= self.count {
            return None;
        }
        let z_span = u64::try_from(i64::from(self.max[2]) - i64::from(self.min[2]) + 1).ok()?;
        let y_span = u64::try_from(i64::from(self.max[1]) - i64::from(self.min[1]) + 1).ok()?;
        let yz = y_span.checked_mul(z_span)?;
        let x = cursor / yz;
        let rest = cursor % yz;
        let y = rest / z_span;
        let z = rest % z_span;
        Some((i32::try_from(i64::from(self.min[0]) + i64::try_from(x).ok()?).ok()?, i32::try_from(i64::from(self.min[1]) + i64::try_from(y).ok()?).ok()?, i32::try_from(i64::from(self.min[2]) + i64::try_from(z).ok()?).ok()?))
    }

    fn contains(self, cell: (i32, i32, i32)) -> bool {
        cell.0 >= self.min[0] && cell.0 <= self.max[0] && cell.1 >= self.min[1] && cell.1 <= self.max[1] && cell.2 >= self.min[2] && cell.2 <= self.max[2]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CollisionMutationStage {
    PreflightNew,
    PreflightOld,
    Remove,
    Insert,
    Commit,
    Complete,
    Rejected,
}

pub(crate) struct CollisionIndexMutation {
    owner: CollisionIndexOwner,
    id: String,
    bounds: CollisionAabb,
    old_bounds: Option<CollisionAabb>,
    old_span: Option<CollisionCellSpan>,
    new_span: Option<CollisionCellSpan>,
    stage: CollisionMutationStage,
    cursor: u64,
    missing_cells: usize,
    reclaimed_cells: usize,
}

#[derive(Debug)]
pub(crate) enum CollisionMutationStep {
    Pending,
    Complete,
    Rejected(CollisionIndexRejectedOwner),
    Stale,
}

pub(crate) struct CollisionIndexRemoval {
    owner: CollisionIndexOwner,
    id: String,
    bounds: CollisionAabb,
    span: Option<CollisionCellSpan>,
    cursor: u64,
    complete: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CollisionQueryStage {
    Cells,
    Oversized,
    Entries,
    Complete,
}

pub(crate) struct CollisionQueryCursor {
    owner: CollisionIndexOwner,
    bounds: CollisionAabb,
    span: Option<CollisionCellSpan>,
    stage: CollisionQueryStage,
    cell_cursor: u64,
    member_cursor: usize,
    candidates: FixedOwnerSet<String>,
    truncated: bool,
    examined_cells: usize,
    examined_members: usize,
    retiring_key: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CollisionQueryStep {
    Pending,
    Complete,
    Stale,
}

impl CollisionQueryCursor {
    pub(crate) fn candidate(&self, index: usize) -> Option<&String> {
        self.candidates.iter().nth(index)
    }

    pub(crate) fn len(&self) -> usize {
        self.candidates.len()
    }

    pub(crate) fn truncated(&self) -> bool {
        self.truncated
    }

    pub(crate) fn retire_one_owner(&mut self) -> bool {
        if let Some(key) = self.retiring_key.as_mut() {
            if key.capacity() != 0 {
                drop(std::mem::take(key));
                return false;
            }
            self.retiring_key.take();
            return false;
        }
        if let Some(key) = self.candidates.pop_first() {
            self.retiring_key = Some(key);
            return false;
        }
        if self.candidates.retire_backing() {
            return false;
        }
        true
    }

    pub(crate) fn terminal_owners_empty(&self) -> bool {
        self.candidates.terminal_owners_empty() && self.retiring_key.is_none()
    }
}

impl CollisionIndexMutation {
    pub(crate) fn retire_one_owner(&mut self) -> bool {
        if self.id.capacity() != 0 {
            drop(std::mem::take(&mut self.id));
            return false;
        }
        true
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CollisionIndexOwnerCensusStep {
    Pending { items: usize, bytes: usize },
    Complete,
    Rejected,
}

#[derive(Default)]
pub(crate) struct CollisionIndexOwnerCensusCursor {
    section: u8,
    index: usize,
    inner: usize,
}

fn collision_index_string_credit(value: &String) -> Option<(usize, usize)> {
    (value.capacity() <= 16 * 1024).then_some((usize::from(value.capacity() != 0), value.capacity()))
}

impl CollisionSpatialIndex {
    const MAX_CELLS_PER_ENTRY: u64 = 4_096;

    pub(crate) fn new(cell_size: f32) -> Self {
        assert!(cell_size.is_finite() && cell_size > 0.0);
        Self { cell_size, entries: FixedOwnerMap::new(), cells: FixedOwnerMap::new(), oversized: FixedOwnerSet::new(), retiring_key: None, retiring_bucket: None }
    }

    pub(crate) fn begin_replacement(&self, owner: CollisionIndexOwner, id: String, bounds: CollisionAabb) -> CollisionIndexMutation {
        let old_bounds = self.entries.get(id.as_str()).copied();
        CollisionIndexMutation {
            owner,
            id,
            bounds,
            old_bounds,
            old_span: old_bounds.and_then(|value| CollisionCellSpan::new(self.cell_size, value)),
            new_span: CollisionCellSpan::new(self.cell_size, bounds),
            stage: CollisionMutationStage::PreflightNew,
            cursor: 0,
            missing_cells: 0,
            reclaimed_cells: 0,
        }
    }

    pub(crate) fn step_replacement(&mut self, mutation: &mut CollisionIndexMutation, current: CollisionIndexOwner) -> CollisionMutationStep {
        if mutation.owner != current {
            return CollisionMutationStep::Stale;
        }
        match mutation.stage {
            CollisionMutationStage::PreflightNew => {
                if self.entries.get(mutation.id.as_str()).is_none() && self.entries.len() == FIXED_OWNER_SLOTS {
                    return Self::reject_mutation(mutation);
                }
                if let Some(span) = mutation.new_span {
                    if let Some(cell) = span.cell(mutation.cursor) {
                        mutation.cursor += 1;
                        match self.cells.get(&cell) {
                            Some(bucket) if !bucket.contains(mutation.id.as_str()) && bucket.len() == FIXED_OWNER_SLOTS => return Self::reject_mutation(mutation),
                            Some(_) => {}
                            None => mutation.missing_cells += 1,
                        }
                        return CollisionMutationStep::Pending;
                    }
                } else if !self.oversized.contains(mutation.id.as_str()) && self.oversized.len() == FIXED_OWNER_SLOTS {
                    return Self::reject_mutation(mutation);
                }
                mutation.stage = CollisionMutationStage::PreflightOld;
                mutation.cursor = 0;
                CollisionMutationStep::Pending
            }
            CollisionMutationStage::PreflightOld => {
                if let Some(span) = mutation.old_span {
                    if let Some(cell) = span.cell(mutation.cursor) {
                        mutation.cursor += 1;
                        if mutation.new_span.is_none_or(|next| !next.contains(cell)) && self.cells.get(&cell).is_some_and(|bucket| bucket.len() == 1 && bucket.contains(mutation.id.as_str())) {
                            mutation.reclaimed_cells += 1;
                        }
                        return CollisionMutationStep::Pending;
                    }
                }
                if self.cells.len().checked_add(mutation.missing_cells).and_then(|total| total.checked_sub(mutation.reclaimed_cells)).is_none_or(|total| total > FIXED_OWNER_SLOTS) {
                    return Self::reject_mutation(mutation);
                }
                mutation.stage = CollisionMutationStage::Remove;
                mutation.cursor = 0;
                CollisionMutationStep::Pending
            }
            CollisionMutationStage::Remove => {
                if let Some(span) = mutation.old_span {
                    if let Some(cell) = span.cell(mutation.cursor) {
                        mutation.cursor += 1;
                        let empty = self.cells.get_mut(&cell).is_some_and(|bucket| {
                            drop(bucket.remove_entry(mutation.id.as_str()));
                            bucket.is_empty()
                        });
                        if empty {
                            drop(self.cells.remove_entry(&cell));
                        }
                        return CollisionMutationStep::Pending;
                    }
                } else if mutation.old_bounds.is_some() {
                    drop(self.oversized.remove_entry(mutation.id.as_str()));
                }
                mutation.stage = CollisionMutationStage::Insert;
                mutation.cursor = 0;
                CollisionMutationStep::Pending
            }
            CollisionMutationStage::Insert => {
                if let Some(span) = mutation.new_span {
                    if let Some(cell) = span.cell(mutation.cursor) {
                        mutation.cursor += 1;
                        if self.cells.get(&cell).is_none() {
                            let bucket = FixedOwnerSet::new();
                            match self.cells.try_insert(cell, bucket) {
                                Ok(FixedOwnerMapInsert::Inserted) => {}
                                Ok(FixedOwnerMapInsert::Occupied { .. }) | Err(_) => unreachable!("preflighted fixed collision cell"),
                            }
                        }
                        let bucket = self.cells.get_mut(&cell).expect("preflighted fixed collision bucket");
                        match bucket.try_insert(mutation.id.clone()) {
                            Ok(FixedOwnerSetInsert::Inserted) => {}
                            Ok(FixedOwnerSetInsert::Present { input }) => drop(input),
                            Err(input) => {
                                drop(input);
                                unreachable!("preflighted fixed collision member");
                            }
                        }
                        return CollisionMutationStep::Pending;
                    }
                } else {
                    match self.oversized.try_insert(mutation.id.clone()) {
                        Ok(FixedOwnerSetInsert::Inserted) => {}
                        Ok(FixedOwnerSetInsert::Present { input }) => drop(input),
                        Err(input) => {
                            drop(input);
                            unreachable!("preflighted oversized collision member");
                        }
                    }
                }
                mutation.stage = CollisionMutationStage::Commit;
                CollisionMutationStep::Pending
            }
            CollisionMutationStage::Commit => {
                drop(self.entries.remove_entry(mutation.id.as_str()));
                let id = std::mem::take(&mut mutation.id);
                match self.entries.try_insert(id, mutation.bounds) {
                    Ok(FixedOwnerMapInsert::Inserted) => {}
                    Ok(FixedOwnerMapInsert::Occupied { .. }) | Err(_) => unreachable!("preflighted collision entry"),
                }
                mutation.stage = CollisionMutationStage::Complete;
                CollisionMutationStep::Complete
            }
            CollisionMutationStage::Complete => CollisionMutationStep::Complete,
            CollisionMutationStage::Rejected => CollisionMutationStep::Rejected(CollisionIndexRejectedOwner::Capacity(String::new(), mutation.bounds)),
        }
    }

    fn reject_mutation(mutation: &mut CollisionIndexMutation) -> CollisionMutationStep {
        mutation.stage = CollisionMutationStage::Rejected;
        CollisionMutationStep::Rejected(CollisionIndexRejectedOwner::Capacity(std::mem::take(&mut mutation.id), mutation.bounds))
    }

    pub(crate) fn begin_removal(&self, owner: CollisionIndexOwner, id: String) -> Option<CollisionIndexRemoval> {
        let bounds = *self.entries.get(id.as_str())?;
        Some(CollisionIndexRemoval { owner, id, bounds, span: CollisionCellSpan::new(self.cell_size, bounds), cursor: 0, complete: false })
    }

    pub(crate) fn step_removal(&mut self, removal: &mut CollisionIndexRemoval, current: CollisionIndexOwner) -> CollisionMutationStep {
        if removal.owner != current {
            return CollisionMutationStep::Stale;
        }
        if removal.complete {
            return CollisionMutationStep::Complete;
        }
        if let Some(span) = removal.span {
            if let Some(cell) = span.cell(removal.cursor) {
                removal.cursor += 1;
                let empty = self.cells.get_mut(&cell).is_some_and(|bucket| {
                    drop(bucket.remove_entry(removal.id.as_str()));
                    bucket.is_empty()
                });
                if empty {
                    drop(self.cells.remove_entry(&cell));
                }
                return CollisionMutationStep::Pending;
            }
        } else {
            drop(self.oversized.remove_entry(removal.id.as_str()));
        }
        drop(self.entries.remove_entry(removal.id.as_str()));
        removal.complete = true;
        CollisionMutationStep::Complete
    }

    pub(crate) fn begin_query(&self, owner: CollisionIndexOwner, bounds: CollisionAabb) -> CollisionQueryCursor {
        let span = CollisionCellSpan::new(self.cell_size, bounds);
        CollisionQueryCursor {
            owner,
            bounds,
            span,
            stage: if span.is_some() { CollisionQueryStage::Cells } else { CollisionQueryStage::Entries },
            cell_cursor: 0,
            member_cursor: 0,
            candidates: FixedOwnerSet::new(),
            truncated: false,
            examined_cells: 0,
            examined_members: 0,
            retiring_key: None,
        }
    }

    pub(crate) fn step_query(&self, query: &mut CollisionQueryCursor, current: CollisionIndexOwner) -> CollisionQueryStep {
        if query.owner != current {
            return CollisionQueryStep::Stale;
        }
        let candidate = match query.stage {
            CollisionQueryStage::Cells => {
                let span = query.span.expect("cell query span");
                let Some(cell) = span.cell(query.cell_cursor) else {
                    query.stage = CollisionQueryStage::Oversized;
                    query.member_cursor = 0;
                    return CollisionQueryStep::Pending;
                };
                let Some(bucket) = self.cells.get(&cell) else {
                    query.cell_cursor += 1;
                    query.examined_cells += 1;
                    return CollisionQueryStep::Pending;
                };
                match bucket.iter().nth(query.member_cursor) {
                    Some(id) => {
                        query.member_cursor += 1;
                        Some(id)
                    }
                    None => {
                        query.cell_cursor += 1;
                        query.member_cursor = 0;
                        query.examined_cells += 1;
                        return CollisionQueryStep::Pending;
                    }
                }
            }
            CollisionQueryStage::Oversized => match self.oversized.iter().nth(query.member_cursor) {
                Some(id) => {
                    query.member_cursor += 1;
                    Some(id)
                }
                None => {
                    query.stage = CollisionQueryStage::Complete;
                    return CollisionQueryStep::Complete;
                }
            },
            CollisionQueryStage::Entries => match self.entries.keys().nth(query.member_cursor) {
                Some(id) => {
                    query.member_cursor += 1;
                    Some(id)
                }
                None => {
                    query.stage = CollisionQueryStage::Complete;
                    return CollisionQueryStep::Complete;
                }
            },
            CollisionQueryStage::Complete => return CollisionQueryStep::Complete,
        };
        if let Some(id) = candidate {
            query.examined_members += 1;
            if self.entries.get(id.as_str()).is_some_and(|entry| entry.intersects(&query.bounds)) && !query.candidates.contains(id.as_str()) {
                match query.candidates.try_insert(id.clone()) {
                    Ok(FixedOwnerSetInsert::Inserted) => {}
                    Ok(FixedOwnerSetInsert::Present { input }) => drop(input),
                    Err(input) => {
                        drop(input);
                        query.truncated = true;
                    }
                }
            }
        }
        CollisionQueryStep::Pending
    }

    pub(crate) fn retire_one_owner(&mut self) -> bool {
        if let Some(key) = self.retiring_key.as_mut() {
            if key.capacity() != 0 {
                drop(std::mem::take(key));
                return false;
            }
            self.retiring_key.take();
            return false;
        }
        if let Some(bucket) = self.retiring_bucket.as_mut() {
            if let Some(id) = bucket.pop_first() {
                self.retiring_key = Some(id);
                return false;
            }
            if bucket.retire_backing() {
                return false;
            }
            self.retiring_bucket.take();
            return false;
        }
        if let Some((key, _)) = self.entries.pop_first() {
            self.retiring_key = Some(key);
            return false;
        }
        if let Some((_, bucket)) = self.cells.pop_first() {
            self.retiring_bucket = Some(bucket);
            return false;
        }
        if let Some(key) = self.oversized.pop_first() {
            self.retiring_key = Some(key);
            return false;
        }
        if self.entries.retire_backing() {
            return false;
        }
        if self.cells.retire_backing() {
            return false;
        }
        if self.oversized.retire_backing() {
            return false;
        }
        true
    }

    pub(crate) fn terminal_owners_empty(&self) -> bool {
        self.entries.terminal_owners_empty() && self.cells.terminal_owners_empty() && self.oversized.terminal_owners_empty() && self.retiring_key.is_none() && self.retiring_bucket.is_none()
    }

    #[cfg(test)]
    pub(crate) fn fixed_backing_witness_for_test(&self) -> [(usize, usize, usize); 3] {
        [
            (self.entries.backing_ptr().map_or(0, |pointer| pointer.cast::<()>() as usize), FixedOwnerMap::<String, CollisionAabb>::page_bytes(), self.entries.len()),
            (self.cells.backing_ptr().map_or(0, |pointer| pointer.cast::<()>() as usize), FixedOwnerMap::<(i32, i32, i32), FixedOwnerSet<String>>::page_bytes(), self.cells.len()),
            (self.oversized.backing_ptr().map_or(0, |pointer| pointer.cast::<()>() as usize), FixedOwnerMap::<String, ()>::page_bytes(), self.oversized.len()),
        ]
    }

    pub(crate) fn census_one_owner(&self, cursor: &mut CollisionIndexOwnerCensusCursor) -> CollisionIndexOwnerCensusStep {
        let credit = match cursor.section {
            0 => self.entries.backing_credit(),
            1 => match self.entries.keys().nth(cursor.index) {
                Some(id) => collision_index_string_credit(id).and_then(|(items, bytes)| items.checked_add(1).map(|items| (items, bytes))),
                None => {
                    cursor.section += 1;
                    cursor.index = 0;
                    return CollisionIndexOwnerCensusStep::Pending { items: 0, bytes: 0 };
                }
            },
            2 => self.cells.backing_credit(),
            3 => match self.cells.values().nth(cursor.index) {
                Some(bucket) if cursor.inner == 0 => {
                    cursor.inner = 1;
                    bucket.backing_credit().and_then(|(items, bytes)| items.checked_add(1).map(|items| (items, bytes)))
                }
                Some(bucket) => match bucket.iter().nth(cursor.inner - 1) {
                    Some(id) => collision_index_string_credit(id),
                    None => {
                        cursor.index += 1;
                        cursor.inner = 0;
                        return CollisionIndexOwnerCensusStep::Pending { items: 0, bytes: 0 };
                    }
                },
                None => {
                    cursor.section += 1;
                    cursor.index = 0;
                    cursor.inner = 0;
                    return CollisionIndexOwnerCensusStep::Pending { items: 0, bytes: 0 };
                }
            },
            4 => self.oversized.backing_credit(),
            5 => match self.oversized.iter().nth(cursor.index) {
                Some(id) => collision_index_string_credit(id).and_then(|(items, bytes)| items.checked_add(1).map(|items| (items, bytes))),
                None => {
                    cursor.section += 1;
                    cursor.index = 0;
                    return CollisionIndexOwnerCensusStep::Pending { items: 0, bytes: 0 };
                }
            },
            6 => self.retiring_key.as_ref().map_or(Some((0, 0)), collision_index_string_credit),
            7 => {
                let Some(bucket) = &self.retiring_bucket else {
                    cursor.section += 1;
                    return CollisionIndexOwnerCensusStep::Pending { items: 0, bytes: 0 };
                };
                if cursor.inner == 0 {
                    cursor.inner = 1;
                    bucket.backing_credit()
                } else {
                    match bucket.iter().nth(cursor.inner - 1) {
                        Some(id) => collision_index_string_credit(id),
                        None => {
                            cursor.section += 1;
                            Some((0, 0))
                        }
                    }
                }
            }
            _ => return CollisionIndexOwnerCensusStep::Complete,
        };
        let Some((items, bytes)) = credit else { return CollisionIndexOwnerCensusStep::Rejected };
        if matches!(cursor.section, 1 | 5) {
            cursor.index += 1;
        } else if matches!(cursor.section, 3 | 7) && cursor.inner != 0 {
            cursor.inner += 1;
        } else {
            cursor.section += 1;
        }
        CollisionIndexOwnerCensusStep::Pending { items, bytes }
    }

    #[cfg(test)]
    fn install_for_test(&mut self, id: &str, bounds: CollisionAabb) -> bool {
        let owner = CollisionIndexOwner { operation: 1, generation: 1 };
        let mut mutation = self.begin_replacement(owner, id.to_string(), bounds);
        for _ in 0..20_000 {
            match self.step_replacement(&mut mutation, owner) {
                CollisionMutationStep::Pending => {}
                CollisionMutationStep::Complete => return true,
                CollisionMutationStep::Rejected(_) | CollisionMutationStep::Stale => return false,
            }
        }
        false
    }

    #[cfg(test)]
    fn remove_for_test(&mut self, id: &str) -> bool {
        let owner = CollisionIndexOwner { operation: 1, generation: 1 };
        let Some(mut removal) = self.begin_removal(owner, id.to_string()) else {
            return false;
        };
        for _ in 0..20_000 {
            match self.step_removal(&mut removal, owner) {
                CollisionMutationStep::Pending => {}
                CollisionMutationStep::Complete => return true,
                CollisionMutationStep::Rejected(_) | CollisionMutationStep::Stale => return false,
            }
        }
        false
    }

    #[cfg(test)]
    fn candidates_for_test(&self, bounds: CollisionAabb) -> Vec<String> {
        let owner = CollisionIndexOwner { operation: 1, generation: 1 };
        let mut query = self.begin_query(owner, bounds);
        for _ in 0..20_000 {
            match self.step_query(&mut query, owner) {
                CollisionQueryStep::Pending => {}
                CollisionQueryStep::Complete => return query.candidates.iter().cloned().collect(),
                CollisionQueryStep::Stale => return Vec::new(),
            }
        }
        Vec::new()
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
    ContainmentA,
    ContainmentB,
    SampleInit,
    Sampling,
    SamplingPointA,
    SamplingPointB,
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
    containment_hit: bool,
    current_sample: Option<[f32; 3]>,
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
            containment_hit: false,
            current_sample: None,
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
            CollisionOverlapStage::ContainmentA => self.step_containment(a, world_a, true),
            CollisionOverlapStage::ContainmentB => self.step_containment(b, world_b, false),
            CollisionOverlapStage::SampleInit => self.init_samples(),
            CollisionOverlapStage::Sampling => self.begin_sample(context),
            CollisionOverlapStage::SamplingPointA => self.step_sample_part(a, world_a, true),
            CollisionOverlapStage::SamplingPointB => self.step_sample_part(b, world_b, false),
            CollisionOverlapStage::Complete => self.complete_result(),
        };
        context.consume_fuel(1);
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
        self.current_sample = Some(self.fallback_center);
        self.part_a_cursor = 0;
        self.part_b_cursor = 0;
        self.containment_hit = false;
        self.stage = CollisionOverlapStage::ContainmentA;
        CollisionStepResult::Pending
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

    fn step_containment(&mut self, body: &CollisionBody, world: &Pose3d, first: bool) -> CollisionStepResult {
        let cursor = if first { &mut self.part_a_cursor } else { &mut self.part_b_cursor };
        let sample = self.current_sample.expect("containment point");
        if let Some(part) = body.parts.get(*cursor) {
            *cursor += 1;
            let local = world.inverse().transform_point(&Point3d::new(sample[0], sample[1], sample[2]));
            let part_local = part.local_pose.inverse().transform_point(&local);
            self.containment_hit |= part.shape.contains_point(&part.local_pose, &part_local);
            return CollisionStepResult::Pending;
        }
        if !self.containment_hit {
            return self.finish(0.0, false);
        }
        self.containment_hit = false;
        if first {
            self.stage = CollisionOverlapStage::ContainmentB;
        } else {
            self.stage = CollisionOverlapStage::SampleInit;
            self.current_sample = None;
        }
        CollisionStepResult::Pending
    }

    fn begin_sample<C: CollisionStepContext>(&mut self, context: &mut C) -> CollisionStepResult {
        if context.is_cancelled() {
            return CollisionStepResult::Cancelled;
        }
        if self.sample_cursor == self.sample_count {
            return self.finish(self.estimated_overlap(), false);
        }
        let sample = [self.next_sample_axis(0), self.next_sample_axis(1), self.next_sample_axis(2)];
        self.last_sample = Some(sample);
        self.current_sample = Some(sample);
        self.sample_cursor += 1;
        self.part_a_cursor = 0;
        self.part_b_cursor = 0;
        self.containment_hit = false;
        self.stage = CollisionOverlapStage::SamplingPointA;
        CollisionStepResult::Pending
    }

    fn step_sample_part(&mut self, body: &CollisionBody, world: &Pose3d, first: bool) -> CollisionStepResult {
        let cursor = if first { &mut self.part_a_cursor } else { &mut self.part_b_cursor };
        let sample = self.current_sample.expect("sampling point");
        if let Some(part) = body.parts.get(*cursor) {
            *cursor += 1;
            let local = world.inverse().transform_point(&Point3d::new(sample[0], sample[1], sample[2]));
            let part_local = part.local_pose.inverse().transform_point(&local);
            self.containment_hit |= part.shape.contains_point(&part.local_pose, &part_local);
            return CollisionStepResult::Pending;
        }
        if first {
            if self.containment_hit {
                self.containment_hit = false;
                self.stage = CollisionOverlapStage::SamplingPointB;
            } else {
                self.stage = CollisionOverlapStage::Sampling;
                self.current_sample = None;
            }
            return CollisionStepResult::Pending;
        }
        if self.containment_hit {
            self.inside_both += 1;
            if self.estimated_overlap() > self.overlap_budget {
                return self.finish(self.overlap_budget + 1.0, true);
            }
        }
        self.containment_hit = false;
        self.current_sample = None;
        self.stage = CollisionOverlapStage::Sampling;
        CollisionStepResult::Pending
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
        let volumes = vec![WorldVolumeProps { id: "v1".to_string(), origin: [0.0, 0.0, 0.0], orientation: None, scale: Some(dsl::DslValue::Array(vec![dsl::DslValue::float(4.0), dsl::DslValue::float(4.0), dsl::DslValue::float(4.0)])) }];
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
        assert_eq!(vec3_scale([1.0, 2.0, 3.0], &Some(dsl::DslValue::float(2.0))), [2.0, 4.0, 6.0]);
        assert_eq!(vec3_scale([1.0, 2.0, 3.0], &Some(dsl::DslValue::Array(vec![dsl::DslValue::float(2.0), dsl::DslValue::float(3.0), dsl::DslValue::float(4.0)]))), [2.0, 6.0, 12.0]);
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
        let volumes = vec![WorldVolumeProps { id: "far".into(), origin: [100.0, 0.0, 0.0], orientation: None, scale: None }, WorldVolumeProps { id: "near".into(), origin: [0.0, 0.0, 0.0], orientation: None, scale: Some(dsl::DslValue::float(4.0)) }];
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
        assert!(index.install_for_test("zeta", near));
        assert!(index.install_for_test("alpha", near));
        assert!(index.install_for_test("far", far));
        assert_eq!(index.candidates_for_test(near), vec!["alpha".to_string(), "zeta".to_string()]);
        assert!(index.install_for_test("zeta", far));
        assert_eq!(index.candidates_for_test(near), vec!["alpha".to_string()]);
        assert!(index.remove_for_test("alpha"));
        assert!(!index.remove_for_test("missing"));
        assert!(index.candidates_for_test(near).is_empty());
    }

    #[test]
    fn spatial_index_bounds_adversarial_cell_spans() {
        let mut index = CollisionSpatialIndex::new(1.0);
        let world = CollisionAabb { min: [-1.0e9; 3], max: [1.0e9; 3] };
        let near = CollisionAabb { min: [-1.0; 3], max: [1.0; 3] };
        assert!(index.install_for_test("world", world));
        assert!(index.install_for_test("near", near));
        assert_eq!(index.candidates_for_test(near), vec!["near".to_string(), "world".to_string()]);
        assert_eq!(index.candidates_for_test(world), vec!["near".to_string(), "world".to_string()]);
        assert!(index.remove_for_test("world"));
    }

    #[test]
    fn spatial_resumable_query_narrows_sparse_cells_without_visiting_distant_population() {
        let mut index = CollisionSpatialIndex::new(1.0);
        let near = CollisionAabb { min: [0.1; 3], max: [0.2; 3] };
        assert!(index.install_for_test("near", near));
        for value in 1..FIXED_OWNER_SLOTS {
            let coordinate = value as f32 * 4.0;
            assert!(index.install_for_test(&format!("far-{value:02}"), CollisionAabb { min: [coordinate; 3], max: [coordinate + 0.1; 3] }));
        }
        let owner = CollisionIndexOwner { operation: 7, generation: 9 };
        let mut query = index.begin_query(owner, near);
        for _ in 0..100 {
            if matches!(index.step_query(&mut query, owner), CollisionQueryStep::Complete) {
                break;
            }
        }
        assert_eq!(query.candidate(0).map(String::as_str), Some("near"));
        assert_eq!(query.len(), 1);
        assert_eq!(query.examined_cells, 1);
        assert_eq!(query.examined_members, 1);
        assert!(!query.truncated());
    }

    #[test]
    fn spatial_capacity_plus_one_refusal_preserves_exact_old_state() {
        let mut index = CollisionSpatialIndex::new(8.0);
        let bounds = CollisionAabb { min: [0.0; 3], max: [0.5; 3] };
        for value in 0..FIXED_OWNER_SLOTS {
            assert!(index.install_for_test(&format!("entry-{value:02}"), bounds));
        }
        let before = index.candidates_for_test(bounds);
        assert!(!index.install_for_test("entry-plus-one", bounds));
        assert_eq!(index.candidates_for_test(bounds), before);
    }

    #[test]
    fn spatial_stale_owner_cannot_finish_partial_replacement() {
        let mut index = CollisionSpatialIndex::new(1.0);
        let old = CollisionAabb { min: [0.0; 3], max: [0.2; 3] };
        let next = CollisionAabb { min: [3.0; 3], max: [3.2; 3] };
        assert!(index.install_for_test("owned", old));
        let owner = CollisionIndexOwner { operation: 11, generation: 13 };
        let mut mutation = index.begin_replacement(owner, "owned".into(), next);
        assert!(matches!(index.step_replacement(&mut mutation, owner), CollisionMutationStep::Pending));
        assert!(matches!(index.step_replacement(&mut mutation, CollisionIndexOwner { operation: 11, generation: 14 }), CollisionMutationStep::Stale));
        assert_eq!(index.candidates_for_test(old), vec!["owned".to_string()]);
        assert!(index.candidates_for_test(next).is_empty());
    }

    #[test]
    fn spatial_multi_cell_oversized_replacement_and_removal_make_bounded_progress() {
        let mut index = CollisionSpatialIndex::new(1.0);
        let multi = CollisionAabb { min: [0.0; 3], max: [1.1, 0.2, 0.2] };
        let oversized = CollisionAabb { min: [-1.0e9; 3], max: [1.0e9; 3] };
        assert!(index.install_for_test("multi", multi));
        assert!(index.install_for_test("oversized", oversized));
        assert_eq!(index.candidates_for_test(multi), vec!["multi".to_string(), "oversized".to_string()]);
        assert!(index.install_for_test("multi", CollisionAabb { min: [4.0; 3], max: [4.2; 3] }));
        assert!(index.remove_for_test("oversized"));
        assert!(index.candidates_for_test(multi).is_empty());
    }

    #[test]
    fn spatial_index_close_retains_bucket_values_and_retires_one_credited_owner_per_grant() {
        fn retained_credit(index: &CollisionSpatialIndex) -> (usize, usize) {
            let mut cursor = CollisionIndexOwnerCensusCursor::default();
            let mut credit = (0usize, 0usize);
            loop {
                match index.census_one_owner(&mut cursor) {
                    CollisionIndexOwnerCensusStep::Pending { items, bytes } => {
                        credit.0 = credit.0.checked_add(items).expect("bounded items");
                        credit.1 = credit.1.checked_add(bytes).expect("bounded bytes");
                    }
                    CollisionIndexOwnerCensusStep::Complete => return credit,
                    CollisionIndexOwnerCensusStep::Rejected => panic!("bounded credit"),
                }
            }
        }

        let mut index = CollisionSpatialIndex::new(2.0);
        assert!(index.install_for_test("alpha-owned", CollisionAabb { min: [-1.0; 3], max: [1.0; 3] }));
        assert!(index.install_for_test("oversized-owned", CollisionAabb { min: [-1.0e9; 3], max: [1.0e9; 3] }));
        let mut grants = 0;
        while !index.terminal_owners_empty() {
            let before = retained_credit(&index);
            assert!(!index.retire_one_owner(), "a populated spatial index cannot bulk-retire in one grant");
            let after = retained_credit(&index);
            assert!(before.0.saturating_sub(after.0) <= 1, "one close grant releases at most one exact allocation/root");
            assert!(before.1.saturating_sub(after.1) <= 16 * 1024, "one close grant releases at most one admitted page");
            grants += 1;
        }
        assert!(grants > 4, "entry, bucket vector, nested ids, and oversized key retire independently");
        assert!(index.retire_one_owner());
    }

    #[test]
    fn spatial_fixed_collections_use_the_credited_pages_and_return_identical_plus_one_owners() {
        let mut entries = FixedOwnerMap::<String, CollisionAabb>::new();
        let entry_page = entries.backing_ptr().expect("entry page");
        for index in 0..FIXED_OWNER_SLOTS {
            assert!(matches!(entries.try_insert(format!("entry-{index:02}"), CollisionAabb { min: [index as f32; 3], max: [index as f32 + 1.0; 3] }), Ok(FixedOwnerMapInsert::Inserted)));
        }
        let rejected_entry = String::from("entry-plus-one");
        let rejected_entry_ptr = rejected_entry.as_ptr();
        let Err((rejected_entry, _)) = entries.try_insert(rejected_entry, CollisionAabb { min: [0.0; 3], max: [1.0; 3] }) else { panic!("entry cap + 1") };
        assert_eq!(rejected_entry.as_ptr(), rejected_entry_ptr);
        assert_eq!(entries.backing_ptr(), Some(entry_page));

        let mut cells = FixedOwnerMap::<(i32, i32, i32), FixedOwnerSet<String>>::new();
        let cell_page = cells.backing_ptr().expect("cell page");
        for index in 0..FIXED_OWNER_SLOTS {
            assert!(matches!(cells.try_insert((index as i32, 0, 0), FixedOwnerSet::new()), Ok(FixedOwnerMapInsert::Inserted)));
        }
        let rejected_bucket = FixedOwnerSet::new();
        let rejected_page = rejected_bucket.backing_ptr();
        let Err((_, rejected_bucket)) = cells.try_insert((FIXED_OWNER_SLOTS as i32, 0, 0), rejected_bucket) else { panic!("cell cap + 1") };
        assert_eq!(rejected_bucket.backing_ptr(), rejected_page);
        assert_eq!(cells.backing_ptr(), Some(cell_page));

        let mut oversized = FixedOwnerSet::<String>::new();
        let oversized_page = oversized.backing_ptr().expect("oversized page");
        for index in 0..FIXED_OWNER_SLOTS {
            assert!(matches!(oversized.try_insert(format!("oversized-{index:02}")), Ok(FixedOwnerSetInsert::Inserted)));
        }
        let rejected_oversized = String::from("oversized-plus-one");
        let rejected_oversized_ptr = rejected_oversized.as_ptr();
        let Err(rejected_oversized) = oversized.try_insert(rejected_oversized) else { panic!("oversized cap + 1") };
        assert_eq!(rejected_oversized.as_ptr(), rejected_oversized_ptr);
        assert_eq!(oversized.backing_ptr(), Some(oversized_page));

        for _ in 0..FIXED_OWNER_SLOTS {
            drop(entries.pop_first().expect("one entry"));
            drop(cells.pop_first().expect("one cell"));
            drop(oversized.pop_first().expect("one oversized id"));
        }
        assert!(entries.retire_backing());
        assert!(!cells.terminal_owners_empty(), "only one actual collection page retires per close grant");
        assert!(cells.retire_backing());
        assert!(oversized.retire_backing());
        assert!(entries.terminal_owners_empty() && cells.terminal_owners_empty() && oversized.terminal_owners_empty());
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
