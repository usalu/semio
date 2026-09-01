//! 🔺️ PdfDiff (1.7) — handcrafted sparse diff over the real object-graph model. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION D2 `📄️pdf` row.
//! Replaces the old op-slot `{snapshot, insert_page, remove_page_at, ...}` template (one field
//! per mutation, `snapshot: Option<PdfSnapshot>` full-replace fast path — banned by the recipe)
//! with a real per-field patch: `pages` is an index-keyed triple of flat `PdfPageDiff` patches,
//! `objects` is an `ObjRef`-keyed (the `(id,gen)` pair) triple of recursive `PdfValueDiff`
//! patches mirroring `PdfObject`'s own shape (mirrors json's `JsonValueDiff` pattern: `Replace`
//! on node-KIND change, direct field/collection diff when the kind is stable — `Array` gets an
//! index-keyed triple, `Dict`/`Stream.dict` get a name-keyed triple, stream data/filter concepts
//! are whole-value tri-state), and `trailer` reuses that SAME name-keyed `PdfDictDiff` triple
//! shape verbatim (the recipe's own guidance: "trailer is itself a Dict-shaped structure").
//!
//! Naming deviation from the ticket brief (documented per its own "your call, document it"):
//! the brief sketches an enum named `PdfValue`; this codebase's already-real, already-tested
//! object model (`⚙️engine`, `📸️snapshot`) calls it `PdfObject` with `Int(i64)` and exact
//! decimal `Real(PdfDecimal)` values kept separate so the writer retains numeric semantics;
//! the diff mirrors
//! `PdfObject`'s real shape field-for-field instead of inventing a parallel vocabulary.

use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfDecimal, PdfDictEntry, PdfInfo, PdfObject, PdfPage, PdfPredictor, PdfSnapshot, PdfStreamFilter};
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use schema::ArtifactSchema;
use std::collections::{HashMap, HashSet};

//#region 🔖️PageDiff
/// 📄️ Sparse per-field patch for one `PdfPage` (a WEAK entity per the recipe -- a value struct,
/// never sub-diffed beyond its own flat fields). `crop_box` is tri-state: `None` = unchanged,
/// `Some(None)` = cleared, `Some(Some(b))` = set.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfPageDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub media_box: Option<[f64; 4]>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub crop_box: Option<Option<[f64; 4]>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rotate: Option<i32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_page_diff(page: &mut PdfPage, diff: &PdfPageDiff) {
    if let Some(v) = diff.media_box {
        page.media_box = v;
    }
    if let Some(v) = &diff.crop_box {
        page.crop_box = *v;
    }
    if let Some(v) = diff.rotate {
        page.rotate = v;
    }
    if let Some(v) = &diff.text {
        page.text = v.clone();
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn page_diff_between(a: &PdfPage, b: &PdfPage) -> PdfPageDiff {
    PdfPageDiff { media_box: (a.media_box != b.media_box).then_some(b.media_box), crop_box: (a.crop_box != b.crop_box).then_some(b.crop_box), rotate: (a.rotate != b.rotate).then_some(b.rotate), text: (a.text != b.text).then(|| b.text.clone()) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_page_diff_empty(d: &PdfPageDiff) -> bool {
    d == &PdfPageDiff::default()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_page_diff(base: &mut PdfPageDiff, other: PdfPageDiff) {
    if other.media_box.is_some() {
        base.media_box = other.media_box;
    }
    if other.crop_box.is_some() {
        base.crop_box = other.crop_box;
    }
    if other.rotate.is_some() {
        base.rotate = other.rotate;
    }
    if other.text.is_some() {
        base.text = other.text;
    }
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
    let mut pages: Vec<PdfPage> = base.to_vec();
    for m in &diff.modified {
        if let Some(p) = pages.get_mut(m.index) {
            apply_page_diff(p, &m.diff);
        }
    }
    let mut removed_sorted = diff.removed.clone();
    removed_sorted.sort_unstable();
    removed_sorted.dedup();
    for idx in removed_sorted.into_iter().rev() {
        if idx < pages.len() {
            pages.remove(idx);
        }
    }
    let mut added_sorted: Vec<&PdfPageAdded> = diff.added.iter().collect();
    added_sorted.sort_by_key(|a| a.index);
    for a in added_sorted {
        pages.insert(a.index, a.page.clone());
    }
    pages
}

/// 🧭️ `between` matching for index-keyed collections (recipe): pairwise `0..min(len)` as
/// `modified`, base tail as `removed`, other tail as `added`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pages_diff_between(a: &[PdfPage], b: &[PdfPage]) -> PdfPagesDiff {
    let min = a.len().min(b.len());
    let mut modified = Vec::new();
    for i in 0..min {
        let d = page_diff_between(&a[i], &b[i]);
        if !is_page_diff_empty(&d) {
            modified.push(PdfPageModified { index: i, diff: d });
        }
    }
    let removed: Vec<usize> = if a.len() > b.len() { (b.len()..a.len()).collect() } else { Vec::new() };
    let added: Vec<PdfPageAdded> = if b.len() > a.len() { (a.len()..b.len()).map(|i| PdfPageAdded { index: i, page: b[i].clone() }).collect() } else { Vec::new() };
    PdfPagesDiff { removed, modified, added }
}

/// ➕️ Index-transported absorb via symbolic position simulation (recipe canonical cases:
/// `Insert+Remove-before`, `Insert+Insert` same index both survive, `Add+SetField` patches into
/// the carried added payload) -- same algorithm shape as json's `absorb_array_diff`, specialized
/// to flat `PdfPageDiff` (no recursion needed, pages are weak/flat entities).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_law_pages_diff(d1: PdfPagesDiff, d2: PdfPagesDiff) -> PdfPagesDiff {
    enum Origin {
        Base(usize),
        D1Added(usize),
    }
    enum AfterSlot {
        Base { orig: usize, diff: Option<PdfPageDiff> },
        D1Added { tag: usize, patch: Option<PdfPageDiff> },
        D2Added(PdfPage),
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
    let d1_modified: HashMap<usize, PdfPageDiff> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();

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
                        Some(mut existing) => {
                            absorb_page_diff(&mut existing, m.diff.clone());
                            existing
                        }
                        None => m.diff.clone(),
                    };
                    *diff = if is_page_diff_empty(&combined) { None } else { Some(combined) };
                }
                AfterSlot::D1Added { patch, .. } => {
                    let combined = match patch.take() {
                        Some(mut existing) => {
                            absorb_page_diff(&mut existing, m.diff.clone());
                            existing
                        }
                        None => m.diff.clone(),
                    };
                    *patch = if is_page_diff_empty(&combined) { None } else { Some(combined) };
                }
                AfterSlot::D2Added(_) => {}
            }
        }
    }
    let mut d2_added_order: Vec<usize> = (0..d2.added.len()).collect();
    d2_added_order.sort_by_key(|&tag| d2.added[tag].index);
    for tag in d2_added_order {
        let pos = d2.added[tag].index.min(after.len());
        after.insert(pos, AfterSlot::D2Added(d2.added[tag].page.clone()));
    }

    let mut modified = Vec::new();
    let mut added = Vec::new();
    for (pos, slot) in after.into_iter().enumerate() {
        match slot {
            AfterSlot::Base { orig, diff: Some(diff) } => modified.push(PdfPageModified { index: orig, diff }),
            AfterSlot::Base { .. } => {}
            AfterSlot::D1Added { tag, patch } => {
                let mut page = d1.added[tag].page.clone();
                if let Some(patch) = patch {
                    apply_page_diff(&mut page, &patch);
                }
                added.push(PdfPageAdded { index: pos, page });
            }
            AfterSlot::D2Added(page) => added.push(PdfPageAdded { index: pos, page }),
        }
    }
    final_removed.sort_unstable();
    final_removed.dedup();
    PdfPagesDiff { removed: final_removed, modified, added }
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
fn absorb_array_diff(d1: PdfArrayDiff, d2: PdfArrayDiff) -> PdfArrayDiff {
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
        (PdfValueDiff::Array { diff: a1 }, PdfValueDiff::Array { diff: a2 }) => PdfValueDiff::Array { diff: absorb_array_diff(a1, a2) },
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

use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::snapshot::PdfIndirectObject;

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

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.pdf.1.7`. `schema` is an identity field and is never diffed. `info` is a
/// WEAK value struct (recipe: whole-value replaced, never sub-diffed).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pdf.1.7.diff")]
pub struct PdfDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub declared_version: Option<String>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<PdfInfo>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub pages: Option<PdfPagesDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub objects: Option<PdfObjectsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub trailer: Option<PdfDictDiff>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
fn validate_pages_diff(diff: &PdfPagesDiff, base: &[PdfPage]) -> MutationApplyResult<()> {
    validate_index_triple(base.len(), &diff.removed, &diff.modified.iter().map(|item| item.index).collect::<Vec<_>>(), &diff.added.iter().map(|item| item.index).collect::<Vec<_>>(), "pages")
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
fn validate_pdf_diff(diff: &PdfDiff, base: &PdfSnapshot) -> MutationApplyResult<()> {
    if let Some(pages) = &diff.pages {
        validate_pages_diff(pages, &base.pages).map_err(|error| error.under(["pages"]))?;
    }
    if let Some(objects) = &diff.objects {
        validate_objects_diff(objects, &base.objects).map_err(|error| error.under(["objects"]))?;
    }
    if let Some(trailer) = &diff.trailer {
        validate_dict_diff(trailer, &base.trailer).map_err(|error| error.under(["trailer"]))?;
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
    if let Some(od) = &diff.objects {
        next.objects = apply_objects_diff(od, &base.objects);
    }
    if let Some(td) = &diff.trailer {
        next.trailer = apply_dict_diff(td, &base.trailer);
    }
    next
}

impl MutationDiff<PdfSnapshot> for PdfDiff {
    fn apply(&self, base: &PdfSnapshot) -> MutationApplyResult<PdfSnapshot> {
        validate_pdf_diff(self, base)?;
        Ok(apply_pdf_diff_unchecked(self, base))
    }

    /// ➕️ Structural, total, base-free sequential-coalesce absorb (`## Absorb` contract).
    /// Scalars: LWW. `pages`/`objects`/`trailer`: composed via their own key/index-transported
    /// absorb helpers above.
    fn absorb(&mut self, other: Self) {
        if other.declared_version.is_some() {
            self.declared_version = other.declared_version;
        }
        if other.info.is_some() {
            self.info = other.info;
        }
        self.pages = match (self.pages.take(), other.pages) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => {
                let m = absorb_law_pages_diff(a, b);
                if m.is_empty() {
                    None
                } else {
                    Some(m)
                }
            }
        };
        self.objects = match (self.objects.take(), other.objects) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => {
                let m = absorb_law_objects_diff(a, b);
                if m.is_empty() {
                    None
                } else {
                    Some(m)
                }
            }
        };
        self.trailer = match (self.trailer.take(), other.trailer) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => {
                let m = absorb_dict_diff(a, b);
                if m.is_empty() {
                    None
                } else {
                    Some(m)
                }
            }
        };
    }
}

