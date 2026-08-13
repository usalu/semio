//! 🔺️ `reorder-nodes` — sparse diff construction; a no-op when `parent` is not a `Group` or
//! `from` is out of range for its `children`.

use super::mutation::ReorderNodes;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{IndexAdded, IndexedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{diff_at_path, node_at, DrawGroupDiff, DrawNodeDiff, SemioDrawingDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ReorderNodes, base: &SemioDrawingSnapshot) -> SemioDrawingDiff {
    match node_at(base, &payload.parent) {
        Some(DrawNode::Group { children, .. }) if payload.from < children.len() => {
            let item = children[payload.from].clone();
            diff_at_path(&payload.parent, DrawNodeDiff::Group(DrawGroupDiff { transform: None, children: Some(IndexedTripleDiff { removed: vec![payload.from], modified: Vec::new(), added: vec![IndexAdded { index: payload.to, item }] }) }))
        }
        _ => SemioDrawingDiff::default(),
    }
}
//#endregion 🔖️Diff
