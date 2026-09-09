//! 🎯️ 🎯️ Remodeling play app commands command — `edit-calibration`.

use crate::editor::remodeling::config::{RemodelingConfig, RemodelingConfigMutation};
use crate::mutations::{create_camera_calibration, update_camera_calibration};
use crate::op::RemodelingMutation;
use crate::{CameraCalibration, RemodelingSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "edit-calibration")]
pub struct EditCalibration {
    pub camera_id: String,
    pub label: String,
    pub model: String,
    pub fx: f64,
    pub fy: f64,
    pub cx: f64,
    pub cy: f64,
    pub skew: f64,
    pub k1: f32,
    pub k2: f32,
    pub k3: f32,
    pub p1: f32,
    pub p2: f32,
    pub locked: bool,
}

pub fn handle(payload: &EditCalibration, doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, RemodelingConfig>) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation>, Fault> {
    let entry = CameraCalibration {
        id: payload.camera_id.clone(),
        label: payload.label.clone(),
        model: payload.model.clone(),
        fx: payload.fx,
        fy: payload.fy,
        cx: payload.cx,
        cy: payload.cy,
        skew: payload.skew,
        distortion: [payload.k1, payload.k2, payload.k3, payload.p1, payload.p2],
        rms_reprojection_px: None,
        locked: payload.locked,
    };
    let mutation = match doc.snapshot.calibration.cameras.iter().any(|camera| camera.id == payload.camera_id) {
        true => update_camera_calibration(entry),
        false => create_camera_calibration(entry),
    };
    Ok(Emit::mutations(vec![mutation]))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
