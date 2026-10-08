//! 🔺️ Sparse diff builder for `UpdateCameraCalibration`, in the vocabulary's one guard order:
//! missing target ⇒ Error, non-finite intrinsics/distortion ⇒ Fatal, identical resubmission ⇒ Warning.
use crate::diff::{RemodelingDiff, RemodelingRow};
use crate::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::UpdateCameraCalibration, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    let Some(existing) = base.calibration.cameras.iter().find(|camera| camera.id == payload.camera.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Camera calibration \"{}\" does not exist.", payload.camera.id), [payload.camera.id.clone()]);
    };
    let camera = &payload.camera;
    let non_finite = !camera.fx.is_finite()
        || !camera.fy.is_finite()
        || !camera.cx.is_finite()
        || !camera.cy.is_finite()
        || !camera.skew.is_finite()
        || camera.distortion.iter().any(|v| !v.is_finite())
        || camera.rms_reprojection_px.is_some_and(|v| !v.is_finite());
    if non_finite {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Camera calibration \"{}\" has non-finite intrinsics or distortion.", payload.camera.id), [payload.camera.id.clone()]);
    }
    if existing == &payload.camera {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Camera calibration \"{}\" is already up to date.", payload.camera.id));
    }
    protocol::MutationOutcome::new(RemodelingDiff::camera_rows(vec![RemodelingRow::Replace { entity: payload.camera.clone() }]))
}
//#endregion 🔖️Diff
