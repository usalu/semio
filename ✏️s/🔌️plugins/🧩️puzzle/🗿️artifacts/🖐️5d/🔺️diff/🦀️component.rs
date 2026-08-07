//! 🔺️ Puzzle 5d artifact — the sparse diff over the typed projection: id-keyed part/fastener
//! collection deltas plus the scalar meta, and the whole-document replacement that wins over them.
//! The `serde_json::Value` and `Puzzle5dPlayProjection` bridge impls of the same diff live in `🔧️op`
//! beside the newtypes they patch. Camera pose is session-only app runtime state, never part of this
//! diff — see the app's `🦀️config.rs`.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::puzzle5d::{Puzzle5dFastener, Puzzle5dMeta, Puzzle5dPart, Puzzle5dProjection};
use protocol::OperationDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Collections
/// 🪪️ Stable-id accessor shared by every id-keyed document collection entry.
pub(crate) trait Puzzle5dHasId {
    fn id(&self) -> &str;
}
impl Puzzle5dHasId for Puzzle5dPart {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Puzzle5dHasId for Puzzle5dFastener {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dPartsDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, Puzzle5dPart)>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dFastenersDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, Puzzle5dFastener)>,
}

pub(crate) fn apply_puzzle5d_collection_diff<T: Puzzle5dHasId + Clone>(items: &mut Vec<T>, removed: &[String], set: &[(usize, T)]) {
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

pub(crate) fn puzzle5d_index_of<T: Puzzle5dHasId>(items: &[T], id: &str) -> Option<usize> {
    items.iter().position(|item| item.id() == id)
}
//#endregion 🔖️Collections

//#region 🔖️Diff
/// 🩹️ Sparse puzzle-5d diff over both id-keyed collections plus the scalar meta. Camera pose is
/// session-only app runtime state, never part of this diff — see the app's `Puzzle5dConfig`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dDiff {
    /// 🌍️ Whole-document replacement (example load, engine fill, layout); wins over every field below.
    pub document: Option<Puzzle5dProjection>,
    pub parts: Puzzle5dPartsDiff,
    pub fasteners: Puzzle5dFastenersDiff,
    pub meta: Option<Puzzle5dMeta>,
}

pub(crate) fn puzzle5d_diff_absorb(diff: &mut Puzzle5dDiff, other: Puzzle5dDiff) {
    if other.document.is_some() {
        *diff = Puzzle5dDiff { document: other.document, ..Default::default() };
        return;
    }
    diff.parts.removed.extend(other.parts.removed);
    diff.parts.set.extend(other.parts.set);
    diff.fasteners.removed.extend(other.fasteners.removed);
    diff.fasteners.set.extend(other.fasteners.set);
    if other.meta.is_some() {
        diff.meta = other.meta;
    }
}

impl OperationDiff<Puzzle5dProjection> for Puzzle5dDiff {
    fn apply(&self, projection: &Puzzle5dProjection) -> Puzzle5dProjection {
        if let Some(document) = &self.document {
            return document.clone();
        }
        let mut next = projection.clone();
        apply_puzzle5d_collection_diff(&mut next.parts, &self.parts.removed, &self.parts.set);
        apply_puzzle5d_collection_diff(&mut next.fasteners, &self.fasteners.removed, &self.fasteners.set);
        if let Some(meta) = &self.meta {
            next.meta = meta.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        puzzle5d_diff_absorb(self, other);
    }
}
//#endregion 🔖️Diff
