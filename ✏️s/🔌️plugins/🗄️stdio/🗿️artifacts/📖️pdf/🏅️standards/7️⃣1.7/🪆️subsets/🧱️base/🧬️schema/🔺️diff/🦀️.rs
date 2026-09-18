//! 🔺️ PdfDiff (1.7) — sparse diff over the typed document model. Ticket
//! 26/09/18/PDF-ARTIFACT-SPEC-COMPLETE widened it with the model: `pages` is an index-keyed
//! triple of per-field page patches whose `content` and `annotations` are themselves
//! index-keyed triples (an inserted operator is one added row, not a re-sent stream); every
//! document collection (`fonts`, `images`, `forms`, graphics states, shadings, patterns, named
//! colour spaces and property lists, embedded files) is an id-keyed triple of whole values;
//! `outlines`, `named_destinations`, `page_labels`, `output_intents` are index-keyed triples;
//! the catalog scalars are tri-state (`Clear`/`Set`); `info` is whole-value replaced; and the
//! retained COS lanes keep their recursive `PdfValueDiff` patches (`objects` keyed by `ObjRef`,
//! `trailer`/`catalog_extra` name-keyed).
//!
//! Every triple follows the recipe: `removed`/`modified` address BASE state (removals processed
//! descending), `added` addresses FINAL state (ascending insert).
//!
//! Codec: the derive-owned value encoding is the wire — one JSON line as text, the container-less
//! pack record body as binary — so every lane rides one codec instead of a hand-rolled one per
//! field (see `🔖️DiffCodec`).

use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{HashMap, HashSet};

//#region 🔖️TriState
/// 🎚️ A change to an optional field: cleared, or set to `value`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfSet<T> {
    Clear,
    Set { value: T },
}