impl DiffAlgebra<PdfSnapshot> for PdfDiff {
    /// 🔁️ Diff-level undo, derived generically from `between` (correct by construction): the
    /// state delta from `self.apply(base)` back to `base`.
    fn inverse(&self, base: &PdfSnapshot) -> Self {
        let mid = apply_pdf_diff_unchecked(self, base);
        Self::between(&mid, base)
    }

    /// 🧭️ State delta (compose `GetXDiff`): `pages` positionally matched (index key), `objects`
    /// and `trailer` matched by their real keys (`ObjRef`/dict key name).
    fn between(base: &PdfSnapshot, other: &PdfSnapshot) -> Self {
        let declared_version = (base.declared_version != other.declared_version).then(|| other.declared_version.clone());
        let info = (base.info != other.info).then(|| other.info.clone());
        let pages = {
            let d = pages_diff_between(&base.pages, &other.pages);
            if d.is_empty() {
                None
            } else {
                Some(d)
            }
        };
        let objects = {
            let d = objects_diff_between(&base.objects, &other.objects);
            if d.is_empty() {
                None
            } else {
                Some(d)
            }
        };
        let trailer = {
            let d = dict_diff_between(&base.trailer, &other.trailer);
            if d.is_empty() {
                None
            } else {
                Some(d)
            }
        };
        PdfDiff { declared_version, info, pages, objects, trailer }
    }

    fn is_empty(&self) -> bool {
        self.declared_version.is_none() && self.info.is_none() && self.pages.is_none() && self.objects.is_none() && self.trailer.is_none()
    }
}

//#endregion 🔖️Diff

