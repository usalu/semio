//! 🔺️ `flatten` — sparse diff construction; a no-op when `at` is not a `Group` or any descendant
//! group has a non-identity `transform` (see `🦠️mutation/🦀️component.rs`'s doc comment for why).

use super::mutation::FlattenNode;
use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioTransform;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{diff_at_path, node_at, DrawNodeDiff, SemioDrawingDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️CollectLeaves
/// 🍃️️ Depth-first leaf collection through any run of identity-transform descendant `Group`s;
/// `None` the moment a non-identity transform is found (refuse rather than approximate).
pub(crate) fn collect_flattened_leaves(children: &[DrawNode]) -> Option<Vec<DrawNode>> {
    let mut out = Vec::new();
    for child in children {
        match child {
            DrawNode::Group { transform, children: nested } => {
                if *transform != SemioTransform::identity() {
                    return None;
                }
                out.extend(collect_flattened_leaves(nested)?);
            }
            leaf => out.push(leaf.clone()),
        }
    }
    Some(out)
}
//#endregion 🔖️CollectLeaves

//#region 🔖️Diff
pub fn diff(payload: &FlattenNode, base: &SemioDrawingSnapshot) -> SemioDrawingDiff {
    match node_at(base, &payload.at) {
        Some(DrawNode::Group { transform, children }) => match collect_flattened_leaves(children) {
            Some(leaves) => diff_at_path(&payload.at, DrawNodeDiff::Replace { node: DrawNode::Group { transform: *transform, children: leaves } }),
            None => SemioDrawingDiff::default(),
        },
        _ => SemioDrawingDiff::default(),
    }
}
//#endregion 🔖️Diff
