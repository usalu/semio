//! 🔺️ Sparse diff builder for `UpdateRigExtrinsic`. A missing target ⇒ Error
//! `mutation.target-missing`; a non-finite rotation or translation ⇒ Fatal `mutation.invariant`;
//! an identical pose ⇒ Warning `mutation.no-op`.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::UpdateRigExtrinsic, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
    let Some(existing) = base.calibration.rig.iter().find(|extrinsic| extrinsic.camera_id == payload.extrinsic.camera_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Rig extrinsic \"{}\" does not exist.", payload.extrinsic.camera_id), [payload.extrinsic.camera_id.clone()]);
    };
    if payload.extrinsic.rotation_wxyz.iter().any(|value| !value.is_finite()) || payload.extrinsic.translation_m.iter().any(|value| !value.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Rig extrinsic \"{}\" has a non-finite rotation or translation.", payload.extrinsic.camera_id), [payload.extrinsic.camera_id.clone()]);
    }
    if existing == &payload.extrinsic {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Rig extrinsic \"{}\" is unchanged.", payload.extrinsic.camera_id));
    }
    let mut calibration = base.calibration.clone();
    if let Some(existing) = calibration.rig.iter_mut().find(|extrinsic| extrinsic.camera_id == payload.extrinsic.camera_id) {
        *existing = payload.extrinsic.clone();
    }
    protocol::MutationOutcome::new(RemodelDiff { calibration: Some(calibration), ..Default::default() })
}
//#endregion 🔖️Diff
