//! ↩️ Inverse for `ChangeNodeKindUnit`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeNodeKindUnit, base: &Block2dSnapshot) -> Result<Vec<Block2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::super::change_node_kind_unit::change_node_kind_unit(base.node_kind.unit.clone())]

    })())
}
//#endregion 🔖️Inverse
