//! 🔺️ `insert-row` — sparse diff construction. `SemioTableDiff::rows` is a whole-list-per-diff
//! wrapper (`SemioTableRowList`), not a sparse triple — every row mutation rebuilds the full
//! ordered `values` vec from `base` and wraps it.

use super::mutation::InsertRow;
use crate::artifacts::semio::standards::v1::subsets::table::schema::diff::{SemioTableDiff, SemioTableRowList};
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &InsertRow, base: &SemioTableSnapshot) -> SemioTableDiff {
    let mut rows = base.rows.clone();
    let at = payload.index.min(rows.len());
    rows.insert(at, payload.row.clone());
    SemioTableDiff { columns: None, rows: Some(SemioTableRowList { values: rows }) }
}
//#endregion 🔖️Diff
