//! ↩️ Inverse for `InsertSpatialNode`.

use crate::schema::snapshot::SemioModelSnapshot;
use crate::schema::mutations::remove_spatial_node;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::InsertSpatialNode, _base: &SemioModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::remove_spatial_node::RemoveSpatialNode>,
{
    let super::InsertSpatialNode { node, .. } = payload;
    Ok(vec![Op::from(remove_spatial_node::RemoveSpatialNode { id: node.id.clone() })])
}
//#endregion 🔖️Inverse
