//! ↩️ Inverse for `CreateReference` — always a `delete-reference` of the id it created.
use super::mutation::CreateReference;
use crate::delete_reference::mutation::DeleteReference;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<CreateReference> + From<DeleteReference>>(payload: &super::mutation::CreateReference, _base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![crate::delete_reference::mutation::delete_reference(payload.reference.id.clone())]

    })())
}
//#endregion 🔖️Inverse
