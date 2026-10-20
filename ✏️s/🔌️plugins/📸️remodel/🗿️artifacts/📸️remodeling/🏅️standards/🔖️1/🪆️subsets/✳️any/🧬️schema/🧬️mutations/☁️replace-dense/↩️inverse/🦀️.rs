//! ↩️ Inverse for `ReplaceDense` — the OLD `ReconstructionResults.dense` from BASE.
use crate::standards::v1::subsets::any::schema::mutations::replace_dense::component::ReplaceDense;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ReplaceDense>>(_payload: &crate::standards::v1::subsets::any::schema::mutations::replace_dense::component::ReplaceDense, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::replace_dense(base.results.dense.clone())]

    })())
}
//#endregion 🔖️Inverse
