//! ↩️ Inverse for `UpdateIngestParams` — the OLD `IngestParams` looked up from BASE.
use crate::mutations::RemodelingMutation;
use crate::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::UpdateIngestParams, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    vec![super::update_ingest_params(base.params.ingest.clone())]
}
//#endregion 🔖️Inverse
