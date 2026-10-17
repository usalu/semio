//! ↩️ Inverse for `SetSpatialNode`.

use crate::schema::snapshot::SemioModelSnapshot;
use crate::schema::mutations::set_spatial_node;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetSpatialNode, base: &SemioModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_spatial_node::SetSpatialNode>,
{
    let super::SetSpatialNode { id, kind, name, parent_id, placement } = payload;
    Ok(match base.spatial.iter().find(|n| &n.id == id) {
        Some(original) => vec![Op::from(set_spatial_node::SetSpatialNode {
            id: id.clone(),
            kind: kind.as_ref().map(|_| original.kind),
            name: name.as_ref().map(|_| original.name.clone()),
            parent_id: parent_id.as_ref().map(|_| original.parent_id.clone()),
            placement: placement.as_ref().map(|_| original.placement),
        })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
