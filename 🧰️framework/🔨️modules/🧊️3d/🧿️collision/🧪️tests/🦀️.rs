//! 🧪️ `parry3d`-differential coverage for `🧿️collision`. `parry3d` lives ONLY in
//! `[dev-dependencies]` on this crate — the one place in the framework allowed to keep it, purely
//! as the oracle proving our BVH triangle-mesh queries agree with it.

use super::*;
use crate::rigid::{Quaternion, UnitQuaternion, Vector3};
use parry3d::query::PointQuery;

//#region 🔖️Lcg
/// 🎲️ A constant-seeded LCG (Numerical Recipes constants) — deterministic, no `rand` crate.
struct Lcg(u64);

impl Lcg {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next_u32(&mut self) -> u32 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.0 >> 32) as u32
    }

    /// 🎯️ Uniform in `[-1, 1]`.
    fn next_unit(&mut self) -> f32 {
        (self.next_u32() as f32 / u32::MAX as f32) * 2.0 - 1.0
    }

    fn next_vector3(&mut self, scale: f32) -> Vector3 {
        Vector3::new(self.next_unit() * scale, self.next_unit() * scale, self.next_unit() * scale)
    }
}
//#endregion 🔖️Lcg

//#region 🔖️Meshes
/// 📦️ Outward-wound axis-aligned cube (centered at the origin) with 12 triangles, wound so each
/// face's cross product points away from the center — verified once in `cube_faces_wind_outward`.
fn cube_mesh(half_extent: f32) -> (Vec<Point3>, Vec<[u32; 3]>) {
    let s = half_extent;
    let vertices = vec![
        Point3::new(-s, -s, -s),
        Point3::new(s, -s, -s),
        Point3::new(s, s, -s),
        Point3::new(-s, s, -s),
        Point3::new(-s, -s, s),
        Point3::new(s, -s, s),
        Point3::new(s, s, s),
        Point3::new(-s, s, s),
    ];
    let triangles = vec![[0, 3, 2], [0, 2, 1], [4, 5, 6], [4, 6, 7], [0, 1, 5], [0, 5, 4], [3, 7, 6], [3, 6, 2], [0, 4, 7], [0, 7, 3], [1, 2, 6], [1, 6, 5]];
    (vertices, triangles)
}

/// ▬️ Two coplanar triangles (a quad on `z = z`) spanning `[cx-hw,cx+hw] x [cy-hw,cy+hw]`.
fn quad_mesh(z: f32, cx: f32, cy: f32, half_width: f32) -> (Vec<Point3>, Vec<[u32; 3]>) {
    let (s, hw) = (z, half_width);
    let vertices = vec![Point3::new(cx - hw, cy - hw, s), Point3::new(cx + hw, cy - hw, s), Point3::new(cx + hw, cy + hw, s), Point3::new(cx - hw, cy + hw, s)];
    (vertices, vec![[0, 1, 2], [0, 2, 3]])
}
//#endregion 🔖️Meshes

//#region 🔖️OracleBridge
fn to_na_point(p: Point3) -> parry3d::na::Point3<f32> {
    parry3d::na::Point3::new(p.x, p.y, p.z)
}

fn to_na_isometry(pose: Isometry3) -> parry3d::na::Isometry3<f32> {
    let translation = parry3d::na::Translation3::new(pose.translation.x, pose.translation.y, pose.translation.z);
    let q = pose.rotation.quaternion();
    let rotation = parry3d::na::UnitQuaternion::from_quaternion(parry3d::na::Quaternion::new(q.w, q.i, q.j, q.k));
    parry3d::na::Isometry3::from_parts(translation, rotation)
}

fn oracle_trimesh(vertices: &[Point3], triangles: &[[u32; 3]]) -> parry3d::shape::TriMesh {
    let verts: Vec<_> = vertices.iter().copied().map(to_na_point).collect();
    parry3d::shape::TriMesh::with_flags(verts, triangles.to_vec(), parry3d::shape::TriMeshFlags::ORIENTED | parry3d::shape::TriMeshFlags::MERGE_DUPLICATE_VERTICES)
}

