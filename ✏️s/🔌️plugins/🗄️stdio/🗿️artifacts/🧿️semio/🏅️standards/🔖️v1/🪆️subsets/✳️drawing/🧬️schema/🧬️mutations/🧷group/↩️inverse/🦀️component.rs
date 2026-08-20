//! ↩️ `group` — undo is `ungroup` at the address the new `Group` landed at; a no-op
//! (`Vec::new()`) for the same invalid shapes `diff` refuses.

use super::diff::is_contiguous_ascending;
use super::mutation::GroupNodes;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{node_at, NodePath};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::{ungroup, SemioDrawingMutation};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &GroupNodes, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    if !is_contiguous_ascending(&payload.indices) {
        return Vec::new();
    }
    match node_at(base, &payload.parent) {
        Some(DrawNode::Group { children, .. }) if payload.indices.iter().all(|&i| i < children.len()) => {
            let mut path = payload.parent.path.clone();
            path.push(payload.indices[0]);
            vec![SemioDrawingMutation::Ungroup(ungroup::mutation::UngroupNode { at: NodePath { layer: payload.parent.layer, path } })]
        }
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
