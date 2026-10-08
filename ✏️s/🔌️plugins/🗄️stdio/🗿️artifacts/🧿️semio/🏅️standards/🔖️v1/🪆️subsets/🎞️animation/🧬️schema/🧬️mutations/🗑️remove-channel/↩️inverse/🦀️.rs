//! ↩️ Inverse for `RemoveChannel`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveChannel, base: &SemioAnimationSnapshot) -> Result<Vec<SemioAnimationMutation>, semio_framework_value::ValueError> {
    use SemioAnimationMutation::*;
    let super::RemoveChannel { timeline_index, index } = payload;
    Ok(match channel_at(base, *timeline_index, *index) {
        Some(c) => vec![InsertChannel(insert_channel::InsertChannel { timeline_index: *timeline_index, index: *index, channel: c.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
