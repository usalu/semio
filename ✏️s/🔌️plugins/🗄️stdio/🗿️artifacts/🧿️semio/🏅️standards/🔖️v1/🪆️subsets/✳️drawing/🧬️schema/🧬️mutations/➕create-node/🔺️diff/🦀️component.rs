//! 🔺️ `create-node` — sparse diff construction; a no-op when `parent` does not resolve to a
//! `Group` (nothing to add children to), matching `node_at`'s own "absent ⇒ `None`" convention.

use super::mutation::CreateNode;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{IndexAdded, IndexedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{diff_at_path, node_at, DrawGroupDiff, DrawNodeDiff, SemioDrawingDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &CreateNode, base: &SemioDrawingSnapshot) -> SemioDrawingDiff {
    match node_at(base, &payload.parent) {
        Some(DrawNode::Group { children, .. }) => {
            let at = payload.index.min(children.len());
            diff_at_path(&payload.parent, DrawNodeDiff::Group(DrawGroupDiff { transform: None, children: Some(IndexedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![IndexAdded { index: at, item: payload.node.clone() }] }) }))
        }
        _ => SemioDrawingDiff::default(),
    }
}
//#endregion 🔖️Diff
