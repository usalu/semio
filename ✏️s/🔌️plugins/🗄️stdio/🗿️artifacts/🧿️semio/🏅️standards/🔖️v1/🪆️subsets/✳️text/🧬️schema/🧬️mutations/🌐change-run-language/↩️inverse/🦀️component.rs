//! ↩️ `change-run-language` — undo restores BASE's language at that index; out-of-range BASE
//! index ⇒ `Vec::new()`.

use super::mutation::ChangeRunLanguage;
use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::SemioTextMutation;
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &ChangeRunLanguage, base: &SemioTextSnapshot) -> Vec<SemioTextMutation> {
    match base.runs.get(payload.index) {
        Some(run) => vec![SemioTextMutation::ChangeRunLanguage(ChangeRunLanguage { index: payload.index, new_language: run.language.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
