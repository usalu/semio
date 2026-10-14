//! ↩️ Inverse for `RotateObject` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use super::mutation::RotateObject;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<RotateObject>>(payload: &super::mutation::RotateObject, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(item) = base.objects.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::rotate_object::mutation::rotate_object(item.id.clone(), item.orientation)]

    })())
}
//#endregion 🔖️Inverse
