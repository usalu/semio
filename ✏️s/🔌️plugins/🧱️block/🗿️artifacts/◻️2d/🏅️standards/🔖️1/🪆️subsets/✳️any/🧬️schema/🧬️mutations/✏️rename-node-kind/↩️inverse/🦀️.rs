//! ↩️ Inverse for `RenameNodeKind`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &crate::rename_node_kind::RenameNodeKind, base: &Block2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::rename_node_kind::RenameNodeKind> {
    Ok((|| {
    vec![crate::rename_node_kind::rename_node_kind(base.node_kind.name.clone())]

    })())
}
//#endregion 🔖️Inverse
