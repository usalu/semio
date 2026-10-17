//! 🔀️ The tri-state patch value and the index-keyed and id-keyed triple diffs every collection lane rides on.

use super::*;

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
pub(super) fn tri<T: Clone + PartialEq>(a: &Option<T>, b: &Option<T>) -> Option<PdfSet<T>> {
    (a != b).then(|| PdfSet::from_option(b))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn apply_tri<T: Clone>(slot: &mut Option<T>, diff: &Option<PdfSet<T>>) {
    if let Some(set) = diff {
        *slot = set.clone().into_option();
    }
}
//#endregion 🔖️TriState

//#region 🔖️IndexedTriple
/// 📦️ An index-keyed triple of whole values (positional collections: content operators,
/// annotations, outlines, named destinations, page labels, output intents).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfIndexedItem<T> {
    pub index: usize,
    pub value: T,
}

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
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
        let kept = base
            .iter()
            .cloned()
            .enumerate()
            .filter(|(index, _)| !self.removed.contains(index))
            .map(|(index, slot)| match self.modified.iter().rev().find(|item| item.index == index) {
                Some(item) => match slot {
                    Slot::Base(origin) | Slot::Modified(origin, _) => Slot::Modified(origin, item.value.clone()),
                    Slot::Added(_) => Slot::Added(item.value.clone()),
                },
                None => slot,
            })
            .collect::<Vec<_>>();
        let mut order: Vec<&PdfIndexedItem<T>> = self.added.iter().collect();
        order.sort_by_key(|item| item.index);
        order.into_iter().fold(kept, |mut layout, item| {
            layout.insert(item.index.min(layout.len()), Slot::Added(item.value.clone()));
            layout
        })
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
pub(super) enum Slot<T> {
    Base(usize),
    Modified(usize, T),
    Added(T),
}

/// 🧱️ Where one position of a triple's FINAL state came from: a surviving BASE index or an `added` row.
#[derive(Clone, Copy)]
pub(super) enum Origin {
    Base(usize),
    Added,
}

/// 🧭️ The final layout of an index triple over `base_len` base items, in `apply` order: removals descending, additions
/// ascending at their final index.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn triple_layout(base_len: usize, removed: &[usize], added: &[usize]) -> Vec<Origin> {
    let mut layout: Vec<Origin> = (0..base_len).map(Origin::Base).collect();
    let distinct: std::collections::BTreeSet<usize> = removed.iter().copied().collect();
    for index in distinct.into_iter().rev() {
        if index < layout.len() {
            layout.remove(index);
        }
    }
    let mut order: Vec<usize> = (0..added.len()).collect();
    order.sort_by_key(|tag| added[*tag]);
    for tag in order {
        layout.insert(added[tag].min(layout.len()), Origin::Added);
    }
    layout
}

impl<T: Clone + PartialEq> PdfIndexedDiff<T> {
    /// ↩️ The negative triple over `base`: it removes what this one added, restores every modified item to its base value at
    /// the position the item holds in the final state, and re-adds every removed base item at its base index.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn inverse(&self, base: &[T]) -> Self {
        let added: Vec<usize> = self.added.iter().map(|item| item.index).collect();
        let mut inverse = Self::default();
        for (position, origin) in triple_layout(base.len(), &self.removed, &added).into_iter().enumerate() {
            match origin {
                Origin::Added => inverse.removed.push(position),
                Origin::Base(index) if self.modified.iter().any(|item| item.index == index) => {
                    if let Some(previous) = base.get(index) {
                        inverse.modified.push(PdfIndexedItem { index: position, value: previous.clone() });
                    }
                }
                Origin::Base(_) => {}
            }
        }
        let mut removed = self.removed.clone();
        removed.sort_unstable();
        removed.dedup();
        for index in removed {
            if let Some(previous) = base.get(index) {
                inverse.added.push(PdfIndexedItem { index, value: previous.clone() });
            }
        }
        inverse
    }
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

    /// ↩️ The negative triple over `base`: it removes what this one added, restores every modified item to its base value and
    /// re-adds every removed base item at its base index.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn inverse(&self, base: &[T]) -> Self {
        let mut inverse = Self::default();
        inverse.removed = self.added.iter().map(|item| item.value.key().to_string()).collect();
        inverse.modified = self.modified.iter().filter_map(|item| base.iter().find(|candidate| candidate.key() == item.key).map(|previous| PdfKeyedItem { key: item.key.clone(), value: previous.clone() })).collect();
        inverse.added = self.removed.iter().filter_map(|key| base.iter().position(|candidate| candidate.key() == key).map(|index| PdfIndexedItem { index, value: base[index].clone() })).collect();
        inverse.added.sort_by_key(|item| item.index);
        inverse
    }
}
//#endregion 🔖️KeyedTriple
