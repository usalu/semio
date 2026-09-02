//! ↩️ Inverse for `UpdateIngestParams` — the OLD `IngestParams` looked up from BASE.
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::UpdateIngestParams, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    vec![super::update_ingest_params(base.params.ingest.clone())]
}
//#endregion 🔖️Inverse
