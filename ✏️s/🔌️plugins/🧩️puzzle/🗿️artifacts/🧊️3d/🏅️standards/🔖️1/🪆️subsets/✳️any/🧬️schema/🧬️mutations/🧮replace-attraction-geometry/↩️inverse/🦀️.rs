//! ↩️ Inverse for `ReplaceAttractionGeometry` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation;
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ReplaceAttractionGeometry, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    let Some(item) = base.attractions.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::replace_attraction_geometry::mutation::replace_attraction_geometry(crate::standards::v1::subsets::any::schema::mutations::ReplaceAttractionGeometry { id: item.id.clone(), new_gap: item.gap, new_shift: item.shift, new_rise: item.rise, new_rotation: item.rotation, new_turn: item.turn, new_tilt: item.tilt, new_x: item.x, new_y: item.y })]
}
//#endregion 🔖️Inverse
