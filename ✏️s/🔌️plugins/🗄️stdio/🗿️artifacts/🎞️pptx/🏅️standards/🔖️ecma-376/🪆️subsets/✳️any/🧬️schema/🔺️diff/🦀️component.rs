//! 🔺️ PptxDiff — handcrafted sparse diff over `PptxSnapshot` (`opc: OpcPackage` +
//! `presentation: PptxPresentation`). No `snapshot: Option<PptxSnapshot>` full-replace slot --
//! even `SetSnapshot`'s diff is the sparse field-by-field `PptxDiff::between(base, next)`.
//!
//! `presentation.slides` is index-keyed (slide ORDER matters); within a modified slide,
//! `shapes` is index-keyed too -- both via the same `IndexedTripleDiff<D, T>` generic engine
//! `📜️docx` established for its own `body`/`runs` collections (own copy here, per that wave's
//! own `glue_followup` note: the ownership boundary keeps each artifact from touching a shared
//! `zip::opc`-adjacent location this wave, so each OOXML sibling carries its own copy for now).
//! `PptxShapeDiff` covers `TextBox`/`Picture`/`Placeholder` with real per-field diffs and a
//! `Replace{shape}` fallback on shape-KIND change (same "Replace on kind change" rule as
//! json/xml/dxf/docx's `DocxBlockDiff`).
//!
//! **OPC diff placement**: same as docx (see `f4-docx-report.md`) -- `zip::opc::OpcPackage` has
//! no diff type of its own yet; `PptxOpc*Diff` is defined HERE (own-file ownership boundary),
//! flagged in `glue_followup` for hoisting to `zip::opc` once xlsx/bcf need the identical shape
//! too (docx already flagged the same hoist).

use crate::artifacts::pptx::schema::snapshot::{PptxParagraph, PptxPresentation, PptxRun, PptxShape, PptxSlide, PptxTransform, PptxXmlPart};
use crate::artifacts::pptx::PptxSnapshot;
use crate::artifacts::xml::schema::snapshot::XmlNode;
use crate::artifacts::zip::opc::{OpcContentTypes, OpcPackage, OpcPart, OpcRelationship, OpcTargetMode};
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//#region 🔖️GenericCollectionTriples
/// 🌳 Index-keyed collection triple, generic over the item type `T` and its per-field diff type
/// `D`. `removed`/`modified` indices refer to BASE state (descending removal order on apply);
/// `added` indices refer to FINAL state (ascending insert, `min(index, len)`).
// 🩹 `bound(...)` overrides serde's default per-field-`default` bound inference (a known
// serde_derive limitation -- see `📜️docx`'s identical note): the real requirement is only
// `Serialize`/`Deserialize` on the item types, not `Default`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", bound(serialize = "D: Serialize, T: Serialize", deserialize = "D: Deserialize<'de>, T: Deserialize<'de>"))]
pub struct IndexedTripleDiff<D, T> {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<IndexModified<D>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<IndexAdded<T>>,
}

