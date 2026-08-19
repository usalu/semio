//! ↩️ Inverse for `ChangeEdgeVisible` — restores the BASE field value on the addressed edge. Missing
//! target ⇒ `Vec::new()`.
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::ChangeEdgeVisible, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
    let Some(edge) = base.edges.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::artifacts::puzzle2d::mutations::change_edge_visible::mutation::change_edge_visible(edge.id.clone(), edge.visible)]
}
//#endregion 🔖️Inverse
