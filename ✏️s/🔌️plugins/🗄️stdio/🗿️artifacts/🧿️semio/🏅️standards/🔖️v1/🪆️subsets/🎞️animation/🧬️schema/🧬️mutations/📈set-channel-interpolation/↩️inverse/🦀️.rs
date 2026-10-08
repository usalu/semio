//! ↩️ Inverse for `SetChannelInterpolation`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetChannelInterpolation, base: &SemioAnimationSnapshot) -> Result<Vec<SemioAnimationMutation>, semio_framework_value::ValueError> {
    use SemioAnimationMutation::*;
    let super::SetChannelInterpolation { timeline_index, index, .. } = payload;
    Ok(match channel_at(base, *timeline_index, *index) {
        Some(c) => vec![SetChannelInterpolation(set_channel_interpolation::SetChannelInterpolation { timeline_index: *timeline_index, index: *index, interpolation: c.interpolation })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
