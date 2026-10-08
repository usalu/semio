//! ↩️ Inverse for `InsertStream`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::InsertStream, base: &SemioVideoSnapshot) -> Result<Vec<SemioVideoMutation>, semio_framework_value::ValueError> {
    let super::InsertStream { index, .. } = payload;
    Ok(vec![SemioVideoMutation::RemoveStream(remove_stream::RemoveStream { index: *index })])
}
//#endregion 🔖️Inverse
