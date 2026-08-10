//! 🎯️ Remodel play app commands — camera calibration and ground control points.

use crate::apps::remodel::config::{RemodelConfig, RemodelConfigMutation};
use crate::artifacts::remodel::engine::next_remodel_id;
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::{CameraCalibration, GcpObservation, GroundControlPoint, RemodelSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️EditCalibration
pub mod edit_calibration {
    use super::*;

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

    pub fn handle(payload: &EditCalibration, doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
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
        let mut calibration = doc.snapshot.calibration.clone();
        match calibration.cameras.iter_mut().find(|camera| camera.id == payload.camera_id) {
            Some(existing) => *existing = entry,
            None => calibration.cameras.push(entry),
        }
        Ok(Emit::mutations(vec![RemodelMutation::SetCalibration { calibration }]))
    }
}
//#endregion 🔖️EditCalibration

//#region 🔖️CalibrateCameras
pub mod calibrate_cameras {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "calibrate-cameras")]
    pub struct CalibrateCameras {}

    /// 🎯️ Auto-derives placeholder pinhole intrinsics (`fx = fy = max(width, height)`, principal point
    /// centered, no distortion — mirroring the reconstruction engine's own uncalibrated-input heuristic)
    /// for every camera id referenced by a stream that has no calibration entry yet. A documented
    /// simplification standing in for a real Zhang/checkerboard calibration pass (no calibration target
    /// detection is wired into this program).
    pub fn handle(_payload: &CalibrateCameras, doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
        let scene = doc.snapshot;
        let mut calibration = scene.calibration.clone();
        for stream in &scene.streams {
            let Some(camera_id) = &stream.camera_id else { continue };
            if calibration.cameras.iter().any(|camera| &camera.id == camera_id) {
                continue;
            }
            let Some(frame) = stream.frames.first() else { continue };
            let Some(asset) = scene.assets.get(&frame.asset_id) else { continue };
            let (width, height) = (asset.width.max(1), asset.height.max(1));
            let f = f64::from(width.max(height));
            calibration.cameras.push(CameraCalibration {
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
            });
        }
        Ok(Emit::mutations(vec![RemodelMutation::SetCalibration { calibration }]))
    }
}
//#endregion 🔖️CalibrateCameras

//#region 🔖️AddGcp
pub mod add_gcp {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-gcp")]
    pub struct AddGcp {
        pub name: String,
        pub world_x: f64,
        pub world_y: f64,
        pub world_z: f64,
    }

    pub fn handle(payload: &AddGcp, doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
        let id = next_remodel_id("gcp");
        let mut gcps = doc.snapshot.gcps.clone();
        gcps.push(GroundControlPoint { id, name: payload.name.clone(), world_position: [payload.world_x, payload.world_y, payload.world_z], observations: Vec::new() });
        Ok(Emit::mutations(vec![RemodelMutation::SetGcps { gcps }]))
    }
}
//#endregion 🔖️AddGcp

//#region 🔖️RemoveGcp
pub mod remove_gcp {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "remove-gcp")]
    pub struct RemoveGcp {
        pub gcp_id: String,
    }

    pub fn handle(payload: &RemoveGcp, doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
        let gcps: Vec<GroundControlPoint> = doc.snapshot.gcps.iter().filter(|gcp| gcp.id != payload.gcp_id).cloned().collect();
        Ok(Emit::mutations(vec![RemodelMutation::SetGcps { gcps }]))
    }
}
//#endregion 🔖️RemoveGcp

//#region 🔖️PlaceGcpObservation
pub mod place_gcp_observation {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "place-gcp-observation")]
    pub struct PlaceGcpObservation {
        pub gcp_id: String,
        pub stream_id: String,
        pub frame_index: u32,
        pub pixel_x: f32,
        pub pixel_y: f32,
    }

    pub fn handle(payload: &PlaceGcpObservation, doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
        let mut gcps = doc.snapshot.gcps.clone();
        let Some(gcp) = gcps.iter_mut().find(|gcp| gcp.id == payload.gcp_id) else { return Ok(Emit::default()) };
        gcp.observations.push(GcpObservation { stream_id: payload.stream_id.clone(), frame_index: payload.frame_index, pixel: [payload.pixel_x, payload.pixel_y] });
        Ok(Emit::mutations(vec![RemodelMutation::SetGcps { gcps }]))
    }
}
//#endregion 🔖️PlaceGcpObservation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::remodel::testkit::{app, dispatch};
    use crate::apps::remodel::RemodelCommand;

    #[test]
    fn edit_calibration_inserts_then_updates_the_same_camera_entry() {
        let mut app = app();
        let payload = |fx: f64| {
            RemodelCommand::EditCalibration(edit_calibration::EditCalibration {
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

    #[test]
    fn gcps_are_added_observed_and_removed() {
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
    #[test]
    fn calibrate_cameras_skips_streams_without_frames() {
        let mut app = app();
        dispatch(&mut app, RemodelCommand::AddStream(crate::apps::remodel::commands::ingest::add_stream::AddStream { name: "Front".into(), kind: "video".into(), camera_id: "cam-0".into() }));
        dispatch(&mut app, RemodelCommand::CalibrateCameras(calibrate_cameras::CalibrateCameras {}));
        assert!(app.snapshot().expect("projection").calibration.cameras.is_empty());
    }
}
//#endregion 🧪️Tests
