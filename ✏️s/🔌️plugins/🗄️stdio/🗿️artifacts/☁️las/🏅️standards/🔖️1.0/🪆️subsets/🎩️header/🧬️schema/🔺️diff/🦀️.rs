//! 🔺️ LasDiff — handcrafted sparse diff. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: replaces the old
//! `LasDiff{snapshot: Option<LasSnapshot>}` full-replace template with every real header field as
//! a top-level `Option<T>` scalar plus an index-keyed `vlrs` triple and an index-keyed `points`
//! triple, each entity individually patchable.

/// 🧩 Ordered removed keys, modified values, and inserted items.
pub(crate) type IndexedDiffParts<D, T> = (Vec<usize>, Vec<(usize, D)>, Vec<(usize, T)>);

use std::collections::{BTreeSet, HashMap, HashSet};

use crate::schema::snapshot::{LasHeader, LasPoint, LasVlr};
use crate::LasSnapshot;
use protocol::command::DiffAlgebra;
use protocol::{DiffCodec};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_indexed_targets(base_len: usize, removed_indices: &[usize], modified_indices: impl IntoIterator<Item = usize>, added_indices: impl IntoIterator<Item = usize>, target: &str) -> MutationApplyResult<()> {
    let mut removed = BTreeSet::new();
    for &index in removed_indices {
        if index >= base_len || !removed.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.invalid-remove-index", "removal target must exist exactly once").at([target, &index.to_string()]));
        }
    }
    let mut modified = BTreeSet::new();
    for index in modified_indices {
        if index >= base_len || removed.contains(&index) || !modified.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.invalid-modify-index", "modification target must exist exactly once and remain present").at([target, &index.to_string()]));
        }
    }
    let mut additions: Vec<usize> = added_indices.into_iter().collect();
    additions.sort_unstable();
    let mut previous = None;
    for (length, index) in (base_len - removed.len()..).zip(additions) {
        if index > length || previous == Some(index) {
            return Err(MutationApplyError::new("mutation.apply.invalid-add-index", "addition target must be unique and within the evolving sequence").at([target, &index.to_string()]));
        }
        previous = Some(index);
    }
    Ok(())
}
use framework_schema::ArtifactSchema;

//#region 🔖️IndexedAbsorb
/// 🏷️ Structural, base-free label used only inside [`absorb_indexed_triple`] to simulate the
/// two-step index-transform (base→mid via `d1`, mid→after via `d2`) — mirrors
/// `txt`'s `Lbl`/`simulate_labels`/`absorb_pair` pattern (own copy: no cross-artifact type
/// sharing), generalized once here over `vlrs` and `points` (same collection shape, same
/// artifact) instead of copy-pasted twice.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Lbl {
    Base(usize),
    Added1(usize),
    Added2(usize),
}

/// ➡️ Simulates one collection-triple's position algebra over an abstract label array: remove
/// the given base/mid indices, then insert `added` labels ascending at `min(index, current_len)`.
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

/// 🔺️ Borrowed removal, modification, and addition views for one indexed diff.
type IndexedDiffRef<'a, D, T> = (&'a [usize], &'a [(usize, D)], &'a [(usize, T)]);

/// ➕️ Generic index-keyed collection-triple absorb: `self` is base→mid (`d1`), `other` is
/// mid→after (`d2`). `merge_field_diff`/`patch_item` are the only per-entity-type logic —
/// everything else (index transport, annihilate-on-remove, patch-into-added) is the recipe's
/// normative algorithm, identical for `vlrs` and `points`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_indexed_triple<Item: Clone, D: Clone + Default + PartialEq>(
    (d1_removed, d1_modified, d1_added): IndexedDiffRef<'_, D, Item>,
    (d2_removed, d2_modified, d2_added): IndexedDiffRef<'_, D, Item>,
    absorb_field: impl Fn(&mut D, D),
    patch_item: impl Fn(&mut Item, &D),
) -> IndexedDiffParts<D, Item> {
    let max_ref =
        d1_removed.iter().copied().chain(d1_modified.iter().map(|(i, _)| *i)).chain(d1_added.iter().map(|(i, _)| *i)).chain(d2_removed.iter().copied()).chain(d2_modified.iter().map(|(i, _)| *i)).chain(d2_added.iter().map(|(i, _)| *i)).max();
    let l1 = max_ref.map_or(0, |m| m + 2);

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
        mid_labels.push(Lbl::Base(usize::MAX)); // inert padding, never referenced by mid_pos_of_base
    }

    let d2_added_lbl: Vec<(usize, Lbl)> = d2_added.iter().enumerate().map(|(k, (idx, _))| (*idx, Lbl::Added2(k))).collect();
    let after_labels = simulate_labels(mid_labels, d2_removed, &d2_added_lbl);

    let d1_modified_at: HashMap<usize, &D> = d1_modified.iter().map(|(i, d)| (*i, d)).collect();
    let d2_modified_at: HashMap<usize, &D> = d2_modified.iter().map(|(i, d)| (*i, d)).collect();

    let mut present_base: HashSet<usize> = HashSet::new();
    let mut modified: Vec<(usize, D)> = Vec::new();
    let mut added: Vec<(usize, Item)> = Vec::new();

    for (pos, l) in after_labels.into_iter().enumerate() {
        match l {
            Lbl::Base(i) if i != usize::MAX => {
                present_base.insert(i);
                let mid_pos = mid_pos_of_base.get(&i).copied();
                let d1v = d1_modified_at.get(&i).copied().cloned();
                let d2v = mid_pos.and_then(|m| d2_modified_at.get(&m)).copied().cloned();
                let merged = match (d1v, d2v) {
                    (None, None) => None,
                    (Some(a), None) => Some(a),
                    (None, Some(b)) => Some(b),
                    (Some(mut a), Some(b)) => {
                        absorb_field(&mut a, b);
                        Some(a)
                    }
                };
                if let Some(d) = merged {
                    if d != D::default() {
                        modified.push((i, d));
                    }
                }
            }
            Lbl::Base(_) => { /* padding survived untouched -- never real, ignore */ }
            Lbl::Added1(j) => {
                let mid_pos = mid_pos_of_added1.get(&j).copied();
                let mut item = d1_added[j].1.clone();
                if let Some(m) = mid_pos {
                    if let Some(d) = d2_modified_at.get(&m) {
                        patch_item(&mut item, d);
                    }
                }
                added.push((pos, item));
            }
            Lbl::Added2(k) => {
                added.push((pos, d2_added[k].1.clone()));
            }
        }
    }

    let removed: Vec<usize> = (0..l1).filter(|i| !present_base.contains(i)).collect();
    (removed, modified, added)
}
//#endregion 🔖️IndexedAbsorb

