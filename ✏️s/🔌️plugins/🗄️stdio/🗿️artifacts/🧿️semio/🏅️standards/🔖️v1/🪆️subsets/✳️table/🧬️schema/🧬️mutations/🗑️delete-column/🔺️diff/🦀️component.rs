//! 🔺️ `delete-column` — sparse diff construction; a column name absent from `base` is a no-op
//! clone (columns/rows unchanged).

use super::mutation::DeleteColumn;
use crate::artifacts::semio::standards::v1::subsets::table::schema::diff::{SemioTableColumnList, SemioTableDiff, SemioTableRowList};
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteColumn, base: &SemioTableSnapshot) -> SemioTableDiff {
    let mut columns = base.columns.clone();
    let mut rows = base.rows.clone();
    if let Some(at) = columns.iter().position(|c| c.name == payload.name) {
        columns.remove(at);
        for row in &mut rows {
            if at < row.cells.len() {
                row.cells.remove(at);
            }
        }
    }
    SemioTableDiff { columns: Some(SemioTableColumnList { values: columns }), rows: Some(SemioTableRowList { values: rows }) }
}
//#endregion 🔖️Diff
