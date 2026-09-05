//! ↩️ Inverse for `InsertRow`.

use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::{SemioTableMutation, remove_row};
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::{SemioTableRow, SemioTableSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::InsertRow, base: &SemioTableSnapshot) -> Vec<SemioTableMutation> {
    let at = payload.index.min(base.rows.len());
    vec![SemioTableMutation::RemoveRow(remove_row::RemoveRow { index: at })]
}
//#endregion 🔖️Inverse
