//! ↩️ Inverse for `ReplaceFastenerGeometry` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ReplaceFastenerGeometry, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    let Some(item) = base.fasteners.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::artifacts::puzzle5d::mutations::replace_fastener_geometry::mutation::replace_fastener_geometry(item.id.clone(), item.gap, item.shift, item.rise, item.rotation, item.turn, item.tilt, item.x, item.y)]
}
//#endregion 🔖️Inverse
