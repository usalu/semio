//! ↩️ `delete-shell` — restores the removed shell AT ITS OWN INDEX, not at the end.
//!
//! `create-shell` can only APPEND, so a lone `CreateShell` puts the escrowed shell back last, which
//! restores the document only when the deleted shell WAS last — the case this leaf's committed
//! fixture happens to exercise. Removing index `i` closes the whole index space above it, so the
//! tail is lifted off and re-declared in order (ticket 26/08/23/END-TO-END-TESTING-REFACTOR). Solid
//! membership names shells by id and `delete-shell` deliberately does not cascade into it, so
//! lifting the tail off and putting it back leaves every `solid.shells` list exactly as it was.
//! Missing target ⇒ `Vec::new()`.

use super::mutation::DeleteShell;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{create_shell, delete_shell, SemioBrepMutation};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &DeleteShell, base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
    let Some(index) = base.shells.iter().position(|x| x.id == payload.id) else {
        return Vec::new();
    };
    let tail = &base.shells[index..];
    let mut undo: Vec<SemioBrepMutation> = tail
        .iter()
        .skip(1)
        .map(|x| SemioBrepMutation::DeleteShell(delete_shell::mutation::DeleteShell { id: x.id.clone() }))
        .collect();
    undo.extend(tail.iter().map(|x| SemioBrepMutation::CreateShell(create_shell::mutation::CreateShell { id: x.id.clone(), faces: x.faces.clone() })));
    undo
}
//#endregion 🔖️Inverse
