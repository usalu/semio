//! ↩️ Inverse for `InsertRun`.

use crate::schema::mutations::{remove_run, SemioTextMutation};
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::InsertRun, base: &SemioTextSnapshot) -> Result<Vec<SemioTextMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let at = payload.index.min(base.runs.len());
    vec![SemioTextMutation::RemoveRun(remove_run::RemoveRun { index: at })]

    })())
}
//#endregion 🔖️Inverse
