//! ↩️ Inverse for `RemoveRow`.

use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::{SemioTableMutation, insert_row};
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveRow, base: &SemioTableSnapshot) -> Vec<SemioTableMutation> {
    match base.rows.get(payload.index) {
        Some(row) => vec![SemioTableMutation::InsertRow(insert_row::InsertRow { index: payload.index, row: row.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
