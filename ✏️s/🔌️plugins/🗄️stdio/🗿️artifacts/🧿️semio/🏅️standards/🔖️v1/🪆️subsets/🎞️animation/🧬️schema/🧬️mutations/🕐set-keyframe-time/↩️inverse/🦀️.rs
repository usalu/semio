//! ↩️ Inverse for `SetKeyframeTime`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetKeyframeTime, base: &SemioAnimationSnapshot) -> Result<Vec<SemioAnimationMutation>, semio_framework_value::ValueError> {
    use SemioAnimationMutation::*;
    let super::SetKeyframeTime { timeline_index, channel_index, index, .. } = payload;
    Ok(match keyframe_at(base, *timeline_index, *channel_index, *index) {
        Some(k) => vec![SetKeyframeTime(set_keyframe_time::SetKeyframeTime { timeline_index: *timeline_index, channel_index: *channel_index, index: *index, t: k.t })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
