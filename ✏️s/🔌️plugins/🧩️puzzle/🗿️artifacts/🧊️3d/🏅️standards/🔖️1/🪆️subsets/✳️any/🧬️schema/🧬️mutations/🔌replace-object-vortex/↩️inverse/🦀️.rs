//! ↩️ Inverse for `ReplaceObjectVortex` — restores the BASE vortex payload. Missing target ⇒
//! `Vec::new()`.
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ReplaceObjectVortex, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    let Some(object) = base.objects.iter().find(|entry| entry.id == payload.object_id) else {
        return Vec::new();
    };
    let Some(vortex) = object.vortices.iter().find(|vortex| vortex.id == payload.vortex_id) else {
        return Vec::new();
    };
    vec![crate::artifacts::puzzle3d::mutations::replace_object_vortex::mutation::replace_object_vortex(payload.object_id.clone(), payload.vortex_id.clone(), vortex.clone())]
}
//#endregion 🔖️Inverse
