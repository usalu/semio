//! 🔺️ DxfDiff — handcrafted sparse diff. Ticket
//! `26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION`: replaces the old
//! `DxfDiff{snapshot: Option<DxfSnapshot>}` full-replace template with real per-field patches:
//! a name-keyed triple for `header_vars`, one name-keyed triple per typed table kind
//! (`layers`/`styles`/`linetypes`), an index-keyed triple for `blocks` (each block's own nested
//! `entities` reuses the SAME entity-diff machinery as the top-level collection), and an
//! index-keyed triple for the top-level `entities` — each entity's own diff is `Replace{entity}`
//! when the entity KIND changes at that index, or a kind-specific field diff when it doesn't
//! (the plan's json/xml "Replace on kind change" rule, applied to an enum collection element).
//!
//! Two small intra-file generic cores do the position/name algebra ONCE (mirrors `stdio.obj`'s
//! `ObjIndexElem`/`generic_apply`/`generic_between`/`generic_absorb_pair` and its name-keyed
//! `HasFaces`-style sibling): `DxfIndexElem` for the two index-keyed collections
//! (`entities`/`blocks`, both reused verbatim for a block's own nested `entities`), and
//! `DxfNamedElem` for the four name-keyed collections (`header_vars`/`layers`/`styles`/
//! `linetypes`). Every PUBLIC diff type below stays a fully concrete, per-artifact named type —
//! this is pure code reuse WITHIN this one file, never exported.
//!
//! 🧪️ F6: `protocol::DiffCodec` for `DxfDiff` is **hand-rolled** (§`HandcraftedDiffCodec` below).
//! `#[derive(dsl::DslDiff)]` confirmed rejected by a real `cargo check`: `DxfEntityDiff` (this
//! file's own enum, `Replace{entity:DxfEntity}` plus one variant per typed entity kind) has no
//! `DslField` impl — `error[E0277]: the trait bound 'DxfEntityDiff: DslField' is not satisfied`
//! at `DxfEntityModified.diff: DxfEntityDiff` (recon report §3a — a data-carrying enum has no
//! `DslField` source, derivable or otherwise). Zero `Option<Option<_>>` tri-state anywhere in this
//! diff tree (3b does not apply) — this is the "enum-only" hand-roll case, same shape as
//! `stdio.json`/`stdio.svg`'s `SvgNodeDiff`. Grammar follows §5's established convention (hex for
//! strings/bytes, positional `[f1,f2,...]` tuples for structs, single-uppercase-letter tag prefix
//! for enums, `name{[removed];[modified];[added]}` for collection triples) — see
//! `f6-recon-report.md` in this ticket folder.

use std::collections::{BTreeSet, HashSet};

use crate::artifacts::dxf::schema::snapshot::{DxfBlock, DxfEntity, DxfHeaderVar, DxfLayer, DxfLinetype, DxfOtherTable, DxfStyle, DxfTables, DxfTag, DxfValue, DxfVertex};
use crate::artifacts::dxf::DxfSnapshot;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️IndexCollectionCore
/// 🧮 Per-item sparse-diff behavior shared by the two index-keyed collections (`DxfEntity`,
/// `DxfBlock` — the latter's own nested `entities` field reuses `DxfEntity`'s impl directly).
trait DxfIndexElem: Clone + PartialEq {
    type Diff: Clone + PartialEq;
    async fn diff_is_empty(d: &Self::Diff) -> bool;
    async fn diff_between(a: &Self, b: &Self) -> Self::Diff;
    async fn diff_apply(d: &Self::Diff, item: &mut Self);
    async fn diff_absorb(base: &mut Self::Diff, other: Self::Diff);
}

/// ▶️ Applies a `(removed, modified, added)` triple to a base array — modified on BASE
/// positions first, then removed descending, then added ascending clamped to `min(index,len)`
/// (recipe's normative apply order).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn generic_apply<T: DxfIndexElem>(base: &[T], removed: &[usize], modified: &[(usize, T::Diff)], added: &[(usize, T)]) -> Vec<T> {
    let mut items = base.to_vec();
    for (idx, d) in modified {
        T::diff_apply(d, &mut items[*idx]);
    }
    let mut removed_desc = removed.to_vec();
    removed_desc.sort_unstable_by(|a, b| b.cmp(a));
    for idx in removed_desc {
        items.remove(idx);
    }
    let mut adds: Vec<&(usize, T)> = added.iter().collect();
    adds.sort_by_key(|(i, _)| *i);
    for (idx, item) in adds {
        items.insert(*idx, item.clone());
    }
    items
}

/// 🧭️ Pairwise-by-position state delta (recipe's "index keys pairwise by position" rule).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn generic_between<T: DxfIndexElem>(base: &[T], other: &[T]) -> (Vec<usize>, Vec<(usize, T::Diff)>, Vec<(usize, T)>) {
    let min_len = base.len().min(other.len());
    let mut modified = Vec::new();
    for i in 0..min_len {
        let d = T::diff_between(&base[i], &other[i]);
        if !T::diff_is_empty(&d) {
            modified.push((i, d));
        }
    }
    let removed: Vec<usize> = if base.len() > other.len() { (other.len()..base.len()).collect() } else { Vec::new() };
    let added: Vec<(usize, T)> = if other.len() > base.len() { (base.len()..other.len()).map(|i| (i, other[i].clone())).collect() } else { Vec::new() };
    (removed, modified, added)
}

/// 🏷️ Structural, base-free label used only inside [`generic_absorb_pair`] to simulate the
/// two-step position transform (base→mid via `d1`, mid→after via `d2`) — mirrors `stdio.txt`'s
/// proven `Lbl`/`simulate_labels`/`absorb_pair` shape (also mirrored verbatim in `stdio.obj`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Lbl {
    Base(usize),
    Added1(usize),
    Added2(usize),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn simulate_labels(labels: Vec<Lbl>, removed: &[usize], added: &[(usize, Lbl)]) -> Vec<Lbl> {
    let removed_set: HashSet<usize> = removed.iter().copied().collect();
    let mut survivors: Vec<Lbl> = labels.into_iter().enumerate().filter(|(i, _)| !removed_set.contains(i)).map(|(_, l)| l).collect();
    let mut added_sorted = added.to_vec();
    added_sorted.sort_by_key(|(idx, _)| *idx);
    for (idx, label) in added_sorted {
        let pos = idx.min(survivors.len());
        survivors.insert(pos, label);
    }
    survivors
}

/// ➕️ Absorbs `d1` (base→mid) then `d2` (mid→after) into a single base→after triple.
#[allow(clippy::type_complexity)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn generic_absorb_pair<T: DxfIndexElem>(
    d1_removed: &[usize],
    d1_modified: &[(usize, T::Diff)],
    d1_added: &[(usize, T)],
    d2_removed: &[usize],
    d2_modified: &[(usize, T::Diff)],
    d2_added: &[(usize, T)],
) -> (Vec<usize>, Vec<(usize, T::Diff)>, Vec<(usize, T)>) {
    use std::collections::HashMap;
    let max_ref =
        d1_removed.iter().copied().chain(d1_modified.iter().map(|(i, _)| *i)).chain(d1_added.iter().map(|(i, _)| *i)).chain(d2_removed.iter().copied()).chain(d2_modified.iter().map(|(i, _)| *i)).chain(d2_added.iter().map(|(i, _)| *i)).max();
    let l1 = max_ref.map(|m| m + 2).unwrap_or(0);

    let base_labels: Vec<Lbl> = (0..l1).map(Lbl::Base).collect();
    let d1_added_lbl: Vec<(usize, Lbl)> = d1_added.iter().enumerate().map(|(j, (idx, _))| (*idx, Lbl::Added1(j))).collect();
    let mut mid_labels = simulate_labels(base_labels, d1_removed, &d1_added_lbl);

    let mut mid_pos_of_base: HashMap<usize, usize> = HashMap::new();
    let mut mid_pos_of_added1: HashMap<usize, usize> = HashMap::new();
    for (pos, l) in mid_labels.iter().enumerate() {
        match l {
            Lbl::Base(i) => {
                mid_pos_of_base.insert(*i, pos);
            }
            Lbl::Added1(j) => {
                mid_pos_of_added1.insert(*j, pos);
            }
            Lbl::Added2(_) => {}
        }
    }
    while mid_labels.len() < l1 {
        mid_labels.push(Lbl::Base(usize::MAX));
    }

    let d2_added_lbl: Vec<(usize, Lbl)> = d2_added.iter().enumerate().map(|(k, (idx, _))| (*idx, Lbl::Added2(k))).collect();
    let after_labels = simulate_labels(mid_labels.clone(), d2_removed, &d2_added_lbl);

    let d2_modified_at: HashMap<usize, &T::Diff> = d2_modified.iter().map(|(i, d)| (*i, d)).collect();
    let d1_modified_at: HashMap<usize, &T::Diff> = d1_modified.iter().map(|(i, d)| (*i, d)).collect();

    let mut present_base: HashSet<usize> = HashSet::new();
    let mut modified: Vec<(usize, T::Diff)> = Vec::new();
    let mut added: Vec<(usize, T)> = Vec::new();

    for (pos, l) in after_labels.into_iter().enumerate() {
        match l {
            Lbl::Base(i) if i != usize::MAX => {
                present_base.insert(i);
                let mid_pos = mid_pos_of_base.get(&i).copied();
                let d2d = mid_pos.and_then(|m| d2_modified_at.get(&m).copied()).cloned();
                let d1d = d1_modified_at.get(&i).copied().cloned();
                let combined = match (d1d, d2d) {
                    (None, None) => None,
                    (Some(a), None) => Some(a),
                    (None, Some(b)) => Some(b),
                    (Some(mut a), Some(b)) => {
                        T::diff_absorb(&mut a, b);
                        Some(a)
                    }
                };
                if let Some(d) = combined {
                    if !T::diff_is_empty(&d) {
                        modified.push((i, d));
                    }
                }
            }
            Lbl::Base(_) => {}
            Lbl::Added1(j) => {
                let mid_pos = mid_pos_of_added1.get(&j).copied();
                let (_, base_item) = &d1_added[j];
                let mut item = base_item.clone();
                if let Some(m) = mid_pos {
                    if let Some(d2d) = d2_modified_at.get(&m) {
                        T::diff_apply(d2d, &mut item);
                    }
                }
                added.push((pos, item));
            }
            Lbl::Added2(k) => {
                let (_, item) = &d2_added[k];
                added.push((pos, item.clone()));
            }
        }
    }

    let removed: Vec<usize> = (0..l1).filter(|i| !present_base.contains(i)).collect();
    (removed, modified, added)
}
//#endregion 🔖️IndexCollectionCore

