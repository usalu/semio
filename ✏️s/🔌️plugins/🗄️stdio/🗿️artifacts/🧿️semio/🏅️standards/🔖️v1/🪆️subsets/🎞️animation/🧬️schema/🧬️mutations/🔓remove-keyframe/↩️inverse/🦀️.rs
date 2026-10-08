//! ↩️ Inverse for `RemoveKeyframe`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveKeyframe, base: &SemioAnimationSnapshot) -> Result<Vec<SemioAnimationMutation>, semio_framework_value::ValueError> {
    use SemioAnimationMutation::*;
    let super::RemoveKeyframe { timeline_index, channel_index, index } = payload;
    Ok(match keyframe_at(base, *timeline_index, *channel_index, *index) {
        Some(k) => vec![InsertKeyframe(insert_keyframe::InsertKeyframe { timeline_index: *timeline_index, channel_index: *channel_index, index: *index, keyframe: k.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
