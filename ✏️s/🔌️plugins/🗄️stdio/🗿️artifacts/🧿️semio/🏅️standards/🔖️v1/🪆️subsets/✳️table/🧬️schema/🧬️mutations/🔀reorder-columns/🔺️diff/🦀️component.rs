//! 🔺️ `reorder-columns` — sparse diff construction; a column name absent from `base` is
//! `mutation.target-missing`, and `to_index` already matching the column's current position is
//! `mutation.no-op`. Applies the IDENTICAL remove-then-insert (same `from` → `to`) to every row's
//! `cells` — the CRITICAL alignment invariant.

use super::mutation::ReorderColumns;
use crate::artifacts::semio::standards::v1::subsets::table::schema::diff::{SemioTableColumnList, SemioTableDiff, SemioTableRowList};
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ReorderColumns, base: &SemioTableSnapshot) -> protocol::MutationOutcome<SemioTableDiff> {
    let Some(from) = base.columns.iter().position(|c| c.name == payload.name) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Column \"{}\" does not exist.", payload.name), [payload.name.clone()]);
    };
    if from == payload.to_index {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Column \"{}\" is already at position #{}.", payload.name, payload.to_index));
    }
    let mut columns = base.columns.clone();
    let mut rows = base.rows.clone();
    let column = columns.remove(from);
    let to = payload.to_index.min(columns.len());
    columns.insert(to, column);

    for row in &mut rows {
        if from < row.cells.len() {
            let cell = row.cells.remove(from);
            let pos = to.min(row.cells.len());
            row.cells.insert(pos, cell);
        }
    }
    protocol::MutationOutcome::new(SemioTableDiff { columns: Some(SemioTableColumnList { values: columns }), rows: Some(SemioTableRowList { values: rows }) })
}
//#endregion 🔖️Diff
