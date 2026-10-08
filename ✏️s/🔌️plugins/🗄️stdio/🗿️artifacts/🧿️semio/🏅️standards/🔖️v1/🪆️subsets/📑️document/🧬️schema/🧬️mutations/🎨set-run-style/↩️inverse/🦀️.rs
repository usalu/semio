//! ↩️ Inverse for `SetRunStyle`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetRunStyle, base: &SemioDocumentSnapshot) -> Result<Vec<SemioDocumentMutation>, semio_framework_value::ValueError> {
    let super::SetRunStyle { path, run_index, .. } = payload;
    Ok(match block_at(base, path).and_then(runs_of).and_then(|r| r.get(*run_index)) {
        Some(run) => vec![SemioDocumentMutation::SetRunStyle(set_run_style::SetRunStyle { path: path.clone(), run_index: *run_index, style: run.style.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
