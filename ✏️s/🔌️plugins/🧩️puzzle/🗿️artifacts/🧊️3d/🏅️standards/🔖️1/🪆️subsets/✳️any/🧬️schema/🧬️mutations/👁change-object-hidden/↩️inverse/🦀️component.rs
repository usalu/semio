//! ↩️ Inverse for `ChangeObjectHidden` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::ChangeObjectHidden, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    let Some(item) = base.objects.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::artifacts::puzzle3d::mutations::change_object_hidden::mutation::change_object_hidden(item.id.clone(), item.hidden)]
}
//#endregion 🔖️Inverse