impl<D, T> Default for IndexedTripleDiff<D, T> {
    fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexModified<D> {
    pub index: usize,
    pub diff: D,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexAdded<T> {
    pub index: usize,
    pub item: T,
}

/// 🏷️ Name/key-keyed collection triple, generic over key `K`, item `T`, and per-field diff `D`.
/// `added` carries the full item (which already contains its own key).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", bound(serialize = "K: Serialize, D: Serialize, T: Serialize", deserialize = "K: Deserialize<'de>, D: Deserialize<'de>, T: Deserialize<'de>"))]
pub struct NamedTripleDiff<K, D, T> {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<K>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<NamedModified<K, D>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<T>,
}

impl<K, D, T> Default for NamedTripleDiff<K, D, T> {
    fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedModified<K, D> {
    pub key: K,
    pub diff: D,
}
//#endregion 🔖️GenericCollectionTriples

//#region 🔖️PresentationDiffTypes
pub type PptxSlidesDiff = IndexedTripleDiff<PptxSlideDiff, PptxSlide>;
pub type PptxShapesDiff = IndexedTripleDiff<PptxShapeDiff, PptxShape>;
pub type PptxParagraphsDiff = IndexedTripleDiff<PptxParagraphDiff, PptxParagraph>;
pub type PptxRunsDiff = IndexedTripleDiff<PptxRunDiff, PptxRun>;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PptxSlideDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shapes: Option<PptxShapesDiff>,
}

/// 🌳 Per-shape diff, shaped like `PptxShape` (`TextBox`/`Picture`/`Placeholder` each get real
/// field diffs; `Replace` covers a shape-KIND change, incl. anything involving `Other`, whose
/// logical XML node is never sub-diffed).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum PptxShapeDiff {
    TextBox(PptxTextBoxDiff),
    Picture(PptxPictureDiff),
    Placeholder(PptxPlaceholderDiff),
    Replace { shape: PptxShape },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PptxTextBoxDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_frame: Option<PptxParagraphsDiff>,
    /// 🏷️ Weak (value) entity per the recipe: whole-value replaced, never sub-diffed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<PptxTransform>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PptxPictureDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blip_rel_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<PptxTransform>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PptxPlaceholderDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_frame: Option<PptxParagraphsDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<PptxTransform>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PptxParagraphDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runs: Option<PptxRunsDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PptxRunDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    /// 🏳️ Tri-state: `None` = unchanged, `Some(None)` = font_size cleared, `Some(Some(sz))` = set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font_size: Option<Option<u32>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PptxPresentationDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slides: Option<PptxSlidesDiff>,
}
//#endregion 🔖️PresentationDiffTypes

//#region 🔖️OpcDiffTypes
pub type PptxOpcCtEntriesDiff = NamedTripleDiff<String, String, (String, String)>;
pub type PptxOpcPartsDiff = NamedTripleDiff<String, PptxOpcPartDiff, OpcPart>;
pub type PptxOpcRelListDiff = NamedTripleDiff<String, PptxOpcRelDiff, OpcRelationship>;
pub type PptxOpcRelationshipsDiff = NamedTripleDiff<String, PptxOpcRelListDiff, (String, Vec<OpcRelationship>)>;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PptxOpcContentTypesDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub defaults: Option<PptxOpcCtEntriesDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overrides: Option<PptxOpcCtEntriesDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PptxOpcPartDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PptxOpcRelDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rel_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_mode: Option<OpcTargetMode>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PptxOpcDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_types: Option<PptxOpcContentTypesDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parts: Option<PptxOpcPartsDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationships: Option<PptxOpcRelationshipsDiff>,
}
//#endregion 🔖️OpcDiffTypes

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.pptx`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pptx.diff")]
pub struct PptxDiff {
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opc: Option<PptxOpcDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation: Option<PptxPresentationDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub xml_parts: Option<Vec<PptxXmlPart>>,
}
//#endregion 🔖️Diff

//#region 🔖️GenericIndexedEngine
/// 🧮️ Between (positional, per the recipe's index-keyed matching rule): pairwise-compares
/// `0..min(base,other)` as `modified`, base tail as `removed`, other tail as `added`.
fn between_indexed<T, D>(base: &[T], other: &[T], diff_item: impl Fn(&T, &T) -> Option<D>) -> Option<IndexedTripleDiff<D, T>>
where
    T: Clone + PartialEq,
{
    let min_len = base.len().min(other.len());
    let mut modified = Vec::new();
    for i in 0..min_len {
        if base[i] != other[i] {
            if let Some(d) = diff_item(&base[i], &other[i]) {
                modified.push(IndexModified { index: i, diff: d });
            }
        }
    }
    let removed: Vec<usize> = (other.len()..base.len()).collect();
    let added: Vec<IndexAdded<T>> = (min_len..other.len()).map(|i| IndexAdded { index: i, item: other[i].clone() }).collect();
    if modified.is_empty() && removed.is_empty() && added.is_empty() {
        None
    } else {
        Some(IndexedTripleDiff { removed, modified, added })
    }
}

fn apply_indexed<T, D>(items: &mut Vec<T>, diff: &IndexedTripleDiff<D, T>, apply_item: impl Fn(&mut T, &D) -> MutationApplyResult<()>) -> MutationApplyResult<()>
where
    T: Clone,
{
    let mut removed = std::collections::HashSet::new();
    for &index in &diff.removed {
        if index >= items.len() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "indexed removal target does not exist"));
        }
        if !removed.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "indexed removal target is repeated"));
        }
    }
    let mut modified = std::collections::HashSet::new();
    for m in &diff.modified {
        if m.index >= items.len() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "indexed modification target does not exist"));
        }
        if removed.contains(&m.index) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "indexed modification targets a removed item"));
        }
        if !modified.insert(m.index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "indexed modification target is repeated"));
        }
    }
    let final_len = items.len() - removed.len() + diff.added.len();
    let mut added = std::collections::HashSet::new();
    for add in &diff.added {
        if add.index >= final_len {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "indexed addition is outside the final collection"));
        }
        if !added.insert(add.index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "indexed addition occupies a repeated final position"));
        }
    }
    for m in &diff.modified {
        apply_item(&mut items[m.index], &m.diff).map_err(|error| error.under(["modified"]))?;
    }
    let mut removed_sorted = diff.removed.clone();
    removed_sorted.sort_unstable_by(|a, b| b.cmp(a));
    for idx in removed_sorted {
        items.remove(idx);
    }
    let mut additions: Vec<&IndexAdded<T>> = diff.added.iter().collect();
    additions.sort_by_key(|a| a.index);
    for add in additions {
        items.insert(add.index, add.item.clone());
    }
    Ok(())
}

fn inverse_indexed<T, D>(base_items: &[T], diff: &IndexedTripleDiff<D, T>, inverse_item: impl Fn(&T, &D) -> D) -> IndexedTripleDiff<D, T>
where
    T: Clone,
{
    let removed: Vec<usize> = diff.added.iter().map(|a| a.index).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(original) = base_items.get(m.index) {
            let next_index = transform_index(m.index, &diff.removed, &diff.added);
            modified.push(IndexModified { index: next_index, diff: inverse_item(original, &m.diff) });
        }
    }
    let mut added = Vec::new();
    for &idx in &diff.removed {
        if let Some(original) = base_items.get(idx) {
            added.push(IndexAdded { index: idx, item: original.clone() });
        }
    }
    added.sort_by_key(|a| a.index);
    IndexedTripleDiff { removed, modified, added }
}

/// 🧮️ Maps a base-side index through a diff's OWN removed/added to the position it ends up at
/// once that diff has been applied (svg `SvgDiff`'s `transform_index`, generalized).
fn transform_index<T>(idx: usize, removed: &[usize], added: &[IndexAdded<T>]) -> usize {
    let removed_before = removed.iter().filter(|&&r| r < idx).count();
    let pos = idx - removed_before;
    let mut order: Vec<usize> = added.iter().map(|a| a.index).collect();
    order.sort_unstable();
    let mut shift = 0usize;
    for target in order {
        if target <= pos + shift {
            shift += 1;
        } else {
            break;
        }
    }
    pos + shift
}

enum ItemOrigin {
    Base(usize),
    Added(usize),
}

fn simulate_mid_origins<T>(base_len: usize, removed: &[usize], added: &[IndexAdded<T>]) -> Vec<ItemOrigin> {
    let mut mid: Vec<ItemOrigin> = (0..base_len).filter(|i| !removed.contains(i)).map(ItemOrigin::Base).collect();
    let mut order: Vec<(usize, usize)> = added.iter().enumerate().map(|(k, a)| (a.index, k)).collect();
    order.sort_by_key(|(idx, _)| *idx);
    for (idx, k) in order {
        let at = idx.min(mid.len());
        mid.insert(at, ItemOrigin::Added(k));
    }
    mid
}

/// 🧮️ Sequential-coalesce absorb per the recipe's normative algorithm (svg/docx precedent):
/// `absorb_item` recursively absorbs two per-field diffs of the SAME item; `apply_item` patches a
/// `D` onto a `T` (needed when `d2` modifies an item `d1` just added).
#[allow(clippy::too_many_arguments)]
fn absorb_indexed<T, D>(d1: IndexedTripleDiff<D, T>, d2: IndexedTripleDiff<D, T>, absorb_item: impl Fn(D, D) -> D, apply_item: impl Fn(&T, &D) -> T) -> IndexedTripleDiff<D, T>
where
    T: Clone,
    D: Clone,
{
    let d1_ref_max = d1.removed.iter().copied().chain(d1.modified.iter().map(|m| m.index)).max();
    let mut base_len = d1_ref_max.map(|m| m + 1).unwrap_or(0);
    let mid_len_needed_by_d1 = d1.added.iter().map(|a| a.index + 1).max().unwrap_or(0);
    while base_len.saturating_sub(d1.removed.len()) + d1.added.len() < mid_len_needed_by_d1 {
        base_len += 1;
    }
    let d2_ref_max = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).max();
    let required_mid_len = d2_ref_max.map(|m| m + 1).unwrap_or(0);
    while base_len.saturating_sub(d1.removed.len()) + d1.added.len() < required_mid_len {
        base_len += 1;
    }

    let mid = simulate_mid_origins(base_len, &d1.removed, &d1.added);

    let mut removed = d1.removed.clone();
    let mut modified = d1.modified;
    let mut working_added = d1.added;
    let mut annihilated: std::collections::HashSet<usize> = std::collections::HashSet::new();

    for &r2 in &d2.removed {
        match mid.get(r2) {
            Some(ItemOrigin::Base(bi)) => {
                if !removed.contains(bi) {
                    removed.push(*bi);
                }
                modified.retain(|m| &m.index != bi);
            }
            Some(ItemOrigin::Added(k)) => {
                annihilated.insert(*k);
            }
            None => {}
        }
    }
    for m2 in &d2.modified {
        match mid.get(m2.index) {
            Some(ItemOrigin::Base(bi)) => {
                if removed.contains(bi) {
                    continue;
                }
                match modified.iter_mut().find(|m| &m.index == bi) {
                    Some(existing) => existing.diff = absorb_item(existing.diff.clone(), m2.diff.clone()),
                    None => modified.push(IndexModified { index: *bi, diff: m2.diff.clone() }),
                }
            }
            Some(ItemOrigin::Added(k)) => {
                if annihilated.contains(k) {
                    continue;
                }
                if let Some(add) = working_added.get_mut(*k) {
                    add.item = apply_item(&add.item, &m2.diff);
                }
            }
            None => {}
        }
    }

    let mut added = Vec::new();
    for (k, add) in working_added.into_iter().enumerate() {
        if annihilated.contains(&k) {
            continue;
        }
        let final_index = transform_index(add.index, &d2.removed, &d2.added);
        added.push(IndexAdded { index: final_index, item: add.item });
    }
    for a2 in &d2.added {
        added.push(a2.clone());
    }
    added.sort_by_key(|a| a.index);

    IndexedTripleDiff { removed, modified, added }
}
//#endregion 🔖️GenericIndexedEngine

//#region 🔖️GenericNamedEngine
fn between_named<K, T, D>(base: &[T], other: &[T], key_of: impl Fn(&T) -> K, diff_item: impl Fn(&T, &T) -> Option<D>) -> Option<NamedTripleDiff<K, D, T>>
where
    K: PartialEq + Clone,
    T: Clone + PartialEq,
{
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for b in base {
        let bk = key_of(b);
        match other.iter().find(|o| key_of(o) == bk) {
            None => removed.push(bk),
            Some(o) if o != b => {
                if let Some(d) = diff_item(b, o) {
                    modified.push(NamedModified { key: bk, diff: d });
                }
            }
            Some(_) => {}
        }
    }
    let mut added = Vec::new();
    for o in other {
        let ok = key_of(o);
        if !base.iter().any(|b| key_of(b) == ok) {
            added.push(o.clone());
        }
    }
    if removed.is_empty() && modified.is_empty() && added.is_empty() {
        None
    } else {
        Some(NamedTripleDiff { removed, modified, added })
    }
}

fn apply_named<K, T, D>(items: &mut Vec<T>, diff: &NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, apply_item: impl Fn(&mut T, &D) -> MutationApplyResult<()>) -> MutationApplyResult<()>
where
    K: PartialEq + Clone,
    T: Clone,
{
    let keys: Vec<K> = items.iter().map(&key_of).collect();
    for (position, key) in diff.removed.iter().enumerate() {
        if !keys.contains(key) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "named removal target does not exist"));
        }
        if diff.removed[..position].contains(key) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "named removal target is repeated"));
        }
    }
    for (position, key) in diff.modified.iter().enumerate() {
        if !keys.contains(&key.key) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "named modification target does not exist"));
        }
        if diff.removed.contains(&key.key) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "named modification targets a removed item"));
        }
        if diff.modified[..position].iter().any(|candidate| candidate.key == key.key) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "named modification target is repeated"));
        }
    }
    for item in &diff.added {
        let key = key_of(item);
        if keys.contains(&key) || diff.added.iter().filter(|candidate| key_of(candidate) == key).count() != 1 {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "named addition target already exists"));
        }
    }
    items.retain(|i| !diff.removed.contains(&key_of(i)));
    for m in &diff.modified {
        let item = items.iter_mut().find(|i| key_of(i) == m.key).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "named modification target does not exist"))?;
        apply_item(item, &m.diff).map_err(|error| error.under(["modified"]))?;
    }
    for item in &diff.added {
        items.push(item.clone());
    }
    Ok(())
}

fn inverse_named<K, T, D>(base_items: &[T], diff: &NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, inverse_item: impl Fn(&T, &D) -> D) -> NamedTripleDiff<K, D, T>
where
    K: PartialEq + Clone,
    T: Clone,
{
    let removed: Vec<K> = diff.added.iter().map(&key_of).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(original) = base_items.iter().find(|i| key_of(i) == m.key) {
            modified.push(NamedModified { key: m.key.clone(), diff: inverse_item(original, &m.diff) });
        }
    }
    let mut added = Vec::new();
    for k in &diff.removed {
        if let Some(original) = base_items.iter().find(|i| &key_of(i) == k) {
            added.push(original.clone());
        }
    }
    NamedTripleDiff { removed, modified, added }
}

/// 🧮️ Name-keyed absorb -- identity is the KEY (not position), so no index transport is needed:
/// a `d2`-removal of a `d1`-added key annihilates the add; a `d2`-modify of a `d1`-added key
/// patches into the carried payload; everything else composes directly on the shared key space.
fn absorb_named<K, T, D>(d1: NamedTripleDiff<K, D, T>, d2: NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, absorb_item: impl Fn(D, D) -> D, apply_item: impl Fn(&mut T, &D)) -> NamedTripleDiff<K, D, T>
where
    K: PartialEq + Clone,
    T: Clone,
    D: Clone,
{
    let d1_added_keys: Vec<K> = d1.added.iter().map(&key_of).collect();
    let mut removed = d1.removed.clone();
    let mut annihilated: Vec<K> = Vec::new();
    for k in &d2.removed {
        if d1_added_keys.contains(k) {
            annihilated.push(k.clone());
        } else if !removed.contains(k) {
            removed.push(k.clone());
        }
    }
    let mut working_added: Vec<T> = d1.added.into_iter().filter(|a| !annihilated.contains(&key_of(a))).collect();
    let mut modified: Vec<NamedModified<K, D>> = d1.modified.into_iter().filter(|m| !removed.contains(&m.key)).collect();
    for m2 in &d2.modified {
        if let Some(added) = working_added.iter_mut().find(|a| key_of(a) == m2.key) {
            apply_item(added, &m2.diff);
            continue;
        }
        if removed.contains(&m2.key) {
            continue;
        }
        match modified.iter_mut().find(|m| m.key == m2.key) {
            Some(existing) => existing.diff = absorb_item(existing.diff.clone(), m2.diff.clone()),
            None => modified.push(NamedModified { key: m2.key.clone(), diff: m2.diff.clone() }),
        }
    }
    for a2 in &d2.added {
        let k2 = key_of(a2);
        match working_added.iter_mut().find(|a| key_of(a) == k2) {
            Some(existing) => *existing = a2.clone(),
            None => working_added.push(a2.clone()),
        }
    }
    NamedTripleDiff { removed, modified, added: working_added }
}
//#endregion 🔖️GenericNamedEngine

//#region 🔖️PresentationDiffLogic
fn diff_run(old: &PptxRun, new: &PptxRun) -> Option<PptxRunDiff> {
    if old == new {
        return None;
    }
    Some(PptxRunDiff {
        text: (old.text != new.text).then(|| new.text.clone()),
        bold: (old.bold != new.bold).then_some(new.bold),
        italic: (old.italic != new.italic).then_some(new.italic),
        font_size: (old.font_size != new.font_size).then_some(new.font_size),
    })
}

fn apply_run(run: &mut PptxRun, diff: &PptxRunDiff) -> MutationApplyResult<()> {
    if let Some(v) = &diff.text {
        run.text = v.clone();
    }
    if let Some(v) = diff.bold {
        run.bold = v;
    }
    if let Some(v) = diff.italic {
        run.italic = v;
    }
    if let Some(v) = diff.font_size {
        run.font_size = v;
    }
    Ok(())
}

fn inverse_run(base: &PptxRun, diff: &PptxRunDiff) -> PptxRunDiff {
    PptxRunDiff { text: diff.text.as_ref().map(|_| base.text.clone()), bold: diff.bold.map(|_| base.bold), italic: diff.italic.map(|_| base.italic), font_size: diff.font_size.map(|_| base.font_size) }
}

fn absorb_run_diff(a: PptxRunDiff, b: PptxRunDiff) -> PptxRunDiff {
    PptxRunDiff { text: b.text.or(a.text), bold: b.bold.or(a.bold), italic: b.italic.or(a.italic), font_size: b.font_size.or(a.font_size) }
}

fn run_with_diff_applied(run: &PptxRun, diff: &PptxRunDiff) -> PptxRun {
    let mut out = run.clone();
    apply_run_for_absorb(&mut out, diff);
    out
}

fn apply_run_for_absorb(run: &mut PptxRun, diff: &PptxRunDiff) {
    if let Some(value) = &diff.text {
        run.text = value.clone();
    }
    if let Some(value) = diff.bold {
        run.bold = value;
    }
    if let Some(value) = diff.italic {
        run.italic = value;
    }
    if let Some(value) = diff.font_size {
        run.font_size = value;
    }
}

fn diff_paragraph(old: &PptxParagraph, new: &PptxParagraph) -> Option<PptxParagraphDiff> {
    let runs = between_indexed(&old.runs, &new.runs, diff_run);
    if runs.is_none() {
        None
    } else {
        Some(PptxParagraphDiff { runs })
    }
}

fn apply_paragraph(p: &mut PptxParagraph, diff: &PptxParagraphDiff) -> MutationApplyResult<()> {
    if let Some(rd) = &diff.runs {
        apply_indexed(&mut p.runs, rd, apply_run).map_err(|error| error.under(["runs"]))?;
    }
    Ok(())
}

fn inverse_paragraph(base: &PptxParagraph, diff: &PptxParagraphDiff) -> PptxParagraphDiff {
    PptxParagraphDiff { runs: diff.runs.as_ref().map(|rd| inverse_indexed(&base.runs, rd, inverse_run)) }
}

fn absorb_paragraph_diff(mut a: PptxParagraphDiff, b: PptxParagraphDiff) -> PptxParagraphDiff {
    a.runs = match (a.runs.take(), b.runs) {
        (None, x) => x,
        (x, None) => x,
        (Some(ra), Some(rb)) => Some(absorb_indexed(ra, rb, absorb_run_diff, run_with_diff_applied)),
    };
    a
}

fn paragraph_with_diff_applied(p: &PptxParagraph, diff: &PptxParagraphDiff) -> PptxParagraph {
    let mut out = p.clone();
    apply_paragraph_for_absorb(&mut out, diff);
    out
}

fn apply_paragraph_for_absorb(paragraph: &mut PptxParagraph, diff: &PptxParagraphDiff) {
    if let Some(runs) = &diff.runs {
        apply_indexed_for_absorb(&mut paragraph.runs, runs, apply_run_for_absorb);
    }
}

fn diff_shape(old: &PptxShape, new: &PptxShape) -> Option<PptxShapeDiff> {
    if old == new {
        return None;
    }
    match (old, new) {
        (PptxShape::TextBox { text_frame: otf, position: op }, PptxShape::TextBox { text_frame: ntf, position: np }) => {
            let text_frame = between_indexed(otf, ntf, diff_paragraph);
            let position = (op != np).then_some(*np);
            if text_frame.is_none() && position.is_none() {
                None
            } else {
                Some(PptxShapeDiff::TextBox(PptxTextBoxDiff { text_frame, position }))
            }
        }
        (PptxShape::Picture { blip_rel_id: oid, position: op }, PptxShape::Picture { blip_rel_id: nid, position: np }) => {
            let blip_rel_id = (oid != nid).then(|| nid.clone());
            let position = (op != np).then_some(*np);
            if blip_rel_id.is_none() && position.is_none() {
                None
            } else {
                Some(PptxShapeDiff::Picture(PptxPictureDiff { blip_rel_id, position }))
            }
        }
        (PptxShape::Placeholder { kind: ok, text_frame: otf, position: op }, PptxShape::Placeholder { kind: nk, text_frame: ntf, position: np }) => {
            let kind = (ok != nk).then(|| nk.clone());
            let text_frame = between_indexed(otf, ntf, diff_paragraph);
            let position = (op != np).then_some(*np);
            if kind.is_none() && text_frame.is_none() && position.is_none() {
                None
            } else {
                Some(PptxShapeDiff::Placeholder(PptxPlaceholderDiff { kind, text_frame, position }))
            }
        }
        _ => Some(PptxShapeDiff::Replace { shape: new.clone() }),
    }
}

fn apply_shape(shape: &mut PptxShape, diff: &PptxShapeDiff) -> MutationApplyResult<()> {
    match diff {
        PptxShapeDiff::Replace { shape: new } => *shape = new.clone(),
        PptxShapeDiff::TextBox(td) => {
            let PptxShape::TextBox { text_frame, position } = shape else {
                return Err(MutationApplyError::new("mutation.apply.kind-mismatch", "text-box diff targets another shape kind"));
            };
            if let Some(tfd) = &td.text_frame {
                apply_indexed(text_frame, tfd, apply_paragraph).map_err(|error| error.under(["textFrame"]))?;
            }
            if let Some(p) = &td.position {
                *position = *p;
            }
        }
        PptxShapeDiff::Picture(pd) => {
            let PptxShape::Picture { blip_rel_id, position } = shape else {
                return Err(MutationApplyError::new("mutation.apply.kind-mismatch", "picture diff targets another shape kind"));
            };
            if let Some(v) = &pd.blip_rel_id {
                *blip_rel_id = v.clone();
            }
            if let Some(p) = &pd.position {
                *position = *p;
            }
        }
        PptxShapeDiff::Placeholder(phd) => {
            let PptxShape::Placeholder { kind, text_frame, position } = shape else {
                return Err(MutationApplyError::new("mutation.apply.kind-mismatch", "placeholder diff targets another shape kind"));
            };
            if let Some(k) = &phd.kind {
                *kind = k.clone();
            }
            if let Some(tfd) = &phd.text_frame {
                apply_indexed(text_frame, tfd, apply_paragraph).map_err(|error| error.under(["textFrame"]))?;
            }
            if let Some(p) = &phd.position {
                *position = *p;
            }
        }
    }
    Ok(())
}

fn shape_with_diff_applied(shape: &PptxShape, diff: &PptxShapeDiff) -> PptxShape {
    let mut out = shape.clone();
    apply_shape_for_absorb(&mut out, diff);
    out
}

fn apply_shape_for_absorb(shape: &mut PptxShape, diff: &PptxShapeDiff) {
    match diff {
        PptxShapeDiff::Replace { shape: new } => *shape = new.clone(),
        PptxShapeDiff::TextBox(change) => {
            if let PptxShape::TextBox { text_frame, position } = shape {
                if let Some(text_frame_diff) = &change.text_frame {
                    apply_indexed_for_absorb(text_frame, text_frame_diff, apply_paragraph_for_absorb);
                }
                if let Some(value) = change.position {
                    *position = value;
                }
            }
        }
        PptxShapeDiff::Picture(change) => {
            if let PptxShape::Picture { blip_rel_id, position } = shape {
                if let Some(value) = &change.blip_rel_id {
                    *blip_rel_id = value.clone();
                }
                if let Some(value) = change.position {
                    *position = value;
                }
            }
        }
        PptxShapeDiff::Placeholder(change) => {
            if let PptxShape::Placeholder { kind, text_frame, position } = shape {
                if let Some(value) = &change.kind {
                    *kind = value.clone();
                }
                if let Some(text_frame_diff) = &change.text_frame {
                    apply_indexed_for_absorb(text_frame, text_frame_diff, apply_paragraph_for_absorb);
                }
                if let Some(value) = change.position {
                    *position = value;
                }
            }
        }
    }
}

fn inverse_shape(base: &PptxShape, diff: &PptxShapeDiff) -> PptxShapeDiff {
    match diff {
        PptxShapeDiff::Replace { .. } => PptxShapeDiff::Replace { shape: base.clone() },
        PptxShapeDiff::TextBox(td) => {
            let PptxShape::TextBox { text_frame, position } = base else { return PptxShapeDiff::Replace { shape: base.clone() } };
            PptxShapeDiff::TextBox(PptxTextBoxDiff { text_frame: td.text_frame.as_ref().map(|tfd| inverse_indexed(text_frame, tfd, inverse_paragraph)), position: td.position.as_ref().map(|_| *position) })
        }
        PptxShapeDiff::Picture(pd) => {
            let PptxShape::Picture { blip_rel_id, position } = base else { return PptxShapeDiff::Replace { shape: base.clone() } };
            PptxShapeDiff::Picture(PptxPictureDiff { blip_rel_id: pd.blip_rel_id.as_ref().map(|_| blip_rel_id.clone()), position: pd.position.as_ref().map(|_| *position) })
        }
        PptxShapeDiff::Placeholder(phd) => {
            let PptxShape::Placeholder { kind, text_frame, position } = base else { return PptxShapeDiff::Replace { shape: base.clone() } };
            PptxShapeDiff::Placeholder(PptxPlaceholderDiff {
                kind: phd.kind.as_ref().map(|_| kind.clone()),
                text_frame: phd.text_frame.as_ref().map(|tfd| inverse_indexed(text_frame, tfd, inverse_paragraph)),
                position: phd.position.as_ref().map(|_| *position),
            })
        }
    }
}

fn absorb_shape_diff(a: PptxShapeDiff, b: PptxShapeDiff) -> PptxShapeDiff {
    match (a, b) {
        (_, PptxShapeDiff::Replace { shape }) => PptxShapeDiff::Replace { shape },
        (PptxShapeDiff::Replace { shape }, b) => PptxShapeDiff::Replace { shape: shape_with_diff_applied(&shape, &b) },
        (PptxShapeDiff::TextBox(ta), PptxShapeDiff::TextBox(tb)) => PptxShapeDiff::TextBox(absorb_text_box_diff(ta, tb)),
        (PptxShapeDiff::Picture(pa), PptxShapeDiff::Picture(pb)) => PptxShapeDiff::Picture(absorb_picture_diff(pa, pb)),
        (PptxShapeDiff::Placeholder(pa), PptxShapeDiff::Placeholder(pb)) => PptxShapeDiff::Placeholder(absorb_placeholder_diff(pa, pb)),
        (_, b) => b,
    }
}

fn absorb_text_box_diff(mut a: PptxTextBoxDiff, b: PptxTextBoxDiff) -> PptxTextBoxDiff {
    if b.position.is_some() {
        a.position = b.position;
    }
    a.text_frame = match (a.text_frame.take(), b.text_frame) {
        (None, x) => x,
        (x, None) => x,
        (Some(ta), Some(tb)) => Some(absorb_indexed(ta, tb, absorb_paragraph_diff, paragraph_with_diff_applied)),
    };
    a
}

fn absorb_picture_diff(mut a: PptxPictureDiff, b: PptxPictureDiff) -> PptxPictureDiff {
    if b.blip_rel_id.is_some() {
        a.blip_rel_id = b.blip_rel_id;
    }
    if b.position.is_some() {
        a.position = b.position;
    }
    a
}

fn absorb_placeholder_diff(mut a: PptxPlaceholderDiff, b: PptxPlaceholderDiff) -> PptxPlaceholderDiff {
    if b.kind.is_some() {
        a.kind = b.kind;
    }
    if b.position.is_some() {
        a.position = b.position;
    }
    a.text_frame = match (a.text_frame.take(), b.text_frame) {
        (None, x) => x,
        (x, None) => x,
        (Some(ta), Some(tb)) => Some(absorb_indexed(ta, tb, absorb_paragraph_diff, paragraph_with_diff_applied)),
    };
    a
}

fn diff_slide(old: &PptxSlide, new: &PptxSlide) -> Option<PptxSlideDiff> {
    let shapes = between_indexed(&old.shapes, &new.shapes, diff_shape);
    if shapes.is_none() {
        None
    } else {
        Some(PptxSlideDiff { shapes })
    }
}

fn apply_slide(slide: &mut PptxSlide, diff: &PptxSlideDiff) -> MutationApplyResult<()> {
    if let Some(sd) = &diff.shapes {
        apply_indexed(&mut slide.shapes, sd, apply_shape).map_err(|error| error.under(["shapes"]))?;
    }
    Ok(())
}

fn slide_with_diff_applied(slide: &PptxSlide, diff: &PptxSlideDiff) -> PptxSlide {
    let mut out = slide.clone();
    apply_slide_for_absorb(&mut out, diff);
    out
}

fn apply_slide_for_absorb(slide: &mut PptxSlide, diff: &PptxSlideDiff) {
    if let Some(shapes) = &diff.shapes {
        apply_indexed_for_absorb(&mut slide.shapes, shapes, apply_shape_for_absorb);
    }
}

fn apply_indexed_for_absorb<T, D>(items: &mut Vec<T>, diff: &IndexedTripleDiff<D, T>, apply_item: impl Fn(&mut T, &D))
where
    T: Clone,
{
    for modified in &diff.modified {
        apply_item(&mut items[modified.index], &modified.diff);
    }
    let mut removed = diff.removed.clone();
    removed.sort_unstable_by(|a, b| b.cmp(a));
    for index in removed {
        items.remove(index);
    }
    let mut added: Vec<&IndexAdded<T>> = diff.added.iter().collect();
    added.sort_by_key(|item| item.index);
    for item in added {
        items.insert(item.index, item.item.clone());
    }
}

fn inverse_slide(base: &PptxSlide, diff: &PptxSlideDiff) -> PptxSlideDiff {
    PptxSlideDiff { shapes: diff.shapes.as_ref().map(|sd| inverse_indexed(&base.shapes, sd, inverse_shape)) }
}

fn absorb_slide_diff(mut a: PptxSlideDiff, b: PptxSlideDiff) -> PptxSlideDiff {
    a.shapes = match (a.shapes.take(), b.shapes) {
        (None, x) => x,
        (x, None) => x,
        (Some(sa), Some(sb)) => Some(absorb_indexed(sa, sb, absorb_shape_diff, shape_with_diff_applied)),
    };
    a
}

fn diff_presentation(base: &PptxPresentation, other: &PptxPresentation) -> Option<PptxPresentationDiff> {
    let slides = between_indexed(&base.slides, &other.slides, diff_slide);
    if slides.is_none() {
        None
    } else {
        Some(PptxPresentationDiff { slides })
    }
}

fn apply_presentation_diff(presentation: &mut PptxPresentation, diff: &PptxPresentationDiff) -> MutationApplyResult<()> {
    if let Some(sd) = &diff.slides {
        apply_indexed(&mut presentation.slides, sd, apply_slide).map_err(|error| error.under(["slides"]))?;
    }
    Ok(())
}

fn inverse_presentation_diff(base: &PptxPresentation, diff: &PptxPresentationDiff) -> PptxPresentationDiff {
    PptxPresentationDiff { slides: diff.slides.as_ref().map(|sd| inverse_indexed(&base.slides, sd, inverse_slide)) }
}

fn absorb_presentation_diff(a: PptxPresentationDiff, b: PptxPresentationDiff) -> PptxPresentationDiff {
    PptxPresentationDiff {
        slides: match (a.slides, b.slides) {
            (None, x) => x,
            (x, None) => x,
            (Some(sa), Some(sb)) => Some(absorb_indexed(sa, sb, absorb_slide_diff, slide_with_diff_applied)),
        },
    }
}
//#endregion 🔖️PresentationDiffLogic

//#region 🔖️OpcDiffLogic
fn diff_ct_entries(old: &[(String, String)], new: &[(String, String)]) -> Option<PptxOpcCtEntriesDiff> {
    between_named(old, new, |(k, _)| k.clone(), |(_, ov), (_, nv)| (ov != nv).then(|| nv.clone()))
}

fn apply_ct_entries(entries: &mut Vec<(String, String)>, diff: &PptxOpcCtEntriesDiff) -> MutationApplyResult<()> {
    apply_named(
        entries,
        diff,
        |(k, _)| k.clone(),
        |(_, value), next| {
            *value = next.clone();
            Ok(())
        },
    )
}

fn inverse_ct_entries(base: &[(String, String)], diff: &PptxOpcCtEntriesDiff) -> PptxOpcCtEntriesDiff {
    inverse_named(base, diff, |(k, _)| k.clone(), |(_, v), _| v.clone())
}

fn absorb_ct_entries(a: PptxOpcCtEntriesDiff, b: PptxOpcCtEntriesDiff) -> PptxOpcCtEntriesDiff {
    // 🏷️ `D = String` here is already a whole-value replace (LWW) -- absorbing two such diffs on
    // the SAME key is just "the later one wins", i.e. `b`.
    absorb_named(a, b, |(k, _)| k.clone(), |_av, bv| bv, |(_, v), nv| *v = nv.clone())
}

fn diff_content_types(old: &OpcContentTypes, new: &OpcContentTypes) -> Option<PptxOpcContentTypesDiff> {
    let defaults = diff_ct_entries(&old.defaults, &new.defaults);
    let overrides = diff_ct_entries(&old.overrides, &new.overrides);
    if defaults.is_none() && overrides.is_none() {
        None
    } else {
        Some(PptxOpcContentTypesDiff { defaults, overrides })
    }
}

fn diff_part(old: &OpcPart, new: &OpcPart) -> Option<PptxOpcPartDiff> {
    if old == new {
        return None;
    }
    Some(PptxOpcPartDiff { content_type: (old.content_type != new.content_type).then(|| new.content_type.clone()), bytes: (old.bytes != new.bytes).then(|| new.bytes.clone()) })
}

fn apply_part(part: &mut OpcPart, diff: &PptxOpcPartDiff) {
    if let Some(v) = &diff.content_type {
        part.content_type = v.clone();
    }
    if let Some(v) = &diff.bytes {
        part.bytes = v.clone();
    }
}

fn part_with_diff_applied(part: &OpcPart, diff: &PptxOpcPartDiff) -> OpcPart {
    let mut out = part.clone();
    apply_part(&mut out, diff);
    out
}

fn inverse_part(base: &OpcPart, diff: &PptxOpcPartDiff) -> PptxOpcPartDiff {
    PptxOpcPartDiff { content_type: diff.content_type.as_ref().map(|_| base.content_type.clone()), bytes: diff.bytes.as_ref().map(|_| base.bytes.clone()) }
}

fn absorb_part_diff(mut a: PptxOpcPartDiff, b: PptxOpcPartDiff) -> PptxOpcPartDiff {
    if b.content_type.is_some() {
        a.content_type = b.content_type;
    }
    if b.bytes.is_some() {
        a.bytes = b.bytes;
    }
    a
}

fn diff_parts(old: &[OpcPart], new: &[OpcPart]) -> Option<PptxOpcPartsDiff> {
    between_named(old, new, |p| p.path.clone(), diff_part)
}

fn diff_rel(old: &OpcRelationship, new: &OpcRelationship) -> Option<PptxOpcRelDiff> {
    if old == new {
        return None;
    }
    Some(PptxOpcRelDiff { rel_type: (old.rel_type != new.rel_type).then(|| new.rel_type.clone()), target: (old.target != new.target).then(|| new.target.clone()), target_mode: (old.target_mode != new.target_mode).then_some(new.target_mode) })
}

fn apply_rel(rel: &mut OpcRelationship, diff: &PptxOpcRelDiff) {
    if let Some(v) = &diff.rel_type {
        rel.rel_type = v.clone();
    }
    if let Some(v) = &diff.target {
        rel.target = v.clone();
    }
    if let Some(v) = diff.target_mode {
        rel.target_mode = v;
    }
}

fn inverse_rel(base: &OpcRelationship, diff: &PptxOpcRelDiff) -> PptxOpcRelDiff {
    PptxOpcRelDiff { rel_type: diff.rel_type.as_ref().map(|_| base.rel_type.clone()), target: diff.target.as_ref().map(|_| base.target.clone()), target_mode: diff.target_mode.map(|_| base.target_mode) }
}

fn absorb_rel_diff(mut a: PptxOpcRelDiff, b: PptxOpcRelDiff) -> PptxOpcRelDiff {
    if b.rel_type.is_some() {
        a.rel_type = b.rel_type;
    }
    if b.target.is_some() {
        a.target = b.target;
    }
    if b.target_mode.is_some() {
        a.target_mode = b.target_mode;
    }
    a
}

fn diff_rel_list(old: &[OpcRelationship], new: &[OpcRelationship]) -> Option<PptxOpcRelListDiff> {
    between_named(old, new, |r| r.id.clone(), diff_rel)
}

fn apply_rel_list(list: &mut Vec<OpcRelationship>, diff: &PptxOpcRelListDiff) -> MutationApplyResult<()> {
    apply_named(
        list,
        diff,
        |r| r.id.clone(),
        |relationship, change| {
            apply_rel(relationship, change);
            Ok(())
        },
    )
}

fn rel_list_with_diff_applied(list: &[OpcRelationship], diff: &PptxOpcRelListDiff) -> Vec<OpcRelationship> {
    let mut out = list.to_vec();
    out.retain(|relationship| !diff.removed.contains(&relationship.id));
    for modified in &diff.modified {
        if let Some(relationship) = out.iter_mut().find(|relationship| relationship.id == modified.key) {
            apply_rel(relationship, &modified.diff);
        }
    }
    out.extend(diff.added.iter().cloned());
    out
}

fn inverse_rel_list(base: &[OpcRelationship], diff: &PptxOpcRelListDiff) -> PptxOpcRelListDiff {
    inverse_named(base, diff, |r| r.id.clone(), inverse_rel)
}

fn absorb_rel_list_diff(a: PptxOpcRelListDiff, b: PptxOpcRelListDiff) -> PptxOpcRelListDiff {
    absorb_named(a, b, |r| r.id.clone(), absorb_rel_diff, apply_rel)
}

fn diff_relationships(old: &HashMap<String, Vec<OpcRelationship>>, new: &HashMap<String, Vec<OpcRelationship>>) -> Option<PptxOpcRelationshipsDiff> {
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for (owner, list) in old {
        match new.get(owner) {
            None => removed.push(owner.clone()),
            Some(nlist) => {
                if let Some(d) = diff_rel_list(list, nlist) {
                    modified.push(NamedModified { key: owner.clone(), diff: d });
                }
            }
        }
    }
    let mut added = Vec::new();
    for (owner, list) in new {
        if !old.contains_key(owner) {
            added.push((owner.clone(), list.clone()));
        }
    }
    if removed.is_empty() && modified.is_empty() && added.is_empty() {
        None
    } else {
        Some(PptxOpcRelationshipsDiff { removed, modified, added })
    }
}

fn apply_relationships(rels: &mut HashMap<String, Vec<OpcRelationship>>, diff: &PptxOpcRelationshipsDiff) -> MutationApplyResult<()> {
    for (position, owner) in diff.removed.iter().enumerate() {
        if !rels.contains_key(owner) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "relationship owner does not exist"));
        }
        if diff.removed[..position].contains(owner) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "relationship owner removal is repeated"));
        }
    }
    for (position, m) in diff.modified.iter().enumerate() {
        if !rels.contains_key(&m.key) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "relationship owner does not exist"));
        }
        if diff.removed.contains(&m.key) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "relationship owner is both removed and modified"));
        }
        if diff.modified[..position].iter().any(|candidate| candidate.key == m.key) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "relationship owner modification is repeated"));
        }
    }
    for (position, (owner, _)) in diff.added.iter().enumerate() {
        if rels.contains_key(owner) || diff.added[..position].iter().any(|(candidate, _)| candidate == owner) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "relationship owner already exists"));
        }
        if diff.removed.contains(owner) || diff.modified.iter().any(|candidate| candidate.key == *owner) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "relationship owner is both changed and added"));
        }
    }
    for owner in &diff.removed {
        rels.remove(owner);
    }
    for m in &diff.modified {
        let list = rels.get_mut(&m.key).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "relationship owner does not exist"))?;
        apply_rel_list(list, &m.diff).map_err(|error| error.under(["modified"]))?;
    }
    for (owner, list) in &diff.added {
        rels.insert(owner.clone(), list.clone());
    }
    Ok(())
}

fn inverse_relationships(base: &HashMap<String, Vec<OpcRelationship>>, diff: &PptxOpcRelationshipsDiff) -> PptxOpcRelationshipsDiff {
    let removed: Vec<String> = diff.added.iter().map(|(owner, _)| owner.clone()).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(list) = base.get(&m.key) {
            modified.push(NamedModified { key: m.key.clone(), diff: inverse_rel_list(list, &m.diff) });
        }
    }
    let mut added = Vec::new();
    for owner in &diff.removed {
        if let Some(list) = base.get(owner) {
            added.push((owner.clone(), list.clone()));
        }
    }
    PptxOpcRelationshipsDiff { removed, modified, added }
}

fn absorb_relationships(d1: PptxOpcRelationshipsDiff, d2: PptxOpcRelationshipsDiff) -> PptxOpcRelationshipsDiff {
    absorb_named(d1, d2, |(owner, _)| owner.clone(), absorb_rel_list_diff, |(_, list), diff| *list = rel_list_with_diff_applied(list, diff))
}

fn diff_opc(base: &OpcPackage, other: &OpcPackage) -> Option<PptxOpcDiff> {
    let content_types = diff_content_types(&base.content_types, &other.content_types);
    let parts = diff_parts(&base.parts, &other.parts);
    let relationships = diff_relationships(&base.relationships, &other.relationships);
    if content_types.is_none() && parts.is_none() && relationships.is_none() {
        None
    } else {
        Some(PptxOpcDiff { content_types, parts, relationships })
    }
}

fn apply_opc_diff(opc: &mut OpcPackage, diff: &PptxOpcDiff) -> MutationApplyResult<()> {
    if let Some(d) = &diff.content_types {
        if let Some(dd) = &d.defaults {
            apply_ct_entries(&mut opc.content_types.defaults, dd).map_err(|error| error.under(["contentTypes", "defaults"]))?;
        }
        if let Some(dd) = &d.overrides {
            apply_ct_entries(&mut opc.content_types.overrides, dd).map_err(|error| error.under(["contentTypes", "overrides"]))?;
        }
    }
    if let Some(d) = &diff.parts {
        apply_named(
            &mut opc.parts,
            d,
            |p| p.path.clone(),
            |part, change| {
                apply_part(part, change);
                Ok(())
            },
        )
        .map_err(|error| error.under(["parts"]))?;
    }
    if let Some(d) = &diff.relationships {
        apply_relationships(&mut opc.relationships, d).map_err(|error| error.under(["relationships"]))?;
    }
    Ok(())
}

fn inverse_opc_diff(base: &OpcPackage, diff: &PptxOpcDiff) -> PptxOpcDiff {
    PptxOpcDiff {
        content_types: diff
            .content_types
            .as_ref()
            .map(|d| PptxOpcContentTypesDiff { defaults: d.defaults.as_ref().map(|dd| inverse_ct_entries(&base.content_types.defaults, dd)), overrides: d.overrides.as_ref().map(|dd| inverse_ct_entries(&base.content_types.overrides, dd)) }),
        parts: diff.parts.as_ref().map(|d| inverse_named(&base.parts, d, |p| p.path.clone(), inverse_part)),
        relationships: diff.relationships.as_ref().map(|d| inverse_relationships(&base.relationships, d)),
    }
}

fn absorb_opc_diff(a: PptxOpcDiff, b: PptxOpcDiff) -> PptxOpcDiff {
    PptxOpcDiff {
        content_types: match (a.content_types, b.content_types) {
            (None, x) => x,
            (x, None) => x,
            (Some(ca), Some(cb)) => Some(PptxOpcContentTypesDiff {
                defaults: match (ca.defaults, cb.defaults) {
                    (None, x) => x,
                    (x, None) => x,
                    (Some(da), Some(db)) => Some(absorb_ct_entries(da, db)),
                },
                overrides: match (ca.overrides, cb.overrides) {
                    (None, x) => x,
                    (x, None) => x,
                    (Some(da), Some(db)) => Some(absorb_ct_entries(da, db)),
                },
            }),
        },
        parts: match (a.parts, b.parts) {
            (None, x) => x,
            (x, None) => x,
            (Some(pa), Some(pb)) => Some(absorb_named(pa, pb, |p| p.path.clone(), absorb_part_diff, |part, diff| *part = part_with_diff_applied(part, diff))),
        },
        relationships: match (a.relationships, b.relationships) {
            (None, x) => x,
            (x, None) => x,
            (Some(ra), Some(rb)) => Some(absorb_relationships(ra, rb)),
        },
    }
}
//#endregion 🔖️OpcDiffLogic

//#region 🔖️Apply
impl MutationDiff<PptxSnapshot> for PptxDiff {
    fn apply(&self, base: &PptxSnapshot) -> MutationApplyResult<PptxSnapshot> {
        let mut next = base.clone();
        if let Some(d) = &self.opc {
            apply_opc_diff(&mut next.opc, d).map_err(|error| error.under(["opc"]))?;
        }
        if let Some(d) = &self.presentation {
            apply_presentation_diff(&mut next.presentation, d).map_err(|error| error.under(["presentation"]))?;
        }
        if let Some(xml_parts) = &self.xml_parts {
            next.xml_parts = xml_parts.clone();
        }
        next.normalize_logical_keys();
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        self.opc = match (self.opc.take(), other.opc) {
            (None, x) => x,
            (x, None) => x,
            (Some(a), Some(b)) => Some(absorb_opc_diff(a, b)),
        };
        self.presentation = match (self.presentation.take(), other.presentation) {
            (None, x) => x,
            (x, None) => x,
            (Some(a), Some(b)) => Some(absorb_presentation_diff(a, b)),
        };
        if other.xml_parts.is_some() {
            self.xml_parts = other.xml_parts;
        }
    }
}
//#endregion 🔖️Apply

//#region 🔖️DiffAlgebra
impl DiffAlgebra<PptxSnapshot> for PptxDiff {
    fn inverse(&self, base: &PptxSnapshot) -> Self {
        PptxDiff {
            opc: self.opc.as_ref().map(|d| inverse_opc_diff(&base.opc, d)),
            presentation: self.presentation.as_ref().map(|d| inverse_presentation_diff(&base.presentation, d)),
            xml_parts: self.xml_parts.as_ref().map(|_| base.xml_parts.clone()),
        }
    }

    fn between(base: &PptxSnapshot, other: &PptxSnapshot) -> Self {
        PptxDiff { opc: diff_opc(&base.opc, &other.opc), presentation: diff_presentation(&base.presentation, &other.presentation), xml_parts: (base.xml_parts != other.xml_parts).then(|| other.xml_parts.clone()) }
    }

    fn is_empty(&self) -> bool {
        self.opc.is_none() && self.presentation.is_none() && self.xml_parts.is_none()
    }
}
//#endregion 🔖️DiffAlgebra

//#region 🔖️SetSnapshot
/// 🧩 Builds the sparse field-by-field diff for a `SetSnapshot` mutation. No `snapshot:
/// Option<PptxSnapshot>` full-replace slot -- this IS `PptxDiff::between`.
pub fn diff_set_snapshot(base: &PptxSnapshot, next: &PptxSnapshot) -> PptxDiff {
    PptxDiff::between(base, next)
}

