//! 🔺️ Diff for `InsertRow`.

use crate::artifacts::semio::standards::v1::subsets::table::schema::diff::{SemioTableDiff, SemioTableRowList};
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::{SemioTableRow, SemioTableSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertRow, base: &SemioTableSnapshot) -> protocol::MutationOutcome<SemioTableDiff> {
    let mut rows = base.rows.clone();
    let at = payload.index.min(rows.len());
    rows.insert(at, payload.row.clone());
    let outcome = protocol::MutationOutcome::new(SemioTableDiff { columns: None, rows: Some(SemioTableRowList { values: rows }) });
    if at == payload.index {
        outcome
    } else {
        outcome.warn("mutation.clamped", format!("Insert index {} was out of range; inserted at #{} instead.", payload.index, at))
    }
}
//#endregion 🔖️Diff
