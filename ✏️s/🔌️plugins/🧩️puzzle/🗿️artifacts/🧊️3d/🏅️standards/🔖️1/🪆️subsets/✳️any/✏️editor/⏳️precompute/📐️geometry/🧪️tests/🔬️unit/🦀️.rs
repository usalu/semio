use super::*;

use crate::standards::v1::subsets::any::schema::precompute_model_tests::context::*;
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
fn spatial_entries_admit_document_scale_beyond_one_cell_member_bucket() {
    let mut index = CollisionSpatialIndex::new(8.0);
    let installed = 4 * FIXED_OWNER_SLOTS;
    for value in 0..installed {
        let coordinate = value as f32 * 64.0;
        assert!(index.install_for_test(&format!("document-{value:03}"), CollisionAabb { min: [coordinate; 3], max: [coordinate + 0.5; 3] }), "entry {value} beyond the bookkeeping batch must still be admitted");
    }
    let witness = index.fixed_backing_witness_for_test();
    assert_eq!((witness[0].2, witness[2].2), (installed, 0));
    assert!(witness[1].2 >= installed, "each document entry owns at least one cell");
    for (page, (_, bytes, _)) in witness.into_iter().enumerate() {
        assert!(bytes <= DOCUMENT_OWNER_PAGE_BYTES, "spatial page {page} claims {bytes} bytes beyond the declared document page ceiling");
    }
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
        assert!(before.1.saturating_sub(after.1) <= DOCUMENT_OWNER_PAGE_BYTES, "one close grant releases at most one admitted page");
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

#[test]
fn document_scale_capacities_are_derived_from_the_flagship_fixture_not_the_bookkeeping_batch() {
    assert!(DOCUMENT_OBJECT_SLOTS >= NAKAGIN_DOCUMENT_OBJECTS + DOCUMENT_FILL_HEADROOM_SLOTS, "objects must hold the flagship fixture plus its declared fill headroom");
    assert!(DOCUMENT_ATTRACTION_SLOTS >= NAKAGIN_DOCUMENT_OBJECTS + DOCUMENT_FILL_HEADROOM_SLOTS, "one attraction per placement shares the object headroom");
    assert_eq!((DOCUMENT_VORTEX_SLOTS, DOCUMENT_ATTRACTION_SLOTS, DOCUMENT_CELL_SLOTS), (2 * DOCUMENT_OBJECT_SLOTS, DOCUMENT_OBJECT_SLOTS, 4 * DOCUMENT_OBJECT_SLOTS));
    assert_eq!((DOCUMENT_VOLUME_SLOTS, DOCUMENT_CANDIDATE_SLOTS), (DOCUMENT_KIND_SLOTS, 4 * DOCUMENT_KIND_SLOTS));
    for (slots, page) in [
        (DOCUMENT_OBJECT_SLOTS, FixedOwnerVec::<crate::standards::v1::subsets::any::schema::FixtureObject, DOCUMENT_OBJECT_SLOTS>::page_bytes()),
        (DOCUMENT_ATTRACTION_SLOTS, FixedOwnerVec::<crate::standards::v1::subsets::any::schema::AttractionProps, DOCUMENT_ATTRACTION_SLOTS>::page_bytes()),
        (DOCUMENT_VOLUME_SLOTS, FixedOwnerVec::<WorldVolumeProps, DOCUMENT_VOLUME_SLOTS>::page_bytes()),
        (DOCUMENT_OBJECT_SLOTS, FixedOwnerMap::<String, CollisionAabb, DOCUMENT_OBJECT_SLOTS>::page_bytes()),
        (DOCUMENT_CELL_SLOTS, FixedOwnerMap::<(i32, i32, i32), FixedOwnerSet<String>, DOCUMENT_CELL_SLOTS>::page_bytes()),
        (DOCUMENT_VORTEX_SLOTS, FixedOwnerMap::<String, (), DOCUMENT_VORTEX_SLOTS>::page_bytes()),
    ] {
        assert!(page > 0 && page <= DOCUMENT_OWNER_PAGE_BYTES, "a {slots}-slot document page claims {page} bytes beyond the declared ceiling");
    }
    assert!(
        FixedOwnerMap::<(i32, i32, i32), FixedOwnerSet<String>, DOCUMENT_CELL_SLOTS>::page_bytes() + DOCUMENT_CELL_SLOTS * FixedOwnerMap::<String, ()>::page_bytes() <= 8 * 1024 * 1024,
        "fully occupied cells keep their lazily allocated member buckets bounded"
    );
}

/// ⚖️ LAW: an owner whose page allocation was REFUSED refuses every insert and reports zero
/// capacity — it never traps.
///
/// 🧊️ `FixedOwnerVec::new`/`FixedOwnerMap::new` already answer a failed `try_reserve_exact` with a
/// page-less owner instead of aborting, but the owner then reported the FULL declared capacity and
/// `expect`ed its page on the first push. On build #29 of ticket 26/09/02 that is what turned a
/// guest running out of linear memory into `panicked … live fixed owner page` → wasm `unreachable`
/// → `shard 0 lost`, two seconds after the command-ingress prologue had already started answering
/// `plugin.command-page-allocation`. A native suite cannot exhaust a 512 MiB linear memory, so the
/// law reaches the state through the same constructor the refusal path returns.
#[test]
fn an_owner_whose_page_was_refused_refuses_every_insert_instead_of_trapping() {
    let mut values = FixedOwnerVec::<u32, 8>::refused();
    assert_eq!(values.capacity(), 0, "a refused owner holds nothing, whatever its declared width");
    assert_eq!(values.len(), 0);
    assert_eq!(values.try_push(7), Err(7), "a refused owner refuses its first push instead of trapping");
    assert!(values.as_slice().is_empty() && values.pop().is_none() && values.backing_credit().is_none());
    assert!(values.terminal_owners_empty(), "a refused owner is already terminal — there is no page to give back");
    assert!(!values.retire_backing());

    let mut entries = FixedOwnerMap::<String, u32, 8>::refused();
    assert_eq!(entries.capacity(), 0);
    let refused = entries.try_insert("a".to_string(), 3).expect_err("a refused map refuses its first insert");
    assert_eq!(refused, ("a".to_string(), 3), "the refused entry is handed back whole");
    assert!(entries.get("a").is_none() && !entries.contains_key("a") && entries.is_empty());
    assert!(entries.pop_first().is_none() && entries.remove_entry("a").is_none());
    assert!(entries.terminal_owners_empty());

    let mut members = FixedOwnerSet::<String, 8>::refused();
    assert_eq!(members.capacity(), 0);
    assert_eq!(members.try_insert("a".to_string()).expect_err("a refused set refuses its first insert"), "a".to_string());
    assert!(members.is_empty() && members.terminal_owners_empty());

    let live = FixedOwnerVec::<u32, 8>::new();
    assert_eq!(live.capacity(), 8, "an owner that got its page still reports its declared width");
    assert_eq!(FixedOwnerMap::<String, u32, 8>::new().capacity(), 8);
    assert_eq!(FixedOwnerSet::<String, 8>::new().capacity(), 8);
}


/// ⚖️ LAW: every sub-page a fixed owner asks the guest for stays at or under
/// `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES` — the owners never ask for a block a fragmented guest
/// refuses first.
///
/// 🧊️ The guest runs on one linear memory that grows and never shrinks, served by `dlmalloc` with a
/// 64 KiB granularity, so a single block larger than that is the first request a fragmented or
/// nearly-full guest refuses. `FixedOwnerVec<FixtureObject, DOCUMENT_OBJECT_SLOTS>` used to ask for
/// ≈432 KiB in ONE piece, and `DOCUMENT_OWNER_PAGE_BYTES` admitted up to a whole MiB — the block
/// whose refusal abandoned a Nakagin fill plan the user was 40+ s into (W-F6 §8 item 2). The
/// declared width is unchanged; it is now backed by lazily claimed sub-pages, and this law is what
/// keeps a later widening from re-crossing the ceiling.
#[test]
fn every_fixed_owner_sub_page_request_stays_under_the_guest_contiguous_ceiling() {
    use crate::standards::v1::subsets::any::schema::{AttractionProps, BrushCompatibleCandidate, CableKindCatalog, FixtureObject, KindCompatEntry, ObjectKind, VortexKindCatalog};
    for (owner, bytes) in [
        ("fixture objects", FixedOwnerVec::<FixtureObject, DOCUMENT_OBJECT_SLOTS>::sub_page_bytes()),
        ("fixture attractions", FixedOwnerVec::<AttractionProps, DOCUMENT_ATTRACTION_SLOTS>::sub_page_bytes()),
        ("fixture target volumes", FixedOwnerVec::<WorldVolumeProps, DOCUMENT_VOLUME_SLOTS>::sub_page_bytes()),
        ("catalog objects", FixedOwnerVec::<ObjectKind, DOCUMENT_KIND_SLOTS>::sub_page_bytes()),
        ("catalog vortices", FixedOwnerVec::<VortexKindCatalog, DOCUMENT_KIND_SLOTS>::sub_page_bytes()),
        ("catalog cables", FixedOwnerVec::<CableKindCatalog, DOCUMENT_KIND_SLOTS>::sub_page_bytes()),
        ("kind compatibility", FixedOwnerVec::<KindCompatEntry, DOCUMENT_KIND_SLOTS>::sub_page_bytes()),
        ("placed lookup", FixedOwnerMap::<String, usize, DOCUMENT_OBJECT_SLOTS>::sub_page_bytes()),
        ("candidate cache", FixedOwnerMap::<String, Vec<BrushCompatibleCandidate>>::sub_page_bytes()),
        ("seed object ids", FixedOwnerMap::<String, (), DOCUMENT_OBJECT_SLOTS>::sub_page_bytes()),
        ("meshes", FixedOwnerMap::<String, CollisionBody, DOCUMENT_KIND_SLOTS>::sub_page_bytes()),
        ("blocked vortex ids", FixedOwnerMap::<String, (), DOCUMENT_VORTEX_SLOTS>::sub_page_bytes()),
        ("candidates seen", FixedOwnerMap::<String, (), DOCUMENT_CANDIDATE_SLOTS>::sub_page_bytes()),
        ("candidate classification", FixedOwnerMap::<String, BrushCompatibleCandidate, DOCUMENT_CANDIDATE_SLOTS>::sub_page_bytes()),
        ("kind weights", FixedOwnerMap::<String, f64, DOCUMENT_KIND_SLOTS>::sub_page_bytes()),
        ("collision entries", FixedOwnerMap::<String, CollisionAabb, DOCUMENT_OBJECT_SLOTS>::sub_page_bytes()),
        ("collision cells", FixedOwnerMap::<(i32, i32, i32), FixedOwnerSet<String>, DOCUMENT_CELL_SLOTS>::sub_page_bytes()),
        ("collision cell members", FixedOwnerMap::<String, ()>::sub_page_bytes()),
        ("collision oversized", FixedOwnerMap::<String, (), DOCUMENT_KIND_SLOTS>::sub_page_bytes()),
        ("collision query candidates", FixedOwnerMap::<String, (), DOCUMENT_OBJECT_SLOTS>::sub_page_bytes()),
    ] {
        assert!(bytes > 0 && bytes <= GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES, "the {owner} owner asks the guest for {bytes} contiguous bytes, over the {GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES}-byte ceiling");
    }
}

/// ⚖️ LAW: an owner wider than one sub-page reads, writes and retires across its boundaries — the
/// declared width is unchanged by the split, and one close grant gives back exactly one sub-page.
///
/// 🧊️ Index arithmetic across lazily claimed sub-pages is where a chunked owner silently loses
/// entries, and the retirement cursor is priced in owners per grant: a document-scale owner that
/// gave its whole backing back in one call would spend a whole interactive close step on it.
#[test]
fn an_owner_wider_than_one_sub_page_reads_writes_and_retires_across_its_boundaries() {
    let sub_page = FixedOwnerVec::<[u64; 32], DOCUMENT_OBJECT_SLOTS>::sub_page_slots();
    let filled = 2 * sub_page + 5;
    let mut values = FixedOwnerVec::<[u64; 32], DOCUMENT_OBJECT_SLOTS>::new();
    for index in 0..filled {
        assert_eq!(values.try_push([index as u64; 32]), Ok(()));
    }
    assert_eq!((values.len(), values.capacity()), (filled, DOCUMENT_OBJECT_SLOTS));
    assert_eq!(values.backing_credit(), Some((3, 3 * sub_page * size_of::<[u64; 32]>())), "three claimed sub-pages, credited as three owners");
    assert_eq!(values.get(sub_page - 1), Some(&[sub_page as u64 - 1; 32]));
    assert_eq!(values.get(sub_page), Some(&[sub_page as u64; 32]), "the first slot of the second sub-page");
    assert_eq!(values.get(filled - 1), Some(&[filled as u64 - 1; 32]));
    assert_eq!(values.get(filled), None);
    assert!(values.iter().enumerate().all(|(index, value)| value == &[index as u64; 32]), "iteration crosses the sub-page boundaries in order");
    assert_eq!(values.iter().count(), filled);
    *values.get_mut(sub_page + 1).expect("a slot on the second sub-page") = [u64::MAX; 32];
    assert_eq!(values.get(sub_page + 1), Some(&[u64::MAX; 32]));
    for index in (0..filled).rev() {
        let expected = if index == sub_page + 1 { [u64::MAX; 32] } else { [index as u64; 32] };
        assert_eq!(values.pop(), Some(expected));
    }
    assert!(values.pop().is_none());
    for remaining in (0..3).rev() {
        assert!(values.retire_backing(), "one sub-page per close grant");
        assert_eq!(values.backing_credit().map_or(0, |credit| credit.0), remaining);
    }
    assert!(!values.retire_backing() && values.terminal_owners_empty());
    assert_eq!(values.capacity(), 0, "a fully retired owner backs nothing and says so");
}

/// ⚖️ LAW: an owner whose MIDDLE sub-page the guest refused keeps every earlier sub-page readable,
/// reports the capacity it actually backs, and refuses the insert that needed the missing page.
///
/// 🧊️ Lazily claimed sub-pages move the refusal from construction to the boundary crossing: the
/// owner is alive and half-filled when the guest says no. Reporting the declared width there is the
/// same lie that turned build #29's out-of-memory refusal into a guest trap — the preflight every
/// collision mutation runs compares `len()` against `capacity()`, so the honest answer is the one
/// that turns an exhausted guest into a refused mutation instead of an `unreachable!`.
#[test]
fn an_owner_whose_middle_sub_page_was_refused_keeps_the_earlier_ones_and_reports_honest_capacity() {
    let backed = FixedOwnerVec::<[u64; 32], DOCUMENT_OBJECT_SLOTS>::sub_page_slots();
    assert!(backed < DOCUMENT_OBJECT_SLOTS && backed * size_of::<[u64; 32]>() <= GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES, "a 512 KiB declared page is backed by more than one sub-page");
    {
        let _fragmented = OwnerReservationLimit::install(usize::MAX, 1);
        let mut values = FixedOwnerVec::<[u64; 32], DOCUMENT_OBJECT_SLOTS>::new();
        assert_eq!(values.capacity(), DOCUMENT_OBJECT_SLOTS, "an owner that got its first sub-page still reports its declared width");
        for index in 0..backed {
            assert_eq!(values.try_push([index as u64; 32]), Ok(()), "slot {index} rides an already claimed sub-page");
        }
        let refused = [u64::MAX; 32];
        assert_eq!(values.try_push(refused), Err(refused), "the value that needed the refused sub-page is handed back whole");
        assert_eq!((values.len(), values.capacity()), (backed, backed), "a refused sub-page drops the reported capacity to what is actually backed");
        assert_eq!(values.get(0), Some(&[0u64; 32]));
        assert_eq!(values.get(backed - 1), Some(&[backed as u64 - 1; 32]));
        assert_eq!(values.iter().count(), backed, "every earlier sub-page stays readable");
        assert_eq!(values.backing_credit().map(|credit| credit.0), Some(1), "only the sub-pages it actually holds are credited");
        assert_eq!(values.pop(), Some([backed as u64 - 1; 32]));
    }
    let entries = FixedOwnerMap::<usize, [u64; 32], DOCUMENT_OBJECT_SLOTS>::sub_page_slots();
    assert!(entries < DOCUMENT_OBJECT_SLOTS);
    let _fragmented = OwnerReservationLimit::install(usize::MAX, 1);
    let mut map = FixedOwnerMap::<usize, [u64; 32], DOCUMENT_OBJECT_SLOTS>::new();
    for index in 0..entries {
        assert!(matches!(map.try_insert(index, [index as u64; 32]), Ok(FixedOwnerMapInsert::Inserted)));
    }
    assert_eq!(map.try_insert(entries, [7; 32]).expect_err("the entry that needed the refused sub-page"), (entries, [7; 32]));
    assert_eq!((map.len(), map.capacity()), (entries, entries));
    assert_eq!(map.get(&0), Some(&[0u64; 32]));
    assert_eq!(map.get(&(entries - 1)), Some(&[entries as u64 - 1; 32]));
    assert_eq!(map.iter().count(), entries);
}
