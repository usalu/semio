//! ↩️ Inverse for `ChangeNodeKindIcon`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &crate::change_node_kind_icon::ChangeNodeKindIcon, base: &Block2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_node_kind_icon::ChangeNodeKindIcon> {
    Ok((|| {
    vec![crate::change_node_kind_icon::change_node_kind_icon(base.node_kind.icon.clone())]

    })())
}
//#endregion 🔖️Inverse