//#region 🔖️NamedCollectionCore
/// 🧮 Per-item sparse-diff behavior shared by the four name-keyed collections (`DxfHeaderVar`,
/// `DxfLayer`, `DxfStyle`, `DxfLinetype`) — same shape as `stdio.obj`'s `HasFaces`/group pattern,
/// generalized into a trait since there are four distinct named types here, not two structurally
/// identical ones. No rename tracking: nothing in this artifact's mutation vocabulary renames a
/// header var/layer/style/linetype in place (matches `stdio.obj`'s groups/objects, not `stdio.zip`'s
/// entries).
trait DxfNamedElem: Clone + PartialEq {
    type Diff: Clone + PartialEq + Default;
    async fn key(&self) -> &str;
    async fn diff_between(a: &Self, b: &Self) -> Self::Diff;
    async fn diff_apply(d: &Self::Diff, item: &mut Self);
    async fn diff_absorb(base: &mut Self::Diff, other: Self::Diff);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn named_apply<T: DxfNamedElem>(base: &[T], removed: &[String], modified: &[(String, T::Diff)], added: &[(usize, T)]) -> Vec<T> {
    let mut items = base.to_vec();
    for (key, d) in modified {
        for item in &mut items {
            if item.key() == key {
                T::diff_apply(d, item);
            }
        }
    }
    let removed_set: HashSet<&str> = removed.iter().map(String::as_str).collect();
    items.retain(|it| !removed_set.contains(&it.key()));
    let mut adds: Vec<&(usize, T)> = added.iter().collect();
    adds.sort_by_key(|(i, _)| *i);
    for (idx, item) in adds {
        items.insert(*idx, item.clone());
    }
    items
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn named_between<T: DxfNamedElem>(base: &[T], other: &[T]) -> (Vec<String>, Vec<(String, T::Diff)>, Vec<(usize, T)>) {
    let base_keys: HashSet<&str> = base.iter().map(|t| t.key()).collect();
    let other_keys: HashSet<&str> = other.iter().map(|t| t.key()).collect();
    let removed: Vec<String> = base.iter().filter(|t| !other_keys.contains(&t.key())).map(|t| t.key().to_string()).collect();
    let mut modified = Vec::new();
    for bt in base {
        if let Some(ot) = other.iter().find(|o| o.key() == bt.key()) {
            let d = T::diff_between(bt, ot);
            if d != T::Diff::default() {
                modified.push((bt.key().to_string(), d));
            }
        }
    }
    let added: Vec<(usize, T)> = other.iter().enumerate().filter(|(_, t)| !base_keys.contains(&t.key())).map(|(i, t)| (i, t.clone())).collect();
    (removed, modified, added)
}

#[allow(clippy::type_complexity)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn named_absorb_pair<T: DxfNamedElem>(
    d1_removed: &[String],
    d1_modified: &[(String, T::Diff)],
    d1_added: &[(usize, T)],
    d2_removed: &[String],
    d2_modified: &[(String, T::Diff)],
    d2_added: &[(usize, T)],
) -> (Vec<String>, Vec<(String, T::Diff)>, Vec<(usize, T)>) {
    let added_keys: HashSet<String> = d1_added.iter().map(|(_, t)| t.key().to_string()).collect();
    let mut merged_removed: Vec<String> = d1_removed.to_vec();
    let mut annihilated: HashSet<String> = HashSet::new();
    for key in d2_removed {
        if added_keys.contains(key) {
            annihilated.insert(key.clone());
        } else if !merged_removed.contains(key) {
            merged_removed.push(key.clone());
        }
    }
    let mut merged_modified: Vec<(String, T::Diff)> = d1_modified.iter().filter(|(k, _)| !merged_removed.contains(k)).cloned().collect();
    let mut merged_added: Vec<(usize, T)> = d1_added.iter().filter(|(_, t)| !annihilated.contains(&t.key())).cloned().collect();

    for (key, d2d) in d2_modified {
        if added_keys.contains(key) {
            if annihilated.contains(key) {
                continue;
            }
            if let Some((_, item)) = merged_added.iter_mut().find(|(_, t)| t.key() == key) {
                T::diff_apply(d2d, item);
            }
        } else {
            if merged_removed.contains(key) {
                continue;
            }
            if let Some((_, existing)) = merged_modified.iter_mut().find(|(k, _)| k == key) {
                T::diff_absorb(existing, d2d.clone());
            } else {
                merged_modified.push((key.clone(), d2d.clone()));
            }
        }
    }
    merged_added.extend(d2_added.iter().cloned());
    (merged_removed, merged_modified, merged_added)
}
//#endregion 🔖️NamedCollectionCore

//#region 🔖️HeaderVarDiff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfHeaderVarDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_code: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<DxfValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_group_codes: Option<Vec<(i32, DxfValue)>>,
}
impl DxfNamedElem for DxfHeaderVar {
    type Diff = DxfHeaderVarDiff;
    async fn key(&self) -> &str {
        &self.name
    }
    async fn diff_between(a: &Self, b: &Self) -> Self::Diff {
        DxfHeaderVarDiff {
            group_code: (a.group_code != b.group_code).then_some(b.group_code),
            value: (a.value != b.value).then(|| b.value.clone()),
            extra_group_codes: (a.extra_group_codes != b.extra_group_codes).then(|| b.extra_group_codes.clone()),
        }
    }
    async fn diff_apply(d: &Self::Diff, item: &mut Self) {
        if let Some(v) = d.group_code {
            item.group_code = v;
        }
        if let Some(v) = &d.value {
            item.value = v.clone();
        }
        if let Some(v) = &d.extra_group_codes {
            item.extra_group_codes = v.clone();
        }
    }
    async fn diff_absorb(base: &mut Self::Diff, other: Self::Diff) {
        if other.group_code.is_some() {
            base.group_code = other.group_code;
        }
        if other.value.is_some() {
            base.value = other.value;
        }
        if other.extra_group_codes.is_some() {
            base.extra_group_codes = other.extra_group_codes;
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfHeaderVarModified {
    pub name: String,
    pub diff: DxfHeaderVarDiff,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfHeaderVarAdded {
    pub index: usize,
    pub header_var: DxfHeaderVar,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfHeaderVarsDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<DxfHeaderVarModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<DxfHeaderVarAdded>,
}
impl DxfHeaderVarsDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn apply(&self, base: &[DxfHeaderVar]) -> Vec<DxfHeaderVar> {
        let modified: Vec<(String, DxfHeaderVarDiff)> = self.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
        let added: Vec<(usize, DxfHeaderVar)> = self.added.iter().map(|a| (a.index, a.header_var.clone())).collect();
        named_apply(base, &self.removed, &modified, &added)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn between(base: &[DxfHeaderVar], other: &[DxfHeaderVar]) -> Option<Self> {
        let (removed, modified, added) = named_between(base, other);
        let d = Self { removed, modified: modified.into_iter().map(|(name, diff)| DxfHeaderVarModified { name, diff }).collect(), added: added.into_iter().map(|(index, header_var)| DxfHeaderVarAdded { index, header_var }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn absorb(d1: Self, d2: Self) -> Option<Self> {
        let d1m: Vec<(String, DxfHeaderVarDiff)> = d1.modified.into_iter().map(|m| (m.name, m.diff)).collect();
        let d1a: Vec<(usize, DxfHeaderVar)> = d1.added.into_iter().map(|a| (a.index, a.header_var)).collect();
        let d2m: Vec<(String, DxfHeaderVarDiff)> = d2.modified.into_iter().map(|m| (m.name, m.diff)).collect();
        let d2a: Vec<(usize, DxfHeaderVar)> = d2.added.into_iter().map(|a| (a.index, a.header_var)).collect();
        let (removed, modified, added) = named_absorb_pair::<DxfHeaderVar>(&d1.removed, &d1m, &d1a, &d2.removed, &d2m, &d2a);
        let d = Self { removed, modified: modified.into_iter().map(|(name, diff)| DxfHeaderVarModified { name, diff }).collect(), added: added.into_iter().map(|(index, header_var)| DxfHeaderVarAdded { index, header_var }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
}
//#endregion 🔖️HeaderVarDiff

//#region 🔖️LayerDiff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfLayerDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linetype: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}
impl DxfNamedElem for DxfLayer {
    type Diff = DxfLayerDiff;
    async fn key(&self) -> &str {
        &self.name
    }
    async fn diff_between(a: &Self, b: &Self) -> Self::Diff {
        DxfLayerDiff {
            color: (a.color != b.color).then_some(b.color),
            linetype: (a.linetype != b.linetype).then(|| b.linetype.clone()),
            flags: (a.flags != b.flags).then_some(b.flags),
            unknown_group_codes: (a.unknown_group_codes != b.unknown_group_codes).then(|| b.unknown_group_codes.clone()),
        }
    }
    async fn diff_apply(d: &Self::Diff, item: &mut Self) {
        if let Some(v) = d.color {
            item.color = v;
        }
        if let Some(v) = &d.linetype {
            item.linetype = v.clone();
        }
        if let Some(v) = d.flags {
            item.flags = v;
        }
        if let Some(v) = &d.unknown_group_codes {
            item.unknown_group_codes = v.clone();
        }
    }
    async fn diff_absorb(base: &mut Self::Diff, other: Self::Diff) {
        if other.color.is_some() {
            base.color = other.color;
        }
        if other.linetype.is_some() {
            base.linetype = other.linetype;
        }
        if other.flags.is_some() {
            base.flags = other.flags;
        }
        if other.unknown_group_codes.is_some() {
            base.unknown_group_codes = other.unknown_group_codes;
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfLayerModified {
    pub name: String,
    pub diff: DxfLayerDiff,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfLayerAdded {
    pub index: usize,
    pub layer: DxfLayer,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfLayersDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<DxfLayerModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<DxfLayerAdded>,
}
impl DxfLayersDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn apply(&self, base: &[DxfLayer]) -> Vec<DxfLayer> {
        let modified: Vec<(String, DxfLayerDiff)> = self.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
        let added: Vec<(usize, DxfLayer)> = self.added.iter().map(|a| (a.index, a.layer.clone())).collect();
        named_apply(base, &self.removed, &modified, &added)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn between(base: &[DxfLayer], other: &[DxfLayer]) -> Option<Self> {
        let (removed, modified, added) = named_between(base, other);
        let d = Self { removed, modified: modified.into_iter().map(|(name, diff)| DxfLayerModified { name, diff }).collect(), added: added.into_iter().map(|(index, layer)| DxfLayerAdded { index, layer }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn absorb(d1: Self, d2: Self) -> Option<Self> {
        let d1m: Vec<(String, DxfLayerDiff)> = d1.modified.into_iter().map(|m| (m.name, m.diff)).collect();
        let d1a: Vec<(usize, DxfLayer)> = d1.added.into_iter().map(|a| (a.index, a.layer)).collect();
        let d2m: Vec<(String, DxfLayerDiff)> = d2.modified.into_iter().map(|m| (m.name, m.diff)).collect();
        let d2a: Vec<(usize, DxfLayer)> = d2.added.into_iter().map(|a| (a.index, a.layer)).collect();
        let (removed, modified, added) = named_absorb_pair::<DxfLayer>(&d1.removed, &d1m, &d1a, &d2.removed, &d2m, &d2a);
        let d = Self { removed, modified: modified.into_iter().map(|(name, diff)| DxfLayerModified { name, diff }).collect(), added: added.into_iter().map(|(index, layer)| DxfLayerAdded { index, layer }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
}
//#endregion 🔖️LayerDiff

//#region 🔖️StyleDiff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfStyleDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}
impl DxfNamedElem for DxfStyle {
    type Diff = DxfStyleDiff;
    async fn key(&self) -> &str {
        &self.name
    }
    async fn diff_between(a: &Self, b: &Self) -> Self::Diff {
        DxfStyleDiff {
            flags: (a.flags != b.flags).then_some(b.flags),
            font_name: (a.font_name != b.font_name).then(|| b.font_name.clone()),
            unknown_group_codes: (a.unknown_group_codes != b.unknown_group_codes).then(|| b.unknown_group_codes.clone()),
        }
    }
    async fn diff_apply(d: &Self::Diff, item: &mut Self) {
        if let Some(v) = d.flags {
            item.flags = v;
        }
        if let Some(v) = &d.font_name {
            item.font_name = v.clone();
        }
        if let Some(v) = &d.unknown_group_codes {
            item.unknown_group_codes = v.clone();
        }
    }
    async fn diff_absorb(base: &mut Self::Diff, other: Self::Diff) {
        if other.flags.is_some() {
            base.flags = other.flags;
        }
        if other.font_name.is_some() {
            base.font_name = other.font_name;
        }
        if other.unknown_group_codes.is_some() {
            base.unknown_group_codes = other.unknown_group_codes;
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfStyleModified {
    pub name: String,
    pub diff: DxfStyleDiff,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfStyleAdded {
    pub index: usize,
    pub style: DxfStyle,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfStylesDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<DxfStyleModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<DxfStyleAdded>,
}
impl DxfStylesDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn apply(&self, base: &[DxfStyle]) -> Vec<DxfStyle> {
        let modified: Vec<(String, DxfStyleDiff)> = self.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
        let added: Vec<(usize, DxfStyle)> = self.added.iter().map(|a| (a.index, a.style.clone())).collect();
        named_apply(base, &self.removed, &modified, &added)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn between(base: &[DxfStyle], other: &[DxfStyle]) -> Option<Self> {
        let (removed, modified, added) = named_between(base, other);
        let d = Self { removed, modified: modified.into_iter().map(|(name, diff)| DxfStyleModified { name, diff }).collect(), added: added.into_iter().map(|(index, style)| DxfStyleAdded { index, style }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn absorb(d1: Self, d2: Self) -> Option<Self> {
        let d1m: Vec<(String, DxfStyleDiff)> = d1.modified.into_iter().map(|m| (m.name, m.diff)).collect();
        let d1a: Vec<(usize, DxfStyle)> = d1.added.into_iter().map(|a| (a.index, a.style)).collect();
        let d2m: Vec<(String, DxfStyleDiff)> = d2.modified.into_iter().map(|m| (m.name, m.diff)).collect();
        let d2a: Vec<(usize, DxfStyle)> = d2.added.into_iter().map(|a| (a.index, a.style)).collect();
        let (removed, modified, added) = named_absorb_pair::<DxfStyle>(&d1.removed, &d1m, &d1a, &d2.removed, &d2m, &d2a);
        let d = Self { removed, modified: modified.into_iter().map(|(name, diff)| DxfStyleModified { name, diff }).collect(), added: added.into_iter().map(|(index, style)| DxfStyleAdded { index, style }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
}
//#endregion 🔖️StyleDiff

//#region 🔖️LinetypeDiff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfLinetypeDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}
impl DxfNamedElem for DxfLinetype {
    type Diff = DxfLinetypeDiff;
    async fn key(&self) -> &str {
        &self.name
    }
    async fn diff_between(a: &Self, b: &Self) -> Self::Diff {
        DxfLinetypeDiff {
            flags: (a.flags != b.flags).then_some(b.flags),
            description: (a.description != b.description).then(|| b.description.clone()),
            unknown_group_codes: (a.unknown_group_codes != b.unknown_group_codes).then(|| b.unknown_group_codes.clone()),
        }
    }
    async fn diff_apply(d: &Self::Diff, item: &mut Self) {
        if let Some(v) = d.flags {
            item.flags = v;
        }
        if let Some(v) = &d.description {
            item.description = v.clone();
        }
        if let Some(v) = &d.unknown_group_codes {
            item.unknown_group_codes = v.clone();
        }
    }
    async fn diff_absorb(base: &mut Self::Diff, other: Self::Diff) {
        if other.flags.is_some() {
            base.flags = other.flags;
        }
        if other.description.is_some() {
            base.description = other.description;
        }
        if other.unknown_group_codes.is_some() {
            base.unknown_group_codes = other.unknown_group_codes;
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfLinetypeModified {
    pub name: String,
    pub diff: DxfLinetypeDiff,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfLinetypeAdded {
    pub index: usize,
    pub linetype: DxfLinetype,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfLinetypesDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<DxfLinetypeModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<DxfLinetypeAdded>,
}
impl DxfLinetypesDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn apply(&self, base: &[DxfLinetype]) -> Vec<DxfLinetype> {
        let modified: Vec<(String, DxfLinetypeDiff)> = self.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
        let added: Vec<(usize, DxfLinetype)> = self.added.iter().map(|a| (a.index, a.linetype.clone())).collect();
        named_apply(base, &self.removed, &modified, &added)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn between(base: &[DxfLinetype], other: &[DxfLinetype]) -> Option<Self> {
        let (removed, modified, added) = named_between(base, other);
        let d = Self { removed, modified: modified.into_iter().map(|(name, diff)| DxfLinetypeModified { name, diff }).collect(), added: added.into_iter().map(|(index, linetype)| DxfLinetypeAdded { index, linetype }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn absorb(d1: Self, d2: Self) -> Option<Self> {
        let d1m: Vec<(String, DxfLinetypeDiff)> = d1.modified.into_iter().map(|m| (m.name, m.diff)).collect();
        let d1a: Vec<(usize, DxfLinetype)> = d1.added.into_iter().map(|a| (a.index, a.linetype)).collect();
        let d2m: Vec<(String, DxfLinetypeDiff)> = d2.modified.into_iter().map(|m| (m.name, m.diff)).collect();
        let d2a: Vec<(usize, DxfLinetype)> = d2.added.into_iter().map(|a| (a.index, a.linetype)).collect();
        let (removed, modified, added) = named_absorb_pair::<DxfLinetype>(&d1.removed, &d1m, &d1a, &d2.removed, &d2m, &d2a);
        let d = Self { removed, modified: modified.into_iter().map(|(name, diff)| DxfLinetypeModified { name, diff }).collect(), added: added.into_iter().map(|(index, linetype)| DxfLinetypeAdded { index, linetype }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
}
//#endregion 🔖️LinetypeDiff

//#region 🔖️TablesDiff
/// 🔺️ Groups the three name-keyed table diffs — `DxfTables` itself is a weak grouping struct
/// (not a collection), so its diff is a plain per-field `Option<...>` struct, one per sub-collection.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfTablesDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layers: Option<DxfLayersDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub styles: Option<DxfStylesDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linetypes: Option<DxfLinetypesDiff>,
}
impl DxfTablesDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.layers.as_ref().map_or(true, DxfLayersDiff::is_empty) && self.styles.as_ref().map_or(true, DxfStylesDiff::is_empty) && self.linetypes.as_ref().map_or(true, DxfLinetypesDiff::is_empty)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn apply(&self, base: &DxfTables) -> DxfTables {
        DxfTables {
            layers: match &self.layers {
                Some(d) => d.apply(&base.layers),
                None => base.layers.clone(),
            },
            styles: match &self.styles {
                Some(d) => d.apply(&base.styles),
                None => base.styles.clone(),
            },
            linetypes: match &self.linetypes {
                Some(d) => d.apply(&base.linetypes),
                None => base.linetypes.clone(),
            },
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn between(base: &DxfTables, other: &DxfTables) -> Option<Self> {
        let d = Self { layers: DxfLayersDiff::between(&base.layers, &other.layers), styles: DxfStylesDiff::between(&base.styles, &other.styles), linetypes: DxfLinetypesDiff::between(&base.linetypes, &other.linetypes) };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn absorb(a: Self, b: Self) -> Option<Self> {
        let layers = match (a.layers, b.layers) {
            (None, None) => None,
            (Some(x), None) => Some(x),
            (None, Some(y)) => Some(y),
            (Some(x), Some(y)) => DxfLayersDiff::absorb(x, y),
        };
        let styles = match (a.styles, b.styles) {
            (None, None) => None,
            (Some(x), None) => Some(x),
            (None, Some(y)) => Some(y),
            (Some(x), Some(y)) => DxfStylesDiff::absorb(x, y),
        };
        let linetypes = match (a.linetypes, b.linetypes) {
            (None, None) => None,
            (Some(x), None) => Some(x),
            (None, Some(y)) => Some(y),
            (Some(x), Some(y)) => DxfLinetypesDiff::absorb(x, y),
        };
        let d = Self { layers, styles, linetypes };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
}
//#endregion 🔖️TablesDiff

//#region 🔖️EntityDiff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfLineDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<[f64; 3]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<[f64; 3]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfCircleDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub center: Option<[f64; 3]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfArcDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub center: Option<[f64; 3]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_angle: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_angle: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}
/// 🔺️ `vertices` is a weak leaf value (a polyline's own vertex list) — whole-vec replaced,
/// never sub-diffed (recipe's weak-entity rule).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfPolylineDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vertices: Option<Vec<DxfVertex>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfTextDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<[f64; 3]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfSolidDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub points: Option<[[f64; 3]; 4]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfInsertDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<[f64; 3]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<[f64; 3]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotation: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}
/// 🔺️ `group_codes` is a weak leaf value — whole-vec replaced (the entity's `kind` never
/// changes within an `Other` variant match; a kind change is handled by `DxfEntityDiff::Replace`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfOtherDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_codes: Option<Vec<(i32, DxfValue)>>,
}

/// 🔺️ Per-entity diff: `Replace` when the entity KIND changes at this index (the plan's
/// json/xml "Replace on kind change" rule applied to this enum collection element); otherwise a
/// kind-specific sparse field diff.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DxfEntityDiff {
    Replace { entity: DxfEntity },
    Line(DxfLineDiff),
    Circle(DxfCircleDiff),
    Arc(DxfArcDiff),
    Polyline(DxfPolylineDiff),
    Text(DxfTextDiff),
    Solid(DxfSolidDiff),
    Insert(DxfInsertDiff),
    Other(DxfOtherDiff),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn entity_diff_is_empty(d: &DxfEntityDiff) -> bool {
    match d {
        DxfEntityDiff::Replace { .. } => false,
        DxfEntityDiff::Line(x) => x == &DxfLineDiff::default(),
        DxfEntityDiff::Circle(x) => x == &DxfCircleDiff::default(),
        DxfEntityDiff::Arc(x) => x == &DxfArcDiff::default(),
        DxfEntityDiff::Polyline(x) => x == &DxfPolylineDiff::default(),
        DxfEntityDiff::Text(x) => x == &DxfTextDiff::default(),
        DxfEntityDiff::Solid(x) => x == &DxfSolidDiff::default(),
        DxfEntityDiff::Insert(x) => x == &DxfInsertDiff::default(),
        DxfEntityDiff::Other(x) => x == &DxfOtherDiff::default(),
    }
}

/// 🧭️ Kind-matched pairwise `between` — same kind at both positions produces a sparse
/// kind-specific diff; a kind change produces `Replace{entity: b}`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn entity_diff_between(a: &DxfEntity, b: &DxfEntity) -> DxfEntityDiff {
    match (a, b) {
        (DxfEntity::Line { start: sa, end: ea, layer: la, unknown_group_codes: ua }, DxfEntity::Line { start: sb, end: eb, layer: lb, unknown_group_codes: ub }) => {
            DxfEntityDiff::Line(DxfLineDiff { start: (sa != sb).then_some(*sb), end: (ea != eb).then_some(*eb), layer: (la != lb).then(|| lb.clone()), unknown_group_codes: (ua != ub).then(|| ub.clone()) })
        }
        (DxfEntity::Circle { center: ca, radius: ra, layer: la, unknown_group_codes: ua }, DxfEntity::Circle { center: cb, radius: rb, layer: lb, unknown_group_codes: ub }) => {
            DxfEntityDiff::Circle(DxfCircleDiff { center: (ca != cb).then_some(*cb), radius: (ra != rb).then_some(*rb), layer: (la != lb).then(|| lb.clone()), unknown_group_codes: (ua != ub).then(|| ub.clone()) })
        }
        (DxfEntity::Arc { center: ca, radius: ra, start_angle: saa, end_angle: eaa, layer: la, unknown_group_codes: ua }, DxfEntity::Arc { center: cb, radius: rb, start_angle: sab, end_angle: eab, layer: lb, unknown_group_codes: ub }) => {
            DxfEntityDiff::Arc(DxfArcDiff {
                center: (ca != cb).then_some(*cb),
                radius: (ra != rb).then_some(*rb),
                start_angle: (saa != sab).then_some(*sab),
                end_angle: (eaa != eab).then_some(*eab),
                layer: (la != lb).then(|| lb.clone()),
                unknown_group_codes: (ua != ub).then(|| ub.clone()),
            })
        }
        (DxfEntity::Polyline { vertices: va, closed: cla, layer: la, unknown_group_codes: ua }, DxfEntity::Polyline { vertices: vb, closed: clb, layer: lb, unknown_group_codes: ub }) => {
            DxfEntityDiff::Polyline(DxfPolylineDiff { vertices: (va != vb).then(|| vb.clone()), closed: (cla != clb).then_some(*clb), layer: (la != lb).then(|| lb.clone()), unknown_group_codes: (ua != ub).then(|| ub.clone()) })
        }
        (DxfEntity::Text { position: pa, height: ha, value: vaa, layer: la, unknown_group_codes: ua }, DxfEntity::Text { position: pb, height: hb, value: vab, layer: lb, unknown_group_codes: ub }) => DxfEntityDiff::Text(DxfTextDiff {
            position: (pa != pb).then_some(*pb),
            height: (ha != hb).then_some(*hb),
            value: (vaa != vab).then(|| vab.clone()),
            layer: (la != lb).then(|| lb.clone()),
            unknown_group_codes: (ua != ub).then(|| ub.clone()),
        }),
        (DxfEntity::Solid { points: pa, layer: la, unknown_group_codes: ua }, DxfEntity::Solid { points: pb, layer: lb, unknown_group_codes: ub }) => {
            DxfEntityDiff::Solid(DxfSolidDiff { points: (pa != pb).then_some(*pb), layer: (la != lb).then(|| lb.clone()), unknown_group_codes: (ua != ub).then(|| ub.clone()) })
        }
        (DxfEntity::Insert { block_name: ba, position: pa, scale: sca, rotation: ra, layer: la, unknown_group_codes: ua }, DxfEntity::Insert { block_name: bb, position: pb, scale: scb, rotation: rb, layer: lb, unknown_group_codes: ub }) => {
            DxfEntityDiff::Insert(DxfInsertDiff {
                block_name: (ba != bb).then(|| bb.clone()),
                position: (pa != pb).then_some(*pb),
                scale: (sca != scb).then_some(*scb),
                rotation: (ra != rb).then_some(*rb),
                layer: (la != lb).then(|| lb.clone()),
                unknown_group_codes: (ua != ub).then(|| ub.clone()),
            })
        }
        (DxfEntity::Other { kind: ka, group_codes: ga }, DxfEntity::Other { kind: kb, group_codes: gb }) if ka == kb => DxfEntityDiff::Other(DxfOtherDiff { group_codes: (ga != gb).then(|| gb.clone()) }),
        _ => DxfEntityDiff::Replace { entity: b.clone() },
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_line_diff(d: &DxfLineDiff, start: &mut [f64; 3], end: &mut [f64; 3], layer: &mut String, unknown: &mut Vec<(i32, DxfValue)>) {
    if let Some(v) = d.start {
        *start = v;
    }
    if let Some(v) = d.end {
        *end = v;
    }
    if let Some(v) = &d.layer {
        *layer = v.clone();
    }
    if let Some(v) = &d.unknown_group_codes {
        *unknown = v.clone();
    }
}

/// ▶️ Applies a kind-specific diff to an entity — used both by `diff_apply` (real position) and
/// by absorb's `Replace`+kind-diff branch (patch-into-the-carried-replacement, same shape as the
/// recipe's canonical `Insert+SetField` "patch into added payload" case).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_entity_diff(d: &DxfEntityDiff, item: &mut DxfEntity) {
    match (d, item) {
        (DxfEntityDiff::Line(ld), DxfEntity::Line { start, end, layer, unknown_group_codes }) => apply_line_diff(ld, start, end, layer, unknown_group_codes),
        (DxfEntityDiff::Circle(cd), DxfEntity::Circle { center, radius, layer, unknown_group_codes }) => {
            if let Some(v) = cd.center {
                *center = v;
            }
            if let Some(v) = cd.radius {
                *radius = v;
            }
            if let Some(v) = &cd.layer {
                *layer = v.clone();
            }
            if let Some(v) = &cd.unknown_group_codes {
                *unknown_group_codes = v.clone();
            }
        }
        (DxfEntityDiff::Arc(ad), DxfEntity::Arc { center, radius, start_angle, end_angle, layer, unknown_group_codes }) => {
            if let Some(v) = ad.center {
                *center = v;
            }
            if let Some(v) = ad.radius {
                *radius = v;
            }
            if let Some(v) = ad.start_angle {
                *start_angle = v;
            }
            if let Some(v) = ad.end_angle {
                *end_angle = v;
            }
            if let Some(v) = &ad.layer {
                *layer = v.clone();
            }
            if let Some(v) = &ad.unknown_group_codes {
                *unknown_group_codes = v.clone();
            }
        }
        (DxfEntityDiff::Polyline(pd), DxfEntity::Polyline { vertices, closed, layer, unknown_group_codes }) => {
            if let Some(v) = &pd.vertices {
                *vertices = v.clone();
            }
            if let Some(v) = pd.closed {
                *closed = v;
            }
            if let Some(v) = &pd.layer {
                *layer = v.clone();
            }
            if let Some(v) = &pd.unknown_group_codes {
                *unknown_group_codes = v.clone();
            }
        }
        (DxfEntityDiff::Text(td), DxfEntity::Text { position, height, value, layer, unknown_group_codes }) => {
            if let Some(v) = td.position {
                *position = v;
            }
            if let Some(v) = td.height {
                *height = v;
            }
            if let Some(v) = &td.value {
                *value = v.clone();
            }
            if let Some(v) = &td.layer {
                *layer = v.clone();
            }
            if let Some(v) = &td.unknown_group_codes {
                *unknown_group_codes = v.clone();
            }
        }
        (DxfEntityDiff::Solid(sd), DxfEntity::Solid { points, layer, unknown_group_codes }) => {
            if let Some(v) = sd.points {
                *points = v;
            }
            if let Some(v) = &sd.layer {
                *layer = v.clone();
            }
            if let Some(v) = &sd.unknown_group_codes {
                *unknown_group_codes = v.clone();
            }
        }
        (DxfEntityDiff::Insert(id), DxfEntity::Insert { block_name, position, scale, rotation, layer, unknown_group_codes }) => {
            if let Some(v) = &id.block_name {
                *block_name = v.clone();
            }
            if let Some(v) = id.position {
                *position = v;
            }
            if let Some(v) = id.scale {
                *scale = v;
            }
            if let Some(v) = id.rotation {
                *rotation = v;
            }
            if let Some(v) = &id.layer {
                *layer = v.clone();
            }
            if let Some(v) = &id.unknown_group_codes {
                *unknown_group_codes = v.clone();
            }
        }
        (DxfEntityDiff::Other(od), DxfEntity::Other { group_codes, .. }) => {
            if let Some(v) = &od.group_codes {
                *group_codes = v.clone();
            }
        }
        _ => {} // kind mismatch without Replace: contract violation, graceful no-op
    }
}

impl DxfIndexElem for DxfEntity {
    type Diff = DxfEntityDiff;
    async fn diff_is_empty(d: &Self::Diff) -> bool {
        entity_diff_is_empty(d)
    }
    async fn diff_between(a: &Self, b: &Self) -> Self::Diff {
        entity_diff_between(a, b)
    }
    async fn diff_apply(d: &Self::Diff, item: &mut Self) {
        if let DxfEntityDiff::Replace { entity } = d {
            *item = entity.clone();
        } else {
            apply_entity_diff(d, item);
        }
    }
    /// ➕️ Structural, base-free absorb over entity diffs: two same-kind kind-specific diffs
    /// merge field-by-field (LWW); a `Replace` on either side wins (mid/after ultimately becomes
    /// that literal entity), with a trailing kind-specific `other` patched INTO the carried
    /// replacement payload — the recipe's canonical "patch into added/replaced payload" case.
    async fn diff_absorb(base: &mut Self::Diff, other: Self::Diff) {
        *base = match (base.clone(), other) {
            (DxfEntityDiff::Replace { .. }, DxfEntityDiff::Replace { entity: e2 }) => DxfEntityDiff::Replace { entity: e2 },
            (DxfEntityDiff::Replace { mut entity }, other_diff) => {
                apply_entity_diff(&other_diff, &mut entity);
                DxfEntityDiff::Replace { entity }
            }
            (_, DxfEntityDiff::Replace { entity }) => DxfEntityDiff::Replace { entity },
            (DxfEntityDiff::Line(mut a), DxfEntityDiff::Line(b)) => {
                if b.start.is_some() {
                    a.start = b.start;
                }
                if b.end.is_some() {
                    a.end = b.end;
                }
                if b.layer.is_some() {
                    a.layer = b.layer;
                }
                if b.unknown_group_codes.is_some() {
                    a.unknown_group_codes = b.unknown_group_codes;
                }
                DxfEntityDiff::Line(a)
            }
            (DxfEntityDiff::Circle(mut a), DxfEntityDiff::Circle(b)) => {
                if b.center.is_some() {
                    a.center = b.center;
                }
                if b.radius.is_some() {
                    a.radius = b.radius;
                }
                if b.layer.is_some() {
                    a.layer = b.layer;
                }
                if b.unknown_group_codes.is_some() {
                    a.unknown_group_codes = b.unknown_group_codes;
                }
                DxfEntityDiff::Circle(a)
            }
            (DxfEntityDiff::Arc(mut a), DxfEntityDiff::Arc(b)) => {
                if b.center.is_some() {
                    a.center = b.center;
                }
                if b.radius.is_some() {
                    a.radius = b.radius;
                }
                if b.start_angle.is_some() {
                    a.start_angle = b.start_angle;
                }
                if b.end_angle.is_some() {
                    a.end_angle = b.end_angle;
                }
                if b.layer.is_some() {
                    a.layer = b.layer;
                }
                if b.unknown_group_codes.is_some() {
                    a.unknown_group_codes = b.unknown_group_codes;
                }
                DxfEntityDiff::Arc(a)
            }
            (DxfEntityDiff::Polyline(mut a), DxfEntityDiff::Polyline(b)) => {
                if b.vertices.is_some() {
                    a.vertices = b.vertices;
                }
                if b.closed.is_some() {
                    a.closed = b.closed;
                }
                if b.layer.is_some() {
                    a.layer = b.layer;
                }
                if b.unknown_group_codes.is_some() {
                    a.unknown_group_codes = b.unknown_group_codes;
                }
                DxfEntityDiff::Polyline(a)
            }
            (DxfEntityDiff::Text(mut a), DxfEntityDiff::Text(b)) => {
                if b.position.is_some() {
                    a.position = b.position;
                }
                if b.height.is_some() {
                    a.height = b.height;
                }
                if b.value.is_some() {
                    a.value = b.value;
                }
                if b.layer.is_some() {
                    a.layer = b.layer;
                }
                if b.unknown_group_codes.is_some() {
                    a.unknown_group_codes = b.unknown_group_codes;
                }
                DxfEntityDiff::Text(a)
            }
            (DxfEntityDiff::Solid(mut a), DxfEntityDiff::Solid(b)) => {
                if b.points.is_some() {
                    a.points = b.points;
                }
                if b.layer.is_some() {
                    a.layer = b.layer;
                }
                if b.unknown_group_codes.is_some() {
                    a.unknown_group_codes = b.unknown_group_codes;
                }
                DxfEntityDiff::Solid(a)
            }
            (DxfEntityDiff::Insert(mut a), DxfEntityDiff::Insert(b)) => {
                if b.block_name.is_some() {
                    a.block_name = b.block_name;
                }
                if b.position.is_some() {
                    a.position = b.position;
                }
                if b.scale.is_some() {
                    a.scale = b.scale;
                }
                if b.rotation.is_some() {
                    a.rotation = b.rotation;
                }
                if b.layer.is_some() {
                    a.layer = b.layer;
                }
                if b.unknown_group_codes.is_some() {
                    a.unknown_group_codes = b.unknown_group_codes;
                }
                DxfEntityDiff::Insert(a)
            }
            (DxfEntityDiff::Other(mut a), DxfEntityDiff::Other(b)) => {
                if b.group_codes.is_some() {
                    a.group_codes = b.group_codes;
                }
                DxfEntityDiff::Other(a)
            }
            // 🧭️ Structurally-inconsistent kind mismatch without a Replace (shouldn't occur when
            // both diffs came from real `between`/mutation calls against a consistent sequence of
            // states): fall back to `other`, best-effort, never panics (total per the absorb contract).
            (_, other_diff) => other_diff,
        };
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfEntityModified {
    pub index: usize,
    pub diff: DxfEntityDiff,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfEntityAdded {
    pub index: usize,
    pub entity: DxfEntity,
}
/// 🔺️ Index-keyed removed/modified/added triple over an entity collection — reused for BOTH
/// `DxfSnapshot::entities` and each `DxfBlock::entities`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfEntitiesDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<DxfEntityModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<DxfEntityAdded>,
}
impl DxfEntitiesDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn apply(&self, base: &[DxfEntity]) -> Vec<DxfEntity> {
        let modified: Vec<(usize, DxfEntityDiff)> = self.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
        let added: Vec<(usize, DxfEntity)> = self.added.iter().map(|a| (a.index, a.entity.clone())).collect();
        generic_apply(base, &self.removed, &modified, &added)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn between(base: &[DxfEntity], other: &[DxfEntity]) -> Option<Self> {
        let (removed, modified, added) = generic_between(base, other);
        let d = Self { removed, modified: modified.into_iter().map(|(index, diff)| DxfEntityModified { index, diff }).collect(), added: added.into_iter().map(|(index, entity)| DxfEntityAdded { index, entity }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn absorb(d1: Self, d2: Self) -> Option<Self> {
        let d1m: Vec<(usize, DxfEntityDiff)> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();
        let d1a: Vec<(usize, DxfEntity)> = d1.added.into_iter().map(|a| (a.index, a.entity)).collect();
        let d2m: Vec<(usize, DxfEntityDiff)> = d2.modified.into_iter().map(|m| (m.index, m.diff)).collect();
        let d2a: Vec<(usize, DxfEntity)> = d2.added.into_iter().map(|a| (a.index, a.entity)).collect();
        let (removed, modified, added) = generic_absorb_pair::<DxfEntity>(&d1.removed, &d1m, &d1a, &d2.removed, &d2m, &d2a);
        let d = Self { removed, modified: modified.into_iter().map(|(index, diff)| DxfEntityModified { index, diff }).collect(), added: added.into_iter().map(|(index, entity)| DxfEntityAdded { index, entity }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
}
//#endregion 🔖️EntityDiff

//#region 🔖️BlockDiff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfBlockDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_point: Option<[f64; 3]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<DxfEntitiesDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}
impl DxfIndexElem for DxfBlock {
    type Diff = DxfBlockDiff;
    async fn diff_is_empty(d: &Self::Diff) -> bool {
        d == &DxfBlockDiff::default()
    }
    async fn diff_between(a: &Self, b: &Self) -> Self::Diff {
        DxfBlockDiff {
            name: (a.name != b.name).then(|| b.name.clone()),
            base_point: (a.base_point != b.base_point).then_some(b.base_point),
            entities: DxfEntitiesDiff::between(&a.entities, &b.entities),
            unknown_group_codes: (a.unknown_group_codes != b.unknown_group_codes).then(|| b.unknown_group_codes.clone()),
        }
    }
    async fn diff_apply(d: &Self::Diff, item: &mut Self) {
        if let Some(v) = &d.name {
            item.name = v.clone();
        }
        if let Some(v) = d.base_point {
            item.base_point = v;
        }
        if let Some(ed) = &d.entities {
            item.entities = ed.apply(&item.entities);
        }
        if let Some(v) = &d.unknown_group_codes {
            item.unknown_group_codes = v.clone();
        }
    }
    async fn diff_absorb(base: &mut Self::Diff, other: Self::Diff) {
        if other.name.is_some() {
            base.name = other.name;
        }
        if other.base_point.is_some() {
            base.base_point = other.base_point;
        }
        base.entities = match (base.entities.take(), other.entities) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => DxfEntitiesDiff::absorb(a, b),
        };
        if other.unknown_group_codes.is_some() {
            base.unknown_group_codes = other.unknown_group_codes;
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfBlockModified {
    pub index: usize,
    pub diff: DxfBlockDiff,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfBlockAdded {
    pub index: usize,
    pub block: DxfBlock,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfBlocksDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<DxfBlockModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<DxfBlockAdded>,
}
impl DxfBlocksDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn apply(&self, base: &[DxfBlock]) -> Vec<DxfBlock> {
        let modified: Vec<(usize, DxfBlockDiff)> = self.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
        let added: Vec<(usize, DxfBlock)> = self.added.iter().map(|a| (a.index, a.block.clone())).collect();
        generic_apply(base, &self.removed, &modified, &added)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn between(base: &[DxfBlock], other: &[DxfBlock]) -> Option<Self> {
        let (removed, modified, added) = generic_between(base, other);
        let d = Self { removed, modified: modified.into_iter().map(|(index, diff)| DxfBlockModified { index, diff }).collect(), added: added.into_iter().map(|(index, block)| DxfBlockAdded { index, block }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn absorb(d1: Self, d2: Self) -> Option<Self> {
        let d1m: Vec<(usize, DxfBlockDiff)> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();
        let d1a: Vec<(usize, DxfBlock)> = d1.added.into_iter().map(|a| (a.index, a.block)).collect();
        let d2m: Vec<(usize, DxfBlockDiff)> = d2.modified.into_iter().map(|m| (m.index, m.diff)).collect();
        let d2a: Vec<(usize, DxfBlock)> = d2.added.into_iter().map(|a| (a.index, a.block)).collect();
        let (removed, modified, added) = generic_absorb_pair::<DxfBlock>(&d1.removed, &d1m, &d1a, &d2.removed, &d2m, &d2a);
        let d = Self { removed, modified: modified.into_iter().map(|(index, diff)| DxfBlockModified { index, diff }).collect(), added: added.into_iter().map(|(index, block)| DxfBlockAdded { index, block }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
}
//#endregion 🔖️BlockDiff

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.dxf`. `schema` is an identity field and never appears here.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.dxf.diff")]
pub struct DxfDiff {
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header_vars: Option<DxfHeaderVarsDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tables: Option<DxfTablesDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<DxfBlocksDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<DxfEntitiesDiff>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn target_error(code: &'static str, message: &'static str, target: Vec<String>) -> MutationApplyError {
    MutationApplyError::new(code, message).at(target)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_indexed_targets(base_len: usize, removed_indices: &[usize], modified_indices: impl IntoIterator<Item = usize>, added_indices: impl IntoIterator<Item = usize>, prefix: &[String]) -> MutationApplyResult<()> {
    let mut removed = BTreeSet::new();
    for &index in removed_indices {
        let mut target = prefix.to_vec();
        target.push(index.to_string());
        if index >= base_len || !removed.insert(index) {
            return Err(target_error("invalid-remove-index", "removal target must exist exactly once", target));
        }
    }
    let mut modified = BTreeSet::new();
    for index in modified_indices {
        let mut target = prefix.to_vec();
        target.push(index.to_string());
        if index >= base_len || removed.contains(&index) || !modified.insert(index) {
            return Err(target_error("invalid-modify-index", "modification target must exist exactly once and remain present", target));
        }
    }
    let mut length = base_len - removed.len();
    let mut additions: Vec<usize> = added_indices.into_iter().collect();
    additions.sort_unstable();
    let mut previous = None;
    for index in additions {
        let mut target = prefix.to_vec();
        target.push(index.to_string());
        if index > length || previous == Some(index) {
            return Err(target_error("invalid-add-index", "addition target must be unique and within the evolving sequence", target));
        }
        previous = Some(index);
        length += 1;
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_named_targets<'a>(
    base_keys: impl IntoIterator<Item = &'a str>,
    removed_keys: impl IntoIterator<Item = &'a str>,
    modified_keys: impl IntoIterator<Item = &'a str>,
    added: impl IntoIterator<Item = (usize, &'a str)>,
    prefix: &[String],
) -> MutationApplyResult<()> {
    let mut base = BTreeSet::new();
    for key in base_keys {
        let mut target = prefix.to_vec();
        target.push(key.to_string());
        if !base.insert(key) {
            return Err(target_error("duplicate-base-target", "base names must be unique", target));
        }
    }
    let mut removed = BTreeSet::new();
    for key in removed_keys {
        let mut target = prefix.to_vec();
        target.push(key.to_string());
        if !base.contains(key) || !removed.insert(key) {
            return Err(target_error("invalid-remove-target", "removal target must exist exactly once", target));
        }
    }
    let mut modified = BTreeSet::new();
    for key in modified_keys {
        let mut target = prefix.to_vec();
        target.push(key.to_string());
        if !base.contains(key) || removed.contains(key) || !modified.insert(key) {
            return Err(target_error("invalid-modify-target", "modification target must exist exactly once and remain present", target));
        }
    }
    let mut length = base.len() - removed.len();
    let mut additions: Vec<(usize, &str)> = added.into_iter().collect();
    additions.sort_by_key(|(index, _)| *index);
    let mut added_keys = BTreeSet::new();
    let mut previous = None;
    for (index, key) in additions {
        let mut target = prefix.to_vec();
        target.push(key.to_string());
        if base.contains(key) || !added_keys.insert(key) || index > length || previous == Some(index) {
            return Err(target_error("invalid-add-target", "addition name and position must be unique and valid", target));
        }
        previous = Some(index);
        length += 1;
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn entity_diff_matches(entity: &DxfEntity, diff: &DxfEntityDiff) -> bool {
    matches!(
        (entity, diff),
        (_, DxfEntityDiff::Replace { .. })
            | (DxfEntity::Line { .. }, DxfEntityDiff::Line(_))
            | (DxfEntity::Circle { .. }, DxfEntityDiff::Circle(_))
            | (DxfEntity::Arc { .. }, DxfEntityDiff::Arc(_))
            | (DxfEntity::Polyline { .. }, DxfEntityDiff::Polyline(_))
            | (DxfEntity::Text { .. }, DxfEntityDiff::Text(_))
            | (DxfEntity::Solid { .. }, DxfEntityDiff::Solid(_))
            | (DxfEntity::Insert { .. }, DxfEntityDiff::Insert(_))
            | (DxfEntity::Other { .. }, DxfEntityDiff::Other(_))
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_entities_diff(base: &[DxfEntity], diff: &DxfEntitiesDiff, prefix: &[String]) -> MutationApplyResult<()> {
    validate_indexed_targets(base.len(), &diff.removed, diff.modified.iter().map(|entry| entry.index), diff.added.iter().map(|entry| entry.index), prefix)?;
    for entry in &diff.modified {
        if !entity_diff_matches(&base[entry.index], &entry.diff) {
            let mut target = prefix.to_vec();
            target.push(entry.index.to_string());
            return Err(target_error("entity-kind-mismatch", "kind-specific entity diff must match its target entity", target));
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_dxf_diff(diff: &DxfDiff, base: &DxfSnapshot) -> MutationApplyResult<()> {
    if let Some(value) = &diff.header_vars {
        validate_named_targets(
            base.header_vars.iter().map(|entry| entry.name.as_str()),
            value.removed.iter().map(String::as_str),
            value.modified.iter().map(|entry| entry.name.as_str()),
            value.added.iter().map(|entry| (entry.index, entry.header_var.name.as_str())),
            &["headerVars".to_string()],
        )?;
    }
    if let Some(tables) = &diff.tables {
        if let Some(value) = &tables.layers {
            validate_named_targets(
                base.tables.layers.iter().map(|entry| entry.name.as_str()),
                value.removed.iter().map(String::as_str),
                value.modified.iter().map(|entry| entry.name.as_str()),
                value.added.iter().map(|entry| (entry.index, entry.layer.name.as_str())),
                &["tables".to_string(), "layers".to_string()],
            )?;
        }
        if let Some(value) = &tables.styles {
            validate_named_targets(
                base.tables.styles.iter().map(|entry| entry.name.as_str()),
                value.removed.iter().map(String::as_str),
                value.modified.iter().map(|entry| entry.name.as_str()),
                value.added.iter().map(|entry| (entry.index, entry.style.name.as_str())),
                &["tables".to_string(), "styles".to_string()],
            )?;
        }
        if let Some(value) = &tables.linetypes {
            validate_named_targets(
                base.tables.linetypes.iter().map(|entry| entry.name.as_str()),
                value.removed.iter().map(String::as_str),
                value.modified.iter().map(|entry| entry.name.as_str()),
                value.added.iter().map(|entry| (entry.index, entry.linetype.name.as_str())),
                &["tables".to_string(), "linetypes".to_string()],
            )?;
        }
    }
    if let Some(value) = &diff.blocks {
        validate_indexed_targets(base.blocks.len(), &value.removed, value.modified.iter().map(|entry| entry.index), value.added.iter().map(|entry| entry.index), &["blocks".to_string()])?;
        for entry in &value.modified {
            if let Some(entities) = &entry.diff.entities {
                validate_entities_diff(&base.blocks[entry.index].entities, entities, &["blocks".to_string(), entry.index.to_string(), "entities".to_string()])?;
            }
        }
    }
    if let Some(value) = &diff.entities {
        validate_entities_diff(&base.entities, value, &["entities".to_string()])?;
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_dxf_diff_unchecked(diff: &DxfDiff, base: &DxfSnapshot) -> DxfSnapshot {
    DxfSnapshot {
        schema: base.schema.clone(),
        header_vars: diff.header_vars.as_ref().map_or_else(|| base.header_vars.clone(), |value| value.apply(&base.header_vars)),
        tables: diff.tables.as_ref().map_or_else(|| base.tables.clone(), |value| value.apply(&base.tables)),
        other_tables: base.other_tables.clone(),
        blocks: diff.blocks.as_ref().map_or_else(|| base.blocks.clone(), |value| value.apply(&base.blocks)),
        entities: diff.entities.as_ref().map_or_else(|| base.entities.clone(), |value| value.apply(&base.entities)),
    }
}

impl MutationDiff<DxfSnapshot> for DxfDiff {
    fn apply(&self, base: &DxfSnapshot) -> MutationApplyResult<DxfSnapshot> {
        validate_dxf_diff(self, base)?;
        Ok(apply_dxf_diff_unchecked(self, base))
    }

    /// ➕️ Structural, total, base-free sequential-coalesce (`## Absorb` contract): every
    /// collection uses its own generic absorb-pair transport; `tables` recurses into its own
    /// three sub-collections.
    fn absorb(&mut self, other: Self) {
        self.header_vars = match (self.header_vars.take(), other.header_vars) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => DxfHeaderVarsDiff::absorb(a, b),
        };
        self.tables = match (self.tables.take(), other.tables) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => DxfTablesDiff::absorb(a, b),
        };
        self.blocks = match (self.blocks.take(), other.blocks) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => DxfBlocksDiff::absorb(a, b),
        };
        self.entities = match (self.entities.take(), other.entities) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => DxfEntitiesDiff::absorb(a, b),
        };
    }
}

impl DiffAlgebra<DxfSnapshot> for DxfDiff {
    /// 🔁️ Diff-level undo, derived generically (correct by construction) via `apply` + `between`.
    fn inverse(&self, base: &DxfSnapshot) -> Self {
        let mutated = apply_dxf_diff_unchecked(self, base);
        Self::between(&mutated, base)
    }

    fn between(base: &DxfSnapshot, other: &DxfSnapshot) -> Self {
        DxfDiff {
            header_vars: DxfHeaderVarsDiff::between(&base.header_vars, &other.header_vars),
            tables: DxfTablesDiff::between(&base.tables, &other.tables),
            blocks: DxfBlocksDiff::between(&base.blocks, &other.blocks),
            entities: DxfEntitiesDiff::between(&base.entities, &other.entities),
        }
    }

    fn is_empty(&self) -> bool {
        self.header_vars.as_ref().map_or(true, DxfHeaderVarsDiff::is_empty)
            && self.tables.as_ref().map_or(true, DxfTablesDiff::is_empty)
            && self.blocks.as_ref().map_or(true, DxfBlocksDiff::is_empty)
            && self.entities.as_ref().map_or(true, DxfEntitiesDiff::is_empty)
    }
}

/// 🧩 `SetSnapshot`'s diff is the sparse field-by-field `between(base, next)` — no full-replace
/// slot exists on `DxfDiff` to short-circuit into.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &DxfSnapshot, next: &DxfSnapshot) -> DxfDiff {
    DxfDiff::between(base, next)
}
//#endregion 🔖️Diff

//#region 🔖️MutationDiffBuilders
// 🧮 Item-level `between` wrappers, exposed to `🧬️mutations` so `SetLayer`/`SetStyle`/
// `SetLinetype`/`SetEntity`/`SetBlock`'s `diff()` can compute a sparse per-field patch without the
// private `DxfNamedElem`/`DxfIndexElem` traits themselves leaving this module.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn header_var_diff_between(a: &DxfHeaderVar, b: &DxfHeaderVar) -> DxfHeaderVarDiff {
    <DxfHeaderVar as DxfNamedElem>::diff_between(a, b)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn layer_diff_between(a: &DxfLayer, b: &DxfLayer) -> DxfLayerDiff {
    <DxfLayer as DxfNamedElem>::diff_between(a, b)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn style_diff_between(a: &DxfStyle, b: &DxfStyle) -> DxfStyleDiff {
    <DxfStyle as DxfNamedElem>::diff_between(a, b)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn linetype_diff_between(a: &DxfLinetype, b: &DxfLinetype) -> DxfLinetypeDiff {
    <DxfLinetype as DxfNamedElem>::diff_between(a, b)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn entity_diff_between_pub(a: &DxfEntity, b: &DxfEntity) -> DxfEntityDiff {
    entity_diff_between(a, b)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn block_diff_between(a: &DxfBlock, b: &DxfBlock) -> DxfBlockDiff {
    <DxfBlock as DxfIndexElem>::diff_between(a, b)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_header_var(index: usize, name: &str, header_var: DxfHeaderVar, existed: bool) -> DxfDiff {
    if existed {
        DxfDiff {
            header_vars: Some(DxfHeaderVarsDiff {
                removed: vec![],
                modified: vec![DxfHeaderVarModified { name: name.to_string(), diff: DxfHeaderVarDiff { group_code: Some(header_var.group_code), value: Some(header_var.value), extra_group_codes: Some(header_var.extra_group_codes) } }],
                added: vec![],
            }),
            ..Default::default()
        }
    } else {
        DxfDiff { header_vars: Some(DxfHeaderVarsDiff { removed: vec![], modified: vec![], added: vec![DxfHeaderVarAdded { index, header_var }] }), ..Default::default() }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_header_var(name: &str) -> DxfDiff {
    DxfDiff { header_vars: Some(DxfHeaderVarsDiff { removed: vec![name.to_string()], modified: vec![], added: vec![] }), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_layer(index: usize, layer: DxfLayer) -> DxfDiff {
    DxfDiff { tables: Some(DxfTablesDiff { layers: Some(DxfLayersDiff { removed: vec![], modified: vec![], added: vec![DxfLayerAdded { index, layer }] }), ..Default::default() }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_layer(name: &str) -> DxfDiff {
    DxfDiff { tables: Some(DxfTablesDiff { layers: Some(DxfLayersDiff { removed: vec![name.to_string()], modified: vec![], added: vec![] }), ..Default::default() }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_layer(name: &str, diff: DxfLayerDiff) -> DxfDiff {
    DxfDiff { tables: Some(DxfTablesDiff { layers: Some(DxfLayersDiff { removed: vec![], modified: vec![DxfLayerModified { name: name.to_string(), diff }], added: vec![] }), ..Default::default() }), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_style(index: usize, style: DxfStyle) -> DxfDiff {
    DxfDiff { tables: Some(DxfTablesDiff { styles: Some(DxfStylesDiff { removed: vec![], modified: vec![], added: vec![DxfStyleAdded { index, style }] }), ..Default::default() }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_style(name: &str) -> DxfDiff {
    DxfDiff { tables: Some(DxfTablesDiff { styles: Some(DxfStylesDiff { removed: vec![name.to_string()], modified: vec![], added: vec![] }), ..Default::default() }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_style(name: &str, diff: DxfStyleDiff) -> DxfDiff {
    DxfDiff { tables: Some(DxfTablesDiff { styles: Some(DxfStylesDiff { removed: vec![], modified: vec![DxfStyleModified { name: name.to_string(), diff }], added: vec![] }), ..Default::default() }), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_linetype(index: usize, linetype: DxfLinetype) -> DxfDiff {
    DxfDiff { tables: Some(DxfTablesDiff { linetypes: Some(DxfLinetypesDiff { removed: vec![], modified: vec![], added: vec![DxfLinetypeAdded { index, linetype }] }), ..Default::default() }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_linetype(name: &str) -> DxfDiff {
    DxfDiff { tables: Some(DxfTablesDiff { linetypes: Some(DxfLinetypesDiff { removed: vec![name.to_string()], modified: vec![], added: vec![] }), ..Default::default() }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_linetype(name: &str, diff: DxfLinetypeDiff) -> DxfDiff {
    DxfDiff { tables: Some(DxfTablesDiff { linetypes: Some(DxfLinetypesDiff { removed: vec![], modified: vec![DxfLinetypeModified { name: name.to_string(), diff }], added: vec![] }), ..Default::default() }), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_entity(index: usize, entity: DxfEntity) -> DxfDiff {
    DxfDiff { entities: Some(DxfEntitiesDiff { removed: vec![], modified: vec![], added: vec![DxfEntityAdded { index, entity }] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_entity(index: usize) -> DxfDiff {
    DxfDiff { entities: Some(DxfEntitiesDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_entity(index: usize, diff: DxfEntityDiff) -> DxfDiff {
    DxfDiff { entities: Some(DxfEntitiesDiff { removed: vec![], modified: vec![DxfEntityModified { index, diff }], added: vec![] }), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_block(index: usize, block: DxfBlock) -> DxfDiff {
    DxfDiff { blocks: Some(DxfBlocksDiff { removed: vec![], modified: vec![], added: vec![DxfBlockAdded { index, block }] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_block(index: usize) -> DxfDiff {
    DxfDiff { blocks: Some(DxfBlocksDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_block(index: usize, diff: DxfBlockDiff) -> DxfDiff {
    DxfDiff { blocks: Some(DxfBlocksDiff { removed: vec![], modified: vec![DxfBlockModified { index, diff }], added: vec![] }), ..Default::default() }
}
//#endregion 🔖️MutationDiffBuilders

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: **hand-rolled** `protocol::DiffCodec` for `DxfDiff` — `#[derive(dsl::DslDiff)]`
/// rejected (see module doc for the real compiler citation). Same grammar family `GifDiff`'s and
/// `SvgDiff`'s hand-rolled codecs use (bracket-depth-aware split, hex for strings/bytes,
/// `[0]`/`[1,x]` for `Option<T>`, single-uppercase-letter tag prefix for data-carrying enums) —
/// this file re-derives its own copies of the small helper primitives since each hand-rolled
/// codec is self-contained (no shared "hand-roll helpers" module exists yet). Every helper touched
/// by `🧬️mutations`'s own hand-rolled `OpText`/`OpBinary` is `pub(crate)` for reuse (same pattern
/// svg's diff file uses for its mutations sibling).
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_f64(v: f64) -> String {
    format!("{v}")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}

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
/// 🧺 Self-bracketing plain list — the shared core every `Vec<T>` field's grammar uses (group
/// codes, vertices, entities, header vars, …).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(enc).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
}
//#endregion 🔖️Primitives

//#region 🔖️GeometryCodecs
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_point3(p: &[f64; 3]) -> String {
    format!("[{},{},{}]", enc_f64(p[0]), enc_f64(p[1]), enc_f64(p[2]))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_point3(s: &str) -> Result<[f64; 3], String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z] = parts.as_slice() else { return Err(format!("point3: expected 3 fields, got {}", parts.len())) };
    Ok([dec_f64(x)?, dec_f64(y)?, dec_f64(z)?])
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_points4(p: &[[f64; 3]; 4]) -> String {
    enc_list(p.as_slice(), enc_point3)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_points4(s: &str) -> Result<[[f64; 3]; 4], String> {
    let v = dec_list(s, dec_point3)?;
    let arr: [[f64; 3]; 4] = v.clone().try_into().map_err(|_| format!("points4: expected 4 points, got {}", v.len()))?;
    Ok(arr)
}
//#endregion 🔖️GeometryCodecs

//#region 🔖️ValueCodecs
/// 🧮 `DxfValue` (data-carrying enum — this file's own root cause for the `DslDiff`/`DslOps`
/// rejection): `S[hex]`/`I[digits]`/`D[float]`/`P[x,y,z]`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_dxf_value(v: &DxfValue) -> String {
    match v {
        DxfValue::Str { value } => format!("S[{}]", enc_str(value)),
        DxfValue::Int { value } => format!("I[{value}]"),
        DxfValue::Double { value } => format!("D[{}]", enc_f64(*value)),
        DxfValue::Point { value } => format!("P[{},{},{}]", enc_f64(value[0]), enc_f64(value[1]), enc_f64(value[2])),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_dxf_value(s: &str) -> Result<DxfValue, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "S" => Ok(DxfValue::Str { value: dec_str(inner)? }),
        "I" => Ok(DxfValue::Int { value: inner.parse().map_err(|e: std::num::ParseIntError| e.to_string())? }),
        "D" => Ok(DxfValue::Double { value: dec_f64(inner)? }),
        "P" => {
            let parts = split_top_level(inner, ',');
            let [x, y, z] = parts.as_slice() else { return Err(format!("dxf value point: expected 3 fields, got {}", parts.len())) };
            Ok(DxfValue::Point { value: [dec_f64(x)?, dec_f64(y)?, dec_f64(z)?] })
        }
        other => Err(format!("dxf value: unknown tag {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_group_code(pair: &(i32, DxfValue)) -> String {
    format!("[{},{}]", pair.0, enc_dxf_value(&pair.1))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_group_code(s: &str) -> Result<(i32, DxfValue), String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [code, value] = parts.as_slice() else { return Err(format!("group code: expected 2 fields, got {}", parts.len())) };
    Ok((code.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, dec_dxf_value(value)?))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_group_codes(v: &[(i32, DxfValue)]) -> String {
    enc_list(v, enc_group_code)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_group_codes(s: &str) -> Result<Vec<(i32, DxfValue)>, String> {
    dec_list(s, dec_group_code)
}
//#endregion 🔖️ValueCodecs

//#region 🔖️EntityValueCodecs
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_vertex(v: &DxfVertex) -> String {
    format!("[{},{},{},{},{}]", enc_f64(v.x), enc_f64(v.y), enc_f64(v.z), enc_f64(v.bulge), enc_group_codes(&v.unknown_group_codes))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_vertex(s: &str) -> Result<DxfVertex, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z, bulge, unknown] = parts.as_slice() else { return Err(format!("vertex: expected 5 fields, got {}", parts.len())) };
    Ok(DxfVertex { x: dec_f64(x)?, y: dec_f64(y)?, z: dec_f64(z)?, bulge: dec_f64(bulge)?, unknown_group_codes: dec_group_codes(unknown)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_vertices(vs: &[DxfVertex]) -> String {
    enc_list(vs, enc_vertex)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_vertices(s: &str) -> Result<Vec<DxfVertex>, String> {
    dec_list(s, dec_vertex)
}

/// 📐️ `DxfEntity` (data-carrying enum, the whole entity — not its diff): tag prefix per kind,
/// `L`=Line, `C`=Circle, `A`=Arc, `W`=Polyline, `T`=Text, `S`=Solid, `I`=Insert, `O`=Other.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_dxf_entity(e: &DxfEntity) -> String {
    match e {
        DxfEntity::Line { start, end, layer, unknown_group_codes } => {
            format!("L[{},{},{},{}]", enc_point3(start), enc_point3(end), enc_str(layer), enc_group_codes(unknown_group_codes))
        }
        DxfEntity::Circle { center, radius, layer, unknown_group_codes } => {
            format!("C[{},{},{},{}]", enc_point3(center), enc_f64(*radius), enc_str(layer), enc_group_codes(unknown_group_codes))
        }
        DxfEntity::Arc { center, radius, start_angle, end_angle, layer, unknown_group_codes } => {
            format!("A[{},{},{},{},{},{}]", enc_point3(center), enc_f64(*radius), enc_f64(*start_angle), enc_f64(*end_angle), enc_str(layer), enc_group_codes(unknown_group_codes))
        }
        DxfEntity::Polyline { vertices, closed, layer, unknown_group_codes } => format!("W[{},{},{},{}]", enc_vertices(vertices), if *closed { "1" } else { "0" }, enc_str(layer), enc_group_codes(unknown_group_codes)),
        DxfEntity::Text { position, height, value, layer, unknown_group_codes } => format!("T[{},{},{},{},{}]", enc_point3(position), enc_f64(*height), enc_str(value), enc_str(layer), enc_group_codes(unknown_group_codes)),
        DxfEntity::Solid { points, layer, unknown_group_codes } => {
            format!("S[{},{},{}]", enc_points4(points), enc_str(layer), enc_group_codes(unknown_group_codes))
        }
        DxfEntity::Insert { block_name, position, scale, rotation, layer, unknown_group_codes } => {
            format!("I[{},{},{},{},{},{}]", enc_str(block_name), enc_point3(position), enc_point3(scale), enc_f64(*rotation), enc_str(layer), enc_group_codes(unknown_group_codes))
        }
        DxfEntity::Other { kind, group_codes } => format!("O[{},{}]", enc_str(kind), enc_group_codes(group_codes)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_dxf_entity(s: &str) -> Result<DxfEntity, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "L" => {
            let parts = split_top_level(inner, ',');
            let [start, end, layer, unknown] = parts.as_slice() else { return Err(format!("entity line: expected 4 fields, got {}", parts.len())) };
            Ok(DxfEntity::Line { start: dec_point3(start)?, end: dec_point3(end)?, layer: dec_str(layer)?, unknown_group_codes: dec_group_codes(unknown)? })
        }
        "C" => {
            let parts = split_top_level(inner, ',');
            let [center, radius, layer, unknown] = parts.as_slice() else { return Err(format!("entity circle: expected 4 fields, got {}", parts.len())) };
            Ok(DxfEntity::Circle { center: dec_point3(center)?, radius: dec_f64(radius)?, layer: dec_str(layer)?, unknown_group_codes: dec_group_codes(unknown)? })
        }
        "A" => {
            let parts = split_top_level(inner, ',');
            let [center, radius, start_angle, end_angle, layer, unknown] = parts.as_slice() else { return Err(format!("entity arc: expected 6 fields, got {}", parts.len())) };
            Ok(DxfEntity::Arc { center: dec_point3(center)?, radius: dec_f64(radius)?, start_angle: dec_f64(start_angle)?, end_angle: dec_f64(end_angle)?, layer: dec_str(layer)?, unknown_group_codes: dec_group_codes(unknown)? })
        }
        "W" => {
            let parts = split_top_level(inner, ',');
            let [vertices, closed, layer, unknown] = parts.as_slice() else { return Err(format!("entity polyline: expected 4 fields, got {}", parts.len())) };
            Ok(DxfEntity::Polyline { vertices: dec_vertices(vertices)?, closed: *closed == "1", layer: dec_str(layer)?, unknown_group_codes: dec_group_codes(unknown)? })
        }
        "T" => {
            let parts = split_top_level(inner, ',');
            let [position, height, value, layer, unknown] = parts.as_slice() else { return Err(format!("entity text: expected 5 fields, got {}", parts.len())) };
            Ok(DxfEntity::Text { position: dec_point3(position)?, height: dec_f64(height)?, value: dec_str(value)?, layer: dec_str(layer)?, unknown_group_codes: dec_group_codes(unknown)? })
        }
        "S" => {
            let parts = split_top_level(inner, ',');
            let [points, layer, unknown] = parts.as_slice() else { return Err(format!("entity solid: expected 3 fields, got {}", parts.len())) };
            Ok(DxfEntity::Solid { points: dec_points4(points)?, layer: dec_str(layer)?, unknown_group_codes: dec_group_codes(unknown)? })
        }
        "I" => {
            let parts = split_top_level(inner, ',');
            let [block_name, position, scale, rotation, layer, unknown] = parts.as_slice() else { return Err(format!("entity insert: expected 6 fields, got {}", parts.len())) };
            Ok(DxfEntity::Insert { block_name: dec_str(block_name)?, position: dec_point3(position)?, scale: dec_point3(scale)?, rotation: dec_f64(rotation)?, layer: dec_str(layer)?, unknown_group_codes: dec_group_codes(unknown)? })
        }
        "O" => {
            let parts = split_top_level(inner, ',');
            let [kind, group_codes] = parts.as_slice() else { return Err(format!("entity other: expected 2 fields, got {}", parts.len())) };
            Ok(DxfEntity::Other { kind: dec_str(kind)?, group_codes: dec_group_codes(group_codes)? })
        }
        other => Err(format!("dxf entity: unknown tag {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_dxf_entities(es: &[DxfEntity]) -> String {
    enc_list(es, enc_dxf_entity)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_dxf_entities(s: &str) -> Result<Vec<DxfEntity>, String> {
    dec_list(s, dec_dxf_entity)
}
//#endregion 🔖️EntityValueCodecs

//#region 🔖️ItemCodecs
/// 🏷️ Full (non-diff) item encoders — self-bracketing positional tuples, used by `added` entries
/// in every collection triple AND by `🧬️mutations`'s `SetSnapshot`/`Insert*`/`Set*` argument
/// payloads (hence `pub(crate)`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_header_var(hv: &DxfHeaderVar) -> String {
    format!("[{},{},{},{}]", enc_str(&hv.name), hv.group_code, enc_dxf_value(&hv.value), enc_group_codes(&hv.extra_group_codes))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_header_var(s: &str) -> Result<DxfHeaderVar, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, group_code, value, extra] = parts.as_slice() else { return Err(format!("header var: expected 4 fields, got {}", parts.len())) };
    Ok(DxfHeaderVar { name: dec_str(name)?, group_code: group_code.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, value: dec_dxf_value(value)?, extra_group_codes: dec_group_codes(extra)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_layer(l: &DxfLayer) -> String {
    format!("[{},{},{},{},{}]", enc_str(&l.name), l.color, enc_str(&l.linetype), l.flags, enc_group_codes(&l.unknown_group_codes))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_layer(s: &str) -> Result<DxfLayer, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, color, linetype, flags, unknown] = parts.as_slice() else { return Err(format!("layer: expected 5 fields, got {}", parts.len())) };
    Ok(DxfLayer {
        name: dec_str(name)?,
        color: color.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
        linetype: dec_str(linetype)?,
        flags: flags.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
        unknown_group_codes: dec_group_codes(unknown)?,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_style(s: &DxfStyle) -> String {
    format!("[{},{},{},{}]", enc_str(&s.name), s.flags, enc_str(&s.font_name), enc_group_codes(&s.unknown_group_codes))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_style(s: &str) -> Result<DxfStyle, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, flags, font_name, unknown] = parts.as_slice() else { return Err(format!("style: expected 4 fields, got {}", parts.len())) };
    Ok(DxfStyle { name: dec_str(name)?, flags: flags.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, font_name: dec_str(font_name)?, unknown_group_codes: dec_group_codes(unknown)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_linetype(l: &DxfLinetype) -> String {
    format!("[{},{},{},{}]", enc_str(&l.name), l.flags, enc_str(&l.description), enc_group_codes(&l.unknown_group_codes))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_linetype(s: &str) -> Result<DxfLinetype, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, flags, description, unknown] = parts.as_slice() else { return Err(format!("linetype: expected 4 fields, got {}", parts.len())) };
    Ok(DxfLinetype { name: dec_str(name)?, flags: flags.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, description: dec_str(description)?, unknown_group_codes: dec_group_codes(unknown)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block(b: &DxfBlock) -> String {
    format!("[{},{},{},{}]", enc_str(&b.name), enc_point3(&b.base_point), enc_dxf_entities(&b.entities), enc_group_codes(&b.unknown_group_codes))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block(s: &str) -> Result<DxfBlock, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, base_point, entities, unknown] = parts.as_slice() else { return Err(format!("block: expected 4 fields, got {}", parts.len())) };
    Ok(DxfBlock { name: dec_str(name)?, base_point: dec_point3(base_point)?, entities: dec_dxf_entities(entities)?, unknown_group_codes: dec_group_codes(unknown)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_dxf_tag(t: &DxfTag) -> String {
    format!("[{},{}]", t.code, enc_str(&t.value))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_dxf_tag(s: &str) -> Result<DxfTag, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [code, value] = parts.as_slice() else { return Err(format!("tag: expected 2 fields, got {}", parts.len())) };
    Ok(DxfTag { code: code.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, value: dec_str(value)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_other_table(t: &DxfOtherTable) -> String {
    format!("[{},{}]", enc_str(&t.name), enc_list(&t.tags, enc_dxf_tag))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_other_table(s: &str) -> Result<DxfOtherTable, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, tags] = parts.as_slice() else { return Err(format!("other table: expected 2 fields, got {}", parts.len())) };
    Ok(DxfOtherTable { name: dec_str(name)?, tags: dec_list(tags, dec_dxf_tag)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_dxf_tables(t: &DxfTables) -> String {
    format!("[{},{},{}]", enc_list(&t.layers, enc_layer), enc_list(&t.styles, enc_style), enc_list(&t.linetypes, enc_linetype))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_dxf_tables(s: &str) -> Result<DxfTables, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [layers, styles, linetypes] = parts.as_slice() else { return Err(format!("tables: expected 3 fields, got {}", parts.len())) };
    Ok(DxfTables { layers: dec_list(layers, dec_layer)?, styles: dec_list(styles, dec_style)?, linetypes: dec_list(linetypes, dec_linetype)? })
}
/// 🧬️ Whole `DxfSnapshot` — needed by `🧬️mutations::DxfMutation::SetSnapshot`'s `OpText`/
/// `OpBinary` payload (§3a's mutation-side blocker: `SetSnapshot` always carries the whole
/// snapshot, so this grammar is exercised by the mutation codec even though `DxfDiff` never
/// embeds a full snapshot itself).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_dxf_snapshot(s: &DxfSnapshot) -> String {
    format!("[{},{},{},{},{},{}]", enc_str(&s.schema), enc_list(&s.header_vars, enc_header_var), enc_dxf_tables(&s.tables), enc_list(&s.other_tables, enc_other_table), enc_list(&s.blocks, enc_block), enc_dxf_entities(&s.entities),)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_dxf_snapshot(s: &str) -> Result<DxfSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, header_vars, tables, other_tables, blocks, entities] = parts.as_slice() else {
        return Err(format!("snapshot: expected 6 fields, got {}", parts.len()));
    };
    Ok(DxfSnapshot {
        schema: dec_str(schema)?,
        header_vars: dec_list(header_vars, dec_header_var)?,
        tables: dec_dxf_tables(tables)?,
        other_tables: dec_list(other_tables, dec_other_table)?,
        blocks: dec_list(blocks, dec_block)?,
        entities: dec_dxf_entities(entities)?,
    })
}
//#endregion 🔖️ItemCodecs

//#region 🔖️DiffValueCodecs
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_header_var_diff(d: &DxfHeaderVarDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.group_code, |v| v.to_string()), encode_option(&d.value, enc_dxf_value), encode_option(&d.extra_group_codes, |v| enc_group_codes(v)),)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_header_var_diff(s: &str) -> Result<DxfHeaderVarDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [group_code, value, extra] = parts.as_slice() else { return Err(format!("header var diff: expected 3 fields, got {}", parts.len())) };
    Ok(DxfHeaderVarDiff { group_code: decode_option(group_code, |v| v.parse().map_err(|e: std::num::ParseIntError| e.to_string()))?, value: decode_option(value, dec_dxf_value)?, extra_group_codes: decode_option(extra, dec_group_codes)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_layer_diff(d: &DxfLayerDiff) -> String {
    format!("[{},{},{},{}]", encode_option(&d.color, |v| v.to_string()), encode_option(&d.linetype, |v| enc_str(v)), encode_option(&d.flags, |v| v.to_string()), encode_option(&d.unknown_group_codes, |v| enc_group_codes(v)),)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_layer_diff(s: &str) -> Result<DxfLayerDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [color, linetype, flags, unknown] = parts.as_slice() else { return Err(format!("layer diff: expected 4 fields, got {}", parts.len())) };
    Ok(DxfLayerDiff {
        color: decode_option(color, |v| v.parse().map_err(|e: std::num::ParseIntError| e.to_string()))?,
        linetype: decode_option(linetype, dec_str)?,
        flags: decode_option(flags, |v| v.parse().map_err(|e: std::num::ParseIntError| e.to_string()))?,
        unknown_group_codes: decode_option(unknown, dec_group_codes)?,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_style_diff(d: &DxfStyleDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.flags, |v| v.to_string()), encode_option(&d.font_name, |v| enc_str(v)), encode_option(&d.unknown_group_codes, |v| enc_group_codes(v)),)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_style_diff(s: &str) -> Result<DxfStyleDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [flags, font_name, unknown] = parts.as_slice() else { return Err(format!("style diff: expected 3 fields, got {}", parts.len())) };
    Ok(DxfStyleDiff { flags: decode_option(flags, |v| v.parse().map_err(|e: std::num::ParseIntError| e.to_string()))?, font_name: decode_option(font_name, dec_str)?, unknown_group_codes: decode_option(unknown, dec_group_codes)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_linetype_diff(d: &DxfLinetypeDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.flags, |v| v.to_string()), encode_option(&d.description, |v| enc_str(v)), encode_option(&d.unknown_group_codes, |v| enc_group_codes(v)),)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_linetype_diff(s: &str) -> Result<DxfLinetypeDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [flags, description, unknown] = parts.as_slice() else { return Err(format!("linetype diff: expected 3 fields, got {}", parts.len())) };
    Ok(DxfLinetypeDiff { flags: decode_option(flags, |v| v.parse().map_err(|e: std::num::ParseIntError| e.to_string()))?, description: decode_option(description, dec_str)?, unknown_group_codes: decode_option(unknown, dec_group_codes)? })
}

/// 🌳 `DxfEntityDiff` itself needs a tag (like `SvgNodeDiff`/`XmlNode`) since it appears standalone
/// at the `entities=`/`blocks=`'s nested `modified` entry position, not always inside an
/// already-disambiguating container. `R`=Replace (carries a WHOLE `DxfEntity`, itself tag-prefixed
/// — a tag-inside-a-tag, unambiguous since the outer bracket always closes the inner one first),
/// else one letter per typed entity kind (same letters `enc_dxf_entity` uses).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_entity_diff(d: &DxfEntityDiff) -> String {
    match d {
        DxfEntityDiff::Replace { entity } => format!("R[{}]", enc_dxf_entity(entity)),
        DxfEntityDiff::Line(x) => format!("L[{},{},{},{}]", encode_option(&x.start, enc_point3), encode_option(&x.end, enc_point3), encode_option(&x.layer, |v| enc_str(v)), encode_option(&x.unknown_group_codes, |v| enc_group_codes(v))),
        DxfEntityDiff::Circle(x) => format!("C[{},{},{},{}]", encode_option(&x.center, enc_point3), encode_option(&x.radius, |v| enc_f64(*v)), encode_option(&x.layer, |v| enc_str(v)), encode_option(&x.unknown_group_codes, |v| enc_group_codes(v))),
        DxfEntityDiff::Arc(x) => format!(
            "A[{},{},{},{},{},{}]",
            encode_option(&x.center, enc_point3),
            encode_option(&x.radius, |v| enc_f64(*v)),
            encode_option(&x.start_angle, |v| enc_f64(*v)),
            encode_option(&x.end_angle, |v| enc_f64(*v)),
            encode_option(&x.layer, |v| enc_str(v)),
            encode_option(&x.unknown_group_codes, |v| enc_group_codes(v))
        ),
        DxfEntityDiff::Polyline(x) => format!(
            "W[{},{},{},{}]",
            encode_option(&x.vertices, |v| enc_vertices(v)),
            encode_option(&x.closed, |v| if *v { "1".to_string() } else { "0".to_string() }),
            encode_option(&x.layer, |v| enc_str(v)),
            encode_option(&x.unknown_group_codes, |v| enc_group_codes(v))
        ),
        DxfEntityDiff::Text(x) => format!(
            "T[{},{},{},{},{}]",
            encode_option(&x.position, enc_point3),
            encode_option(&x.height, |v| enc_f64(*v)),
            encode_option(&x.value, |v| enc_str(v)),
            encode_option(&x.layer, |v| enc_str(v)),
            encode_option(&x.unknown_group_codes, |v| enc_group_codes(v))
        ),
        DxfEntityDiff::Solid(x) => format!("S[{},{},{}]", encode_option(&x.points, enc_points4), encode_option(&x.layer, |v| enc_str(v)), encode_option(&x.unknown_group_codes, |v| enc_group_codes(v))),
        DxfEntityDiff::Insert(x) => format!(
            "I[{},{},{},{},{},{}]",
            encode_option(&x.block_name, |v| enc_str(v)),
            encode_option(&x.position, enc_point3),
            encode_option(&x.scale, enc_point3),
            encode_option(&x.rotation, |v| enc_f64(*v)),
            encode_option(&x.layer, |v| enc_str(v)),
            encode_option(&x.unknown_group_codes, |v| enc_group_codes(v))
        ),
        DxfEntityDiff::Other(x) => format!("O[{}]", encode_option(&x.group_codes, |v| enc_group_codes(v))),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_entity_diff(s: &str) -> Result<DxfEntityDiff, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "R" => Ok(DxfEntityDiff::Replace { entity: dec_dxf_entity(inner)? }),
        "L" => {
            let parts = split_top_level(inner, ',');
            let [start, end, layer, unknown] = parts.as_slice() else { return Err(format!("line diff: expected 4 fields, got {}", parts.len())) };
            Ok(DxfEntityDiff::Line(DxfLineDiff { start: decode_option(start, dec_point3)?, end: decode_option(end, dec_point3)?, layer: decode_option(layer, dec_str)?, unknown_group_codes: decode_option(unknown, dec_group_codes)? }))
        }
        "C" => {
            let parts = split_top_level(inner, ',');
            let [center, radius, layer, unknown] = parts.as_slice() else { return Err(format!("circle diff: expected 4 fields, got {}", parts.len())) };
            Ok(DxfEntityDiff::Circle(DxfCircleDiff { center: decode_option(center, dec_point3)?, radius: decode_option(radius, dec_f64)?, layer: decode_option(layer, dec_str)?, unknown_group_codes: decode_option(unknown, dec_group_codes)? }))
        }
        "A" => {
            let parts = split_top_level(inner, ',');
            let [center, radius, start_angle, end_angle, layer, unknown] = parts.as_slice() else { return Err(format!("arc diff: expected 6 fields, got {}", parts.len())) };
            Ok(DxfEntityDiff::Arc(DxfArcDiff {
                center: decode_option(center, dec_point3)?,
                radius: decode_option(radius, dec_f64)?,
                start_angle: decode_option(start_angle, dec_f64)?,
                end_angle: decode_option(end_angle, dec_f64)?,
                layer: decode_option(layer, dec_str)?,
                unknown_group_codes: decode_option(unknown, dec_group_codes)?,
            }))
        }
        "W" => {
            let parts = split_top_level(inner, ',');
            let [vertices, closed, layer, unknown] = parts.as_slice() else { return Err(format!("polyline diff: expected 4 fields, got {}", parts.len())) };
            Ok(DxfEntityDiff::Polyline(DxfPolylineDiff {
                vertices: decode_option(vertices, dec_vertices)?,
                closed: decode_option(closed, |v| Ok(v == "1"))?,
                layer: decode_option(layer, dec_str)?,
                unknown_group_codes: decode_option(unknown, dec_group_codes)?,
            }))
        }
        "T" => {
            let parts = split_top_level(inner, ',');
            let [position, height, value, layer, unknown] = parts.as_slice() else { return Err(format!("text diff: expected 5 fields, got {}", parts.len())) };
            Ok(DxfEntityDiff::Text(DxfTextDiff {
                position: decode_option(position, dec_point3)?,
                height: decode_option(height, dec_f64)?,
                value: decode_option(value, dec_str)?,
                layer: decode_option(layer, dec_str)?,
                unknown_group_codes: decode_option(unknown, dec_group_codes)?,
            }))
        }
        "S" => {
            let parts = split_top_level(inner, ',');
            let [points, layer, unknown] = parts.as_slice() else { return Err(format!("solid diff: expected 3 fields, got {}", parts.len())) };
            Ok(DxfEntityDiff::Solid(DxfSolidDiff { points: decode_option(points, dec_points4)?, layer: decode_option(layer, dec_str)?, unknown_group_codes: decode_option(unknown, dec_group_codes)? }))
        }
        "I" => {
            let parts = split_top_level(inner, ',');
            let [block_name, position, scale, rotation, layer, unknown] = parts.as_slice() else { return Err(format!("insert diff: expected 6 fields, got {}", parts.len())) };
            Ok(DxfEntityDiff::Insert(DxfInsertDiff {
                block_name: decode_option(block_name, dec_str)?,
                position: decode_option(position, dec_point3)?,
                scale: decode_option(scale, dec_point3)?,
                rotation: decode_option(rotation, dec_f64)?,
                layer: decode_option(layer, dec_str)?,
                unknown_group_codes: decode_option(unknown, dec_group_codes)?,
            }))
        }
        "O" => Ok(DxfEntityDiff::Other(DxfOtherDiff { group_codes: decode_option(inner, dec_group_codes)? })),
        other => Err(format!("entity diff: unknown tag {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_block_diff(d: &DxfBlockDiff) -> String {
    format!("[{},{},{},{}]", encode_option(&d.name, |v| enc_str(v)), encode_option(&d.base_point, enc_point3), encode_option(&d.entities, enc_entities_diff), encode_option(&d.unknown_group_codes, |v| enc_group_codes(v)),)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_block_diff(s: &str) -> Result<DxfBlockDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, base_point, entities, unknown] = parts.as_slice() else { return Err(format!("block diff: expected 4 fields, got {}", parts.len())) };
    Ok(DxfBlockDiff { name: decode_option(name, dec_str)?, base_point: decode_option(base_point, dec_point3)?, entities: decode_option(entities, dec_entities_diff)?, unknown_group_codes: decode_option(unknown, dec_group_codes)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_tables_diff(t: &DxfTablesDiff) -> String {
    format!("[{},{},{}]", encode_option(&t.layers, enc_layers_diff), encode_option(&t.styles, enc_styles_diff), encode_option(&t.linetypes, enc_linetypes_diff),)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_tables_diff(s: &str) -> Result<DxfTablesDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [layers, styles, linetypes] = parts.as_slice() else { return Err(format!("tables diff: expected 3 fields, got {}", parts.len())) };
    Ok(DxfTablesDiff { layers: decode_option(layers, dec_layers_diff)?, styles: decode_option(styles, dec_styles_diff)?, linetypes: decode_option(linetypes, dec_linetypes_diff)? })
}
//#endregion 🔖️DiffValueCodecs

//#region 🔖️CollectionTripleCodecs
/// 🧮 Generic name-keyed `[removed];[modified];[added]` core — mirrors this file's own
/// `DxfNamedElem`/`named_between` generic core above, one level up (string grammar instead of
/// structural diff algebra). `modified` entries are `hexname:diffpayload`, `added` entries are
/// `index:fullitempayload` (recipe's own convention — the index is always a bare decimal
/// preceding the first colon, unambiguous since no payload anywhere in this grammar emits a
/// literal `:`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_name_triple<T, D>(removed: &[String], modified: &[(String, D)], added: &[(usize, T)], enc_diff: impl Fn(&D) -> String, enc_item: impl Fn(&T) -> String) -> String {
    let removed_s = removed.iter().map(|n| enc_str(n)).collect::<Vec<_>>().join(",");
    let modified_s = modified.iter().map(|(n, d)| format!("{}:{}", enc_str(n), enc_diff(d))).collect::<Vec<_>>().join(",");
    let added_s = added.iter().map(|(i, t)| format!("{i}:{}", enc_item(t))).collect::<Vec<_>>().join(",");
    format!("[{removed_s}];[{modified_s}];[{added_s}]")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_name_triple<T, D>(body: &str, dec_diff: impl Fn(&str) -> Result<D, String>, dec_item: impl Fn(&str) -> Result<T, String>) -> Result<(Vec<String>, Vec<(String, D)>, Vec<(usize, T)>), String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("name triple: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_str).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (name, rest) = entry.split_once(':').ok_or_else(|| format!("name triple modified: bad entry {entry:?}"))?;
            Ok((dec_str(name)?, dec_diff(rest)?))
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("name triple added: bad entry {entry:?}"))?;
            Ok((parse_usize(idx)?, dec_item(rest)?))
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok((removed, modified, added))
}
/// 🧮 Generic index-keyed twin of [`enc_name_triple`]/[`dec_name_triple`] — used by
/// `entities`/`blocks` (both top-level and, for `entities`, nested inside a block diff).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_index_triple<T, D>(removed: &[usize], modified: &[(usize, D)], added: &[(usize, T)], enc_diff: impl Fn(&D) -> String, enc_item: impl Fn(&T) -> String) -> String {
    let removed_s = removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified_s = modified.iter().map(|(i, d)| format!("{i}:{}", enc_diff(d))).collect::<Vec<_>>().join(",");
    let added_s = added.iter().map(|(i, t)| format!("{i}:{}", enc_item(t))).collect::<Vec<_>>().join(",");
    format!("[{removed_s}];[{modified_s}];[{added_s}]")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_index_triple<T, D>(body: &str, dec_diff: impl Fn(&str) -> Result<D, String>, dec_item: impl Fn(&str) -> Result<T, String>) -> Result<(Vec<usize>, Vec<(usize, D)>, Vec<(usize, T)>), String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("index triple: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("index triple modified: bad entry {entry:?}"))?;
            Ok((parse_usize(idx)?, dec_diff(rest)?))
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("index triple added: bad entry {entry:?}"))?;
            Ok((parse_usize(idx)?, dec_item(rest)?))
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok((removed, modified, added))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_header_vars_diff(d: &DxfHeaderVarsDiff) -> String {
    let modified: Vec<(String, DxfHeaderVarDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
    let added: Vec<(usize, DxfHeaderVar)> = d.added.iter().map(|a| (a.index, a.header_var.clone())).collect();
    enc_name_triple(&d.removed, &modified, &added, enc_header_var_diff, enc_header_var)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_header_vars_diff(s: &str) -> Result<DxfHeaderVarsDiff, String> {
    let (removed, modified, added) = dec_name_triple(s, dec_header_var_diff, dec_header_var)?;
    Ok(DxfHeaderVarsDiff { removed, modified: modified.into_iter().map(|(name, diff)| DxfHeaderVarModified { name, diff }).collect(), added: added.into_iter().map(|(index, header_var)| DxfHeaderVarAdded { index, header_var }).collect() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_layers_diff(d: &DxfLayersDiff) -> String {
    let modified: Vec<(String, DxfLayerDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
    let added: Vec<(usize, DxfLayer)> = d.added.iter().map(|a| (a.index, a.layer.clone())).collect();
    enc_name_triple(&d.removed, &modified, &added, enc_layer_diff, enc_layer)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_layers_diff(s: &str) -> Result<DxfLayersDiff, String> {
    let (removed, modified, added) = dec_name_triple(s, dec_layer_diff, dec_layer)?;
    Ok(DxfLayersDiff { removed, modified: modified.into_iter().map(|(name, diff)| DxfLayerModified { name, diff }).collect(), added: added.into_iter().map(|(index, layer)| DxfLayerAdded { index, layer }).collect() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_styles_diff(d: &DxfStylesDiff) -> String {
    let modified: Vec<(String, DxfStyleDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
    let added: Vec<(usize, DxfStyle)> = d.added.iter().map(|a| (a.index, a.style.clone())).collect();
    enc_name_triple(&d.removed, &modified, &added, enc_style_diff, enc_style)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_styles_diff(s: &str) -> Result<DxfStylesDiff, String> {
    let (removed, modified, added) = dec_name_triple(s, dec_style_diff, dec_style)?;
    Ok(DxfStylesDiff { removed, modified: modified.into_iter().map(|(name, diff)| DxfStyleModified { name, diff }).collect(), added: added.into_iter().map(|(index, style)| DxfStyleAdded { index, style }).collect() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_linetypes_diff(d: &DxfLinetypesDiff) -> String {
    let modified: Vec<(String, DxfLinetypeDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
    let added: Vec<(usize, DxfLinetype)> = d.added.iter().map(|a| (a.index, a.linetype.clone())).collect();
    enc_name_triple(&d.removed, &modified, &added, enc_linetype_diff, enc_linetype)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_linetypes_diff(s: &str) -> Result<DxfLinetypesDiff, String> {
    let (removed, modified, added) = dec_name_triple(s, dec_linetype_diff, dec_linetype)?;
    Ok(DxfLinetypesDiff { removed, modified: modified.into_iter().map(|(name, diff)| DxfLinetypeModified { name, diff }).collect(), added: added.into_iter().map(|(index, linetype)| DxfLinetypeAdded { index, linetype }).collect() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_entities_diff(d: &DxfEntitiesDiff) -> String {
    let modified: Vec<(usize, DxfEntityDiff)> = d.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
    let added: Vec<(usize, DxfEntity)> = d.added.iter().map(|a| (a.index, a.entity.clone())).collect();
    enc_index_triple(&d.removed, &modified, &added, enc_entity_diff, enc_dxf_entity)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_entities_diff(s: &str) -> Result<DxfEntitiesDiff, String> {
    let (removed, modified, added) = dec_index_triple(s, dec_entity_diff, dec_dxf_entity)?;
    Ok(DxfEntitiesDiff { removed, modified: modified.into_iter().map(|(index, diff)| DxfEntityModified { index, diff }).collect(), added: added.into_iter().map(|(index, entity)| DxfEntityAdded { index, entity }).collect() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_blocks_diff(d: &DxfBlocksDiff) -> String {
    let modified: Vec<(usize, DxfBlockDiff)> = d.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
    let added: Vec<(usize, DxfBlock)> = d.added.iter().map(|a| (a.index, a.block.clone())).collect();
    enc_index_triple(&d.removed, &modified, &added, enc_block_diff, enc_block)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_blocks_diff(s: &str) -> Result<DxfBlocksDiff, String> {
    let (removed, modified, added) = dec_index_triple(s, dec_block_diff, dec_block)?;
    Ok(DxfBlocksDiff { removed, modified: modified.into_iter().map(|(index, diff)| DxfBlockModified { index, diff }).collect(), added: added.into_iter().map(|(index, block)| DxfBlockAdded { index, block }).collect() })
}
//#endregion 🔖️CollectionTripleCodecs

//#region 🔖️BinaryPrimitives
/// 🧪️ P2-FG1: real LEB128-varint-framed binary primitives (length-prefixed bytes/utf8, tri-state
/// `Option<T>` presence byte) backing the upgraded `OpBinary`/`DiffCodec` frames below — reuses
/// `store::write_varint_u64`/`store::write_varint_i64`/`store::ByteReader` (crate-root re-exports
/// of `os_pack`'s own varint/reader primitives — `store`/`dsl`/`protocol` are all `extern crate
/// self as …` aliases for the SAME kernel crate, see `📦️glue.rs`) rather than reinventing varint
/// encode/decode. Same shape `stdio.json`'s own upgrade introduced
/// (`🔣️json/…/🔺️diff/🦀️component.rs`'s `#region 🔖️BinaryPrimitives`).
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
pub(crate) fn write_option_bin<T>(out: &mut Vec<u8>, opt: &Option<T>, enc: impl Fn(&T, &mut Vec<u8>)) {
    match opt {
        None => out.push(0),
        Some(v) => {
            out.push(1);
            enc(v, out);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_option_bin<T>(reader: &mut store::ByteReader<'_>, dec: impl Fn(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<Option<T>, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(None),
        1 => Ok(Some(dec(reader)?)),
        other => Err(format!("option binary: bad tag {other}")),
    }
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️ItemBinaryCodecs
/// 🧪️ P2-FG1: real recursive binary twins of every `enc_*`/`dec_*` item-literal codec above
/// (`#region 🔖️GeometryCodecs`/`#region 🔖️ValueCodecs`/`#region 🔖️EntityValueCodecs`/
/// `#region 🔖️ItemCodecs`) — backs the upgraded `OpBinary` frame
/// (`../🧬️mutations/🦀️component.rs`) and every collection's `added`-entry / `entity-diff`'s
/// `Replace`-arm payload below. Tag numbering mirrors each text codec's own letter-tag order
/// (`0`=first letter, `1`=second, …), independent per type (not reused across unrelated shapes).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_f64_bin(out: &mut Vec<u8>, v: f64) {
    out.extend_from_slice(&v.to_le_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_f64_bin(reader: &mut store::ByteReader<'_>) -> Result<f64, String> {
    reader.read_f64_le().map_err(|e| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_point3_bin(out: &mut Vec<u8>, p: &[f64; 3]) {
    write_f64_bin(out, p[0]);
    write_f64_bin(out, p[1]);
    write_f64_bin(out, p[2]);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_point3_bin(reader: &mut store::ByteReader<'_>) -> Result<[f64; 3], String> {
    Ok([read_f64_bin(reader)?, read_f64_bin(reader)?, read_f64_bin(reader)?])
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_points4_bin(out: &mut Vec<u8>, p: &[[f64; 3]; 4]) {
    for pt in p {
        write_point3_bin(out, pt);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_points4_bin(reader: &mut store::ByteReader<'_>) -> Result<[[f64; 3]; 4], String> {
    Ok([read_point3_bin(reader)?, read_point3_bin(reader)?, read_point3_bin(reader)?, read_point3_bin(reader)?])
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_dxf_value_bin(v: &DxfValue, out: &mut Vec<u8>) {
    match v {
        DxfValue::Str { value } => {
            out.push(0);
            write_str_lp(out, value);
        }
        DxfValue::Int { value } => {
            out.push(1);
            store::write_varint_i64(out, *value);
        }
        DxfValue::Double { value } => {
            out.push(2);
            write_f64_bin(out, *value);
        }
        DxfValue::Point { value } => {
            out.push(3);
            write_point3_bin(out, value);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_dxf_value_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfValue, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(DxfValue::Str { value: read_str_lp(reader)? }),
        1 => Ok(DxfValue::Int { value: reader.read_varint_i64().map_err(|e| e.to_string())? }),
        2 => Ok(DxfValue::Double { value: read_f64_bin(reader)? }),
        3 => Ok(DxfValue::Point { value: read_point3_bin(reader)? }),
        other => Err(format!("dxf value binary: unknown tag {other}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_group_code_bin(pair: &(i32, DxfValue), out: &mut Vec<u8>) {
    store::write_varint_i64(out, pair.0 as i64);
    enc_dxf_value_bin(&pair.1, out);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_group_code_bin(reader: &mut store::ByteReader<'_>) -> Result<(i32, DxfValue), String> {
    let code = reader.read_varint_i64().map_err(|e| e.to_string())? as i32;
    let value = dec_dxf_value_bin(reader)?;
    Ok((code, value))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_group_codes_bin(v: &[(i32, DxfValue)], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, v.len() as u64);
    for pair in v {
        enc_group_code_bin(pair, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_group_codes_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<(i32, DxfValue)>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(dec_group_code_bin(reader)?);
    }
    Ok(out)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_vertex_bin(v: &DxfVertex, out: &mut Vec<u8>) {
    write_f64_bin(out, v.x);
    write_f64_bin(out, v.y);
    write_f64_bin(out, v.z);
    write_f64_bin(out, v.bulge);
    enc_group_codes_bin(&v.unknown_group_codes, out);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_vertex_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfVertex, String> {
    let x = read_f64_bin(reader)?;
    let y = read_f64_bin(reader)?;
    let z = read_f64_bin(reader)?;
    let bulge = read_f64_bin(reader)?;
    let unknown_group_codes = dec_group_codes_bin(reader)?;
    Ok(DxfVertex { x, y, z, bulge, unknown_group_codes })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_vertices_bin(vs: &[DxfVertex], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, vs.len() as u64);
    for v in vs {
        enc_vertex_bin(v, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_vertices_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<DxfVertex>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(dec_vertex_bin(reader)?);
    }
    Ok(out)
}

/// 📐️ `DxfEntity` binary twin of [`enc_dxf_entity`]/[`dec_dxf_entity`] — tag `0`=Line,`1`=Circle,
/// `2`=Arc,`3`=Polyline,`4`=Text,`5`=Solid,`6`=Insert,`7`=Other (same order `enc_dxf_entity`'s own
/// L/C/A/W/T/S/I/O letters appear in).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_dxf_entity_bin(e: &DxfEntity, out: &mut Vec<u8>) {
    match e {
        DxfEntity::Line { start, end, layer, unknown_group_codes } => {
            out.push(0);
            write_point3_bin(out, start);
            write_point3_bin(out, end);
            write_str_lp(out, layer);
            enc_group_codes_bin(unknown_group_codes, out);
        }
        DxfEntity::Circle { center, radius, layer, unknown_group_codes } => {
            out.push(1);
            write_point3_bin(out, center);
            write_f64_bin(out, *radius);
            write_str_lp(out, layer);
            enc_group_codes_bin(unknown_group_codes, out);
        }
        DxfEntity::Arc { center, radius, start_angle, end_angle, layer, unknown_group_codes } => {
            out.push(2);
            write_point3_bin(out, center);
            write_f64_bin(out, *radius);
            write_f64_bin(out, *start_angle);
            write_f64_bin(out, *end_angle);
            write_str_lp(out, layer);
            enc_group_codes_bin(unknown_group_codes, out);
        }
        DxfEntity::Polyline { vertices, closed, layer, unknown_group_codes } => {
            out.push(3);
            enc_vertices_bin(vertices, out);
            out.push(if *closed { 1 } else { 0 });
            write_str_lp(out, layer);
            enc_group_codes_bin(unknown_group_codes, out);
        }
        DxfEntity::Text { position, height, value, layer, unknown_group_codes } => {
            out.push(4);
            write_point3_bin(out, position);
            write_f64_bin(out, *height);
            write_str_lp(out, value);
            write_str_lp(out, layer);
            enc_group_codes_bin(unknown_group_codes, out);
        }
        DxfEntity::Solid { points, layer, unknown_group_codes } => {
            out.push(5);
            write_points4_bin(out, points);
            write_str_lp(out, layer);
            enc_group_codes_bin(unknown_group_codes, out);
        }
        DxfEntity::Insert { block_name, position, scale, rotation, layer, unknown_group_codes } => {
            out.push(6);
            write_str_lp(out, block_name);
            write_point3_bin(out, position);
            write_point3_bin(out, scale);
            write_f64_bin(out, *rotation);
            write_str_lp(out, layer);
            enc_group_codes_bin(unknown_group_codes, out);
        }
        DxfEntity::Other { kind, group_codes } => {
            out.push(7);
            write_str_lp(out, kind);
            enc_group_codes_bin(group_codes, out);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_dxf_entity_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfEntity, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => {
            let start = read_point3_bin(reader)?;
            let end = read_point3_bin(reader)?;
            let layer = read_str_lp(reader)?;
            let unknown_group_codes = dec_group_codes_bin(reader)?;
            Ok(DxfEntity::Line { start, end, layer, unknown_group_codes })
        }
        1 => {
            let center = read_point3_bin(reader)?;
            let radius = read_f64_bin(reader)?;
            let layer = read_str_lp(reader)?;
            let unknown_group_codes = dec_group_codes_bin(reader)?;
            Ok(DxfEntity::Circle { center, radius, layer, unknown_group_codes })
        }
        2 => {
            let center = read_point3_bin(reader)?;
            let radius = read_f64_bin(reader)?;
            let start_angle = read_f64_bin(reader)?;
            let end_angle = read_f64_bin(reader)?;
            let layer = read_str_lp(reader)?;
            let unknown_group_codes = dec_group_codes_bin(reader)?;
            Ok(DxfEntity::Arc { center, radius, start_angle, end_angle, layer, unknown_group_codes })
        }
        3 => {
            let vertices = dec_vertices_bin(reader)?;
            let closed = reader.read_u8().map_err(|e| e.to_string())? != 0;
            let layer = read_str_lp(reader)?;
            let unknown_group_codes = dec_group_codes_bin(reader)?;
            Ok(DxfEntity::Polyline { vertices, closed, layer, unknown_group_codes })
        }
        4 => {
            let position = read_point3_bin(reader)?;
            let height = read_f64_bin(reader)?;
            let value = read_str_lp(reader)?;
            let layer = read_str_lp(reader)?;
            let unknown_group_codes = dec_group_codes_bin(reader)?;
            Ok(DxfEntity::Text { position, height, value, layer, unknown_group_codes })
        }
        5 => {
            let points = read_points4_bin(reader)?;
            let layer = read_str_lp(reader)?;
            let unknown_group_codes = dec_group_codes_bin(reader)?;
            Ok(DxfEntity::Solid { points, layer, unknown_group_codes })
        }
        6 => {
            let block_name = read_str_lp(reader)?;
            let position = read_point3_bin(reader)?;
            let scale = read_point3_bin(reader)?;
            let rotation = read_f64_bin(reader)?;
            let layer = read_str_lp(reader)?;
            let unknown_group_codes = dec_group_codes_bin(reader)?;
            Ok(DxfEntity::Insert { block_name, position, scale, rotation, layer, unknown_group_codes })
        }
        7 => {
            let kind = read_str_lp(reader)?;
            let group_codes = dec_group_codes_bin(reader)?;
            Ok(DxfEntity::Other { kind, group_codes })
        }
        other => Err(format!("dxf entity binary: unknown tag {other}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_dxf_entities_bin(es: &[DxfEntity], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, es.len() as u64);
    for e in es {
        enc_dxf_entity_bin(e, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_dxf_entities_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<DxfEntity>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(dec_dxf_entity_bin(reader)?);
    }
    Ok(out)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_header_var_bin(hv: &DxfHeaderVar, out: &mut Vec<u8>) {
    write_str_lp(out, &hv.name);
    store::write_varint_i64(out, hv.group_code as i64);
    enc_dxf_value_bin(&hv.value, out);
    enc_group_codes_bin(&hv.extra_group_codes, out);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_header_var_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfHeaderVar, String> {
    let name = read_str_lp(reader)?;
    let group_code = reader.read_varint_i64().map_err(|e| e.to_string())? as i32;
    let value = dec_dxf_value_bin(reader)?;
    let extra_group_codes = dec_group_codes_bin(reader)?;
    Ok(DxfHeaderVar { name, group_code, value, extra_group_codes })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_layer_bin(l: &DxfLayer, out: &mut Vec<u8>) {
    write_str_lp(out, &l.name);
    store::write_varint_i64(out, l.color as i64);
    write_str_lp(out, &l.linetype);
    store::write_varint_i64(out, l.flags as i64);
    enc_group_codes_bin(&l.unknown_group_codes, out);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_layer_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfLayer, String> {
    let name = read_str_lp(reader)?;
    let color = reader.read_varint_i64().map_err(|e| e.to_string())? as i32;
    let linetype = read_str_lp(reader)?;
    let flags = reader.read_varint_i64().map_err(|e| e.to_string())? as i32;
    let unknown_group_codes = dec_group_codes_bin(reader)?;
    Ok(DxfLayer { name, color, linetype, flags, unknown_group_codes })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_style_bin(s: &DxfStyle, out: &mut Vec<u8>) {
    write_str_lp(out, &s.name);
    store::write_varint_i64(out, s.flags as i64);
    write_str_lp(out, &s.font_name);
    enc_group_codes_bin(&s.unknown_group_codes, out);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_style_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfStyle, String> {
    let name = read_str_lp(reader)?;
    let flags = reader.read_varint_i64().map_err(|e| e.to_string())? as i32;
    let font_name = read_str_lp(reader)?;
    let unknown_group_codes = dec_group_codes_bin(reader)?;
    Ok(DxfStyle { name, flags, font_name, unknown_group_codes })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_linetype_bin(l: &DxfLinetype, out: &mut Vec<u8>) {
    write_str_lp(out, &l.name);
    store::write_varint_i64(out, l.flags as i64);
    write_str_lp(out, &l.description);
    enc_group_codes_bin(&l.unknown_group_codes, out);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_linetype_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfLinetype, String> {
    let name = read_str_lp(reader)?;
    let flags = reader.read_varint_i64().map_err(|e| e.to_string())? as i32;
    let description = read_str_lp(reader)?;
    let unknown_group_codes = dec_group_codes_bin(reader)?;
    Ok(DxfLinetype { name, flags, description, unknown_group_codes })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block_bin(b: &DxfBlock, out: &mut Vec<u8>) {
    write_str_lp(out, &b.name);
    write_point3_bin(out, &b.base_point);
    enc_dxf_entities_bin(&b.entities, out);
    enc_group_codes_bin(&b.unknown_group_codes, out);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfBlock, String> {
    let name = read_str_lp(reader)?;
    let base_point = read_point3_bin(reader)?;
    let entities = dec_dxf_entities_bin(reader)?;
    let unknown_group_codes = dec_group_codes_bin(reader)?;
    Ok(DxfBlock { name, base_point, entities, unknown_group_codes })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_dxf_tag_bin(t: &DxfTag, out: &mut Vec<u8>) {
    store::write_varint_i64(out, t.code as i64);
    write_str_lp(out, &t.value);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_dxf_tag_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfTag, String> {
    let code = reader.read_varint_i64().map_err(|e| e.to_string())? as i32;
    let value = read_str_lp(reader)?;
    Ok(DxfTag { code, value })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_other_table_bin(t: &DxfOtherTable, out: &mut Vec<u8>) {
    write_str_lp(out, &t.name);
    store::pack_rt::write_varint_u64(out, t.tags.len() as u64);
    for tag in &t.tags {
        enc_dxf_tag_bin(tag, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_other_table_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfOtherTable, String> {
    let name = read_str_lp(reader)?;
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut tags = Vec::with_capacity(count as usize);
    for _ in 0..count {
        tags.push(dec_dxf_tag_bin(reader)?);
    }
    Ok(DxfOtherTable { name, tags })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_dxf_tables_bin(t: &DxfTables, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, t.layers.len() as u64);
    for l in &t.layers {
        enc_layer_bin(l, out);
    }
    store::pack_rt::write_varint_u64(out, t.styles.len() as u64);
    for s in &t.styles {
        enc_style_bin(s, out);
    }
    store::pack_rt::write_varint_u64(out, t.linetypes.len() as u64);
    for l in &t.linetypes {
        enc_linetype_bin(l, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_dxf_tables_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfTables, String> {
    let lc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut layers = Vec::with_capacity(lc as usize);
    for _ in 0..lc {
        layers.push(dec_layer_bin(reader)?);
    }
    let sc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut styles = Vec::with_capacity(sc as usize);
    for _ in 0..sc {
        styles.push(dec_style_bin(reader)?);
    }
    let ltc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut linetypes = Vec::with_capacity(ltc as usize);
    for _ in 0..ltc {
        linetypes.push(dec_linetype_bin(reader)?);
    }
    Ok(DxfTables { layers, styles, linetypes })
}
/// 🧬️ Whole `DxfSnapshot` binary twin of [`enc_dxf_snapshot`]/[`dec_dxf_snapshot`] — needed by
/// `🧬️mutations::DxfMutation::SetSnapshot`'s upgraded `OpBinary` payload.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_dxf_snapshot_bin(s: &DxfSnapshot, out: &mut Vec<u8>) {
    write_str_lp(out, &s.schema);
    store::pack_rt::write_varint_u64(out, s.header_vars.len() as u64);
    for hv in &s.header_vars {
        enc_header_var_bin(hv, out);
    }
    enc_dxf_tables_bin(&s.tables, out);
    store::pack_rt::write_varint_u64(out, s.other_tables.len() as u64);
    for t in &s.other_tables {
        enc_other_table_bin(t, out);
    }
    store::pack_rt::write_varint_u64(out, s.blocks.len() as u64);
    for b in &s.blocks {
        enc_block_bin(b, out);
    }
    enc_dxf_entities_bin(&s.entities, out);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_dxf_snapshot_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfSnapshot, String> {
    let schema = read_str_lp(reader)?;
    let hvc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut header_vars = Vec::with_capacity(hvc as usize);
    for _ in 0..hvc {
        header_vars.push(dec_header_var_bin(reader)?);
    }
    let tables = dec_dxf_tables_bin(reader)?;
    let otc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut other_tables = Vec::with_capacity(otc as usize);
    for _ in 0..otc {
        other_tables.push(dec_other_table_bin(reader)?);
    }
    let bc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut blocks = Vec::with_capacity(bc as usize);
    for _ in 0..bc {
        blocks.push(dec_block_bin(reader)?);
    }
    let entities = dec_dxf_entities_bin(reader)?;
    Ok(DxfSnapshot { schema, header_vars, tables, other_tables, blocks, entities })
}
//#endregion 🔖️ItemBinaryCodecs

//#region 🔖️DiffBinaryCodecs
/// 🧪️ P2-FG1: real recursive binary twins of every `enc_*_diff`/`dec_*_diff` codec above
/// (`#region 🔖️DiffValueCodecs`) — backs the upgraded `DiffCodec` frame (`#region 🔖️TopLevel`
/// below). Every `Option<T>` field uses [`write_option_bin`]/[`read_option_bin`]'s tri-state
/// presence byte, the binary twin of the text codec's `encode_option`/`decode_option` `[0]`/
/// `[1,<value>]` pair.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_header_var_diff_bin(d: &DxfHeaderVarDiff, out: &mut Vec<u8>) {
    write_option_bin(out, &d.group_code, |v, out| store::write_varint_i64(out, *v as i64));
    write_option_bin(out, &d.value, |v, out| enc_dxf_value_bin(v, out));
    write_option_bin(out, &d.extra_group_codes, |v, out| enc_group_codes_bin(v, out));
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_header_var_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfHeaderVarDiff, String> {
    let group_code = read_option_bin(reader, |r| Ok(semio_framework_plugin::resolve_ready(r.read_varint_i64()).map_err(|e| e.to_string())? as i32))?;
    let value = read_option_bin(reader, dec_dxf_value_bin)?;
    let extra_group_codes = read_option_bin(reader, dec_group_codes_bin)?;
    Ok(DxfHeaderVarDiff { group_code, value, extra_group_codes })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_layer_diff_bin(d: &DxfLayerDiff, out: &mut Vec<u8>) {
    write_option_bin(out, &d.color, |v, out| store::write_varint_i64(out, *v as i64));
    write_option_bin(out, &d.linetype, |v, out| write_str_lp(out, v));
    write_option_bin(out, &d.flags, |v, out| store::write_varint_i64(out, *v as i64));
    write_option_bin(out, &d.unknown_group_codes, |v, out| enc_group_codes_bin(v, out));
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_layer_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfLayerDiff, String> {
    let color = read_option_bin(reader, |r| Ok(semio_framework_plugin::resolve_ready(r.read_varint_i64()).map_err(|e| e.to_string())? as i32))?;
    let linetype = read_option_bin(reader, read_str_lp)?;
    let flags = read_option_bin(reader, |r| Ok(semio_framework_plugin::resolve_ready(r.read_varint_i64()).map_err(|e| e.to_string())? as i32))?;
    let unknown_group_codes = read_option_bin(reader, dec_group_codes_bin)?;
    Ok(DxfLayerDiff { color, linetype, flags, unknown_group_codes })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_style_diff_bin(d: &DxfStyleDiff, out: &mut Vec<u8>) {
    write_option_bin(out, &d.flags, |v, out| store::write_varint_i64(out, *v as i64));
    write_option_bin(out, &d.font_name, |v, out| write_str_lp(out, v));
    write_option_bin(out, &d.unknown_group_codes, |v, out| enc_group_codes_bin(v, out));
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_style_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfStyleDiff, String> {
    let flags = read_option_bin(reader, |r| Ok(semio_framework_plugin::resolve_ready(r.read_varint_i64()).map_err(|e| e.to_string())? as i32))?;
    let font_name = read_option_bin(reader, read_str_lp)?;
    let unknown_group_codes = read_option_bin(reader, dec_group_codes_bin)?;
    Ok(DxfStyleDiff { flags, font_name, unknown_group_codes })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_linetype_diff_bin(d: &DxfLinetypeDiff, out: &mut Vec<u8>) {
    write_option_bin(out, &d.flags, |v, out| store::write_varint_i64(out, *v as i64));
    write_option_bin(out, &d.description, |v, out| write_str_lp(out, v));
    write_option_bin(out, &d.unknown_group_codes, |v, out| enc_group_codes_bin(v, out));
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_linetype_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfLinetypeDiff, String> {
    let flags = read_option_bin(reader, |r| Ok(semio_framework_plugin::resolve_ready(r.read_varint_i64()).map_err(|e| e.to_string())? as i32))?;
    let description = read_option_bin(reader, read_str_lp)?;
    let unknown_group_codes = read_option_bin(reader, dec_group_codes_bin)?;
    Ok(DxfLinetypeDiff { flags, description, unknown_group_codes })
}

/// 🌳 `DxfEntityDiff` binary twin of [`enc_entity_diff`]/[`dec_entity_diff`] — tag `0`-`7` matches
/// [`enc_dxf_entity_bin`]'s own per-kind numbering (kind-specific field diff), `8`=`Replace`
/// (carries a whole binary-encoded `DxfEntity`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_entity_diff_bin(d: &DxfEntityDiff, out: &mut Vec<u8>) {
    match d {
        DxfEntityDiff::Replace { entity } => {
            out.push(8);
            enc_dxf_entity_bin(entity, out);
        }
        DxfEntityDiff::Line(x) => {
            out.push(0);
            write_option_bin(out, &x.start, |v, out| write_point3_bin(out, v));
            write_option_bin(out, &x.end, |v, out| write_point3_bin(out, v));
            write_option_bin(out, &x.layer, |v, out| write_str_lp(out, v));
            write_option_bin(out, &x.unknown_group_codes, |v, out| enc_group_codes_bin(v, out));
        }
        DxfEntityDiff::Circle(x) => {
            out.push(1);
            write_option_bin(out, &x.center, |v, out| write_point3_bin(out, v));
            write_option_bin(out, &x.radius, |v, out| write_f64_bin(out, *v));
            write_option_bin(out, &x.layer, |v, out| write_str_lp(out, v));
            write_option_bin(out, &x.unknown_group_codes, |v, out| enc_group_codes_bin(v, out));
        }
        DxfEntityDiff::Arc(x) => {
            out.push(2);
            write_option_bin(out, &x.center, |v, out| write_point3_bin(out, v));
            write_option_bin(out, &x.radius, |v, out| write_f64_bin(out, *v));
            write_option_bin(out, &x.start_angle, |v, out| write_f64_bin(out, *v));
            write_option_bin(out, &x.end_angle, |v, out| write_f64_bin(out, *v));
            write_option_bin(out, &x.layer, |v, out| write_str_lp(out, v));
            write_option_bin(out, &x.unknown_group_codes, |v, out| enc_group_codes_bin(v, out));
        }
        DxfEntityDiff::Polyline(x) => {
            out.push(3);
            write_option_bin(out, &x.vertices, |v, out| enc_vertices_bin(v, out));
            write_option_bin(out, &x.closed, |v, out| out.push(if *v { 1 } else { 0 }));
            write_option_bin(out, &x.layer, |v, out| write_str_lp(out, v));
            write_option_bin(out, &x.unknown_group_codes, |v, out| enc_group_codes_bin(v, out));
        }
        DxfEntityDiff::Text(x) => {
            out.push(4);
            write_option_bin(out, &x.position, |v, out| write_point3_bin(out, v));
            write_option_bin(out, &x.height, |v, out| write_f64_bin(out, *v));
            write_option_bin(out, &x.value, |v, out| write_str_lp(out, v));
            write_option_bin(out, &x.layer, |v, out| write_str_lp(out, v));
            write_option_bin(out, &x.unknown_group_codes, |v, out| enc_group_codes_bin(v, out));
        }
        DxfEntityDiff::Solid(x) => {
            out.push(5);
            write_option_bin(out, &x.points, |v, out| write_points4_bin(out, v));
            write_option_bin(out, &x.layer, |v, out| write_str_lp(out, v));
            write_option_bin(out, &x.unknown_group_codes, |v, out| enc_group_codes_bin(v, out));
        }
        DxfEntityDiff::Insert(x) => {
            out.push(6);
            write_option_bin(out, &x.block_name, |v, out| write_str_lp(out, v));
            write_option_bin(out, &x.position, |v, out| write_point3_bin(out, v));
            write_option_bin(out, &x.scale, |v, out| write_point3_bin(out, v));
            write_option_bin(out, &x.rotation, |v, out| write_f64_bin(out, *v));
            write_option_bin(out, &x.layer, |v, out| write_str_lp(out, v));
            write_option_bin(out, &x.unknown_group_codes, |v, out| enc_group_codes_bin(v, out));
        }
        DxfEntityDiff::Other(x) => {
            out.push(7);
            write_option_bin(out, &x.group_codes, |v, out| enc_group_codes_bin(v, out));
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_entity_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfEntityDiff, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        8 => Ok(DxfEntityDiff::Replace { entity: dec_dxf_entity_bin(reader)? }),
        0 => {
            let start = read_option_bin(reader, read_point3_bin)?;
            let end = read_option_bin(reader, read_point3_bin)?;
            let layer = read_option_bin(reader, read_str_lp)?;
            let unknown_group_codes = read_option_bin(reader, dec_group_codes_bin)?;
            Ok(DxfEntityDiff::Line(DxfLineDiff { start, end, layer, unknown_group_codes }))
        }
        1 => {
            let center = read_option_bin(reader, read_point3_bin)?;
            let radius = read_option_bin(reader, read_f64_bin)?;
            let layer = read_option_bin(reader, read_str_lp)?;
            let unknown_group_codes = read_option_bin(reader, dec_group_codes_bin)?;
            Ok(DxfEntityDiff::Circle(DxfCircleDiff { center, radius, layer, unknown_group_codes }))
        }
        2 => {
            let center = read_option_bin(reader, read_point3_bin)?;
            let radius = read_option_bin(reader, read_f64_bin)?;
            let start_angle = read_option_bin(reader, read_f64_bin)?;
            let end_angle = read_option_bin(reader, read_f64_bin)?;
            let layer = read_option_bin(reader, read_str_lp)?;
            let unknown_group_codes = read_option_bin(reader, dec_group_codes_bin)?;
            Ok(DxfEntityDiff::Arc(DxfArcDiff { center, radius, start_angle, end_angle, layer, unknown_group_codes }))
        }
        3 => {
            let vertices = read_option_bin(reader, dec_vertices_bin)?;
            let closed = read_option_bin(reader, |r| Ok(semio_framework_plugin::resolve_ready(r.read_u8()).map_err(|e| e.to_string())? != 0))?;
            let layer = read_option_bin(reader, read_str_lp)?;
            let unknown_group_codes = read_option_bin(reader, dec_group_codes_bin)?;
            Ok(DxfEntityDiff::Polyline(DxfPolylineDiff { vertices, closed, layer, unknown_group_codes }))
        }
        4 => {
            let position = read_option_bin(reader, read_point3_bin)?;
            let height = read_option_bin(reader, read_f64_bin)?;
            let value = read_option_bin(reader, read_str_lp)?;
            let layer = read_option_bin(reader, read_str_lp)?;
            let unknown_group_codes = read_option_bin(reader, dec_group_codes_bin)?;
            Ok(DxfEntityDiff::Text(DxfTextDiff { position, height, value, layer, unknown_group_codes }))
        }
        5 => {
            let points = read_option_bin(reader, read_points4_bin)?;
            let layer = read_option_bin(reader, read_str_lp)?;
            let unknown_group_codes = read_option_bin(reader, dec_group_codes_bin)?;
            Ok(DxfEntityDiff::Solid(DxfSolidDiff { points, layer, unknown_group_codes }))
        }
        6 => {
            let block_name = read_option_bin(reader, read_str_lp)?;
            let position = read_option_bin(reader, read_point3_bin)?;
            let scale = read_option_bin(reader, read_point3_bin)?;
            let rotation = read_option_bin(reader, read_f64_bin)?;
            let layer = read_option_bin(reader, read_str_lp)?;
            let unknown_group_codes = read_option_bin(reader, dec_group_codes_bin)?;
            Ok(DxfEntityDiff::Insert(DxfInsertDiff { block_name, position, scale, rotation, layer, unknown_group_codes }))
        }
        7 => {
            let group_codes = read_option_bin(reader, dec_group_codes_bin)?;
            Ok(DxfEntityDiff::Other(DxfOtherDiff { group_codes }))
        }
        other => Err(format!("entity diff binary: unknown tag {other}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block_diff_bin(d: &DxfBlockDiff, out: &mut Vec<u8>) {
    write_option_bin(out, &d.name, |v, out| write_str_lp(out, v));
    write_option_bin(out, &d.base_point, |v, out| write_point3_bin(out, v));
    write_option_bin(out, &d.entities, |v, out| enc_entities_diff_bin(v, out));
    write_option_bin(out, &d.unknown_group_codes, |v, out| enc_group_codes_bin(v, out));
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfBlockDiff, String> {
    let name = read_option_bin(reader, read_str_lp)?;
    let base_point = read_option_bin(reader, read_point3_bin)?;
    let entities = read_option_bin(reader, dec_entities_diff_bin)?;
    let unknown_group_codes = read_option_bin(reader, dec_group_codes_bin)?;
    Ok(DxfBlockDiff { name, base_point, entities, unknown_group_codes })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_tables_diff_bin(t: &DxfTablesDiff, out: &mut Vec<u8>) {
    write_option_bin(out, &t.layers, |v, out| enc_layers_diff_bin(v, out));
    write_option_bin(out, &t.styles, |v, out| enc_styles_diff_bin(v, out));
    write_option_bin(out, &t.linetypes, |v, out| enc_linetypes_diff_bin(v, out));
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_tables_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfTablesDiff, String> {
    let layers = read_option_bin(reader, dec_layers_diff_bin)?;
    let styles = read_option_bin(reader, dec_styles_diff_bin)?;
    let linetypes = read_option_bin(reader, dec_linetypes_diff_bin)?;
    Ok(DxfTablesDiff { layers, styles, linetypes })
}
//#endregion 🔖️DiffBinaryCodecs

//#region 🔖️CollectionTripleBinaryCodecs
/// 🧮 Binary twins of [`enc_name_triple`]/[`dec_name_triple`] and [`enc_index_triple`]/
/// [`dec_index_triple`] above — varint-counted lists instead of `;`-separated bracket sections,
/// same removed/modified/added shape.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_name_triple_bin<T, D>(removed: &[String], modified: &[(String, D)], added: &[(usize, T)], out: &mut Vec<u8>, enc_diff: impl Fn(&D, &mut Vec<u8>), enc_item: impl Fn(&T, &mut Vec<u8>)) {
    store::pack_rt::write_varint_u64(out, removed.len() as u64);
    for name in removed {
        write_str_lp(out, name);
    }
    store::pack_rt::write_varint_u64(out, modified.len() as u64);
    for (name, d) in modified {
        write_str_lp(out, name);
        enc_diff(d, out);
    }
    store::pack_rt::write_varint_u64(out, added.len() as u64);
    for (idx, item) in added {
        store::pack_rt::write_varint_u64(out, *idx as u64);
        enc_item(item, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_name_triple_bin<T, D>(
    reader: &mut store::ByteReader<'_>,
    dec_diff: impl Fn(&mut store::ByteReader<'_>) -> Result<D, String>,
    dec_item: impl Fn(&mut store::ByteReader<'_>) -> Result<T, String>,
) -> Result<(Vec<String>, Vec<(String, D)>, Vec<(usize, T)>), String> {
    let rc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(rc as usize);
    for _ in 0..rc {
        removed.push(read_str_lp(reader)?);
    }
    let mc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(mc as usize);
    for _ in 0..mc {
        let name = read_str_lp(reader)?;
        let d = dec_diff(reader)?;
        modified.push((name, d));
    }
    let ac = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(ac as usize);
    for _ in 0..ac {
        let idx = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let item = dec_item(reader)?;
        added.push((idx, item));
    }
    Ok((removed, modified, added))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_index_triple_bin<T, D>(removed: &[usize], modified: &[(usize, D)], added: &[(usize, T)], out: &mut Vec<u8>, enc_diff: impl Fn(&D, &mut Vec<u8>), enc_item: impl Fn(&T, &mut Vec<u8>)) {
    store::pack_rt::write_varint_u64(out, removed.len() as u64);
    for idx in removed {
        store::pack_rt::write_varint_u64(out, *idx as u64);
    }
    store::pack_rt::write_varint_u64(out, modified.len() as u64);
    for (idx, d) in modified {
        store::pack_rt::write_varint_u64(out, *idx as u64);
        enc_diff(d, out);
    }
    store::pack_rt::write_varint_u64(out, added.len() as u64);
    for (idx, item) in added {
        store::pack_rt::write_varint_u64(out, *idx as u64);
        enc_item(item, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_index_triple_bin<T, D>(
    reader: &mut store::ByteReader<'_>,
    dec_diff: impl Fn(&mut store::ByteReader<'_>) -> Result<D, String>,
    dec_item: impl Fn(&mut store::ByteReader<'_>) -> Result<T, String>,
) -> Result<(Vec<usize>, Vec<(usize, D)>, Vec<(usize, T)>), String> {
    let rc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(rc as usize);
    for _ in 0..rc {
        removed.push(reader.read_varint_u64().map_err(|e| e.to_string())? as usize);
    }
    let mc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(mc as usize);
    for _ in 0..mc {
        let idx = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let d = dec_diff(reader)?;
        modified.push((idx, d));
    }
    let ac = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(ac as usize);
    for _ in 0..ac {
        let idx = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let item = dec_item(reader)?;
        added.push((idx, item));
    }
    Ok((removed, modified, added))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_header_vars_diff_bin(d: &DxfHeaderVarsDiff, out: &mut Vec<u8>) {
    let modified: Vec<(String, DxfHeaderVarDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
    let added: Vec<(usize, DxfHeaderVar)> = d.added.iter().map(|a| (a.index, a.header_var.clone())).collect();
    enc_name_triple_bin(&d.removed, &modified, &added, out, enc_header_var_diff_bin, enc_header_var_bin);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_header_vars_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfHeaderVarsDiff, String> {
    let (removed, modified, added) = dec_name_triple_bin(reader, dec_header_var_diff_bin, dec_header_var_bin)?;
    Ok(DxfHeaderVarsDiff { removed, modified: modified.into_iter().map(|(name, diff)| DxfHeaderVarModified { name, diff }).collect(), added: added.into_iter().map(|(index, header_var)| DxfHeaderVarAdded { index, header_var }).collect() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_layers_diff_bin(d: &DxfLayersDiff, out: &mut Vec<u8>) {
    let modified: Vec<(String, DxfLayerDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
    let added: Vec<(usize, DxfLayer)> = d.added.iter().map(|a| (a.index, a.layer.clone())).collect();
    enc_name_triple_bin(&d.removed, &modified, &added, out, enc_layer_diff_bin, enc_layer_bin);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_layers_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfLayersDiff, String> {
    let (removed, modified, added) = dec_name_triple_bin(reader, dec_layer_diff_bin, dec_layer_bin)?;
    Ok(DxfLayersDiff { removed, modified: modified.into_iter().map(|(name, diff)| DxfLayerModified { name, diff }).collect(), added: added.into_iter().map(|(index, layer)| DxfLayerAdded { index, layer }).collect() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_styles_diff_bin(d: &DxfStylesDiff, out: &mut Vec<u8>) {
    let modified: Vec<(String, DxfStyleDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
    let added: Vec<(usize, DxfStyle)> = d.added.iter().map(|a| (a.index, a.style.clone())).collect();
    enc_name_triple_bin(&d.removed, &modified, &added, out, enc_style_diff_bin, enc_style_bin);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_styles_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfStylesDiff, String> {
    let (removed, modified, added) = dec_name_triple_bin(reader, dec_style_diff_bin, dec_style_bin)?;
    Ok(DxfStylesDiff { removed, modified: modified.into_iter().map(|(name, diff)| DxfStyleModified { name, diff }).collect(), added: added.into_iter().map(|(index, style)| DxfStyleAdded { index, style }).collect() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_linetypes_diff_bin(d: &DxfLinetypesDiff, out: &mut Vec<u8>) {
    let modified: Vec<(String, DxfLinetypeDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
    let added: Vec<(usize, DxfLinetype)> = d.added.iter().map(|a| (a.index, a.linetype.clone())).collect();
    enc_name_triple_bin(&d.removed, &modified, &added, out, enc_linetype_diff_bin, enc_linetype_bin);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_linetypes_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfLinetypesDiff, String> {
    let (removed, modified, added) = dec_name_triple_bin(reader, dec_linetype_diff_bin, dec_linetype_bin)?;
    Ok(DxfLinetypesDiff { removed, modified: modified.into_iter().map(|(name, diff)| DxfLinetypeModified { name, diff }).collect(), added: added.into_iter().map(|(index, linetype)| DxfLinetypeAdded { index, linetype }).collect() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_entities_diff_bin(d: &DxfEntitiesDiff, out: &mut Vec<u8>) {
    let modified: Vec<(usize, DxfEntityDiff)> = d.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
    let added: Vec<(usize, DxfEntity)> = d.added.iter().map(|a| (a.index, a.entity.clone())).collect();
    enc_index_triple_bin(&d.removed, &modified, &added, out, enc_entity_diff_bin, enc_dxf_entity_bin);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_entities_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfEntitiesDiff, String> {
    let (removed, modified, added) = dec_index_triple_bin(reader, dec_entity_diff_bin, dec_dxf_entity_bin)?;
    Ok(DxfEntitiesDiff { removed, modified: modified.into_iter().map(|(index, diff)| DxfEntityModified { index, diff }).collect(), added: added.into_iter().map(|(index, entity)| DxfEntityAdded { index, entity }).collect() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_blocks_diff_bin(d: &DxfBlocksDiff, out: &mut Vec<u8>) {
    let modified: Vec<(usize, DxfBlockDiff)> = d.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
    let added: Vec<(usize, DxfBlock)> = d.added.iter().map(|a| (a.index, a.block.clone())).collect();
    enc_index_triple_bin(&d.removed, &modified, &added, out, enc_block_diff_bin, enc_block_bin);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_blocks_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfBlocksDiff, String> {
    let (removed, modified, added) = dec_index_triple_bin(reader, dec_block_diff_bin, dec_block_bin)?;
    Ok(DxfBlocksDiff { removed, modified: modified.into_iter().map(|(index, diff)| DxfBlockModified { index, diff }).collect(), added: added.into_iter().map(|(index, block)| DxfBlockAdded { index, block }).collect() })
}
//#endregion 🔖️CollectionTripleBinaryCodecs

//#region 🔖️TopLevel
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_dxf_diff(d: &DxfDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.header_vars {
        tokens.push(format!("header-vars={}", enc_header_vars_diff(v)));
    }
    if let Some(v) = &d.tables {
        tokens.push(format!("tables={}", enc_tables_diff(v)));
    }
    if let Some(v) = &d.blocks {
        tokens.push(format!("blocks={}", enc_blocks_diff(v)));
    }
    if let Some(v) = &d.entities {
        tokens.push(format!("entities={}", enc_entities_diff(v)));
    }
    tokens.join(" ")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_dxf_diff(line: &str) -> Result<DxfDiff, String> {
    let mut d = DxfDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("header-vars=") {
            d.header_vars = Some(dec_header_vars_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("tables=") {
            d.tables = Some(dec_tables_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("blocks=") {
            d.blocks = Some(dec_blocks_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("entities=") {
            d.entities = Some(dec_entities_diff(rest)?);
        } else {
            return Err(format!("dxf diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffCodec for DxfDiff {
    fn print_diff(&self) -> String {
        print_dxf_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_dxf_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// 🧪️ P2-FG1: REAL binary frame (`format u8 | flags u8 | per-present-field payload`), matching
    /// `../💾️binary/📡️component.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
    /// upgraded from F6's `print_diff().into_bytes()` text-as-binary shortcut (100% of stdio's
    /// `DiffCodec` impls were still on that shortcut per the P2-W0 census). `flags` is a 4-bit
    /// presence mask (bit0=`header_vars`,bit1=`tables`,bit2=`blocks`,bit3=`entities`) since
    /// `DxfDiff` has FOUR independently optional top-level fields, unlike `stdio.json`'s single
    /// `value` (one `has_value` byte there); each PRESENT field's own real recursive
    /// collection-triple/tri-state binary payload follows, genuinely structured
    /// (`#region 🔖️DiffBinaryCodecs`/`#region 🔖️CollectionTripleBinaryCodecs` above), never
    /// text-as-bytes.
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut flags: u8 = 0;
        if self.header_vars.is_some() {
            flags |= 0b0001;
        }
        if self.tables.is_some() {
            flags |= 0b0010;
        }
        if self.blocks.is_some() {
            flags |= 0b0100;
        }
        if self.entities.is_some() {
            flags |= 0b1000;
        }
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, flags];
        if let Some(v) = &self.header_vars {
            enc_header_vars_diff_bin(v, &mut out);
        }
        if let Some(v) = &self.tables {
            enc_tables_diff_bin(v, &mut out);
        }
        if let Some(v) = &self.blocks {
            enc_blocks_diff_bin(v, &mut out);
        }
        if let Some(v) = &self.entities {
            enc_entities_diff_bin(v, &mut out);
        }
        Ok(out)
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("diff format", 0, e.to_string()))?;
        let flags = reader.read_u8().map_err(|e| malformed("diff flags", 1, e.to_string()))?;
        let header_vars = if flags & 0b0001 != 0 { Some(dec_header_vars_diff_bin(&mut reader).map_err(|e| malformed("diff header_vars", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None };
        let tables = if flags & 0b0010 != 0 { Some(dec_tables_diff_bin(&mut reader).map_err(|e| malformed("diff tables", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None };
        let blocks = if flags & 0b0100 != 0 { Some(dec_blocks_diff_bin(&mut reader).map_err(|e| malformed("diff blocks", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None };
        let entities = if flags & 0b1000 != 0 { Some(dec_entities_diff_bin(&mut reader).map_err(|e| malformed("diff entities", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None };
        Ok(DxfDiff { header_vars, tables, blocks, entities })
    }
}
//#endregion 🔖️TopLevel

//#region 🔖️DemoCases
/// 🧪️ P2-FG1: representative `DxfDiff` values — the empty/no-op diff, a single-collection sparse
/// diff (entities only, both a removal and an addition), and a rich multi-collection diff
/// exercising every collection kind simultaneously (name-keyed + index-keyed, `Replace`
/// (kind-change) AND non-`Replace` kind-specific patches, a nested block-level `entities`
/// sub-diff) — the single source of truth reused by `diff_codec_text_binary_roundtrip_law` below
/// AND by `⚙️engine/🦀️component.rs`'s `diff_grammar_conformance_law`/`protocol_walk_law`
/// conformance tests.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<DxfDiff> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn line_entity() -> DxfEntity {
        DxfEntity::Line { start: [0.0, 0.0, 0.0], end: [1.0, 2.0, 3.0], layer: "0".into(), unknown_group_codes: vec![(40, DxfValue::Double { value: 1.5 })] }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn block_with_entity() -> DxfBlock {
        DxfBlock { name: "B1".into(), base_point: [1.0, 2.0, 3.0], entities: vec![line_entity()], unknown_group_codes: vec![(5, DxfValue::Str { value: "handle".into() })] }
    }

    let entities_only = DxfDiff {
        header_vars: None,
        tables: None,
        blocks: None,
        entities: Some(DxfEntitiesDiff { removed: vec![2], modified: vec![], added: vec![DxfEntityAdded { index: 0, entity: DxfEntity::Circle { center: [0.0, 0.0, 0.0], radius: 1.0, layer: "0".into(), unknown_group_codes: vec![] } }] }),
    };

    let rich = DxfDiff {
        header_vars: Some(DxfHeaderVarsDiff {
            removed: vec!["$DROP".to_string()],
            modified: vec![DxfHeaderVarModified {
                name: "$MOD".to_string(),
                diff: DxfHeaderVarDiff { group_code: Some(40), value: Some(DxfValue::Double { value: 2.5 }), extra_group_codes: Some(vec![(999, DxfValue::Str { value: "note".into() })]) },
            }],
            added: vec![DxfHeaderVarAdded { index: 1, header_var: DxfHeaderVar { name: "$NEW".into(), group_code: 70, value: DxfValue::Int { value: 3 }, extra_group_codes: vec![] } }],
        }),
        tables: Some(DxfTablesDiff {
            layers: Some(DxfLayersDiff {
                removed: vec!["OLD".to_string()],
                modified: vec![DxfLayerModified { name: "0".to_string(), diff: DxfLayerDiff { color: Some(3), linetype: Some("DASHED".into()), flags: Some(1), unknown_group_codes: None } }],
                added: vec![DxfLayerAdded { index: 1, layer: DxfLayer { name: "NEW".into(), color: 5, linetype: "CONTINUOUS".into(), flags: 0, unknown_group_codes: vec![] } }],
            }),
            styles: Some(DxfStylesDiff::default()),
            linetypes: None,
        }),
        blocks: Some(DxfBlocksDiff {
            removed: vec![],
            modified: vec![DxfBlockModified {
                index: 0,
                diff: DxfBlockDiff { name: None, base_point: Some([9.0, 9.0, 9.0]), entities: Some(DxfEntitiesDiff { removed: vec![], modified: vec![], added: vec![DxfEntityAdded { index: 0, entity: line_entity() }] }), unknown_group_codes: None },
            }],
            added: vec![DxfBlockAdded { index: 1, block: block_with_entity() }],
        }),
        entities: Some(DxfEntitiesDiff {
            removed: vec![2],
            modified: vec![
                DxfEntityModified { index: 0, diff: DxfEntityDiff::Line(DxfLineDiff { start: Some([9.0, 9.0, 9.0]), end: None, layer: None, unknown_group_codes: None }) },
                DxfEntityModified { index: 1, diff: DxfEntityDiff::Replace { entity: DxfEntity::Text { position: [0.0, 0.0, 0.0], height: 1.0, value: "swap".into(), layer: "0".into(), unknown_group_codes: vec![] } } },
            ],
            added: vec![DxfEntityAdded { index: 3, entity: DxfEntity::Other { kind: "3DFACE".into(), group_codes: vec![(10, DxfValue::Double { value: 0.0 })] } }],
        }),
    };

    vec![DxfDiff::default(), entities_only, rich]
}
//#endregion 🔖️DemoCases
//#endregion 🔖️HandcraftedDiffCodec

//#region 🧪️Tests
#[cfg(test)]
mod handcrafted_diff_codec_tests {
    use super::*;
    use protocol::DiffCodec;

    /// 🧪️ `DiffCodec` text/binary round-trip laws over every `demo_diff_cases()` fixture (`#region
    /// 🔖️DemoCases` above) — the empty diff, a single-collection sparse diff, and the rich case
    /// exercising every collection triple (name-keyed AND index-keyed) simultaneously, plus the
    /// `Replace` (kind-change) branch of `DxfEntityDiff` and a NON-`Replace` kind-specific patch,
    /// plus a nested block-level `entities` sub-diff (the SAME `DxfEntitiesDiff` machinery reused
    /// at two tree depths) — shared with `⚙️engine/🦀️component.rs`'s own conformance laws.
    #[semio_framework_async_macros::async_test]
    async fn diff_codec_text_binary_roundtrip_law() {
        for d in demo_diff_cases() {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must never contain a newline, for {d:?}");
            let parsed = DxfDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e:?}, for {d:?}"));
            assert_eq!(parsed, d, "parse_diff(print_diff(d)) == d");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e:?}, for {d:?}"));
            let decoded = DxfDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e:?}, for {d:?}"));
            assert_eq!(decoded, d, "decode_diff(encode_diff(d)) == d");

            let printed2 = d.print_diff();
            assert_eq!(printed, printed2, "print_diff must be deterministic, for {d:?}");
        }

        assert!(DxfDiff::default().print_diff().is_empty());
        assert_eq!(DxfDiff::parse_diff("").expect("parse empty"), DxfDiff::default());
    }
}
//#endregion 🧪️Tests
