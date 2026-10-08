//! ↩️ Inverse for `SetKeyframeValue`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetKeyframeValue, base: &SemioAnimationSnapshot) -> Result<Vec<SemioAnimationMutation>, semio_framework_value::ValueError> {
    use SemioAnimationMutation::*;
    let super::SetKeyframeValue { timeline_index, channel_index, index, .. } = payload;
    Ok(match keyframe_at(base, *timeline_index, *channel_index, *index) {
        Some(k) => vec![SetKeyframeValue(set_keyframe_value::SetKeyframeValue { timeline_index: *timeline_index, channel_index: *channel_index, index: *index, value: k.value.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
