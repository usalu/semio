//! ↩️ Inverse for `ChangeNodeKindLabel`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &crate::change_node_kind_label::ChangeNodeKindLabel, base: &Block2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_node_kind_label::ChangeNodeKindLabel> {
    Ok((|| {
    vec![crate::change_node_kind_label::change_node_kind_label(base.node_kind.label.clone())]

    })())
}
//#endregion 🔖️Inverse
