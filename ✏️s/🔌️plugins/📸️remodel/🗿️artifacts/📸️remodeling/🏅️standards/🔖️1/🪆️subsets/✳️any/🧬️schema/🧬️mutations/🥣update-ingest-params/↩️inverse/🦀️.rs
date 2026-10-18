//! ↩️ Inverse for `UpdateIngestParams` — the OLD `IngestParams` looked up from BASE.
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::update_ingest_params::component::UpdateIngestParams;
use crate::standards::v1::subsets::any::schema::mutations::update_ingest_params::component::update_ingest_params;

//#region 🔖️Inverse
pub fn inverse<Op: From<UpdateIngestParams>>(_payload: &super::UpdateIngestParams, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::update_ingest_params(base.params.ingest.clone())]

    })())
}
//#endregion 🔖️Inverse
