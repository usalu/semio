//! ↩️ Inverse for `ReplaceFastenerGeometry` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use crate::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ReplaceFastenerGeometry, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    let Some(item) = base.fasteners.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::mutations::replace_fastener_semio_framework_geometry::replace_fastener_geometry(crate::mutations::ReplaceFastenerGeometry { id: item.id.clone(), new_gap: item.gap, new_shift: item.shift, new_rise: item.rise, new_rotation: item.rotation, new_turn: item.turn, new_tilt: item.tilt, new_x: item.x, new_y: item.y })]
}
//#endregion 🔖️Inverse
