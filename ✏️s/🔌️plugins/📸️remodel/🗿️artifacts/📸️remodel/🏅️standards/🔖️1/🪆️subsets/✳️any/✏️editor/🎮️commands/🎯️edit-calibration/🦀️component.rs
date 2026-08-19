//! 🎯️ 🎯️ Remodel play app commands command — `edit-calibration`.

use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use crate::artifacts::remodel::mutations::{create_camera_calibration, update_camera_calibration};
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::{CameraCalibration, RemodelSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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

pub async fn handle(payload: &EditCalibration, doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
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
mod tests {
    use super::*;
    use crate::editor::remodel::commands::{add_gcp, calibrate_cameras, place_gcp_observation, remove_gcp};
    use crate::editor::remodel::testkit::{app, dispatch};
    use crate::editor::remodel::RemodelCommand;

    #[semio_framework_async_macros::async_test]
    async fn edit_calibration_inserts_then_updates_the_same_camera_entry() {
        let mut app = app();
        let payload = |fx: f64| {
            RemodelCommand::EditCalibration(EditCalibration {
                camera_id: "cam-1".into(),
                label: "Front".into(),
                model: "pinhole".into(),
                fx,
                fy: fx,
                cx: 0.0,
                cy: 0.0,
                skew: 0.0,
                k1: 0.0,
                k2: 0.0,
                k3: 0.0,
                p1: 0.0,
                p2: 0.0,
                locked: false,
            })
        };
        dispatch(&mut app, payload(1000.0));
        assert_eq!(app.snapshot().expect("projection").calibration.cameras.len(), 1);
        dispatch(&mut app, payload(2000.0));
        let cameras = app.snapshot().expect("projection").calibration.cameras;
        assert_eq!(cameras.len(), 1, "the same camera id is updated in place, never duplicated");
        assert_eq!(cameras[0].fx, 2000.0);
    }

    #[semio_framework_async_macros::async_test]
    async fn gcps_are_added_observed_and_removed() {
        let mut app = app();
        dispatch(&mut app, RemodelCommand::AddGcp(add_gcp::AddGcp { name: "Corner".into(), world_x: 1.0, world_y: 2.0, world_z: 3.0 }));
        let gcp_id = app.snapshot().expect("projection").gcps[0].id.clone();
        dispatch(&mut app, RemodelCommand::PlaceGcpObservation(place_gcp_observation::PlaceGcpObservation { gcp_id: gcp_id.clone(), stream_id: "stream-1".into(), frame_index: 0, pixel_x: 10.0, pixel_y: 20.0 }));
        assert_eq!(app.snapshot().expect("projection").gcps[0].observations.len(), 1);
        dispatch(&mut app, RemodelCommand::RemoveGcp(remove_gcp::RemoveGcp { gcp_id }));
        assert!(app.snapshot().expect("projection").gcps.is_empty());
    }

    /// 🎯️ `calibrateCameras` only derives intrinsics for stream-referenced cameras that have a decoded
    /// first frame — a stream with no frames contributes nothing.
    #[semio_framework_async_macros::async_test]
    async fn calibrate_cameras_skips_streams_without_frames() {
        let mut app = app();
        dispatch(&mut app, RemodelCommand::AddStream(crate::editor::remodel::commands::add_stream::AddStream { name: "Front".into(), kind: "video".into(), camera_id: "cam-0".into() }));
        dispatch(&mut app, RemodelCommand::CalibrateCameras(calibrate_cameras::CalibrateCameras {}));
        assert!(app.snapshot().expect("projection").calibration.cameras.is_empty());
    }
}
//#endregion 🧪️Tests
