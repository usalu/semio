//! ↩️ Inverse for `SetTimelineName`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioAnimationSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::set_timeline_name;
#[allow(unused_imports)]
use crate::schema::mutations::support::timeline_at;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetTimelineName, base: &SemioAnimationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_timeline_name::SetTimelineName>,
{
    
    let super::SetTimelineName { index, .. } = payload;
    Ok(vec![Op::from(set_timeline_name::SetTimelineName { index: *index, name: timeline_at(base, *index).and_then(|t| t.name.clone()) })])
}
//#endregion 🔖️Inverse