/// 🧩 Builds the diff for inserting `slide` at `index` (FINAL state).
pub fn diff_insert_slide(index: usize, slide: PptxSlide) -> PptxDiff {
    let slides = PptxSlidesDiff { added: vec![IndexAdded { index, item: slide }], ..Default::default() };
    PptxDiff { opc: None, presentation: Some(PptxPresentationDiff { slides: Some(slides) }), xml_parts: None }
}

/// 🧩 Builds the diff for removing the slide at `index` (BASE-state index).
pub fn diff_remove_slide(index: usize) -> PptxDiff {
    let slides = PptxSlidesDiff { removed: vec![index], ..Default::default() };
    PptxDiff { opc: None, presentation: Some(PptxPresentationDiff { slides: Some(slides) }), xml_parts: None }
}

/// 🧩 Builds the diff for moving the slide at BASE-state index `from` to FINAL-state index `to`
/// -- represented as a plain removed+added pair (the collection triple has no separate "moved"
/// concept; `apply_indexed`'s own remove-then-insert semantics already reconstruct a move
/// correctly from that pair, same as any other index-keyed collection in this recipe).
pub fn diff_move_slide(presentation: &PptxPresentation, from: usize, to: usize) -> PptxDiff {
    let Some(slide) = presentation.slides.get(from) else { return PptxDiff::default() };
    if from == to {
        return PptxDiff::default();
    }
    let slides = PptxSlidesDiff { removed: vec![from], added: vec![IndexAdded { index: to, item: slide.clone() }], modified: vec![] };
    PptxDiff { opc: None, presentation: Some(PptxPresentationDiff { slides: Some(slides) }), xml_parts: None }
}

