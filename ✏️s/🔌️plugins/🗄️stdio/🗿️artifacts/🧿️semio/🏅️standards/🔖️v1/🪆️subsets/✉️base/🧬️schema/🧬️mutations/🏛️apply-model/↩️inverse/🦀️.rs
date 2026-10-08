//! ↩️ Inverse for `ApplyModel`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::ApplyModel, base: &SemioSnapshot) -> Result<Vec<SemioMutation>, semio_framework_value::ValueError> {
    Ok(match &base.subset {
        SemioSubsetSnapshot::Model(b) => <SemioModelMutation as Mutation<SemioModelSnapshot>>::inverse(&payload.mutation, b)?.into_iter().map(|inner| SemioMutation::ApplyModel(super::ApplyModel { mutation: inner })).collect(),
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
