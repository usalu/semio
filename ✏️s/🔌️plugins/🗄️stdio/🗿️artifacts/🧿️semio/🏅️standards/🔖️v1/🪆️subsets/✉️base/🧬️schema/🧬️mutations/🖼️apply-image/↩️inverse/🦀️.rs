//! ↩️ Inverse for `ApplyImage`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::ApplyImage, base: &SemioSnapshot) -> Result<Vec<SemioMutation>, semio_framework_value::ValueError> {
    Ok(match &base.subset {
        SemioSubsetSnapshot::Image(b) => <SemioImageMutation as Mutation<SemioImageSnapshot>>::inverse(&payload.mutation, b)?.into_iter().map(|inner| SemioMutation::ApplyImage(super::ApplyImage { mutation: inner })).collect(),
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
