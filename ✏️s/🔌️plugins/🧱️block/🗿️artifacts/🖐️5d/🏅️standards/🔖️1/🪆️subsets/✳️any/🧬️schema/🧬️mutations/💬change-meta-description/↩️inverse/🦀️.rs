//! ↩️ Inverse for `ChangeMetaDescription`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &crate::change_meta_description::ChangeMetaDescription, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_meta_description::ChangeMetaDescription> {
    Ok((|| {
    vec![crate::change_meta_description::change_meta_description(base.meta.description.clone())]

    })())
}
//#endregion 🔖️Inverse
