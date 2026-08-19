//! 🔺️ `remove-row` — sparse diff construction; Error `mutation.target-missing` when the BASE
//! index is out of range.

use super::mutation::RemoveRow;
use crate::artifacts::semio::standards::v1::subsets::table::schema::diff::{SemioTableDiff, SemioTableRowList};
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &RemoveRow, base: &SemioTableSnapshot) -> protocol::MutationOutcome<SemioTableDiff> {
    if payload.index >= base.rows.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Row #{} does not exist.", payload.index), [payload.index.to_string()]);
    }
    let mut rows = base.rows.clone();
    rows.remove(payload.index);
    protocol::MutationOutcome::new(SemioTableDiff { columns: None, rows: Some(SemioTableRowList { values: rows }) })
}
//#endregion 🔖️Diff
