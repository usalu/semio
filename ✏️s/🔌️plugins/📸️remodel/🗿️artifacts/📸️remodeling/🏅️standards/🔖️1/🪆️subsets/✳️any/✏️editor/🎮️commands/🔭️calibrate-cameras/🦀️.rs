//! 🎯️ 🎯️ Remodeling play app commands command — `calibrate-cameras`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_s_artifact_remodel_remodeling_mut_calib::standards::v1::subsets::any::schema::mutations::create_camera_calibration::create_camera_calibration;
use semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::RemodelingMutation;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::CameraCalibration;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "calibrate-cameras")]
pub struct CalibrateCameras {}

/// 🎯️ Auto-derives placeholder pinhole intrinsics (`fx = fy = max(width, height)`, principal point
/// centered, no distortion — mirroring the reconstruction engine's own uncalibrated-input heuristic)
/// for every camera id referenced by a stream that has no calibration entry yet, one
/// `create-camera-calibration` per newly-derived camera. A documented simplification standing in for
/// a real Zhang/checkerboard calibration pass (no calibration target detection is wired into this
/// program).
pub fn handle(_payload: &CalibrateCameras, doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
    let scene = doc.snapshot;
    let mut seen: Vec<String> = scene.calibration.cameras.iter().map(|camera| camera.id.clone()).collect();
    let mut mutations = Vec::new();
    for stream in &scene.streams {
        let Some(camera_id) = &stream.camera_id else { continue };
        if seen.iter().any(|id| id == camera_id) {
            continue;
        }
        let Some(frame) = stream.frames.first() else { continue };
        let Some((width, height)) = semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::remodeling_asset_dimensions(scene, &frame.asset_id) else { continue };
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
