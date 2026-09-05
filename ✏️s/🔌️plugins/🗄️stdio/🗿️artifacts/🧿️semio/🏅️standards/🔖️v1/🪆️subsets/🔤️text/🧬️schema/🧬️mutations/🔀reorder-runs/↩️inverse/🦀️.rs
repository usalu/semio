//! ↩️ Inverse for `ReorderRuns`.

use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::SemioTextMutation;
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::ReorderRuns, base: &SemioTextSnapshot) -> Vec<SemioTextMutation> {
    let len = base.runs.len();
    if len == 0 || payload.from >= len {
        return Vec::new();
    }
    let landed_at = payload.to.min(len - 1);
    vec![SemioTextMutation::ReorderRuns(super::ReorderRuns { from: landed_at, to: payload.from })]
}
//#endregion 🔖️Inverse
