//! ↩️ `delete-edge` — restores the removed edge AT ITS OWN INDEX, not at the end.
//!
//! `create-edge` can only APPEND, so a lone `CreateEdge` puts the escrowed edge back last. That
//! happens to restore the document only when the deleted edge WAS last — which is exactly what this
//! leaf's committed fixture does, so the defect stayed invisible until `mutate-semio-brep`'s subject
//! phase started running (ticket 26/08/23/END-TO-END-TESTING-REFACTOR) and the sibling
//! `delete-vertex` cascade exposed the same shape. Removing index `i` closes the whole index space
//! above it, so the tail is lifted off and re-declared in order — the remedy `🧊️obj`'s `RemoveFace`
//! and `✳️kit`'s `unbind-representation` both needed. Every id in the lift-off is one this very
//! `base` still carries, which matters because `delete-edge` of an absent id is an Error
//! (`mutation.target-missing`), not a no-op. Missing target ⇒ `Vec::new()`.

use super::mutation::DeleteEdge;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{create_edge, delete_edge, SemioBrepMutation};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &DeleteEdge, base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
    let Some(index) = base.edges.iter().position(|x| x.id == payload.id) else {
        return Vec::new();
    };
    let tail = &base.edges[index..];
    let mut undo: Vec<SemioBrepMutation> = tail
        .iter()
        .skip(1)
        .map(|x| SemioBrepMutation::DeleteEdge(delete_edge::mutation::DeleteEdge { id: x.id.clone() }))
        .collect();
    undo.extend(tail.iter().map(|x| {
        SemioBrepMutation::CreateEdge(create_edge::mutation::CreateEdge { id: x.id.clone(), start_vertex: x.start_vertex.clone(), end_vertex: x.end_vertex.clone(), curve: x.curve.clone() })
    }));
    undo
}
//#endregion 🔖️Inverse
