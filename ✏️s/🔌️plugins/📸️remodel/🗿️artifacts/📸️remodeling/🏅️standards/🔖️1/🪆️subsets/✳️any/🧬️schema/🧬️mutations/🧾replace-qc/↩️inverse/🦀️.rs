//! ↩️ Inverse for `ReplaceQc` — the OLD `ReconstructionResults.qc` from BASE.
use crate::mutations::RemodelingMutation;
use crate::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ReplaceQc, base: &RemodelingSnapshot) -> Result<Vec<RemodelingMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::replace_qc(base.results.qc.clone())]

    })())
}
//#endregion 🔖️Inverse
