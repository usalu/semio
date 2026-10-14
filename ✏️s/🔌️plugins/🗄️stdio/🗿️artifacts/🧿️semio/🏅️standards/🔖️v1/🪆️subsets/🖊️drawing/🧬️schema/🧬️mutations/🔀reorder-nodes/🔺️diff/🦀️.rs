//! 🔺️ Diff for `ReorderNodes`.

use semio_s_artifact_stdio_semio_base::schema::triples::IndexAdded;
use semio_s_artifact_stdio_semio_base::schema::triples::IndexedTripleDiff;
use crate::schema::diff::{diff_at_path, node_at, DrawGroupDiff, DrawNodeDiff, SemioDrawingDiff};
use crate::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::ReorderNodes, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<SemioDrawingDiff> {
    match node_at(base, &payload.parent) {
        Some(DrawNode::Group { children, .. }) if payload.from < children.len() => {
            if payload.from == payload.to {
                return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Node #{} in layer #{} is already at position #{}.", payload.from, payload.parent.layer, payload.to));
            }
            let item = children[payload.from].clone();
            protocol::MutationOutcome::new(diff_at_path(
                &payload.parent,
                DrawNodeDiff::Group(DrawGroupDiff { transform: None, children: Some(IndexedTripleDiff { removed: vec![payload.from], modified: Vec::new(), added: vec![IndexAdded { index: payload.to, item }] }) }),
            ))
        }
        _ => protocol::MutationOutcome::error("mutation.target-missing", format!("Parent at layer #{} does not resolve to a group, or index #{} is out of range.", payload.parent.layer, payload.from), [payload.parent.layer.to_string()]),
    }
}
//#endregion 🔖️Diff
