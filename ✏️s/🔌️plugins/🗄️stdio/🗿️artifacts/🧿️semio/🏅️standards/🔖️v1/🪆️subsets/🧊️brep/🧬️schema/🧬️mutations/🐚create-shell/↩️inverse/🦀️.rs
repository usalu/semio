//! ↩️ Inverse for `CreateShell`.

use crate::standards::v1::subsets::brep::schema::mutations::{delete_shell, SemioBrepMutation};
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateShell, _base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
    vec![SemioBrepMutation::DeleteShell(delete_shell::DeleteShell { id: payload.id.clone() })]
}
//#endregion 🔖️Inverse
