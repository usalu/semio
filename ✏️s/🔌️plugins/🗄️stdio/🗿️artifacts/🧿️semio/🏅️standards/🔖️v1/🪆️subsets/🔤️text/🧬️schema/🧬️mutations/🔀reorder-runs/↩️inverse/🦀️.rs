//! ↩️ Inverse for `ReorderRuns`.

use crate::schema::mutations::SemioTextMutation;
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::ReorderRuns, base: &SemioTextSnapshot) -> Result<Vec<SemioTextMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let len = base.runs.len();
    if len == 0 || payload.from >= len {
        return Vec::new();
    }
    let landed_at = payload.to.min(len - 1);
    vec![SemioTextMutation::ReorderRuns(super::ReorderRuns { from: landed_at, to: payload.from })]

    })())
}
//#endregion 🔖️Inverse
