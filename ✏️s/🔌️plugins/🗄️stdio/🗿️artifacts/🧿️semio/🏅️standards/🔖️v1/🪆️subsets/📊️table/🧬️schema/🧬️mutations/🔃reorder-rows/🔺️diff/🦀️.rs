//! 🔺️ Diff for `ReorderRows`.

use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff, Replace};
use crate::standards::v1::subsets::table::schema::diff::{SemioTableColumnDiff, SemioTableDiff, SemioTableRowDiff};
use crate::standards::v1::subsets::table::schema::snapshot::{SemioTableColumn, SemioTableSnapshot};
use crate::standards::v1::subsets::value::schema::snapshot::SemioValue;

//#region 🔖️Diff
/// 🧮️ Moves one row: the `rows` triple removes its base index and re-adds the same row at the landing position.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::ReorderRows, base: &SemioTableSnapshot) -> protocol::MutationOutcome<SemioTableDiff> {
    let Some(row) = base.rows.get(payload.from) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Row #{} does not exist.", payload.from), [payload.from.to_string()]);
    };
    if payload.from == payload.to {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Row #{} is already at position #{}.", payload.from, payload.to));
    }
    let at = payload.to.min(base.rows.len() - 1);
    protocol::MutationOutcome::new(SemioTableDiff { columns: None, rows: Some(IndexedTripleDiff { removed: vec![payload.from], added: vec![IndexAdded { index: at, item: row.clone() }], ..Default::default() }) })
}
//#endregion 🔖️Diff
