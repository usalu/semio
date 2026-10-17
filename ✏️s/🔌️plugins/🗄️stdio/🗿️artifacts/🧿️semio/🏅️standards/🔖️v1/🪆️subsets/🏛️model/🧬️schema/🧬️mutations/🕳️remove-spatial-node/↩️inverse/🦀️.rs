//! ↩️ Inverse for `RemoveSpatialNode`.

use crate::schema::snapshot::SemioModelSnapshot;
use crate::schema::mutations::insert_spatial_node;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::RemoveSpatialNode, base: &SemioModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::insert_spatial_node::InsertSpatialNode>,
{
    let super::RemoveSpatialNode { id } = payload;
    Ok(match base.spatial.iter().position(|n| &n.id == id) {
        Some(at) => vec![Op::from(insert_spatial_node::InsertSpatialNode { node: base.spatial[at].clone(), at: Some(at) })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
