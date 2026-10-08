//! ↩️ Inverse for `RemoveTimeline`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveTimeline, base: &SemioAnimationSnapshot) -> Result<Vec<SemioAnimationMutation>, semio_framework_value::ValueError> {
    use SemioAnimationMutation::*;
    let super::RemoveTimeline { index } = payload;
    Ok(match timeline_at(base, *index) {
        Some(t) => vec![InsertTimeline(insert_timeline::InsertTimeline { index: *index, timeline: t.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
