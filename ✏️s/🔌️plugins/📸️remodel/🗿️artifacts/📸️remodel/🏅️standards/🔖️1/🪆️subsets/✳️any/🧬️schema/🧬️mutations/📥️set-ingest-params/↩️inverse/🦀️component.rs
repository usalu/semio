//! ↩️ Inverse for `SetIngestParams`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetIngestParams { params: base.params.ingest.clone() }]
}
//#endregion 🔖️Inverse
