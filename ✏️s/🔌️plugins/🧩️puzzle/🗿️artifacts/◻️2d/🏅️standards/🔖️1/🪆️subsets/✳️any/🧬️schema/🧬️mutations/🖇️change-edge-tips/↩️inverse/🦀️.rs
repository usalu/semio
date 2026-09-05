//! ↩️ Inverse for `ChangeEdgeTips` — restores the BASE field value on the addressed edge. Missing
//! target ⇒ `Vec::new()`.
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeEdgeTips, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
    let Some(edge) = base.edges.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::artifacts::puzzle2d::mutations::change_edge_tips::change_edge_tips(edge.id.clone(), edge.source_tip.clone(), edge.target_tip.clone())]
}
//#endregion 🔖️Inverse
