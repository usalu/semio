//! ↩️ Inverse for `SetRunText`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetRunText, base: &SemioDocumentSnapshot) -> Result<Vec<SemioDocumentMutation>, semio_framework_value::ValueError> {
    let super::SetRunText { path, run_index, .. } = payload;
    Ok(match block_at(base, path).and_then(runs_of).and_then(|r| r.get(*run_index)) {
        Some(run) => vec![SemioDocumentMutation::SetRunText(set_run_text::SetRunText { path: path.clone(), run_index: *run_index, text: run.text.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