fn wrap_slide_diff(slide_index: usize, slide_diff: PptxSlideDiff) -> PptxDiff {
    let slides = PptxSlidesDiff { modified: vec![IndexModified { index: slide_index, diff: slide_diff }], ..Default::default() };
    PptxDiff { opc: None, presentation: Some(PptxPresentationDiff { slides: Some(slides) }), xml_parts: None }
}

/// 🧩 Builds the diff for inserting `shape` at `shape_index` (FINAL state) on the slide at
/// `slide_index` (BASE-state index -- slides are not reindexed by this mutation).
pub fn diff_insert_shape(slide_index: usize, shape_index: usize, shape: PptxShape) -> PptxDiff {
    let shapes = PptxShapesDiff { added: vec![IndexAdded { index: shape_index, item: shape }], ..Default::default() };
    wrap_slide_diff(slide_index, PptxSlideDiff { shapes: Some(shapes) })
}

/// 🧩 Builds the diff for removing the shape at `shape_index` (BASE-state index) on the slide at
/// `slide_index`.
pub fn diff_remove_shape(slide_index: usize, shape_index: usize) -> PptxDiff {
    let shapes = PptxShapesDiff { removed: vec![shape_index], ..Default::default() };
    wrap_slide_diff(slide_index, PptxSlideDiff { shapes: Some(shapes) })
}

/// 🧩 Builds the diff for replacing a `TextBox`/`Placeholder` shape's `text_frame`. A no-op
/// (`PptxDiff::default()`) if the shape doesn't exist or doesn't carry a text frame (`Picture`,
/// `Other`).
pub fn diff_set_shape_text(presentation: &PptxPresentation, slide_index: usize, shape_index: usize, text_frame: Vec<PptxParagraph>) -> PptxDiff {
    let Some(slide) = presentation.slides.get(slide_index) else { return PptxDiff::default() };
    let Some(shape) = slide.shapes.get(shape_index) else { return PptxDiff::default() };
    let shape_diff = match shape {
        PptxShape::TextBox { text_frame: old_tf, .. } => {
            let Some(tf_diff) = between_indexed(old_tf, &text_frame, diff_paragraph) else { return PptxDiff::default() };
            PptxShapeDiff::TextBox(PptxTextBoxDiff { text_frame: Some(tf_diff), position: None })
        }
        PptxShape::Placeholder { text_frame: old_tf, .. } => {
            let Some(tf_diff) = between_indexed(old_tf, &text_frame, diff_paragraph) else { return PptxDiff::default() };
            PptxShapeDiff::Placeholder(PptxPlaceholderDiff { kind: None, text_frame: Some(tf_diff), position: None })
        }
        PptxShape::Picture { .. } | PptxShape::Other { .. } => return PptxDiff::default(),
    };
    let shapes = PptxShapesDiff { modified: vec![IndexModified { index: shape_index, diff: shape_diff }], ..Default::default() };
    wrap_slide_diff(slide_index, PptxSlideDiff { shapes: Some(shapes) })
}

/// 🧩 Builds the diff for setting a shape's `position`. A no-op if the shape doesn't exist or
/// already has that position (`Other` never carries a typed position -- its raw `xml` is never
/// touched by this mutation).
pub fn diff_set_shape_position(presentation: &PptxPresentation, slide_index: usize, shape_index: usize, position: PptxTransform) -> PptxDiff {
    let Some(slide) = presentation.slides.get(slide_index) else { return PptxDiff::default() };
    let Some(shape) = slide.shapes.get(shape_index) else { return PptxDiff::default() };
    let shape_diff = match shape {
        PptxShape::TextBox { position: old, .. } if *old != position => PptxShapeDiff::TextBox(PptxTextBoxDiff { text_frame: None, position: Some(position) }),
        PptxShape::Picture { position: old, .. } if *old != position => PptxShapeDiff::Picture(PptxPictureDiff { blip_rel_id: None, position: Some(position) }),
        PptxShape::Placeholder { position: old, .. } if *old != position => PptxShapeDiff::Placeholder(PptxPlaceholderDiff { kind: None, text_frame: None, position: Some(position) }),
        _ => return PptxDiff::default(),
    };
    let shapes = PptxShapesDiff { modified: vec![IndexModified { index: shape_index, diff: shape_diff }], ..Default::default() };
    wrap_slide_diff(slide_index, PptxSlideDiff { shapes: Some(shapes) })
}
//#endregion 🔖️SetSnapshot

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: **hand-rolled** `protocol::DiffCodec` for `PptxDiff` — `#[derive(dsl::DslDiff)]`
/// confirmed rejected for THREE independent reasons (all captured verbatim from a real
/// `cargo check`, see `f6-pptx-report.md`): (1) `PptxShapeDiff` is a genuine data-carrying enum
/// (`TextBox`/`Picture`/`Placeholder`/`Replace`) reached through `PptxSlideDiff.shapes` — no
/// `DslField` impl exists for it (matches `SvgNodeDiff`'s blocker, svg's `🔺️diff/🦀️component.rs`);
/// (2) `PptxRunDiff.font_size: Option<Option<u32>>` is tri-state — same blocker as `GifFrameDiff`;
/// (3) a THIRD, previously-undocumented blocker beyond `f6-recon-report.md` §3: this artifact's
/// (and `📜️docx`'s) generic `IndexedTripleDiff<D, T>`/`NamedTripleDiff<K, D, T>` collection-diff
/// engine cannot be `#[derive(dsl::DslRecord)]`d AT ALL — the derive macro has no generics support
/// (confirmed: attempting it on `IndexedTripleDiff<D, T>` emits literally malformed codegen,
/// `struct IndexedTripleDiff<D, T><D, T>`, `error[E0107]: missing generics for struct`). Even a
/// pptx snapshot/diff tree with zero enums and zero tri-state fields would still be blocked by this
/// alone, since EVERY collection field in this recipe's shape routes through one of these two
/// generic engines. Same grammar style `GifDiff`/`SvgDiff`'s hand-rolled codecs use
/// (bracket-depth-aware split, hex for strings/bytes, `[0]`/`[1,x]` for `Option<T>`, single-letter
/// tag prefix for data-carrying enums, `[removed];[modified];[added]` for collection triples) — see
/// `SvgDiff`'s doc comment for the primitive rationale; this file re-derives its own copies (no
/// shared "hand-roll helpers" module exists yet).
//#region 🔖️Primitives
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
pub(crate) fn enc_xml_parts(parts: &[PptxXmlPart]) -> String {
    let value = dsl::to_dsl_value(&parts).expect("serializable logical pptx xml parts");
    hex_encode(&store::pack_rt::encode_wire_value(&value))
}
pub(crate) fn dec_xml_parts(s: &str) -> Result<Vec<PptxXmlPart>, String> {
    let value = store::pack_rt::decode_wire_value(&hex_decode(s)?).map_err(|error| error.to_string())?;
    dsl::from_dsl_value(value)
}
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
pub(crate) fn dec_bool(s: &str) -> Result<bool, String> {
    match s {
        "1" => Ok(true),
        "0" => Ok(false),
        other => Err(format!("bool: bad token {other:?}")),
    }
}
pub(crate) fn bool_str(b: bool) -> &'static str {
    if b {
        "1"
    } else {
        "0"
    }
}

