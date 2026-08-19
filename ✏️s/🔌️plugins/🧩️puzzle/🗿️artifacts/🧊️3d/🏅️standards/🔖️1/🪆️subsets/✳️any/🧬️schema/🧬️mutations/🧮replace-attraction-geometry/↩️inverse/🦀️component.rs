//! ↩️ Inverse for `ReplaceAttractionGeometry` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::ReplaceAttractionGeometry, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    let Some(item) = base.attractions.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::artifacts::puzzle3d::mutations::replace_attraction_geometry::mutation::replace_attraction_geometry(item.id.clone(), item.gap, item.shift, item.rise, item.rotation, item.turn, item.tilt, item.x, item.y)]
}
//#endregion 🔖️Inverse
