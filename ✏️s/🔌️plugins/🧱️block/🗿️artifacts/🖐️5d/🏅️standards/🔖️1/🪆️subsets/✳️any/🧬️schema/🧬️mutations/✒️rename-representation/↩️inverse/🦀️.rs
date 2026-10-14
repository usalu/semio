//! ↩️ Inverse for `RenameRepresentation`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::rename_representation::RenameRepresentation, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::rename_representation::RenameRepresentation> {
    Ok((|| {
    match base.representations.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![crate::rename_representation::rename_representation(payload.id.clone(), existing.name.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
