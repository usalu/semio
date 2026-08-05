//! 🔺️ Block 5D artifact — the operation diff and its `OperationDiff` law, plus the id-keyed collection
//! diff plumbing shared with `🔧️op`'s `backwards()` (split out of the old constitutional `op` crate).

use crate::artifacts::block5d::{Block5dDefinition, Block5dGripKind, Block5dGripTemplate, Block5dPart2d, Block5dPart3d};
use crate::core::{BlockAttribute, BlockAuthor, BlockCamera2d, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation};
use protocol::OperationDiff;
use serde::{Deserialize, Serialize};

// #region 🔖️Collections
pub(crate) trait Block5dHasId {
    fn id(&self) -> &str;
}
impl Block5dHasId for BlockRepresentation {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Block5dHasId for Block5dGripKind {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Block5dHasId for Block5dGripTemplate {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Block5dHasId for BlockCompatibilityRule {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Block5dHasId for BlockAttribute {
    fn id(&self) -> &str {
        &self.key
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block5dCollectionDiff<T> {
    pub removed: Vec<String>,
    pub set: Vec<(usize, T)>,
}

/// 🧩️ Manual (not derived) `Default` — `#[derive(Default)]` on a generic struct bounds every type
/// parameter by `Default`, even though `Vec<(usize, T)>` never needs it.
impl<T> Default for Block5dCollectionDiff<T> {
    fn default() -> Self {
        Self { removed: Vec::new(), set: Vec::new() }
    }
}

fn apply_block5d_collection_diff<T: Block5dHasId + Clone>(items: &mut Vec<T>, removed: &[String], set: &[(usize, T)]) {
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

/// 🔍️ Reused by `🔧️op`'s `Operation::backwards` to look up a row's pre-operation state.
pub(crate) fn block5d_index_of<T: Block5dHasId>(items: &[T], id: &str) -> Option<usize> {
    items.iter().position(|item| item.id() == id)
}
// #endregion 🔖️Collections

// #region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block5dDiff {
    pub document: Option<Block5dDefinition>,
    pub part_kind: Option<BlockKindIdentity>,
    pub part_2d: Option<Block5dPart2d>,
    pub part_3d: Option<Block5dPart3d>,
    pub representations: Block5dCollectionDiff<BlockRepresentation>,
    pub grip_kinds: Block5dCollectionDiff<Block5dGripKind>,
    pub grips: Block5dCollectionDiff<Block5dGripTemplate>,
    pub compatibility: Block5dCollectionDiff<BlockCompatibilityRule>,
    pub attributes: Block5dCollectionDiff<BlockAttribute>,
    pub authors: Option<Vec<BlockAuthor>>,
    pub camera2d: Option<BlockCamera2d>,
    pub camera3d: Option<BlockCamera3d>,
    pub meta: Option<BlockMeta>,
}

fn block5d_diff_absorb(diff: &mut Block5dDiff, other: Block5dDiff) {
    if other.document.is_some() {
        *diff = Block5dDiff { document: other.document, ..Default::default() };
        return;
    }
    if other.part_kind.is_some() {
        diff.part_kind = other.part_kind;
    }
    if other.part_2d.is_some() {
        diff.part_2d = other.part_2d;
    }
    if other.part_3d.is_some() {
        diff.part_3d = other.part_3d;
    }
    diff.representations.removed.extend(other.representations.removed);
    diff.representations.set.extend(other.representations.set);
    diff.grip_kinds.removed.extend(other.grip_kinds.removed);
    diff.grip_kinds.set.extend(other.grip_kinds.set);
    diff.grips.removed.extend(other.grips.removed);
    diff.grips.set.extend(other.grips.set);
    diff.compatibility.removed.extend(other.compatibility.removed);
    diff.compatibility.set.extend(other.compatibility.set);
    diff.attributes.removed.extend(other.attributes.removed);
    diff.attributes.set.extend(other.attributes.set);
    if other.authors.is_some() {
        diff.authors = other.authors;
    }
    if other.camera2d.is_some() {
        diff.camera2d = other.camera2d;
    }
    if other.camera3d.is_some() {
        diff.camera3d = other.camera3d;
    }
    if other.meta.is_some() {
        diff.meta = other.meta;
    }
}

impl OperationDiff<Block5dDefinition> for Block5dDiff {
    fn apply(&self, projection: &Block5dDefinition) -> Block5dDefinition {
        if let Some(document) = &self.document {
            return document.clone();
        }
        let mut next = projection.clone();
        if let Some(part_kind) = &self.part_kind {
            next.part_kind = part_kind.clone();
        }
        if let Some(part_2d) = &self.part_2d {
            next.part_2d = part_2d.clone();
        }
        if let Some(part_3d) = &self.part_3d {
            next.part_3d = part_3d.clone();
        }
        apply_block5d_collection_diff(&mut next.representations, &self.representations.removed, &self.representations.set);
        apply_block5d_collection_diff(&mut next.grip_kinds, &self.grip_kinds.removed, &self.grip_kinds.set);
        apply_block5d_collection_diff(&mut next.grips, &self.grips.removed, &self.grips.set);
        apply_block5d_collection_diff(&mut next.compatibility, &self.compatibility.removed, &self.compatibility.set);
        apply_block5d_collection_diff(&mut next.attributes, &self.attributes.removed, &self.attributes.set);
        if let Some(authors) = &self.authors {
            next.authors = authors.clone();
        }
        if let Some(camera2d) = &self.camera2d {
            next.camera2d = camera2d.clone();
        }
        if let Some(camera3d) = &self.camera3d {
            next.camera3d = camera3d.clone();
        }
        if let Some(meta) = &self.meta {
            next.meta = meta.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        block5d_diff_absorb(self, other);
    }
}
// #endregion 🔖️Diff
