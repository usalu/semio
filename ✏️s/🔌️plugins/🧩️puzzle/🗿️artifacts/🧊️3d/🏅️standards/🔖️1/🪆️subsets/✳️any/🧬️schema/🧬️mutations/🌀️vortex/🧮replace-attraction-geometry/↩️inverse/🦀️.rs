//! ↩️ Inverse for `ReplaceAttractionGeometry` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use super::mutation::ReplaceAttractionGeometry;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ReplaceAttractionGeometry>>(payload: &super::mutation::ReplaceAttractionGeometry, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(item) = base.attractions.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::replace_attraction_geometry::mutation::replace_attraction_geometry(crate::replace_attraction_geometry::mutation::ReplaceAttractionGeometry { id: item.id.clone(), new_gap: item.gap, new_shift: item.shift, new_rise: item.rise, new_rotation: item.rotation, new_turn: item.turn, new_tilt: item.tilt, new_x: item.x, new_y: item.y })]

    })())
}
//#endregion 🔖️Inverse
