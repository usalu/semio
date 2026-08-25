//! ↩️ `delete-face` — restores the removed face AT ITS OWN INDEX, not at the end.
//!
//! `create-face` can only APPEND, so a lone `CreateFace` puts the escrowed face back last, which
//! restores the document only when the deleted face WAS last — the case this leaf's committed
//! fixture happens to exercise. Removing index `i` closes the whole index space above it, so the
//! tail is lifted off and re-declared in order (the remedy `🧊️obj`'s `RemoveFace` and `✳️kit`'s
//! `unbind-representation` both needed; ticket 26/08/23/END-TO-END-TESTING-REFACTOR). Shell
//! membership names faces by id and `delete-face` deliberately does not cascade into it, so lifting
//! the tail off and putting it back leaves every `shell.faces` list exactly as it was.
//! Missing target ⇒ `Vec::new()`.

use super::mutation::DeleteFace;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{create_face, delete_face, SemioBrepMutation};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &DeleteFace, base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
    let Some(index) = base.faces.iter().position(|x| x.id == payload.id) else {
        return Vec::new();
    };
    let tail = &base.faces[index..];
    let mut undo: Vec<SemioBrepMutation> = tail
        .iter()
        .skip(1)
        .map(|x| SemioBrepMutation::DeleteFace(delete_face::mutation::DeleteFace { id: x.id.clone() }))
        .collect();
    undo.extend(tail.iter().map(|x| {
        SemioBrepMutation::CreateFace(create_face::mutation::CreateFace { id: x.id.clone(), outer_loop: x.outer_loop.clone(), inner_loops: x.inner_loops.clone(), surface: x.surface.clone(), orientation: x.orientation })
    }));
    undo
}
//#endregion 🔖️Inverse
