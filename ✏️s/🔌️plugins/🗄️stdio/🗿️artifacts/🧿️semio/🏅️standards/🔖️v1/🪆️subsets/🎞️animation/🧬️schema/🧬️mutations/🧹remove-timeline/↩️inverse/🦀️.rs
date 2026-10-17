//! ↩️ Inverse for `RemoveTimeline`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioAnimationSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::insert_timeline;
#[allow(unused_imports)]
use crate::schema::mutations::support::timeline_at;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::RemoveTimeline, base: &SemioAnimationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::insert_timeline::InsertTimeline>,
{
    
    let super::RemoveTimeline { index } = payload;
    Ok(match timeline_at(base, *index) {
        Some(t) => vec![Op::from(insert_timeline::InsertTimeline { index: *index, timeline: t.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