//#region 🔖️VlrDiff
/// 📦️ Sparse per-field patch for one `LasVlr`. `data` is retained/replaced byte-verbatim
/// (weak-value raw-retention field, never sub-diffed).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct LasVlrDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub record_id: Option<u16>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_vlr_diff(vlr: &mut LasVlr, diff: &LasVlrDiff) {
    if let Some(v) = &diff.user_id {
        vlr.user_id = v.clone();
    }
    if let Some(v) = diff.record_id {
        vlr.record_id = v;
    }
    if let Some(v) = &diff.description {
        vlr.description = v.clone();
    }
    if let Some(v) = &diff.data {
        vlr.data = v.clone();
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn vlr_between(a: &LasVlr, b: &LasVlr) -> LasVlrDiff {
    LasVlrDiff {
        user_id: (a.user_id != b.user_id).then(|| b.user_id.clone()),
        record_id: (a.record_id != b.record_id).then_some(b.record_id),
        description: (a.description != b.description).then(|| b.description.clone()),
        data: (a.data != b.data).then(|| b.data.clone()),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_vlr_diff(base: &mut LasVlrDiff, other: LasVlrDiff) {
    if other.user_id.is_some() {
        base.user_id = other.user_id;
    }
    if other.record_id.is_some() {
        base.record_id = other.record_id;
    }
    if other.description.is_some() {
        base.description = other.description;
    }
    if other.data.is_some() {
        base.data = other.data;
    }
}

/// 📦️ One `vlrs.modified[]` entity — `index` is the VLR's position in BASE.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct LasVlrModified {
    pub index: usize,
    pub diff: LasVlrDiff,
}

/// 📦️ One `vlrs.added[]` entity — `index` is the VLR's position in the FINAL sequence.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct LasVlrAdded {
    pub index: usize,
    pub vlr: LasVlr,
}

/// 📦️ Sparse index-keyed `vlrs` triple.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct LasVlrsDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<LasVlrModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<LasVlrAdded>,
}

