//! ↩️ Inverse for `GroupNodes`.

use crate::schema::diff::{node_at, NodePath};
use crate::schema::mutations::ungroup_node;
use crate::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::GroupNodes, base: &SemioDrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::ungroup_node::UngroupNode>,
{
    Ok((|| {
    if !super::is_contiguous_ascending(&payload.indices) {
        return Vec::new();
    }
    match node_at(base, &payload.parent) {
        Some(DrawNode::Group { children, .. }) if payload.indices.iter().all(|&i| i < children.len()) => {
            let mut path = payload.parent.path.clone();
            path.push(payload.indices[0]);
            vec![Op::from(ungroup_node::UngroupNode { at: NodePath { layer: payload.parent.layer, path } })]
        }
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
