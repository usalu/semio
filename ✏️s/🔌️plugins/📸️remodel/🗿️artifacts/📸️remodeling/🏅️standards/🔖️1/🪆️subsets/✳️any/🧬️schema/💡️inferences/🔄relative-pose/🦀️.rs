//! 🔄 `relative-pose` — a real `store::InferredField<RemodelingSnapshot>` (not the whole-snapshot
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

use crate::{CameraPosePreview, RemodelingSnapshot};
use crate::lie::{Se3, So3};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️PoseDelta
/// 🔄️ The rigid motion from the previous camera in trajectory order to this one, `Se3` logged down
/// to a translation vector and a scalar rotation angle (radians, always `>= 0`) — zero for the
/// first pose (no predecessor).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct RemodelingPoseDelta {
    pub translation_delta: [f64; 3],
    pub rotation_angle_rad: f64,
}

fn se3_from_preview(pose: &CameraPosePreview) -> Se3 {
    let q = crate::lie::Quatd { w: pose.rotation_wxyz[0] as f64, x: pose.rotation_wxyz[1] as f64, y: pose.rotation_wxyz[2] as f64, z: pose.rotation_wxyz[3] as f64 };
    Se3 { r: So3::from_quat(q.normalize()), t: [pose.translation[0] as f64, pose.translation[1] as f64, pose.translation[2] as f64] }
}

fn trajectory_poses(snapshot: &RemodelingSnapshot) -> &[CameraPosePreview] {
    snapshot.results.trajectory.as_ref().map_or(&[], |trajectory| trajectory.poses.as_slice())
}
//#endregion 🔖️PoseDelta

//#region 🔖️InferredField
pub struct RemodelingRelativeCameraPose;

impl store::InferredField<RemodelingSnapshot> for RemodelingRelativeCameraPose {
    type Key = String;
    type Value = RemodelingPoseDelta;

    const FIELD_ID: &'static str = "s.remodel.remodeling.inference.relative_camera_pose";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["results"]
    }

    /// 🧭 One step per pose, in trajectory order; every pose but the first names its immediate
    /// predecessor as its sole parent, so the chain is a real linear DAG, not independent roots.
    fn plan(snapshot: &RemodelingSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        let poses = trajectory_poses(snapshot);
        poses.iter().enumerate().map(|(index, pose)| store::InferenceStep { key: pose.camera_id.clone(), parents: if index == 0 { Vec::new() } else { vec![poses[index - 1].camera_id.clone()] } }).collect()
    }

    /// 🔑 Only `key`'s OWN rotation/translation — the predecessor's raw pose is covered by the
    /// predecessor's own `dep_input` and folded in via its already-computed `DepHash` through
    /// `plan`'s parent edge, exactly the "excluding parents' own upstream values" contract.
    fn dep_input(snapshot: &RemodelingSnapshot, key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
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
    fn compute(snapshot: &RemodelingSnapshot, key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        let poses = trajectory_poses(snapshot);
        let Some(index) = poses.iter().position(|pose| &pose.camera_id == key) else {
            return RemodelingPoseDelta::default();
        };
        if index == 0 {
            return RemodelingPoseDelta::default();
        }
        let relative = se3_from_preview(&poses[index - 1]).inverse().semio_compose_rs(&se3_from_preview(&poses[index]));
        RemodelingPoseDelta { translation_delta: relative.t, rotation_angle_rad: crate::algebra::vec3d_length(relative.r.log()) }
    }
}
//#endregion 🔖️InferredField

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
