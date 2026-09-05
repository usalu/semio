//! 🔺️ Diff for `FlattenNode`.

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{DrawNodeDiff, NodePath, SemioDrawingDiff, diff_at_path, node_at};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::FlattenNode, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<SemioDrawingDiff> {
    let Some(node) = node_at(base, &payload.at) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node at layer #{} does not exist.", payload.at.layer), [payload.at.layer.to_string()]);
    };
    match node {
        DrawNode::Group { transform, children } => match super::collect_flattened_leaves(children) {
            Some(leaves) if leaves != *children => protocol::MutationOutcome::new(diff_at_path(&payload.at, DrawNodeDiff::Replace { node: DrawNode::Group { transform: *transform, children: leaves } })),
            Some(_) => protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node in layer #{} is already flat.", payload.at.layer)),
            None => protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node in layer #{} cannot be flattened without losing a descendant transform.", payload.at.layer)),
        },
        _ => protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node in layer #{} is not a group.", payload.at.layer)),
    }
}
//#endregion 🔖️Diff
