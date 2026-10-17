//! ↩️ Inverse for `SetChannelTarget`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioAnimationSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::channel_at;
#[allow(unused_imports)]
use crate::schema::mutations::set_channel_target;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetChannelTarget, base: &SemioAnimationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_channel_target::SetChannelTarget>,
{
    
    let super::SetChannelTarget { timeline_index, index, .. } = payload;
    Ok(match channel_at(base, *timeline_index, *index) {
        Some(c) => vec![Op::from(set_channel_target::SetChannelTarget { timeline_index: *timeline_index, index: *index, target: c.target.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