impl<T: Clone> PdfSet<T> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_option(value: &Option<T>) -> Self {
        match value {
            Some(value) => PdfSet::Set { value: value.clone() },
            None => PdfSet::Clear,
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn into_option(self) -> Option<T> {
        match self {
            PdfSet::Set { value } => Some(value),
            PdfSet::Clear => None,
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn tri<T: Clone + PartialEq>(a: &Option<T>, b: &Option<T>) -> Option<PdfSet<T>> {
    (a != b).then(|| PdfSet::from_option(b))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_tri<T: Clone>(slot: &mut Option<T>, diff: &Option<PdfSet<T>>) {
    if let Some(set) = diff {
        *slot = set.clone().into_option();
    }
}
//#endregion 🔖️TriState

//#region 🔖️IndexedTriple
/// 📦️ An index-keyed triple of whole values (positional collections: content operators,
/// annotations, outlines, named destinations, page labels, output intents).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfIndexedItem<T> {
    pub index: usize,
    pub value: T,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfIndexedDiff<T> {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<PdfIndexedItem<T>>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<PdfIndexedItem<T>>,
}

impl<T> Default for PdfIndexedDiff<T> {
    fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() }
    }
}

impl<T: Clone + PartialEq> PdfIndexedDiff<T> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }

    /// 🧭️ Positional delta: common prefix of equal items is untouched; the divergent tail is
    /// matched by position (modified where both sides have an item, removed/added beyond).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn between(a: &[T], b: &[T]) -> Self {
        let mut diff = Self::default();
        let common = a.len().min(b.len());
        for index in 0..common {
            if a[index] != b[index] {
                diff.modified.push(PdfIndexedItem { index, value: b[index].clone() });
            }
        }
        for index in common..a.len() {
            diff.removed.push(index);
        }
        for index in common..b.len() {
            diff.added.push(PdfIndexedItem { index, value: b[index].clone() });
        }
        diff
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn validate(&self, base_len: usize, field: &str) -> MutationApplyResult<()> {
        validate_index_triple(base_len, &self.removed, &self.modified.iter().map(|item| item.index).collect::<Vec<_>>(), &self.added.iter().map(|item| item.index).collect::<Vec<_>>(), field)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply(&self, base: &[T]) -> Vec<T> {
        let mut next: Vec<T> = base.to_vec();
        for item in &self.modified {
            if let Some(slot) = next.get_mut(item.index) {
                *slot = item.value.clone();
            }
        }
        let mut removed = self.removed.clone();
        removed.sort_unstable_by(|a, b| b.cmp(a));
        for index in removed {
            if index < next.len() {
                next.remove(index);
            }
        }
        let mut added = self.added.clone();
        added.sort_by_key(|item| item.index);
        for item in added {
            let index = item.index.min(next.len());
            next.insert(index, item.value);
        }
        next
    }

    /// ➕️ Sequential coalesce: `self` then `other`, transported through `self`'s index moves.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn absorb(self, other: Self) -> Self {
        // 🧮 Rebuild through a virtual base: every index the pair ever touches is materialized
        // as a slot, both diffs applied in order, and the combined triple read back off it.
        let max_base = self.removed.iter().copied().chain(self.modified.iter().map(|m| m.index)).max().map(|m| m + 1).unwrap_or(0).max(self.added.len() + other.removed.iter().copied().chain(other.modified.iter().map(|m| m.index)).max().map(|m| m + 1).unwrap_or(0));
        let base: Vec<Slot<T>> = (0..max_base + self.added.len() + other.added.len() + 1).map(|index| Slot::Base(index)).collect();
        let mid = self.map_slots(&base);
        let end = other.map_slots(&mid);
        Self::from_slots(&base, &end)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn map_slots(&self, base: &[Slot<T>]) -> Vec<Slot<T>> {
        let mut next: Vec<Slot<T>> = base.to_vec();
        for item in &self.modified {
            if let Some(slot) = next.get_mut(item.index) {
                *slot = match slot {
                    Slot::Base(index) => Slot::Modified(*index, item.value.clone()),
                    Slot::Modified(index, _) => Slot::Modified(*index, item.value.clone()),
                    Slot::Added(_) => Slot::Added(item.value.clone()),
                };
            }
        }
        let mut removed = self.removed.clone();
        removed.sort_unstable_by(|a, b| b.cmp(a));
        for index in removed {
            if index < next.len() {
                next.remove(index);
            }
        }
        let mut added = self.added.clone();
        added.sort_by_key(|item| item.index);
        for item in added {
            next.insert(item.index.min(next.len()), Slot::Added(item.value));
        }
        next
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn from_slots(base: &[Slot<T>], end: &[Slot<T>]) -> Self {
        let mut diff = Self::default();
        let surviving: HashSet<usize> = end
            .iter()
            .filter_map(|slot| match slot {
                Slot::Base(index) | Slot::Modified(index, _) => Some(*index),
                Slot::Added(_) => None,
            })
            .collect();
        for slot in base {
            if let Slot::Base(index) = slot {
                if !surviving.contains(index) {
                    diff.removed.push(*index);
                }
            }
        }
        for (position, slot) in end.iter().enumerate() {
            match slot {
                Slot::Modified(index, value) => diff.modified.push(PdfIndexedItem { index: *index, value: value.clone() }),
                Slot::Added(value) => diff.added.push(PdfIndexedItem { index: position, value: value.clone() }),
                Slot::Base(_) => {}
            }
        }
        diff
    }
}

#[derive(Clone)]
enum Slot<T> {
    Base(usize),
    Modified(usize, T),
    Added(T),
}
//#endregion 🔖️IndexedTriple

//#region 🔖️KeyedTriple
/// 🆔 An id-keyed triple of whole values (the document collections keyed by `id`/`name`).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfKeyedItem<T> {
    pub key: String,
    pub value: T,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfKeyedDiff<T> {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<PdfKeyedItem<T>>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<PdfIndexedItem<T>>,
}

impl<T> Default for PdfKeyedDiff<T> {
    fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() }
    }
}

/// 🔑 A collection item that carries its own key.
pub trait Keyed {
    fn key(&self) -> &str;
}

macro_rules! keyed {
    ($($ty:ty => $field:ident),* $(,)?) => {
        $(impl Keyed for $ty {
            fn key(&self) -> &str {
                &self.$field
            }
        })*
    };
}
keyed!(PdfFont => id, PdfImage => id, PdfFormXObject => id, PdfExtGState => id, PdfShading => id, PdfPattern => id, PdfNamedColorSpace => name, PdfNamedProperties => name, PdfEmbeddedFile => id);

impl<T: Clone + PartialEq + Keyed> PdfKeyedDiff<T> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn between(a: &[T], b: &[T]) -> Self {
        let mut diff = Self::default();
        let b_keys: HashSet<&str> = b.iter().map(Keyed::key).collect();
        for item in a {
            if !b_keys.contains(item.key()) {
                diff.removed.push(item.key().to_string());
            }
        }
        for (index, item) in b.iter().enumerate() {
            match a.iter().find(|candidate| candidate.key() == item.key()) {
                Some(existing) if existing == item => {}
                Some(_) => diff.modified.push(PdfKeyedItem { key: item.key().to_string(), value: item.clone() }),
                None => diff.added.push(PdfIndexedItem { index, value: item.clone() }),
            }
        }
        diff
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn validate(&self, base: &[T], field: &str) -> MutationApplyResult<()> {
        let keys: Vec<String> = base.iter().map(|item| item.key().to_string()).collect();
        validate_named_keys(&keys, &self.removed, &self.modified.iter().map(|item| item.key.clone()).collect::<Vec<_>>(), &self.added.iter().map(|item| item.value.key().to_string()).collect::<Vec<_>>(), field)?;
        for item in &self.modified {
            if item.value.key() != item.key {
                return Err(MutationApplyError::new("mutation.apply.conflicting-target", "keyed modification renames its own key").at([field]));
            }
        }
        let removed_indices: Vec<usize> = (0..self.removed.len()).collect();
        validate_index_triple(base.len(), &removed_indices, &[], &self.added.iter().map(|item| item.index).collect::<Vec<_>>(), field)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply(&self, base: &[T]) -> Vec<T> {
        let mut next: Vec<T> = base.iter().filter(|item| !self.removed.iter().any(|key| key == item.key())).cloned().collect();
        for item in &self.modified {
            if let Some(slot) = next.iter_mut().find(|candidate| candidate.key() == item.key) {
                *slot = item.value.clone();
            }
        }
        let mut added = self.added.clone();
        added.sort_by_key(|item| item.index);
        for item in added {
            next.insert(item.index.min(next.len()), item.value);
        }
        next
    }

    /// ➕️ Sequential coalesce by key: later removals win over earlier additions/modifications,
    /// later modifications replace earlier ones, additions of a key removed earlier become
    /// modifications of the base.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn absorb(self, other: Self) -> Self {
        let mut out = Self::default();
        let first_added: HashSet<String> = self.added.iter().map(|item| item.value.key().to_string()).collect();
        for key in self.removed.iter().chain(other.removed.iter()) {
            if first_added.contains(key) && other.removed.contains(key) {
                continue;
            }
            if other.added.iter().any(|item| item.value.key() == key) && self.removed.contains(key) {
                continue;
            }
            if !out.removed.contains(key) {
                out.removed.push(key.clone());
            }
        }
        for item in &self.modified {
            if !other.removed.contains(&item.key) && !other.modified.iter().any(|later| later.key == item.key) {
                out.modified.push(item.clone());
            }
        }
        for item in &other.modified {
            if let Some(added) = self.added.iter().find(|added| added.value.key() == item.key) {
                out.added.push(PdfIndexedItem { index: added.index, value: item.value.clone() });
            } else {
                out.modified.push(item.clone());
            }
        }
        for item in &self.added {
            if !other.removed.contains(&item.value.key().to_string()) && !other.modified.iter().any(|later| later.key == item.value.key()) {
                out.added.push(item.clone());
            }
        }
        for item in &other.added {
            if self.removed.contains(&item.value.key().to_string()) {
                out.modified.push(PdfKeyedItem { key: item.value.key().to_string(), value: item.value.clone() });
            } else {
                out.added.push(item.clone());
            }
        }
        out.added.sort_by_key(|item| item.index);
        out
    }
}
//#endregion 🔖️KeyedTriple

//#region 🔖️PageDiff
/// 📄️ Sparse patch for one `PdfPage`: scalar fields are set/tri-state, `content` and
/// `annotations` are index-keyed triples, `extra` is a name-keyed dictionary patch.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfPageDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub media_box: Option<PdfRect>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub crop_box: Option<PdfSet<PdfRect>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bleed_box: Option<PdfSet<PdfRect>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub trim_box: Option<PdfSet<PdfRect>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub art_box: Option<PdfSet<PdfRect>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rotate: Option<i32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub user_unit: Option<PdfSet<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<PdfIndexedDiff<PdfOp>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<PdfIndexedDiff<PdfAnnotation>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<PdfSet<PdfTransparencyGroup>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PdfSet<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub struct_parents: Option<PdfSet<u32>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub transition: Option<PdfSet<Vec<PdfDictEntry>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<PdfSet<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PdfSet<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub additional_actions: Option<Vec<PdfDictEntry>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<PdfDictDiff>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_page_diff(page: &mut PdfPage, diff: &PdfPageDiff) {
    if let Some(v) = diff.media_box {
        page.media_box = v;
    }
    apply_tri(&mut page.crop_box, &diff.crop_box);
    apply_tri(&mut page.bleed_box, &diff.bleed_box);
    apply_tri(&mut page.trim_box, &diff.trim_box);
    apply_tri(&mut page.art_box, &diff.art_box);
    if let Some(v) = diff.rotate {
        page.rotate = v;
    }
    apply_tri(&mut page.user_unit, &diff.user_unit);
    if let Some(content) = &diff.content {
        page.content = content.apply(&page.content);
    }
    if let Some(annotations) = &diff.annotations {
        page.annotations = annotations.apply(&page.annotations);
    }
    apply_tri(&mut page.group, &diff.group);
    apply_tri(&mut page.thumbnail, &diff.thumbnail);
    apply_tri(&mut page.struct_parents, &diff.struct_parents);
    apply_tri(&mut page.transition, &diff.transition);
    apply_tri(&mut page.duration, &diff.duration);
    apply_tri(&mut page.metadata, &diff.metadata);
    if let Some(v) = &diff.additional_actions {
        page.additional_actions = v.clone();
    }
    if let Some(extra) = &diff.extra {
        page.extra = apply_dict_diff(extra, &page.extra);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn page_diff_between(a: &PdfPage, b: &PdfPage) -> PdfPageDiff {
    let content = PdfIndexedDiff::between(&a.content, &b.content);
    let annotations = PdfIndexedDiff::between(&a.annotations, &b.annotations);
    let extra = dict_diff_between(&a.extra, &b.extra);
    PdfPageDiff {
        media_box: (a.media_box != b.media_box).then_some(b.media_box),
        crop_box: tri(&a.crop_box, &b.crop_box),
        bleed_box: tri(&a.bleed_box, &b.bleed_box),
        trim_box: tri(&a.trim_box, &b.trim_box),
        art_box: tri(&a.art_box, &b.art_box),
        rotate: (a.rotate != b.rotate).then_some(b.rotate),
        user_unit: tri(&a.user_unit, &b.user_unit),
        content: (!content.is_empty()).then_some(content),
        annotations: (!annotations.is_empty()).then_some(annotations),
        group: tri(&a.group, &b.group),
        thumbnail: tri(&a.thumbnail, &b.thumbnail),
        struct_parents: tri(&a.struct_parents, &b.struct_parents),
        transition: tri(&a.transition, &b.transition),
        duration: tri(&a.duration, &b.duration),
        metadata: tri(&a.metadata, &b.metadata),
        additional_actions: (a.additional_actions != b.additional_actions).then(|| b.additional_actions.clone()),
        extra: (!extra.is_empty()).then_some(extra),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_page_diff(diff: &PdfPageDiff, base: &PdfPage) -> MutationApplyResult<()> {
    if let Some(content) = &diff.content {
        content.validate(base.content.len(), "content")?;
    }
    if let Some(annotations) = &diff.annotations {
        annotations.validate(base.annotations.len(), "annotations")?;
    }
    if let Some(extra) = &diff.extra {
        validate_dict_diff(extra, &base.extra)?;
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_page_diff(base: &mut PdfPageDiff, other: PdfPageDiff) {
    if other.media_box.is_some() {
        base.media_box = other.media_box;
    }
    for (slot, next) in [(&mut base.crop_box, other.crop_box), (&mut base.bleed_box, other.bleed_box), (&mut base.trim_box, other.trim_box), (&mut base.art_box, other.art_box)] {
        if next.is_some() {
            *slot = next;
        }
    }
    if other.rotate.is_some() {
        base.rotate = other.rotate;
    }
    if other.user_unit.is_some() {
        base.user_unit = other.user_unit;
    }
    base.content = match (base.content.take(), other.content) {
        (None, b) => b,
        (a, None) => a,
        (Some(a), Some(b)) => {
            let merged = a.absorb(b);
            (!merged.is_empty()).then_some(merged)
        }
    };
    base.annotations = match (base.annotations.take(), other.annotations) {
        (None, b) => b,
        (a, None) => a,
        (Some(a), Some(b)) => {
            let merged = a.absorb(b);
            (!merged.is_empty()).then_some(merged)
        }
    };
    if other.group.is_some() {
        base.group = other.group;
    }
    if other.thumbnail.is_some() {
        base.thumbnail = other.thumbnail;
    }
    if other.struct_parents.is_some() {
        base.struct_parents = other.struct_parents;
    }
    if other.transition.is_some() {
        base.transition = other.transition;
    }
    if other.duration.is_some() {
        base.duration = other.duration;
    }
    if other.metadata.is_some() {
        base.metadata = other.metadata;
    }
    if other.additional_actions.is_some() {
        base.additional_actions = other.additional_actions;
    }
    base.extra = match (base.extra.take(), other.extra) {
        (None, b) => b,
        (a, None) => a,
        (Some(a), Some(b)) => {
            let merged = absorb_dict_diff(a, b);
            (!merged.is_empty()).then_some(merged)
        }
    };
}
//#endregion 🔖️PageDiff

//#region 🔖️PagesTriple
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfPageModified {
    pub index: usize,
    pub diff: PdfPageDiff,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfPageAdded {
    pub index: usize,
    pub page: PdfPage,
}

/// 📦️ Index-keyed `pages` triple (positional -- the recipe's "index usize" key kind).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfPagesDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<PdfPageModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<PdfPageAdded>,
}

impl PdfPagesDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

/// ▶️ Apply semantics (normative): `removed`/`modified` indices refer to BASE state (removals
/// processed descending); `added` indices refer to FINAL state (ascending insert).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_pages_diff(diff: &PdfPagesDiff, base: &[PdfPage]) -> Vec<PdfPage> {
    let mut next: Vec<PdfPage> = base.to_vec();
    for modified in &diff.modified {
        if let Some(page) = next.get_mut(modified.index) {
            apply_page_diff(page, &modified.diff);
        }
    }
    let mut removed = diff.removed.clone();
    removed.sort_unstable_by(|a, b| b.cmp(a));
    for index in removed {
        if index < next.len() {
            next.remove(index);
        }
    }
    let mut added = diff.added.clone();
    added.sort_by_key(|item| item.index);
    for item in added {
        next.insert(item.index.min(next.len()), item.page);
    }
    next
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pages_diff_between(a: &[PdfPage], b: &[PdfPage]) -> PdfPagesDiff {
    let mut diff = PdfPagesDiff::default();
    let common = a.len().min(b.len());
    for index in 0..common {
        if a[index] != b[index] {
            diff.modified.push(PdfPageModified { index, diff: page_diff_between(&a[index], &b[index]) });
        }
    }
    for index in common..a.len() {
        diff.removed.push(index);
    }
    for index in common..b.len() {
        diff.added.push(PdfPageAdded { index, page: b[index].clone() });
    }
    diff
}

/// ➕️ Sequential coalesce of two page triples: `d1`'s modified pages absorb `d2`'s patches
/// where both touch a page that survives, `d2`'s removals of `d1`'s additions cancel, and the
/// rest is transported through `d1`'s index moves via the same virtual-base slot algorithm the
/// generic triple uses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_law_pages_diff(d1: PdfPagesDiff, d2: &PdfPagesDiff) -> PdfPagesDiff {
    #[derive(Clone, PartialEq)]
    struct PageSlotValue {
        page: Option<PdfPage>,
        patch: PdfPageDiff,
    }
    let first: PdfIndexedDiff<PageSlotValue> = PdfIndexedDiff { removed: d1.removed.clone(), modified: d1.modified.iter().map(|m| PdfIndexedItem { index: m.index, value: PageSlotValue { page: None, patch: m.diff.clone() } }).collect(), added: d1.added.iter().map(|a| PdfIndexedItem { index: a.index, value: PageSlotValue { page: Some(a.page.clone()), patch: PdfPageDiff::default() } }).collect() };
    let second: PdfIndexedDiff<PageSlotValue> = PdfIndexedDiff { removed: d2.removed.clone(), modified: d2.modified.iter().map(|m| PdfIndexedItem { index: m.index, value: PageSlotValue { page: None, patch: m.diff.clone() } }).collect(), added: d2.added.iter().map(|a| PdfIndexedItem { index: a.index, value: PageSlotValue { page: Some(a.page.clone()), patch: PdfPageDiff::default() } }).collect() };
    let max_base = first.removed.iter().copied().chain(first.modified.iter().map(|m| m.index)).max().map(|m| m + 1).unwrap_or(0).max(first.added.len() + second.removed.iter().copied().chain(second.modified.iter().map(|m| m.index)).max().map(|m| m + 1).unwrap_or(0));
    let base: Vec<Slot<PageSlotValue>> = (0..max_base + first.added.len() + second.added.len() + 1).map(Slot::Base).collect();
    let mut mid = base.clone();
    for item in &first.modified {
        if let Some(slot) = mid.get_mut(item.index) {
            *slot = match slot {
                Slot::Base(index) | Slot::Modified(index, _) => Slot::Modified(*index, item.value.clone()),
                Slot::Added(_) => Slot::Added(item.value.clone()),
            };
        }
    }
    let mut removed = first.removed.clone();
    removed.sort_unstable_by(|a, b| b.cmp(a));
    for index in removed {
        if index < mid.len() {
            mid.remove(index);
        }
    }
    let mut added = first.added.clone();
    added.sort_by_key(|item| item.index);
    for item in added {
        mid.insert(item.index.min(mid.len()), Slot::Added(item.value));
    }
    let mut end = mid.clone();
    for item in &second.modified {
        if let Some(slot) = end.get_mut(item.index) {
            *slot = match slot {
                Slot::Base(index) => Slot::Modified(*index, item.value.clone()),
                Slot::Modified(index, previous) => {
                    let mut patch = previous.patch.clone();
                    absorb_page_diff(&mut patch, item.value.patch.clone());
                    Slot::Modified(*index, PageSlotValue { page: None, patch })
                }
                Slot::Added(previous) => {
                    let mut page = previous.page.clone().unwrap_or_default();
                    apply_page_diff(&mut page, &item.value.patch);
                    Slot::Added(PageSlotValue { page: Some(page), patch: PdfPageDiff::default() })
                }
            };
        }
    }
    let mut removed = second.removed.clone();
    removed.sort_unstable_by(|a, b| b.cmp(a));
    for index in removed {
        if index < end.len() {
            end.remove(index);
        }
    }
    let mut added = second.added.clone();
    added.sort_by_key(|item| item.index);
    for item in added {
        end.insert(item.index.min(end.len()), Slot::Added(item.value));
    }
    let surviving: HashSet<usize> = end
        .iter()
        .filter_map(|slot| match slot {
            Slot::Base(index) | Slot::Modified(index, _) => Some(*index),
            Slot::Added(_) => None,
        })
        .collect();
    let mut out = PdfPagesDiff::default();
    for slot in &base {
        if let Slot::Base(index) = slot {
            if !surviving.contains(index) {
                out.removed.push(*index);
            }
        }
    }
    for (position, slot) in end.iter().enumerate() {
        match slot {
            Slot::Modified(index, value) => out.modified.push(PdfPageModified { index: *index, diff: value.patch.clone() }),
            Slot::Added(value) => out.added.push(PdfPageAdded { index: position, page: value.page.clone().unwrap_or_default() }),
            Slot::Base(_) => {}
        }
    }
    out
}
//#endregion 🔖️PagesTriple

//#region 🔖️DictDiff (reused for nested Dict/Stream.dict AND top-level trailer)
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfDictModified {
    pub key: String,
    pub diff: PdfValueDiff,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfDictAdded {
    pub index: usize,
    pub key: String,
    pub item: PdfObject,
}

/// 📦️ Name-keyed `Dict`/`Stream.dict`/`trailer` triple (order-preserving `Vec`, per-entry
/// identity is the key NAME -- first occurrence wins on duplicate keys, matching real PDF
/// dictionaries which practically never repeat a key; documented simplification).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfDictDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<PdfDictModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<PdfDictAdded>,
}

impl PdfDictDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_dict_diff(diff: &PdfDictDiff, base: &[PdfDictEntry]) -> Vec<PdfDictEntry> {
    let mut entries: Vec<PdfDictEntry> = base.to_vec();
    for m in &diff.modified {
        if let Some(pos) = entries.iter().position(|e| e.key == m.key) {
            let old = entries[pos].value.clone();
            entries[pos].value = apply_value_diff(&m.diff, &old);
        }
    }
    for key in &diff.removed {
        if let Some(pos) = entries.iter().position(|e| &e.key == key) {
            entries.remove(pos);
        }
    }
    let mut added_sorted = diff.added.clone();
    added_sorted.sort_by_key(|a| a.index);
    for a in added_sorted {
        entries.insert(a.index, PdfDictEntry { key: a.key, value: a.item });
    }
    entries
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dict_diff_between(a: &[PdfDictEntry], b: &[PdfDictEntry]) -> PdfDictDiff {
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for ae in a {
        match b.iter().find(|be| be.key == ae.key) {
            Some(be) => {
                if let Some(d) = value_diff_between(&ae.value, &be.value) {
                    modified.push(PdfDictModified { key: ae.key.clone(), diff: d });
                }
            }
            None => removed.push(ae.key.clone()),
        }
    }
    let mut added = Vec::new();
    for (i, be) in b.iter().enumerate() {
        if !a.iter().any(|ae| ae.key == be.key) {
            added.push(PdfDictAdded { index: i, key: be.key.clone(), item: be.value.clone() });
        }
    }
    PdfDictDiff { removed, modified, added }
}

/// ➕️ Name-keyed absorb (key identity, non-positional -- mirrors json's `absorb_object_diff`):
/// a `d2`-removal of a `d1`-added key annihilates the add; a `d2`-modify of a `d1`-added key
/// patches the carried payload; a `d2`-modify-of-`d1`-removed key is dropped (illegal, matches
/// `apply`'s no-op rule).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_dict_diff(d1: PdfDictDiff, d2: PdfDictDiff) -> PdfDictDiff {
    let mut removed: Vec<String> = d1.removed;
    let mut modified: Vec<PdfDictModified> = d1.modified;
    let mut added: Vec<PdfDictAdded> = d1.added;
    let mut merged_removed: HashSet<String> = HashSet::new();

    for key in d2.removed {
        if let Some(pos) = added.iter().position(|a| a.key == key) {
            added.remove(pos);
        } else if let Some(pos) = modified.iter().position(|m| m.key == key) {
            modified.remove(pos);
            if merged_removed.insert(key.clone()) {
                removed.push(key);
            }
        } else if merged_removed.insert(key.clone()) {
            removed.push(key);
        }
    }
    for m in d2.modified {
        if let Some(a) = added.iter_mut().find(|a| a.key == m.key) {
            a.item = apply_value_diff(&m.diff, &a.item);
        } else if let Some(pos) = modified.iter().position(|e| e.key == m.key) {
            let combined = absorb_value_diff(modified[pos].diff.clone(), m.diff.clone());
            if is_value_diff_effectively_empty(&combined) {
                modified.remove(pos);
            } else {
                modified[pos].diff = combined;
            }
        } else if !removed.contains(&m.key) {
            modified.push(PdfDictModified { key: m.key, diff: m.diff });
        }
    }
    for a in d2.added {
        added.push(a);
    }
    added.sort_by_key(|a| a.index);
    removed.sort();
    removed.dedup();
    PdfDictDiff { removed, modified, added }
}
//#endregion 🔖️DictDiff

//#region 🔖️ArrayDiff (nested inside PdfValueDiff::Array only)
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfArrayModified {
    pub index: usize,
    pub diff: PdfValueDiff,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfArrayAdded {
    pub index: usize,
    pub item: PdfObject,
}

/// 📦️ Index-keyed `Array` triple.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfArrayDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<PdfArrayModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<PdfArrayAdded>,
}

impl PdfArrayDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_array_diff(diff: &PdfArrayDiff, base: &[PdfObject]) -> Vec<PdfObject> {
    let mut items: Vec<PdfObject> = base.to_vec();
    for m in &diff.modified {
        if let Some(old) = base.get(m.index) {
            if let Some(slot) = items.get_mut(m.index) {
                *slot = apply_value_diff(&m.diff, old);
            }
        }
    }
    let mut removed_sorted = diff.removed.clone();
    removed_sorted.sort_unstable();
    removed_sorted.dedup();
    for idx in removed_sorted.into_iter().rev() {
        if idx < items.len() {
            items.remove(idx);
        }
    }
    let mut added_sorted = diff.added.clone();
    added_sorted.sort_by_key(|a| a.index);
    for a in added_sorted {
        items.insert(a.index, a.item);
    }
    items
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn array_diff_between(a: &[PdfObject], b: &[PdfObject]) -> PdfArrayDiff {
    let min = a.len().min(b.len());
    let mut modified = Vec::new();
    for i in 0..min {
        if let Some(d) = value_diff_between(&a[i], &b[i]) {
            modified.push(PdfArrayModified { index: i, diff: d });
        }
    }
    let removed: Vec<usize> = if a.len() > b.len() { (b.len()..a.len()).collect() } else { Vec::new() };
    let added: Vec<PdfArrayAdded> = if b.len() > a.len() { (a.len()..b.len()).map(|i| PdfArrayAdded { index: i, item: b[i].clone() }).collect() } else { Vec::new() };
    PdfArrayDiff { removed, modified, added }
}

/// ➕️ Index-transported absorb via symbolic position simulation (same shape as json's
/// `absorb_array_diff`, specialized to `PdfObject`/`PdfValueDiff`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_array_diff(d1: PdfArrayDiff, d2: &PdfArrayDiff) -> PdfArrayDiff {
    enum Origin {
        Base(usize),
        D1Added(usize),
    }
    enum AfterSlot {
        Base { orig: usize, diff: Option<PdfValueDiff> },
        D1Added { tag: usize, patch: Option<PdfValueDiff> },
        D2Added(PdfObject),
    }

    let max_ref = d1
        .removed
        .iter()
        .copied()
        .chain(d1.modified.iter().map(|m| m.index))
        .chain(d1.added.iter().map(|a| a.index))
        .chain(d2.removed.iter().copied())
        .chain(d2.modified.iter().map(|m| m.index))
        .chain(d2.added.iter().map(|a| a.index))
        .max()
        .unwrap_or(0);
    let n = max_ref + d1.removed.len() + d2.removed.len() + 64;

    let mut mid: Vec<Origin> = (0..n).map(Origin::Base).collect();
    let mut d1_removed_sorted = d1.removed.clone();
    d1_removed_sorted.sort_unstable();
    d1_removed_sorted.dedup();
    for idx in d1_removed_sorted.iter().rev() {
        if *idx < mid.len() {
            mid.remove(*idx);
        }
    }
    let mut d1_added_order: Vec<usize> = (0..d1.added.len()).collect();
    d1_added_order.sort_by_key(|&tag| d1.added[tag].index);
    for tag in d1_added_order {
        let pos = d1.added[tag].index.min(mid.len());
        mid.insert(pos, Origin::D1Added(tag));
    }
    let d1_modified: HashMap<usize, PdfValueDiff> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();

    let mut after: Vec<AfterSlot> = mid
        .iter()
        .map(|origin| match origin {
            Origin::Base(orig) => AfterSlot::Base { orig: *orig, diff: d1_modified.get(orig).cloned() },
            Origin::D1Added(tag) => AfterSlot::D1Added { tag: *tag, patch: None },
        })
        .collect();

    let mut final_removed: Vec<usize> = d1.removed.clone();
    let mut d2_removed_sorted = d2.removed.clone();
    d2_removed_sorted.sort_unstable();
    d2_removed_sorted.dedup();
    for idx in d2_removed_sorted.iter().rev() {
        if *idx < after.len() {
            match after.remove(*idx) {
                AfterSlot::Base { orig, .. } => final_removed.push(orig),
                AfterSlot::D1Added { .. } => {}
                AfterSlot::D2Added(_) => {}
            }
        }
    }
    for m in &d2.modified {
        if let Some(slot) = after.get_mut(m.index) {
            match slot {
                AfterSlot::Base { diff, .. } => {
                    let combined = match diff.take() {
                        Some(existing) => absorb_value_diff(existing, m.diff.clone()),
                        None => m.diff.clone(),
                    };
                    *diff = if is_value_diff_effectively_empty(&combined) { None } else { Some(combined) };
                }
                AfterSlot::D1Added { patch, .. } => {
                    let combined = match patch.take() {
                        Some(existing) => absorb_value_diff(existing, m.diff.clone()),
                        None => m.diff.clone(),
                    };
                    *patch = if is_value_diff_effectively_empty(&combined) { None } else { Some(combined) };
                }
                AfterSlot::D2Added(_) => {}
            }
        }
    }
    let mut d2_added_order: Vec<usize> = (0..d2.added.len()).collect();
    d2_added_order.sort_by_key(|&tag| d2.added[tag].index);
    for tag in d2_added_order {
        let pos = d2.added[tag].index.min(after.len());
        after.insert(pos, AfterSlot::D2Added(d2.added[tag].item.clone()));
    }

    let mut modified = Vec::new();
    let mut added = Vec::new();
    for (pos, slot) in after.into_iter().enumerate() {
        match slot {
            AfterSlot::Base { orig, diff: Some(diff) } => modified.push(PdfArrayModified { index: orig, diff }),
            AfterSlot::Base { .. } => {}
            AfterSlot::D1Added { tag, patch } => {
                let mut item = d1.added[tag].item.clone();
                if let Some(patch) = patch {
                    item = apply_value_diff(&patch, &item);
                }
                added.push(PdfArrayAdded { index: pos, item });
            }
            AfterSlot::D2Added(item) => added.push(PdfArrayAdded { index: pos, item }),
        }
    }
    final_removed.sort_unstable();
    final_removed.dedup();
    PdfArrayDiff { removed: final_removed, modified, added }
}
//#endregion 🔖️ArrayDiff

//#region 🔖️ValueDiff
/// 🔺️ Recursive diff mirroring [`PdfObject`]'s shape. `Replace` is the fallback used whenever a
/// node's KIND changes between base and next (e.g. `Int` -> `Name`); the other variants are
/// direct/structural diffs used whenever the kind is stable.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum PdfValueDiff {
    /// 🔁️ Whole-node replace -- the node's KIND changed, or a mutation explicitly overwrites it.
    Replace {
        value: PdfObject,
    },
    Bool {
        value: bool,
    },
    Int {
        value: i64,
    },
    Real {
        value: PdfDecimal,
    },
    Str {
        value: Vec<u8>,
    },
    Name {
        value: String,
    },
    Ref {
        value: ObjRef,
    },
    Array {
        diff: PdfArrayDiff,
    },
    Dict {
        diff: PdfDictDiff,
    },
    /// 🌊️ `dict` and decoded logical `data` are independently sparse.
    Stream {
        #[value(default, skip_serializing_if = "Option::is_none")]
        dict: Option<PdfDictDiff>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        data: Option<Vec<u8>>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        filters: Option<Vec<PdfStreamFilter>>,
    },
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_value_diff(diff: &PdfValueDiff, base: &PdfObject) -> PdfObject {
    match diff {
        PdfValueDiff::Replace { value } => value.clone(),
        PdfValueDiff::Bool { value } => PdfObject::Bool(*value),
        PdfValueDiff::Int { value } => PdfObject::Int(*value),
        PdfValueDiff::Real { value } => PdfObject::Real(value.clone()),
        PdfValueDiff::Str { value } => PdfObject::Str(value.clone()),
        PdfValueDiff::Name { value } => PdfObject::Name(value.clone()),
        PdfValueDiff::Ref { value } => PdfObject::Ref(*value),
        PdfValueDiff::Array { diff } => {
            let items: &[PdfObject] = match base {
                PdfObject::Array(a) => a.as_slice(),
                _ => &[],
            };
            PdfObject::Array(apply_array_diff(diff, items))
        }
        PdfValueDiff::Dict { diff } => {
            let entries: &[PdfDictEntry] = match base {
                PdfObject::Dict(d) => d.as_slice(),
                PdfObject::Stream { dict, .. } => dict.as_slice(),
                _ => &[],
            };
            PdfObject::Dict(apply_dict_diff(diff, entries))
        }
        PdfValueDiff::Stream { dict, data, filters } => {
            let (base_dict, base_data, base_filters): (&[PdfDictEntry], &[u8], &[PdfStreamFilter]) = match base {
                PdfObject::Stream { dict, data, filters } => (dict.as_slice(), data.as_slice(), filters.as_slice()),
                _ => (&[], &[], &[]),
            };
            PdfObject::Stream {
                dict: match dict {
                    Some(d) => apply_dict_diff(d, base_dict),
                    None => base_dict.to_vec(),
                },
                data: data.clone().unwrap_or_else(|| base_data.to_vec()),
                filters: filters.clone().unwrap_or_else(|| base_filters.to_vec()),
            }
        }
    }
}

/// 🧭️ State-delta construction: `None` when nodes are equal; a direct field/collection diff when
/// the KIND is stable; `Replace` when it changed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn value_diff_between(a: &PdfObject, b: &PdfObject) -> Option<PdfValueDiff> {
    if a == b {
        return None;
    }
    match (a, b) {
        (PdfObject::Null, PdfObject::Null) => None,
        (PdfObject::Bool(_), PdfObject::Bool(nb)) => Some(PdfValueDiff::Bool { value: *nb }),
        (PdfObject::Int(_), PdfObject::Int(nb)) => Some(PdfValueDiff::Int { value: *nb }),
        (PdfObject::Real(_), PdfObject::Real(nb)) => Some(PdfValueDiff::Real { value: nb.clone() }),
        (PdfObject::Str(_), PdfObject::Str(nb)) => Some(PdfValueDiff::Str { value: nb.clone() }),
        (PdfObject::Name(_), PdfObject::Name(nb)) => Some(PdfValueDiff::Name { value: nb.clone() }),
        (PdfObject::Ref(_), PdfObject::Ref(nb)) => Some(PdfValueDiff::Ref { value: *nb }),
        (PdfObject::Array(av), PdfObject::Array(bv)) => {
            let d = array_diff_between(av, bv);
            if d.is_empty() {
                None
            } else {
                Some(PdfValueDiff::Array { diff: d })
            }
        }
        (PdfObject::Dict(ad), PdfObject::Dict(bd)) => {
            let d = dict_diff_between(ad, bd);
            if d.is_empty() {
                None
            } else {
                Some(PdfValueDiff::Dict { diff: d })
            }
        }
        (PdfObject::Stream { dict: ad, data: adata, filters: af }, PdfObject::Stream { dict: bd, data: bdata, filters: bf }) => {
            let dict_d = {
                let d = dict_diff_between(ad, bd);
                if d.is_empty() {
                    None
                } else {
                    Some(d)
                }
            };
            let data_d = (adata != bdata).then(|| bdata.clone());
            let filters_d = (af != bf).then(|| bf.clone());
            if dict_d.is_none() && data_d.is_none() && filters_d.is_none() {
                None
            } else {
                Some(PdfValueDiff::Stream { dict: dict_d, data: data_d, filters: filters_d })
            }
        }
        _ => Some(PdfValueDiff::Replace { value: b.clone() }),
    }
}

/// 🕳️ Whether a (possibly freshly-absorbed) node diff represents no actual change -- a scalar
/// replace/field diff is never "empty" in isolation (LWW field limitation, matches json's own
/// documented behavior), but an empty collection triple genuinely changes nothing.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_value_diff_effectively_empty(d: &PdfValueDiff) -> bool {
    match d {
        PdfValueDiff::Array { diff } => diff.is_empty(),
        PdfValueDiff::Dict { diff } => diff.is_empty(),
        PdfValueDiff::Stream { dict, data, filters } => dict.is_none() && data.is_none() && filters.is_none(),
        _ => false,
    }
}

/// ➕️ Diff-level absorb: `d2` always wins on a full `Replace`; a `Replace` in `d1` gets `d2`
/// baked into its known literal value via `apply_value_diff`; otherwise both sides share the
/// same node KIND (guaranteed by construction against the real intervening mid state) and
/// compose per-kind, recursing into collections.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_value_diff(d1: PdfValueDiff, d2: PdfValueDiff) -> PdfValueDiff {
    if matches!(d2, PdfValueDiff::Replace { .. }) {
        return d2;
    }
    if let PdfValueDiff::Replace { value } = d1 {
        return PdfValueDiff::Replace { value: apply_value_diff(&d2, &value) };
    }
    match (d1, d2) {
        (PdfValueDiff::Bool { .. }, PdfValueDiff::Bool { value }) => PdfValueDiff::Bool { value },
        (PdfValueDiff::Int { .. }, PdfValueDiff::Int { value }) => PdfValueDiff::Int { value },
        (PdfValueDiff::Real { .. }, PdfValueDiff::Real { value }) => PdfValueDiff::Real { value },
        (PdfValueDiff::Str { .. }, PdfValueDiff::Str { value }) => PdfValueDiff::Str { value },
        (PdfValueDiff::Name { .. }, PdfValueDiff::Name { value }) => PdfValueDiff::Name { value },
        (PdfValueDiff::Ref { .. }, PdfValueDiff::Ref { value }) => PdfValueDiff::Ref { value },
        (PdfValueDiff::Array { diff: a1 }, PdfValueDiff::Array { diff: a2 }) => PdfValueDiff::Array { diff: absorb_array_diff(a1, &a2) },
        (PdfValueDiff::Dict { diff: d1 }, PdfValueDiff::Dict { diff: d2 }) => PdfValueDiff::Dict { diff: absorb_dict_diff(d1, d2) },
        (PdfValueDiff::Stream { dict: d1, data: da1, filters: f1 }, PdfValueDiff::Stream { dict: d2, data: da2, filters: f2 }) => PdfValueDiff::Stream {
            dict: match (d1, d2) {
                (None, x) => x,
                (x, None) => x,
                (Some(a), Some(b)) => Some(absorb_dict_diff(a, b)),
            },
            data: da2.or(da1),
            filters: f2.or(f1),
        },
        (_, other) => other, // defensive LWW fallback; real sequential diffs never hit this arm.
    }
}
//#endregion 🔖️ValueDiff

//#region 🔖️ObjectsTriple
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfObjectModified {
    pub id: ObjRef,
    pub diff: PdfValueDiff,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfObjectAdded {
    pub index: usize,
    pub id: ObjRef,
    pub value: PdfObject,
}

/// 📦️ `(id,gen)`-keyed `objects` triple (the recipe's "numeric id" key kind; `ObjRef` -- the
/// `PdfIndirectObject`'s own real key -- is used whole rather than splitting to bare `id`, since
/// a distinct `gen` genuinely identifies a distinct indirect object per spec).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfObjectsDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<ObjRef>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<PdfObjectModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<PdfObjectAdded>,
}

impl PdfObjectsDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

use crate::standards::v1_7::subsets::base::schema::snapshot::PdfIndirectObject;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_objects_diff(diff: &PdfObjectsDiff, base: &[PdfIndirectObject]) -> Vec<PdfIndirectObject> {
    let mut objects: Vec<PdfIndirectObject> = base.to_vec();
    for m in &diff.modified {
        if let Some(pos) = objects.iter().position(|o| o.id == m.id) {
            let old = objects[pos].value.clone();
            objects[pos].value = apply_value_diff(&m.diff, &old);
        }
    }
    for id in &diff.removed {
        if let Some(pos) = objects.iter().position(|o| &o.id == id) {
            objects.remove(pos);
        }
    }
    let mut added_sorted = diff.added.clone();
    added_sorted.sort_by_key(|a| a.index);
    for a in added_sorted {
        objects.insert(a.index, PdfIndirectObject { id: a.id, value: a.value });
    }
    objects
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn objects_diff_between(a: &[PdfIndirectObject], b: &[PdfIndirectObject]) -> PdfObjectsDiff {
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for ao in a {
        match b.iter().find(|bo| bo.id == ao.id) {
            Some(bo) => {
                if let Some(d) = value_diff_between(&ao.value, &bo.value) {
                    modified.push(PdfObjectModified { id: ao.id, diff: d });
                }
            }
            None => removed.push(ao.id),
        }
    }
    let mut added = Vec::new();
    for (i, bo) in b.iter().enumerate() {
        if !a.iter().any(|ao| ao.id == bo.id) {
            added.push(PdfObjectAdded { index: i, id: bo.id, value: bo.value.clone() });
        }
    }
    PdfObjectsDiff { removed, modified, added }
}

/// ➕️ Id-keyed absorb (key identity, non-positional -- mirrors json's `absorb_object_diff` and
/// this file's own `absorb_dict_diff`, keyed by `ObjRef` instead of `String`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_law_objects_diff(d1: PdfObjectsDiff, d2: PdfObjectsDiff) -> PdfObjectsDiff {
    let mut removed: Vec<ObjRef> = d1.removed;
    let mut modified: Vec<PdfObjectModified> = d1.modified;
    let mut added: Vec<PdfObjectAdded> = d1.added;
    let mut merged_removed: HashSet<ObjRef> = HashSet::new();

    for id in d2.removed {
        if let Some(pos) = added.iter().position(|a| a.id == id) {
            added.remove(pos);
        } else if let Some(pos) = modified.iter().position(|m| m.id == id) {
            modified.remove(pos);
            if merged_removed.insert(id) {
                removed.push(id);
            }
        } else if merged_removed.insert(id) {
            removed.push(id);
        }
    }
    for m in d2.modified {
        if let Some(a) = added.iter_mut().find(|a| a.id == m.id) {
            a.value = apply_value_diff(&m.diff, &a.value);
        } else if let Some(pos) = modified.iter().position(|e| e.id == m.id) {
            let combined = absorb_value_diff(modified[pos].diff.clone(), m.diff.clone());
            if is_value_diff_effectively_empty(&combined) {
                modified.remove(pos);
            } else {
                modified[pos].diff = combined;
            }
        } else if !removed.contains(&m.id) {
            modified.push(PdfObjectModified { id: m.id, diff: m.diff });
        }
    }
    for a in d2.added {
        added.push(a);
    }
    added.sort_by_key(|a| a.index);
    PdfObjectsDiff { removed, modified, added }
}
//#endregion 🔖️ObjectsTriple

//#region 🔖️PathAddressing
/// 🧭️ One step of a `NodePath`-style address into ONE object's `PdfObject` tree -- used by
/// `SetDictEntry`/`RemoveDictEntry` mutations (recipe: "path addresses nesting inside one
/// object's PdfValue tree, same NodePath-style addressing xml/svg use"). Per ISO 32000-1, a raw
/// `Stream` can only ever be an indirect object's OWN top-level value (never nested inside an
/// Array/Dict as a value -- that requires an indirect `Ref`), so only `path == []` can possibly
/// address a `Stream`'s dict; every deeper step is guaranteed `Array`/`Dict`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum PdfPathSegment {
    ArrayIndex { index: usize },
    DictKey { key: String },
}

/// 🔍️ Walks `path` from `root`, returning the container reached (an `Array`/`Dict`/`Stream`) --
/// or `None` on an out-of-range/kind-mismatched step (graceful no-op upstream).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn resolve_value<'a>(root: &'a PdfObject, path: &[PdfPathSegment]) -> Option<&'a PdfObject> {
    let mut current = root;
    for seg in path {
        current = match (seg, current) {
            (PdfPathSegment::ArrayIndex { index }, PdfObject::Array(items)) => items.get(*index)?,
            (PdfPathSegment::DictKey { key }, PdfObject::Dict(entries)) => &entries.iter().find(|e| &e.key == key)?.value,
            (PdfPathSegment::DictKey { key }, PdfObject::Stream { dict, .. }) => &dict.iter().find(|e| &e.key == key)?.value,
            _ => return None,
        };
    }
    Some(current)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dict_entries_of(value: &PdfObject) -> Option<&[PdfDictEntry]> {
    match value {
        PdfObject::Dict(d) => Some(d.as_slice()),
        PdfObject::Stream { dict, .. } => Some(dict.as_slice()),
        _ => None,
    }
}

/// 🧩️ Wraps a leaf `PdfDictDiff` (a modification to the dict/stream-dict located at `path` inside
/// object `id`) into a full `PdfDiff`, folding `path` from innermost to outermost. Only the
/// OUTERMOST step (`path == []`, i.e. the object's own top-level value) can be a `Stream` --
/// every step beyond that is guaranteed `Dict`/`Array` per `PdfPathSegment`'s own doc comment.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_at_object_path(id: ObjRef, path: &[PdfPathSegment], is_root_stream: bool, leaf: PdfDictDiff) -> PdfDiff {
    let mut node = if is_root_stream { PdfValueDiff::Stream { dict: Some(leaf), data: None, filters: None } } else { PdfValueDiff::Dict { diff: leaf } };
    for seg in path.iter().rev() {
        node = match seg {
            PdfPathSegment::ArrayIndex { index } => PdfValueDiff::Array { diff: PdfArrayDiff { modified: vec![PdfArrayModified { index: *index, diff: node }], ..Default::default() } },
            PdfPathSegment::DictKey { key } => PdfValueDiff::Dict { diff: PdfDictDiff { modified: vec![PdfDictModified { key: key.clone(), diff: node }], ..Default::default() } },
        };
    }
    PdfDiff { objects: Some(PdfObjectsDiff { modified: vec![PdfObjectModified { id, diff: node }], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️PathAddressing
fn validate_index_triple(base_len: usize, removed: &[usize], modified: &[usize], added: &[usize], field: &str) -> MutationApplyResult<()> {
    let mut removed_set = HashSet::new();
    for &index in removed {
        if index >= base_len {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "indexed removal target does not exist").at(vec![field.to_string(), index.to_string()]));
        }
        if !removed_set.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "indexed removal target is repeated").at(vec![field.to_string(), index.to_string()]));
        }
    }
    let mut modified_set = HashSet::new();
    for &index in modified {
        if index >= base_len {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "indexed modification target does not exist").at(vec![field.to_string(), index.to_string()]));
        }
        if removed_set.contains(&index) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "indexed modification targets a removed item").at(vec![field.to_string(), index.to_string()]));
        }
        if !modified_set.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "indexed modification target is repeated").at(vec![field.to_string(), index.to_string()]));
        }
    }
    let final_len = base_len - removed_set.len() + added.len();
    let mut added_set = HashSet::new();
    for &index in added {
        if index >= final_len {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "indexed addition is outside the final collection").at(vec![field.to_string(), index.to_string()]));
        }
        if !added_set.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "indexed addition occupies a repeated final position").at(vec![field.to_string(), index.to_string()]));
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_named_keys<K: Eq + std::hash::Hash + Clone>(base: &[K], removed: &[K], modified: &[K], added: &[K], field: &str) -> MutationApplyResult<()> {
    let base_set: HashSet<K> = base.iter().cloned().collect();
    if base_set.len() != base.len() {
        return Err(MutationApplyError::new("mutation.apply.duplicate-target", "base collection contains duplicate keys").at([field]));
    }
    let mut removed_set = HashSet::new();
    for key in removed {
        if !base_set.contains(key) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "named removal target does not exist").at([field]));
        }
        if !removed_set.insert(key.clone()) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "named removal target is repeated").at([field]));
        }
    }
    let mut modified_set = HashSet::new();
    for key in modified {
        if !base_set.contains(key) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "named modification target does not exist").at([field]));
        }
        if removed_set.contains(key) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "named modification targets a removed item").at([field]));
        }
        if !modified_set.insert(key.clone()) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "named modification target is repeated").at([field]));
        }
    }
    let mut added_set = HashSet::new();
    for key in added {
        if base_set.contains(key) || !added_set.insert(key.clone()) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "named addition target already exists").at([field]));
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_dict_diff(diff: &PdfDictDiff, base: &[PdfDictEntry]) -> MutationApplyResult<()> {
    validate_named_keys(
        &base.iter().map(|entry| entry.key.clone()).collect::<Vec<_>>(),
        &diff.removed,
        &diff.modified.iter().map(|item| item.key.clone()).collect::<Vec<_>>(),
        &diff.added.iter().map(|item| item.key.clone()).collect::<Vec<_>>(),
        "dict",
    )?;
    for modified in &diff.modified {
        let entry = base.iter().find(|entry| entry.key == modified.key).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "dictionary modification target does not exist").at(vec!["dict".to_string(), modified.key.clone()]))?;
        validate_value_diff(&modified.diff, &entry.value).map_err(|error| error.under(vec!["dict".to_string(), modified.key.clone()]))?;
    }
    let removed_indices: Vec<usize> = (0..diff.removed.len()).collect();
    validate_index_triple(base.len(), &removed_indices, &[], &diff.added.iter().map(|item| item.index).collect::<Vec<_>>(), "dict")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_array_diff(diff: &PdfArrayDiff, base: &[PdfObject]) -> MutationApplyResult<()> {
    validate_index_triple(base.len(), &diff.removed, &diff.modified.iter().map(|item| item.index).collect::<Vec<_>>(), &diff.added.iter().map(|item| item.index).collect::<Vec<_>>(), "array")?;
    for modified in &diff.modified {
        let item = base.get(modified.index).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "array modification target does not exist").at(vec!["array".to_string(), modified.index.to_string()]))?;
        validate_value_diff(&modified.diff, item).map_err(|error| error.under(vec!["array".to_string(), modified.index.to_string()]))?;
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_value_diff(diff: &PdfValueDiff, base: &PdfObject) -> MutationApplyResult<()> {
    match diff {
        PdfValueDiff::Replace { .. } => Ok(()),
        PdfValueDiff::Bool { .. } if matches!(base, PdfObject::Bool(_)) => Ok(()),
        PdfValueDiff::Int { .. } if matches!(base, PdfObject::Int(_)) => Ok(()),
        PdfValueDiff::Real { .. } if matches!(base, PdfObject::Real(_)) => Ok(()),
        PdfValueDiff::Str { .. } if matches!(base, PdfObject::Str(_)) => Ok(()),
        PdfValueDiff::Name { .. } if matches!(base, PdfObject::Name(_)) => Ok(()),
        PdfValueDiff::Ref { .. } if matches!(base, PdfObject::Ref(_)) => Ok(()),
        PdfValueDiff::Array { diff } => match base {
            PdfObject::Array(items) => validate_array_diff(diff, items),
            _ => Err(MutationApplyError::new("mutation.apply.kind-mismatch", "array diff targets a non-array value")),
        },
        PdfValueDiff::Dict { diff } => match base {
            PdfObject::Dict(entries) => validate_dict_diff(diff, entries),
            _ => Err(MutationApplyError::new("mutation.apply.kind-mismatch", "dictionary diff targets a non-dictionary value")),
        },
        PdfValueDiff::Stream { dict, .. } => match base {
            PdfObject::Stream { dict: entries, .. } => {
                if let Some(dict) = dict {
                    validate_dict_diff(dict, entries)?;
                }
                Ok(())
            }
            _ => Err(MutationApplyError::new("mutation.apply.kind-mismatch", "stream diff targets a non-stream value")),
        },
        _ => Err(MutationApplyError::new("mutation.apply.kind-mismatch", "scalar diff targets a value of another kind")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_objects_diff(diff: &PdfObjectsDiff, base: &[PdfIndirectObject]) -> MutationApplyResult<()> {
    let keys: Vec<ObjRef> = base.iter().map(|object| object.id).collect();
    validate_named_keys(&keys, &diff.removed, &diff.modified.iter().map(|item| item.id).collect::<Vec<_>>(), &diff.added.iter().map(|item| item.id).collect::<Vec<_>>(), "objects")?;
    let removed_indices: Vec<usize> = (0..diff.removed.len()).collect();
    validate_index_triple(base.len(), &removed_indices, &[], &diff.added.iter().map(|item| item.index).collect::<Vec<_>>(), "objects")?;
    for modified in &diff.modified {
        let object = base.iter().find(|object| object.id == modified.id).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "object modification target does not exist").at(["objects"]))?;
        validate_value_diff(&modified.diff, &object.value).map_err(|error| error.under(["objects"]))?;
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.pdf.1.7`. `schema` is an identity field and is never diffed. `info` is a
/// WEAK value struct (whole-value replaced, never sub-diffed).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pdf.1.7.diff")]
pub struct PdfDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub declared_version: Option<String>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub pages: Option<PdfPagesDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub fonts: Option<PdfKeyedDiff<PdfFont>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub images: Option<PdfKeyedDiff<PdfImage>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub forms: Option<PdfKeyedDiff<PdfFormXObject>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub ext_g_states: Option<PdfKeyedDiff<PdfExtGState>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub shadings: Option<PdfKeyedDiff<PdfShading>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub patterns: Option<PdfKeyedDiff<PdfPattern>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub color_spaces: Option<PdfKeyedDiff<PdfNamedColorSpace>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PdfKeyedDiff<PdfNamedProperties>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub outlines: Option<PdfIndexedDiff<PdfOutlineItem>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub named_destinations: Option<PdfIndexedDiff<PdfNamedDestination>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub page_labels: Option<PdfIndexedDiff<PdfPageLabelRange>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub embedded_files: Option<PdfKeyedDiff<PdfEmbeddedFile>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub output_intents: Option<PdfIndexedDiff<PdfOutputIntent>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub acro_form: Option<PdfSet<PdfAcroForm>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub optional_content: Option<PdfSet<PdfOptionalContent>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub page_layout: Option<PdfSet<PdfPageLayout>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub page_mode: Option<PdfSet<PdfPageMode>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub viewer_preferences: Option<PdfSet<PdfViewerPreferences>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub open_action: Option<PdfSet<PdfOpenAction>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<PdfSet<String>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mark_info: Option<PdfSet<PdfMarkInfo>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PdfSet<String>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub document_id: Option<PdfSet<[Vec<u8>; 2]>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<PdfSet<PdfEncryption>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<PdfInfo>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub catalog_extra: Option<PdfDictDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub objects: Option<PdfObjectsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub trailer: Option<PdfDictDiff>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_pages_diff(diff: &PdfPagesDiff, base: &[PdfPage]) -> MutationApplyResult<()> {
    validate_index_triple(base.len(), &diff.removed, &diff.modified.iter().map(|item| item.index).collect::<Vec<_>>(), &diff.added.iter().map(|item| item.index).collect::<Vec<_>>(), "pages")?;
    for modified in &diff.modified {
        validate_page_diff(&modified.diff, &base[modified.index]).map_err(|error| error.under(vec!["pages".to_string(), modified.index.to_string()]))?;
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_pdf_diff(diff: &PdfDiff, base: &PdfSnapshot) -> MutationApplyResult<()> {
    if let Some(pages) = &diff.pages {
        validate_pages_diff(pages, &base.pages).map_err(|error| error.under(["pages"]))?;
    }
    macro_rules! keyed {
        ($($field:ident),*) => {
            $(if let Some(lane) = &diff.$field {
                lane.validate(&base.$field, stringify!($field))?;
            })*
        };
    }
    keyed!(fonts, images, forms, ext_g_states, shadings, patterns, color_spaces, properties, embedded_files);
    macro_rules! indexed {
        ($($field:ident),*) => {
            $(if let Some(lane) = &diff.$field {
                lane.validate(base.$field.len(), stringify!($field))?;
            })*
        };
    }
    indexed!(outlines, named_destinations, page_labels, output_intents);
    if let Some(objects) = &diff.objects {
        validate_objects_diff(objects, &base.objects).map_err(|error| error.under(["objects"]))?;
    }
    if let Some(trailer) = &diff.trailer {
        validate_dict_diff(trailer, &base.trailer).map_err(|error| error.under(["trailer"]))?;
    }
    if let Some(extra) = &diff.catalog_extra {
        validate_dict_diff(extra, &base.catalog_extra).map_err(|error| error.under(["catalogExtra"]))?;
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_pdf_diff_unchecked(diff: &PdfDiff, base: &PdfSnapshot) -> PdfSnapshot {
    let mut next = base.clone();
    if let Some(v) = &diff.declared_version {
        next.declared_version = v.clone();
    }
    if let Some(v) = &diff.info {
        next.info = v.clone();
    }
    if let Some(pd) = &diff.pages {
        next.pages = apply_pages_diff(pd, &base.pages);
    }
    macro_rules! lanes {
        ($($field:ident),*) => {
            $(if let Some(lane) = &diff.$field {
                next.$field = lane.apply(&base.$field);
            })*
        };
    }
    lanes!(fonts, images, forms, ext_g_states, shadings, patterns, color_spaces, properties, embedded_files, outlines, named_destinations, page_labels, output_intents);
    apply_tri(&mut next.acro_form, &diff.acro_form);
    apply_tri(&mut next.optional_content, &diff.optional_content);
    apply_tri(&mut next.page_layout, &diff.page_layout);
    apply_tri(&mut next.page_mode, &diff.page_mode);
    apply_tri(&mut next.viewer_preferences, &diff.viewer_preferences);
    apply_tri(&mut next.open_action, &diff.open_action);
    apply_tri(&mut next.language, &diff.language);
    apply_tri(&mut next.mark_info, &diff.mark_info);
    apply_tri(&mut next.metadata, &diff.metadata);
    apply_tri(&mut next.document_id, &diff.document_id);
    apply_tri(&mut next.encryption, &diff.encryption);
    if let Some(extra) = &diff.catalog_extra {
        next.catalog_extra = apply_dict_diff(extra, &base.catalog_extra);
    }
    if let Some(od) = &diff.objects {
        next.objects = apply_objects_diff(od, &base.objects);
    }
    if let Some(td) = &diff.trailer {
        next.trailer = apply_dict_diff(td, &base.trailer);
    }
    next
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_option<T>(slot: &mut Option<T>, other: Option<T>, merge: impl FnOnce(T, T) -> Option<T>) {
    *slot = match (slot.take(), other) {
        (None, b) => b,
        (a, None) => a,
        (Some(a), Some(b)) => merge(a, b),
    };
}

impl MutationDiff<PdfSnapshot> for PdfDiff {
    fn apply(&self, base: &PdfSnapshot) -> MutationApplyResult<PdfSnapshot> {
        validate_pdf_diff(self, base)?;
        Ok(apply_pdf_diff_unchecked(self, base))
    }

    /// ➕️ Structural, total, base-free sequential-coalesce absorb (`## Absorb` contract).
    /// Scalars and tri-states: LWW. Triples: composed via their own key/index-transported absorb.
    fn absorb(&mut self, other: Self) {
        if other.declared_version.is_some() {
            self.declared_version = other.declared_version;
        }
        if other.info.is_some() {
            self.info = other.info;
        }
        absorb_option(&mut self.pages, other.pages, |a, b| {
            let m = absorb_law_pages_diff(a, &b);
            (!m.is_empty()).then_some(m)
        });
        macro_rules! lanes {
            ($($field:ident),*) => {
                $(absorb_option(&mut self.$field, other.$field, |a, b| {
                    let m = a.absorb(b);
                    (!m.is_empty()).then_some(m)
                });)*
            };
        }
        lanes!(fonts, images, forms, ext_g_states, shadings, patterns, color_spaces, properties, embedded_files, outlines, named_destinations, page_labels, output_intents);
        macro_rules! lww {
            ($($field:ident),*) => {
                $(if other.$field.is_some() {
                    self.$field = other.$field;
                })*
            };
        }
        lww!(acro_form, optional_content, page_layout, page_mode, viewer_preferences, open_action, language, mark_info, metadata, document_id, encryption);
        absorb_option(&mut self.catalog_extra, other.catalog_extra, |a, b| {
            let m = absorb_dict_diff(a, b);
            (!m.is_empty()).then_some(m)
        });
        absorb_option(&mut self.objects, other.objects, |a, b| {
            let m = absorb_law_objects_diff(a, b);
            (!m.is_empty()).then_some(m)
        });
        absorb_option(&mut self.trailer, other.trailer, |a, b| {
            let m = absorb_dict_diff(a, b);
            (!m.is_empty()).then_some(m)
        });
    }
}

impl DiffAlgebra<PdfSnapshot> for PdfDiff {
    /// 🔁️ Diff-level undo, derived generically from `between` (correct by construction): the
    /// state delta from `self.apply(base)` back to `base`.
    fn inverse(&self, base: &PdfSnapshot) -> Self {
        let mid = apply_pdf_diff_unchecked(self, base);
        Self::between(&mid, base)
    }

    /// 🧭️ State delta (compose `GetXDiff`): `pages` positionally matched, id collections by key,
    /// `objects` and `trailer` matched by their real keys (`ObjRef`/dict key name).
    fn between(base: &PdfSnapshot, other: &PdfSnapshot) -> Self {
        fn keyed<T: Clone + PartialEq + Keyed>(a: &[T], b: &[T]) -> Option<PdfKeyedDiff<T>> {
            let d = PdfKeyedDiff::between(a, b);
            (!d.is_empty()).then_some(d)
        }
        fn indexed<T: Clone + PartialEq>(a: &[T], b: &[T]) -> Option<PdfIndexedDiff<T>> {
            let d = PdfIndexedDiff::between(a, b);
            (!d.is_empty()).then_some(d)
        }
        fn dict(a: &[PdfDictEntry], b: &[PdfDictEntry]) -> Option<PdfDictDiff> {
            let d = dict_diff_between(a, b);
            (!d.is_empty()).then_some(d)
        }
        let pages = {
            let d = pages_diff_between(&base.pages, &other.pages);
            (!d.is_empty()).then_some(d)
        };
        let objects = {
            let d = objects_diff_between(&base.objects, &other.objects);
            (!d.is_empty()).then_some(d)
        };
        PdfDiff {
            declared_version: (base.declared_version != other.declared_version).then(|| other.declared_version.clone()),
            pages,
            fonts: keyed(&base.fonts, &other.fonts),
            images: keyed(&base.images, &other.images),
            forms: keyed(&base.forms, &other.forms),
            ext_g_states: keyed(&base.ext_g_states, &other.ext_g_states),
            shadings: keyed(&base.shadings, &other.shadings),
            patterns: keyed(&base.patterns, &other.patterns),
            color_spaces: keyed(&base.color_spaces, &other.color_spaces),
            properties: keyed(&base.properties, &other.properties),
            outlines: indexed(&base.outlines, &other.outlines),
            named_destinations: indexed(&base.named_destinations, &other.named_destinations),
            page_labels: indexed(&base.page_labels, &other.page_labels),
            embedded_files: keyed(&base.embedded_files, &other.embedded_files),
            output_intents: indexed(&base.output_intents, &other.output_intents),
            acro_form: tri(&base.acro_form, &other.acro_form),
            optional_content: tri(&base.optional_content, &other.optional_content),
            page_layout: tri(&base.page_layout, &other.page_layout),
            page_mode: tri(&base.page_mode, &other.page_mode),
            viewer_preferences: tri(&base.viewer_preferences, &other.viewer_preferences),
            open_action: tri(&base.open_action, &other.open_action),
            language: tri(&base.language, &other.language),
            mark_info: tri(&base.mark_info, &other.mark_info),
            metadata: tri(&base.metadata, &other.metadata),
            document_id: tri(&base.document_id, &other.document_id),
            encryption: tri(&base.encryption, &other.encryption),
            info: (base.info != other.info).then(|| other.info.clone()),
            catalog_extra: dict(&base.catalog_extra, &other.catalog_extra),
            objects,
            trailer: dict(&base.trailer, &other.trailer),
        }
    }

    fn is_empty(&self) -> bool {
        self == &PdfDiff::default()
    }
}
//#endregion 🔖️Diff

//#region 🔖️MutationDiffBuilders
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn page_patch(index: usize, diff: PdfPageDiff) -> PdfDiff {
    PdfDiff { pages: Some(PdfPagesDiff { modified: vec![PdfPageModified { index, diff }], ..Default::default() }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_page(index: usize, page: PdfPage) -> PdfDiff {
    PdfDiff { pages: Some(PdfPagesDiff { added: vec![PdfPageAdded { index, page }], ..Default::default() }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_page(index: usize) -> PdfDiff {
    PdfDiff { pages: Some(PdfPagesDiff { removed: vec![index], ..Default::default() }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_page_media_box(index: usize, media_box: PdfRect) -> PdfDiff {
    page_patch(index, PdfPageDiff { media_box: Some(media_box), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_page_crop_box(index: usize, crop_box: Option<PdfRect>) -> PdfDiff {
    page_patch(index, PdfPageDiff { crop_box: Some(PdfSet::from_option(&crop_box)), ..Default::default() })
}
/// 📐 Sets one of the optional page boxes (`bleed`, `trim`, `art`) or clears it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_page_box(index: usize, kind: PdfPageBox, rect: Option<PdfRect>) -> PdfDiff {
    let set = Some(PdfSet::from_option(&rect));
    page_patch(
        index,
        match kind {
            PdfPageBox::Crop => PdfPageDiff { crop_box: set, ..Default::default() },
            PdfPageBox::Bleed => PdfPageDiff { bleed_box: set, ..Default::default() },
            PdfPageBox::Trim => PdfPageDiff { trim_box: set, ..Default::default() },
            PdfPageBox::Art => PdfPageDiff { art_box: set, ..Default::default() },
        },
    )
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_page_rotation(index: usize, rotation: i32) -> PdfDiff {
    page_patch(index, PdfPageDiff { rotate: Some(rotation), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_page_user_unit(index: usize, user_unit: Option<f64>) -> PdfDiff {
    page_patch(index, PdfPageDiff { user_unit: Some(PdfSet::from_option(&user_unit)), ..Default::default() })
}
/// ✏️️ Replaces page `index`'s whole content (computed against `base` so the diff stays sparse
/// where operators agree).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_page_content(base: &PdfSnapshot, index: usize, content: &[PdfOp]) -> PdfDiff {
    let current: &[PdfOp] = base.pages.get(index).map(|page| page.content.as_slice()).unwrap_or(&[]);
    let content = PdfIndexedDiff::between(current, content);
    if content.is_empty() {
        return PdfDiff::default();
    }
    page_patch(index, PdfPageDiff { content: Some(content), ..Default::default() })
}
/// ➕️ Appends operators to page `index`'s content.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_append_page_content(base: &PdfSnapshot, index: usize, content: &[PdfOp]) -> PdfDiff {
    let start = base.pages.get(index).map(|page| page.content.len()).unwrap_or(0);
    page_patch(index, PdfPageDiff { content: Some(PdfIndexedDiff { added: content.iter().enumerate().map(|(offset, op)| PdfIndexedItem { index: start + offset, value: op.clone() }).collect(), ..Default::default() }), ..Default::default() })
}
/// ➕️ Inserts operators at position `at` of page `index`'s content.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_content(index: usize, at: usize, content: &[PdfOp]) -> PdfDiff {
    page_patch(index, PdfPageDiff { content: Some(PdfIndexedDiff { added: content.iter().enumerate().map(|(offset, op)| PdfIndexedItem { index: at + offset, value: op.clone() }).collect(), ..Default::default() }), ..Default::default() })
}
/// 🗑️ Removes `count` operators from position `at` of page `index`'s content.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_content(index: usize, at: usize, count: usize) -> PdfDiff {
    page_patch(index, PdfPageDiff { content: Some(PdfIndexedDiff { removed: (at..at + count).collect(), ..Default::default() }), ..Default::default() })
}
/// ✏️️ Replaces the operator at `at` of page `index`'s content.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_replace_content(index: usize, at: usize, op: PdfOp) -> PdfDiff {
    page_patch(index, PdfPageDiff { content: Some(PdfIndexedDiff { modified: vec![PdfIndexedItem { index: at, value: op }], ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_annotation(index: usize, at: usize, annotation: PdfAnnotation) -> PdfDiff {
    page_patch(index, PdfPageDiff { annotations: Some(PdfIndexedDiff { added: vec![PdfIndexedItem { index: at, value: annotation }], ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_annotation(index: usize, at: usize) -> PdfDiff {
    page_patch(index, PdfPageDiff { annotations: Some(PdfIndexedDiff { removed: vec![at], ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_annotation(index: usize, at: usize, annotation: PdfAnnotation) -> PdfDiff {
    page_patch(index, PdfPageDiff { annotations: Some(PdfIndexedDiff { modified: vec![PdfIndexedItem { index: at, value: annotation }], ..Default::default() }), ..Default::default() })
}
/// 🔀️ Moves the page at BASE-state index `from` to FINAL-state index `to` -- `removed`/`added`
/// compose the move (no dedicated "moved" slot on `PdfPagesDiff`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_move_page(base: &PdfSnapshot, from: usize, to: usize) -> PdfDiff {
    let Some(page) = base.pages.get(from) else { return PdfDiff::default() };
    let final_to = to.min(base.pages.len().saturating_sub(1));
    if from == final_to {
        return PdfDiff::default();
    }
    PdfDiff { pages: Some(PdfPagesDiff { removed: vec![from], added: vec![PdfPageAdded { index: final_to, page: page.clone() }], ..Default::default() }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_info(info: PdfInfo) -> PdfDiff {
    PdfDiff { info: Some(info), ..Default::default() }
}

/// 🆔 Upsert of one keyed collection item: `modified` when the key exists in `base`, `added`
/// at the end otherwise.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn keyed_upsert<T: Clone + PartialEq + Keyed>(base: &[T], value: T) -> Option<PdfKeyedDiff<T>> {
    match base.iter().find(|item| item.key() == value.key()) {
        Some(existing) if *existing == value => None,
        Some(_) => Some(PdfKeyedDiff { modified: vec![PdfKeyedItem { key: value.key().to_string(), value }], ..Default::default() }),
        None => Some(PdfKeyedDiff { added: vec![PdfIndexedItem { index: base.len(), value }], ..Default::default() }),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn keyed_remove<T: Clone + PartialEq + Keyed>(base: &[T], key: &str) -> Option<PdfKeyedDiff<T>> {
    base.iter().any(|item| item.key() == key).then(|| PdfKeyedDiff { removed: vec![key.to_string()], ..Default::default() })
}
macro_rules! keyed_builders {
    ($($set:ident / $remove:ident => $field:ident : $ty:ty),* $(,)?) => {
        $(
            // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
            pub fn $set(base: &PdfSnapshot, value: $ty) -> PdfDiff {
                PdfDiff { $field: keyed_upsert(&base.$field, value), ..Default::default() }
            }
            // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
            pub fn $remove(base: &PdfSnapshot, key: &str) -> PdfDiff {
                PdfDiff { $field: keyed_remove(&base.$field, key), ..Default::default() }
            }
        )*
    };
}
keyed_builders!(
    diff_set_font / diff_remove_font => fonts: PdfFont,
    diff_set_image / diff_remove_image => images: PdfImage,
    diff_set_form / diff_remove_form => forms: PdfFormXObject,
    diff_set_ext_g_state / diff_remove_ext_g_state => ext_g_states: PdfExtGState,
    diff_set_shading / diff_remove_shading => shadings: PdfShading,
    diff_set_pattern / diff_remove_pattern => patterns: PdfPattern,
    diff_set_color_space / diff_remove_color_space => color_spaces: PdfNamedColorSpace,
    diff_set_properties / diff_remove_properties => properties: PdfNamedProperties,
    diff_set_embedded_file / diff_remove_embedded_file => embedded_files: PdfEmbeddedFile,
);
/// 📑 Replaces the outline tree.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_outlines(base: &PdfSnapshot, outlines: &[PdfOutlineItem]) -> PdfDiff {
    let d = PdfIndexedDiff::between(&base.outlines, outlines);
    PdfDiff { outlines: (!d.is_empty()).then_some(d), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_named_destination(base: &PdfSnapshot, destination: PdfNamedDestination) -> PdfDiff {
    let d = match base.named_destinations.iter().position(|item| item.name == destination.name) {
        Some(index) if base.named_destinations[index] == destination => return PdfDiff::default(),
        Some(index) => PdfIndexedDiff { modified: vec![PdfIndexedItem { index, value: destination }], ..Default::default() },
        None => PdfIndexedDiff { added: vec![PdfIndexedItem { index: base.named_destinations.len(), value: destination }], ..Default::default() },
    };
    PdfDiff { named_destinations: Some(d), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_named_destination(base: &PdfSnapshot, name: &str) -> PdfDiff {
    match base.named_destinations.iter().position(|item| item.name == name) {
        Some(index) => PdfDiff { named_destinations: Some(PdfIndexedDiff { removed: vec![index], ..Default::default() }), ..Default::default() },
        None => PdfDiff::default(),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_page_labels(base: &PdfSnapshot, labels: &[PdfPageLabelRange]) -> PdfDiff {
    let d = PdfIndexedDiff::between(&base.page_labels, labels);
    PdfDiff { page_labels: (!d.is_empty()).then_some(d), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_output_intents(base: &PdfSnapshot, intents: &[PdfOutputIntent]) -> PdfDiff {
    let d = PdfIndexedDiff::between(&base.output_intents, intents);
    PdfDiff { output_intents: (!d.is_empty()).then_some(d), ..Default::default() }
}
macro_rules! tri_builders {
    ($($name:ident => $field:ident : $ty:ty),* $(,)?) => {
        $(
            // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
            pub fn $name(base: &PdfSnapshot, value: Option<$ty>) -> PdfDiff {
                PdfDiff { $field: tri(&base.$field, &value), ..Default::default() }
            }
        )*
    };
}
tri_builders!(
    diff_set_acro_form => acro_form: PdfAcroForm,
    diff_set_optional_content => optional_content: PdfOptionalContent,
    diff_set_page_layout => page_layout: PdfPageLayout,
    diff_set_page_mode => page_mode: PdfPageMode,
    diff_set_viewer_preferences => viewer_preferences: PdfViewerPreferences,
    diff_set_open_action => open_action: PdfOpenAction,
    diff_set_language => language: String,
    diff_set_mark_info => mark_info: PdfMarkInfo,
    diff_set_metadata => metadata: String,
    diff_set_document_id => document_id: [Vec<u8>; 2],
    diff_set_encryption => encryption: PdfEncryption,
);
/// 🔧️ Upserts `key` in the catalog's retained entries.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_catalog_entry(base: &PdfSnapshot, key: &str, value: PdfObject) -> PdfDiff {
    let leaf = match base.catalog_extra.iter().position(|e| e.key == key) {
        Some(pos) => match value_diff_between(&base.catalog_extra[pos].value, &value) {
            None => return PdfDiff::default(),
            Some(d) => PdfDictDiff { modified: vec![PdfDictModified { key: key.to_string(), diff: d }], ..Default::default() },
        },
        None => PdfDictDiff { added: vec![PdfDictAdded { index: base.catalog_extra.len(), key: key.to_string(), item: value }], ..Default::default() },
    };
    PdfDiff { catalog_extra: Some(leaf), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_catalog_entry(base: &PdfSnapshot, key: &str) -> PdfDiff {
    if !base.catalog_extra.iter().any(|e| e.key == key) {
        return PdfDiff::default();
    }
    PdfDiff { catalog_extra: Some(PdfDictDiff { removed: vec![key.to_string()], ..Default::default() }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_object(id: ObjRef, index: usize, value: PdfObject) -> PdfDiff {
    PdfDiff { objects: Some(PdfObjectsDiff { added: vec![PdfObjectAdded { index, id, value }], ..Default::default() }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_object(id: ObjRef) -> PdfDiff {
    PdfDiff { objects: Some(PdfObjectsDiff { removed: vec![id], ..Default::default() }), ..Default::default() }
}
/// 🔧️ Upserts object `id`'s value: `modified` against BASE if present, `added` (at the final Vec
/// position) otherwise.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_object_value(base: &PdfSnapshot, id: ObjRef, value: PdfObject) -> PdfDiff {
    match base.objects.iter().find(|o| o.id == id) {
        Some(existing) => match value_diff_between(&existing.value, &value) {
            None => PdfDiff::default(),
            Some(d) => PdfDiff { objects: Some(PdfObjectsDiff { modified: vec![PdfObjectModified { id, diff: d }], ..Default::default() }), ..Default::default() },
        },
        None => diff_insert_object(id, base.objects.len(), value),
    }
}
/// 🔧️ Upserts `key` at `path` inside object `id`'s value tree (`modified` if `key` already
/// exists at that container, `added` otherwise). Graceful empty diff if `id`/`path` don't
/// resolve to a real `Dict`/`Stream` container in `base`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_dict_entry(base: &PdfSnapshot, id: ObjRef, path: &[PdfPathSegment], key: &str, value: PdfObject) -> PdfDiff {
    let Some(obj) = base.objects.iter().find(|o| o.id == id) else { return PdfDiff::default() };
    let Some(container) = resolve_value(&obj.value, path) else { return PdfDiff::default() };
    let Some(entries) = dict_entries_of(container) else { return PdfDiff::default() };
    let is_root_stream = path.is_empty() && matches!(obj.value, PdfObject::Stream { .. });
    let leaf = match entries.iter().position(|e| e.key == key) {
        Some(pos) => match value_diff_between(&entries[pos].value, &value) {
            None => return PdfDiff::default(),
            Some(d) => PdfDictDiff { modified: vec![PdfDictModified { key: key.to_string(), diff: d }], ..Default::default() },
        },
        None => PdfDictDiff { added: vec![PdfDictAdded { index: entries.len(), key: key.to_string(), item: value }], ..Default::default() },
    };
    diff_at_object_path(id, path, is_root_stream, leaf)
}
/// 🔧️ Removes `key` at `path` inside object `id`'s value tree. Graceful empty diff if the key
/// isn't actually present in `base`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_dict_entry(base: &PdfSnapshot, id: ObjRef, path: &[PdfPathSegment], key: &str) -> PdfDiff {
    let Some(obj) = base.objects.iter().find(|o| o.id == id) else { return PdfDiff::default() };
    let Some(container) = resolve_value(&obj.value, path) else { return PdfDiff::default() };
    let Some(entries) = dict_entries_of(container) else { return PdfDiff::default() };
    if !entries.iter().any(|e| e.key == key) {
        return PdfDiff::default();
    }
    let is_root_stream = path.is_empty() && matches!(obj.value, PdfObject::Stream { .. });
    let leaf = PdfDictDiff { removed: vec![key.to_string()], ..Default::default() };
    diff_at_object_path(id, path, is_root_stream, leaf)
}
/// 🔧️ Upserts `key` in the top-level trailer dictionary.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_trailer_entry(base: &PdfSnapshot, key: &str, value: PdfObject) -> PdfDiff {
    let leaf = match base.trailer.iter().position(|e| e.key == key) {
        Some(pos) => match value_diff_between(&base.trailer[pos].value, &value) {
            None => return PdfDiff::default(),
            Some(d) => PdfDictDiff { modified: vec![PdfDictModified { key: key.to_string(), diff: d }], ..Default::default() },
        },
        None => PdfDictDiff { added: vec![PdfDictAdded { index: base.trailer.len(), key: key.to_string(), item: value }], ..Default::default() },
    };
    PdfDiff { trailer: Some(leaf), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_trailer_entry(base: &PdfSnapshot, key: &str) -> PdfDiff {
    if !base.trailer.iter().any(|e| e.key == key) {
        return PdfDiff::default();
    }
    PdfDiff { trailer: Some(PdfDictDiff { removed: vec![key.to_string()], ..Default::default() }), ..Default::default() }
}

/// 📐 Which optional page box a mutation addresses.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum PdfPageBox {
    Crop,
    Bleed,
    Trim,
    Art,
}
//#endregion 🔖️MutationDiffBuilders

//#region 🔖️DiffCodec
/// 🧾 One codec for every lane: the derive-owned value encoding. Text is the one-line
/// `value=<dsl value>` record (`📝️text/📖️.grammar.semio`), binary is the `OP_BINARY_FORMAT`
/// byte followed by the container-less pack record body of the same value
/// (`💾️binary/📡️.protocol.semio`). Both are deterministic and decode back to the identical
/// `PdfDiff`.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
struct PdfDiffRecord {
    value: dsl::DslValue,
}

impl protocol::DiffCodec for PdfDiff {
    fn print_diff(&self) -> String {
        let model = PdfDiffRecord { value: dsl::ToValue::to_value(self) };
        dsl::print(&model.__dsl_to_record(), &PdfDiffRecord::__dsl_spec(), dsl::JoinMode::Inline)
    }
    fn parse_diff(text: &str) -> Result<Self, store::TextError> {
        let record = dsl::parse(text, &PdfDiffRecord::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits { max_bytes: 64 * 1024 * 1024, ..dsl::Limits::default() }, mode: dsl::SourceMode::Inline })?;
        let model = PdfDiffRecord::__dsl_from_record(&record)?;
        <Self as dsl::FromValue>::from_value(model.value).map_err(|error| store::TextError::new(error.to_string(), dsl::TextSpan::at(1, 1)))
    }
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT];
        out.extend_from_slice(&store::pack_rt::encode_wire_value(&dsl::ToValue::to_value(self)));
        Ok(out)
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        match bytes.first() {
            Some(format) if *format == store::pack_rt::OP_BINARY_FORMAT => {}
            Some(format) => return Err(malformed("diff format", 0, format!("expected {}, got {format}", store::pack_rt::OP_BINARY_FORMAT))),
            None => return Err(malformed("diff format", 0, "empty diff".into())),
        }
        let value = store::pack_rt::decode_wire_value(&bytes[1..]).map_err(|error| malformed("diff body", 1, error.to_string()))?;
        <Self as dsl::FromValue>::from_value(value).map_err(|error| malformed("diff value", 1, error.to_string()))
    }
}
//#endregion 🔖️DiffCodec

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::standards::v1_7::subsets::base::schema::snapshot::ObjRef;
pub use crate::standards::v1_7::subsets::base::schema::snapshot::PdfPage;
pub use crate::standards::v1_7::subsets::base::schema::snapshot::PdfStreamFilter;
//#endregion 🔁️Re-exports
