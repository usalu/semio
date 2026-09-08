//! ↩️ Inverse for `ScalePart3d` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use crate::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ScalePart3d, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    let Some(item) = base.parts.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::mutations::scale_part_3d::scale_part_3d(item.id.clone(), item.part_3d.scale)]
}
//#endregion 🔖️Inverse
