//! ↩️ Inverse for `InsertKeyframe`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::InsertKeyframe, base: &SemioAnimationSnapshot) -> Result<Vec<SemioAnimationMutation>, semio_framework_value::ValueError> {
    use SemioAnimationMutation::*;
    let super::InsertKeyframe { timeline_index, channel_index, index, .. } = payload;
    Ok({
        vec![RemoveKeyframe(remove_keyframe::RemoveKeyframe { timeline_index: *timeline_index, channel_index: *channel_index, index: *index })]
    })
}
//#endregion 🔖️Inverse
