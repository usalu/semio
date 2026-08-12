//! 🔺️ `create-column` — sparse diff construction. Inserts the new column at `at =
//! index.unwrap_or(columns.len()).min(columns.len())`, then inserts `SemioValue::Null` at the
//! SAME `at` into every row's `cells` — the CRITICAL alignment invariant.

use super::mutation::CreateColumn;
use crate::artifacts::semio::standards::v1::subsets::table::schema::diff::{SemioTableColumnList, SemioTableDiff, SemioTableRowList};
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::{SemioTableColumn, SemioTableSnapshot};
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValue;

//#region 🔖️Diff
pub fn diff(payload: &CreateColumn, base: &SemioTableSnapshot) -> SemioTableDiff {
    let mut columns = base.columns.clone();
    let at = payload.index.unwrap_or(columns.len()).min(columns.len());
    columns.insert(at, SemioTableColumn { name: payload.name.clone(), kind: payload.kind });

    let mut rows = base.rows.clone();
    for row in &mut rows {
        let pos = at.min(row.cells.len());
        row.cells.insert(pos, SemioValue::Null);
    }

    SemioTableDiff { columns: Some(SemioTableColumnList { values: columns }), rows: Some(SemioTableRowList { values: rows }) }
}
//#endregion 🔖️Diff
