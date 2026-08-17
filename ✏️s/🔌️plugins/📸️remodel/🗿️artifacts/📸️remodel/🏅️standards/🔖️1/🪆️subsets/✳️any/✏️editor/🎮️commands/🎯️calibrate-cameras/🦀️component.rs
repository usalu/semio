//! 🎯️ 🎯️ Remodel play app commands command — `calibrate-cameras`.

use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use crate::artifacts::remodel::schema::next_remodel_id;
use crate::artifacts::remodel::mutations::{add_gcp_observation, create_camera_calibration, create_gcp, delete_gcp, update_camera_calibration};
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::{CameraCalibration, GcpObservation, GroundControlPoint, RemodelSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
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
pub fn handle(_payload: &CalibrateCameras, doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    let scene = doc.snapshot;
    let mut seen: Vec<String> = scene.calibration.cameras.iter().map(|camera| camera.id.clone()).collect();
    let mut mutations = Vec::new();
    for stream in &scene.streams {
        let Some(camera_id) = &stream.camera_id else { continue };
        if seen.iter().any(|id| id == camera_id) {
            continue;
        }
        let Some(frame) = stream.frames.first() else { continue };
        let Some(asset) = crate::artifacts::remodel::remodel_asset(&scene.assets, &frame.asset_id) else { continue };
        let (width, height) = (asset.width.max(1), asset.height.max(1));
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
