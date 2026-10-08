//! ↩️ Inverse for `SetSpatialNode`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetSpatialNode, base: &SemioModelSnapshot) -> Result<Vec<SemioModelMutation>, semio_framework_value::ValueError> {
    let super::SetSpatialNode { id, kind, name, parent_id, placement } = payload;
    Ok(match base.spatial.iter().find(|n| &n.id == id) {
        Some(original) => vec![SemioModelMutation::SetSpatialNode(set_spatial_node::SetSpatialNode {
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