pub(crate) fn split_top_level(s: &str, sep: char) -> Vec<&str> {
    if s.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            c if c == sep && depth == 0 => {
                out.push(&s[start..i]);
                start = i + c.len_utf8();
            }
            _ => {}
        }
    }
    out.push(&s[start..]);
    out
}
pub(crate) fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}
pub(crate) fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
pub(crate) fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(enc).collect::<Vec<_>>().join(","))
}
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
}
//#endregion 🔖️Primitives

//#region 🔖️GenericTripleCodecs
/// 🌳 Index-keyed collection triple, generic codec: `[removed];[modified];[added]`, `modified`
/// entries `idx:diff`, `added` entries `idx:item` — reused by every `IndexedTripleDiff<D, T>`
/// instantiation in this artifact (`PptxSlidesDiff`/`PptxShapesDiff`/`PptxParagraphsDiff`/`PptxRunsDiff`).
pub(crate) fn enc_indexed<D, T>(triple: &IndexedTripleDiff<D, T>, enc_diff: impl Fn(&D) -> String, enc_item: impl Fn(&T) -> String) -> String {
    let removed = triple.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = triple.modified.iter().map(|m| format!("{}:{}", m.index, enc_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = triple.added.iter().map(|a| format!("{}:{}", a.index, enc_item(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
pub(crate) fn dec_indexed<D, T>(body: &str, dec_diff: impl Fn(&str) -> Result<D, String>, dec_item: impl Fn(&str) -> Result<T, String>) -> Result<IndexedTripleDiff<D, T>, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("indexed triple: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("indexed modified: bad entry {entry:?}"))?;
            Ok(IndexModified { index: parse_usize(idx)?, diff: dec_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("indexed added: bad entry {entry:?}"))?;
            Ok(IndexAdded { index: parse_usize(idx)?, item: dec_item(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(IndexedTripleDiff { removed, modified, added })
}

/// 🏷️ Name-keyed collection triple, generic codec: `[removed];[modified];[added]`, `removed`/
/// `modified.key` hex-encoded string keys, `added` entries the whole item (already carrying its own
/// key, per `NamedTripleDiff`'s own shape) — reused by every `NamedTripleDiff<String, D, T>`
/// instantiation (`PptxOpc*Diff`'s content-types/parts/relationships triples).
pub(crate) fn enc_named<D, T>(triple: &NamedTripleDiff<String, D, T>, enc_diff: impl Fn(&D) -> String, enc_item: impl Fn(&T) -> String) -> String {
    let removed = triple.removed.iter().map(|k| enc_str(k)).collect::<Vec<_>>().join(",");
    let modified = triple.modified.iter().map(|m| format!("{}:{}", enc_str(&m.key), enc_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = triple.added.iter().map(&enc_item).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
pub(crate) fn dec_named<D, T>(body: &str, dec_diff: impl Fn(&str) -> Result<D, String>, dec_item: impl Fn(&str) -> Result<T, String>) -> Result<NamedTripleDiff<String, D, T>, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("named triple: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_str).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (key, rest) = entry.split_once(':').ok_or_else(|| format!("named modified: bad entry {entry:?}"))?;
            Ok(NamedModified { key: dec_str(key)?, diff: dec_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_item).collect::<Result<Vec<_>, String>>()?;
    Ok(NamedTripleDiff { removed, modified, added })
}
//#endregion 🔖️GenericTripleCodecs

//#region 🔖️PptxValueCodecs
pub(crate) fn enc_transform(t: &PptxTransform) -> String {
    format!("[{},{},{},{}]", t.x, t.y, t.cx, t.cy)
}
pub(crate) fn dec_transform(s: &str) -> Result<PptxTransform, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, cx, cy] = parts.as_slice() else { return Err(format!("transform: expected 4 fields, got {}", parts.len())) };
    let i = |s: &str| s.parse::<i64>().map_err(|e: std::num::ParseIntError| e.to_string());
    Ok(PptxTransform { x: i(x)?, y: i(y)?, cx: i(cx)?, cy: i(cy)? })
}
pub(crate) fn enc_run(r: &PptxRun) -> String {
    format!("[{},{},{},{}]", enc_str(&r.text), bool_str(r.bold), bool_str(r.italic), encode_option(&r.font_size, |v| v.to_string()))
}
pub(crate) fn dec_run(s: &str) -> Result<PptxRun, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [text, bold, italic, font_size] = parts.as_slice() else { return Err(format!("run: expected 4 fields, got {}", parts.len())) };
    Ok(PptxRun { text: dec_str(text)?, bold: dec_bool(bold)?, italic: dec_bool(italic)?, font_size: decode_option(font_size, |v| v.parse::<u32>().map_err(|e: std::num::ParseIntError| e.to_string()))? })
}
/// 🌱 `PptxParagraph` has exactly one field (`runs: Vec<PptxRun>`) — its positional tuple collapses
/// to that field's own list encoding, no extra wrapping bracket.
pub(crate) fn enc_paragraph(p: &PptxParagraph) -> String {
    enc_list(&p.runs, enc_run)
}
pub(crate) fn dec_paragraph(s: &str) -> Result<PptxParagraph, String> {
    Ok(PptxParagraph { runs: dec_list(s, dec_run)? })
}
/// 🖼️ `PptxShape` (full item): a genuine data-carrying enum — single-uppercase-tag prefix (`B`=
/// TextBox, `P`=Picture, `H`=Placeholder, `O`=Other), same convention as `enc_xml_node`.
pub(crate) fn enc_shape(s: &PptxShape) -> String {
    match s {
        PptxShape::TextBox { text_frame, position } => format!("B[{},{}]", enc_list(text_frame, enc_paragraph), enc_transform(position)),
        PptxShape::Picture { blip_rel_id, position } => format!("P[{},{}]", enc_str(blip_rel_id), enc_transform(position)),
        PptxShape::Placeholder { kind, text_frame, position } => format!("H[{},{},{}]", enc_str(kind), enc_list(text_frame, enc_paragraph), enc_transform(position)),
        PptxShape::Other { node } => {
            let value = dsl::to_dsl_value(node).expect("serializable logical xml node");
            format!("O[{}]", hex_encode(&store::pack_rt::encode_wire_value(&value)))
        }
    }
}
pub(crate) fn dec_shape(s: &str) -> Result<PptxShape, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "B" => {
            let parts = split_top_level(inner, ',');
            let [text_frame, position] = parts.as_slice() else { return Err(format!("shape textbox: expected 2 fields, got {}", parts.len())) };
            Ok(PptxShape::TextBox { text_frame: dec_list(text_frame, dec_paragraph)?, position: dec_transform(position)? })
        }
        "P" => {
            let parts = split_top_level(inner, ',');
            let [blip_rel_id, position] = parts.as_slice() else { return Err(format!("shape picture: expected 2 fields, got {}", parts.len())) };
            Ok(PptxShape::Picture { blip_rel_id: dec_str(blip_rel_id)?, position: dec_transform(position)? })
        }
        "H" => {
            let parts = split_top_level(inner, ',');
            let [kind, text_frame, position] = parts.as_slice() else { return Err(format!("shape placeholder: expected 3 fields, got {}", parts.len())) };
            Ok(PptxShape::Placeholder { kind: dec_str(kind)?, text_frame: dec_list(text_frame, dec_paragraph)?, position: dec_transform(position)? })
        }
        "O" => {
            let value = store::pack_rt::decode_wire_value(&hex_decode(inner)?).map_err(|error| error.to_string())?;
            Ok(PptxShape::Other { node: dsl::from_dsl_value(value)? })
        }
        other => Err(format!("shape: unknown tag {other:?}")),
    }
}
/// 🌱 `PptxSlide` has exactly one field (`shapes: Vec<PptxShape>`) — same collapse as `PptxParagraph`.
pub(crate) fn enc_slide(s: &PptxSlide) -> String {
    enc_list(&s.shapes, enc_shape)
}
pub(crate) fn dec_slide(s: &str) -> Result<PptxSlide, String> {
    Ok(PptxSlide { shapes: dec_list(s, dec_shape)? })
}
//#endregion 🔖️PptxValueCodecs

//#region 🔖️PptxDiffValueCodecs
pub(crate) fn enc_run_diff(d: &PptxRunDiff) -> String {
    format!(
        "[{},{},{},{}]",
        encode_option(&d.text, |v| enc_str(v)),
        encode_option(&d.bold, |v| bool_str(*v).to_string()),
        encode_option(&d.italic, |v| bool_str(*v).to_string()),
        encode_option(&d.font_size, |inner: &Option<u32>| encode_option(inner, |v| v.to_string())),
    )
}
pub(crate) fn dec_run_diff(s: &str) -> Result<PptxRunDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [text, bold, italic, font_size] = parts.as_slice() else { return Err(format!("run diff: expected 4 fields, got {}", parts.len())) };
    Ok(PptxRunDiff {
        text: decode_option(text, dec_str)?,
        bold: decode_option(bold, dec_bool)?,
        italic: decode_option(italic, dec_bool)?,
        font_size: decode_option(font_size, |s| decode_option(s, |v| v.parse::<u32>().map_err(|e: std::num::ParseIntError| e.to_string())))?,
    })
}
pub(crate) fn enc_runs_diff(d: &PptxRunsDiff) -> String {
    enc_indexed(d, enc_run_diff, enc_run)
}
pub(crate) fn dec_runs_diff(s: &str) -> Result<PptxRunsDiff, String> {
    dec_indexed(s, dec_run_diff, dec_run)
}
/// 🌱 `PptxParagraphDiff` has exactly one field (`runs: Option<PptxRunsDiff>`) — collapses to that
/// field's own `encode_option`.
pub(crate) fn enc_paragraph_diff(d: &PptxParagraphDiff) -> String {
    encode_option(&d.runs, enc_runs_diff)
}
pub(crate) fn dec_paragraph_diff(s: &str) -> Result<PptxParagraphDiff, String> {
    Ok(PptxParagraphDiff { runs: decode_option(s, dec_runs_diff)? })
}
pub(crate) fn enc_paragraphs_diff(d: &PptxParagraphsDiff) -> String {
    enc_indexed(d, enc_paragraph_diff, enc_paragraph)
}
pub(crate) fn dec_paragraphs_diff(s: &str) -> Result<PptxParagraphsDiff, String> {
    dec_indexed(s, dec_paragraph_diff, dec_paragraph)
}
/// 🌳 `PptxShapeDiff` needs its own tag (same 4 letters `enc_shape` uses for `B`/`P`/`H`, plus `R`
/// for `Replace` — never mixed in the same parse context, so reuse is unambiguous) since, unlike
/// `PptxShape`, it appears standalone at the `PptxShapesDiff` `modified` entry position.
pub(crate) fn enc_shape_diff(d: &PptxShapeDiff) -> String {
    match d {
        PptxShapeDiff::TextBox(td) => format!("B[{},{}]", encode_option(&td.text_frame, enc_paragraphs_diff), encode_option(&td.position, enc_transform)),
        PptxShapeDiff::Picture(pd) => format!("P[{},{}]", encode_option(&pd.blip_rel_id, |v| enc_str(v)), encode_option(&pd.position, enc_transform)),
        PptxShapeDiff::Placeholder(phd) => format!("H[{},{},{}]", encode_option(&phd.kind, |v| enc_str(v)), encode_option(&phd.text_frame, enc_paragraphs_diff), encode_option(&phd.position, enc_transform),),
        PptxShapeDiff::Replace { shape } => format!("R[{}]", enc_shape(shape)),
    }
}
pub(crate) fn dec_shape_diff(s: &str) -> Result<PptxShapeDiff, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "B" => {
            let parts = split_top_level(inner, ',');
            let [text_frame, position] = parts.as_slice() else { return Err(format!("shape diff textbox: expected 2 fields, got {}", parts.len())) };
            Ok(PptxShapeDiff::TextBox(PptxTextBoxDiff { text_frame: decode_option(text_frame, dec_paragraphs_diff)?, position: decode_option(position, dec_transform)? }))
        }
        "P" => {
            let parts = split_top_level(inner, ',');
            let [blip_rel_id, position] = parts.as_slice() else { return Err(format!("shape diff picture: expected 2 fields, got {}", parts.len())) };
            Ok(PptxShapeDiff::Picture(PptxPictureDiff { blip_rel_id: decode_option(blip_rel_id, dec_str)?, position: decode_option(position, dec_transform)? }))
        }
        "H" => {
            let parts = split_top_level(inner, ',');
            let [kind, text_frame, position] = parts.as_slice() else { return Err(format!("shape diff placeholder: expected 3 fields, got {}", parts.len())) };
            Ok(PptxShapeDiff::Placeholder(PptxPlaceholderDiff { kind: decode_option(kind, dec_str)?, text_frame: decode_option(text_frame, dec_paragraphs_diff)?, position: decode_option(position, dec_transform)? }))
        }
        "R" => Ok(PptxShapeDiff::Replace { shape: dec_shape(inner)? }),
        other => Err(format!("shape diff: unknown tag {other:?}")),
    }
}
pub(crate) fn enc_shapes_diff(d: &PptxShapesDiff) -> String {
    enc_indexed(d, enc_shape_diff, enc_shape)
}
pub(crate) fn dec_shapes_diff(s: &str) -> Result<PptxShapesDiff, String> {
    dec_indexed(s, dec_shape_diff, dec_shape)
}
/// 🌱 `PptxSlideDiff` has exactly one field (`shapes: Option<PptxShapesDiff>`) — same collapse as
/// `PptxParagraphDiff`.
pub(crate) fn enc_slide_diff(d: &PptxSlideDiff) -> String {
    encode_option(&d.shapes, enc_shapes_diff)
}
pub(crate) fn dec_slide_diff(s: &str) -> Result<PptxSlideDiff, String> {
    Ok(PptxSlideDiff { shapes: decode_option(s, dec_shapes_diff)? })
}
pub(crate) fn enc_slides_diff(d: &PptxSlidesDiff) -> String {
    enc_indexed(d, enc_slide_diff, enc_slide)
}
pub(crate) fn dec_slides_diff(s: &str) -> Result<PptxSlidesDiff, String> {
    dec_indexed(s, dec_slide_diff, dec_slide)
}
/// 🌱 `PptxPresentationDiff` has exactly one field (`slides: Option<PptxSlidesDiff>`) — same
/// collapse.
fn enc_presentation_diff(d: &PptxPresentationDiff) -> String {
    encode_option(&d.slides, enc_slides_diff)
}
fn dec_presentation_diff(s: &str) -> Result<PptxPresentationDiff, String> {
    Ok(PptxPresentationDiff { slides: decode_option(s, dec_slides_diff)? })
}
//#endregion 🔖️PptxDiffValueCodecs

//#region 🔖️OpcValueCodecs
pub(crate) fn enc_ct_entry((k, v): &(String, String)) -> String {
    format!("[{},{}]", enc_str(k), enc_str(v))
}
pub(crate) fn dec_ct_entry(s: &str) -> Result<(String, String), String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [k, v] = parts.as_slice() else { return Err(format!("ct entry: expected 2 fields, got {}", parts.len())) };
    Ok((dec_str(k)?, dec_str(v)?))
}
pub(crate) fn enc_part(p: &OpcPart) -> String {
    format!("[{},{},{}]", enc_str(&p.path), enc_str(&p.content_type), hex_encode(&p.bytes))
}
pub(crate) fn dec_part(s: &str) -> Result<OpcPart, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [path, content_type, bytes] = parts.as_slice() else { return Err(format!("part: expected 3 fields, got {}", parts.len())) };
    Ok(OpcPart { path: dec_str(path)?, content_type: dec_str(content_type)?, bytes: hex_decode(bytes)? })
}
pub(crate) fn enc_target_mode(m: &OpcTargetMode) -> String {
    match m {
        OpcTargetMode::Internal => "0".to_string(),
        OpcTargetMode::External => "1".to_string(),
    }
}
pub(crate) fn dec_target_mode(s: &str) -> Result<OpcTargetMode, String> {
    match s {
        "0" => Ok(OpcTargetMode::Internal),
        "1" => Ok(OpcTargetMode::External),
        other => Err(format!("target mode: bad token {other:?}")),
    }
}
pub(crate) fn enc_rel(r: &OpcRelationship) -> String {
    format!("[{},{},{},{}]", enc_str(&r.id), enc_str(&r.rel_type), enc_str(&r.target), enc_target_mode(&r.target_mode))
}
pub(crate) fn dec_rel(s: &str) -> Result<OpcRelationship, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, rel_type, target, target_mode] = parts.as_slice() else { return Err(format!("rel: expected 4 fields, got {}", parts.len())) };
    Ok(OpcRelationship { id: dec_str(id)?, rel_type: dec_str(rel_type)?, target: dec_str(target)?, target_mode: dec_target_mode(target_mode)? })
}
/// 🌱 A relationships-owner `added` entry carries `(owner, whole rel list)` — `owner` itself IS
/// the `NamedTripleDiff` key, matching `enc_ct_entry`'s `(key, value)` shape.
pub(crate) fn enc_owner_rels((owner, list): &(String, Vec<OpcRelationship>)) -> String {
    format!("[{},{}]", enc_str(owner), enc_list(list, enc_rel))
}
pub(crate) fn dec_owner_rels(s: &str) -> Result<(String, Vec<OpcRelationship>), String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [owner, list] = parts.as_slice() else { return Err(format!("owner rels: expected 2 fields, got {}", parts.len())) };
    Ok((dec_str(owner)?, dec_list(list, dec_rel)?))
}
//#endregion 🔖️OpcValueCodecs

//#region 🔖️OpcDiffValueCodecs
pub(crate) fn enc_part_diff(d: &PptxOpcPartDiff) -> String {
    format!("[{},{}]", encode_option(&d.content_type, |v| enc_str(v)), encode_option(&d.bytes, |v| hex_encode(v)))
}
pub(crate) fn dec_part_diff(s: &str) -> Result<PptxOpcPartDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [content_type, bytes] = parts.as_slice() else { return Err(format!("part diff: expected 2 fields, got {}", parts.len())) };
    Ok(PptxOpcPartDiff { content_type: decode_option(content_type, dec_str)?, bytes: decode_option(bytes, hex_decode)? })
}
pub(crate) fn enc_rel_diff(d: &PptxOpcRelDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.rel_type, |v| enc_str(v)), encode_option(&d.target, |v| enc_str(v)), encode_option(&d.target_mode, enc_target_mode),)
}
pub(crate) fn dec_rel_diff(s: &str) -> Result<PptxOpcRelDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [rel_type, target, target_mode] = parts.as_slice() else { return Err(format!("rel diff: expected 3 fields, got {}", parts.len())) };
    Ok(PptxOpcRelDiff { rel_type: decode_option(rel_type, dec_str)?, target: decode_option(target, dec_str)?, target_mode: decode_option(target_mode, dec_target_mode)? })
}
pub(crate) fn enc_ct_entries_diff(d: &PptxOpcCtEntriesDiff) -> String {
    enc_named(d, |v| enc_str(v), enc_ct_entry)
}
pub(crate) fn dec_ct_entries_diff(s: &str) -> Result<PptxOpcCtEntriesDiff, String> {
    dec_named(s, dec_str, dec_ct_entry)
}
fn enc_content_types_diff(d: &PptxOpcContentTypesDiff) -> String {
    format!("[{},{}]", encode_option(&d.defaults, enc_ct_entries_diff), encode_option(&d.overrides, enc_ct_entries_diff))
}
fn dec_content_types_diff(s: &str) -> Result<PptxOpcContentTypesDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [defaults, overrides] = parts.as_slice() else { return Err(format!("content types diff: expected 2 fields, got {}", parts.len())) };
    Ok(PptxOpcContentTypesDiff { defaults: decode_option(defaults, dec_ct_entries_diff)?, overrides: decode_option(overrides, dec_ct_entries_diff)? })
}
fn enc_parts_diff(d: &PptxOpcPartsDiff) -> String {
    enc_named(d, enc_part_diff, enc_part)
}
fn dec_parts_diff(s: &str) -> Result<PptxOpcPartsDiff, String> {
    dec_named(s, dec_part_diff, dec_part)
}
pub(crate) fn enc_rel_list_diff(d: &PptxOpcRelListDiff) -> String {
    enc_named(d, enc_rel_diff, enc_rel)
}
pub(crate) fn dec_rel_list_diff(s: &str) -> Result<PptxOpcRelListDiff, String> {
    dec_named(s, dec_rel_diff, dec_rel)
}
fn enc_relationships_diff(d: &PptxOpcRelationshipsDiff) -> String {
    enc_named(d, enc_rel_list_diff, enc_owner_rels)
}
fn dec_relationships_diff(s: &str) -> Result<PptxOpcRelationshipsDiff, String> {
    dec_named(s, dec_rel_list_diff, dec_owner_rels)
}
fn enc_opc_diff(d: &PptxOpcDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.content_types, enc_content_types_diff), encode_option(&d.parts, enc_parts_diff), encode_option(&d.relationships, enc_relationships_diff),)
}
fn dec_opc_diff(s: &str) -> Result<PptxOpcDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [content_types, parts_f, relationships] = parts.as_slice() else { return Err(format!("opc diff: expected 3 fields, got {}", parts.len())) };
    Ok(PptxOpcDiff { content_types: decode_option(content_types, dec_content_types_diff)?, parts: decode_option(parts_f, dec_parts_diff)?, relationships: decode_option(relationships, dec_relationships_diff)? })
}
//#endregion 🔖️OpcDiffValueCodecs

