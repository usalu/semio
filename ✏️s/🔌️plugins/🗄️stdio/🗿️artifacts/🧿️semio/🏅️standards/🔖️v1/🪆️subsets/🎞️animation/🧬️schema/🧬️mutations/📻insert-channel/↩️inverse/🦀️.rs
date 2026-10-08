//! ↩️ Inverse for `InsertChannel`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::InsertChannel, base: &SemioAnimationSnapshot) -> Result<Vec<SemioAnimationMutation>, semio_framework_value::ValueError> {
    use SemioAnimationMutation::*;
    let super::InsertChannel { timeline_index, index, .. } = payload;
    Ok(vec![RemoveChannel(remove_channel::RemoveChannel { timeline_index: *timeline_index, index: *index })])
}
//#endregion 🔖️Inverse
