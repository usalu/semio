//! ↩️ Inverse for `CreateColumn`.

use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::{SemioTableMutation, delete_column};
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::{SemioTableCellKind, SemioTableColumn, SemioTableSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateColumn, _base: &SemioTableSnapshot) -> Vec<SemioTableMutation> {
    vec![SemioTableMutation::DeleteColumn(delete_column::DeleteColumn { name: payload.name.clone() })]
}
//#endregion 🔖️Inverse
