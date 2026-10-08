//! ↩️ Inverse for `InsertSpatialNode`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::InsertSpatialNode, base: &SemioModelSnapshot) -> Result<Vec<SemioModelMutation>, semio_framework_value::ValueError> {
    let super::InsertSpatialNode { node, .. } = payload;
    Ok(vec![SemioModelMutation::RemoveSpatialNode(remove_spatial_node::RemoveSpatialNode { id: node.id.clone() })])
}
//#endregion 🔖️Inverse
