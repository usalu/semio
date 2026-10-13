//! ↩️ Inverse for `CreateNode`.

use crate::schema::diff::{node_at, NodePath};
use crate::schema::mutations::{delete_node, SemioDrawingMutation};
use crate::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateNode, base: &SemioDrawingSnapshot) -> Result<Vec<SemioDrawingMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match node_at(base, &payload.parent) {
        Some(DrawNode::Group { children, .. }) => {
            let at = payload.index.min(children.len());
            let mut path = payload.parent.path.clone();
            path.push(at);
            vec![SemioDrawingMutation::DeleteNode(delete_node::DeleteNode { at: NodePath { layer: payload.parent.layer, path } })]
        }
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
