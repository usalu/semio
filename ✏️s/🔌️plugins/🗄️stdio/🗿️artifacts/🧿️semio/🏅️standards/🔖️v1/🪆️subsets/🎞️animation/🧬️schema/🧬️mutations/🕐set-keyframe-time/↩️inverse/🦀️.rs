//! ↩️ Inverse for `SetKeyframeTime`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioAnimationSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::keyframe_at;
#[allow(unused_imports)]
use crate::schema::mutations::set_keyframe_time;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetKeyframeTime, base: &SemioAnimationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_keyframe_time::SetKeyframeTime>,
{
    
    let super::SetKeyframeTime { timeline_index, channel_index, index, .. } = payload;
    Ok(match keyframe_at(base, *timeline_index, *channel_index, *index) {
        Some(k) => vec![Op::from(set_keyframe_time::SetKeyframeTime { timeline_index: *timeline_index, channel_index: *channel_index, index: *index, t: k.t })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
