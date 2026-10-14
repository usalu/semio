//! ↩️ Inverse for `ChangeNodeKindUnit`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &crate::change_node_kind_unit::ChangeNodeKindUnit, base: &Block2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_node_kind_unit::ChangeNodeKindUnit> {
    Ok((|| {
    vec![crate::change_node_kind_unit::change_node_kind_unit(base.node_kind.unit.clone())]

    })())
}
//#endregion 🔖️Inverse