fn oracle_intersects(pose_a: Isometry3, a: (&[Point3], &[[u32; 3]]), pose_b: Isometry3, b: (&[Point3], &[[u32; 3]])) -> bool {
    let (mesh_a, mesh_b) = (oracle_trimesh(a.0, a.1), oracle_trimesh(b.0, b.1));
    parry3d::query::intersection_test(&to_na_isometry(pose_a), &mesh_a, &to_na_isometry(pose_b), &mesh_b).unwrap_or(false)
}

fn oracle_contains(pose: Isometry3, mesh: (&[Point3], &[[u32; 3]]), point: Point3) -> bool {
    let shape = oracle_trimesh(mesh.0, mesh.1);
    shape.contains_point(&to_na_isometry(pose), &to_na_point(point))
}

fn ours_intersects(pose_a: Isometry3, a: (&[Point3], &[[u32; 3]]), pose_b: Isometry3, b: (&[Point3], &[[u32; 3]])) -> bool {
    let mesh_a = TriMesh::new(a.0.to_vec(), a.1.to_vec());
    let mesh_b = TriMesh::new(b.0.to_vec(), b.1.to_vec());
    intersection_test(pose_a, &mesh_a, pose_b, &mesh_b)
}

fn ours_contains(pose: Isometry3, mesh: (&[Point3], &[[u32; 3]]), point: Point3) -> bool {
    let tri_mesh = TriMesh::new(mesh.0.to_vec(), mesh.1.to_vec());
    contains_point(pose, &tri_mesh, point)
}

fn pose_at(translation: Vector3) -> Isometry3 {
    Isometry3::from_parts(translation, UnitQuaternion::identity())
}

fn random_rotation(lcg: &mut Lcg) -> UnitQuaternion {
    loop {
        let from = lcg.next_vector3(1.0);
        let to = lcg.next_vector3(1.0);
        if let Some(rotation) = UnitQuaternion::rotation_between(from, to) {
            return rotation;
        }
    }
}

fn random_na_rotation(rotation: UnitQuaternion) -> parry3d::na::UnitQuaternion<f32> {
    let q = rotation.quaternion();
    parry3d::na::UnitQuaternion::from_quaternion(parry3d::na::Quaternion::new(q.w, q.i, q.j, q.k))
}
//#endregion 🔖️OracleBridge

//#region 🔖️Tests
#[test]
fn cube_faces_wind_outward() {
    let (vertices, triangles) = cube_mesh(1.0);
    for triangle in &triangles {
        let [a, b, c] = triangle.map(|index| vertices[index as usize]);
        let centroid = Point3::new((a.x + b.x + c.x) / 3.0, (a.y + b.y + c.y) / 3.0, (a.z + b.z + c.z) / 3.0);
        let normal = (b - a).cross(c - a);
        assert!(normal.dot(centroid.coords()) > 0.0, "triangle {triangle:?} should wind outward");
    }
}

#[test]
fn disjoint_cubes_never_intersect() {
    let cube = cube_mesh(1.0);
    let (a, b) = ((cube.0.as_slice(), cube.1.as_slice()), (cube.0.as_slice(), cube.1.as_slice()));
    let far = pose_at(Vector3::new(10.0, 0.0, 0.0));
    let identity = Isometry3::identity();
    assert_eq!(ours_intersects(identity, a, far, b), oracle_intersects(identity, a, far, b));
    assert!(!ours_intersects(identity, a, far, b));
}

#[test]
fn touching_face_to_face_cubes_intersect() {
    let cube = cube_mesh(1.0);
    let mesh = (cube.0.as_slice(), cube.1.as_slice());
    let identity = Isometry3::identity();
    let touching = pose_at(Vector3::new(2.0, 0.0, 0.0));
    assert_eq!(ours_intersects(identity, mesh, touching, mesh), oracle_intersects(identity, mesh, touching, mesh));
}

#[test]
fn overlapping_cubes_intersect() {
    let cube = cube_mesh(1.0);
    let mesh = (cube.0.as_slice(), cube.1.as_slice());
    let identity = Isometry3::identity();
    let overlapping = pose_at(Vector3::new(1.0, 0.0, 0.0));
    assert_eq!(ours_intersects(identity, mesh, overlapping, mesh), oracle_intersects(identity, mesh, overlapping, mesh));
    assert!(ours_intersects(identity, mesh, overlapping, mesh));
}

