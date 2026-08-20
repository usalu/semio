//! ↩️ `delete-shell` — reconstructs the removed shell from BASE via `CreateShell`.
//! Missing target ⇒ `Vec::new()`.

use super::mutation::DeleteShell;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{create_shell, SemioBrepMutation};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &DeleteShell, base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
    match base.shells.iter().find(|x| x.id == payload.id) {
        Some(x) => vec![SemioBrepMutation::CreateShell(create_shell::mutation::CreateShell { id: x.id.clone(), faces: x.faces.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
