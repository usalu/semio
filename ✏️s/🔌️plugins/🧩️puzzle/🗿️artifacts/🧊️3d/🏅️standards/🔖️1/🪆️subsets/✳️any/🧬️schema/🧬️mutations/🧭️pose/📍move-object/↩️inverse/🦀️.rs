//! ↩️ Inverse for `MoveObject` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use super::mutation::MoveObject;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<MoveObject>>(payload: &super::mutation::MoveObject, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(item) = base.objects.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::move_object::mutation::move_object(item.id.clone(), item.origin)]

    })())
}
//#endregion 🔖️Inverse
