//! 🔺️ `reorder-rows` — sparse diff construction; an out-of-range BASE `from` is a no-op clone.
//! Mirrors `✳️text`'s own `reorder-runs` diff exactly.

use super::mutation::ReorderRows;
use crate::artifacts::semio::standards::v1::subsets::table::schema::diff::{SemioTableDiff, SemioTableRowList};
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReorderRows, base: &SemioTableSnapshot) -> SemioTableDiff {
    let mut rows = base.rows.clone();
    if payload.from < rows.len() {
        let item = rows.remove(payload.from);
        let at = payload.to.min(rows.len());
        rows.insert(at, item);
    }
    SemioTableDiff { columns: None, rows: Some(SemioTableRowList { values: rows }) }
}
//#endregion 🔖️Diff
