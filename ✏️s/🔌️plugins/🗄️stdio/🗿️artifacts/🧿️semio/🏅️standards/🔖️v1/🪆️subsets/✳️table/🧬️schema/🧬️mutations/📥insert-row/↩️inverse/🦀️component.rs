//! ↩️ `insert-row` — undo is `remove-row` at the (clamped) FINAL-state index the row landed at,
//! which is also a valid BASE-state index for the follow-up removal.

use super::mutation::InsertRow;
use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::remove_row;
use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::SemioTableMutation;
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &InsertRow, base: &SemioTableSnapshot) -> Vec<SemioTableMutation> {
    let at = payload.index.min(base.rows.len());
    vec![SemioTableMutation::RemoveRow(remove_row::mutation::RemoveRow { index: at })]
}
//#endregion 🔖️Inverse
