//! 🔺️ Sparse diff builder for `UpdateCameraCalibration`. Missing target ⇒ Error; identical
//! resubmission ⇒ Warning; non-finite intrinsics/distortion ⇒ Fatal.
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::UpdateCameraCalibration, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    let Some(existing) = base.calibration.cameras.iter().find(|camera| camera.id == payload.camera.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Camera calibration \"{}\" does not exist.", payload.camera.id), [payload.camera.id.clone()]);
    };
    if existing == &payload.camera {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Camera calibration \"{}\" is already up to date.", payload.camera.id));
    }
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
    let mut calibration = base.calibration.clone();
    if let Some(existing) = calibration.cameras.iter_mut().find(|camera| camera.id == payload.camera.id) {
        *existing = payload.camera.clone();
    }
    protocol::MutationOutcome::new(RemodelingDiff { calibration: Some(calibration), ..Default::default() })
}
//#endregion 🔖️Diff
