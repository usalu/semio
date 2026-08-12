//! ↩️ Inverse for `ChangePart3dMesh` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ChangePart3dMesh, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    let Some(item) = base.parts.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::artifacts::puzzle5d::mutations::change_part_3d_mesh::mutation::change_part_3d_mesh(item.id.clone(), item.part_3d.mesh_url.clone())]
}
//#endregion 🔖️Inverse