#[test]
fn nested_cube_reports_no_boundary_crossing_like_the_oracle() {
    let outer = cube_mesh(4.0);
    let inner = cube_mesh(1.0);
    let outer_mesh = (outer.0.as_slice(), outer.1.as_slice());
    let inner_mesh = (inner.0.as_slice(), inner.1.as_slice());
    let identity = Isometry3::identity();
    assert_eq!(ours_intersects(identity, outer_mesh, identity, inner_mesh), oracle_intersects(identity, outer_mesh, identity, inner_mesh));
    assert!(!ours_intersects(identity, outer_mesh, identity, inner_mesh), "a pure surface-mesh test cannot see full containment without any boundary crossing");
}

#[test]
fn shared_edge_meshes_intersect() {
    let hinge_a = (vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0), Point3::new(1.0, 0.0, 1.0)], vec![[0u32, 1, 2]]);
    let hinge_b = (vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0), Point3::new(-1.0, 0.0, 1.0)], vec![[0u32, 1, 2]]);
    let identity = Isometry3::identity();
    let a = (hinge_a.0.as_slice(), hinge_a.1.as_slice());
    let b = (hinge_b.0.as_slice(), hinge_b.1.as_slice());
    assert_eq!(ours_intersects(identity, a, identity, b), oracle_intersects(identity, a, identity, b));
    assert!(ours_intersects(identity, a, identity, b), "triangles sharing a full edge must intersect");
}

#[test]
fn coplanar_overlapping_quads_intersect() {
    let a = quad_mesh(0.0, 0.0, 0.0, 1.0);
    let b = quad_mesh(0.0, 1.0, 0.0, 1.0);
    let identity = Isometry3::identity();
    let a_ref = (a.0.as_slice(), a.1.as_slice());
    let b_ref = (b.0.as_slice(), b.1.as_slice());
    assert_eq!(ours_intersects(identity, a_ref, identity, b_ref), oracle_intersects(identity, a_ref, identity, b_ref));
    assert!(ours_intersects(identity, a_ref, identity, b_ref));
}

#[test]
fn coplanar_disjoint_quads_do_not_intersect() {
    let a = quad_mesh(0.0, 0.0, 0.0, 1.0);
    let b = quad_mesh(0.0, 10.0, 0.0, 1.0);
    let identity = Isometry3::identity();
    let a_ref = (a.0.as_slice(), a.1.as_slice());
    let b_ref = (b.0.as_slice(), b.1.as_slice());
    assert_eq!(ours_intersects(identity, a_ref, identity, b_ref), oracle_intersects(identity, a_ref, identity, b_ref));
    assert!(!ours_intersects(identity, a_ref, identity, b_ref));
}

#[test]
fn contains_point_matches_oracle_for_axis_aligned_samples() {
    let cube = cube_mesh(1.0);
    let mesh = (cube.0.as_slice(), cube.1.as_slice());
    let identity = Isometry3::identity();
    for point in [Point3::new(0.0, 0.0, 0.0), Point3::new(0.9, 0.0, 0.0), Point3::new(1.5, 0.0, 0.0), Point3::new(0.0, -0.99, 0.0), Point3::new(5.0, 5.0, 5.0)] {
        assert_eq!(ours_contains(identity, mesh, point), oracle_contains(identity, mesh, point), "mismatch at {point:?}");
    }
}

#[test]
fn contains_point_matches_oracle_under_translation_and_rotation() {
    let cube = cube_mesh(1.0);
    let mesh = (cube.0.as_slice(), cube.1.as_slice());
    let mut lcg = Lcg::new(0x5EED_C0FF_EE01);
    for _ in 0..64 {
        let translation = lcg.next_vector3(3.0);
        let rotation = random_rotation(&mut lcg);
        let pose = Isometry3::from_parts(translation, rotation);
        let sample = Point3::from_coords(translation + lcg.next_vector3(2.0));
        assert_eq!(ours_contains(pose, mesh, sample), oracle_contains(pose, mesh, sample), "mismatch for translation={translation:?} sample={sample:?}");
    }
}

