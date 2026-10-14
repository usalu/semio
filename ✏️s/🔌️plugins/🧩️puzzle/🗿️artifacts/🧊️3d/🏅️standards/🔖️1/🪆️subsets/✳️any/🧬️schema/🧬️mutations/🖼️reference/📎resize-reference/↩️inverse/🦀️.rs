//! ↩️ Inverse for `ResizeReference` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use super::mutation::ResizeReference;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ResizeReference>>(payload: &super::mutation::ResizeReference, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(item) = base.references.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::resize_reference::mutation::resize_reference(item.id.clone(), item.width_world)]

    })())
}
//#endregion 🔖️Inverse
