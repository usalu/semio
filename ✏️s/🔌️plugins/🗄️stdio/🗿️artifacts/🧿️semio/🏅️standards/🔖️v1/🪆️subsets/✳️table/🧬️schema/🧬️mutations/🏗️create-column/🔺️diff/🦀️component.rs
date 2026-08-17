//! 🔺️ `create-column` — sparse diff construction. Fatal `mutation.duplicate-id` when a column
//! named `name` already exists (name is the native key, see `📸️snapshot/🦀️component.rs`).
//! Otherwise inserts the new column at `at = index.unwrap_or(columns.len()).min(columns.len())`,
//! then inserts `SemioValue::Null` at the SAME `at` into every row's `cells` — the CRITICAL
//! alignment invariant.

use super::mutation::CreateColumn;
use crate::artifacts::semio::standards::v1::subsets::table::schema::diff::{SemioTableColumnList, SemioTableDiff, SemioTableRowList};
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::{SemioTableColumn, SemioTableSnapshot};
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValue;

//#region 🔖️Diff
pub fn diff(payload: &CreateColumn, base: &SemioTableSnapshot) -> protocol::MutationOutcome<SemioTableDiff> {
    if base.columns.iter().any(|c| c.name == payload.name) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A column named \"{}\" already exists.", payload.name), [payload.name.clone()]);
    }
    let mut columns = base.columns.clone();
    let at = payload.index.unwrap_or(columns.len()).min(columns.len());
    columns.insert(at, SemioTableColumn { name: payload.name.clone(), kind: payload.kind });

    let mut rows = base.rows.clone();
    for row in &mut rows {
        let pos = at.min(row.cells.len());
        row.cells.insert(pos, SemioValue::Null);
    }

    protocol::MutationOutcome::new(SemioTableDiff { columns: Some(SemioTableColumnList { values: columns }), rows: Some(SemioTableRowList { values: rows }) })
}
//#endregion 🔖️Diff
