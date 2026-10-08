//! ↩️ Inverse for `InsertTimeline`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::InsertTimeline, base: &SemioAnimationSnapshot) -> Result<Vec<SemioAnimationMutation>, semio_framework_value::ValueError> {
    use SemioAnimationMutation::*;
    let super::InsertTimeline { index, .. } = payload;
    Ok(vec![RemoveTimeline(remove_timeline::RemoveTimeline { index: *index })])
}
//#endregion 🔖️Inverse
