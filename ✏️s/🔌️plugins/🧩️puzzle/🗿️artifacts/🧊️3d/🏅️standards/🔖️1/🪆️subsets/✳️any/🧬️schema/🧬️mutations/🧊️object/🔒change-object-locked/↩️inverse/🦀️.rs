//! ↩️ Inverse for `ChangeObjectLocked` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use super::mutation::ChangeObjectLocked;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ChangeObjectLocked>>(payload: &super::mutation::ChangeObjectLocked, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(item) = base.objects.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::change_object_locked::mutation::change_object_locked(item.id.clone(), item.locked)]

    })())
}
//#endregion 🔖️Inverse
