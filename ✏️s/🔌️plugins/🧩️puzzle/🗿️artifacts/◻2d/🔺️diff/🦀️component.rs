//! 🔺️ Puzzle 2d artifact — the sparse diff over the typed projection: id-keyed node/edge
//! collection deltas plus the scalar meta, and the whole-document replacement that wins over them.
//! The `serde_json::Value` and `Puzzle2dPlayProjection` bridge impls of the same diff live in
//! `🔧️op` beside the newtypes they patch.

use crate::artifacts::puzzle2d::{Puzzle2dEdge, Puzzle2dMeta, Puzzle2dNode, Puzzle2dProjection};
use protocol::OperationDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Collections
/// 🪪️ Stable-id accessor shared by every id-keyed document collection entry.
pub(crate) trait Puzzle2dHasId {
    fn id(&self) -> &str;
}

impl Puzzle2dHasId for Puzzle2dNode {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Puzzle2dHasId for Puzzle2dEdge {
    fn id(&self) -> &str {
        &self.id
    }
}

/// 🩹️ Sparse id-keyed collection diff — removals plus id-or-index `set`s (replace when the id
/// already exists, else insert at the recorded index).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dNodesDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, Puzzle2dNode)>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dEdgesDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, Puzzle2dEdge)>,
}

fn apply_puzzle2d_collection_diff<T: Puzzle2dHasId + Clone>(items: &mut Vec<T>, removed: &[String], set: &[(usize, T)]) {
    for id in removed {
        items.retain(|item| item.id() != id);
    }
    for (index, item) in set {
        if let Some(pos) = items.iter().position(|entry| entry.id() == item.id()) {
            items[pos] = item.clone();
        } else {
            items.insert((*index).min(items.len()), item.clone());
        }
    }
}

pub(crate) fn puzzle2d_index_of<T: Puzzle2dHasId>(items: &[T], id: &str) -> Option<usize> {
    items.iter().position(|item| item.id() == id)
}
//#endregion 🔖️Collections

//#region 🔖️Diff
/// 🩹️ Sparse puzzle-2d diff over both id-keyed collections plus the scalar meta. The camera is
/// deliberately absent: it is session-only `Puzzle2dConfig` state in the play app (see `setCamera`'s
/// `ActionKind::View`), never a document field, so there is no `SetCamera` operation left to diff.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dDiff {
    /// 🌍️ Whole-document replacement (example load, engine fill, layout); wins over every field below.
    pub document: Option<Puzzle2dProjection>,
    pub nodes: Puzzle2dNodesDiff,
    pub edges: Puzzle2dEdgesDiff,
    pub meta: Option<Puzzle2dMeta>,
}

pub(crate) fn puzzle2d_diff_absorb(diff: &mut Puzzle2dDiff, other: Puzzle2dDiff) {
    if other.document.is_some() {
        *diff = Puzzle2dDiff { document: other.document, ..Default::default() };
        return;
    }
    diff.nodes.removed.extend(other.nodes.removed);
    diff.nodes.set.extend(other.nodes.set);
    diff.edges.removed.extend(other.edges.removed);
    diff.edges.set.extend(other.edges.set);
    if other.meta.is_some() {
        diff.meta = other.meta;
    }
}

impl OperationDiff<Puzzle2dProjection> for Puzzle2dDiff {
    fn apply(&self, projection: &Puzzle2dProjection) -> Puzzle2dProjection {
        if let Some(document) = &self.document {
            return document.clone();
        }
        let mut next = projection.clone();
        apply_puzzle2d_collection_diff(&mut next.nodes, &self.nodes.removed, &self.nodes.set);
        apply_puzzle2d_collection_diff(&mut next.edges, &self.edges.removed, &self.edges.set);
        if let Some(meta) = &self.meta {
            next.meta = meta.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        puzzle2d_diff_absorb(self, other);
    }
}
//#endregion 🔖️Diff
