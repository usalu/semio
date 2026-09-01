//! ↩️ Inverse for `InsertRun`.

use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::{SemioTextMutation, remove_run};
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::{SemioTextRun, SemioTextSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::InsertRun, base: &SemioTextSnapshot) -> Vec<SemioTextMutation> {
    let at = payload.index.min(base.runs.len());
    vec![SemioTextMutation::RemoveRun(remove_run::RemoveRun { index: at })]
}
//#endregion 🔖️Inverse
