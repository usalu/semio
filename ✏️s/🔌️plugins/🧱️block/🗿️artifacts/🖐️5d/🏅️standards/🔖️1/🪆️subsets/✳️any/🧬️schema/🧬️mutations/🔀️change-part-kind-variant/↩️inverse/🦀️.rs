//! ↩️ Inverse for `ChangePartKindVariant`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &crate::change_part_kind_variant::ChangePartKindVariant, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_part_kind_variant::ChangePartKindVariant> {
    Ok((|| {
    vec![crate::change_part_kind_variant::change_part_kind_variant(base.part_kind.variant.clone())]

    })())
}
//#endregion 🔖️Inverse
