//! ↩️ Inverse for `DisconnectHandles` — reconstructs a `connect-handles` of the captured BASE
//! edge. Missing target ⇒ `Vec::new()`.
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DisconnectHandles, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
    let Some(edge) = base.edges.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::artifacts::puzzle2d::mutations::connect_handles::mutation::connect_handles(
        edge.id.clone(),
        edge.source.clone(),
        edge.target.clone(),
        edge.edge_kind.clone(),
        edge.gap,
        edge.shift,
        edge.rise,
        edge.rotation,
        edge.turn,
        edge.tilt,
        edge.x,
        edge.y,
        edge.source_tip.clone(),
        edge.target_tip.clone(),
    )]
}
//#endregion 🔖️Inverse
