//! ↩️ Inverse for `InsertTag`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::InsertTag, base: &SemioAudioSnapshot) -> Result<Vec<SemioAudioMutation>, semio_framework_value::ValueError> {
    let super::InsertTag { index, .. } = payload;
    Ok(vec![SemioAudioMutation::RemoveTag(remove_tag::RemoveTag { index: (*index).min(base.tags.len()) })])
}
//#endregion 🔖️Inverse
