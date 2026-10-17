//! ↩️ Inverse for `SetKeyframeValue`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioAnimationSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::keyframe_at;
#[allow(unused_imports)]
use crate::schema::mutations::set_keyframe_value;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetKeyframeValue, base: &SemioAnimationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_keyframe_value::SetKeyframeValue>,
{
    
    let super::SetKeyframeValue { timeline_index, channel_index, index, .. } = payload;
    Ok(match keyframe_at(base, *timeline_index, *channel_index, *index) {
        Some(k) => vec![Op::from(set_keyframe_value::SetKeyframeValue { timeline_index: *timeline_index, channel_index: *channel_index, index: *index, value: k.value.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
