//! 🌳️ The recursive COS value patches: dictionaries, arrays, values and the object triple.

use super::*;

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
    pub(super) fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn apply_dict_diff(diff: &PdfDictDiff, base: &[PdfDictEntry]) -> Vec<PdfDictEntry> {
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

/// ➕️ Name-keyed absorb (key identity, non-positional -- mirrors json's `absorb_object_diff`):
/// a `d2`-removal of a `d1`-added key annihilates the add; a `d2`-modify of a `d1`-added key
/// patches the carried payload; a `d2`-modify-of-`d1`-removed key is dropped (illegal, matches
/// `apply`'s no-op rule).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn absorb_dict_diff(d1: PdfDictDiff, d2: PdfDictDiff) -> PdfDictDiff {
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
/// ↩️ The negative dictionary patch over `base`: it removes the keys `diff` added, patches every modified entry back through
/// its negative value diff and re-adds every removed entry at its base position.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn inverse_dict_diff(diff: &PdfDictDiff, base: &[PdfDictEntry]) -> PdfDictDiff {
    PdfDictDiff {
        removed: diff.added.iter().map(|item| item.key.clone()).collect(),
        modified: diff.modified.iter().filter_map(|item| base.iter().find(|entry| entry.key == item.key).map(|entry| PdfDictModified { key: item.key.clone(), diff: inverse_value_diff(&item.diff, &entry.value) })).collect(),
        added: {
            let mut added: Vec<PdfDictAdded> = diff.removed.iter().filter_map(|key| base.iter().position(|entry| &entry.key == key).map(|index| PdfDictAdded { index, key: key.clone(), item: base[index].value.clone() })).collect();
            added.sort_by_key(|item| item.index);
            added
        },
    }
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
    pub(super) fn is_empty(&self) -> bool {
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
/// ↩️ The negative array patch over `base`: it removes what `diff` added, patches every modified item back through its
/// negative value diff at the position the item holds in the final state, and re-adds every removed item at its base index.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_array_diff(diff: &PdfArrayDiff, base: &[PdfObject]) -> PdfArrayDiff {
    let added: Vec<usize> = diff.added.iter().map(|item| item.index).collect();
    let mut inverse = PdfArrayDiff::default();
    for (position, origin) in triple_layout(base.len(), &diff.removed, &added).into_iter().enumerate() {
        match origin {
            Origin::Added => inverse.removed.push(position),
            Origin::Base(index) => {
                if let (Some(modified), Some(item)) = (diff.modified.iter().find(|item| item.index == index), base.get(index)) {
                    inverse.modified.push(PdfArrayModified { index: position, diff: inverse_value_diff(&modified.diff, item) });
                }
            }
        }
    }
    let mut removed = diff.removed.clone();
    removed.sort_unstable();
    removed.dedup();
    for index in removed {
        if let Some(item) = base.get(index) {
            inverse.added.push(PdfArrayAdded { index, item: item.clone() });
        }
    }
    inverse
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
/// ↩️ The negative node diff over `base`: the node's old scalar or kind-changing replacement, or the negative collection patch
/// of a structural diff.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_value_diff(diff: &PdfValueDiff, base: &PdfObject) -> PdfValueDiff {
    match (diff, base) {
        (PdfValueDiff::Array { diff }, PdfObject::Array(items)) => PdfValueDiff::Array { diff: inverse_array_diff(diff, items) },
        (PdfValueDiff::Dict { diff }, PdfObject::Dict(entries)) => PdfValueDiff::Dict { diff: inverse_dict_diff(diff, entries) },
        (PdfValueDiff::Stream { dict, data, filters }, PdfObject::Stream { dict: base_dict, data: base_data, filters: base_filters }) => PdfValueDiff::Stream { dict: dict.as_ref().map(|dict| inverse_dict_diff(dict, base_dict)), data: data.as_ref().map(|_| base_data.clone()), filters: filters.as_ref().map(|_| base_filters.clone()) },
        (PdfValueDiff::Replace { .. }, _) => PdfValueDiff::Replace { value: base.clone() },
        (PdfValueDiff::Bool { .. }, PdfObject::Bool(value)) => PdfValueDiff::Bool { value: *value },
        (PdfValueDiff::Int { .. }, PdfObject::Int(value)) => PdfValueDiff::Int { value: *value },
        (PdfValueDiff::Real { .. }, PdfObject::Real(value)) => PdfValueDiff::Real { value: value.clone() },
        (PdfValueDiff::Str { .. }, PdfObject::Str(value)) => PdfValueDiff::Str { value: value.clone() },
        (PdfValueDiff::Name { .. }, PdfObject::Name(value)) => PdfValueDiff::Name { value: value.clone() },
        (PdfValueDiff::Ref { .. }, PdfObject::Ref(value)) => PdfValueDiff::Ref { value: *value },
        _ => PdfValueDiff::Replace { value: base.clone() },
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
    pub(super) fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

use crate::standards::v1_7::subsets::base::schema::snapshot::PdfIndirectObject;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn apply_objects_diff(diff: &PdfObjectsDiff, base: &[PdfIndirectObject]) -> Vec<PdfIndirectObject> {
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

/// ➕️ Id-keyed absorb (key identity, non-positional -- mirrors json's `absorb_object_diff` and
/// this file's own `absorb_dict_diff`, keyed by `ObjRef` instead of `String`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn absorb_law_objects_diff(d1: PdfObjectsDiff, d2: PdfObjectsDiff) -> PdfObjectsDiff {
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
    removed.sort_by_key(|id| (id.num, id.gen));
    removed.dedup();
    modified.sort_by_key(|item| (item.id.num, item.id.gen));
    PdfObjectsDiff { removed, modified, added }
}
/// ↩️ The negative objects triple over `base`: it removes the objects `diff` added, patches every modified object back through
/// its negative value diff and re-adds every removed object at its base index.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn inverse_objects_diff(diff: &PdfObjectsDiff, base: &[PdfIndirectObject]) -> PdfObjectsDiff {
    let mut removed: Vec<ObjRef> = diff.added.iter().map(|item| item.id).collect();
    removed.sort_by_key(|id| (id.num, id.gen));
    let mut modified: Vec<PdfObjectModified> = diff.modified.iter().filter_map(|item| base.iter().find(|object| object.id == item.id).map(|object| PdfObjectModified { id: item.id, diff: inverse_value_diff(&item.diff, &object.value) })).collect();
    modified.sort_by_key(|item| (item.id.num, item.id.gen));
    let mut added: Vec<PdfObjectAdded> = diff.removed.iter().filter_map(|id| base.iter().position(|object| &object.id == id).map(|index| PdfObjectAdded { index, id: *id, value: base[index].value.clone() })).collect();
    added.sort_by_key(|item| item.index);
    PdfObjectsDiff { removed, modified, added }
}
//#endregion 🔖️ObjectsTriple
