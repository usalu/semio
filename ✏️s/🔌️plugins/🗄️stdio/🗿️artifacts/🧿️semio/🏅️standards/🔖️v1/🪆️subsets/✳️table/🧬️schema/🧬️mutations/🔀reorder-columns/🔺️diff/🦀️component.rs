//! 🔺️ `reorder-columns` — sparse diff construction; a column name absent from `base` is a no-op
//! clone. Applies the IDENTICAL remove-then-insert (same `from` → `to`) to every row's `cells` —
//! the CRITICAL alignment invariant.

use super::mutation::ReorderColumns;
use crate::artifacts::semio::standards::v1::subsets::table::schema::diff::{SemioTableColumnList, SemioTableDiff, SemioTableRowList};
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReorderColumns, base: &SemioTableSnapshot) -> SemioTableDiff {
    let mut columns = base.columns.clone();
    let mut rows = base.rows.clone();
    if let Some(from) = columns.iter().position(|c| c.name == payload.name) {
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
    }
    SemioTableDiff { columns: Some(SemioTableColumnList { values: columns }), rows: Some(SemioTableRowList { values: rows }) }
}
//#endregion 🔖️Diff
