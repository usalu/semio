//! 🔺️ Block 2D artifact — the operation diff and its `OperationDiff` law, plus the id-keyed collection
//! diff plumbing shared with `🔧️op`'s `backwards()` (split out of the old constitutional `op` crate).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::block2d::{Block2dDefinition, Block2dHandleKind, Block2dHandleTemplate, Block2dPresentation};
use crate::core::{BlockAttribute, BlockAuthor, BlockCamera2d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta};
use protocol::OperationDiff;
use serde::{Deserialize, Serialize};

// #region 🔖️Collections
pub(crate) trait Block2dHasId {
    fn id(&self) -> &str;
}
impl Block2dHasId for Block2dHandleKind {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Block2dHasId for Block2dHandleTemplate {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Block2dHasId for BlockCompatibilityRule {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Block2dHasId for BlockAttribute {
    fn id(&self) -> &str {
        &self.key
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block2dCollectionDiff<T> {
    pub removed: Vec<String>,
    pub set: Vec<(usize, T)>,
}

/// 🧩️ Manual (not derived) `Default` — `#[derive(Default)]` on a generic struct bounds every type
/// parameter by `Default`, even though `Vec<(usize, T)>` never needs it.
impl<T> Default for Block2dCollectionDiff<T> {
    fn default() -> Self {
        Self { removed: Vec::new(), set: Vec::new() }
    }
}

fn apply_block2d_collection_diff<T: Block2dHasId + Clone>(items: &mut Vec<T>, removed: &[String], set: &[(usize, T)]) {
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
pub(crate) fn block2d_index_of<T: Block2dHasId>(items: &[T], id: &str) -> Option<usize> {
    items.iter().position(|item| item.id() == id)
}
// #endregion 🔖️Collections

// #region 🔖️Diff
/// 🩹️ Sparse block-2d diff over the four id-keyed tables plus the scalar node_kind/presentation/
/// camera2d/meta.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block2dDiff {
    /// 🌍️ Whole-document replacement (example load, reset); wins over every field below.
    pub document: Option<Block2dDefinition>,
    pub node_kind: Option<BlockKindIdentity>,
    pub presentation: Option<Block2dPresentation>,
    pub handle_kinds: Block2dCollectionDiff<Block2dHandleKind>,
    pub handles: Block2dCollectionDiff<Block2dHandleTemplate>,
    pub compatibility: Block2dCollectionDiff<BlockCompatibilityRule>,
    pub attributes: Block2dCollectionDiff<BlockAttribute>,
    pub authors: Option<Vec<BlockAuthor>>,
    pub camera2d: Option<BlockCamera2d>,
    pub meta: Option<BlockMeta>,
}

fn block2d_diff_absorb(diff: &mut Block2dDiff, other: Block2dDiff) {
    if other.document.is_some() {
        *diff = Block2dDiff { document: other.document, ..Default::default() };
        return;
    }
    if other.node_kind.is_some() {
        diff.node_kind = other.node_kind;
    }
    if other.presentation.is_some() {
        diff.presentation = other.presentation;
    }
    diff.handle_kinds.removed.extend(other.handle_kinds.removed);
    diff.handle_kinds.set.extend(other.handle_kinds.set);
    diff.handles.removed.extend(other.handles.removed);
    diff.handles.set.extend(other.handles.set);
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
    if other.meta.is_some() {
        diff.meta = other.meta;
    }
}

impl OperationDiff<Block2dDefinition> for Block2dDiff {
    fn apply(&self, projection: &Block2dDefinition) -> Block2dDefinition {
        if let Some(document) = &self.document {
            return document.clone();
        }
        let mut next = projection.clone();
        if let Some(node_kind) = &self.node_kind {
            next.node_kind = node_kind.clone();
        }
        if let Some(presentation) = &self.presentation {
            next.presentation = presentation.clone();
        }
        apply_block2d_collection_diff(&mut next.handle_kinds, &self.handle_kinds.removed, &self.handle_kinds.set);
        apply_block2d_collection_diff(&mut next.handles, &self.handles.removed, &self.handles.set);
        apply_block2d_collection_diff(&mut next.compatibility, &self.compatibility.removed, &self.compatibility.set);
        apply_block2d_collection_diff(&mut next.attributes, &self.attributes.removed, &self.attributes.set);
        if let Some(authors) = &self.authors {
            next.authors = authors.clone();
        }
        if let Some(camera2d) = &self.camera2d {
            next.camera2d = camera2d.clone();
        }
        if let Some(meta) = &self.meta {
            next.meta = meta.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        block2d_diff_absorb(self, other);
    }
}
// #endregion 🔖️Diff
