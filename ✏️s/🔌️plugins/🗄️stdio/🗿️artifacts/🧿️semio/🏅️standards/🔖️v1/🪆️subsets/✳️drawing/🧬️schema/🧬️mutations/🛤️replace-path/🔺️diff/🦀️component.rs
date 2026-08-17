//! 🔺️ `replace-path` — sparse diff construction; `at` not resolving to a `Path` (absent, or a
//! different node kind) is `mutation.target-missing` (Error, empty diff); `new_segments`
//! identical to the path's current `segments` is `mutation.no-op` (Warning, empty diff).

use super::mutation::ReplacePath;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{diff_at_path, node_at, DrawNodeDiff, DrawPathDiff, SemioDrawingDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ReplacePath, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<SemioDrawingDiff> {
    match node_at(base, &payload.at) {
        Some(DrawNode::Path { segments, .. }) if *segments == payload.new_segments => protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Path in layer #{} already has those segments.", payload.at.layer)),
        Some(DrawNode::Path { .. }) => protocol::MutationOutcome::new(diff_at_path(&payload.at, DrawNodeDiff::Path(DrawPathDiff { segments: Some(payload.new_segments.clone()), style: None }))),
        _ => protocol::MutationOutcome::error("mutation.target-missing", format!("Node at layer #{} does not exist or is not a path.", payload.at.layer), [payload.at.layer.to_string()]),
    }
}
//#endregion 🔖️Diff
