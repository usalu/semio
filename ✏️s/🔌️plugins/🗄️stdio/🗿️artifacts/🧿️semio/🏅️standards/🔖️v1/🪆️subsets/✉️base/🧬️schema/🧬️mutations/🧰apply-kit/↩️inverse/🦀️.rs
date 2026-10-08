//! ↩️ Inverse for `ApplyKit`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::ApplyKit, base: &SemioSnapshot) -> Result<Vec<SemioMutation>, semio_framework_value::ValueError> {
    Ok(match &base.subset {
        SemioSubsetSnapshot::Kit(b) => <SemioKitMutation as Mutation<SemioKitSnapshot>>::inverse(&payload.mutation, b)?.into_iter().map(|inner| SemioMutation::ApplyKit(super::ApplyKit { mutation: inner })).collect(),
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
