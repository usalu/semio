//! 🔺️ `remove-row` — sparse diff construction; an out-of-range BASE index is a no-op clone.

use super::mutation::RemoveRow;
use crate::artifacts::semio::standards::v1::subsets::table::schema::diff::{SemioTableDiff, SemioTableRowList};
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &RemoveRow, base: &SemioTableSnapshot) -> SemioTableDiff {
    let mut rows = base.rows.clone();
    if payload.index < rows.len() {
        rows.remove(payload.index);
    }
    SemioTableDiff { columns: None, rows: Some(SemioTableRowList { values: rows }) }
}
//#endregion 🔖️Diff