//#region 🔖️BinaryCodecs
/// 🧪️ FG-wave: real recursive BINARY twins of every text-form codec above, backing the upgraded
/// `DiffCodec::encode_diff`/`decode_diff` below (and, via re-export, `../🧬️mutations/🦀️component.rs`'s
/// own upgraded `OpBinary`) — replaces F1's `print_diff().into_bytes()` text-as-binary shortcut.
/// Real LEB128-varint-framed length-prefixed strings/bytes (`store::pack_rt::write_varint_u64` +
/// `store::ByteReader`), fixed 8-byte little-endian `i64` fields (`PptxTransform`'s EMU
/// coordinates — no signed-varint writer is exported from `store::pack_rt`, so these use the same
/// fixed-width primitive `u16`/`u32` protocol fields already use, just wider), 1-byte tri-state
/// presence tags, and 1-byte enum-variant tags — genuinely structured binary, never hex-ASCII text
/// reused as "binary". Same shape `📜️docx/…/🔺️diff/🦀️component.rs`'s own `BinaryPrimitives`/
/// `ValueBinaryCodecs`/`DiffValueBinaryCodecs` regions establish; duplicated here (not imported)
/// per this repo's per-artifact hand-roll convention (no shared "hand-roll helpers" module exists
/// yet, see this file's own `HandcraftedDiffCodec` doc comment).
//#region 🔖️BinaryPrimitives
pub(crate) fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
pub(crate) fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
pub(crate) fn enc_xml_parts_bin(parts: &[PptxXmlPart], out: &mut Vec<u8>) {
    let value = dsl::to_dsl_value(&parts).expect("serializable logical pptx xml parts");
    write_bytes_lp(out, &store::pack_rt::encode_wire_value(&value));
}
pub(crate) fn dec_xml_parts_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<PptxXmlPart>, String> {
    let value = store::pack_rt::decode_wire_value(&read_bytes_lp(reader)?).map_err(|error| error.to_string())?;
    dsl::from_dsl_value(value)
}
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
/// 🌱 `PptxTransform`'s four EMU coordinates are `i64` (theoretically signed, off-canvas shapes
/// notwithstanding) — fixed 8-byte little-endian, same width class the protocol dialect's own
/// `u64`/`i64` `Prim` already uses, just hand-rolled here since no signed-varint writer is
/// exported from `store::pack_rt` (only `write_varint_u64`).
pub(crate) fn write_i64_fixed(out: &mut Vec<u8>, v: i64) {
    out.extend_from_slice(&v.to_le_bytes());
}
pub(crate) fn read_i64_fixed(reader: &mut store::ByteReader<'_>) -> Result<i64, String> {
    let bytes = reader.read_bytes(8).map_err(|e| e.to_string())?;
    let mut arr = [0u8; 8];
    arr.copy_from_slice(bytes);
    Ok(i64::from_le_bytes(arr))
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️ValueBinaryCodecs
/// 🌳️ Full-item (non-diff) binary codecs, mirrored one-for-one against `../🔖️PptxValueCodecs`'s
/// text forms above. `pub(crate)` so `../🧬️mutations/🦀️component.rs` reuses these rather than
/// re-deriving its own copies (same intra-artifact reuse pattern the text codecs already use).
pub(crate) fn enc_transform_bin(t: &PptxTransform, out: &mut Vec<u8>) {
    write_i64_fixed(out, t.x);
    write_i64_fixed(out, t.y);
    write_i64_fixed(out, t.cx);
    write_i64_fixed(out, t.cy);
}
pub(crate) fn dec_transform_bin(reader: &mut store::ByteReader<'_>) -> Result<PptxTransform, String> {
    Ok(PptxTransform { x: read_i64_fixed(reader)?, y: read_i64_fixed(reader)?, cx: read_i64_fixed(reader)?, cy: read_i64_fixed(reader)? })
}

pub(crate) fn enc_run_bin(r: &PptxRun, out: &mut Vec<u8>) {
    write_str_lp(out, &r.text);
    out.push(r.bold as u8);
    out.push(r.italic as u8);
    out.push(if r.font_size.is_some() { 1 } else { 0 });
    if let Some(sz) = r.font_size {
        store::pack_rt::write_varint_u64(out, sz as u64);
    }
}
pub(crate) fn dec_run_bin(reader: &mut store::ByteReader<'_>) -> Result<PptxRun, String> {
    let text = read_str_lp(reader)?;
    let bold = reader.read_u8().map_err(|e| e.to_string())? != 0;
    let italic = reader.read_u8().map_err(|e| e.to_string())? != 0;
    let has_font_size = reader.read_u8().map_err(|e| e.to_string())? != 0;
    let font_size = if has_font_size { Some(reader.read_varint_u64().map_err(|e| e.to_string())? as u32) } else { None };
    Ok(PptxRun { text, bold, italic, font_size })
}

pub(crate) fn enc_paragraph_bin(p: &PptxParagraph, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, p.runs.len() as u64);
    for r in &p.runs {
        enc_run_bin(r, out);
    }
}
pub(crate) fn dec_paragraph_bin(reader: &mut store::ByteReader<'_>) -> Result<PptxParagraph, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut runs = Vec::with_capacity(count as usize);
    for _ in 0..count {
        runs.push(dec_run_bin(reader)?);
    }
    Ok(PptxParagraph { runs })
}
pub(crate) fn enc_paragraph_list_bin(ps: &[PptxParagraph], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, ps.len() as u64);
    for p in ps {
        enc_paragraph_bin(p, out);
    }
}
pub(crate) fn dec_paragraph_list_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<PptxParagraph>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(dec_paragraph_bin(reader)?);
    }
    Ok(out)
}

/// 🖼️ `PptxShape` (full item): `0`=TextBox, `1`=Picture, `2`=Placeholder, `3`=Other — same
/// declaration order as the enum itself, tag-prefixed like `enc_xml_node_bin`'s own convention.
pub(crate) fn enc_shape_bin(s: &PptxShape, out: &mut Vec<u8>) {
    match s {
        PptxShape::TextBox { text_frame, position } => {
            out.push(0);
            enc_paragraph_list_bin(text_frame, out);
            enc_transform_bin(position, out);
        }
        PptxShape::Picture { blip_rel_id, position } => {
            out.push(1);
            write_str_lp(out, blip_rel_id);
            enc_transform_bin(position, out);
        }
        PptxShape::Placeholder { kind, text_frame, position } => {
            out.push(2);
            write_str_lp(out, kind);
            enc_paragraph_list_bin(text_frame, out);
            enc_transform_bin(position, out);
        }
        PptxShape::Other { node } => {
            out.push(3);
            let value = dsl::to_dsl_value(node).expect("serializable logical xml node");
            write_bytes_lp(out, &store::pack_rt::encode_wire_value(&value));
        }
    }
}
pub(crate) fn dec_shape_bin(reader: &mut store::ByteReader<'_>) -> Result<PptxShape, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(PptxShape::TextBox { text_frame: dec_paragraph_list_bin(reader)?, position: dec_transform_bin(reader)? }),
        1 => Ok(PptxShape::Picture { blip_rel_id: read_str_lp(reader)?, position: dec_transform_bin(reader)? }),
        2 => Ok(PptxShape::Placeholder { kind: read_str_lp(reader)?, text_frame: dec_paragraph_list_bin(reader)?, position: dec_transform_bin(reader)? }),
        3 => {
            let value = store::pack_rt::decode_wire_value(&read_bytes_lp(reader)?).map_err(|error| error.to_string())?;
            Ok(PptxShape::Other { node: dsl::from_dsl_value(value)? })
        }
        other => Err(format!("shape binary: unknown tag {other}")),
    }
}
pub(crate) fn enc_slide_bin(s: &PptxSlide, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, s.shapes.len() as u64);
    for sh in &s.shapes {
        enc_shape_bin(sh, out);
    }
}
pub(crate) fn dec_slide_bin(reader: &mut store::ByteReader<'_>) -> Result<PptxSlide, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut shapes = Vec::with_capacity(count as usize);
    for _ in 0..count {
        shapes.push(dec_shape_bin(reader)?);
    }
    Ok(PptxSlide { shapes })
}

