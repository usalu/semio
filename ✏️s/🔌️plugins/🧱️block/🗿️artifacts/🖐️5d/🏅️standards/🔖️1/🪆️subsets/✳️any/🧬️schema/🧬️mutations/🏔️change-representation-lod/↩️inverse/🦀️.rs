//! ↩️ Inverse for `ChangeRepresentationLod`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::change_representation_lod::ChangeRepresentationLod, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_representation_lod::ChangeRepresentationLod> {
    Ok((|| {
    match base.representations.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![crate::change_representation_lod::change_representation_lod(payload.id.clone(), existing.lod.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
