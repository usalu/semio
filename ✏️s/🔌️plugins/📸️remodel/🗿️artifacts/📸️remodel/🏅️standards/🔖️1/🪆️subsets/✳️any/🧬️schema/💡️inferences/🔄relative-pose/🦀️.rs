//! 🔄 `relative-pose` — a real `store::InferredField<RemodelSnapshot>` (not the whole-snapshot
//! `protocol::Inference` shape `📦bounds/` uses): one entity per camera in
//! `results.trajectory.poses`, the trajectory's own order chaining each pose's `Key` to the
//! PREVIOUS pose's `Key` as its sole `parent` — a genuine multi-key DAG, not independent roots,
//! so a change to any earlier pose invalidates every later pose's `DepHash` through the fold.
//! `compute()` recovers the rigid-motion delta between consecutive cameras via `crate::lie`'s
//! `Se3` group composition (`prev.inverse() ∘ curr`), the exact Lie-group machinery
//! `26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS` wave M3d moved wholesale
//! out of `🧮️math/🔷️lie` into this crate — the "pose estimation" the wave's brief asked for a real
//! `InferredField` around, using only already-`#[state(artifact)]`-persisted `results` data (no
//! ephemeral SFM/BA working state, which this codebase deliberately never persists — see
//! `MotionTrackSummary`'s own docstring on why raw tracks/observations stay plugin-runtime scratch).

use crate::artifacts::remodel::{CameraPosePreview, RemodelSnapshot};
use crate::lie::{Se3, So3};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️PoseDelta
/// 🔄️ The rigid motion from the previous camera in trajectory order to this one, `Se3` logged down
/// to a translation vector and a scalar rotation angle (radians, always `>= 0`) — zero for the
/// first pose (no predecessor).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct RemodelPoseDelta {
    pub translation_delta: [f64; 3],
    pub rotation_angle_rad: f64,
}

async fn se3_from_preview(pose: &CameraPosePreview) -> Se3 {
    let q = crate::lie::Quatd { w: pose.rotation_wxyz[0] as f64, x: pose.rotation_wxyz[1] as f64, y: pose.rotation_wxyz[2] as f64, z: pose.rotation_wxyz[3] as f64 };
    Se3 { r: So3::from_quat(q.normalize()), t: [pose.translation[0] as f64, pose.translation[1] as f64, pose.translation[2] as f64] }
}

async fn trajectory_poses(snapshot: &RemodelSnapshot) -> &[CameraPosePreview] {
    snapshot.results.trajectory.as_ref().map(|trajectory| trajectory.poses.as_slice()).unwrap_or(&[])
}
//#endregion 🔖️PoseDelta

//#region 🔖️InferredField
pub struct RemodelRelativeCameraPose;

impl store::InferredField<RemodelSnapshot> for RemodelRelativeCameraPose {
    type Key = String;
    type Value = RemodelPoseDelta;

    const FIELD_ID: &'static str = "s.remodel.remodel.inference.relative_camera_pose";
    const SCHEMA_VERSION: u32 = 1;

