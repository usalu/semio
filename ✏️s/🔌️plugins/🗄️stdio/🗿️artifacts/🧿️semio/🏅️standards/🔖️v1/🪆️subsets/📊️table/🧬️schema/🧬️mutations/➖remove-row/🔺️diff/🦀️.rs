//! 🔺️ Diff for `RemoveRow`.

use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff, Replace};
use crate::standards::v1::subsets::table::schema::diff::{SemioTableColumnDiff, SemioTableDiff, SemioTableRowDiff};
use crate::standards::v1::subsets::table::schema::snapshot::{SemioTableColumn, SemioTableSnapshot};
use crate::standards::v1::subsets::value::schema::snapshot::SemioValue;

//#region 🔖️Diff
/// 🧮️ Removes the row at `index`: the `rows` triple names exactly that removal.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveRow, base: &SemioTableSnapshot) -> protocol::MutationOutcome<SemioTableDiff> {
    if payload.index >= base.rows.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Row #{} does not exist.", payload.index), [payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(SemioTableDiff { columns: None, rows: Some(IndexedTripleDiff { removed: vec![payload.index], ..Default::default() }) })
}
//#endregion 🔖️Diff
