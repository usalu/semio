//! ↩️ `reorder-runs` — undo moves the run back: `reorder{from: min(to, len-1), to: from}`
//! (`📓️taxonomy.md`'s addressing convention #3); out-of-range BASE `from` ⇒ `Vec::new()`.

use super::mutation::ReorderRuns;
use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::SemioTextMutation;
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &ReorderRuns, base: &SemioTextSnapshot) -> Vec<SemioTextMutation> {
    let len = base.runs.len();
    if len == 0 || payload.from >= len {
        return Vec::new();
    }
    let landed_at = payload.to.min(len - 1);
    vec![SemioTextMutation::ReorderRuns(ReorderRuns { from: landed_at, to: payload.from })]
}
//#endregion 🔖️Inverse
