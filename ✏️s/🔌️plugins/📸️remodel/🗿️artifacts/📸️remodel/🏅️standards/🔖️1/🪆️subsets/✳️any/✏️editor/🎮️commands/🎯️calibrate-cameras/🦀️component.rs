//! 🎯️ 🎯️ Remodel play app commands command — `calibrate-cameras`.

use crate::artifacts::remodel::mutations::create_camera_calibration;
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::{CameraCalibration, RemodelSnapshot};
use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "calibrate-cameras")]
pub struct CalibrateCameras {}

/// 🎯️ Auto-derives placeholder pinhole intrinsics (`fx = fy = max(width, height)`, principal point
/// centered, no distortion — mirroring the reconstruction engine's own uncalibrated-input heuristic)
/// for every camera id referenced by a stream that has no calibration entry yet, one
/// `create-camera-calibration` per newly-derived camera. A documented simplification standing in for
/// a real Zhang/checkerboard calibration pass (no calibration target detection is wired into this
/// program).
pub async fn handle(_payload: &CalibrateCameras, doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    let scene = doc.snapshot;
    let mut seen: Vec<String> = scene.calibration.cameras.iter().map(|camera| camera.id.clone()).collect();
    let mut mutations = Vec::new();
    for stream in &scene.streams {
        let Some(camera_id) = &stream.camera_id else { continue };
        if seen.iter().any(|id| id == camera_id) {
            continue;
        }
        let Some(frame) = stream.frames.first() else { continue };
        let Some((width, height)) = crate::artifacts::remodel::remodel_asset_dimensions(scene, &frame.asset_id) else { continue };
        let (width, height) = (width.max(1), height.max(1));
        let f = f64::from(width.max(height));
        seen.push(camera_id.clone());
        mutations.push(create_camera_calibration(CameraCalibration {
            id: camera_id.clone(),
            label: camera_id.clone(),
            model: "pinhole".into(),
            fx: f,
            fy: f,
            cx: f64::from(width) / 2.0,
            cy: f64::from(height) / 2.0,
            skew: 0.0,
            distortion: [0.0; 5],
            rms_reprojection_px: None,
            locked: false,
        }));
    }
    Ok(Emit::mutations(mutations))
}
