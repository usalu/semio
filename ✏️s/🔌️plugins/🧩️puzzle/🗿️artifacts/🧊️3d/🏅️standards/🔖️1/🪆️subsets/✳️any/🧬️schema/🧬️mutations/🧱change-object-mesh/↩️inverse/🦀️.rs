//! ↩️ Inverse for `ChangeObjectMesh` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use crate::mutations::Puzzle3dMutation;
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ChangeObjectMesh, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    let Some(item) = base.objects.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::mutations::change_object_mesh::mutation::change_object_mesh(item.id.clone(), item.mesh_url.clone())]
}
//#endregion 🔖️Inverse
