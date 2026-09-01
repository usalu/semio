//! 🔺️ Diff for `UnflattenNode`.

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{DrawNodeDiff, NodePath, SemioDrawingDiff, diff_at_path, node_at};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::UnflattenNode, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<SemioDrawingDiff> {
    let Some(node) = node_at(base, &payload.at) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node at layer #{} does not exist.", payload.at.layer), [payload.at.layer.to_string()]);
    };
    if *node == payload.original {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node in layer #{} already matches the captured hierarchy.", payload.at.layer));
    }
    protocol::MutationOutcome::new(diff_at_path(&payload.at, DrawNodeDiff::Replace { node: payload.original.clone() }))
}
//#endregion 🔖️Diff
