//! ↩️ Inverse for `ChangeObjectHidden` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use super::mutation::ChangeObjectHidden;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ChangeObjectHidden>>(payload: &super::mutation::ChangeObjectHidden, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(item) = base.objects.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::change_object_hidden::mutation::change_object_hidden(item.id.clone(), item.hidden)]

    })())
}
//#endregion 🔖️Inverse
