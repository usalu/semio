//! ↩️ Inverse for `UngroupNode`.

use crate::schema::diff::node_at;
use crate::schema::mutations::delete_node::parent_and_index;
use crate::schema::mutations::{group_nodes, SemioDrawingMutation};
use crate::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::UngroupNode, base: &SemioDrawingSnapshot) -> Result<Vec<SemioDrawingMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some((parent, group_index)) = parent_and_index(&payload.at) else { return Vec::new() };
    match node_at(base, &payload.at) {
        Some(DrawNode::Group { transform, children }) => {
            let indices: Vec<usize> = (group_index..group_index + children.len()).collect();
            vec![SemioDrawingMutation::GroupNodes(group_nodes::GroupNodes { parent, indices, transform: *transform })]
        }
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
