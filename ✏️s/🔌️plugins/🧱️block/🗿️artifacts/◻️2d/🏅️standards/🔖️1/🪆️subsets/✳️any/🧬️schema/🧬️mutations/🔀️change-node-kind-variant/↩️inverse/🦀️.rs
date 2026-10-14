//! ↩️ Inverse for `ChangeNodeKindVariant`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &crate::change_node_kind_variant::ChangeNodeKindVariant, base: &Block2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_node_kind_variant::ChangeNodeKindVariant> {
    Ok((|| {
    vec![crate::change_node_kind_variant::change_node_kind_variant(base.node_kind.variant.clone())]

    })())
}
//#endregion 🔖️Inverse
