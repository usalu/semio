//! ↩️ Inverse for `ChangeRunLanguage`.

use crate::schema::mutations::SemioTextMutation;
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::ChangeRunLanguage, base: &SemioTextSnapshot) -> Result<Vec<SemioTextMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.runs.get(payload.index) {
        Some(run) => vec![SemioTextMutation::ChangeRunLanguage(super::ChangeRunLanguage { index: payload.index, new_language: run.language.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
