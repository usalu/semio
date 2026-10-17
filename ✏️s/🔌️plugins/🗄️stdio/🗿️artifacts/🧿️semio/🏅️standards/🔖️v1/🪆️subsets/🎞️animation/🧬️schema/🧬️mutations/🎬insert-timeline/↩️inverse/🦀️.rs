//! ↩️ Inverse for `InsertTimeline`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioAnimationSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::remove_timeline;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::InsertTimeline, _base: &SemioAnimationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::remove_timeline::RemoveTimeline>,
{
    
    let super::InsertTimeline { index, .. } = payload;
    Ok(vec![Op::from(remove_timeline::RemoveTimeline { index: *index })])
}
//#endregion 🔖️Inverse
