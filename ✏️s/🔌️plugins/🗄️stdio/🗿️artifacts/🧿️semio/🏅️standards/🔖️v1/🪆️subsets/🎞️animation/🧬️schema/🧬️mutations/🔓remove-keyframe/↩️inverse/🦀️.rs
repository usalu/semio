//! ↩️ Inverse for `RemoveKeyframe`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioAnimationSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::insert_keyframe;
#[allow(unused_imports)]
use crate::schema::mutations::support::keyframe_at;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::RemoveKeyframe, base: &SemioAnimationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::insert_keyframe::InsertKeyframe>,
{
    
    let super::RemoveKeyframe { timeline_index, channel_index, index } = payload;
    Ok(match keyframe_at(base, *timeline_index, *channel_index, *index) {
        Some(k) => vec![Op::from(insert_keyframe::InsertKeyframe { timeline_index: *timeline_index, channel_index: *channel_index, index: *index, keyframe: k.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
