//! ↩️ Inverse for `ApplyValue`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::ApplyValue, base: &SemioSnapshot) -> Result<Vec<SemioMutation>, semio_framework_value::ValueError> {
    Ok(match &base.subset {
        SemioSubsetSnapshot::Value(b) => <SemioValueMutation as Mutation<SemioValueSnapshot>>::inverse(&payload.mutation, b)?.into_iter().map(|inner| SemioMutation::ApplyValue(super::ApplyValue { mutation: inner })).collect(),
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
