//! ↩️ Inverse for `ChangeTargetVolumeLocked` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use super::mutation::ChangeTargetVolumeLocked;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ChangeTargetVolumeLocked>>(payload: &super::mutation::ChangeTargetVolumeLocked, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(item) = base.target_volumes.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::change_target_volume_locked::mutation::change_target_volume_locked(item.id.clone(), item.locked)]

    })())
}
//#endregion 🔖️Inverse