//#region 🔖️MutationDiffBuilders
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_page(index: usize, page: PdfPage) -> PdfDiff {
    PdfDiff { pages: Some(PdfPagesDiff { added: vec![PdfPageAdded { index, page }], ..Default::default() }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_page(index: usize) -> PdfDiff {
    PdfDiff { pages: Some(PdfPagesDiff { removed: vec![index], ..Default::default() }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_page_media_box(index: usize, media_box: [f64; 4]) -> PdfDiff {
    PdfDiff { pages: Some(PdfPagesDiff { modified: vec![PdfPageModified { index, diff: PdfPageDiff { media_box: Some(media_box), ..Default::default() } }], ..Default::default() }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_page_crop_box(index: usize, crop_box: Option<[f64; 4]>) -> PdfDiff {
    PdfDiff { pages: Some(PdfPagesDiff { modified: vec![PdfPageModified { index, diff: PdfPageDiff { crop_box: Some(crop_box), ..Default::default() } }], ..Default::default() }), ..Default::default() }
}
/// ➕️ Appends `text` to page `index`'s authoring text (newline-separated), computed from `base`
/// directly (handcrafted, not apply-and-capture).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_append_page_content(base: &PdfSnapshot, index: usize, text: &str) -> PdfDiff {
    let new_text = match base.pages.get(index) {
        Some(p) if !p.text.is_empty() => format!("{}\n{}", p.text, text),
        _ => text.to_string(),
    };
    PdfDiff { pages: Some(PdfPagesDiff { modified: vec![PdfPageModified { index, diff: PdfPageDiff { text: Some(new_text), ..Default::default() } }], ..Default::default() }), ..Default::default() }
}
/// ✏️️ Replaces page `index`'s authoring text outright (unlike `AppendPageContent`, no read of the
/// prior text is needed to build the FORWARD diff -- only its `inverse` reads `base`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_page_content(index: usize, text: &str) -> PdfDiff {
    PdfDiff { pages: Some(PdfPagesDiff { modified: vec![PdfPageModified { index, diff: PdfPageDiff { text: Some(text.to_string()), ..Default::default() } }], ..Default::default() }), ..Default::default() }
}
/// 🔄️ Sets page `index`'s resolved `/Rotate` value by reusing the sparse
/// `PdfPageDiff::rotate` field; no new diff field is needed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_page_rotation(index: usize, rotation: i32) -> PdfDiff {
    PdfDiff { pages: Some(PdfPagesDiff { modified: vec![PdfPageModified { index, diff: PdfPageDiff { rotate: Some(rotation), ..Default::default() } }], ..Default::default() }), ..Default::default() }
}
/// 🔀️ Moves the page at BASE-state index `from` to FINAL-state index `to` -- `removed`/`added`
/// compose the move (no dedicated "moved" slot on `PdfPagesDiff`), same shape as
/// `PptxMutation::MoveSlide`'s own `diff_move_slide` over `PptxSlidesDiff`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_move_page(base: &PdfSnapshot, from: usize, to: usize) -> PdfDiff {
    let Some(page) = base.pages.get(from) else { return PdfDiff::default() };
    // 🧭️ `to` is clamped to the post-removal length (same guard `PptxMutation::MoveSlide`'s own
    // `diff_move_slide` needs -- `apply_pages_diff`'s `Vec::insert` panics past that bound).
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
/// isn't actually present in `base` (matches `apply`'s no-op-on-missing-key rule).
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
/// 🔧️ Upserts `key` in the top-level trailer dictionary (name-keyed, reuses `PdfDictDiff`
/// directly -- no object/path addressing needed at this level).
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
//#endregion 🔖️MutationDiffBuilders

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: `#[derive(dsl::DslDiff)]` on `PdfDiff` was tried for real and confirmed rejected —
/// `cargo check -p semio-s-plugin-stdio --lib` fails with `the trait bound
/// v1_7::...::PdfObject: DslField is not satisfied` (blocker 3a: `PdfObject` is a genuine
/// data-carrying enum reachable via `PdfValueDiff::Replace`/`Array`/`Dict` items and
/// `PdfDictAdded`/`PdfObjectAdded::value`/`item`) — matching `f6-recon-report.md` §3a/§8's row 25
/// prediction (2 enums: `PdfObject`, `PdfValueDiff`). Typed stream filters on
/// `PdfValueDiff::Stream` is ALSO an independent blocker (3b). `DiffCodec` is hand-rolled below,
/// following svg's real template (`SvgDiff`'s own `HandcraftedDiffCodec` region) — same primitive
/// set (bracket-depth-aware `split_top_level`, hex for strings/bytes, `[0]`/`[1,x]` for
/// `Option<T>`), re-derived locally per the "no shared hand-roll helpers module yet" note.
//#region 🔖️Primitives
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

/// 🧭️ Bracket-depth-aware split (tracks `[`/`]` only): a top-level `sep` inside nested brackets is
/// never mistaken for a field separator — the whole hand-rolled grammar's parsing primitive.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_box(b: &[f64; 4]) -> String {
    format!("[{},{},{},{}]", b[0], b[1], b[2], b[3])
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_box(s: &str) -> Result<[f64; 4], String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [a, b, c, d] = parts.as_slice() else { return Err(format!("box: expected 4 fields, got {}", parts.len())) };
    let f = |s: &str| s.parse::<f64>().map_err(|e: std::num::ParseFloatError| e.to_string());
    Ok([f(a)?, f(b)?, f(c)?, f(d)?])
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_stream_filters(filters: &[PdfStreamFilter]) -> String {
    let values = filters
        .iter()
        .map(|filter| match filter {
            PdfStreamFilter::Flate { predictor: None } => "F[0]".to_string(),
            PdfStreamFilter::Flate { predictor: Some(predictor) } => format!("F[1,{},{},{},{}]", predictor.predictor, predictor.colors, predictor.bits_per_component, predictor.columns,),
            PdfStreamFilter::AsciiHex => "H".to_string(),
            PdfStreamFilter::Ascii85 => "A".to_string(),
            PdfStreamFilter::RunLength => "L".to_string(),
        })
        .collect::<Vec<_>>()
        .join(",");
    format!("[{values}]")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_stream_filters(s: &str) -> Result<Vec<PdfStreamFilter>, String> {
    split_top_level(strip_brackets(s)?, ',')
        .into_iter()
        .filter(|value| !value.is_empty())
        .map(|value| match value {
            "H" => Ok(PdfStreamFilter::AsciiHex),
            "A" => Ok(PdfStreamFilter::Ascii85),
            "L" => Ok(PdfStreamFilter::RunLength),
            _ if value.starts_with("F[") => {
                let fields = split_top_level(strip_brackets(&value[1..])?, ',');
                match fields.as_slice() {
                    ["0"] => Ok(PdfStreamFilter::Flate { predictor: None }),
                    ["1", predictor, colors, bits_per_component, columns] => Ok(PdfStreamFilter::Flate {
                        predictor: Some(PdfPredictor {
                            predictor: predictor.parse().map_err(|error: std::num::ParseIntError| error.to_string())?,
                            colors: colors.parse().map_err(|error: std::num::ParseIntError| error.to_string())?,
                            bits_per_component: bits_per_component.parse().map_err(|error: std::num::ParseIntError| error.to_string())?,
                            columns: columns.parse().map_err(|error: std::num::ParseIntError| error.to_string())?,
                        }),
                    }),
                    _ => Err(format!("flate filter: invalid payload {value:?}")),
                }
            }
            _ => Err(format!("stream filter: unknown tag {value:?}")),
        })
        .collect()
}
//#endregion 🔖️Primitives

//#region 🔖️ObjectValueCodecs
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_objref(r: &ObjRef) -> String {
    format!("[{},{}]", r.num, r.gen)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_objref(s: &str) -> Result<ObjRef, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [num, gen] = parts.as_slice() else { return Err(format!("objref: expected 2 fields, got {}", parts.len())) };
    Ok(ObjRef { num: num.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, gen: gen.parse().map_err(|e: std::num::ParseIntError| e.to_string())? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_dict_entry(e: &PdfDictEntry) -> String {
    format!("[{},{}]", enc_str(&e.key), enc_pdf_object(&e.value))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_dict_entry(s: &str) -> Result<PdfDictEntry, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [key, value] = parts.as_slice() else { return Err(format!("dict entry: expected 2 fields, got {}", parts.len())) };
    Ok(PdfDictEntry { key: dec_str(key)?, value: dec_pdf_object(value)? })
}
/// 🌳 Recursive: `Z`=Null (bare, no payload) / `B[0|1]`=Bool / `I[n]`=Int / `R[n]`=Real /
/// `S[hex]`=Str / `N[hex]`=Name / `A[items]`=Array / `D[entries]`=Dict / `F[num,gen]`=Ref /
/// `T[[entries],hexdata]`=Stream — single-uppercase-letter tag prefix, never ambiguous
/// with the hex payload (hex never starts with an uppercase letter) or with `Z`'s bare form
/// (every other tag is immediately followed by `[`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_pdf_object(v: &PdfObject) -> String {
    match v {
        PdfObject::Null => "Z".to_string(),
        PdfObject::Bool(b) => format!("B[{}]", if *b { "1" } else { "0" }),
        PdfObject::Int(i) => format!("I[{i}]"),
        PdfObject::Real(f) => format!("R[{f}]"),
        PdfObject::Str(bytes) => format!("S[{}]", hex_encode(bytes)),
        PdfObject::Name(s) => format!("N[{}]", enc_str(s)),
        PdfObject::Array(items) => format!("A[{}]", items.iter().map(enc_pdf_object).collect::<Vec<_>>().join(",")),
        PdfObject::Dict(entries) => format!("D[{}]", entries.iter().map(enc_dict_entry).collect::<Vec<_>>().join(",")),
        PdfObject::Ref(r) => format!("F[{}]", enc_objref(r)),
        PdfObject::Stream { dict, data, filters } => format!("T[[{}],{},{}]", dict.iter().map(enc_dict_entry).collect::<Vec<_>>().join(","), hex_encode(data), enc_stream_filters(filters),),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_pdf_object(s: &str) -> Result<PdfObject, String> {
    if s == "Z" {
        return Ok(PdfObject::Null);
    }
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "B" => Ok(PdfObject::Bool(inner == "1")),
        "I" => Ok(PdfObject::Int(inner.parse().map_err(|e: std::num::ParseIntError| e.to_string())?)),
        "R" => Ok(PdfObject::Real(PdfDecimal::parse(inner)?)),
        "S" => Ok(PdfObject::Str(hex_decode(inner)?)),
        "N" => Ok(PdfObject::Name(dec_str(inner)?)),
        "A" => Ok(PdfObject::Array(split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_pdf_object).collect::<Result<Vec<_>, String>>()?)),
        "D" => Ok(PdfObject::Dict(split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_dict_entry).collect::<Result<Vec<_>, String>>()?)),
        "F" => Ok(PdfObject::Ref(dec_objref(inner)?)),
        "T" => {
            let parts = split_top_level(inner, ',');
            let [dict_s, data_s, filters_s] = parts.as_slice() else { return Err(format!("stream: expected 3 fields, got {}", parts.len())) };
            let dict = split_top_level(strip_brackets(dict_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_dict_entry).collect::<Result<Vec<_>, String>>()?;
            let filters = dec_stream_filters(filters_s)?;
            Ok(PdfObject::Stream { dict, data: hex_decode(data_s)?, filters })
        }
        other => Err(format!("pdf object: unknown tag {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_pdf_page(p: &PdfPage) -> String {
    format!("[{},{},{},{}]", enc_box(&p.media_box), encode_option(&p.crop_box, enc_box), p.rotate, enc_str(&p.text))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_pdf_page(s: &str) -> Result<PdfPage, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [media_box, crop_box, rotate, text] = parts.as_slice() else { return Err(format!("page: expected 4 fields, got {}", parts.len())) };
    Ok(PdfPage { media_box: dec_box(media_box)?, crop_box: decode_option(crop_box, dec_box)?, rotate: rotate.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, text: dec_str(text)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_pdf_info(i: &PdfInfo) -> String {
    format!(
        "[{},{},{},{},{},{}]",
        encode_option(&i.title, |v| enc_str(v)),
        encode_option(&i.author, |v| enc_str(v)),
        encode_option(&i.subject, |v| enc_str(v)),
        encode_option(&i.keywords, |v| enc_str(v)),
        encode_option(&i.creator, |v| enc_str(v)),
        encode_option(&i.producer, |v| enc_str(v)),
    )
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_pdf_info(s: &str) -> Result<PdfInfo, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [title, author, subject, keywords, creator, producer] = parts.as_slice() else { return Err(format!("info: expected 6 fields, got {}", parts.len())) };
    Ok(PdfInfo {
        title: decode_option(title, dec_str)?,
        author: decode_option(author, dec_str)?,
        subject: decode_option(subject, dec_str)?,
        keywords: decode_option(keywords, dec_str)?,
        creator: decode_option(creator, dec_str)?,
        producer: decode_option(producer, dec_str)?,
    })
}
//#endregion 🔖️ObjectValueCodecs

//#region 🔖️BinaryPrimitives
/// 🧪️ P2-FG3: real LEB128-varint-framed binary primitives (length-prefixed bytes/utf8, raw
/// little-endian f64) backing the upgraded `OpBinary` (`../🧬️mutations/🦀️component.rs`, which
/// `pub(crate)`-reuses everything in this region and the recursive codecs below) and `DiffCodec`
/// frames -- reuses `store::pack_rt::write_varint_u64`/`store::ByteReader` rather than
/// reinventing varint encode/decode, same shape xml's own `write_str_lp`/`read_str_lp` uses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_f64_bin(out: &mut Vec<u8>, v: f64) {
    out.extend_from_slice(&v.to_le_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_f64_bin(reader: &mut store::ByteReader<'_>) -> Result<f64, String> {
    reader.read_f64_le().map_err(|e| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_pdf_decimal_bin(out: &mut Vec<u8>, value: &PdfDecimal) {
    out.push(value.negative as u8);
    write_str_lp(out, &value.coefficient);
    store::pack_rt::write_varint_u64(out, value.scale as u64);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_pdf_decimal_bin(reader: &mut store::ByteReader<'_>) -> Result<PdfDecimal, String> {
    let negative = reader.read_u8().map_err(|error| error.to_string())? != 0;
    let coefficient = read_str_lp(reader)?;
    let scale = reader.read_varint_u64().map_err(|error| error.to_string())? as u32;
    if coefficient.is_empty() || !coefficient.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err("invalid PDF decimal coefficient".into());
    }
    Ok(PdfDecimal { negative, coefficient, scale })
}
/// ➡️ Zigzag-encodes an `i64` into the `u64` varint domain (`store::pack_rt` only re-exports the
/// UNSIGNED varint writer, `write_varint_u64` -- `store::ByteReader::read_varint_i64` exists as a
/// real method on the read side, but there is no matching free-function writer, so the encode
/// half is reproduced here verbatim from `🎒️pack/🧾️codec/🦀️component.rs`'s own private
/// `zigzag_encode`, same formula, not reinvented).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_varint_i64_bin(out: &mut Vec<u8>, value: i64) {
    let zigzag = ((value << 1) ^ (value >> 63)) as u64;
    store::pack_rt::write_varint_u64(out, zigzag);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_stream_filters_bin(filters: &[PdfStreamFilter], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, filters.len() as u64);
    for filter in filters {
        match filter {
            PdfStreamFilter::Flate { predictor } => {
                out.push(0);
                match predictor {
                    None => out.push(0),
                    Some(predictor) => {
                        out.push(1);
                        for value in [predictor.predictor, predictor.colors, predictor.bits_per_component, predictor.columns] {
                            store::pack_rt::write_varint_u64(out, value as u64);
                        }
                    }
                }
            }
            PdfStreamFilter::AsciiHex => out.push(1),
            PdfStreamFilter::Ascii85 => out.push(2),
            PdfStreamFilter::RunLength => out.push(3),
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_stream_filters_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<PdfStreamFilter>, String> {
    let count = reader.read_varint_u64().map_err(|error| error.to_string())?;
    let mut filters = Vec::with_capacity(count as usize);
    for _ in 0..count {
        filters.push(match reader.read_u8().map_err(|error| error.to_string())? {
            0 => {
                let predictor = match reader.read_u8().map_err(|error| error.to_string())? {
                    0 => None,
                    1 => Some(PdfPredictor {
                        predictor: reader.read_varint_u64().map_err(|error| error.to_string())? as u32,
                        colors: reader.read_varint_u64().map_err(|error| error.to_string())? as u32,
                        bits_per_component: reader.read_varint_u64().map_err(|error| error.to_string())? as u32,
                        columns: reader.read_varint_u64().map_err(|error| error.to_string())? as u32,
                    }),
                    tag => return Err(format!("flate predictor presence: unknown tag {tag}")),
                };
                PdfStreamFilter::Flate { predictor }
            }
            1 => PdfStreamFilter::AsciiHex,
            2 => PdfStreamFilter::Ascii85,
            3 => PdfStreamFilter::RunLength,
            tag => return Err(format!("stream filter binary: unknown tag {tag}")),
        });
    }
    Ok(filters)
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️ObjectValueBinaryCodecs
/// 🧪️ P2-FG3: real recursive binary twins of [`enc_objref`]/[`enc_box`]/[`enc_pdf_object`]/
/// [`enc_pdf_page`]/[`enc_pdf_info`] above -- backs the upgraded `OpBinary`
/// (`../🧬️mutations/🦀️component.rs`, direct mutation payloads) and `DiffCodec`
/// frames below. `pub(crate)` so the sibling `../🧬️mutations/🦀️component.rs` (same artifact,
/// different facet module) can reuse these rather than duplicating them a second time, matching
/// this file's own existing text-codec reuse convention.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_objref_bin(r: &ObjRef, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, r.num as u64);
    store::pack_rt::write_varint_u64(out, r.gen as u64);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_objref_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjRef, String> {
    let num = reader.read_varint_u64().map_err(|e| e.to_string())? as u32;
    let gen = reader.read_varint_u64().map_err(|e| e.to_string())? as u16;
    Ok(ObjRef { num, gen })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_box_bin(b: &[f64; 4], out: &mut Vec<u8>) {
    for v in b {
        write_f64_bin(out, *v);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_box_bin(reader: &mut store::ByteReader<'_>) -> Result<[f64; 4], String> {
    Ok([read_f64_bin(reader)?, read_f64_bin(reader)?, read_f64_bin(reader)?, read_f64_bin(reader)?])
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_path_segment_bin(seg: &PdfPathSegment, out: &mut Vec<u8>) {
    match seg {
        PdfPathSegment::ArrayIndex { index } => {
            out.push(0);
            store::pack_rt::write_varint_u64(out, *index as u64);
        }
        PdfPathSegment::DictKey { key } => {
            out.push(1);
            write_str_lp(out, key);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_path_segment_bin(reader: &mut store::ByteReader<'_>) -> Result<PdfPathSegment, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(PdfPathSegment::ArrayIndex { index: reader.read_varint_u64().map_err(|e| e.to_string())? as usize }),
        1 => Ok(PdfPathSegment::DictKey { key: read_str_lp(reader)? }),
        other => Err(format!("path segment binary: unknown tag {other}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_path_bin(path: &[PdfPathSegment], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, path.len() as u64);
    for seg in path {
        enc_path_segment_bin(seg, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_path_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<PdfPathSegment>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut path = Vec::with_capacity(count as usize);
    for _ in 0..count {
        path.push(dec_path_segment_bin(reader)?);
    }
    Ok(path)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_dict_entry_bin(e: &PdfDictEntry, out: &mut Vec<u8>) {
    write_str_lp(out, &e.key);
    enc_pdf_object_bin(&e.value, out);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_dict_entry_bin(reader: &mut store::ByteReader<'_>) -> Result<PdfDictEntry, String> {
    let key = read_str_lp(reader)?;
    let value = dec_pdf_object_bin(reader)?;
    Ok(PdfDictEntry { key, value })
}
/// 🌳 Recursive: a 1-byte kind tag (`0`=Null/`1`=Bool/`2`=Int/`3`=Real/`4`=Str/`5`=Name/`6`=Array/
/// `7`=Dict/`8`=Ref/`9`=Stream -- distinct numbering from the text codec's letter tags) followed
/// by the real payload (LEB128 varints for `Int`/counts, raw LE `f64` for `Real`, length-prefixed
/// bytes/utf8 for `Str`/`Name`, a varint COUNT then that many recursively-encoded items for
/// `Array`/`Dict` -- genuinely recursive, not text-as-bytes).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_pdf_object_bin(v: &PdfObject, out: &mut Vec<u8>) {
    match v {
        PdfObject::Null => out.push(0),
        PdfObject::Bool(b) => {
            out.push(1);
            out.push(if *b { 1 } else { 0 });
        }
        PdfObject::Int(i) => {
            out.push(2);
            write_varint_i64_bin(out, *i);
        }
        PdfObject::Real(f) => {
            out.push(3);
            write_pdf_decimal_bin(out, f);
        }
        PdfObject::Str(bytes) => {
            out.push(4);
            write_bytes_lp(out, bytes);
        }
        PdfObject::Name(s) => {
            out.push(5);
            write_str_lp(out, s);
        }
        PdfObject::Array(items) => {
            out.push(6);
            store::pack_rt::write_varint_u64(out, items.len() as u64);
            for item in items {
                enc_pdf_object_bin(item, out);
            }
        }
        PdfObject::Dict(entries) => {
            out.push(7);
            store::pack_rt::write_varint_u64(out, entries.len() as u64);
            for entry in entries {
                enc_dict_entry_bin(entry, out);
            }
        }
        PdfObject::Ref(r) => {
            out.push(8);
            enc_objref_bin(r, out);
        }
        PdfObject::Stream { dict, data, filters } => {
            out.push(9);
            store::pack_rt::write_varint_u64(out, dict.len() as u64);
            for entry in dict {
                enc_dict_entry_bin(entry, out);
            }
            write_bytes_lp(out, data);
            enc_stream_filters_bin(filters, out);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_pdf_object_bin(reader: &mut store::ByteReader<'_>) -> Result<PdfObject, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(PdfObject::Null),
        1 => Ok(PdfObject::Bool(reader.read_u8().map_err(|e| e.to_string())? != 0)),
        2 => Ok(PdfObject::Int(reader.read_varint_i64().map_err(|e| e.to_string())?)),
        3 => Ok(PdfObject::Real(read_pdf_decimal_bin(reader)?)),
        4 => Ok(PdfObject::Str(read_bytes_lp(reader)?)),
        5 => Ok(PdfObject::Name(read_str_lp(reader)?)),
        6 => {
            let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut items = Vec::with_capacity(count as usize);
            for _ in 0..count {
                items.push(dec_pdf_object_bin(reader)?);
            }
            Ok(PdfObject::Array(items))
        }
        7 => {
            let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut entries = Vec::with_capacity(count as usize);
            for _ in 0..count {
                entries.push(dec_dict_entry_bin(reader)?);
            }
            Ok(PdfObject::Dict(entries))
        }
        8 => Ok(PdfObject::Ref(dec_objref_bin(reader)?)),
        9 => {
            let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut dict = Vec::with_capacity(count as usize);
            for _ in 0..count {
                dict.push(dec_dict_entry_bin(reader)?);
            }
            let data = read_bytes_lp(reader)?;
            let filters = dec_stream_filters_bin(reader)?;
            Ok(PdfObject::Stream { dict, data, filters })
        }
        other => Err(format!("pdf object binary: unknown tag {other}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_pdf_page_bin(p: &PdfPage, out: &mut Vec<u8>) {
    enc_box_bin(&p.media_box, out);
    out.push(if p.crop_box.is_some() { 1 } else { 0 });
    if let Some(cb) = &p.crop_box {
        enc_box_bin(cb, out);
    }
    write_varint_i64_bin(out, p.rotate as i64);
    write_str_lp(out, &p.text);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_pdf_page_bin(reader: &mut store::ByteReader<'_>) -> Result<PdfPage, String> {
    let media_box = dec_box_bin(reader)?;
    let crop_box = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_box_bin(reader)?) } else { None };
    let rotate = reader.read_varint_i64().map_err(|e| e.to_string())? as i32;
    let text = read_str_lp(reader)?;
    Ok(PdfPage { media_box, crop_box, rotate, text })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_pdf_info_bin(i: &PdfInfo, out: &mut Vec<u8>) {
    for field in [&i.title, &i.author, &i.subject, &i.keywords, &i.creator, &i.producer] {
        out.push(if field.is_some() { 1 } else { 0 });
        if let Some(v) = field {
            write_str_lp(out, v);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_pdf_info_bin(reader: &mut store::ByteReader<'_>) -> Result<PdfInfo, String> {
    let mut read_opt = || -> Result<Option<String>, String> { Ok(if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None }) };
    Ok(PdfInfo { title: read_opt()?, author: read_opt()?, subject: read_opt()?, keywords: read_opt()?, creator: read_opt()?, producer: read_opt()? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_pdf_snapshot_bin(s: &PdfSnapshot, out: &mut Vec<u8>) {
    write_str_lp(out, &s.schema);
    write_str_lp(out, &s.declared_version);
    store::pack_rt::write_varint_u64(out, s.pages.len() as u64);
    for page in &s.pages {
        enc_pdf_page_bin(page, out);
    }
    enc_pdf_info_bin(&s.info, out);
    store::pack_rt::write_varint_u64(out, s.objects.len() as u64);
    for obj in &s.objects {
        enc_objref_bin(&obj.id, out);
        enc_pdf_object_bin(&obj.value, out);
    }
    store::pack_rt::write_varint_u64(out, s.trailer.len() as u64);
    for entry in &s.trailer {
        enc_dict_entry_bin(entry, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_pdf_snapshot_bin(reader: &mut store::ByteReader<'_>) -> Result<PdfSnapshot, String> {
    let schema = read_str_lp(reader)?;
    let declared_version = read_str_lp(reader)?;
    let page_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut pages = Vec::with_capacity(page_count as usize);
    for _ in 0..page_count {
        pages.push(dec_pdf_page_bin(reader)?);
    }
    let info = dec_pdf_info_bin(reader)?;
    let obj_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut objects = Vec::with_capacity(obj_count as usize);
    for _ in 0..obj_count {
        let id = dec_objref_bin(reader)?;
        let value = dec_pdf_object_bin(reader)?;
        objects.push(PdfIndirectObject { id, value });
    }
    let trailer_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut trailer = Vec::with_capacity(trailer_count as usize);
    for _ in 0..trailer_count {
        trailer.push(dec_dict_entry_bin(reader)?);
    }
    Ok(PdfSnapshot { schema, declared_version, pages, info, objects, trailer })
}
//#endregion 🔖️ObjectValueBinaryCodecs

//#region 🔖️DiffValueCodecs
/// 📦️ Index-keyed `pages` triple — `modified` carries the sparse `PdfPageDiff` (single-letter
/// tag:value pairs, same shape `GifFrameDiff`'s hand-rolled codec uses), `added` carries a full
/// `PdfPage`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_pages_diff(d: &PdfPagesDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.index, enc_page_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_pdf_page(&a.page))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_pages_diff(body: &str) -> Result<PdfPagesDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("pages diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("pages modified: bad entry {entry:?}"))?;
            Ok(PdfPageModified { index: parse_usize(idx)?, diff: dec_page_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("pages added: bad entry {entry:?}"))?;
            Ok(PdfPageAdded { index: parse_usize(idx)?, page: dec_pdf_page(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(PdfPagesDiff { removed, modified, added })
}
/// 🏷️ `PdfPageDiff`'s own sparse fields as single-letter `tag:value` pairs inside `[...]` — same
/// shape as gif 89a's hand-rolled `enc_frame_diff`. `M`=media_box, `C`=crop_box (tri-state,
/// ONE level of `encode_option` over the inner `Option<[f64;4]>`), `R`=rotate, `X`=text.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_page_diff(d: &PdfPageDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = &d.media_box {
        parts.push(format!("M:{}", enc_box(v)));
    }
    if let Some(v) = &d.crop_box {
        parts.push(format!("C:{}", encode_option(v, enc_box)));
    }
    if let Some(v) = d.rotate {
        parts.push(format!("R:{v}"));
    }
    if let Some(v) = &d.text {
        parts.push(format!("X:{}", enc_str(v)));
    }
    format!("[{}]", parts.join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_page_diff(s: &str) -> Result<PdfPageDiff, String> {
    let inner = strip_brackets(s)?;
    let mut d = PdfPageDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("page diff: bad entry {entry:?}"))?;
        match tag {
            "M" => d.media_box = Some(dec_box(val)?),
            "C" => d.crop_box = Some(decode_option(val, dec_box)?),
            "R" => d.rotate = Some(val.parse().map_err(|e: std::num::ParseIntError| e.to_string())?),
            "X" => d.text = Some(dec_str(val)?),
            other => return Err(format!("page diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}
/// 📦️ Name-keyed `Dict`/`Stream.dict`/`trailer` triple — reused verbatim for all three per the
/// recipe's "trailer is itself a Dict-shaped structure" guidance (mirrors `PdfDictDiff`'s own Rust
/// shape). Keys are hex (may contain any byte a real PDF name can).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_dict_diff(d: &PdfDictDiff) -> String {
    let removed = d.removed.iter().map(|k| enc_str(k)).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", enc_str(&m.key), enc_value_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}:{}", a.index, enc_str(&a.key), enc_pdf_object(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_dict_diff(body: &str) -> Result<PdfDictDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("dict diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_str).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (key, rest) = entry.split_once(':').ok_or_else(|| format!("dict modified: bad entry {entry:?}"))?;
            Ok(PdfDictModified { key: dec_str(key)?, diff: dec_value_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("dict added: bad entry {entry:?}"))?;
            let (key, item) = rest.split_once(':').ok_or_else(|| format!("dict added: bad entry {entry:?}"))?;
            Ok(PdfDictAdded { index: parse_usize(idx)?, key: dec_str(key)?, item: dec_pdf_object(item)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(PdfDictDiff { removed, modified, added })
}
/// 📦️ Index-keyed `Array` triple (nested inside `PdfValueDiff::Array` only).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_array_diff(d: &PdfArrayDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.index, enc_value_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_pdf_object(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_array_diff(body: &str) -> Result<PdfArrayDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("array diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("array modified: bad entry {entry:?}"))?;
            Ok(PdfArrayModified { index: parse_usize(idx)?, diff: dec_value_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("array added: bad entry {entry:?}"))?;
            Ok(PdfArrayAdded { index: parse_usize(idx)?, item: dec_pdf_object(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(PdfArrayDiff { removed, modified, added })
}
/// 🌳 Recursive, mirrors `enc_pdf_object`'s tag vocabulary: `L`=Replace (whole-node), `B`/`I`/`R`/
/// `S`/`N`/`F`=scalar diffs (new value only, kind is stable), `A[..]`=Array diff, `D[..]`=Dict
/// diff, `T[..]`=Stream diff (its own sparse `D:`/`A:` pairs for `dict`/decoded `data`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_value_diff(d: &PdfValueDiff) -> String {
    match d {
        PdfValueDiff::Replace { value } => format!("L[{}]", enc_pdf_object(value)),
        PdfValueDiff::Bool { value } => format!("B[{}]", if *value { "1" } else { "0" }),
        PdfValueDiff::Int { value } => format!("I[{value}]"),
        PdfValueDiff::Real { value } => format!("R[{value}]"),
        PdfValueDiff::Str { value } => format!("S[{}]", hex_encode(value)),
        PdfValueDiff::Name { value } => format!("N[{}]", enc_str(value)),
        PdfValueDiff::Ref { value } => format!("F[{}]", enc_objref(value)),
        PdfValueDiff::Array { diff } => format!("A[{}]", enc_array_diff(diff)),
        PdfValueDiff::Dict { diff } => format!("D[{}]", enc_dict_diff(diff)),
        PdfValueDiff::Stream { dict, data, filters } => {
            let mut parts = Vec::new();
            if let Some(v) = dict {
                parts.push(format!("D:{}", enc_dict_diff(v)));
            }
            if let Some(v) = data {
                parts.push(format!("A:{}", hex_encode(v)));
            }
            if let Some(v) = filters {
                parts.push(format!("F:{}", enc_stream_filters(v)));
            }
            format!("T[{}]", parts.join(","))
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_value_diff(s: &str) -> Result<PdfValueDiff, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "L" => Ok(PdfValueDiff::Replace { value: dec_pdf_object(inner)? }),
        "B" => Ok(PdfValueDiff::Bool { value: inner == "1" }),
        "I" => Ok(PdfValueDiff::Int { value: inner.parse().map_err(|e: std::num::ParseIntError| e.to_string())? }),
        "R" => Ok(PdfValueDiff::Real { value: PdfDecimal::parse(inner)? }),
        "S" => Ok(PdfValueDiff::Str { value: hex_decode(inner)? }),
        "N" => Ok(PdfValueDiff::Name { value: dec_str(inner)? }),
        "F" => Ok(PdfValueDiff::Ref { value: dec_objref(inner)? }),
        "A" => Ok(PdfValueDiff::Array { diff: dec_array_diff(inner)? }),
        "D" => Ok(PdfValueDiff::Dict { diff: dec_dict_diff(inner)? }),
        "T" => {
            let mut dict = None;
            let mut data = None;
            let mut filters = None;
            for entry in split_top_level(inner, ',') {
                if entry.is_empty() {
                    continue;
                }
                let (etag, val) = entry.split_once(':').ok_or_else(|| format!("stream diff: bad entry {entry:?}"))?;
                match etag {
                    "D" => dict = Some(dec_dict_diff(val)?),
                    "A" => data = Some(hex_decode(val)?),
                    "F" => filters = Some(dec_stream_filters(val)?),
                    other => return Err(format!("stream diff: unknown tag {other:?}")),
                }
            }
            Ok(PdfValueDiff::Stream { dict, data, filters })
        }
        other => Err(format!("value diff: unknown tag {other:?}")),
    }
}
/// 📦️ `(id,gen)`-keyed `objects` triple.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_objects_diff(d: &PdfObjectsDiff) -> String {
    let removed = d.removed.iter().map(enc_objref).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", enc_objref(&m.id), enc_value_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}:{}", a.index, enc_objref(&a.id), enc_pdf_object(&a.value))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_objects_diff(body: &str) -> Result<PdfObjectsDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("objects diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_objref).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (id, rest) = entry.split_once(':').ok_or_else(|| format!("objects modified: bad entry {entry:?}"))?;
            Ok(PdfObjectModified { id: dec_objref(id)?, diff: dec_value_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("objects added: bad entry {entry:?}"))?;
            let (id, value) = rest.split_once(':').ok_or_else(|| format!("objects added: bad entry {entry:?}"))?;
            Ok(PdfObjectAdded { index: parse_usize(idx)?, id: dec_objref(id)?, value: dec_pdf_object(value)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(PdfObjectsDiff { removed, modified, added })
}
//#endregion 🔖️DiffValueCodecs

//#region 🔖️DiffValueBinaryCodecs
/// 🧪️ P2-FG3: real recursive binary twins of [`enc_page_diff`]/[`enc_pages_diff`]/
/// [`enc_dict_diff`]/[`enc_array_diff`]/[`enc_value_diff`]/[`enc_objects_diff`] above -- backs the
/// upgraded `DiffCodec::encode_diff`/`decode_diff` below. Same 1-byte tag numbering scheme as
/// [`enc_pdf_object_bin`] for `value-diff`'s scalar arms, plus `0`=Replace and `7`/`8`/`9`=
/// Array/Dict/Stream (distinct from `enc_pdf_object_bin`'s own numbering since `PdfValueDiff` has
/// one extra variant, `Replace`, that `PdfObject` doesn't). Collection triples (`removed`/
/// `modified`/`added`) each encode as three varint-counted, recursively-encoded lists --
/// genuinely structured binary, never text-as-bytes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_page_diff_bin(d: &PdfPageDiff, out: &mut Vec<u8>) {
    out.push(if d.media_box.is_some() { 1 } else { 0 });
    if let Some(v) = &d.media_box {
        enc_box_bin(v, out);
    }
    out.push(if d.crop_box.is_some() { 1 } else { 0 });
    if let Some(v) = &d.crop_box {
        out.push(if v.is_some() { 1 } else { 0 });
        if let Some(b) = v {
            enc_box_bin(b, out);
        }
    }
    out.push(if d.rotate.is_some() { 1 } else { 0 });
    if let Some(v) = d.rotate {
        write_varint_i64_bin(out, v as i64);
    }
    out.push(if d.text.is_some() { 1 } else { 0 });
    if let Some(v) = &d.text {
        write_str_lp(out, v);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_page_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<PdfPageDiff, String> {
    let mut d = PdfPageDiff::default();
    if reader.read_u8().map_err(|e| e.to_string())? != 0 {
        d.media_box = Some(dec_box_bin(reader)?);
    }
    if reader.read_u8().map_err(|e| e.to_string())? != 0 {
        d.crop_box = Some(if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_box_bin(reader)?) } else { None });
    }
    if reader.read_u8().map_err(|e| e.to_string())? != 0 {
        d.rotate = Some(reader.read_varint_i64().map_err(|e| e.to_string())? as i32);
    }
    if reader.read_u8().map_err(|e| e.to_string())? != 0 {
        d.text = Some(read_str_lp(reader)?);
    }
    Ok(d)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_pages_diff_bin(d: &PdfPagesDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    for idx in &d.removed {
        store::pack_rt::write_varint_u64(out, *idx as u64);
    }
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for m in &d.modified {
        store::pack_rt::write_varint_u64(out, m.index as u64);
        enc_page_diff_bin(&m.diff, out);
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for a in &d.added {
        store::pack_rt::write_varint_u64(out, a.index as u64);
        enc_pdf_page_bin(&a.page, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_pages_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<PdfPagesDiff, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(reader.read_varint_u64().map_err(|e| e.to_string())? as usize);
    }
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let diff = dec_page_diff_bin(reader)?;
        modified.push(PdfPageModified { index, diff });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let page = dec_pdf_page_bin(reader)?;
        added.push(PdfPageAdded { index, page });
    }
    Ok(PdfPagesDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_dict_diff_bin(d: &PdfDictDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    for key in &d.removed {
        write_str_lp(out, key);
    }
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for m in &d.modified {
        write_str_lp(out, &m.key);
        enc_value_diff_bin(&m.diff, out);
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for a in &d.added {
        store::pack_rt::write_varint_u64(out, a.index as u64);
        write_str_lp(out, &a.key);
        enc_pdf_object_bin(&a.item, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_dict_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<PdfDictDiff, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(read_str_lp(reader)?);
    }
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let key = read_str_lp(reader)?;
        let diff = dec_value_diff_bin(reader)?;
        modified.push(PdfDictModified { key, diff });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let key = read_str_lp(reader)?;
        let item = dec_pdf_object_bin(reader)?;
        added.push(PdfDictAdded { index, key, item });
    }
    Ok(PdfDictDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_array_diff_bin(d: &PdfArrayDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    for idx in &d.removed {
        store::pack_rt::write_varint_u64(out, *idx as u64);
    }
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for m in &d.modified {
        store::pack_rt::write_varint_u64(out, m.index as u64);
        enc_value_diff_bin(&m.diff, out);
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for a in &d.added {
        store::pack_rt::write_varint_u64(out, a.index as u64);
        enc_pdf_object_bin(&a.item, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_array_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<PdfArrayDiff, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(reader.read_varint_u64().map_err(|e| e.to_string())? as usize);
    }
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let diff = dec_value_diff_bin(reader)?;
        modified.push(PdfArrayModified { index, diff });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let item = dec_pdf_object_bin(reader)?;
        added.push(PdfArrayAdded { index, item });
    }
    Ok(PdfArrayDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_value_diff_bin(d: &PdfValueDiff, out: &mut Vec<u8>) {
    match d {
        PdfValueDiff::Replace { value } => {
            out.push(0);
            enc_pdf_object_bin(value, out);
        }
        PdfValueDiff::Bool { value } => {
            out.push(1);
            out.push(if *value { 1 } else { 0 });
        }
        PdfValueDiff::Int { value } => {
            out.push(2);
            write_varint_i64_bin(out, *value);
        }
        PdfValueDiff::Real { value } => {
            out.push(3);
            write_pdf_decimal_bin(out, value);
        }
        PdfValueDiff::Str { value } => {
            out.push(4);
            write_bytes_lp(out, value);
        }
        PdfValueDiff::Name { value } => {
            out.push(5);
            write_str_lp(out, value);
        }
        PdfValueDiff::Ref { value } => {
            out.push(6);
            enc_objref_bin(value, out);
        }
        PdfValueDiff::Array { diff } => {
            out.push(7);
            enc_array_diff_bin(diff, out);
        }
        PdfValueDiff::Dict { diff } => {
            out.push(8);
            enc_dict_diff_bin(diff, out);
        }
        PdfValueDiff::Stream { dict, data, filters } => {
            out.push(9);
            out.push(if dict.is_some() { 1 } else { 0 });
            if let Some(v) = dict {
                enc_dict_diff_bin(v, out);
            }
            out.push(if data.is_some() { 1 } else { 0 });
            if let Some(v) = data {
                write_bytes_lp(out, v);
            }
            out.push(if filters.is_some() { 1 } else { 0 });
            if let Some(v) = filters {
                enc_stream_filters_bin(v, out);
            }
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_value_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<PdfValueDiff, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(PdfValueDiff::Replace { value: dec_pdf_object_bin(reader)? }),
        1 => Ok(PdfValueDiff::Bool { value: reader.read_u8().map_err(|e| e.to_string())? != 0 }),
        2 => Ok(PdfValueDiff::Int { value: reader.read_varint_i64().map_err(|e| e.to_string())? }),
        3 => Ok(PdfValueDiff::Real { value: read_pdf_decimal_bin(reader)? }),
        4 => Ok(PdfValueDiff::Str { value: read_bytes_lp(reader)? }),
        5 => Ok(PdfValueDiff::Name { value: read_str_lp(reader)? }),
        6 => Ok(PdfValueDiff::Ref { value: dec_objref_bin(reader)? }),
        7 => Ok(PdfValueDiff::Array { diff: dec_array_diff_bin(reader)? }),
        8 => Ok(PdfValueDiff::Dict { diff: dec_dict_diff_bin(reader)? }),
        9 => {
            let dict = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_dict_diff_bin(reader)?) } else { None };
            let data = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_bytes_lp(reader)?) } else { None };
            let filters = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_stream_filters_bin(reader)?) } else { None };
            Ok(PdfValueDiff::Stream { dict, data, filters })
        }
        other => Err(format!("value diff binary: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_objects_diff_bin(d: &PdfObjectsDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    for id in &d.removed {
        enc_objref_bin(id, out);
    }
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for m in &d.modified {
        enc_objref_bin(&m.id, out);
        enc_value_diff_bin(&m.diff, out);
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for a in &d.added {
        store::pack_rt::write_varint_u64(out, a.index as u64);
        enc_objref_bin(&a.id, out);
        enc_pdf_object_bin(&a.value, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_objects_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<PdfObjectsDiff, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(dec_objref_bin(reader)?);
    }
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let id = dec_objref_bin(reader)?;
        let diff = dec_value_diff_bin(reader)?;
        modified.push(PdfObjectModified { id, diff });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let id = dec_objref_bin(reader)?;
        let value = dec_pdf_object_bin(reader)?;
        added.push(PdfObjectAdded { index, id, value });
    }
    Ok(PdfObjectsDiff { removed, modified, added })
}
//#endregion 🔖️DiffValueBinaryCodecs

//#region 🔖️TopLevel
/// **Grammar**: one space-separated `name=value` token per changed top-level field (absent token
/// = unchanged); `pages`/`objects`/`trailer` print via their own collection-triple/dict-triple
/// codecs above. `declaredVersion`/`info` are plain (non-tri-state) `Option<T>` fields — direct
/// value encoding, no `encode_option` wrapper (the token's own presence already IS the "touched"
/// bit, same convention `SvgDiff`'s `declaration=`/`doctype=` tri-states use one level down from
/// theirs since `PdfDiff` has no tri-state fields of its own — only nested `PdfPageDiff.crop_box`/
/// `PdfValueDiff::Stream.filters` are handled inside their own sub-codecs above).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_pdf_diff(d: &PdfDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.declared_version {
        tokens.push(format!("declared-version={}", enc_str(v)));
    }
    if let Some(v) = &d.info {
        tokens.push(format!("info={}", enc_pdf_info(v)));
    }
    if let Some(v) = &d.pages {
        tokens.push(format!("pages={}", enc_pages_diff(v)));
    }
    if let Some(v) = &d.objects {
        tokens.push(format!("objects={}", enc_objects_diff(v)));
    }
    if let Some(v) = &d.trailer {
        tokens.push(format!("trailer={}", enc_dict_diff(v)));
    }
    tokens.join(" ")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_pdf_diff(line: &str) -> Result<PdfDiff, String> {
    let mut d = PdfDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("declared-version=") {
            d.declared_version = Some(dec_str(rest)?);
        } else if let Some(rest) = token.strip_prefix("info=") {
            d.info = Some(dec_pdf_info(rest)?);
        } else if let Some(rest) = token.strip_prefix("pages=") {
            d.pages = Some(dec_pages_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("objects=") {
            d.objects = Some(dec_objects_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("trailer=") {
            d.trailer = Some(dec_dict_diff(rest)?);
        } else {
            return Err(format!("pdf diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffCodec for PdfDiff {
    fn print_diff(&self) -> String {
        print_pdf_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_pdf_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// 🧪️ P2-FG3: REAL binary frame (`format u8 | flags u8 | [declared_version][info][pages]
    /// [objects][trailer]`), matching `../💾️binary/📡️component.protocol.semio`'s `header fixed 2`
    /// + `chain payload bytes` shape — upgraded from F6's `print_diff().into_bytes()`
    /// text-as-binary shortcut (100% of stdio's `DiffCodec` impls were still on that shortcut per
    /// the P2-W0 census). `flags` bits 0-5 mark `declared_version`/`info`/`pages`/`objects`/
    /// `trailer` presence; each present field's own (genuinely recursive, LEB128-varint/
    /// length-prefixed binary) payload follows in that fixed order.
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut flags: u8 = 0;
        if self.declared_version.is_some() {
            flags |= 0b00001;
        }
        if self.info.is_some() {
            flags |= 0b00010;
        }
        if self.pages.is_some() {
            flags |= 0b00100;
        }
        if self.objects.is_some() {
            flags |= 0b01000;
        }
        if self.trailer.is_some() {
            flags |= 0b010000;
        }
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, flags];
        if let Some(v) = &self.declared_version {
            write_str_lp(&mut out, v);
        }
        if let Some(v) = &self.info {
            enc_pdf_info_bin(v, &mut out);
        }
        if let Some(v) = &self.pages {
            enc_pages_diff_bin(v, &mut out);
        }
        if let Some(v) = &self.objects {
            enc_objects_diff_bin(v, &mut out);
        }
        if let Some(v) = &self.trailer {
            enc_dict_diff_bin(v, &mut out);
        }
        Ok(out)
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let format = reader.read_u8().map_err(|e| malformed("diff format", 0, e.to_string()))?;
        if format != store::pack_rt::OP_BINARY_FORMAT {
            return Err(malformed("diff format", 0, format!("expected {}, got {format}", store::pack_rt::OP_BINARY_FORMAT)));
        }
        let flags = reader.read_u8().map_err(|e| malformed("diff flags", 1, e.to_string()))?;
        if flags & !0b0001_1111 != 0 {
            return Err(malformed("diff flags", 1, format!("unknown flag bits {:#010b}", flags & !0b0001_1111)));
        }
        let declared_version = if flags & 0b00001 != 0 { Some(read_str_lp(&mut reader).map_err(|e| malformed("diff declared_version", reader.position(), e))?) } else { None };
        let info = if flags & 0b00010 != 0 { Some(dec_pdf_info_bin(&mut reader).map_err(|e| malformed("diff info", reader.position(), e))?) } else { None };
        let pages = if flags & 0b00100 != 0 { Some(dec_pages_diff_bin(&mut reader).map_err(|e| malformed("diff pages", reader.position(), e))?) } else { None };
        let objects = if flags & 0b01000 != 0 { Some(dec_objects_diff_bin(&mut reader).map_err(|e| malformed("diff objects", reader.position(), e))?) } else { None };
        let trailer = if flags & 0b010000 != 0 { Some(dec_dict_diff_bin(&mut reader).map_err(|e| malformed("diff trailer", reader.position(), e))?) } else { None };
        if reader.remaining() != 0 {
            return Err(malformed("diff trailing bytes", reader.position(), format!("{} trailing bytes", reader.remaining())));
        }
        Ok(PdfDiff { declared_version, info, pages, objects, trailer })
    }
}
//#endregion 🔖️TopLevel

//#region 🧪️Tests
#[cfg(test)]
mod handcrafted_diff_codec_tests {
    use super::*;
    use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::snapshot::{PdfIndirectObject, STDIO_PDF17_DOCUMENT_SCHEMA};
    use protocol::DiffCodec;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn oref(num: u32, gen: u16) -> ObjRef {
        ObjRef { num, gen }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn page(mb: [f64; 4], cb: Option<[f64; 4]>, rotate: i32, text: &str) -> PdfPage {
        PdfPage { media_box: mb, crop_box: cb, rotate, text: text.into() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn dict(entries: Vec<(&str, PdfObject)>) -> PdfObject {
        PdfObject::Dict(entries.into_iter().map(|(k, v)| PdfDictEntry { key: k.into(), value: v }).collect())
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn entry(k: &str, v: PdfObject) -> PdfDictEntry {
        PdfDictEntry { key: k.into(), value: v }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn a_snapshot() -> PdfSnapshot {
        PdfSnapshot {
            schema: STDIO_PDF17_DOCUMENT_SCHEMA.into(),
            declared_version: "1.7".into(),
            pages: vec![page([0.0, 0.0, 100.0, 100.0], None, 0, "one"), page([0.0, 0.0, 50.0, 50.0], Some([1.0, 1.0, 2.0, 2.0]), 0, "two")],
            info: PdfInfo { title: Some("Base".into()), ..Default::default() },
            objects: vec![
                PdfIndirectObject { id: oref(1, 0), value: dict(vec![("Type", PdfObject::Name("Catalog".into())), ("Count", PdfObject::Int(3))]) },
                PdfIndirectObject { id: oref(2, 0), value: PdfObject::Stream { dict: vec![entry("Length", PdfObject::Int(3))], data: vec![1, 2, 3], filters: vec![PdfStreamFilter::Flate { predictor: None }] } },
                PdfIndirectObject { id: oref(3, 0), value: PdfObject::Array(vec![PdfObject::Int(1), PdfObject::Real(2.5.into()), PdfObject::Ref(oref(1, 0))]) },
            ],
            trailer: vec![entry("Root", PdfObject::Ref(oref(1, 0))), entry("Size", PdfObject::Int(3))],
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn b_snapshot() -> PdfSnapshot {
        PdfSnapshot {
            schema: STDIO_PDF17_DOCUMENT_SCHEMA.into(),
            declared_version: "1.4".into(),
            pages: vec![page([0.0, 0.0, 200.0, 200.0], None, 90, "ONE")],
            info: PdfInfo { title: Some("Changed".into()), author: Some("Ueli".into()), ..Default::default() },
            objects: vec![
                PdfIndirectObject { id: oref(1, 0), value: dict(vec![("Type", PdfObject::Name("Catalog".into())), ("Count", PdfObject::Int(4)), ("New", PdfObject::Bool(false))]) },
                PdfIndirectObject { id: oref(2, 0), value: PdfObject::Stream { dict: vec![entry("Length", PdfObject::Int(3))], data: vec![9, 9], filters: vec![] } },
                PdfIndirectObject { id: oref(4, 0), value: PdfObject::Null },
            ],
            trailer: vec![entry("Root", PdfObject::Ref(oref(1, 0))), entry("Size", PdfObject::Int(4)), entry("Prev", PdfObject::Int(100))],
        }
    }

    /// 🧪️ F6: `DiffCodec` round-trip laws over the hand-rolled `PdfDiff` grammar — exercises the
    /// recursive `PdfValueDiff` tree (`Replace`/`Array`/`Dict`/`Stream` variants, incl. `Stream`'s
    /// own typed filter pipeline), the index-keyed `pages` triple (incl. `PdfPageDiff`'s tri-state
    /// `crop_box`), the id-keyed `objects` triple, and the name-keyed `trailer` triple.
    #[test]
    fn diff_codec_text_binary_roundtrip_law() {
        let a = a_snapshot();
        let b = b_snapshot();
        let cases = vec![PdfDiff::default(), PdfDiff::between(&a, &b), PdfDiff::between(&b, &a), PdfDiff::between(&a, &a)];
        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = PdfDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = PdfDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
    #[test]
    fn rejects_missing_page_target_without_mutating_base() {
        let base = PdfSnapshot::default();
        let diff = PdfDiff { pages: Some(PdfPagesDiff { modified: vec![PdfPageModified { index: 0, diff: PdfPageDiff::default() }], ..Default::default() }), ..Default::default() };
        let result = diff.apply(&base);
        assert_eq!(result.unwrap_err().code, "mutation.apply.missing-target");
        assert_eq!(base, PdfSnapshot::default());
    }
}
//#endregion 🧪️Tests
//#endregion 🔖️HandcraftedDiffCodec

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::snapshot::{PdfIndirectObject, STDIO_PDF17_DOCUMENT_SCHEMA};

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn oref(num: u32, gen: u16) -> ObjRef {
        ObjRef { num, gen }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn page(mb: [f64; 4], cb: Option<[f64; 4]>, rotate: i32, text: &str) -> PdfPage {
        PdfPage { media_box: mb, crop_box: cb, rotate, text: text.into() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn dict(entries: Vec<(&str, PdfObject)>) -> PdfObject {
        PdfObject::Dict(entries.into_iter().map(|(k, v)| PdfDictEntry { key: k.into(), value: v }).collect())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn entry(k: &str, v: PdfObject) -> PdfDictEntry {
        PdfDictEntry { key: k.into(), value: v }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn base_snapshot() -> PdfSnapshot {
        PdfSnapshot {
            schema: STDIO_PDF17_DOCUMENT_SCHEMA.into(),
            declared_version: "1.7".into(),
            pages: vec![page([0.0, 0.0, 100.0, 100.0], None, 0, "one"), page([0.0, 0.0, 50.0, 50.0], None, 0, "two")],
            info: PdfInfo { title: Some("Base".into()), ..Default::default() },
            objects: vec![PdfIndirectObject { id: oref(1, 0), value: dict(vec![("Type", PdfObject::Name("Catalog".into())), ("Count", PdfObject::Int(3))]) }, PdfIndirectObject { id: oref(2, 0), value: PdfObject::Int(7) }],
            trailer: vec![entry("Root", PdfObject::Ref(oref(1, 0))), entry("Size", PdfObject::Int(2)), entry("Extra", PdfObject::Bool(true))],
        }
    }

    //#region between_roundtrip_law
    #[test]
    fn between_roundtrip_law_value_scalars_and_kind_change() {
        let cases = [
            (PdfObject::Null, PdfObject::Bool(true)),
            (PdfObject::Bool(true), PdfObject::Bool(false)),
            (PdfObject::Int(1), PdfObject::Int(2)),
            (PdfObject::Real(1.5.into()), PdfObject::Real(2.5.into())),
            (PdfObject::Str(b"a".to_vec()), PdfObject::Str(b"b".to_vec())),
            (PdfObject::Name("A".into()), PdfObject::Name("B".into())),
            (PdfObject::Ref(oref(1, 0)), PdfObject::Ref(oref(2, 1))),
            (PdfObject::Int(1), PdfObject::Name("one".into())), // kind change -> Replace
        ];
        for (a, b) in cases {
            match value_diff_between(&a, &b) {
                None => assert_eq!(a, b),
                Some(d) => assert_eq!(apply_value_diff(&d, &a), b, "a={a:?} b={b:?}"),
            }
        }
    }

    #[test]
    fn between_roundtrip_law_nested_array_and_dict() {
        let a = dict(vec![("Kids", PdfObject::Array(vec![PdfObject::Int(1), PdfObject::Int(2)])), ("N", PdfObject::Int(1))]);
        let b = dict(vec![("Kids", PdfObject::Array(vec![PdfObject::Int(1), PdfObject::Int(20), PdfObject::Int(30)])), ("N", PdfObject::Int(2)), ("Extra", PdfObject::Bool(true))]);
        let d_ab = value_diff_between(&a, &b).expect("must differ");
        assert_eq!(apply_value_diff(&d_ab, &a), b);
        let d_ba = value_diff_between(&b, &a).expect("must differ");
        assert_eq!(apply_value_diff(&d_ba, &b), a);
    }

    #[test]
    fn between_roundtrip_law_snapshot_level() {
        let (a, b) = (base_snapshot(), sweep_b());
        assert_eq!(PdfDiff::between(&a, &b).apply(&a).unwrap(), b);
        assert_eq!(PdfDiff::between(&b, &a).apply(&b).unwrap(), a);
    }

    #[test]
    fn between_self_is_empty() {
        let a = base_snapshot();
        assert!(PdfDiff::between(&a, &a).is_empty());
    }
    //#endregion between_roundtrip_law

    //#region inverse_law
    #[test]
    fn inverse_law_diff_level() {
        let (a, b) = (base_snapshot(), sweep_b());
        let d = PdfDiff::between(&a, &b);
        let mid = d.apply(&a).unwrap();
        assert_eq!(mid, b);
        let inv = d.inverse(&a);
        assert_eq!(inv.apply(&mid).unwrap(), a);
    }
    //#endregion inverse_law

    //#region absorb_law (pages / index-keyed)
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn pages_diff(d: PdfPagesDiff) -> PdfDiff {
        PdfDiff { pages: Some(d), ..Default::default() }
    }

    #[test]
    fn absorb_law_pages_insert_then_remove_before() {
        // base=[a,b,c]; d1=Insert(2,f) -> mid=[a,b,f,c]; d2=Remove(0) -> after=[b,f,c].
        let base = PdfSnapshot { pages: vec![page([0.0; 4], None, 0, "a"), page([0.0; 4], None, 0, "b"), page([0.0; 4], None, 0, "c")], ..base_snapshot() };
        let d1 = pages_diff(PdfPagesDiff { added: vec![PdfPageAdded { index: 2, page: page([0.0; 4], None, 0, "f") }], ..Default::default() });
        let d2 = pages_diff(PdfPagesDiff { removed: vec![0], ..Default::default() });
        let sequential = d2.apply(&d1.apply(&base).unwrap()).unwrap();
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).unwrap(), sequential);
        match &combined.pages {
            Some(p) => {
                assert_eq!(p.removed, vec![0]);
                assert_eq!(p.added.len(), 1);
                assert_eq!(p.added[0].index, 1);
            }
            None => panic!("expected pages diff"),
        }
    }

    #[test]
    fn absorb_law_pages_insert_insert_same_index_both_survive() {
        let base = PdfSnapshot { pages: vec![page([0.0; 4], None, 0, "a"), page([0.0; 4], None, 0, "b")], ..base_snapshot() };
        let d1 = pages_diff(PdfPagesDiff { added: vec![PdfPageAdded { index: 2, page: page([0.0; 4], None, 0, "f") }], ..Default::default() });
        let d2 = pages_diff(PdfPagesDiff { added: vec![PdfPageAdded { index: 2, page: page([0.0; 4], None, 0, "g") }], ..Default::default() });
        let sequential = d2.apply(&d1.apply(&base).unwrap()).unwrap();
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).unwrap(), sequential);
        match &combined.pages {
            Some(p) => assert_eq!(p.added.len(), 2, "both inserts must survive"),
            None => panic!("expected pages diff"),
        }
    }

    #[test]
    fn absorb_law_pages_add_then_setfield_patches_added_payload() {
        let base = PdfSnapshot { pages: vec![], ..base_snapshot() };
        let d1 = pages_diff(PdfPagesDiff { added: vec![PdfPageAdded { index: 0, page: page([0.0; 4], None, 0, "x") }], ..Default::default() });
        let d2 = pages_diff(PdfPagesDiff { modified: vec![PdfPageModified { index: 0, diff: PdfPageDiff { rotate: Some(90), ..Default::default() } }], ..Default::default() });
        let sequential = d2.apply(&d1.apply(&base).unwrap()).unwrap();
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).unwrap(), sequential);
        match &combined.pages {
            Some(p) => {
                assert!(p.modified.is_empty(), "the patch must land INSIDE the carried added payload");
                assert_eq!(p.added[0].page.rotate, 90);
            }
            None => panic!("expected pages diff"),
        }
    }

    #[test]
    fn absorb_law_pages_modify_then_remove_drops_pending_patch() {
        let base = PdfSnapshot { pages: vec![page([0.0; 4], None, 0, "a"), page([0.0; 4], None, 0, "b")], ..base_snapshot() };
        let d1 = pages_diff(PdfPagesDiff { modified: vec![PdfPageModified { index: 0, diff: PdfPageDiff { rotate: Some(180), ..Default::default() } }], ..Default::default() });
        let d2 = pages_diff(PdfPagesDiff { removed: vec![0], ..Default::default() });
        let sequential = d2.apply(&d1.apply(&base).unwrap()).unwrap();
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).unwrap(), sequential);
        match &combined.pages {
            Some(p) => {
                assert_eq!(p.removed, vec![0]);
                assert!(p.modified.is_empty());
            }
            None => panic!("expected pages diff"),
        }
    }

    #[test]
    fn absorb_law_pages_associativity() {
        let s0 = PdfSnapshot { pages: vec![page([0.0; 4], None, 0, "1"), page([0.0; 4], None, 0, "2"), page([0.0; 4], None, 0, "3")], ..base_snapshot() };
        let s1 = PdfSnapshot { pages: vec![page([0.0; 4], None, 0, "1"), page([0.0; 4], None, 0, "9"), page([0.0; 4], None, 0, "3")], ..base_snapshot() };
        let s2 = PdfSnapshot { pages: vec![page([0.0; 4], None, 0, "9"), page([0.0; 4], None, 0, "3"), page([0.0; 4], None, 0, "4")], ..base_snapshot() };
        let s3 = PdfSnapshot { pages: vec![page([0.0; 4], None, 0, "9"), page([0.0; 4], None, 0, "4")], ..base_snapshot() };
        let d1 = PdfDiff::between(&s0, &s1);
        let d2 = PdfDiff::between(&s1, &s2);
        let d3 = PdfDiff::between(&s2, &s3);
        let mut left = d1.clone();
        left.absorb(d2.clone());
        left.absorb(d3.clone());
        let mut right_tail = d2.clone();
        right_tail.absorb(d3.clone());
        let mut right = d1.clone();
        right.absorb(right_tail);
        assert_eq!(left.apply(&s0).unwrap(), s3);
        assert_eq!(right.apply(&s0).unwrap(), s3);
        assert_eq!(left, right);
    }
    //#endregion absorb_law (pages / index-keyed)

    //#region absorb_law (objects / id-keyed)
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn objects_diff(d: PdfObjectsDiff) -> PdfDiff {
        PdfDiff { objects: Some(d), ..Default::default() }
    }

    #[test]
    fn absorb_law_objects_add_then_setfield_patches_added_payload() {
        let base = PdfSnapshot { objects: vec![], ..base_snapshot() };
        let d1 = objects_diff(PdfObjectsDiff { added: vec![PdfObjectAdded { index: 0, id: oref(5, 0), value: dict(vec![("X", PdfObject::Int(1))]) }], ..Default::default() });
        let d2 = objects_diff(PdfObjectsDiff {
            modified: vec![PdfObjectModified { id: oref(5, 0), diff: PdfValueDiff::Dict { diff: PdfDictDiff { added: vec![PdfDictAdded { index: 1, key: "Y".into(), item: PdfObject::Int(2) }], ..Default::default() } } }],
            ..Default::default()
        });
        let sequential = d2.apply(&d1.apply(&base).unwrap()).unwrap();
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).unwrap(), sequential);
        match &combined.objects {
            Some(o) => {
                assert!(o.modified.is_empty());
                assert_eq!(o.added[0].value, dict(vec![("X", PdfObject::Int(1)), ("Y", PdfObject::Int(2))]));
            }
            None => panic!("expected objects diff"),
        }
    }

    #[test]
    fn absorb_law_objects_modify_then_remove_drops_pending_patch() {
        let base = PdfSnapshot { objects: vec![PdfIndirectObject { id: oref(1, 0), value: PdfObject::Int(1) }, PdfIndirectObject { id: oref(2, 0), value: PdfObject::Int(2) }], ..base_snapshot() };
        let d1 = objects_diff(PdfObjectsDiff { modified: vec![PdfObjectModified { id: oref(1, 0), diff: PdfValueDiff::Int { value: 9 } }], ..Default::default() });
        let d2 = objects_diff(PdfObjectsDiff { removed: vec![oref(1, 0)], ..Default::default() });
        let sequential = d2.apply(&d1.apply(&base).unwrap()).unwrap();
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).unwrap(), sequential);
        match &combined.objects {
            Some(o) => {
                assert_eq!(o.removed, vec![oref(1, 0)]);
                assert!(o.modified.is_empty());
            }
            None => panic!("expected objects diff"),
        }
    }

    #[test]
    fn absorb_law_objects_two_independent_inserts_both_survive() {
        let base = PdfSnapshot { objects: vec![], ..base_snapshot() };
        let d1 = objects_diff(PdfObjectsDiff { added: vec![PdfObjectAdded { index: 0, id: oref(5, 0), value: PdfObject::Int(1) }], ..Default::default() });
        let d2 = objects_diff(PdfObjectsDiff { added: vec![PdfObjectAdded { index: 1, id: oref(6, 0), value: PdfObject::Int(2) }], ..Default::default() });
        let sequential = d2.apply(&d1.apply(&base).unwrap()).unwrap();
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).unwrap(), sequential);
        match &combined.objects {
            Some(o) => assert_eq!(o.added.len(), 2),
            None => panic!("expected objects diff"),
        }
    }

    #[test]
    fn absorb_law_objects_associativity() {
        let s0 = PdfSnapshot { objects: vec![PdfIndirectObject { id: oref(1, 0), value: PdfObject::Int(1) }], ..base_snapshot() };
        let s1 = PdfSnapshot { objects: vec![PdfIndirectObject { id: oref(1, 0), value: PdfObject::Int(1) }, PdfIndirectObject { id: oref(2, 0), value: PdfObject::Int(2) }], ..base_snapshot() };
        let s2 = PdfSnapshot { objects: vec![PdfIndirectObject { id: oref(1, 0), value: PdfObject::Int(9) }, PdfIndirectObject { id: oref(2, 0), value: PdfObject::Int(2) }], ..base_snapshot() };
        let s3 = PdfSnapshot { objects: vec![PdfIndirectObject { id: oref(2, 0), value: PdfObject::Int(2) }, PdfIndirectObject { id: oref(3, 0), value: PdfObject::Int(3) }], ..base_snapshot() };
        let d1 = PdfDiff::between(&s0, &s1);
        let d2 = PdfDiff::between(&s1, &s2);
        let d3 = PdfDiff::between(&s2, &s3);
        let mut left = d1.clone();
        left.absorb(d2.clone());
        left.absorb(d3.clone());
        let mut right_tail = d2.clone();
        right_tail.absorb(d3.clone());
        let mut right = d1.clone();
        right.absorb(right_tail);
        assert_eq!(left.apply(&s0).unwrap(), s3);
        assert_eq!(right.apply(&s0).unwrap(), s3);
        assert_eq!(left, right);
    }
    //#endregion absorb_law (objects / id-keyed)

    //#region absorb_law (trailer / name-keyed)
    #[test]
    fn absorb_law_trailer_add_then_setfield_patches_added_payload() {
        let base = PdfSnapshot { trailer: vec![], ..base_snapshot() };
        let d1 = PdfDiff { trailer: Some(PdfDictDiff { added: vec![PdfDictAdded { index: 0, key: "Config".into(), item: dict(vec![]) }], ..Default::default() }), ..Default::default() };
        let d2 = PdfDiff {
            trailer: Some(PdfDictDiff {
                modified: vec![PdfDictModified { key: "Config".into(), diff: PdfValueDiff::Dict { diff: PdfDictDiff { added: vec![PdfDictAdded { index: 0, key: "X".into(), item: PdfObject::Int(5) }], ..Default::default() } } }],
                ..Default::default()
            }),
            ..Default::default()
        };
        let sequential = d2.apply(&d1.apply(&base).unwrap()).unwrap();
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).unwrap(), sequential);
        match &combined.trailer {
            Some(t) => {
                assert!(t.modified.is_empty());
                assert_eq!(t.added[0].item, dict(vec![("X", PdfObject::Int(5))]));
            }
            None => panic!("expected trailer diff"),
        }
    }

    #[test]
    fn absorb_law_trailer_modify_then_remove_drops_pending_patch() {
        let base = PdfSnapshot { trailer: vec![entry("A", PdfObject::Int(1)), entry("B", PdfObject::Int(2))], ..base_snapshot() };
        let d1 = PdfDiff { trailer: Some(PdfDictDiff { modified: vec![PdfDictModified { key: "A".into(), diff: PdfValueDiff::Int { value: 9 } }], ..Default::default() }), ..Default::default() };
        let d2 = PdfDiff { trailer: Some(PdfDictDiff { removed: vec!["A".into()], ..Default::default() }), ..Default::default() };
        let sequential = d2.apply(&d1.apply(&base).unwrap()).unwrap();
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).unwrap(), sequential);
        match &combined.trailer {
            Some(t) => {
                assert_eq!(t.removed, vec!["A".to_string()]);
                assert!(t.modified.is_empty());
            }
            None => panic!("expected trailer diff"),
        }
    }
    //#endregion absorb_law (trailer / name-keyed)

    //#region field_sweep
    /// 📏 `sweep_a`/`sweep_b` differ in EVERY mutable field. `pages` is DEPTH-ASYMMETRIC on
    /// purpose (`a` has 3, `b` has 2) so a SINGLE `between(a,b)` call cannot produce both
    /// `removed` and `added` on that positional collection (documented structural trap) --
    /// `removed` shows up in `between(a,b)`, `added` in `between(b,a)`. `objects`/`trailer` are
    /// id-/name-keyed so both directions freely carry removed+modified+added simultaneously.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sweep_a() -> PdfSnapshot {
        base_snapshot()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sweep_b() -> PdfSnapshot {
        PdfSnapshot {
            schema: STDIO_PDF17_DOCUMENT_SCHEMA.into(),
            declared_version: "1.4".into(), // scalar change
            pages: vec![
                page([0.0, 0.0, 200.0, 200.0], Some([1.0, 1.0, 50.0, 50.0]), 90, "ONE"), // modified: every field, crop_box None->Some
            ], // base had 2 pages -> this direction's tail (index1) is a `removed`
            info: PdfInfo { title: Some("Changed".into()), author: Some("Ueli".into()), ..Default::default() }, // weak whole-value replace
            objects: vec![
                PdfIndirectObject { id: oref(1, 0), value: dict(vec![("Type", PdfObject::Name("Catalog".into())), ("Count", PdfObject::Int(4)), ("New", PdfObject::Bool(false))]) }, // modified: kept+modified+added inside, "Type" kept, "Count" changed, "New" added (base's implicit lack of "New")
                PdfIndirectObject { id: oref(3, 0), value: PdfObject::Name("Added".into()) },                                                                                        // added (base's obj id=2 is absent here -> removed)
            ],
            trailer: vec![entry("Root", PdfObject::Ref(oref(1, 0))), entry("Size", PdfObject::Int(3)), entry("Prev", PdfObject::Int(100))], // modified Size, added Prev
        }
    }

    #[test]
    fn field_sweep_between_roundtrips_both_directions() {
        let (a, b) = (sweep_a(), sweep_b());
        assert_eq!(PdfDiff::between(&a, &b).apply(&a).unwrap(), b);
        assert_eq!(PdfDiff::between(&b, &a).apply(&b).unwrap(), a);
        assert!(PdfDiff::between(&a, &a).is_empty());
    }

    #[test]
    fn field_sweep_every_field_present_in_diff() {
        let (a, b) = (sweep_a(), sweep_b());
        let ab = PdfDiff::between(&a, &b);
        assert!(ab.declared_version.is_some(), "declaredVersion must be present");
        assert!(ab.info.is_some(), "info must be present (weak whole-value replace)");

        let pages = ab.pages.as_ref().expect("pages diff must be present");
        assert!(!pages.modified.is_empty(), "expected a modified page (every field changed)");
        assert!(!pages.removed.is_empty(), "a has more pages than b -> removed tail expected in between(a,b)");
        let ba_pages = PdfDiff::between(&b, &a).pages.expect("pages diff must be present");
        assert!(!ba_pages.added.is_empty(), "b has fewer pages than a -> added tail expected in between(b,a)");
        let pmod = &pages.modified[0].diff;
        assert!(pmod.media_box.is_some() && pmod.crop_box.is_some() && pmod.rotate.is_some() && pmod.text.is_some(), "every PdfPageDiff field must be exercised: {pmod:?}");
        assert_eq!(pmod.crop_box, Some(Some([1.0, 1.0, 50.0, 50.0])), "crop_box tri-state None->Some must round-trip");

        let objects = ab.objects.as_ref().expect("objects diff must be present");
        assert!(!objects.removed.is_empty(), "obj id=2 only in a -> removed");
        assert!(!objects.modified.is_empty(), "obj id=1 changed in both -> modified");
        assert!(!objects.added.is_empty(), "obj id=3 only in b -> added");
        match &objects.modified[0].diff {
            PdfValueDiff::Dict { diff } => {
                assert!(!diff.modified.is_empty(), "Count field must show as a nested Dict-modified entry");
                assert!(!diff.added.is_empty(), "New key must show as a nested Dict-added entry");
            }
            other => panic!("expected a recursive Dict value diff for obj id=1, got {other:?}"),
        }

        let trailer = ab.trailer.as_ref().expect("trailer diff must be present");
        assert!(!trailer.removed.is_empty(), "Extra must be removed");
        assert!(!trailer.modified.is_empty(), "Size must be modified");
        assert!(!trailer.added.is_empty(), "Prev must be added");
    }
    //#endregion field_sweep
}
//#endregion 🧪️Tests
