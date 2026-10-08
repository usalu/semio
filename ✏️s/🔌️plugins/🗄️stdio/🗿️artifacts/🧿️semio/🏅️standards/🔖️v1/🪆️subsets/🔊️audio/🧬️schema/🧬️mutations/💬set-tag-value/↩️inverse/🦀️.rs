//! ↩️ Inverse for `SetTagValue`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetTagValue, base: &SemioAudioSnapshot) -> Result<Vec<SemioAudioMutation>, semio_framework_value::ValueError> {
    let super::SetTagValue { index, .. } = payload;
    Ok(vec![match base.tags.get(*index) {
        Some(t) => SemioAudioMutation::SetTagValue(set_tag_value::SetTagValue { index: *index, value: t.value.clone() }),
        None => return Ok(Vec::new()),
    }])
}
//#endregion 🔖️Inverse
