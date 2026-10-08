//! 🔺️ Sparse diff builder for `CreateRigExtrinsic` — inserts at the entry's canonical `camera_id`
//! position so `delete-rig-extrinsic` puts it back exactly where it was. A duplicate `camera_id` ⇒ Fatal
//! `mutation.duplicate-id`; a `camera_id` naming a camera this base does not calibrate ⇒ Error
//! `mutation.target-missing` (a base that calibrates it hosts the same payload).
use crate::diff::{RemodelingDiff, RemodelingRow};
use crate::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateRigExtrinsic, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if base.calibration.rig.iter().any(|extrinsic| extrinsic.camera_id == payload.extrinsic.camera_id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A rig extrinsic for camera \"{}\" already exists.", payload.extrinsic.camera_id), [payload.extrinsic.camera_id.clone()]);
    }
    if !base.calibration.cameras.iter().any(|camera| camera.id == payload.extrinsic.camera_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Rig extrinsic references unknown camera \"{}\".", payload.extrinsic.camera_id), [payload.extrinsic.camera_id.clone()]);
    }
    protocol::MutationOutcome::new(RemodelingDiff::rig_rows(vec![RemodelingRow::Insert { entity: payload.extrinsic.clone() }]))
}
//#endregion 🔖️Diff
