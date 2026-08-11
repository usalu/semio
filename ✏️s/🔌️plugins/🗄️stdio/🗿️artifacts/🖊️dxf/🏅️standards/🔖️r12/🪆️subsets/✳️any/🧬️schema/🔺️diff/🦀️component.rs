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

use std::collections::HashSet;

use crate::artifacts::dxf::schema::snapshot::{
    DxfBlock, DxfEntity, DxfHeaderVar, DxfLayer, DxfLinetype, DxfStyle, DxfTables, DxfValue,
};
use crate::artifacts::dxf::DxfSnapshot;
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️IndexCollectionCore
/// 🧮 Per-item sparse-diff behavior shared by the two index-keyed collections (`DxfEntity`,
/// `DxfBlock` — the latter's own nested `entities` field reuses `DxfEntity`'s impl directly).
trait DxfIndexElem: Clone + PartialEq {
    type Diff: Clone + PartialEq;
    fn diff_is_empty(d: &Self::Diff) -> bool;
    fn diff_between(a: &Self, b: &Self) -> Self::Diff;
    fn diff_apply(d: &Self::Diff, item: &mut Self);
    fn diff_absorb(base: &mut Self::Diff, other: Self::Diff);
}

/// ▶️ Applies a `(removed, modified, added)` triple to a base array — modified on BASE
/// positions first, then removed descending, then added ascending clamped to `min(index,len)`
/// (recipe's normative apply order).
fn generic_apply<T: DxfIndexElem>(base: &[T], removed: &[usize], modified: &[(usize, T::Diff)], added: &[(usize, T)]) -> Vec<T> {
    let mut items = base.to_vec();
    for (idx, d) in modified {
        if let Some(it) = items.get_mut(*idx) { T::diff_apply(d, it); }
    }
    let mut removed_desc = removed.to_vec();
    removed_desc.sort_unstable_by(|a, b| b.cmp(a));
    removed_desc.dedup();
    for idx in removed_desc {
        if idx < items.len() { items.remove(idx); }
    }
    let mut adds: Vec<&(usize, T)> = added.iter().collect();
    adds.sort_by_key(|(i, _)| *i);
    for (idx, item) in adds {
        let at = (*idx).min(items.len());
        items.insert(at, item.clone());
    }
    items
}