#[test]
fn intersection_test_matches_oracle_across_a_random_pose_corpus() {
    let big = cube_mesh(1.5);
    let small = cube_mesh(0.6);
    let big_mesh = (big.0.as_slice(), big.1.as_slice());
    let small_mesh = (small.0.as_slice(), small.1.as_slice());
    let mut lcg = Lcg::new(0x1234_5678_9ABC_DEF0);
    let mut intersecting_cases = 0;
    for _ in 0..200 {
        let translation = lcg.next_vector3(4.0);
        let rotation = random_rotation(&mut lcg);
        let pose_b = Isometry3::from_parts(translation, rotation);
        let identity = Isometry3::identity();
        let ours = ours_intersects(identity, big_mesh, pose_b, small_mesh);
        let oracle = oracle_intersects(identity, big_mesh, pose_b, small_mesh);
        assert_eq!(ours, oracle, "mismatch for translation={translation:?}");
        intersecting_cases += usize::from(ours);
    }
    assert!(intersecting_cases > 0, "the random corpus should hit at least one intersecting configuration");
    assert!(intersecting_cases < 200, "the random corpus should hit at least one disjoint configuration");
}

#[test]
fn rotation_bridge_agrees_with_the_nalgebra_oracle() {
    let mut lcg = Lcg::new(0xABCD_EF01_2345_6789);
    for _ in 0..32 {
        let from = lcg.next_vector3(1.0);
        let to = lcg.next_vector3(1.0);
        let (Some(ours), Some(oracle)) = (UnitQuaternion::rotation_between(from, to), parry3d::na::UnitQuaternion::rotation_between(&parry3d::na::Vector3::new(from.x, from.y, from.z), &parry3d::na::Vector3::new(to.x, to.y, to.z))) else { continue };
        let probe = lcg.next_vector3(2.0);
        let ours_rotated = ours.apply(probe);
        let oracle_rotated = oracle * parry3d::na::Vector3::new(probe.x, probe.y, probe.z);
        assert!((ours_rotated.x - oracle_rotated.x).abs() < 1e-4, "x mismatch: {ours_rotated:?} vs {oracle_rotated:?}");
        assert!((ours_rotated.y - oracle_rotated.y).abs() < 1e-4, "y mismatch: {ours_rotated:?} vs {oracle_rotated:?}");
        assert!((ours_rotated.z - oracle_rotated.z).abs() < 1e-4, "z mismatch: {ours_rotated:?} vs {oracle_rotated:?}");
    }
}

#[test]
fn quaternion_construction_matches_oracle_component_by_component() {
    let cases = [(0.0, 0.0, 0.0, 1.0), (1.0, 0.0, 0.0, 1.0), (0.0, 1.0, 0.0, 0.0), (0.3, -0.4, 0.5, 0.6)];
    for (i, j, k, w) in cases {
        let ours = UnitQuaternion::from_quaternion(Quaternion::new(w, i, j, k));
        let oracle = parry3d::na::UnitQuaternion::from_quaternion(parry3d::na::Quaternion::new(w, i, j, k));
        let q = ours.quaternion();
        assert!((q.i - oracle.i).abs() < 1e-6 && (q.j - oracle.j).abs() < 1e-6 && (q.k - oracle.k).abs() < 1e-6 && (q.w - oracle.w).abs() < 1e-6);
    }
}

#[test]
fn isometry_bridge_transforms_points_like_the_oracle() {
    let mut lcg = Lcg::new(0x0FED_CBA9_8765_4321);
    for _ in 0..32 {
        let translation = lcg.next_vector3(5.0);
        let rotation = random_rotation(&mut lcg);
        let pose = Isometry3::from_parts(translation, rotation);
        let oracle_pose = parry3d::na::Isometry3::from_parts(parry3d::na::Translation3::new(translation.x, translation.y, translation.z), random_na_rotation(rotation));
        let point = Point3::from_coords(lcg.next_vector3(3.0));
        let ours = pose.transform_point(point);
        let oracle = oracle_pose * to_na_point(point);
        assert!((ours.x - oracle.x).abs() < 1e-4 && (ours.y - oracle.y).abs() < 1e-4 && (ours.z - oracle.z).abs() < 1e-4);
    }
}
//#endregion 🔖️Tests
