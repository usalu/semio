//! ↩️ Inverse for `EditPart3dLabel` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::EditPart3dLabel, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    let Some(item) = base.parts.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::artifacts::puzzle5d::mutations::edit_part_3d_label::mutation::edit_part_3d_label(item.id.clone(), item.part_3d.label.clone())]
}
//#endregion 🔖️Inverse