impl LasVlrsDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }

    /// ▶️ Applies this triple: `modified` by BASE index (no-op if since-removed), then `removed`
    /// (descending order doesn't matter — collected as a set), then `added` ascending, clamped.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn apply_unchecked(&self, base: &[LasVlr]) -> Vec<LasVlr> {
        let mut items = base.to_vec();
        for m in &self.modified {
            apply_vlr_diff(&mut items[m.index], &m.diff);
        }
        let mut removed = self.removed.clone();
        removed.sort_unstable_by(|a, b| b.cmp(a));
        for index in removed {
            items.remove(index);
        }
        let mut added = self.added.clone();
        added.sort_by_key(|a| a.index);
        for a in added {
            items.insert(a.index, a.vlr);
        }
        items
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply(&self, base: &[LasVlr]) -> MutationApplyResult<Vec<LasVlr>> {
        validate_indexed_targets(base.len(), &self.removed, self.modified.iter().map(|value| value.index), self.added.iter().map(|value| value.index), "vlrs")?;
        Ok(self.apply_unchecked(base))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn between(base: &[LasVlr], next: &[LasVlr]) -> Self {
        let min_len = base.len().min(next.len());
        let mut modified = Vec::new();
        for i in 0..min_len {
            let d = vlr_between(&base[i], &next[i]);
            if d != LasVlrDiff::default() {
                modified.push(LasVlrModified { index: i, diff: d });
            }
        }
        let removed: Vec<usize> = (next.len()..base.len()).collect();
        let added: Vec<LasVlrAdded> = (base.len()..next.len()).map(|i| LasVlrAdded { index: i, vlr: next[i].clone() }).collect();
        LasVlrsDiff { removed, modified, added }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_vlrs(d1: Option<LasVlrsDiff>, d2: Option<LasVlrsDiff>) -> Option<LasVlrsDiff> {
    let (d1, d2) = match (d1, d2) {
        (None, None) => return None,
        (Some(d1), None) => return Some(d1),
        (None, Some(d2)) => return Some(d2),
        (Some(d1), Some(d2)) => (d1, d2),
    };
    let d1m: Vec<(usize, LasVlrDiff)> = d1.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
    let d1a: Vec<(usize, LasVlr)> = d1.added.iter().map(|a| (a.index, a.vlr.clone())).collect();
    let d2m: Vec<(usize, LasVlrDiff)> = d2.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
    let d2a: Vec<(usize, LasVlr)> = d2.added.iter().map(|a| (a.index, a.vlr.clone())).collect();
    let (removed, modified, added) = absorb_indexed_triple((&d1.removed, &d1m, &d1a), (&d2.removed, &d2m, &d2a), absorb_vlr_diff, apply_vlr_diff);
    let merged = LasVlrsDiff { removed, modified: modified.into_iter().map(|(index, diff)| LasVlrModified { index, diff }).collect(), added: added.into_iter().map(|(index, vlr)| LasVlrAdded { index, vlr }).collect() };
    if merged.is_empty() {
        None
    } else {
        Some(merged)
    }
}
//#endregion 🔖️VlrDiff

//#region 🔖️PointDiff
/// 📍️ Sparse per-field patch for one `LasPoint`. `gps_time`/`rgb` are tri-state:
/// `None` = unchanged, `Some(None)` = cleared (point demoted out of a format that carries it),
/// `Some(Some(v))` = set.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct LasPointDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub z: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub intensity: Option<u16>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub return_number: Option<u8>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub number_of_returns: Option<u8>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scan_direction_flag: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub edge_of_flight_line: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub classification: Option<u8>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scan_angle_rank: Option<i8>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub user_data: Option<u8>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub point_source_id: Option<u16>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub gps_time: Option<Option<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rgb: Option<Option<(u16, u16, u16)>>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_point_diff(p: &mut LasPoint, diff: &LasPointDiff) {
    if let Some(v) = diff.x {
        p.x = v;
    }
    if let Some(v) = diff.y {
        p.y = v;
    }
    if let Some(v) = diff.z {
        p.z = v;
    }
    if let Some(v) = diff.intensity {
        p.intensity = v;
    }
    if let Some(v) = diff.return_number {
        p.return_number = v;
    }
    if let Some(v) = diff.number_of_returns {
        p.number_of_returns = v;
    }
    if let Some(v) = diff.scan_direction_flag {
        p.scan_direction_flag = v;
    }
    if let Some(v) = diff.edge_of_flight_line {
        p.edge_of_flight_line = v;
    }
    if let Some(v) = diff.classification {
        p.classification = v;
    }
    if let Some(v) = diff.scan_angle_rank {
        p.scan_angle_rank = v;
    }
    if let Some(v) = diff.user_data {
        p.user_data = v;
    }
    if let Some(v) = diff.point_source_id {
        p.point_source_id = v;
    }
    if let Some(v) = diff.gps_time {
        p.gps_time = v;
    }
    if let Some(v) = diff.rgb {
        p.rgb = v;
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn point_between(a: &LasPoint, b: &LasPoint) -> LasPointDiff {
    LasPointDiff {
        x: (a.x != b.x).then_some(b.x),
        y: (a.y != b.y).then_some(b.y),
        z: (a.z != b.z).then_some(b.z),
        intensity: (a.intensity != b.intensity).then_some(b.intensity),
        return_number: (a.return_number != b.return_number).then_some(b.return_number),
        number_of_returns: (a.number_of_returns != b.number_of_returns).then_some(b.number_of_returns),
        scan_direction_flag: (a.scan_direction_flag != b.scan_direction_flag).then_some(b.scan_direction_flag),
        edge_of_flight_line: (a.edge_of_flight_line != b.edge_of_flight_line).then_some(b.edge_of_flight_line),
        classification: (a.classification != b.classification).then_some(b.classification),
        scan_angle_rank: (a.scan_angle_rank != b.scan_angle_rank).then_some(b.scan_angle_rank),
        user_data: (a.user_data != b.user_data).then_some(b.user_data),
        point_source_id: (a.point_source_id != b.point_source_id).then_some(b.point_source_id),
        gps_time: (a.gps_time != b.gps_time).then_some(b.gps_time),
        rgb: (a.rgb != b.rgb).then_some(b.rgb),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_point_diff(base: &mut LasPointDiff, other: &LasPointDiff) {
    if other.x.is_some() {
        base.x = other.x;
    }
    if other.y.is_some() {
        base.y = other.y;
    }
    if other.z.is_some() {
        base.z = other.z;
    }
    if other.intensity.is_some() {
        base.intensity = other.intensity;
    }
    if other.return_number.is_some() {
        base.return_number = other.return_number;
    }
    if other.number_of_returns.is_some() {
        base.number_of_returns = other.number_of_returns;
    }
    if other.scan_direction_flag.is_some() {
        base.scan_direction_flag = other.scan_direction_flag;
    }
    if other.edge_of_flight_line.is_some() {
        base.edge_of_flight_line = other.edge_of_flight_line;
    }
    if other.classification.is_some() {
        base.classification = other.classification;
    }
    if other.scan_angle_rank.is_some() {
        base.scan_angle_rank = other.scan_angle_rank;
    }
    if other.user_data.is_some() {
        base.user_data = other.user_data;
    }
    if other.point_source_id.is_some() {
        base.point_source_id = other.point_source_id;
    }
    if other.gps_time.is_some() {
        base.gps_time = other.gps_time;
    }
    if other.rgb.is_some() {
        base.rgb = other.rgb;
    }
}

/// 📍️ One `points.modified[]` entity — `index` is the point's position in BASE.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct LasPointModified {
    pub index: usize,
    pub diff: LasPointDiff,
}

/// 📍️ One `points.added[]` entity — `index` is the point's position in the FINAL sequence.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct LasPointAdded {
    pub index: usize,
    pub point: LasPoint,
}

/// 📍️ Sparse index-keyed `points` triple.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct LasPointsDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<LasPointModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<LasPointAdded>,
}

impl LasPointsDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn apply_unchecked(&self, base: &[LasPoint]) -> Vec<LasPoint> {
        let mut items = base.to_vec();
        for m in &self.modified {
            apply_point_diff(&mut items[m.index], &m.diff);
        }
        let mut removed = self.removed.clone();
        removed.sort_unstable_by(|a, b| b.cmp(a));
        for index in removed {
            items.remove(index);
        }
        let mut added = self.added.clone();
        added.sort_by_key(|a| a.index);
        for a in added {
            items.insert(a.index, a.point);
        }
        items
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply(&self, base: &[LasPoint]) -> MutationApplyResult<Vec<LasPoint>> {
        validate_indexed_targets(base.len(), &self.removed, self.modified.iter().map(|value| value.index), self.added.iter().map(|value| value.index), "points")?;
        Ok(self.apply_unchecked(base))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn between(base: &[LasPoint], next: &[LasPoint]) -> Self {
        let min_len = base.len().min(next.len());
        let mut modified = Vec::new();
        for i in 0..min_len {
            let d = point_between(&base[i], &next[i]);
            if d != LasPointDiff::default() {
                modified.push(LasPointModified { index: i, diff: d });
            }
        }
        let removed: Vec<usize> = (next.len()..base.len()).collect();
        let added: Vec<LasPointAdded> = (base.len()..next.len()).map(|i| LasPointAdded { index: i, point: next[i].clone() }).collect();
        LasPointsDiff { removed, modified, added }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_points(d1: Option<LasPointsDiff>, d2: Option<LasPointsDiff>) -> Option<LasPointsDiff> {
    let (d1, d2) = match (d1, d2) {
        (None, None) => return None,
        (Some(d1), None) => return Some(d1),
        (None, Some(d2)) => return Some(d2),
        (Some(d1), Some(d2)) => (d1, d2),
    };
    let d1m: Vec<(usize, LasPointDiff)> = d1.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
    let d1a: Vec<(usize, LasPoint)> = d1.added.iter().map(|a| (a.index, a.point.clone())).collect();
    let d2m: Vec<(usize, LasPointDiff)> = d2.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
    let d2a: Vec<(usize, LasPoint)> = d2.added.iter().map(|a| (a.index, a.point.clone())).collect();
    let (removed, modified, added) = absorb_indexed_triple((&d1.removed, &d1m, &d1a), (&d2.removed, &d2m, &d2a), |base, other| absorb_point_diff(base, &other), apply_point_diff);
    let merged = LasPointsDiff { removed, modified: modified.into_iter().map(|(index, diff)| LasPointModified { index, diff }).collect(), added: added.into_iter().map(|(index, point)| LasPointAdded { index, point }).collect() };
    if merged.is_empty() {
        None
    } else {
        Some(merged)
    }
}
//#endregion 🔖️PointDiff

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.las`. Every real header field is a top-level scalar; `schema` is an
/// identity field and never appears here.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.las.diff")]
pub struct LasDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub version_major: Option<u8>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub version_minor: Option<u8>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub system_identifier: Option<String>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub generating_software: Option<String>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub creation_day_of_year: Option<u16>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub creation_year: Option<u16>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub header_size: Option<u16>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub offset_to_point_data: Option<u32>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub number_of_vlrs: Option<u32>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub point_data_format_id: Option<u8>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub point_data_record_length: Option<u16>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub number_of_point_records: Option<u32>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub points_by_return: Option<[u32; 5]>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub x_scale: Option<f64>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub y_scale: Option<f64>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub z_scale: Option<f64>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub x_offset: Option<f64>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub y_offset: Option<f64>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub z_offset: Option<f64>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub max_x: Option<f64>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub min_x: Option<f64>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub max_y: Option<f64>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub min_y: Option<f64>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub max_z: Option<f64>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub min_z: Option<f64>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub vlrs: Option<LasVlrsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub points: Option<LasPointsDiff>,
}

/// ▶️ Applies every header scalar patch onto `header` in place.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_header_diff(header: &mut LasHeader, d: &LasDiff) {
    if let Some(v) = d.version_major {
        header.version_major = v;
    }
    if let Some(v) = d.version_minor {
        header.version_minor = v;
    }
    if let Some(v) = &d.system_identifier {
        header.system_identifier = v.clone();
    }
    if let Some(v) = &d.generating_software {
        header.generating_software = v.clone();
    }
    if let Some(v) = d.creation_day_of_year {
        header.creation_day_of_year = v;
    }
    if let Some(v) = d.creation_year {
        header.creation_year = v;
    }
    if let Some(v) = d.header_size {
        header.header_size = v;
    }
    if let Some(v) = d.offset_to_point_data {
        header.offset_to_point_data = v;
    }
    if let Some(v) = d.number_of_vlrs {
        header.number_of_vlrs = v;
    }
    if let Some(v) = d.point_data_format_id {
        header.point_data_format_id = v;
    }
    if let Some(v) = d.point_data_record_length {
        header.point_data_record_length = v;
    }
    if let Some(v) = d.number_of_point_records {
        header.number_of_point_records = v;
    }
    if let Some(v) = d.points_by_return {
        header.points_by_return = v;
    }
    if let Some(v) = d.x_scale {
        header.x_scale = v;
    }
    if let Some(v) = d.y_scale {
        header.y_scale = v;
    }
    if let Some(v) = d.z_scale {
        header.z_scale = v;
    }
    if let Some(v) = d.x_offset {
        header.x_offset = v;
    }
    if let Some(v) = d.y_offset {
        header.y_offset = v;
    }
    if let Some(v) = d.z_offset {
        header.z_offset = v;
    }
    if let Some(v) = d.max_x {
        header.max_x = v;
    }
    if let Some(v) = d.min_x {
        header.min_x = v;
    }
    if let Some(v) = d.max_y {
        header.max_y = v;
    }
    if let Some(v) = d.min_y {
        header.min_y = v;
    }
    if let Some(v) = d.max_z {
        header.max_z = v;
    }
    if let Some(v) = d.min_z {
        header.min_z = v;
    }
}

impl MutationDiff<LasSnapshot> for LasDiff {
    fn apply(&self, base: &LasSnapshot, _capability: protocol::ApplyCapability) -> MutationApplyResult<LasSnapshot> {
        if let Some(diff) = &self.vlrs {
            validate_indexed_targets(base.vlrs.len(), &diff.removed, diff.modified.iter().map(|value| value.index), diff.added.iter().map(|value| value.index), "vlrs")?;
        }
        if let Some(diff) = &self.points {
            validate_indexed_targets(base.points.len(), &diff.removed, diff.modified.iter().map(|value| value.index), diff.added.iter().map(|value| value.index), "points")?;
        }
        let mut header = base.header.clone();
        apply_header_diff(&mut header, self);
        let vlrs = match &self.vlrs {
            Some(vd) => vd.apply_unchecked(&base.vlrs),
            None => base.vlrs.clone(),
        };
        let points = match &self.points {
            Some(pd) => pd.apply_unchecked(&base.points),
            None => base.points.clone(),
        };
        Ok(LasSnapshot { schema: base.schema.clone(), header, vlrs, points })
    }

    /// ➕️ Structural, total, base-free sequential-coalesce (`## Absorb` contract). Header
    /// scalars: LWW. `vlrs`/`points`: index-transport via [`absorb_indexed_triple`].
    fn absorb(&mut self, other: Self) {
        if other.version_major.is_some() {
            self.version_major = other.version_major;
        }
        if other.version_minor.is_some() {
            self.version_minor = other.version_minor;
        }
        if other.system_identifier.is_some() {
            self.system_identifier = other.system_identifier;
        }
        if other.generating_software.is_some() {
            self.generating_software = other.generating_software;
        }
        if other.creation_day_of_year.is_some() {
            self.creation_day_of_year = other.creation_day_of_year;
        }
        if other.creation_year.is_some() {
            self.creation_year = other.creation_year;
        }
        if other.header_size.is_some() {
            self.header_size = other.header_size;
        }
        if other.offset_to_point_data.is_some() {
            self.offset_to_point_data = other.offset_to_point_data;
        }
        if other.number_of_vlrs.is_some() {
            self.number_of_vlrs = other.number_of_vlrs;
        }
        if other.point_data_format_id.is_some() {
            self.point_data_format_id = other.point_data_format_id;
        }
        if other.point_data_record_length.is_some() {
            self.point_data_record_length = other.point_data_record_length;
        }
        if other.number_of_point_records.is_some() {
            self.number_of_point_records = other.number_of_point_records;
        }
        if other.points_by_return.is_some() {
            self.points_by_return = other.points_by_return;
        }
        if other.x_scale.is_some() {
            self.x_scale = other.x_scale;
        }
        if other.y_scale.is_some() {
            self.y_scale = other.y_scale;
        }
        if other.z_scale.is_some() {
            self.z_scale = other.z_scale;
        }
        if other.x_offset.is_some() {
            self.x_offset = other.x_offset;
        }
        if other.y_offset.is_some() {
            self.y_offset = other.y_offset;
        }
        if other.z_offset.is_some() {
            self.z_offset = other.z_offset;
        }
        if other.max_x.is_some() {
            self.max_x = other.max_x;
        }
        if other.min_x.is_some() {
            self.min_x = other.min_x;
        }
        if other.max_y.is_some() {
            self.max_y = other.max_y;
        }
        if other.min_y.is_some() {
            self.min_y = other.min_y;
        }
        if other.max_z.is_some() {
            self.max_z = other.max_z;
        }
        if other.min_z.is_some() {
            self.min_z = other.min_z;
        }
        self.vlrs = absorb_vlrs(self.vlrs.take(), other.vlrs);
        self.points = absorb_points(self.points.take(), other.points);
    }
}

impl DiffAlgebra<LasSnapshot> for LasDiff {
    /// 🔁️ Diff-level undo, derived generically: the state delta from `self.apply(base)` back to
    /// `base` — `between` is the single source of truth for turning a state pair into a diff.
    fn inverse(&self, base: &LasSnapshot) -> Self {
        let mutated = {
            let mut header = base.header.clone();
            apply_header_diff(&mut header, self);
            LasSnapshot {
                schema: base.schema.clone(),
                header,
                vlrs: self.vlrs.as_ref().map_or_else(|| base.vlrs.clone(), |value| value.apply_unchecked(&base.vlrs)),
                points: self.points.as_ref().map_or_else(|| base.points.clone(), |value| value.apply_unchecked(&base.points)),
            }
        };
        Self::between(&mutated, base)
    }

    /// 🧭️ State delta (compose `GetXDiff`): header scalars compared field-by-field; `vlrs`/
    /// `points` index-keyed matching (pairwise `0..min(len)` = modified, base tail = removed,
    /// other tail = added — the recipe's "index keys pairwise by position" rule).
    fn between(base: &LasSnapshot, other: &LasSnapshot) -> Self {
        let bh = &base.header;
        let oh = &other.header;
        let vlrs_diff = LasVlrsDiff::between(&base.vlrs, &other.vlrs);
        let points_diff = LasPointsDiff::between(&base.points, &other.points);
        LasDiff {
            version_major: (bh.version_major != oh.version_major).then_some(oh.version_major),
            version_minor: (bh.version_minor != oh.version_minor).then_some(oh.version_minor),
            system_identifier: (bh.system_identifier != oh.system_identifier).then(|| oh.system_identifier.clone()),
            generating_software: (bh.generating_software != oh.generating_software).then(|| oh.generating_software.clone()),
            creation_day_of_year: (bh.creation_day_of_year != oh.creation_day_of_year).then_some(oh.creation_day_of_year),
            creation_year: (bh.creation_year != oh.creation_year).then_some(oh.creation_year),
            header_size: (bh.header_size != oh.header_size).then_some(oh.header_size),
            offset_to_point_data: (bh.offset_to_point_data != oh.offset_to_point_data).then_some(oh.offset_to_point_data),
            number_of_vlrs: (bh.number_of_vlrs != oh.number_of_vlrs).then_some(oh.number_of_vlrs),
            point_data_format_id: (bh.point_data_format_id != oh.point_data_format_id).then_some(oh.point_data_format_id),
            point_data_record_length: (bh.point_data_record_length != oh.point_data_record_length).then_some(oh.point_data_record_length),
            number_of_point_records: (bh.number_of_point_records != oh.number_of_point_records).then_some(oh.number_of_point_records),
            points_by_return: (bh.points_by_return != oh.points_by_return).then_some(oh.points_by_return),
            x_scale: (bh.x_scale != oh.x_scale).then_some(oh.x_scale),
            y_scale: (bh.y_scale != oh.y_scale).then_some(oh.y_scale),
            z_scale: (bh.z_scale != oh.z_scale).then_some(oh.z_scale),
            x_offset: (bh.x_offset != oh.x_offset).then_some(oh.x_offset),
            y_offset: (bh.y_offset != oh.y_offset).then_some(oh.y_offset),
            z_offset: (bh.z_offset != oh.z_offset).then_some(oh.z_offset),
            max_x: (bh.max_x != oh.max_x).then_some(oh.max_x),
            min_x: (bh.min_x != oh.min_x).then_some(oh.min_x),
            max_y: (bh.max_y != oh.max_y).then_some(oh.max_y),
            min_y: (bh.min_y != oh.min_y).then_some(oh.min_y),
            max_z: (bh.max_z != oh.max_z).then_some(oh.max_z),
            min_z: (bh.min_z != oh.min_z).then_some(oh.min_z),
            vlrs: if vlrs_diff.is_empty() { None } else { Some(vlrs_diff) },
            points: if points_diff.is_empty() { None } else { Some(points_diff) },
        }
    }

    fn is_empty(&self) -> bool {
        self == &LasDiff::default()
    }
}

/// 🧩 `SetSnapshot`'s diff is the sparse field-by-field `between(base, next)` — no full-replace
/// slot exists on `LasDiff` to short-circuit into.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &LasSnapshot, next: &LasSnapshot) -> LasDiff {
    LasDiff::between(base, next)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_version(major: u8, minor: u8) -> LasDiff {
    LasDiff { version_major: Some(major), version_minor: Some(minor), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_system_identifier(system_identifier: &str) -> LasDiff {
    LasDiff { system_identifier: Some(system_identifier.to_string()), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_software_info(generating_software: &str) -> LasDiff {
    LasDiff { generating_software: Some(generating_software.to_string()), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_creation_date(day_of_year: u16, year: u16) -> LasDiff {
    LasDiff { creation_day_of_year: Some(day_of_year), creation_year: Some(year), ..Default::default() }
}
/// 📏️ One axis of [`diff_set_scale_and_offset`]: the point RECORD this coordinate was decoded
/// from, re-read under the new scale/offset. `record = round((value - offset) / scale)` is the
/// exact inverse of the LAS public-header rule `coordinate = record * scale + offset`, so the
/// integer this returns to is the one on disk. A zero `from_scale` is not a legal LAS header and
/// carries no record to preserve, so the coordinate is left alone rather than turned into NaN.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn coordinate_under(value: f64, from_scale: f64, from_offset: f64, to_scale: f64, to_offset: f64) -> f64 {
    if from_scale == 0.0 {
        return value;
    }
    ((value - from_offset) / from_scale).round() * to_scale + to_offset
}

/// 📏️ Sets the six public-header-block fields that state HOW the on-disk integer point records
/// are turned into real-world coordinates — and changes nothing else about the file's contents.
///
/// ⚠️ This snapshot stores each point in REAL-WORLD coordinates and re-derives its integer record
/// on encode, which is the opposite primacy from the file itself: LAS §"Public Header Block"
/// defines `coordinate = record * scale + offset`, so the record is what the document carries and
/// the coordinate is what is computed from it. Writing only the six header fields therefore used
/// to hold the COORDINATES fixed and silently rewrite all 8,448 point records — a re-quantization,
/// not the header edit this kind is named for. That reading is also lossy in one direction: moving
/// to a COARSER scale rounds every coordinate away and `LasMutation::inverse`'s "put the old scale
/// and offset back" cannot bring it back, so the inverse law held only for rows that happen to
/// refine. The mutation now does what its name says — the records stay exactly where they are and
/// every coordinate is re-read from its own record under the new parameters, which is lossless and
/// exactly invertible for any scale in either direction. Reproduced by
/// `mutate-las-1-0::mutate-set-scale-and-offset` in the parity phase, where the reference
/// (`las::raw`, whose model IS the record) reported `$.points[1].x` 583000.246 against our
/// 583000.491 across 24,320 differences (ticket `26/08/23/END-TO-END-TESTING-REFACTOR`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_scale_and_offset(base: &LasSnapshot, scale: (f64, f64, f64), offset: (f64, f64, f64)) -> LasDiff {
    let modified: Vec<LasPointModified> = base
        .points
        .iter()
        .enumerate()
        .filter_map(|(index, point)| {
            let held = LasPoint {
                x: coordinate_under(point.x, base.header.x_scale, base.header.x_offset, scale.0, offset.0),
                y: coordinate_under(point.y, base.header.y_scale, base.header.y_offset, scale.1, offset.1),
                z: coordinate_under(point.z, base.header.z_scale, base.header.z_offset, scale.2, offset.2),
                ..point.clone()
            };
            let d = point_between(point, &held);
            (d != LasPointDiff::default()).then_some(LasPointModified { index, diff: d })
        })
        .collect();
    LasDiff {
        x_scale: Some(scale.0),
        y_scale: Some(scale.1),
        z_scale: Some(scale.2),
        x_offset: Some(offset.0),
        y_offset: Some(offset.1),
        z_offset: Some(offset.2),
        points: (!modified.is_empty()).then_some(LasPointsDiff { removed: vec![], modified, added: vec![] }),
        ..Default::default()
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_bounds(max: (f64, f64, f64), min: (f64, f64, f64)) -> LasDiff {
    LasDiff { max_x: Some(max.0), max_y: Some(max.1), max_z: Some(max.2), min_x: Some(min.0), min_y: Some(min.1), min_z: Some(min.2), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_points_by_return(counts: [u32; 5]) -> LasDiff {
    LasDiff { points_by_return: Some(counts), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_vlr(base: &LasSnapshot, index: usize, vlr: LasVlr) -> LasDiff {
    // 🧭️ Derived from the REAL collection length (`base.vlrs.len()`), never `base.header
    // .number_of_vlrs` — the header field can be desynced from reality (a raw-decoded fixture,
    // or a directly-constructed test snapshot), and `apply_las_mutation`'s imperative body
    // likewise recomputes from `snapshot.vlrs.len()` post-insert; both sides must agree for
    // `mutation_diff_law` to hold unconditionally, not just on already-synced fixtures.
    LasDiff { number_of_vlrs: Some((base.vlrs.len() + 1) as u32), vlrs: Some(LasVlrsDiff { removed: vec![], modified: vec![], added: vec![LasVlrAdded { index, vlr }] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_vlr(base: &LasSnapshot, index: usize) -> LasDiff {
    LasDiff { number_of_vlrs: Some(base.vlrs.len().saturating_sub(1) as u32), vlrs: Some(LasVlrsDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_vlr_data(index: usize, data: Vec<u8>) -> LasDiff {
    LasDiff { vlrs: Some(LasVlrsDiff { removed: vec![], modified: vec![LasVlrModified { index, diff: LasVlrDiff { data: Some(data), ..Default::default() } }], added: vec![] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_point(base: &LasSnapshot, index: usize, point: LasPoint) -> LasDiff {
    // 🧭️ See `diff_insert_vlr`'s doc comment — derived from `base.points.len()`, not the
    // (possibly-desynced) `base.header.number_of_point_records`.
    LasDiff { number_of_point_records: Some((base.points.len() + 1) as u32), points: Some(LasPointsDiff { removed: vec![], modified: vec![], added: vec![LasPointAdded { index, point }] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_point(base: &LasSnapshot, index: usize) -> LasDiff {
    LasDiff { number_of_point_records: Some(base.points.len().saturating_sub(1) as u32), points: Some(LasPointsDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_point(base: &LasSnapshot, index: usize, point: &LasPoint) -> LasDiff {
    let d = point_between(base.points.get(index).unwrap_or(&LasPoint::default()), point);
    if base.points.get(index).is_some() && d == LasPointDiff::default() {
        return LasDiff::default();
    }
    LasDiff { points: Some(LasPointsDiff { removed: vec![], modified: vec![LasPointModified { index, diff: d }], added: vec![] }), ..Default::default() }
}
//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6 (las, recon-gap-fill — this artifact was MISSED by the F6 recon sweep's §8
/// classification table entirely): **hand-rolled** `protocol::DiffCodec` for `LasDiff` — the
/// derive path (`#[derive(dsl::DslDiff)]`) is NOT usable here. STEP 1 classification done for
/// real (attribute added, `cargo check -p semio-s-plugin-stdio --lib` run, real errors read, then
/// reverted): two independent, confirmed blockers —
///
/// 1. **3b (tri-state)**, the recon's documented rule: `LasPointDiff::gps_time: Option<Option<f64>>`
///    and `LasPointDiff::rgb: Option<Option<(u16, u16, u16)>>` — real compiler output:
///    `error[E0277]: the trait bound `std::option::Option<f64>: DslField` is not satisfied` at
///    `🔺️diff/component.rs:311` (`pub gps_time: Option<Option<f64>>`) and
///    `error[E0277]: the trait bound `std::option::Option<(u16, u16, u16)>: DslField` is not
///    satisfied` at `🔺️diff/component.rs:313` (`pub rgb: Option<Option<(u16, u16, u16)>>`).
/// 2. **A THIRD blocker not named by the recon's 3a/3b taxonomy**: `LasPoint::rgb`'s inner type is
///    a bare tuple `(u16, u16, u16)`, and — same root cause as 3b's missing
///    `impl<T: DslField> DslField for Option<T>` — there is no blanket `impl<A: DslField, B: DslField,
///    ...> DslField for (A, B, ...)` anywhere in the `dsl` crate either (confirmed by the SAME
///    compiler error above: even a single-layer `Option<(u16, u16, u16)>` would fail to bind, tri-state
///    or not). The Mutation side hits this independently and more directly: `LasMutation::SetScaleAndOffset`/
///    `SetBounds` carry bare `(f64, f64, f64)` fields — confirmed via a SEPARATE real `#[derive(semio_framework_dsl_record_derive::DslEnum)]`
///    probe on `LasMutation`: `error[E0277]: the trait bound `(f64, f64, f64): DslField` is not satisfied`
///    (4 occurrences, `scale`/`offset`/`max`/`min`). Both `LasDiff` and `LasMutation` are hand-rolled.
///
/// **Grammar** (real, not `serde_json`): one space-separated `name=value` token per changed
/// top-level field (absent token = unchanged — every `LasDiff` top-level field is a PLAIN
/// `Option<T>`, never tri-state, since no `LasHeader` field is itself optional in the snapshot);
/// `vlrs`/`points` print as `name{[removed];[modified];[added]}` sections (same collection-triple
/// shape as gif 89a's `frames`/`comments`/`app_extensions`). Strings/byte payloads are lowercase
/// hex (no external base64 dep, matches this artifact's own `ArtifactDsl` hex-dump convention and
/// gif 89a/svg's established local idiom). `Option<T>` (both real optional fields AND the
/// `LasPointDiff` tri-states) use the uniform `[0]`=None / `[1,<T>]`=Some(T) tag. Structs are
/// positional `[f1,f2,...]` tuples. `LasVlrDiff`/`LasPointDiff`'s own sparse fields print as
/// single-letter `tag:value` pairs, matching gif 89a's `GifFrameDiff` convention (`LasVlrDiff`:
/// `U`/`R`/`N`/`X`; `LasPointDiff`: `X`/`Y`/`Z`/`I`/`R`/`N`/`D`/`E`/`C`/`A`/`U`/`P`/`G`/`B` — each
/// namespace is local to its own `[...]` block, no cross-type collision).
//#region 🔖️Primitives













//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs








//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs













//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel



//#region 🔖️BinaryDiffCodec
/// 🧪️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: REAL
/// LEB128-varint-framed binary twins of the hex-text codecs above, backing the upgraded
/// `DiffBinary::encode_diff`/`decode_diff` below (`#region 🔖️TopLevel`) — replaces the old F6
/// `print_diff().into_bytes()` text-as-binary shortcut (per the P2-W0 census, 100% of stdio's
/// `DiffCodec` impls were still on it before this pilot ladder; the shortcut this file used to
/// carry is documented, and confirmed removed, in this same region). `LasDiff` is FLAT — every
/// header field a top-level `Option<T>` scalar, matching zip's own `ZipDiff` shape (not json's
/// self-recursive `JsonValue`) — so the header is real, individually-walkable binary: a `u32`
/// bitmask (bit i = field i present, declaration order) followed by only the present fields'
/// values, then two runtime-counted index-keyed triples (`vlrs`/`points`). The triples hit the
/// SAME category of gap zip's own `entries` hits (`protocol-array-of-records`, filed in this
/// wave's `mechanism_gaps`): `Block::Repeat`'s arms are tag-dispatched per iteration and
/// `Prim::Array` only repeats one fixed-width scalar, neither can express "repeat N times, N from
/// a runtime count, each iteration a multi-field record" — the Rust encode/decode below IS
/// genuinely, fully structured binary all the way down (round-trip tested by
/// `diff_codec_text_binary_roundtrip_law`), only the sibling `../💾️binary/📡️component.protocol.
/// semio` file's DESCRIPTION of it bottoms out in one opaque trailing chain past the two real
/// leading fields (`format`, `header_mask`).
//#region 🔖️BinaryPrimitives




//#endregion 🔖️BinaryPrimitives

//#region 🔖️RecordBinaryCodec








//#endregion 🔖️RecordBinaryCodec

//#region 🔖️SparseDiffBinaryCodec































//#endregion 🔖️SparseDiffBinaryCodec

//#region 🔖️HeaderMaskBits



























//#endregion 🔖️HeaderMaskBits
//#endregion 🔖️BinaryDiffCodec


//#endregion 🔖️TopLevel

/// 🧪️ Shared demo-case fixtures (moved out of `mod tests` so both `demo_diff_cases` below AND
/// `mod tests` itself can use them without a `tests::` qualification cycle).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn base_point(seed: u8) -> LasPoint {
    LasPoint {
        x: 100.0 + seed as f64,
        y: -50.0 + seed as f64 * 0.5,
        z: 10.0 + seed as f64 * 0.1,
        intensity: 100 + seed as u16,
        return_number: (seed % 5) + 1,
        number_of_returns: ((seed + 1) % 5) + 1,
        scan_direction_flag: seed % 2 == 0,
        edge_of_flight_line: seed % 3 == 0,
        classification: seed,
        scan_angle_rank: seed as i8 - 10,
        user_data: seed,
        point_source_id: 1000 + seed as u16,
        gps_time: None,
        rgb: None,
    }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn base_vlr(record_id: u16) -> LasVlr {
    LasVlr { user_id: "LASF_Spec".into(), record_id, description: format!("vlr {record_id}"), data: vec![record_id as u8; 3] }
}

/// 🧪️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: representative
/// `LasDiff` cases (empty, and both directions of a real `between()` over two fully-populated
/// snapshots) — single source of truth shared by `diff_codec_text_binary_roundtrip_law` below AND
/// `⚙️engine/🦀️.rs`'s `diff_grammar_conformance_law`/`protocol_walk_law` conformance
/// tests, per CLAUDE.md (no duplicated literal case lists).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<LasDiff> {
    let mut pa0 = base_point(1);
    pa0.gps_time = Some(1000.0);
    let a = LasSnapshot {
        schema: "stdio.las".into(),
        header: LasHeader {
            version_major: 1,
            version_minor: 2,
            system_identifier: "before-system".into(),
            generating_software: "before-software".into(),
            creation_day_of_year: 10,
            creation_year: 2020,
            header_size: 227,
            offset_to_point_data: 227,
            number_of_vlrs: 2,
            point_data_format_id: 1,
            point_data_record_length: 28,
            number_of_point_records: 2,
            points_by_return: [1, 1, 0, 0, 0],
            x_scale: 0.01,
            y_scale: 0.01,
            z_scale: 0.01,
            x_offset: 0.0,
            y_offset: 0.0,
            z_offset: 0.0,
            max_x: 100.0,
            min_x: 0.0,
            max_y: 100.0,
            min_y: 0.0,
            max_z: 100.0,
            min_z: 0.0,
        },
        vlrs: vec![base_vlr(100), base_vlr(101)],
        points: vec![pa0, base_point(2)],
    };
    let b = LasSnapshot {
        schema: "stdio.las".into(),
        header: LasHeader {
            version_major: 2,
            version_minor: 4,
            system_identifier: "after-system".into(),
            generating_software: "after-software".into(),
            creation_day_of_year: 250,
            creation_year: 2026,
            header_size: 375,
            offset_to_point_data: 500,
            number_of_vlrs: 1,
            point_data_format_id: 3,
            point_data_record_length: 34,
            number_of_point_records: 3,
            points_by_return: [0, 0, 2, 1, 0],
            x_scale: 0.001,
            y_scale: 0.001,
            z_scale: 0.001,
            x_offset: 500.0,
            y_offset: 500.0,
            z_offset: 10.0,
            max_x: 999.0,
            min_x: -1.0,
            max_y: 999.0,
            min_y: -1.0,
            max_z: 50.0,
            min_z: -50.0,
        },
        vlrs: vec![base_vlr(9)],
        points: vec![LasPoint { gps_time: None, rgb: Some((10, 20, 30)), ..base_point(9) }, base_point(2), base_point(3)],
    };
    vec![LasDiff::default(), <LasDiff as DiffAlgebra<LasSnapshot>>::between(&a, &b), <LasDiff as DiffAlgebra<LasSnapshot>>::between(&b, &a)]
}
//#endregion 🔖️HandcraftedDiffCodec

//#region Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion Tests

#[cfg(test)]
use protocol::{DiffBinary,DiffText};
