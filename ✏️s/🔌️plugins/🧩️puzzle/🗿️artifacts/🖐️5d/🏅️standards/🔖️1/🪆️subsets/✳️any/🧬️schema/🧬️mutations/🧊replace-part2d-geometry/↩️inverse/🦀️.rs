//! ↩️ Inverse for `ReplacePart2dGeometry` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ReplacePart2dGeometry, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    let Some(item) = base.parts.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::artifacts::puzzle5d::mutations::replace_part_2d_geometry::replace_part_2d_geometry(item.id.clone(), item.part_2d.shape.clone(), item.part_2d.radius, item.part_2d.width, item.part_2d.height)]
}
//#endregion 🔖️Inverse
