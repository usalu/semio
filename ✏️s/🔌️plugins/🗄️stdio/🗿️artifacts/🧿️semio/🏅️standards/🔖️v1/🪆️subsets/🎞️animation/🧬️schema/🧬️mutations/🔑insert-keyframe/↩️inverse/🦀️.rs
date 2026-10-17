//! ↩️ Inverse for `InsertKeyframe`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioAnimationSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::remove_keyframe;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::InsertKeyframe, _base: &SemioAnimationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::remove_keyframe::RemoveKeyframe>,
{
    
    let super::InsertKeyframe { timeline_index, channel_index, index, .. } = payload;
    Ok({
        vec![Op::from(remove_keyframe::RemoveKeyframe { timeline_index: *timeline_index, channel_index: *channel_index, index: *index })]
    })
}
//#endregion 🔖️Inverse
