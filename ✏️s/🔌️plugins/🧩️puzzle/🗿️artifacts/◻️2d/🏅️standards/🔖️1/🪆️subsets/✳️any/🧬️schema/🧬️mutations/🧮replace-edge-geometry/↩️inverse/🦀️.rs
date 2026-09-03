//! ↩️ Inverse for `ReplaceEdgeGeometry` — restores the BASE connection pose. Missing target ⇒
//! `Vec::new()`.
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ReplaceEdgeGeometry, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
    let Some(edge) = base.edges.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::artifacts::puzzle2d::mutations::replace_edge_geometry::replace_edge_geometry(edge.id.clone(), edge.gap, edge.shift, edge.rise, edge.rotation, edge.turn, edge.tilt, edge.x, edge.y)]
}
//#endregion 🔖️Inverse
