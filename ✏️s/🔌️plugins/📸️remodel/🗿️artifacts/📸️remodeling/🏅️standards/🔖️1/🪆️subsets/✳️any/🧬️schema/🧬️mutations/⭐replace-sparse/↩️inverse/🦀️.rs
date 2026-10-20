//! ↩️ Inverse for `ReplaceSparse` — the OLD `ReconstructionResults.sparse` from BASE.
use crate::standards::v1::subsets::any::schema::mutations::replace_sparse::component::ReplaceSparse;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ReplaceSparse>>(_payload: &crate::standards::v1::subsets::any::schema::mutations::replace_sparse::component::ReplaceSparse, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::replace_sparse(base.results.sparse.clone())]

    })())
}
//#endregion 🔖️Inverse
