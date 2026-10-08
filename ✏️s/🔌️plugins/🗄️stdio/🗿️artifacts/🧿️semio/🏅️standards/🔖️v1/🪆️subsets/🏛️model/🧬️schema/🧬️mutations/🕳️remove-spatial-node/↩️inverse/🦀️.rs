//! ↩️ Inverse for `RemoveSpatialNode`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveSpatialNode, base: &SemioModelSnapshot) -> Result<Vec<SemioModelMutation>, semio_framework_value::ValueError> {
    let super::RemoveSpatialNode { id } = payload;
    Ok(match base.spatial.iter().find(|n| &n.id == id) {
        Some(original) => vec![SemioModelMutation::InsertSpatialNode(insert_spatial_node::InsertSpatialNode { node: original.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
