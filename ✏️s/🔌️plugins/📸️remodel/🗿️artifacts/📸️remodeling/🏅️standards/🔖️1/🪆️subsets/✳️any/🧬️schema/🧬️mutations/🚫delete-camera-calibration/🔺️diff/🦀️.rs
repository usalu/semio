//! 🔺️ Sparse diff builder for `DeleteCameraCalibration`. Missing target ⇒ Error; a camera any stream
//! binds through `camera_id` or any rig extrinsic names ⇒ Error `mutation.referenced` — a calibration
//! owns nothing, so it may not be removed while another record still depends on it (the same
//! ownership rule `delete-stream` and `delete-asset` follow). Refusing rather than cascading keeps
//! `create-camera-calibration` the exact inverse: the delete never destroys a record it does not own.
use crate::diff::RemodelingDiff;
use crate::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteCameraCalibration, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if !base.calibration.cameras.iter().any(|camera| camera.id == payload.camera_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Camera calibration \"{}\" does not exist.", payload.camera_id), [payload.camera_id.clone()]);
    }
    let mut referencing: Vec<String> = base.streams.iter().filter(|stream| stream.camera_id.as_deref() == Some(payload.camera_id.as_str())).map(|stream| stream.id.clone()).collect();
    if base.calibration.rig.iter().any(|extrinsic| extrinsic.camera_id == payload.camera_id) {
        referencing.push(format!("calibration.rig.{}", payload.camera_id));
    }
    if !referencing.is_empty() {
        return protocol::MutationOutcome::error("mutation.referenced", format!("Camera calibration \"{}\" is still referenced by {} record(s); detach them first.", payload.camera_id, referencing.len()), referencing);
    }
    let mut calibration = base.calibration.clone();
    calibration.cameras.retain(|camera| camera.id != payload.camera_id);
    protocol::MutationOutcome::new(RemodelingDiff { calibration: Some(calibration), ..Default::default() })
}
//#endregion 🔖️Diff
