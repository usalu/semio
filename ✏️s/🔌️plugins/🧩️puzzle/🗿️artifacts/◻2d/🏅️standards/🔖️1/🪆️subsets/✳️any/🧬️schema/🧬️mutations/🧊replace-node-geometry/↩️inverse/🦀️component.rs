//! ↩️ Inverse for `ReplaceNodeGeometry` — restores the BASE shape/extent. Missing target ⇒ `Vec::new()`.
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::ReplaceNodeGeometry, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::artifacts::puzzle2d::mutations::replace_node_geometry::mutation::replace_node_geometry(node.id.clone(), node.shape.clone(), node.radius, node.width, node.height)]
}
//#endregion 🔖️Inverse
