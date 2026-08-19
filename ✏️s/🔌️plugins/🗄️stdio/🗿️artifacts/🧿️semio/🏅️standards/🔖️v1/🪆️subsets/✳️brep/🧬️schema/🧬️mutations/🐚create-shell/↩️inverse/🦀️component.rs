//! ↩️ `create-shell` — undo is `deleteshell` (`delete_shell`) at the same id.

use super::mutation::CreateShell;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{delete_shell, SemioBrepMutation};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &CreateShell, _base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
    vec![SemioBrepMutation::DeleteShell(delete_shell::mutation::DeleteShell { id: payload.id.clone() })]
}
//#endregion 🔖️Inverse
