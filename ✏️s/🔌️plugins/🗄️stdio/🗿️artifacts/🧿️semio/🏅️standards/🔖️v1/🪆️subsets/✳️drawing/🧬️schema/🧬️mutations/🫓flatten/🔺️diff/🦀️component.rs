//! 🔺️ `flatten` — sparse diff construction; an absent `at` is `mutation.target-missing` (Error,
//! empty diff). `at` not resolving to a `Group`, any descendant group having a non-identity
//! `transform` (see `🦠️mutation/🦀️component.rs`'s doc comment for why), or the group already
//! being flat (no change to `children`) is `mutation.no-op` (Warning, empty diff).

use super::mutation::FlattenNode;
use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioTransform;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{diff_at_path, node_at, DrawNodeDiff, SemioDrawingDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️CollectLeaves
/// 🍃️️ Depth-first leaf collection through any run of identity-transform descendant `Group`s;
/// `None` the moment a non-identity transform is found (refuse rather than approximate).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn collect_flattened_leaves(children: &[DrawNode]) -> Option<Vec<DrawNode>> {
    let mut out = Vec::new();
    for child in children {
        match child {
            DrawNode::Group { transform, children: nested } => {
                if *transform != SemioTransform::identity() {
                    return None;
                }
                out.extend(Box::pin(collect_flattened_leaves(nested))?);
            }
            leaf => out.push(leaf.clone()),
        }
    }
    Some(out)
}
//#endregion 🔖️CollectLeaves

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &FlattenNode, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<SemioDrawingDiff> {
    let Some(node) = node_at(base, &payload.at) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node at layer #{} does not exist.", payload.at.layer), [payload.at.layer.to_string()]);
    };
    match node {
        DrawNode::Group { transform, children } => match collect_flattened_leaves(children) {
            Some(leaves) if leaves != *children => protocol::MutationOutcome::new(diff_at_path(&payload.at, DrawNodeDiff::Replace { node: DrawNode::Group { transform: *transform, children: leaves } })),
            Some(_) => protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node in layer #{} is already flat.", payload.at.layer)),
            None => protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node in layer #{} cannot be flattened without losing a descendant transform.", payload.at.layer)),
        },
        _ => protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node in layer #{} is not a group.", payload.at.layer)),
    }
}
//#endregion 🔖️Diff
