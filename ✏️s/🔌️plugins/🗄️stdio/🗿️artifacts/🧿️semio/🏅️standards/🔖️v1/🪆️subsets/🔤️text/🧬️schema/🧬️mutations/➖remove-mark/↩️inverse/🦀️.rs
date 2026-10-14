//! ↩️ Inverse for `RemoveMark`.

use crate::schema::mutations::{add_mark, SemioTextMutation};
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveMark, base: &SemioTextSnapshot) -> Result<Vec<SemioTextMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.runs.get(payload.run_index).and_then(|run| run.marks.get(payload.index)) {
        Some(mark) => vec![SemioTextMutation::AddMark(add_mark::AddMark { run_index: payload.run_index, index: payload.index, mark: mark.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
