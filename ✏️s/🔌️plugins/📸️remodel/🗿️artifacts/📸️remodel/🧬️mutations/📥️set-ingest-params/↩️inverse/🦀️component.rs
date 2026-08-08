//! ↩️ Inverse for `SetIngestParams`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelProjection) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetIngestParams { params: base.params.ingest.clone() }]
}
//#endregion 🔖️Inverse
