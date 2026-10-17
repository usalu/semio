//! ↩️ Inverse for `SetChannelInterpolation`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioAnimationSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::channel_at;
#[allow(unused_imports)]
use crate::schema::mutations::set_channel_interpolation;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetChannelInterpolation, base: &SemioAnimationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_channel_interpolation::SetChannelInterpolation>,
{
    
    let super::SetChannelInterpolation { timeline_index, index, .. } = payload;
    Ok(match channel_at(base, *timeline_index, *index) {
        Some(c) => vec![Op::from(set_channel_interpolation::SetChannelInterpolation { timeline_index: *timeline_index, index: *index, interpolation: c.interpolation })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
