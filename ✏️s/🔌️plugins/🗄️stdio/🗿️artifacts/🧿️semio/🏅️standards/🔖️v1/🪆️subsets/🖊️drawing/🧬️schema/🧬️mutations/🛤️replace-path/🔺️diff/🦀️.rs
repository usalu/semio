//! 🔺️ Diff for `ReplacePath`.

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{DrawNodeDiff, DrawPathDiff, NodePath, SemioDrawingDiff, diff_at_path, node_at};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, PathSegment, SemioDrawingSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::ReplacePath, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<SemioDrawingDiff> {
    match node_at(base, &payload.at) {
        Some(DrawNode::Path { segments, .. }) if *segments == payload.new_segments => protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Path in layer #{} already has those segments.", payload.at.layer)),
        Some(DrawNode::Path { .. }) => protocol::MutationOutcome::new(diff_at_path(&payload.at, DrawNodeDiff::Path(DrawPathDiff { segments: Some(payload.new_segments.clone()), style: None }))),
        _ => protocol::MutationOutcome::error("mutation.target-missing", format!("Node at layer #{} does not exist or is not a path.", payload.at.layer), [payload.at.layer.to_string()]),
    }
}
//#endregion 🔖️Diff
