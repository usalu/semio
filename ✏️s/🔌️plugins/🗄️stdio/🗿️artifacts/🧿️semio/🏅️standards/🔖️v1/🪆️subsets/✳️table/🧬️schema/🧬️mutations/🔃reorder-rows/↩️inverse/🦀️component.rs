//! ↩️ `reorder-rows` — undo moves the row back: `reorder{from: min(to, len-1), to: from}`
//! (`📓️taxonomy.md`'s addressing convention #3); out-of-range BASE `from` ⇒ `Vec::new()`. Mirrors
//! `✳️text`'s own `reorder_runs::inverse` exactly (same clamping).

use super::mutation::ReorderRows;
use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::SemioTableMutation;
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &ReorderRows, base: &SemioTableSnapshot) -> Vec<SemioTableMutation> {
    let len = base.rows.len();
    if len == 0 || payload.from >= len {
        return Vec::new();
    }
    let landed_at = payload.to.min(len - 1);
    vec![SemioTableMutation::ReorderRows(ReorderRows { from: landed_at, to: payload.from })]
}
//#endregion 🔖️Inverse