pub(crate) fn enc_target_mode_bin(m: &OpcTargetMode, out: &mut Vec<u8>) {
    out.push(match m {
        OpcTargetMode::Internal => 0,
        OpcTargetMode::External => 1,
    });
}
pub(crate) fn dec_target_mode_bin(reader: &mut store::ByteReader<'_>) -> Result<OpcTargetMode, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(OpcTargetMode::Internal),
        1 => Ok(OpcTargetMode::External),
        other => Err(format!("target mode binary: bad value {other}")),
    }
}
pub(crate) fn enc_part_bin(p: &OpcPart, out: &mut Vec<u8>) {
    write_str_lp(out, &p.path);
    write_str_lp(out, &p.content_type);
    write_bytes_lp(out, &p.bytes);
}
pub(crate) fn dec_part_bin(reader: &mut store::ByteReader<'_>) -> Result<OpcPart, String> {
    let path = read_str_lp(reader)?;
    let content_type = read_str_lp(reader)?;
    let bytes = read_bytes_lp(reader)?;
    Ok(OpcPart { path, content_type, bytes })
}
pub(crate) fn enc_rel_bin(r: &OpcRelationship, out: &mut Vec<u8>) {
    write_str_lp(out, &r.id);
    write_str_lp(out, &r.rel_type);
    write_str_lp(out, &r.target);
    enc_target_mode_bin(&r.target_mode, out);
}
pub(crate) fn dec_rel_bin(reader: &mut store::ByteReader<'_>) -> Result<OpcRelationship, String> {
    let id = read_str_lp(reader)?;
    let rel_type = read_str_lp(reader)?;
    let target = read_str_lp(reader)?;
    let target_mode = dec_target_mode_bin(reader)?;
    Ok(OpcRelationship { id, rel_type, target, target_mode })
}
pub(crate) fn enc_ct_entry_bin(e: &(String, String), out: &mut Vec<u8>) {
    write_str_lp(out, &e.0);
    write_str_lp(out, &e.1);
}
pub(crate) fn dec_ct_entry_bin(reader: &mut store::ByteReader<'_>) -> Result<(String, String), String> {
    Ok((read_str_lp(reader)?, read_str_lp(reader)?))
}
pub(crate) fn enc_owner_rels_bin(e: &(String, Vec<OpcRelationship>), out: &mut Vec<u8>) {
    write_str_lp(out, &e.0);
    store::pack_rt::write_varint_u64(out, e.1.len() as u64);
    for r in &e.1 {
        enc_rel_bin(r, out);
    }
}
pub(crate) fn dec_owner_rels_bin(reader: &mut store::ByteReader<'_>) -> Result<(String, Vec<OpcRelationship>), String> {
    let owner = read_str_lp(reader)?;
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut list = Vec::with_capacity(count as usize);
    for _ in 0..count {
        list.push(dec_rel_bin(reader)?);
    }
    Ok((owner, list))
}
//#endregion 🔖️ValueBinaryCodecs

//#region 🔖️GenericTripleBinaryCodecs
/// 🌳️ Binary twin of `enc_indexed`/`dec_indexed` -- three varint-counted sections (removed
/// indices / modified index+diff pairs / added index+item pairs), generic over `D`/`T`.
fn enc_indexed_triple_bin<D, T>(diff: &IndexedTripleDiff<D, T>, enc_d: impl Fn(&D, &mut Vec<u8>), enc_t: impl Fn(&T, &mut Vec<u8>), out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, diff.removed.len() as u64);
    for i in &diff.removed {
        store::pack_rt::write_varint_u64(out, *i as u64);
    }
    store::pack_rt::write_varint_u64(out, diff.modified.len() as u64);
    for m in &diff.modified {
        store::pack_rt::write_varint_u64(out, m.index as u64);
        enc_d(&m.diff, out);
    }
    store::pack_rt::write_varint_u64(out, diff.added.len() as u64);
    for a in &diff.added {
        store::pack_rt::write_varint_u64(out, a.index as u64);
        enc_t(&a.item, out);
    }
}
fn dec_indexed_triple_bin<D, T>(reader: &mut store::ByteReader<'_>, dec_d: impl Fn(&mut store::ByteReader<'_>) -> Result<D, String>, dec_t: impl Fn(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<IndexedTripleDiff<D, T>, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(reader.read_varint_u64().map_err(|e| e.to_string())? as usize);
    }
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let diff = dec_d(reader)?;
        modified.push(IndexModified { index, diff });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let item = dec_t(reader)?;
        added.push(IndexAdded { index, item });
    }
    Ok(IndexedTripleDiff { removed, modified, added })
}

/// 🏷️ Binary twin of `enc_named`/`dec_named` -- three varint-counted sections (removed keys /
/// modified key+diff pairs / added whole items), generic over `K`/`D`/`T`.
fn enc_named_triple_bin<K, D, T>(diff: &NamedTripleDiff<K, D, T>, enc_k: impl Fn(&K, &mut Vec<u8>), enc_d: impl Fn(&D, &mut Vec<u8>), enc_t: impl Fn(&T, &mut Vec<u8>), out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, diff.removed.len() as u64);
    for k in &diff.removed {
        enc_k(k, out);
    }
    store::pack_rt::write_varint_u64(out, diff.modified.len() as u64);
    for m in &diff.modified {
        enc_k(&m.key, out);
        enc_d(&m.diff, out);
    }
    store::pack_rt::write_varint_u64(out, diff.added.len() as u64);
    for t in &diff.added {
        enc_t(t, out);
    }
}
fn dec_named_triple_bin<K, D, T>(
    reader: &mut store::ByteReader<'_>,
    dec_k: impl Fn(&mut store::ByteReader<'_>) -> Result<K, String>,
    dec_d: impl Fn(&mut store::ByteReader<'_>) -> Result<D, String>,
    dec_t: impl Fn(&mut store::ByteReader<'_>) -> Result<T, String>,
) -> Result<NamedTripleDiff<K, D, T>, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(dec_k(reader)?);
    }
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let key = dec_k(reader)?;
        let diff = dec_d(reader)?;
        modified.push(NamedModified { key, diff });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        added.push(dec_t(reader)?);
    }
    Ok(NamedTripleDiff { removed, modified, added })
}
//#endregion 🔖️GenericTripleBinaryCodecs

//#region 🔖️DiffValueBinaryCodecs
fn enc_run_diff_bin(d: &PptxRunDiff, out: &mut Vec<u8>) {
    out.push(if d.text.is_some() { 1 } else { 0 });
    if let Some(v) = &d.text {
        write_str_lp(out, v);
    }
    out.push(if d.bold.is_some() { 1 } else { 0 });
    if let Some(v) = &d.bold {
        out.push(*v as u8);
    }
    out.push(if d.italic.is_some() { 1 } else { 0 });
    if let Some(v) = &d.italic {
        out.push(*v as u8);
    }
    // 🏳️ `font_size: Option<Option<u32>>` — doubly-nested tri-state, two presence bytes.
    out.push(if d.font_size.is_some() { 1 } else { 0 });
    if let Some(inner) = &d.font_size {
        out.push(if inner.is_some() { 1 } else { 0 });
        if let Some(sz) = inner {
            store::pack_rt::write_varint_u64(out, *sz as u64);
        }
    }
}
fn dec_run_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<PptxRunDiff, String> {
    let text = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
    let bold = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(reader.read_u8().map_err(|e| e.to_string())? != 0) } else { None };
    let italic = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(reader.read_u8().map_err(|e| e.to_string())? != 0) } else { None };
    let font_size = if reader.read_u8().map_err(|e| e.to_string())? != 0 {
        let inner = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(reader.read_varint_u64().map_err(|e| e.to_string())? as u32) } else { None };
        Some(inner)
    } else {
        None
    };
    Ok(PptxRunDiff { text, bold, italic, font_size })
}
fn enc_runs_diff_bin(d: &PptxRunsDiff, out: &mut Vec<u8>) {
    enc_indexed_triple_bin(d, enc_run_diff_bin, enc_run_bin, out)
}
fn dec_runs_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<PptxRunsDiff, String> {
    dec_indexed_triple_bin(reader, dec_run_diff_bin, dec_run_bin)
}

fn enc_paragraph_diff_bin(d: &PptxParagraphDiff, out: &mut Vec<u8>) {
    out.push(if d.runs.is_some() { 1 } else { 0 });
    if let Some(v) = &d.runs {
        enc_runs_diff_bin(v, out);
    }
}
fn dec_paragraph_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<PptxParagraphDiff, String> {
    let runs = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_runs_diff_bin(reader)?) } else { None };
    Ok(PptxParagraphDiff { runs })
}
fn enc_paragraphs_diff_bin(d: &PptxParagraphsDiff, out: &mut Vec<u8>) {
    enc_indexed_triple_bin(d, enc_paragraph_diff_bin, enc_paragraph_bin, out)
}
fn dec_paragraphs_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<PptxParagraphsDiff, String> {
    dec_indexed_triple_bin(reader, dec_paragraph_diff_bin, dec_paragraph_bin)
}

/// 🌳️ `PptxShapeDiff` -- `0`=TextBox, `1`=Picture, `2`=Placeholder, `3`=Replace, same tag
/// numbering `enc_shape_bin` uses for the full-item form (never mixed in the same binary stream).
fn enc_shape_diff_bin(d: &PptxShapeDiff, out: &mut Vec<u8>) {
    match d {
        PptxShapeDiff::TextBox(td) => {
            out.push(0);
            out.push(if td.text_frame.is_some() { 1 } else { 0 });
            if let Some(v) = &td.text_frame {
                enc_paragraphs_diff_bin(v, out);
            }
            out.push(if td.position.is_some() { 1 } else { 0 });
            if let Some(v) = &td.position {
                enc_transform_bin(v, out);
            }
        }
        PptxShapeDiff::Picture(pd) => {
            out.push(1);
            out.push(if pd.blip_rel_id.is_some() { 1 } else { 0 });
            if let Some(v) = &pd.blip_rel_id {
                write_str_lp(out, v);
            }
            out.push(if pd.position.is_some() { 1 } else { 0 });
            if let Some(v) = &pd.position {
                enc_transform_bin(v, out);
            }
        }
        PptxShapeDiff::Placeholder(phd) => {
            out.push(2);
            out.push(if phd.kind.is_some() { 1 } else { 0 });
            if let Some(v) = &phd.kind {
                write_str_lp(out, v);
            }
            out.push(if phd.text_frame.is_some() { 1 } else { 0 });
            if let Some(v) = &phd.text_frame {
                enc_paragraphs_diff_bin(v, out);
            }
            out.push(if phd.position.is_some() { 1 } else { 0 });
            if let Some(v) = &phd.position {
                enc_transform_bin(v, out);
            }
        }
        PptxShapeDiff::Replace { shape } => {
            out.push(3);
            enc_shape_bin(shape, out);
        }
    }
}
fn dec_shape_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<PptxShapeDiff, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => {
            let text_frame = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_paragraphs_diff_bin(reader)?) } else { None };
            let position = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_transform_bin(reader)?) } else { None };
            Ok(PptxShapeDiff::TextBox(PptxTextBoxDiff { text_frame, position }))
        }
        1 => {
            let blip_rel_id = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
            let position = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_transform_bin(reader)?) } else { None };
            Ok(PptxShapeDiff::Picture(PptxPictureDiff { blip_rel_id, position }))
        }
        2 => {
            let kind = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
            let text_frame = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_paragraphs_diff_bin(reader)?) } else { None };
            let position = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_transform_bin(reader)?) } else { None };
            Ok(PptxShapeDiff::Placeholder(PptxPlaceholderDiff { kind, text_frame, position }))
        }
        3 => Ok(PptxShapeDiff::Replace { shape: dec_shape_bin(reader)? }),
        other => Err(format!("shape diff binary: unknown tag {other}")),
    }
}
fn enc_shapes_diff_bin(d: &PptxShapesDiff, out: &mut Vec<u8>) {
    enc_indexed_triple_bin(d, enc_shape_diff_bin, enc_shape_bin, out)
}
fn dec_shapes_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<PptxShapesDiff, String> {
    dec_indexed_triple_bin(reader, dec_shape_diff_bin, dec_shape_bin)
}

fn enc_slide_diff_bin(d: &PptxSlideDiff, out: &mut Vec<u8>) {
    out.push(if d.shapes.is_some() { 1 } else { 0 });
    if let Some(v) = &d.shapes {
        enc_shapes_diff_bin(v, out);
    }
}
fn dec_slide_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<PptxSlideDiff, String> {
    let shapes = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_shapes_diff_bin(reader)?) } else { None };
    Ok(PptxSlideDiff { shapes })
}
fn enc_slides_diff_bin(d: &PptxSlidesDiff, out: &mut Vec<u8>) {
    enc_indexed_triple_bin(d, enc_slide_diff_bin, enc_slide_bin, out)
}
fn dec_slides_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<PptxSlidesDiff, String> {
    dec_indexed_triple_bin(reader, dec_slide_diff_bin, dec_slide_bin)
}

fn enc_presentation_diff_bin(d: &PptxPresentationDiff, out: &mut Vec<u8>) {
    out.push(if d.slides.is_some() { 1 } else { 0 });
    if let Some(v) = &d.slides {
        enc_slides_diff_bin(v, out);
    }
}
fn dec_presentation_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<PptxPresentationDiff, String> {
    let slides = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_slides_diff_bin(reader)?) } else { None };
    Ok(PptxPresentationDiff { slides })
}

fn enc_part_diff_bin(d: &PptxOpcPartDiff, out: &mut Vec<u8>) {
    out.push(if d.content_type.is_some() { 1 } else { 0 });
    if let Some(v) = &d.content_type {
        write_str_lp(out, v);
    }
    out.push(if d.bytes.is_some() { 1 } else { 0 });
    if let Some(v) = &d.bytes {
        write_bytes_lp(out, v);
    }
}
fn dec_part_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<PptxOpcPartDiff, String> {
    let content_type = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
    let bytes = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_bytes_lp(reader)?) } else { None };
    Ok(PptxOpcPartDiff { content_type, bytes })
}

