//! ↩️ Inverse for `RemoveSpatialNode`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveSpatialNode, base: &SemioModelSnapshot) -> Result<Vec<SemioModelMutation>, semio_framework_value::ValueError> {
    let super::RemoveSpatialNode { id } = payload;
    Ok(match base.spatial.iter().position(|n| &n.id == id) {
        Some(at) => vec![SemioModelMutation::InsertSpatialNode(insert_spatial_node::InsertSpatialNode { node: base.spatial[at].clone(), at: Some(at) })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