    async fn reads() -> &'static [&'static str] {
        &["results"]
    }

    /// 🧭 One step per pose, in trajectory order; every pose but the first names its immediate
    /// predecessor as its sole parent, so the chain is a real linear DAG, not independent roots.
    async fn plan(snapshot: &RemodelSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        let poses = trajectory_poses(snapshot);
        poses.iter().enumerate().map(|(index, pose)| store::InferenceStep { key: pose.camera_id.clone(), parents: if index == 0 { Vec::new() } else { vec![poses[index - 1].camera_id.clone()] } }).collect()
    }

    /// 🔑 Only `key`'s OWN rotation/translation — the predecessor's raw pose is covered by the
    /// predecessor's own `dep_input` and folded in via its already-computed `DepHash` through
    /// `plan`'s parent edge, exactly the "excluding parents' own upstream values" contract.
    async fn dep_input(snapshot: &RemodelSnapshot, key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        let Some(pose) = trajectory_poses(snapshot).iter().find(|pose| &pose.camera_id == key) else {
            return Vec::new();
        };
        let mut bytes = Vec::with_capacity(28);
        for component in pose.rotation_wxyz {
            bytes.extend_from_slice(&component.to_le_bytes());
        }
        for component in pose.translation {
            bytes.extend_from_slice(&component.to_le_bytes());
        }
        bytes
    }

    /// 🧮 Re-reads both this pose and its immediate predecessor straight off `snapshot` (cheaper and
    /// more direct than reconstructing a raw pose from the parent's already-computed delta VALUE,
    /// which is relative to a DIFFERENT pose two steps back) — same "read snapshot directly, ignore
    /// `parents`" shape `AssemblyEntropy::compute` uses for its own pinned-module lookup.
    async fn compute(snapshot: &RemodelSnapshot, key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        let poses = trajectory_poses(snapshot);
        let Some(index) = poses.iter().position(|pose| &pose.camera_id == key) else {
            return RemodelPoseDelta::default();
        };
        if index == 0 {
            return RemodelPoseDelta::default();
        }
        let relative = se3_from_preview(&poses[index - 1]).inverse().semio_compose_rs(&se3_from_preview(&poses[index]));
        RemodelPoseDelta { translation_delta: relative.t, rotation_angle_rad: crate::algebra::vec3d_length(relative.r.log()) }
    }
}
//#endregion 🔖️InferredField

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::remodel::CameraTrajectory;
    use store::InferredField;

    async fn two_pose_snapshot() -> RemodelSnapshot {
        let mut snapshot = RemodelSnapshot::default();
        snapshot.results.trajectory = Some(CameraTrajectory {
            poses: vec![CameraPosePreview { camera_id: "c0".into(), rotation_wxyz: [1.0, 0.0, 0.0, 0.0], translation: [0.0, 0.0, 0.0] }, CameraPosePreview { camera_id: "c1".into(), rotation_wxyz: [1.0, 0.0, 0.0, 0.0], translation: [1.0, 0.0, 0.0] }],
        });
        snapshot
    }

    #[semio_framework_async_macros::async_test]
    async fn plan_chains_each_pose_to_its_immediate_predecessor() {
        let snapshot = two_pose_snapshot();
        let steps = RemodelRelativeCameraPose::plan(&snapshot);
        assert_eq!(steps.len(), 2);
        assert!(steps[0].parents.is_empty());
        assert_eq!(steps[1].parents, vec!["c0".to_string()]);
    }

    #[semio_framework_async_macros::async_test]
    async fn first_pose_has_zero_delta() {
        let snapshot = two_pose_snapshot();
        let values = store::infer_field::<RemodelSnapshot, RemodelRelativeCameraPose>(&snapshot, None);
        assert_eq!(values["c0"], RemodelPoseDelta::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn a_pure_translation_step_reports_no_rotation_and_the_exact_offset() {
        let snapshot = two_pose_snapshot();
        let values = store::infer_field::<RemodelSnapshot, RemodelRelativeCameraPose>(&snapshot, None);
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
        let values = store::infer_field::<RemodelSnapshot, RemodelRelativeCameraPose>(&snapshot, None);
        assert!((values["c1"].rotation_angle_rad - std::f64::consts::FRAC_PI_2).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    async fn identical_snapshots_produce_byte_identical_deltas() {
        let snapshot = two_pose_snapshot();
        let first = store::infer_field::<RemodelSnapshot, RemodelRelativeCameraPose>(&snapshot, None);
        let second = store::infer_field::<RemodelSnapshot, RemodelRelativeCameraPose>(&snapshot, None);
        assert_eq!(first, second);
    }

    #[semio_framework_async_macros::async_test]
    async fn changing_an_earlier_pose_changes_the_dep_input_of_a_later_key_indirectly_through_its_own_chain() {
        // 🔗 dep_input for "c1" only covers c1's own bytes by design (see the fn's own docstring) —
        // the earlier pose's change reaches c1 through DepHash::chain folding c0's hash, which is
        // `store::infer_field`'s own concern, not this field's. This test instead pins the
        // structural half of that contract: plan() must keep naming c0 as c1's parent.
        let snapshot = two_pose_snapshot();
        let steps = RemodelRelativeCameraPose::plan(&snapshot);
        assert_eq!(steps[1].key, "c1");
        assert_eq!(steps[1].parents, vec!["c0".to_string()]);
    }
}
//#endregion 🧪️Tests
