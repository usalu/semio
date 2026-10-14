//! ↩️ Inverse for `DeleteReference` — reconstructs a `create-reference` of the captured
//! BASE entry. Missing target ⇒ `Vec::new()`.
use super::mutation::DeleteReference;
use crate::create_reference::mutation::CreateReference;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<DeleteReference> + From<CreateReference>>(payload: &super::mutation::DeleteReference, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(item) = base.references.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    let index = base.references.iter().position(|entry| entry.id == payload.id);
    vec![crate::create_reference::mutation::create_reference(item.clone(), index)]

    })())
}
//#endregion 🔖️Inverse
