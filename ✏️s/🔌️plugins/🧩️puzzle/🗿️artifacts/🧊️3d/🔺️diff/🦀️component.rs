//! 🔺️ Puzzle 3d artifact — the sparse diff over the typed projection: id-keyed object/attraction/
//! target-volume/reference collection deltas plus the scalar meta, and the whole-document
//! replacement that wins over them. The `serde_json::Value` and `Puzzle3dPlayProjection` bridge
//! impls of the same diff live in `🔧️op` beside the newtypes they patch.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::puzzle3d::{Puzzle3dAttraction, Puzzle3dMeta, Puzzle3dObject, Puzzle3dProjection, Puzzle3dReference, Puzzle3dTargetVolume};
use protocol::OperationDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Collections
/// 🪪️ Stable-id accessor shared by every id-keyed document collection entry.
pub(crate) trait Puzzle3dHasId {
    fn id(&self) -> &str;
}
impl Puzzle3dHasId for Puzzle3dObject {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Puzzle3dHasId for Puzzle3dAttraction {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Puzzle3dHasId for Puzzle3dTargetVolume {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Puzzle3dHasId for Puzzle3dReference {
    fn id(&self) -> &str {
        &self.id
    }
}

/// 🩹️ Sparse id-keyed collection diff — removals plus id-or-index `set`s (replace when the id
/// already exists, else insert at the recorded index).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dObjectsDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, Puzzle3dObject)>,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dAttractionsDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, Puzzle3dAttraction)>,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dTargetVolumesDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, Puzzle3dTargetVolume)>,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dReferencesDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, Puzzle3dReference)>,
}

fn apply_puzzle3d_collection_diff<T: Puzzle3dHasId + Clone>(items: &mut Vec<T>, removed: &[String], set: &[(usize, T)]) {
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

pub(crate) fn puzzle3d_index_of<T: Puzzle3dHasId>(items: &[T], id: &str) -> Option<usize> {
    items.iter().position(|item| item.id() == id)
}
//#endregion 🔖️Collections

//#region 🔖️Diff
/// 🩹️ Sparse puzzle-3d diff over every id-keyed collection plus the scalar meta. Camera is
/// intentionally absent — it is session-only per-window runtime state, never a document operation.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dDiff {
    /// 🌍️ Whole-document replacement (example load, engine fill, layout); wins over every field below.
    pub document: Option<Puzzle3dProjection>,
    pub objects: Puzzle3dObjectsDiff,
    pub attractions: Puzzle3dAttractionsDiff,
    pub target_volumes: Puzzle3dTargetVolumesDiff,
    pub references: Puzzle3dReferencesDiff,
    pub meta: Option<Puzzle3dMeta>,
}

pub(crate) fn puzzle3d_diff_absorb(diff: &mut Puzzle3dDiff, other: Puzzle3dDiff) {
    if other.document.is_some() {
        *diff = Puzzle3dDiff { document: other.document, ..Default::default() };
        return;
    }
    diff.objects.removed.extend(other.objects.removed);
    diff.objects.set.extend(other.objects.set);
    diff.attractions.removed.extend(other.attractions.removed);
    diff.attractions.set.extend(other.attractions.set);
    diff.target_volumes.removed.extend(other.target_volumes.removed);
    diff.target_volumes.set.extend(other.target_volumes.set);
    diff.references.removed.extend(other.references.removed);
    diff.references.set.extend(other.references.set);
    if other.meta.is_some() {
        diff.meta = other.meta;
    }
}

impl OperationDiff<Puzzle3dProjection> for Puzzle3dDiff {
    fn apply(&self, projection: &Puzzle3dProjection) -> Puzzle3dProjection {
        if let Some(document) = &self.document {
            return document.clone();
        }
        let mut next = projection.clone();
        apply_puzzle3d_collection_diff(&mut next.objects, &self.objects.removed, &self.objects.set);
        apply_puzzle3d_collection_diff(&mut next.attractions, &self.attractions.removed, &self.attractions.set);
        apply_puzzle3d_collection_diff(&mut next.target_volumes, &self.target_volumes.removed, &self.target_volumes.set);
        apply_puzzle3d_collection_diff(&mut next.references, &self.references.removed, &self.references.set);
        if let Some(meta) = &self.meta {
            next.meta = meta.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        puzzle3d_diff_absorb(self, other);
    }
}
//#endregion 🔖️Diff
