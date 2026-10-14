//! ↩️ Inverse for `ChangeNodeKindDescription`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &crate::change_node_kind_description::ChangeNodeKindDescription, base: &Block2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_node_kind_description::ChangeNodeKindDescription> {
    Ok((|| {
    vec![crate::change_node_kind_description::change_node_kind_description(base.node_kind.description.clone())]

    })())
}
//#endregion 🔖️Inverse
