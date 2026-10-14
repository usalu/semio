//! ↩️ Inverse for `UpdateIngestParams` — the OLD `IngestParams` looked up from BASE.
use crate::standards::v1::subsets::any::schema::mutations::update_ingest_params::UpdateIngestParams;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<UpdateIngestParams>>(_payload: &super::UpdateIngestParams, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::update_ingest_params(base.params.ingest.clone())]

    })())
}
//#endregion 🔖️Inverse
