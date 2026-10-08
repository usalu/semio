//! ↩️ Inverse for `InsertSample`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::InsertSample, base: &SemioVideoSnapshot) -> Result<Vec<SemioVideoMutation>, semio_framework_value::ValueError> {
    let super::InsertSample { stream_index, index, .. } = payload;
    Ok(vec![SemioVideoMutation::RemoveSample(remove_sample::RemoveSample { stream_index: *stream_index, index: *index })])
}
//#endregion 🔖️Inverse
