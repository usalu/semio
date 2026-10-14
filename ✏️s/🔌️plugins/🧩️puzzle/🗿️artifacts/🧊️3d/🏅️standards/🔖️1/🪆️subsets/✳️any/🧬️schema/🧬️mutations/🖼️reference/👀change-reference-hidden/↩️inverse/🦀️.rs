//! ↩️ Inverse for `ChangeReferenceHidden` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use super::mutation::ChangeReferenceHidden;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ChangeReferenceHidden>>(payload: &super::mutation::ChangeReferenceHidden, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(item) = base.references.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::change_reference_hidden::mutation::change_reference_hidden(item.id.clone(), item.hidden)]

    })())
}
//#endregion 🔖️Inverse
