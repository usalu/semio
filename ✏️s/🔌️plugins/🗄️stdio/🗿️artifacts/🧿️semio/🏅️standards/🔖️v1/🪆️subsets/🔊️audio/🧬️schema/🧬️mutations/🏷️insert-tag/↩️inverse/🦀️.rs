//! ↩️ Inverse for `InsertTag`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op: SemioAudioOp>(payload: &super::InsertTag, base: &SemioAudioSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    let super::InsertTag { index, .. } = payload;
    Ok(vec![Op::from(remove_tag::RemoveTag { index: (*index).min(base.tags.len()) })])
}
//#endregion 🔖️Inverse
