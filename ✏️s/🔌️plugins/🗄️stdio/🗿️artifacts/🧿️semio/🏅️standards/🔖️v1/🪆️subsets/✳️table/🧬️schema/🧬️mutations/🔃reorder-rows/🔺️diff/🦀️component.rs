//! 🔺️ `reorder-rows` — sparse diff construction; an out-of-range BASE `from` is
//! `mutation.target-missing`, and `from == to` (already in place) is `mutation.no-op`. Mirrors
//! `✳️text`'s own `reorder-runs` diff exactly.

use super::mutation::ReorderRows;
use crate::artifacts::semio::standards::v1::subsets::table::schema::diff::{SemioTableDiff, SemioTableRowList};
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &ReorderRows, base: &SemioTableSnapshot) -> protocol::MutationOutcome<SemioTableDiff> {
    if payload.from >= base.rows.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Row #{} does not exist.", payload.from), [payload.from.to_string()]);
    }
    if payload.from == payload.to {
        return protocol::MutationOutcome::empty().await.warn("mutation.no-op", format!("Row #{} is already at position #{}.", payload.from, payload.to));
    }
    let mut rows = base.rows.clone();
    let item = rows.remove(payload.from);
    let at = payload.to.min(rows.len());
    rows.insert(at, item);
    protocol::MutationOutcome::new(SemioTableDiff { columns: None, rows: Some(SemioTableRowList { values: rows }) })
}
//#endregion 🔖️Diff
