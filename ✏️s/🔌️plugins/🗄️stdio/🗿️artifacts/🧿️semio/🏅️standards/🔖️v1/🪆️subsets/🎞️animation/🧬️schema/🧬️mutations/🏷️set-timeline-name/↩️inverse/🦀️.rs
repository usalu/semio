//! ↩️ Inverse for `SetTimelineName`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetTimelineName, base: &SemioAnimationSnapshot) -> Result<Vec<SemioAnimationMutation>, semio_framework_value::ValueError> {
    use SemioAnimationMutation::*;
    let super::SetTimelineName { index, .. } = payload;
    Ok(vec![SetTimelineName(set_timeline_name::SetTimelineName { index: *index, name: timeline_at(base, *index).and_then(|t| t.name.clone()) })])
}
//#endregion 🔖️Inverse