/// 🧭️ Pairwise-by-position state delta (recipe's "index keys pairwise by position" rule).
fn generic_between<T: DxfIndexElem>(base: &[T], other: &[T]) -> (Vec<usize>, Vec<(usize, T::Diff)>, Vec<(usize, T)>) {
    let min_len = base.len().min(other.len());
    let mut modified = Vec::new();
    for i in 0..min_len {
        let d = T::diff_between(&base[i], &other[i]);
        if !T::diff_is_empty(&d) { modified.push((i, d)); }
    }
    let removed: Vec<usize> = if base.len() > other.len() { (other.len()..base.len()).collect() } else { Vec::new() };
    let added: Vec<(usize, T)> = if other.len() > base.len() {
        (base.len()..other.len()).map(|i| (i, other[i].clone())).collect()
    } else {
        Vec::new()
    };
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
fn generic_absorb_pair<T: DxfIndexElem>(
    d1_removed: &[usize], d1_modified: &[(usize, T::Diff)], d1_added: &[(usize, T)],
    d2_removed: &[usize], d2_modified: &[(usize, T::Diff)], d2_added: &[(usize, T)],
) -> (Vec<usize>, Vec<(usize, T::Diff)>, Vec<(usize, T)>) {
    use std::collections::HashMap;
    let max_ref = d1_removed.iter().copied()
        .chain(d1_modified.iter().map(|(i, _)| *i))
        .chain(d1_added.iter().map(|(i, _)| *i))
        .chain(d2_removed.iter().copied())
        .chain(d2_modified.iter().map(|(i, _)| *i))
        .chain(d2_added.iter().map(|(i, _)| *i))
        .max();
    let l1 = max_ref.map(|m| m + 2).unwrap_or(0);

    let base_labels: Vec<Lbl> = (0..l1).map(Lbl::Base).collect();
    let d1_added_lbl: Vec<(usize, Lbl)> = d1_added.iter().enumerate().map(|(j, (idx, _))| (*idx, Lbl::Added1(j))).collect();
    let mut mid_labels = simulate_labels(base_labels, d1_removed, &d1_added_lbl);

    let mut mid_pos_of_base: HashMap<usize, usize> = HashMap::new();
    let mut mid_pos_of_added1: HashMap<usize, usize> = HashMap::new();
    for (pos, l) in mid_labels.iter().enumerate() {
        match l {
            Lbl::Base(i) => { mid_pos_of_base.insert(*i, pos); }
            Lbl::Added1(j) => { mid_pos_of_added1.insert(*j, pos); }
            Lbl::Added2(_) => {}
        }
    }
    while mid_labels.len() < l1 { mid_labels.push(Lbl::Base(usize::MAX)); }

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
                    (Some(mut a), Some(b)) => { T::diff_absorb(&mut a, b); Some(a) }
                };
                if let Some(d) = combined {
                    if !T::diff_is_empty(&d) { modified.push((i, d)); }
                }
            }
            Lbl::Base(_) => {}
            Lbl::Added1(j) => {
                let mid_pos = mid_pos_of_added1.get(&j).copied();
                let (_, base_item) = &d1_added[j];
                let mut item = base_item.clone();
                if let Some(m) = mid_pos {
                    if let Some(d2d) = d2_modified_at.get(&m) { T::diff_apply(d2d, &mut item); }
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
    fn key(&self) -> &str;
    fn diff_between(a: &Self, b: &Self) -> Self::Diff;
    fn diff_apply(d: &Self::Diff, item: &mut Self);
    fn diff_absorb(base: &mut Self::Diff, other: Self::Diff);
}

fn named_apply<T: DxfNamedElem>(base: &[T], removed: &[String], modified: &[(String, T::Diff)], added: &[(usize, T)]) -> Vec<T> {
    let mut items = base.to_vec();
    for (key, d) in modified {
        if let Some(it) = items.iter_mut().find(|it| it.key() == key) { T::diff_apply(d, it); }
    }
    let removed_set: HashSet<&str> = removed.iter().map(String::as_str).collect();
    items.retain(|it| !removed_set.contains(it.key()));
    let mut adds: Vec<&(usize, T)> = added.iter().collect();
    adds.sort_by_key(|(i, _)| *i);
    for (idx, item) in adds {
        let at = (*idx).min(items.len());
        items.insert(at, item.clone());
    }
    items
}

fn named_between<T: DxfNamedElem>(base: &[T], other: &[T]) -> (Vec<String>, Vec<(String, T::Diff)>, Vec<(usize, T)>) {
    let base_keys: HashSet<&str> = base.iter().map(|t| t.key()).collect();
    let other_keys: HashSet<&str> = other.iter().map(|t| t.key()).collect();
    let removed: Vec<String> = base.iter().filter(|t| !other_keys.contains(t.key())).map(|t| t.key().to_string()).collect();
    let mut modified = Vec::new();
    for bt in base {
        if let Some(ot) = other.iter().find(|o| o.key() == bt.key()) {
            let d = T::diff_between(bt, ot);
            if d != T::Diff::default() { modified.push((bt.key().to_string(), d)); }
        }
    }
    let added: Vec<(usize, T)> = other.iter().enumerate().filter(|(_, t)| !base_keys.contains(t.key())).map(|(i, t)| (i, t.clone())).collect();
    (removed, modified, added)
}

#[allow(clippy::type_complexity)]
fn named_absorb_pair<T: DxfNamedElem>(
    d1_removed: &[String], d1_modified: &[(String, T::Diff)], d1_added: &[(usize, T)],
    d2_removed: &[String], d2_modified: &[(String, T::Diff)], d2_added: &[(usize, T)],
) -> (Vec<String>, Vec<(String, T::Diff)>, Vec<(usize, T)>) {
    let added_keys: HashSet<String> = d1_added.iter().map(|(_, t)| t.key().to_string()).collect();
    let mut merged_removed: Vec<String> = d1_removed.to_vec();
    let mut annihilated: HashSet<String> = HashSet::new();
    for key in d2_removed {
        if added_keys.contains(key) { annihilated.insert(key.clone()); }
        else if !merged_removed.contains(key) { merged_removed.push(key.clone()); }
    }
    let mut merged_modified: Vec<(String, T::Diff)> = d1_modified.iter().filter(|(k, _)| !merged_removed.contains(k)).cloned().collect();
    let mut merged_added: Vec<(usize, T)> = d1_added.iter().filter(|(_, t)| !annihilated.contains(t.key())).cloned().collect();

    for (key, d2d) in d2_modified {
        if added_keys.contains(key) {
            if annihilated.contains(key) { continue; }
            if let Some((_, item)) = merged_added.iter_mut().find(|(_, t)| t.key() == key) {
                T::diff_apply(d2d, item);
            }
        } else {
            if merged_removed.contains(key) { continue; }
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
    fn key(&self) -> &str { &self.name }
    fn diff_between(a: &Self, b: &Self) -> Self::Diff {
        DxfHeaderVarDiff {
            group_code: (a.group_code != b.group_code).then_some(b.group_code),
            value: (a.value != b.value).then(|| b.value.clone()),
            extra_group_codes: (a.extra_group_codes != b.extra_group_codes).then(|| b.extra_group_codes.clone()),
        }
    }
    fn diff_apply(d: &Self::Diff, item: &mut Self) {
        if let Some(v) = d.group_code { item.group_code = v; }
        if let Some(v) = &d.value { item.value = v.clone(); }
        if let Some(v) = &d.extra_group_codes { item.extra_group_codes = v.clone(); }
    }
    fn diff_absorb(base: &mut Self::Diff, other: Self::Diff) {
        if other.group_code.is_some() { base.group_code = other.group_code; }
        if other.value.is_some() { base.value = other.value; }
        if other.extra_group_codes.is_some() { base.extra_group_codes = other.extra_group_codes; }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfHeaderVarModified { pub name: String, pub diff: DxfHeaderVarDiff }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfHeaderVarAdded { pub index: usize, pub header_var: DxfHeaderVar }
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfHeaderVarsDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub removed: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub modified: Vec<DxfHeaderVarModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub added: Vec<DxfHeaderVarAdded>,
}
impl DxfHeaderVarsDiff {
    pub fn is_empty(&self) -> bool { self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty() }
    fn apply(&self, base: &[DxfHeaderVar]) -> Vec<DxfHeaderVar> {
        let modified: Vec<(String, DxfHeaderVarDiff)> = self.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
        let added: Vec<(usize, DxfHeaderVar)> = self.added.iter().map(|a| (a.index, a.header_var.clone())).collect();
        named_apply(base, &self.removed, &modified, &added)
    }
    fn between(base: &[DxfHeaderVar], other: &[DxfHeaderVar]) -> Option<Self> {
        let (removed, modified, added) = named_between(base, other);
        let d = Self {
            removed,
            modified: modified.into_iter().map(|(name, diff)| DxfHeaderVarModified { name, diff }).collect(),
            added: added.into_iter().map(|(index, header_var)| DxfHeaderVarAdded { index, header_var }).collect(),
        };
        if d.is_empty() { None } else { Some(d) }
    }
    fn absorb(d1: Self, d2: Self) -> Option<Self> {
        let d1m: Vec<(String, DxfHeaderVarDiff)> = d1.modified.into_iter().map(|m| (m.name, m.diff)).collect();
        let d1a: Vec<(usize, DxfHeaderVar)> = d1.added.into_iter().map(|a| (a.index, a.header_var)).collect();
        let d2m: Vec<(String, DxfHeaderVarDiff)> = d2.modified.into_iter().map(|m| (m.name, m.diff)).collect();
        let d2a: Vec<(usize, DxfHeaderVar)> = d2.added.into_iter().map(|a| (a.index, a.header_var)).collect();
        let (removed, modified, added) = named_absorb_pair::<DxfHeaderVar>(&d1.removed, &d1m, &d1a, &d2.removed, &d2m, &d2a);
        let d = Self {
            removed,
            modified: modified.into_iter().map(|(name, diff)| DxfHeaderVarModified { name, diff }).collect(),
            added: added.into_iter().map(|(index, header_var)| DxfHeaderVarAdded { index, header_var }).collect(),
        };
        if d.is_empty() { None } else { Some(d) }
    }
}
//#endregion 🔖️HeaderVarDiff

//#region 🔖️LayerDiff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfLayerDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")] pub color: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub linetype: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub flags: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}
impl DxfNamedElem for DxfLayer {
    type Diff = DxfLayerDiff;
    fn key(&self) -> &str { &self.name }
    fn diff_between(a: &Self, b: &Self) -> Self::Diff {
        DxfLayerDiff {
            color: (a.color != b.color).then_some(b.color),
            linetype: (a.linetype != b.linetype).then(|| b.linetype.clone()),
            flags: (a.flags != b.flags).then_some(b.flags),
            unknown_group_codes: (a.unknown_group_codes != b.unknown_group_codes).then(|| b.unknown_group_codes.clone()),
        }
    }
    fn diff_apply(d: &Self::Diff, item: &mut Self) {
        if let Some(v) = d.color { item.color = v; }
        if let Some(v) = &d.linetype { item.linetype = v.clone(); }
        if let Some(v) = d.flags { item.flags = v; }
        if let Some(v) = &d.unknown_group_codes { item.unknown_group_codes = v.clone(); }
    }
    fn diff_absorb(base: &mut Self::Diff, other: Self::Diff) {
        if other.color.is_some() { base.color = other.color; }
        if other.linetype.is_some() { base.linetype = other.linetype; }
        if other.flags.is_some() { base.flags = other.flags; }
        if other.unknown_group_codes.is_some() { base.unknown_group_codes = other.unknown_group_codes; }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfLayerModified { pub name: String, pub diff: DxfLayerDiff }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfLayerAdded { pub index: usize, pub layer: DxfLayer }
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfLayersDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub removed: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub modified: Vec<DxfLayerModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub added: Vec<DxfLayerAdded>,
}
impl DxfLayersDiff {
    pub fn is_empty(&self) -> bool { self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty() }
    fn apply(&self, base: &[DxfLayer]) -> Vec<DxfLayer> {
        let modified: Vec<(String, DxfLayerDiff)> = self.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
        let added: Vec<(usize, DxfLayer)> = self.added.iter().map(|a| (a.index, a.layer.clone())).collect();
        named_apply(base, &self.removed, &modified, &added)
    }
    fn between(base: &[DxfLayer], other: &[DxfLayer]) -> Option<Self> {
        let (removed, modified, added) = named_between(base, other);
        let d = Self {
            removed,
            modified: modified.into_iter().map(|(name, diff)| DxfLayerModified { name, diff }).collect(),
            added: added.into_iter().map(|(index, layer)| DxfLayerAdded { index, layer }).collect(),
        };
        if d.is_empty() { None } else { Some(d) }
    }
    fn absorb(d1: Self, d2: Self) -> Option<Self> {
        let d1m: Vec<(String, DxfLayerDiff)> = d1.modified.into_iter().map(|m| (m.name, m.diff)).collect();
        let d1a: Vec<(usize, DxfLayer)> = d1.added.into_iter().map(|a| (a.index, a.layer)).collect();
        let d2m: Vec<(String, DxfLayerDiff)> = d2.modified.into_iter().map(|m| (m.name, m.diff)).collect();
        let d2a: Vec<(usize, DxfLayer)> = d2.added.into_iter().map(|a| (a.index, a.layer)).collect();
        let (removed, modified, added) = named_absorb_pair::<DxfLayer>(&d1.removed, &d1m, &d1a, &d2.removed, &d2m, &d2a);
        let d = Self {
            removed,
            modified: modified.into_iter().map(|(name, diff)| DxfLayerModified { name, diff }).collect(),
            added: added.into_iter().map(|(index, layer)| DxfLayerAdded { index, layer }).collect(),
        };
        if d.is_empty() { None } else { Some(d) }
    }
}
//#endregion 🔖️LayerDiff

//#region 🔖️StyleDiff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfStyleDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")] pub flags: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub font_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}
impl DxfNamedElem for DxfStyle {
    type Diff = DxfStyleDiff;
    fn key(&self) -> &str { &self.name }
    fn diff_between(a: &Self, b: &Self) -> Self::Diff {
        DxfStyleDiff {
            flags: (a.flags != b.flags).then_some(b.flags),
            font_name: (a.font_name != b.font_name).then(|| b.font_name.clone()),
            unknown_group_codes: (a.unknown_group_codes != b.unknown_group_codes).then(|| b.unknown_group_codes.clone()),
        }
    }
    fn diff_apply(d: &Self::Diff, item: &mut Self) {
        if let Some(v) = d.flags { item.flags = v; }
        if let Some(v) = &d.font_name { item.font_name = v.clone(); }
        if let Some(v) = &d.unknown_group_codes { item.unknown_group_codes = v.clone(); }
    }
    fn diff_absorb(base: &mut Self::Diff, other: Self::Diff) {
        if other.flags.is_some() { base.flags = other.flags; }
        if other.font_name.is_some() { base.font_name = other.font_name; }
        if other.unknown_group_codes.is_some() { base.unknown_group_codes = other.unknown_group_codes; }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfStyleModified { pub name: String, pub diff: DxfStyleDiff }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfStyleAdded { pub index: usize, pub style: DxfStyle }
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfStylesDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub removed: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub modified: Vec<DxfStyleModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub added: Vec<DxfStyleAdded>,
}
impl DxfStylesDiff {
    pub fn is_empty(&self) -> bool { self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty() }
    fn apply(&self, base: &[DxfStyle]) -> Vec<DxfStyle> {
        let modified: Vec<(String, DxfStyleDiff)> = self.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
        let added: Vec<(usize, DxfStyle)> = self.added.iter().map(|a| (a.index, a.style.clone())).collect();
        named_apply(base, &self.removed, &modified, &added)
    }
    fn between(base: &[DxfStyle], other: &[DxfStyle]) -> Option<Self> {
        let (removed, modified, added) = named_between(base, other);
        let d = Self {
            removed,
            modified: modified.into_iter().map(|(name, diff)| DxfStyleModified { name, diff }).collect(),
            added: added.into_iter().map(|(index, style)| DxfStyleAdded { index, style }).collect(),
        };
        if d.is_empty() { None } else { Some(d) }
    }
    fn absorb(d1: Self, d2: Self) -> Option<Self> {
        let d1m: Vec<(String, DxfStyleDiff)> = d1.modified.into_iter().map(|m| (m.name, m.diff)).collect();
        let d1a: Vec<(usize, DxfStyle)> = d1.added.into_iter().map(|a| (a.index, a.style)).collect();
        let d2m: Vec<(String, DxfStyleDiff)> = d2.modified.into_iter().map(|m| (m.name, m.diff)).collect();
        let d2a: Vec<(usize, DxfStyle)> = d2.added.into_iter().map(|a| (a.index, a.style)).collect();
        let (removed, modified, added) = named_absorb_pair::<DxfStyle>(&d1.removed, &d1m, &d1a, &d2.removed, &d2m, &d2a);
        let d = Self {
            removed,
            modified: modified.into_iter().map(|(name, diff)| DxfStyleModified { name, diff }).collect(),
            added: added.into_iter().map(|(index, style)| DxfStyleAdded { index, style }).collect(),
        };
        if d.is_empty() { None } else { Some(d) }
    }
}
//#endregion 🔖️StyleDiff

//#region 🔖️LinetypeDiff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfLinetypeDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")] pub flags: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}
impl DxfNamedElem for DxfLinetype {
    type Diff = DxfLinetypeDiff;
    fn key(&self) -> &str { &self.name }
    fn diff_between(a: &Self, b: &Self) -> Self::Diff {
        DxfLinetypeDiff {
            flags: (a.flags != b.flags).then_some(b.flags),
            description: (a.description != b.description).then(|| b.description.clone()),
            unknown_group_codes: (a.unknown_group_codes != b.unknown_group_codes).then(|| b.unknown_group_codes.clone()),
        }
    }
    fn diff_apply(d: &Self::Diff, item: &mut Self) {
        if let Some(v) = d.flags { item.flags = v; }
        if let Some(v) = &d.description { item.description = v.clone(); }
        if let Some(v) = &d.unknown_group_codes { item.unknown_group_codes = v.clone(); }
    }
    fn diff_absorb(base: &mut Self::Diff, other: Self::Diff) {
        if other.flags.is_some() { base.flags = other.flags; }
        if other.description.is_some() { base.description = other.description; }
        if other.unknown_group_codes.is_some() { base.unknown_group_codes = other.unknown_group_codes; }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfLinetypeModified { pub name: String, pub diff: DxfLinetypeDiff }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfLinetypeAdded { pub index: usize, pub linetype: DxfLinetype }
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfLinetypesDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub removed: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub modified: Vec<DxfLinetypeModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub added: Vec<DxfLinetypeAdded>,
}
impl DxfLinetypesDiff {
    pub fn is_empty(&self) -> bool { self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty() }
    fn apply(&self, base: &[DxfLinetype]) -> Vec<DxfLinetype> {
        let modified: Vec<(String, DxfLinetypeDiff)> = self.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
        let added: Vec<(usize, DxfLinetype)> = self.added.iter().map(|a| (a.index, a.linetype.clone())).collect();
        named_apply(base, &self.removed, &modified, &added)
    }
    fn between(base: &[DxfLinetype], other: &[DxfLinetype]) -> Option<Self> {
        let (removed, modified, added) = named_between(base, other);
        let d = Self {
            removed,
            modified: modified.into_iter().map(|(name, diff)| DxfLinetypeModified { name, diff }).collect(),
            added: added.into_iter().map(|(index, linetype)| DxfLinetypeAdded { index, linetype }).collect(),
        };
        if d.is_empty() { None } else { Some(d) }
    }
    fn absorb(d1: Self, d2: Self) -> Option<Self> {
        let d1m: Vec<(String, DxfLinetypeDiff)> = d1.modified.into_iter().map(|m| (m.name, m.diff)).collect();
        let d1a: Vec<(usize, DxfLinetype)> = d1.added.into_iter().map(|a| (a.index, a.linetype)).collect();
        let d2m: Vec<(String, DxfLinetypeDiff)> = d2.modified.into_iter().map(|m| (m.name, m.diff)).collect();
        let d2a: Vec<(usize, DxfLinetype)> = d2.added.into_iter().map(|a| (a.index, a.linetype)).collect();
        let (removed, modified, added) = named_absorb_pair::<DxfLinetype>(&d1.removed, &d1m, &d1a, &d2.removed, &d2m, &d2a);
        let d = Self {
            removed,
            modified: modified.into_iter().map(|(name, diff)| DxfLinetypeModified { name, diff }).collect(),
            added: added.into_iter().map(|(index, linetype)| DxfLinetypeAdded { index, linetype }).collect(),
        };
        if d.is_empty() { None } else { Some(d) }
    }
}
//#endregion 🔖️LinetypeDiff

//#region 🔖️TablesDiff
/// 🔺️ Groups the three name-keyed table diffs — `DxfTables` itself is a weak grouping struct
/// (not a collection), so its diff is a plain per-field `Option<...>` struct, one per sub-collection.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfTablesDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")] pub layers: Option<DxfLayersDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub styles: Option<DxfStylesDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub linetypes: Option<DxfLinetypesDiff>,
}
impl DxfTablesDiff {
    pub fn is_empty(&self) -> bool {
        self.layers.as_ref().map_or(true, DxfLayersDiff::is_empty)
            && self.styles.as_ref().map_or(true, DxfStylesDiff::is_empty)
            && self.linetypes.as_ref().map_or(true, DxfLinetypesDiff::is_empty)
    }
    fn apply(&self, base: &DxfTables) -> DxfTables {
        DxfTables {
            layers: match &self.layers { Some(d) => d.apply(&base.layers), None => base.layers.clone() },
            styles: match &self.styles { Some(d) => d.apply(&base.styles), None => base.styles.clone() },
            linetypes: match &self.linetypes { Some(d) => d.apply(&base.linetypes), None => base.linetypes.clone() },
        }
    }
    fn between(base: &DxfTables, other: &DxfTables) -> Option<Self> {
        let d = Self {
            layers: DxfLayersDiff::between(&base.layers, &other.layers),
            styles: DxfStylesDiff::between(&base.styles, &other.styles),
            linetypes: DxfLinetypesDiff::between(&base.linetypes, &other.linetypes),
        };
        if d.is_empty() { None } else { Some(d) }
    }
    fn absorb(a: Self, b: Self) -> Option<Self> {
        let layers = match (a.layers, b.layers) { (None, None) => None, (Some(x), None) => Some(x), (None, Some(y)) => Some(y), (Some(x), Some(y)) => DxfLayersDiff::absorb(x, y) };
        let styles = match (a.styles, b.styles) { (None, None) => None, (Some(x), None) => Some(x), (None, Some(y)) => Some(y), (Some(x), Some(y)) => DxfStylesDiff::absorb(x, y) };
        let linetypes = match (a.linetypes, b.linetypes) { (None, None) => None, (Some(x), None) => Some(x), (None, Some(y)) => Some(y), (Some(x), Some(y)) => DxfLinetypesDiff::absorb(x, y) };
        let d = Self { layers, styles, linetypes };
        if d.is_empty() { None } else { Some(d) }
    }
}
//#endregion 🔖️TablesDiff

//#region 🔖️EntityDiff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfLineDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")] pub start: Option<[f64; 3]>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub end: Option<[f64; 3]>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub layer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfCircleDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")] pub center: Option<[f64; 3]>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub radius: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub layer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfArcDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")] pub center: Option<[f64; 3]>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub radius: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub start_angle: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub end_angle: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub layer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}
/// 🔺️ `vertices` is a weak leaf value (a polyline's own vertex list) — whole-vec replaced,
/// never sub-diffed (recipe's weak-entity rule).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfPolylineDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")] pub vertices: Option<Vec<crate::artifacts::dxf::schema::snapshot::DxfVertex>>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub closed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub layer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfTextDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")] pub position: Option<[f64; 3]>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub height: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub layer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfSolidDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")] pub points: Option<[[f64; 3]; 4]>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub layer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfInsertDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")] pub block_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub position: Option<[f64; 3]>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub scale: Option<[f64; 3]>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub rotation: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub layer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}
/// 🔺️ `group_codes` is a weak leaf value — whole-vec replaced (the entity's `kind` never
/// changes within an `Other` variant match; a kind change is handled by `DxfEntityDiff::Replace`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfOtherDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")] pub group_codes: Option<Vec<(i32, DxfValue)>>,
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
fn entity_diff_between(a: &DxfEntity, b: &DxfEntity) -> DxfEntityDiff {
    match (a, b) {
        (DxfEntity::Line { start: sa, end: ea, layer: la, unknown_group_codes: ua },
         DxfEntity::Line { start: sb, end: eb, layer: lb, unknown_group_codes: ub }) => DxfEntityDiff::Line(DxfLineDiff {
            start: (sa != sb).then_some(*sb),
            end: (ea != eb).then_some(*eb),
            layer: (la != lb).then(|| lb.clone()),
            unknown_group_codes: (ua != ub).then(|| ub.clone()),
        }),
        (DxfEntity::Circle { center: ca, radius: ra, layer: la, unknown_group_codes: ua },
         DxfEntity::Circle { center: cb, radius: rb, layer: lb, unknown_group_codes: ub }) => DxfEntityDiff::Circle(DxfCircleDiff {
            center: (ca != cb).then_some(*cb),
            radius: (ra != rb).then_some(*rb),
            layer: (la != lb).then(|| lb.clone()),
            unknown_group_codes: (ua != ub).then(|| ub.clone()),
        }),
        (DxfEntity::Arc { center: ca, radius: ra, start_angle: saa, end_angle: eaa, layer: la, unknown_group_codes: ua },
         DxfEntity::Arc { center: cb, radius: rb, start_angle: sab, end_angle: eab, layer: lb, unknown_group_codes: ub }) => DxfEntityDiff::Arc(DxfArcDiff {
            center: (ca != cb).then_some(*cb),
            radius: (ra != rb).then_some(*rb),
            start_angle: (saa != sab).then_some(*sab),
            end_angle: (eaa != eab).then_some(*eab),
            layer: (la != lb).then(|| lb.clone()),
            unknown_group_codes: (ua != ub).then(|| ub.clone()),
        }),
        (DxfEntity::Polyline { vertices: va, closed: cla, layer: la, unknown_group_codes: ua },
         DxfEntity::Polyline { vertices: vb, closed: clb, layer: lb, unknown_group_codes: ub }) => DxfEntityDiff::Polyline(DxfPolylineDiff {
            vertices: (va != vb).then(|| vb.clone()),
            closed: (cla != clb).then_some(*clb),
            layer: (la != lb).then(|| lb.clone()),
            unknown_group_codes: (ua != ub).then(|| ub.clone()),
        }),
        (DxfEntity::Text { position: pa, height: ha, value: vaa, layer: la, unknown_group_codes: ua },
         DxfEntity::Text { position: pb, height: hb, value: vab, layer: lb, unknown_group_codes: ub }) => DxfEntityDiff::Text(DxfTextDiff {
            position: (pa != pb).then_some(*pb),
            height: (ha != hb).then_some(*hb),
            value: (vaa != vab).then(|| vab.clone()),
            layer: (la != lb).then(|| lb.clone()),
            unknown_group_codes: (ua != ub).then(|| ub.clone()),
        }),
        (DxfEntity::Solid { points: pa, layer: la, unknown_group_codes: ua },
         DxfEntity::Solid { points: pb, layer: lb, unknown_group_codes: ub }) => DxfEntityDiff::Solid(DxfSolidDiff {
            points: (pa != pb).then_some(*pb),
            layer: (la != lb).then(|| lb.clone()),
            unknown_group_codes: (ua != ub).then(|| ub.clone()),
        }),
        (DxfEntity::Insert { block_name: ba, position: pa, scale: sca, rotation: ra, layer: la, unknown_group_codes: ua },
         DxfEntity::Insert { block_name: bb, position: pb, scale: scb, rotation: rb, layer: lb, unknown_group_codes: ub }) => DxfEntityDiff::Insert(DxfInsertDiff {
            block_name: (ba != bb).then(|| bb.clone()),
            position: (pa != pb).then_some(*pb),
            scale: (sca != scb).then_some(*scb),
            rotation: (ra != rb).then_some(*rb),
            layer: (la != lb).then(|| lb.clone()),
            unknown_group_codes: (ua != ub).then(|| ub.clone()),
        }),
        (DxfEntity::Other { kind: ka, group_codes: ga }, DxfEntity::Other { kind: kb, group_codes: gb }) if ka == kb => {
            DxfEntityDiff::Other(DxfOtherDiff { group_codes: (ga != gb).then(|| gb.clone()) })
        }
        _ => DxfEntityDiff::Replace { entity: b.clone() },
    }
}

fn apply_line_diff(d: &DxfLineDiff, start: &mut [f64; 3], end: &mut [f64; 3], layer: &mut String, unknown: &mut Vec<(i32, DxfValue)>) {
    if let Some(v) = d.start { *start = v; }
    if let Some(v) = d.end { *end = v; }
    if let Some(v) = &d.layer { *layer = v.clone(); }
    if let Some(v) = &d.unknown_group_codes { *unknown = v.clone(); }
}

/// ▶️ Applies a kind-specific diff to an entity — used both by `diff_apply` (real position) and
/// by absorb's `Replace`+kind-diff branch (patch-into-the-carried-replacement, same shape as the
/// recipe's canonical `Insert+SetField` "patch into added payload" case).
fn apply_entity_diff(d: &DxfEntityDiff, item: &mut DxfEntity) {
    match (d, item) {
        (DxfEntityDiff::Line(ld), DxfEntity::Line { start, end, layer, unknown_group_codes }) => apply_line_diff(ld, start, end, layer, unknown_group_codes),
        (DxfEntityDiff::Circle(cd), DxfEntity::Circle { center, radius, layer, unknown_group_codes }) => {
            if let Some(v) = cd.center { *center = v; }
            if let Some(v) = cd.radius { *radius = v; }
            if let Some(v) = &cd.layer { *layer = v.clone(); }
            if let Some(v) = &cd.unknown_group_codes { *unknown_group_codes = v.clone(); }
        }
        (DxfEntityDiff::Arc(ad), DxfEntity::Arc { center, radius, start_angle, end_angle, layer, unknown_group_codes }) => {
            if let Some(v) = ad.center { *center = v; }
            if let Some(v) = ad.radius { *radius = v; }
            if let Some(v) = ad.start_angle { *start_angle = v; }
            if let Some(v) = ad.end_angle { *end_angle = v; }
            if let Some(v) = &ad.layer { *layer = v.clone(); }
            if let Some(v) = &ad.unknown_group_codes { *unknown_group_codes = v.clone(); }
        }
        (DxfEntityDiff::Polyline(pd), DxfEntity::Polyline { vertices, closed, layer, unknown_group_codes }) => {
            if let Some(v) = &pd.vertices { *vertices = v.clone(); }
            if let Some(v) = pd.closed { *closed = v; }
            if let Some(v) = &pd.layer { *layer = v.clone(); }
            if let Some(v) = &pd.unknown_group_codes { *unknown_group_codes = v.clone(); }
        }
        (DxfEntityDiff::Text(td), DxfEntity::Text { position, height, value, layer, unknown_group_codes }) => {
            if let Some(v) = td.position { *position = v; }
            if let Some(v) = td.height { *height = v; }
            if let Some(v) = &td.value { *value = v.clone(); }
            if let Some(v) = &td.layer { *layer = v.clone(); }
            if let Some(v) = &td.unknown_group_codes { *unknown_group_codes = v.clone(); }
        }
        (DxfEntityDiff::Solid(sd), DxfEntity::Solid { points, layer, unknown_group_codes }) => {
            if let Some(v) = sd.points { *points = v; }
            if let Some(v) = &sd.layer { *layer = v.clone(); }
            if let Some(v) = &sd.unknown_group_codes { *unknown_group_codes = v.clone(); }
        }
        (DxfEntityDiff::Insert(id), DxfEntity::Insert { block_name, position, scale, rotation, layer, unknown_group_codes }) => {
            if let Some(v) = &id.block_name { *block_name = v.clone(); }
            if let Some(v) = id.position { *position = v; }
            if let Some(v) = id.scale { *scale = v; }
            if let Some(v) = id.rotation { *rotation = v; }
            if let Some(v) = &id.layer { *layer = v.clone(); }
            if let Some(v) = &id.unknown_group_codes { *unknown_group_codes = v.clone(); }
        }
        (DxfEntityDiff::Other(od), DxfEntity::Other { group_codes, .. }) => {
            if let Some(v) = &od.group_codes { *group_codes = v.clone(); }
        }
        _ => {} // kind mismatch without Replace: contract violation, graceful no-op
    }
}

impl DxfIndexElem for DxfEntity {
    type Diff = DxfEntityDiff;
    fn diff_is_empty(d: &Self::Diff) -> bool { entity_diff_is_empty(d) }
    fn diff_between(a: &Self, b: &Self) -> Self::Diff { entity_diff_between(a, b) }
    fn diff_apply(d: &Self::Diff, item: &mut Self) {
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
    fn diff_absorb(base: &mut Self::Diff, other: Self::Diff) {
        *base = match (base.clone(), other) {
            (DxfEntityDiff::Replace { .. }, DxfEntityDiff::Replace { entity: e2 }) => DxfEntityDiff::Replace { entity: e2 },
            (DxfEntityDiff::Replace { mut entity }, other_diff) => { apply_entity_diff(&other_diff, &mut entity); DxfEntityDiff::Replace { entity } }
            (_, DxfEntityDiff::Replace { entity }) => DxfEntityDiff::Replace { entity },
            (DxfEntityDiff::Line(mut a), DxfEntityDiff::Line(b)) => {
                if b.start.is_some() { a.start = b.start; } if b.end.is_some() { a.end = b.end; }
                if b.layer.is_some() { a.layer = b.layer; } if b.unknown_group_codes.is_some() { a.unknown_group_codes = b.unknown_group_codes; }
                DxfEntityDiff::Line(a)
            }
            (DxfEntityDiff::Circle(mut a), DxfEntityDiff::Circle(b)) => {
                if b.center.is_some() { a.center = b.center; } if b.radius.is_some() { a.radius = b.radius; }
                if b.layer.is_some() { a.layer = b.layer; } if b.unknown_group_codes.is_some() { a.unknown_group_codes = b.unknown_group_codes; }
                DxfEntityDiff::Circle(a)
            }
            (DxfEntityDiff::Arc(mut a), DxfEntityDiff::Arc(b)) => {
                if b.center.is_some() { a.center = b.center; } if b.radius.is_some() { a.radius = b.radius; }
                if b.start_angle.is_some() { a.start_angle = b.start_angle; } if b.end_angle.is_some() { a.end_angle = b.end_angle; }
                if b.layer.is_some() { a.layer = b.layer; } if b.unknown_group_codes.is_some() { a.unknown_group_codes = b.unknown_group_codes; }
                DxfEntityDiff::Arc(a)
            }
            (DxfEntityDiff::Polyline(mut a), DxfEntityDiff::Polyline(b)) => {
                if b.vertices.is_some() { a.vertices = b.vertices; } if b.closed.is_some() { a.closed = b.closed; }
                if b.layer.is_some() { a.layer = b.layer; } if b.unknown_group_codes.is_some() { a.unknown_group_codes = b.unknown_group_codes; }
                DxfEntityDiff::Polyline(a)
            }
            (DxfEntityDiff::Text(mut a), DxfEntityDiff::Text(b)) => {
                if b.position.is_some() { a.position = b.position; } if b.height.is_some() { a.height = b.height; }
                if b.value.is_some() { a.value = b.value; } if b.layer.is_some() { a.layer = b.layer; }
                if b.unknown_group_codes.is_some() { a.unknown_group_codes = b.unknown_group_codes; }
                DxfEntityDiff::Text(a)
            }
            (DxfEntityDiff::Solid(mut a), DxfEntityDiff::Solid(b)) => {
                if b.points.is_some() { a.points = b.points; }
                if b.layer.is_some() { a.layer = b.layer; } if b.unknown_group_codes.is_some() { a.unknown_group_codes = b.unknown_group_codes; }
                DxfEntityDiff::Solid(a)
            }
            (DxfEntityDiff::Insert(mut a), DxfEntityDiff::Insert(b)) => {
                if b.block_name.is_some() { a.block_name = b.block_name; } if b.position.is_some() { a.position = b.position; }
                if b.scale.is_some() { a.scale = b.scale; } if b.rotation.is_some() { a.rotation = b.rotation; }
                if b.layer.is_some() { a.layer = b.layer; } if b.unknown_group_codes.is_some() { a.unknown_group_codes = b.unknown_group_codes; }
                DxfEntityDiff::Insert(a)
            }
            (DxfEntityDiff::Other(mut a), DxfEntityDiff::Other(b)) => {
                if b.group_codes.is_some() { a.group_codes = b.group_codes; }
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
pub struct DxfEntityModified { pub index: usize, pub diff: DxfEntityDiff }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfEntityAdded { pub index: usize, pub entity: DxfEntity }
/// 🔺️ Index-keyed removed/modified/added triple over an entity collection — reused for BOTH
/// `DxfSnapshot::entities` and each `DxfBlock::entities`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfEntitiesDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub modified: Vec<DxfEntityModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub added: Vec<DxfEntityAdded>,
}
impl DxfEntitiesDiff {
    pub fn is_empty(&self) -> bool { self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty() }
    fn apply(&self, base: &[DxfEntity]) -> Vec<DxfEntity> {
        let modified: Vec<(usize, DxfEntityDiff)> = self.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
        let added: Vec<(usize, DxfEntity)> = self.added.iter().map(|a| (a.index, a.entity.clone())).collect();
        generic_apply(base, &self.removed, &modified, &added)
    }
    fn between(base: &[DxfEntity], other: &[DxfEntity]) -> Option<Self> {
        let (removed, modified, added) = generic_between(base, other);
        let d = Self {
            removed,
            modified: modified.into_iter().map(|(index, diff)| DxfEntityModified { index, diff }).collect(),
            added: added.into_iter().map(|(index, entity)| DxfEntityAdded { index, entity }).collect(),
        };
        if d.is_empty() { None } else { Some(d) }
    }
    fn absorb(d1: Self, d2: Self) -> Option<Self> {
        let d1m: Vec<(usize, DxfEntityDiff)> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();
        let d1a: Vec<(usize, DxfEntity)> = d1.added.into_iter().map(|a| (a.index, a.entity)).collect();
        let d2m: Vec<(usize, DxfEntityDiff)> = d2.modified.into_iter().map(|m| (m.index, m.diff)).collect();
        let d2a: Vec<(usize, DxfEntity)> = d2.added.into_iter().map(|a| (a.index, a.entity)).collect();
        let (removed, modified, added) = generic_absorb_pair::<DxfEntity>(&d1.removed, &d1m, &d1a, &d2.removed, &d2m, &d2a);
        let d = Self {
            removed,
            modified: modified.into_iter().map(|(index, diff)| DxfEntityModified { index, diff }).collect(),
            added: added.into_iter().map(|(index, entity)| DxfEntityAdded { index, entity }).collect(),
        };
        if d.is_empty() { None } else { Some(d) }
    }
}
//#endregion 🔖️EntityDiff

//#region 🔖️BlockDiff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfBlockDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")] pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub base_point: Option<[f64; 3]>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub entities: Option<DxfEntitiesDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}
impl DxfIndexElem for DxfBlock {
    type Diff = DxfBlockDiff;
    fn diff_is_empty(d: &Self::Diff) -> bool { d == &DxfBlockDiff::default() }
    fn diff_between(a: &Self, b: &Self) -> Self::Diff {
        DxfBlockDiff {
            name: (a.name != b.name).then(|| b.name.clone()),
            base_point: (a.base_point != b.base_point).then_some(b.base_point),
            entities: DxfEntitiesDiff::between(&a.entities, &b.entities),
            unknown_group_codes: (a.unknown_group_codes != b.unknown_group_codes).then(|| b.unknown_group_codes.clone()),
        }
    }
    fn diff_apply(d: &Self::Diff, item: &mut Self) {
        if let Some(v) = &d.name { item.name = v.clone(); }
        if let Some(v) = d.base_point { item.base_point = v; }
        if let Some(ed) = &d.entities { item.entities = ed.apply(&item.entities); }
        if let Some(v) = &d.unknown_group_codes { item.unknown_group_codes = v.clone(); }
    }
    fn diff_absorb(base: &mut Self::Diff, other: Self::Diff) {
        if other.name.is_some() { base.name = other.name; }
        if other.base_point.is_some() { base.base_point = other.base_point; }
        base.entities = match (base.entities.take(), other.entities) {
            (None, None) => None, (Some(a), None) => Some(a), (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => DxfEntitiesDiff::absorb(a, b),
        };
        if other.unknown_group_codes.is_some() { base.unknown_group_codes = other.unknown_group_codes; }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfBlockModified { pub index: usize, pub diff: DxfBlockDiff }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfBlockAdded { pub index: usize, pub block: DxfBlock }
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DxfBlocksDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub modified: Vec<DxfBlockModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub added: Vec<DxfBlockAdded>,
}
impl DxfBlocksDiff {
    pub fn is_empty(&self) -> bool { self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty() }
    fn apply(&self, base: &[DxfBlock]) -> Vec<DxfBlock> {
        let modified: Vec<(usize, DxfBlockDiff)> = self.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
        let added: Vec<(usize, DxfBlock)> = self.added.iter().map(|a| (a.index, a.block.clone())).collect();
        generic_apply(base, &self.removed, &modified, &added)
    }
    fn between(base: &[DxfBlock], other: &[DxfBlock]) -> Option<Self> {
        let (removed, modified, added) = generic_between(base, other);
        let d = Self {
            removed,
            modified: modified.into_iter().map(|(index, diff)| DxfBlockModified { index, diff }).collect(),
            added: added.into_iter().map(|(index, block)| DxfBlockAdded { index, block }).collect(),
        };
        if d.is_empty() { None } else { Some(d) }
    }
    fn absorb(d1: Self, d2: Self) -> Option<Self> {
        let d1m: Vec<(usize, DxfBlockDiff)> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();
        let d1a: Vec<(usize, DxfBlock)> = d1.added.into_iter().map(|a| (a.index, a.block)).collect();
        let d2m: Vec<(usize, DxfBlockDiff)> = d2.modified.into_iter().map(|m| (m.index, m.diff)).collect();
        let d2a: Vec<(usize, DxfBlock)> = d2.added.into_iter().map(|a| (a.index, a.block)).collect();
        let (removed, modified, added) = generic_absorb_pair::<DxfBlock>(&d1.removed, &d1m, &d1a, &d2.removed, &d2m, &d2a);
        let d = Self {
            removed,
            modified: modified.into_iter().map(|(index, diff)| DxfBlockModified { index, diff }).collect(),
            added: added.into_iter().map(|(index, block)| DxfBlockAdded { index, block }).collect(),
        };
        if d.is_empty() { None } else { Some(d) }
    }
}
//#endregion 🔖️BlockDiff

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.dxf`. `schema` is an identity field and never appears here.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.dxf.diff")]
pub struct DxfDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header_vars: Option<DxfHeaderVarsDiff>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tables: Option<DxfTablesDiff>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<DxfBlocksDiff>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<DxfEntitiesDiff>,
}

impl MutationDiff<DxfSnapshot> for DxfDiff {
    fn apply(&self, base: &DxfSnapshot) -> DxfSnapshot {
        DxfSnapshot {
            schema: base.schema.clone(),
            header_vars: match &self.header_vars { Some(d) => d.apply(&base.header_vars), None => base.header_vars.clone() },
            tables: match &self.tables { Some(d) => d.apply(&base.tables), None => base.tables.clone() },
            other_tables: base.other_tables.clone(),
            blocks: match &self.blocks { Some(d) => d.apply(&base.blocks), None => base.blocks.clone() },
            entities: match &self.entities { Some(d) => d.apply(&base.entities), None => base.entities.clone() },
        }
    }

    /// ➕️ Structural, total, base-free sequential-coalesce (`## Absorb` contract): every
    /// collection uses its own generic absorb-pair transport; `tables` recurses into its own
    /// three sub-collections.
    fn absorb(&mut self, other: Self) {
        self.header_vars = match (self.header_vars.take(), other.header_vars) {
            (None, None) => None, (Some(a), None) => Some(a), (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => DxfHeaderVarsDiff::absorb(a, b),
        };
        self.tables = match (self.tables.take(), other.tables) {
            (None, None) => None, (Some(a), None) => Some(a), (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => DxfTablesDiff::absorb(a, b),
        };
        self.blocks = match (self.blocks.take(), other.blocks) {
            (None, None) => None, (Some(a), None) => Some(a), (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => DxfBlocksDiff::absorb(a, b),
        };
        self.entities = match (self.entities.take(), other.entities) {
            (None, None) => None, (Some(a), None) => Some(a), (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => DxfEntitiesDiff::absorb(a, b),
        };
    }
}

impl DiffAlgebra<DxfSnapshot> for DxfDiff {
    /// 🔁️ Diff-level undo, derived generically (correct by construction) via `apply` + `between`.
    fn inverse(&self, base: &DxfSnapshot) -> Self {
        let mutated = self.apply(base);
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
pub fn diff_set_snapshot(base: &DxfSnapshot, next: &DxfSnapshot) -> DxfDiff {
    DxfDiff::between(base, next)
}
//#endregion 🔖️Diff

//#region 🔖️MutationDiffBuilders
// 🧮 Item-level `between` wrappers, exposed to `🧬️mutations` so `SetLayer`/`SetStyle`/
// `SetLinetype`/`SetEntity`/`SetBlock`'s `diff()` can compute a sparse per-field patch without the
// private `DxfNamedElem`/`DxfIndexElem` traits themselves leaving this module.
pub fn header_var_diff_between(a: &DxfHeaderVar, b: &DxfHeaderVar) -> DxfHeaderVarDiff { <DxfHeaderVar as DxfNamedElem>::diff_between(a, b) }
pub fn layer_diff_between(a: &DxfLayer, b: &DxfLayer) -> DxfLayerDiff { <DxfLayer as DxfNamedElem>::diff_between(a, b) }
pub fn style_diff_between(a: &DxfStyle, b: &DxfStyle) -> DxfStyleDiff { <DxfStyle as DxfNamedElem>::diff_between(a, b) }
pub fn linetype_diff_between(a: &DxfLinetype, b: &DxfLinetype) -> DxfLinetypeDiff { <DxfLinetype as DxfNamedElem>::diff_between(a, b) }
pub fn entity_diff_between_pub(a: &DxfEntity, b: &DxfEntity) -> DxfEntityDiff { entity_diff_between(a, b) }
pub fn block_diff_between(a: &DxfBlock, b: &DxfBlock) -> DxfBlockDiff { <DxfBlock as DxfIndexElem>::diff_between(a, b) }

pub fn diff_set_header_var(index: usize, name: &str, header_var: DxfHeaderVar, existed: bool) -> DxfDiff {
    if existed {
        DxfDiff { header_vars: Some(DxfHeaderVarsDiff { removed: vec![], modified: vec![DxfHeaderVarModified { name: name.to_string(), diff: DxfHeaderVarDiff { group_code: Some(header_var.group_code), value: Some(header_var.value), extra_group_codes: Some(header_var.extra_group_codes) } }], added: vec![] }), ..Default::default() }
    } else {
        DxfDiff { header_vars: Some(DxfHeaderVarsDiff { removed: vec![], modified: vec![], added: vec![DxfHeaderVarAdded { index, header_var }] }), ..Default::default() }
    }
}
pub fn diff_remove_header_var(name: &str) -> DxfDiff {
    DxfDiff { header_vars: Some(DxfHeaderVarsDiff { removed: vec![name.to_string()], modified: vec![], added: vec![] }), ..Default::default() }
}

pub fn diff_insert_layer(index: usize, layer: DxfLayer) -> DxfDiff {
    DxfDiff { tables: Some(DxfTablesDiff { layers: Some(DxfLayersDiff { removed: vec![], modified: vec![], added: vec![DxfLayerAdded { index, layer }] }), ..Default::default() }), ..Default::default() }
}
pub fn diff_remove_layer(name: &str) -> DxfDiff {
    DxfDiff { tables: Some(DxfTablesDiff { layers: Some(DxfLayersDiff { removed: vec![name.to_string()], modified: vec![], added: vec![] }), ..Default::default() }), ..Default::default() }
}
pub fn diff_set_layer(name: &str, diff: DxfLayerDiff) -> DxfDiff {
    DxfDiff { tables: Some(DxfTablesDiff { layers: Some(DxfLayersDiff { removed: vec![], modified: vec![DxfLayerModified { name: name.to_string(), diff }], added: vec![] }), ..Default::default() }), ..Default::default() }
}

pub fn diff_insert_style(index: usize, style: DxfStyle) -> DxfDiff {
    DxfDiff { tables: Some(DxfTablesDiff { styles: Some(DxfStylesDiff { removed: vec![], modified: vec![], added: vec![DxfStyleAdded { index, style }] }), ..Default::default() }), ..Default::default() }
}
pub fn diff_remove_style(name: &str) -> DxfDiff {
    DxfDiff { tables: Some(DxfTablesDiff { styles: Some(DxfStylesDiff { removed: vec![name.to_string()], modified: vec![], added: vec![] }), ..Default::default() }), ..Default::default() }
}
pub fn diff_set_style(name: &str, diff: DxfStyleDiff) -> DxfDiff {
    DxfDiff { tables: Some(DxfTablesDiff { styles: Some(DxfStylesDiff { removed: vec![], modified: vec![DxfStyleModified { name: name.to_string(), diff }], added: vec![] }), ..Default::default() }), ..Default::default() }
}

pub fn diff_insert_linetype(index: usize, linetype: DxfLinetype) -> DxfDiff {
    DxfDiff { tables: Some(DxfTablesDiff { linetypes: Some(DxfLinetypesDiff { removed: vec![], modified: vec![], added: vec![DxfLinetypeAdded { index, linetype }] }), ..Default::default() }), ..Default::default() }
}
pub fn diff_remove_linetype(name: &str) -> DxfDiff {
    DxfDiff { tables: Some(DxfTablesDiff { linetypes: Some(DxfLinetypesDiff { removed: vec![name.to_string()], modified: vec![], added: vec![] }), ..Default::default() }), ..Default::default() }
}
pub fn diff_set_linetype(name: &str, diff: DxfLinetypeDiff) -> DxfDiff {
    DxfDiff { tables: Some(DxfTablesDiff { linetypes: Some(DxfLinetypesDiff { removed: vec![], modified: vec![DxfLinetypeModified { name: name.to_string(), diff }], added: vec![] }), ..Default::default() }), ..Default::default() }
}

pub fn diff_insert_entity(index: usize, entity: DxfEntity) -> DxfDiff {
    DxfDiff { entities: Some(DxfEntitiesDiff { removed: vec![], modified: vec![], added: vec![DxfEntityAdded { index, entity }] }), ..Default::default() }
}
pub fn diff_remove_entity(index: usize) -> DxfDiff {
    DxfDiff { entities: Some(DxfEntitiesDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }
}
pub fn diff_set_entity(index: usize, diff: DxfEntityDiff) -> DxfDiff {
    DxfDiff { entities: Some(DxfEntitiesDiff { removed: vec![], modified: vec![DxfEntityModified { index, diff }], added: vec![] }), ..Default::default() }
}

pub fn diff_insert_block(index: usize, block: DxfBlock) -> DxfDiff {
    DxfDiff { blocks: Some(DxfBlocksDiff { removed: vec![], modified: vec![], added: vec![DxfBlockAdded { index, block }] }), ..Default::default() }
}
pub fn diff_remove_block(index: usize) -> DxfDiff {
    DxfDiff { blocks: Some(DxfBlocksDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }
}
pub fn diff_set_block(index: usize, diff: DxfBlockDiff) -> DxfDiff {
    DxfDiff { blocks: Some(DxfBlocksDiff { removed: vec![], modified: vec![DxfBlockModified { index, diff }], added: vec![] }), ..Default::default() }
}
//#endregion 🔖️MutationDiffBuilders
