//! ↩️ Inverse for `EditObjectLabel` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use super::mutation::EditObjectLabel;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<EditObjectLabel>>(payload: &super::mutation::EditObjectLabel, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(item) = base.objects.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::edit_object_label::mutation::edit_object_label(item.id.clone(), item.label.clone())]

    })())
}
//#endregion 🔖️Inverse
