//! ↩️ Inverse for `MoveObject` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::MoveObject, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    let Some(item) = base.objects.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::artifacts::puzzle3d::mutations::move_object::mutation::move_object(item.id.clone(), item.origin)]
}
//#endregion 🔖️Inverse
