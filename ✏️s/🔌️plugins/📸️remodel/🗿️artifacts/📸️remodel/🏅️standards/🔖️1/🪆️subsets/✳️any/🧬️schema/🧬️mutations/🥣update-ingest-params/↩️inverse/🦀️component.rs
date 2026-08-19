//! ↩️ Inverse for `UpdateIngestParams` — the OLD `IngestParams` looked up from BASE.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::mutation::UpdateIngestParams, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![super::mutation::update_ingest_params(base.params.ingest.clone())]
}
//#endregion 🔖️Inverse
