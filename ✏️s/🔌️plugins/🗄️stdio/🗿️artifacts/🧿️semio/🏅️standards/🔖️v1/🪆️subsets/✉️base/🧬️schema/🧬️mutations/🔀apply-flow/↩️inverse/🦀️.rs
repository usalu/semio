//! ↩️ Inverse for `ApplyFlow`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::ApplyFlow, base: &SemioSnapshot) -> Result<Vec<SemioMutation>, semio_framework_value::ValueError> {
    Ok(match &base.subset {
        SemioSubsetSnapshot::Flow(b) => <SemioFlowMutation as Mutation<SemioFlowSnapshot>>::inverse(&payload.mutation, b)?.into_iter().map(|inner| SemioMutation::ApplyFlow(super::ApplyFlow { mutation: inner })).collect(),
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
