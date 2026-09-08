//! ↩️ Inverse for `EditRun`.

use crate::standards::v1::subsets::text::schema::mutations::SemioTextMutation;
use crate::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::EditRun, base: &SemioTextSnapshot) -> Vec<SemioTextMutation> {
    match base.runs.get(payload.index) {
        Some(run) => vec![SemioTextMutation::EditRun(super::EditRun { index: payload.index, new_content: run.content.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
