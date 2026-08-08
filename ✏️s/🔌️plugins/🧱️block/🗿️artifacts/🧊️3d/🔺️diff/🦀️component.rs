//! 🔺️ Block 3D artifact — the operation diff and its `MutationDiff` law, plus the id-keyed collection
//! diff plumbing shared with `🔧️op`'s `backwards()` (split out of the old constitutional `op` crate).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::block3d::{Block3dDefinition, Block3dVortexKind, Block3dVortexTemplate};
use crate::{BlockAttribute, BlockAuthor, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation};
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};

// #region 🔖️Collections
pub(crate) trait Block3dHasId {
    fn id(&self) -> &str;
}
impl Block3dHasId for BlockRepresentation {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Block3dHasId for Block3dVortexKind {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Block3dHasId for Block3dVortexTemplate {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Block3dHasId for BlockCompatibilityRule {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Block3dHasId for BlockAttribute {
    fn id(&self) -> &str {
        &self.key
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block3dCollectionDiff<T> {
    pub removed: Vec<String>,
    pub set: Vec<(usize, T)>,
}

/// 🧩️ Manual (not derived) `Default` — `#[derive(Default)]` on a generic struct bounds every type
/// parameter by `Default`, even though `Vec<(usize, T)>` never needs it.
impl<T> Default for Block3dCollectionDiff<T> {
    fn default() -> Self {
        Self { removed: Vec::new(), set: Vec::new() }
    }
}

fn apply_block3d_collection_diff<T: Block3dHasId + Clone>(items: &mut Vec<T>, removed: &[String], set: &[(usize, T)]) {
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

/// 🔍️ Reused by `🔧️op`'s `Mutation::inverse` to look up a row's pre-operation state.
pub(crate) fn block3d_index_of<T: Block3dHasId>(items: &[T], id: &str) -> Option<usize> {
    items.iter().position(|item| item.id() == id)
}
// #endregion 🔖️Collections

// #region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block3dDiff {
    pub document: Option<Block3dDefinition>,
    pub object_kind: Option<BlockKindIdentity>,
    pub representations: Block3dCollectionDiff<BlockRepresentation>,
    pub vortex_kinds: Block3dCollectionDiff<Block3dVortexKind>,
    pub vortices: Block3dCollectionDiff<Block3dVortexTemplate>,
    pub compatibility: Block3dCollectionDiff<BlockCompatibilityRule>,
    pub attributes: Block3dCollectionDiff<BlockAttribute>,
    pub authors: Option<Vec<BlockAuthor>>,
    pub camera3d: Option<BlockCamera3d>,
    pub meta: Option<BlockMeta>,
}

fn block3d_diff_absorb(diff: &mut Block3dDiff, other: Block3dDiff) {
    if other.document.is_some() {
        *diff = Block3dDiff { document: other.document, ..Default::default() };
        return;
    }
    if other.object_kind.is_some() {
        diff.object_kind = other.object_kind;
    }
    diff.representations.removed.extend(other.representations.removed);
    diff.representations.set.extend(other.representations.set);
    diff.vortex_kinds.removed.extend(other.vortex_kinds.removed);
    diff.vortex_kinds.set.extend(other.vortex_kinds.set);
    diff.vortices.removed.extend(other.vortices.removed);
    diff.vortices.set.extend(other.vortices.set);
    diff.compatibility.removed.extend(other.compatibility.removed);
    diff.compatibility.set.extend(other.compatibility.set);
    diff.attributes.removed.extend(other.attributes.removed);
    diff.attributes.set.extend(other.attributes.set);
    if other.authors.is_some() {
        diff.authors = other.authors;
    }
    if other.camera3d.is_some() {
        diff.camera3d = other.camera3d;
    }
    if other.meta.is_some() {
        diff.meta = other.meta;
    }
}

impl MutationDiff<Block3dDefinition> for Block3dDiff {
    fn apply(&self, projection: &Block3dDefinition) -> Block3dDefinition {
        if let Some(document) = &self.document {
            return document.clone();
        }
        let mut next = projection.clone();
        if let Some(object_kind) = &self.object_kind {
            next.object_kind = object_kind.clone();
        }
        apply_block3d_collection_diff(&mut next.representations, &self.representations.removed, &self.representations.set);
        apply_block3d_collection_diff(&mut next.vortex_kinds, &self.vortex_kinds.removed, &self.vortex_kinds.set);
        apply_block3d_collection_diff(&mut next.vortices, &self.vortices.removed, &self.vortices.set);
        apply_block3d_collection_diff(&mut next.compatibility, &self.compatibility.removed, &self.compatibility.set);
        apply_block3d_collection_diff(&mut next.attributes, &self.attributes.removed, &self.attributes.set);
        if let Some(authors) = &self.authors {
            next.authors = authors.clone();
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
        block3d_diff_absorb(self, other);
    }
}
// #endregion 🔖️Diff
