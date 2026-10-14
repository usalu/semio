//! ↩️ Inverse for `ChangeObjectKindVariant`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &crate::change_object_kind_variant::ChangeObjectKindVariant, base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_object_kind_variant::ChangeObjectKindVariant> {
    Ok((|| {
    vec![crate::change_object_kind_variant::change_object_kind_variant(base.object_kind.variant.clone())]

    })())
}
//#endregion 🔖️Inverse
