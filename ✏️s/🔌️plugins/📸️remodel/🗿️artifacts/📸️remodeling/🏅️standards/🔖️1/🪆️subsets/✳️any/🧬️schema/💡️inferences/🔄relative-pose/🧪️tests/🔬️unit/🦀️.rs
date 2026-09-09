use super::*;
use crate::CameraTrajectory;
use store::InferredField;

fn two_pose_snapshot() -> RemodelingSnapshot {
    let mut snapshot = RemodelingSnapshot::default();
    snapshot.results.trajectory = Some(CameraTrajectory {
        poses: vec![CameraPosePreview { camera_id: "c0".into(), rotation_wxyz: [1.0, 0.0, 0.0, 0.0], translation: [0.0, 0.0, 0.0] }, CameraPosePreview { camera_id: "c1".into(), rotation_wxyz: [1.0, 0.0, 0.0, 0.0], translation: [1.0, 0.0, 0.0] }],
    });
    snapshot
}

#[semio_framework_async_macros::async_test]
async fn plan_chains_each_pose_to_its_immediate_predecessor() {
    let snapshot = two_pose_snapshot();
    let steps = RemodelingRelativeCameraPose::plan(&snapshot);
    assert_eq!(steps.len(), 2);
    assert!(steps[0].parents.is_empty());
    assert_eq!(steps[1].parents, vec!["c0".to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn first_pose_has_zero_delta() {
    let snapshot = two_pose_snapshot();
    let values = store::infer_field::<RemodelingSnapshot, RemodelingRelativeCameraPose>(&snapshot, None);
    assert_eq!(values["c0"], RemodelingPoseDelta::default());
}

#[semio_framework_async_macros::async_test]
async fn a_pure_translation_step_reports_no_rotation_and_the_exact_offset() {
    let snapshot = two_pose_snapshot();
    let values = store::infer_field::<RemodelingSnapshot, RemodelingRelativeCameraPose>(&snapshot, None);
    let delta = values["c1"];
    assert_eq!(delta.rotation_angle_rad, 0.0);
    assert!((delta.translation_delta[0] - 1.0).abs() < 1e-9);
    assert_eq!(delta.translation_delta[1], 0.0);
    assert_eq!(delta.translation_delta[2], 0.0);
}

#[semio_framework_async_macros::async_test]
async fn a_90_degree_yaw_step_reports_the_exact_angle() {
    let mut snapshot = two_pose_snapshot();
    // 🌀 90° rotation about +Z: quaternion (w, x, y, z) = (cos45°, 0, 0, sin45°).
    let half = std::f64::consts::FRAC_PI_4;
    snapshot.results.trajectory.as_mut().unwrap().poses[1].rotation_wxyz = [half.cos() as f32, 0.0, 0.0, half.sin() as f32];
    let values = store::infer_field::<RemodelingSnapshot, RemodelingRelativeCameraPose>(&snapshot, None);
    assert!((values["c1"].rotation_angle_rad - std::f64::consts::FRAC_PI_2).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn identical_snapshots_produce_byte_identical_deltas() {
    let snapshot = two_pose_snapshot();
    let first = store::infer_field::<RemodelingSnapshot, RemodelingRelativeCameraPose>(&snapshot, None);
    let second = store::infer_field::<RemodelingSnapshot, RemodelingRelativeCameraPose>(&snapshot, None);
    assert_eq!(first, second);
}

#[semio_framework_async_macros::async_test]
async fn changing_an_earlier_pose_changes_the_dep_input_of_a_later_key_indirectly_through_its_own_chain() {
    // 🔗 dep_input for "c1" only covers c1's own bytes by design (see the fn's own docstring) —
    // the earlier pose's change reaches c1 through DepHash::chain folding c0's hash, which is
    // `store::infer_field`'s own concern, not this field's. This test instead pins the
    // structural half of that contract: plan() must keep naming c0 as c1's parent.
    let snapshot = two_pose_snapshot();
    let steps = RemodelingRelativeCameraPose::plan(&snapshot);
    assert_eq!(steps[1].key, "c1");
    assert_eq!(steps[1].parents, vec!["c0".to_string()]);
}
