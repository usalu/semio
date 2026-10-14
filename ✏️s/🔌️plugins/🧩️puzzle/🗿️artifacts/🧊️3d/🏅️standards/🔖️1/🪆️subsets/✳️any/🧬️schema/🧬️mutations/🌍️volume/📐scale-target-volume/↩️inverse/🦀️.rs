//! ↩️ Inverse for `ScaleTargetVolume` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use super::mutation::ScaleTargetVolume;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ScaleTargetVolume>>(payload: &super::mutation::ScaleTargetVolume, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(item) = base.target_volumes.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::scale_target_volume::mutation::scale_target_volume(item.id.clone(), item.scale)]

    })())
}
//#endregion 🔖️Inverse
