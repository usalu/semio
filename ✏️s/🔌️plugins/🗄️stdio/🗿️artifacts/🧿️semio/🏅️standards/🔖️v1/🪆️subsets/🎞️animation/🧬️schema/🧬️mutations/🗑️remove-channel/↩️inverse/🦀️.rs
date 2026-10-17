//! ↩️ Inverse for `RemoveChannel`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioAnimationSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::channel_at;
#[allow(unused_imports)]
use crate::schema::mutations::insert_channel;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::RemoveChannel, base: &SemioAnimationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::insert_channel::InsertChannel>,
{
    
    let super::RemoveChannel { timeline_index, index } = payload;
    Ok(match channel_at(base, *timeline_index, *index) {
        Some(c) => vec![Op::from(insert_channel::InsertChannel { timeline_index: *timeline_index, index: *index, channel: c.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
