//! ↩️ Inverse for `ReplaceQc` — the OLD `ReconstructionResults.qc` from BASE.
use crate::standards::v1::subsets::any::schema::mutations::replace_qc::ReplaceQc;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ReplaceQc>>(_payload: &super::ReplaceQc, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::replace_qc(base.results.qc.clone())]

    })())
}
//#endregion 🔖️Inverse
