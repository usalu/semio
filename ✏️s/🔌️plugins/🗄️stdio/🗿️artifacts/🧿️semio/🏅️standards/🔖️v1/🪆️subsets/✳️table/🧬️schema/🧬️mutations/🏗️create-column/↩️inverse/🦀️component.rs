//! ↩️ `create-column` — undo is `delete-column` by name; `delete-column`'s own inverse recaptures
//! the full per-row cascade, so this side stays a plain single-mutation undo.

use super::mutation::CreateColumn;
use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::delete_column;
use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::SemioTableMutation;
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &CreateColumn, _base: &SemioTableSnapshot) -> Vec<SemioTableMutation> {
    vec![SemioTableMutation::DeleteColumn(delete_column::mutation::DeleteColumn { name: payload.name.clone() })]
}
//#endregion 🔖️Inverse
