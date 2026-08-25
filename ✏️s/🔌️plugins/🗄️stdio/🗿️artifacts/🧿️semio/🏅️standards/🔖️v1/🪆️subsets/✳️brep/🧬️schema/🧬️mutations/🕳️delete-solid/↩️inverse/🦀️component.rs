//! ↩️ `delete-solid` — restores the removed solid AT ITS OWN INDEX, not at the end.
//!
//! `create-solid` can only APPEND, so a lone `CreateSolid` puts the escrowed solid back last, which
//! restores the document only when the deleted solid WAS last — the case this leaf's committed
//! fixture happens to exercise. Removing index `i` closes the whole index space above it, so the
//! tail is lifted off and re-declared in order (ticket 26/08/23/END-TO-END-TESTING-REFACTOR).
//! Nothing in this subset references a solid, so the lift-off disturbs no membership list.
//! Missing target ⇒ `Vec::new()`.

use super::mutation::DeleteSolid;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{create_solid, delete_solid, SemioBrepMutation};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &DeleteSolid, base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
    let Some(index) = base.solids.iter().position(|x| x.id == payload.id) else {
        return Vec::new();
    };
    let tail = &base.solids[index..];
    let mut undo: Vec<SemioBrepMutation> = tail
        .iter()
        .skip(1)
        .map(|x| SemioBrepMutation::DeleteSolid(delete_solid::mutation::DeleteSolid { id: x.id.clone() }))
        .collect();
    undo.extend(tail.iter().map(|x| SemioBrepMutation::CreateSolid(create_solid::mutation::CreateSolid { id: x.id.clone(), shells: x.shells.clone() })));
    undo
}
//#endregion 🔖️Inverse