fn enc_rel_diff_bin(d: &PptxOpcRelDiff, out: &mut Vec<u8>) {
    out.push(if d.rel_type.is_some() { 1 } else { 0 });
    if let Some(v) = &d.rel_type {
        write_str_lp(out, v);
    }
    out.push(if d.target.is_some() { 1 } else { 0 });
    if let Some(v) = &d.target {
        write_str_lp(out, v);
    }
    out.push(if d.target_mode.is_some() { 1 } else { 0 });
    if let Some(v) = &d.target_mode {
        enc_target_mode_bin(v, out);
    }
}
fn dec_rel_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<PptxOpcRelDiff, String> {
    let rel_type = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
    let target = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
    let target_mode = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_target_mode_bin(reader)?) } else { None };
    Ok(PptxOpcRelDiff { rel_type, target, target_mode })
}

fn enc_ct_entries_diff_bin(d: &PptxOpcCtEntriesDiff, out: &mut Vec<u8>) {
    enc_named_triple_bin(d, |k, out| write_str_lp(out, k), |v: &String, out| write_str_lp(out, v), enc_ct_entry_bin, out)
}
fn dec_ct_entries_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<PptxOpcCtEntriesDiff, String> {
    dec_named_triple_bin(reader, |r| read_str_lp(r), |r| read_str_lp(r), dec_ct_entry_bin)
}
fn enc_content_types_diff_bin(d: &PptxOpcContentTypesDiff, out: &mut Vec<u8>) {
    out.push(if d.defaults.is_some() { 1 } else { 0 });
    if let Some(v) = &d.defaults {
        enc_ct_entries_diff_bin(v, out);
    }
    out.push(if d.overrides.is_some() { 1 } else { 0 });
    if let Some(v) = &d.overrides {
        enc_ct_entries_diff_bin(v, out);
    }
}
fn dec_content_types_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<PptxOpcContentTypesDiff, String> {
    let defaults = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_ct_entries_diff_bin(reader)?) } else { None };
    let overrides = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_ct_entries_diff_bin(reader)?) } else { None };
    Ok(PptxOpcContentTypesDiff { defaults, overrides })
}
fn enc_parts_diff_bin(d: &PptxOpcPartsDiff, out: &mut Vec<u8>) {
    enc_named_triple_bin(d, |k, out| write_str_lp(out, k), enc_part_diff_bin, enc_part_bin, out)
}
fn dec_parts_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<PptxOpcPartsDiff, String> {
    dec_named_triple_bin(reader, |r| read_str_lp(r), dec_part_diff_bin, dec_part_bin)
}
fn enc_rel_list_diff_bin(d: &PptxOpcRelListDiff, out: &mut Vec<u8>) {
    enc_named_triple_bin(d, |k, out| write_str_lp(out, k), enc_rel_diff_bin, enc_rel_bin, out)
}
fn dec_rel_list_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<PptxOpcRelListDiff, String> {
    dec_named_triple_bin(reader, |r| read_str_lp(r), dec_rel_diff_bin, dec_rel_bin)
}
fn enc_relationships_diff_bin(d: &PptxOpcRelationshipsDiff, out: &mut Vec<u8>) {
    enc_named_triple_bin(d, |k, out| write_str_lp(out, k), enc_rel_list_diff_bin, enc_owner_rels_bin, out)
}
fn dec_relationships_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<PptxOpcRelationshipsDiff, String> {
    dec_named_triple_bin(reader, |r| read_str_lp(r), dec_rel_list_diff_bin, dec_owner_rels_bin)
}
fn enc_opc_diff_bin(d: &PptxOpcDiff, out: &mut Vec<u8>) {
    out.push(if d.content_types.is_some() { 1 } else { 0 });
    if let Some(v) = &d.content_types {
        enc_content_types_diff_bin(v, out);
    }
    out.push(if d.parts.is_some() { 1 } else { 0 });
    if let Some(v) = &d.parts {
        enc_parts_diff_bin(v, out);
    }
    out.push(if d.relationships.is_some() { 1 } else { 0 });
    if let Some(v) = &d.relationships {
        enc_relationships_diff_bin(v, out);
    }
}
fn dec_opc_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<PptxOpcDiff, String> {
    let content_types = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_content_types_diff_bin(reader)?) } else { None };
    let parts = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_parts_diff_bin(reader)?) } else { None };
    let relationships = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_relationships_diff_bin(reader)?) } else { None };
    Ok(PptxOpcDiff { content_types, parts, relationships })
}
//#endregion 🔖️DiffValueBinaryCodecs
//#endregion 🔖️BinaryCodecs

//#region 🔖️TopLevel
fn print_pptx_diff(d: &PptxDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.opc {
        tokens.push(format!("opc={}", enc_opc_diff(v)));
    }
    if let Some(v) = &d.presentation {
        tokens.push(format!("presentation={}", enc_presentation_diff(v)));
    }
    if let Some(v) = &d.xml_parts {
        tokens.push(format!("xmlParts={}", enc_xml_parts(v)));
    }
    tokens.join(" ")
}
fn parse_pptx_diff(line: &str) -> Result<PptxDiff, String> {
    let mut d = PptxDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("opc=") {
            d.opc = Some(dec_opc_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("presentation=") {
            d.presentation = Some(dec_presentation_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("xmlParts=") {
            d.xml_parts = Some(dec_xml_parts(rest)?);
        } else {
            return Err(format!("pptx diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
struct PptxDiffRecord {
    value: dsl::DslValue,
}

impl protocol::DiffCodec for PptxDiff {
    fn print_diff(&self) -> String {
        let record = PptxDiffRecord { value: dsl::to_dsl_value(self).expect("serializable logical pptx diff") };
        dsl::print(&record.__dsl_to_record(), &PptxDiffRecord::__dsl_spec(), dsl::JoinMode::Inline)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        let record = dsl::parse(line, &PptxDiffRecord::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits { max_bytes: 64 * 1024 * 1024, ..dsl::Limits::default() }, mode: dsl::SourceMode::Inline })?;
        let model = PptxDiffRecord::__dsl_from_record(&record)?;
        dsl::from_dsl_value(model.value).map_err(|error| store::TextError::new(error, dsl::TextSpan::at(1, 1)))
    }
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let value = dsl::to_dsl_value(self).map_err(|detail| protocol::ProtocolError::Malformed { what: "pptx diff", offset: 0, detail })?;
        Ok(store::pack_rt::encode_wire_value(&value))
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "pptx diff", offset: 0, detail: error.to_string() })?;
        dsl::from_dsl_value(value).map_err(|detail| protocol::ProtocolError::Malformed { what: "pptx diff", offset: 0, detail })
    }
}
//#endregion 🔖️TopLevel

//#region 🔖️DemoCases
/// 🧪️ FG-wave: representative `PptxDiff` values (both top-level fields, the `PptxShapeDiff` enum
/// tree incl. `Replace`, the `font_size` tri-state, and the OPC layer's content-types/parts/
/// relationships-by-owner triples) — the single source of truth reused by
/// `diff_codec_text_binary_roundtrip_law` below AND by `⚙️engine/🦀️component.rs`'s
/// `diff_grammar_conformance_law`/`protocol_walk_law` conformance tests, same shape
/// `📜️docx/…/🔺️diff/🦀️component.rs`'s own `demo_diff_cases()` establishes.
pub(crate) fn demo_snapshot_a() -> PptxSnapshot {
    let mut opc = OpcPackage::empty();
    opc.content_types.set_default("rels", crate::artifacts::zip::opc::RELS_CONTENT_TYPE);
    opc.content_types.set_default("xml", "application/xml");
    opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", b"<p:presentation/>".to_vec());
    opc.set_part("ppt/toRemove.xml", "application/xml", b"gone".to_vec());
    opc.add_relationship("", "rId1", crate::artifacts::zip::opc::REL_TYPE_OFFICE_DOCUMENT, "ppt/presentation.xml");
    opc.relationships.insert("ppt/toRemove.xml".into(), vec![OpcRelationship { id: "rId8".into(), rel_type: "http://example/gone".into(), target: "media/gone.png".into(), target_mode: OpcTargetMode::Internal }]);

    PptxSnapshot::from_parts(
        opc,
        Vec::new(),
        PptxPresentation {
            slides: vec![
                PptxSlide {
                    shapes: vec![
                        PptxShape::TextBox { text_frame: vec![PptxParagraph { runs: vec![PptxRun { text: "old".into(), bold: false, italic: false, font_size: Some(10) }] }], position: PptxTransform { x: 1, y: 1, cx: 1, cy: 1 } },
                        PptxShape::Picture { blip_rel_id: "rIdOld".into(), position: PptxTransform::default() },
                    ],
                },
                PptxSlide { shapes: vec![PptxShape::Other { node: XmlNode::Element { name: "p:graphicFrame".into(), attrs: Vec::new(), children: Vec::new() } }] },
            ],
        },
    )
}

pub(crate) fn demo_snapshot_b() -> PptxSnapshot {
    let mut opc = OpcPackage::empty();
    opc.content_types.set_default("rels", crate::artifacts::zip::opc::RELS_CONTENT_TYPE);
    opc.content_types.set_default("xml", "application/xml");
    opc.content_types.set_default("added", "application/octet-stream");
    opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", b"<p:presentation/>changed".to_vec());
    opc.set_part("ppt/added.xml", "application/xml", b"fresh".to_vec());
    opc.add_relationship("", "rId1", crate::artifacts::zip::opc::REL_TYPE_OFFICE_DOCUMENT, "ppt/presentation.xml");
    opc.relationships.insert("ppt/added.xml".into(), vec![OpcRelationship { id: "rId3".into(), rel_type: "http://example/added".into(), target: "media/added.png".into(), target_mode: OpcTargetMode::External }]);

    PptxSnapshot::from_parts(
        opc,
        Vec::new(),
        PptxPresentation {
            slides: vec![PptxSlide {
                shapes: vec![
                    PptxShape::TextBox { text_frame: vec![PptxParagraph { runs: vec![PptxRun { text: "new".into(), bold: true, italic: true, font_size: None }] }], position: PptxTransform { x: 9, y: 9, cx: 9, cy: 9 } },
                    PptxShape::Placeholder { kind: "body".into(), text_frame: vec![PptxParagraph::text("ph")], position: PptxTransform::default() },
                ],
            }],
        },
    )
}

/// 🧪️ The demo cases proper — `default()` (empty diff) plus every real `between()` shape (both
/// directions, and the trivially-empty self-diff).
pub(crate) fn demo_diff_cases() -> Vec<PptxDiff> {
    let a = demo_snapshot_a();
    let b = demo_snapshot_b();
    vec![PptxDiff::default(), PptxDiff::between(&a, &b), PptxDiff::between(&b, &a), PptxDiff::between(&a, &a)]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod handcrafted_diff_codec_tests {
    use super::*;
    use crate::artifacts::zip::opc::{OpcPackage, RELS_CONTENT_TYPE, REL_TYPE_OFFICE_DOCUMENT};
    use protocol::DiffCodec;

    fn elem_snapshot(slides: Vec<PptxSlide>) -> PptxSnapshot {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", b"<p:presentation/>".to_vec());
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, "ppt/presentation.xml");
        PptxSnapshot::from_parts(opc, Vec::new(), PptxPresentation { slides })
    }

    /// 🧪️ `DiffCodec` round-trip laws over the hand-rolled `PptxDiff` grammar — exercises the
    /// `PptxShapeDiff` enum tree (`TextBox`/`Picture`/`Placeholder`/`Replace`), the `font_size`
    /// tri-state (`Some(None)` AND `Some(Some(_))`), nested `IndexedTripleDiff` collections
    /// (slides/shapes/paragraphs/runs) AND `NamedTripleDiff` collections (content types/parts/
    /// relationships, incl. the doubly-nested relationships-by-owner triple).
    #[test]
    fn diff_codec_text_binary_roundtrip_law() {
        let a = elem_snapshot(vec![PptxSlide {
            shapes: vec![
                PptxShape::TextBox { text_frame: vec![PptxParagraph { runs: vec![PptxRun { text: "old".into(), bold: false, italic: false, font_size: Some(10) }] }], position: PptxTransform { x: 1, y: 1, cx: 1, cy: 1 } },
                PptxShape::Picture { blip_rel_id: "rIdOld".into(), position: PptxTransform::default() },
            ],
        }]);
        let mut b_opc_snapshot = elem_snapshot(vec![PptxSlide {
            shapes: vec![
                PptxShape::TextBox { text_frame: vec![PptxParagraph { runs: vec![PptxRun { text: "new".into(), bold: true, italic: true, font_size: None }] }], position: PptxTransform { x: 9, y: 9, cx: 9, cy: 9 } },
                PptxShape::Placeholder { kind: "body".into(), text_frame: vec![PptxParagraph::text("ph")], position: PptxTransform::default() },
            ],
        }]);
        b_opc_snapshot.opc.set_part("ppt/added.xml", "application/xml", b"fresh".to_vec());
        b_opc_snapshot.opc.content_types.set_override("ppt/added.xml", "application/xml");
        b_opc_snapshot.opc.add_relationship("ppt/added.xml", "rId9", "http://example/added", "media/added.png");
        let b = b_opc_snapshot;

        let c = elem_snapshot(vec![]);

        let cases = vec![PptxDiff::default(), PptxDiff::between(&a, &b), PptxDiff::between(&b, &a), PptxDiff::between(&a, &c), PptxDiff::between(&c, &a)];
        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = PptxDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = PptxDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }

    #[test]
    fn logical_xml_parts_diff_apply_inverse_absorb_between_and_codecs() {
        let base = elem_snapshot(Vec::new());
        let mut sourced = base.clone();
        sourced.xml_parts = vec![PptxXmlPart { path: "docProps/core.xml".into(), content_type: "application/vnd.openxmlformats-package.core-properties+xml".into(), document: crate::artifacts::xml::schema::snapshot::XmlDocument::default() }];
        let diff = PptxDiff::between(&base, &sourced);
        assert_eq!(diff.xml_parts, Some(sourced.xml_parts.clone()));
        assert_eq!(diff.apply(&base).unwrap(), sourced);

        let printed = diff.print_diff();
        assert_eq!(PptxDiff::parse_diff(&printed).expect("parse logical XML parts diff"), diff);
        let encoded = diff.encode_diff().expect("encode logical XML parts diff");
        assert_eq!(PptxDiff::decode_diff(&encoded).expect("decode logical XML parts diff"), diff);

        let inverse = diff.inverse(&base);
        assert_eq!(inverse.apply(&sourced).unwrap(), base);
        let mut absorbed = diff.clone();
        absorbed.absorb(inverse);
        assert_eq!(absorbed.apply(&base).unwrap(), base);
        assert!(PptxDiff::between(&sourced, &sourced).is_empty());
    }
}
//#endregion 🧪️Tests
//#endregion 🔖️HandcraftedDiffCodec

#[cfg(test)]
mod result_apply_tests {
    use super::*;

    #[test]
    fn rejects_missing_slide_target_without_mutating_base() {
        let base = PptxSnapshot::default();
        let diff =
            PptxDiff { presentation: Some(PptxPresentationDiff { slides: Some(PptxSlidesDiff { modified: vec![IndexModified { index: 0, diff: PptxSlideDiff::default() }], ..Default::default() }), ..Default::default() }), ..Default::default() };
        let result = diff.apply(&base);
        assert_eq!(result.unwrap_err().code, "mutation.apply.missing-target");
        assert_eq!(base, PptxSnapshot::default());
    }
}
