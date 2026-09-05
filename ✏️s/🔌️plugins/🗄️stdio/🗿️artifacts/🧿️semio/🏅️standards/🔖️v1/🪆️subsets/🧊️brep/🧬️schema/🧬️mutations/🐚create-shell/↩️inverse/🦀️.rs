//! ↩️ Inverse for `CreateShell`.

use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{SemioBrepMutation, delete_shell};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{BrepShell, BrepShellFace, SemioBrepSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateShell, _base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
    vec![SemioBrepMutation::DeleteShell(delete_shell::DeleteShell { id: payload.id.clone() })]
}
//#endregion 🔖️Inverse
