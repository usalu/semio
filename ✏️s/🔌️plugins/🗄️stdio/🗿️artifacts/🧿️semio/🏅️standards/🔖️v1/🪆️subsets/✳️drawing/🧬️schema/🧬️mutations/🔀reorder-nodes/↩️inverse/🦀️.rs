//! ↩️ Inverse for `ReorderNodes`.

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{DrawGroupDiff, DrawNodeDiff, NodePath, SemioDrawingDiff, diff_at_path, node_at};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::ReorderNodes, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    match node_at(base, &payload.parent) {
        Some(DrawNode::Group { children, .. }) if !children.is_empty() && payload.from < children.len() => {
            let landed_at = payload.to.min(children.len() - 1);
            vec![SemioDrawingMutation::ReorderNodes(super::ReorderNodes { parent: payload.parent.clone(), from: landed_at, to: payload.from })]
        }
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
