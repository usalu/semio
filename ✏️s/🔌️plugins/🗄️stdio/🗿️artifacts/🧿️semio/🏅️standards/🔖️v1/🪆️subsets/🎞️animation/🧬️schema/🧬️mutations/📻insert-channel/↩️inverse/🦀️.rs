//! ↩️ Inverse for `InsertChannel`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioAnimationSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::remove_channel;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::InsertChannel, _base: &SemioAnimationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::remove_channel::RemoveChannel>,
{
    
    let super::InsertChannel { timeline_index, index, .. } = payload;
    Ok(vec![Op::from(remove_channel::RemoveChannel { timeline_index: *timeline_index, index: *index })])
}
//#endregion 🔖️Inverse
