//! ↩️ Inverse for `DeleteEdge` — reconstructs the removed edge from BASE. Missing target ⇒
//! `Vec::new()`.
use crate::mutations::{create_edge, TrinityGraphMutation};
use crate::JackSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteEdge, base: &JackSnapshot) -> Vec<TrinityGraphMutation> {
    base.edges().iter().find(|edge| edge.id == payload.id).map(|edge| vec![create_edge(edge.clone())]).unwrap_or_default()
}
//#endregion 🔖️Inverse
