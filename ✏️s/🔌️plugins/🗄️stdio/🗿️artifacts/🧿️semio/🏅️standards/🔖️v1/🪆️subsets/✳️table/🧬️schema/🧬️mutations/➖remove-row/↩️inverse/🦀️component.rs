//! ↩️ `remove-row` — undo re-inserts the captured row at the same BASE-state index; out-of-range
//! BASE index ⇒ `Vec::new()` (nothing was removed, nothing to undo).

use super::mutation::RemoveRow;
use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::insert_row;
use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::SemioTableMutation;
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &RemoveRow, base: &SemioTableSnapshot) -> Vec<SemioTableMutation> {
    match base.rows.get(payload.index) {
        Some(row) => vec![SemioTableMutation::InsertRow(insert_row::mutation::InsertRow { index: payload.index, row: row.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
