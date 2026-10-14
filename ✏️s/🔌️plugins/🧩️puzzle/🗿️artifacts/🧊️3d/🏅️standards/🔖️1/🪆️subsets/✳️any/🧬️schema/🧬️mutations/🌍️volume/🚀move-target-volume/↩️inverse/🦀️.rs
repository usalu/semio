//! ↩️ Inverse for `MoveTargetVolume` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use super::mutation::MoveTargetVolume;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<MoveTargetVolume>>(payload: &super::mutation::MoveTargetVolume, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(item) = base.target_volumes.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::move_target_volume::mutation::move_target_volume(item.id.clone(), item.origin)]

    })())
}
//#endregion 🔖️Inverse
