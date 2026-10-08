//! ↩️ Inverse for `RemoveTag`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveTag, base: &SemioAudioSnapshot) -> Result<Vec<SemioAudioMutation>, semio_framework_value::ValueError> {
    let super::RemoveTag { index } = payload;
    Ok(vec![match base.tags.get(*index) {
        Some(t) => SemioAudioMutation::InsertTag(insert_tag::InsertTag { index: *index, tag: t.clone() }),
        None => return Ok(Vec::new()),
    }])
}
//#endregion 🔖️Inverse
