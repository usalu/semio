//! 🔺️ LasDiff — handcrafted sparse diff. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: replaces the old
//! `LasDiff{snapshot: Option<LasSnapshot>}` full-replace template with every real header field as
//! a top-level `Option<T>` scalar plus an index-keyed `vlrs` triple and an index-keyed `points`
//! triple, each entity individually patchable.

use std::collections::{BTreeSet, HashMap, HashSet};

use crate::artifacts::las::schema::snapshot::{LasHeader, LasPoint, LasVlr};
use crate::artifacts::las::LasSnapshot;
use protocol::command::DiffAlgebra;
use protocol::DiffCodec;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_indexed_targets(base_len: usize, removed_indices: &[usize], modified_indices: impl IntoIterator<Item = usize>, added_indices: impl IntoIterator<Item = usize>, target: &str) -> MutationApplyResult<()> {
    let mut removed = BTreeSet::new();
    for &index in removed_indices {
        if index >= base_len || !removed.insert(index) {
            return Err(MutationApplyError::new("invalid-remove-index", "removal target must exist exactly once").await.at([target, &index.to_string()]));
        }
    }
    let mut modified = BTreeSet::new();
    for index in modified_indices {
        if index >= base_len || removed.contains(&index) || !modified.insert(index) {
            return Err(MutationApplyError::new("invalid-modify-index", "modification target must exist exactly once and remain present").await.at([target, &index.to_string()]));
        }
    }
    let mut length = base_len - removed.len();
    let mut additions: Vec<usize> = added_indices.into_iter().collect();
    additions.sort_unstable();
    let mut previous = None;
    for index in additions {
        if index > length || previous == Some(index) {
            return Err(MutationApplyError::new("invalid-add-index", "addition target must be unique and within the evolving sequence").await.at([target, &index.to_string()]));
        }
        previous = Some(index);
        length += 1;
    }
    Ok(())
}
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

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

/// ➕️ Generic index-keyed collection-triple absorb: `self` is base→mid (`d1`), `other` is
/// mid→after (`d2`). `merge_field_diff`/`patch_item` are the only per-entity-type logic —
/// everything else (index transport, annihilate-on-remove, patch-into-added) is the recipe's
/// normative algorithm, identical for `vlrs` and `points`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_indexed_triple<Item: Clone, D: Clone + Default + PartialEq>(
    d1_removed: &[usize],
    d1_modified: &[(usize, D)],
    d1_added: &[(usize, Item)],
    d2_removed: &[usize],
    d2_modified: &[(usize, D)],
    d2_added: &[(usize, Item)],
    absorb_field: impl Fn(&mut D, D),
    patch_item: impl Fn(&mut Item, &D),
) -> (Vec<usize>, Vec<(usize, D)>, Vec<(usize, Item)>) {
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
                let d1v = d1_modified_at.get(&i).cloned().cloned();
                let d2v = mid_pos.and_then(|m| d2_modified_at.get(&m)).cloned().cloned();
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LasVlrDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_id: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LasVlrModified {
    pub index: usize,
    pub diff: LasVlrDiff,
}

/// 📦️ One `vlrs.added[]` entity — `index` is the VLR's position in the FINAL sequence.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LasVlrAdded {
    pub index: usize,
    pub vlr: LasVlr,
}

/// 📦️ Sparse index-keyed `vlrs` triple.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LasVlrsDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<LasVlrModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    let (removed, modified, added) = absorb_indexed_triple(&d1.removed, &d1m, &d1a, &d2.removed, &d2m, &d2a, absorb_vlr_diff, apply_vlr_diff);
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LasPointDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub z: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intensity: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_number: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_of_returns: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scan_direction_flag: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edge_of_flight_line: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classification: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scan_angle_rank: Option<i8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_data: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub point_source_id: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gps_time: Option<Option<f64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
fn absorb_point_diff(base: &mut LasPointDiff, other: LasPointDiff) {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LasPointModified {
    pub index: usize,
    pub diff: LasPointDiff,
}

/// 📍️ One `points.added[]` entity — `index` is the point's position in the FINAL sequence.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LasPointAdded {
    pub index: usize,
    pub point: LasPoint,
}

/// 📍️ Sparse index-keyed `points` triple.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LasPointsDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<LasPointModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    let (removed, modified, added) = absorb_indexed_triple(&d1.removed, &d1m, &d1a, &d2.removed, &d2m, &d2a, absorb_point_diff, apply_point_diff);
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.las.diff")]
pub struct LasDiff {
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_major: Option<u8>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_minor: Option<u8>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_identifier: Option<String>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generating_software: Option<String>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation_day_of_year: Option<u16>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation_year: Option<u16>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header_size: Option<u16>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset_to_point_data: Option<u32>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_of_vlrs: Option<u32>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub point_data_format_id: Option<u8>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub point_data_record_length: Option<u16>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_of_point_records: Option<u32>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub points_by_return: Option<[u32; 5]>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x_scale: Option<f64>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y_scale: Option<f64>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub z_scale: Option<f64>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x_offset: Option<f64>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y_offset: Option<f64>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub z_offset: Option<f64>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_x: Option<f64>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_x: Option<f64>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_y: Option<f64>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_y: Option<f64>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_z: Option<f64>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_z: Option<f64>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vlrs: Option<LasVlrsDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
    async fn apply(&self, base: &LasSnapshot) -> MutationApplyResult<LasSnapshot> {
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
    async fn absorb(&mut self, other: Self) {
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
    async fn inverse(&self, base: &LasSnapshot) -> Self {
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
        Self::between(&mutated, base).await
    }

    /// 🧭️ State delta (compose `GetXDiff`): header scalars compared field-by-field; `vlrs`/
    /// `points` index-keyed matching (pairwise `0..min(len)` = modified, base tail = removed,
    /// other tail = added — the recipe's "index keys pairwise by position" rule).
    async fn between(base: &LasSnapshot, other: &LasSnapshot) -> Self {
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

    async fn is_empty(&self) -> bool {
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_scale_and_offset(scale: (f64, f64, f64), offset: (f64, f64, f64)) -> LasDiff {
    LasDiff { x_scale: Some(scale.0), y_scale: Some(scale.1), z_scale: Some(scale.2), x_offset: Some(offset.0), y_offset: Some(offset.1), z_offset: Some(offset.2), ..Default::default() }
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
    if index >= base.vlrs.len() {
        return LasDiff::default();
    }
    LasDiff { number_of_vlrs: Some((base.vlrs.len() - 1) as u32), vlrs: Some(LasVlrsDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }
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
    if index >= base.points.len() {
        return LasDiff::default();
    }
    LasDiff { number_of_point_records: Some((base.points.len() - 1) as u32), points: Some(LasPointsDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_point(base: &LasSnapshot, index: usize, point: LasPoint) -> LasDiff {
    match base.points.get(index) {
        Some(existing) => {
            let d = point_between(existing, &point);
            if d == LasPointDiff::default() {
                LasDiff::default()
            } else {
                LasDiff { points: Some(LasPointsDiff { removed: vec![], modified: vec![LasPointModified { index, diff: d }], added: vec![] }), ..Default::default() }
            }
        }
        None => LasDiff::default(),
    }
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
///    `SetBounds` carry bare `(f64, f64, f64)` fields — confirmed via a SEPARATE real `#[derive(dsl::DslOps)]`
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
pub(crate) fn parse_u8(s: &str) -> Result<u8, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_i8(s: &str) -> Result<i8, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_u16(s: &str) -> Result<u16, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_u32(s: &str) -> Result<u32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}

/// 🧭️ Bracket-depth-aware split (tracks `[`/`]` only): a top-level `sep` inside nested brackets is
/// never mistaken for a field separator — the whole hand-rolled grammar's parsing primitive
/// (identical to gif 89a's copy — own copy per artifact, no cross-artifact type sharing).
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
//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_rgb(t: &(u16, u16, u16)) -> String {
    format!("[{},{},{}]", t.0, t.1, t.2)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_rgb(s: &str) -> Result<(u16, u16, u16), String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [r, g, b] = parts.as_slice() else { return Err(format!("rgb: expected 3 fields, got {}", parts.len())) };
    Ok((parse_u16(r)?, parse_u16(g)?, parse_u16(b)?))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_u32x5(a: &[u32; 5]) -> String {
    format!("[{},{},{},{},{}]", a[0], a[1], a[2], a[3], a[4])
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_u32x5(s: &str) -> Result<[u32; 5], String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let vals: Vec<u32> = parts.iter().map(|p| parse_u32(p)).collect::<Result<_, String>>()?;
    vals.try_into().map_err(|v: Vec<u32>| format!("points-by-return: expected 5 values, got {}", v.len()))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_vlr(v: &LasVlr) -> String {
    format!("[{},{},{},{}]", hex_encode(v.user_id.as_bytes()), v.record_id, hex_encode(v.description.as_bytes()), hex_encode(&v.data))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_vlr(s: &str) -> Result<LasVlr, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [user_id, record_id, description, data] = parts.as_slice() else {
        return Err(format!("vlr: expected 4 fields, got {}", parts.len()));
    };
    Ok(LasVlr { user_id: String::from_utf8(hex_decode(user_id)?).map_err(|e| e.to_string())?, record_id: parse_u16(record_id)?, description: String::from_utf8(hex_decode(description)?).map_err(|e| e.to_string())?, data: hex_decode(data)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_point(p: &LasPoint) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{},{},{},{},{},{}]",
        p.x,
        p.y,
        p.z,
        p.intensity,
        p.return_number,
        p.number_of_returns,
        if p.scan_direction_flag { 1 } else { 0 },
        if p.edge_of_flight_line { 1 } else { 0 },
        p.classification,
        p.scan_angle_rank,
        p.user_data,
        p.point_source_id,
        encode_option(&p.gps_time, |v| v.to_string()),
        encode_option(&p.rgb, enc_rgb),
    )
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_point(s: &str) -> Result<LasPoint, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z, intensity, return_number, number_of_returns, scan_direction_flag, edge_of_flight_line, classification, scan_angle_rank, user_data, point_source_id, gps_time, rgb] = parts.as_slice() else {
        return Err(format!("point: expected 14 fields, got {}", parts.len()));
    };
    Ok(LasPoint {
        x: parse_f64(x)?,
        y: parse_f64(y)?,
        z: parse_f64(z)?,
        intensity: parse_u16(intensity)?,
        return_number: parse_u8(return_number)?,
        number_of_returns: parse_u8(number_of_returns)?,
        scan_direction_flag: *scan_direction_flag == "1",
        edge_of_flight_line: *edge_of_flight_line == "1",
        classification: parse_u8(classification)?,
        scan_angle_rank: parse_i8(scan_angle_rank)?,
        user_data: parse_u8(user_data)?,
        point_source_id: parse_u16(point_source_id)?,
        gps_time: decode_option(gps_time, parse_f64)?,
        rgb: decode_option(rgb, dec_rgb)?,
    })
}
//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_vlr_diff(d: &LasVlrDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = &d.user_id {
        parts.push(format!("U:{}", hex_encode(v.as_bytes())));
    }
    if let Some(v) = d.record_id {
        parts.push(format!("R:{v}"));
    }
    if let Some(v) = &d.description {
        parts.push(format!("N:{}", hex_encode(v.as_bytes())));
    }
    if let Some(v) = &d.data {
        parts.push(format!("X:{}", hex_encode(v)));
    }
    format!("[{}]", parts.join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_vlr_diff(s: &str) -> Result<LasVlrDiff, String> {
    let inner = strip_brackets(s)?;
    let mut d = LasVlrDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("vlr diff: bad entry {entry:?}"))?;
        match tag {
            "U" => d.user_id = Some(String::from_utf8(hex_decode(val)?).map_err(|e| e.to_string())?),
            "R" => d.record_id = Some(parse_u16(val)?),
            "N" => d.description = Some(String::from_utf8(hex_decode(val)?).map_err(|e| e.to_string())?),
            "X" => d.data = Some(hex_decode(val)?),
            other => return Err(format!("vlr diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_point_diff(d: &LasPointDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = d.x {
        parts.push(format!("X:{v}"));
    }
    if let Some(v) = d.y {
        parts.push(format!("Y:{v}"));
    }
    if let Some(v) = d.z {
        parts.push(format!("Z:{v}"));
    }
    if let Some(v) = d.intensity {
        parts.push(format!("I:{v}"));
    }
    if let Some(v) = d.return_number {
        parts.push(format!("R:{v}"));
    }
    if let Some(v) = d.number_of_returns {
        parts.push(format!("N:{v}"));
    }
    if let Some(v) = d.scan_direction_flag {
        parts.push(format!("D:{}", if v { 1 } else { 0 }));
    }
    if let Some(v) = d.edge_of_flight_line {
        parts.push(format!("E:{}", if v { 1 } else { 0 }));
    }
    if let Some(v) = d.classification {
        parts.push(format!("C:{v}"));
    }
    if let Some(v) = d.scan_angle_rank {
        parts.push(format!("A:{v}"));
    }
    if let Some(v) = d.user_data {
        parts.push(format!("U:{v}"));
    }
    if let Some(v) = d.point_source_id {
        parts.push(format!("P:{v}"));
    }
    if let Some(v) = d.gps_time {
        parts.push(format!("G:{}", encode_option(&v, |x| x.to_string())));
    }
    if let Some(v) = d.rgb {
        parts.push(format!("B:{}", encode_option(&v, enc_rgb)));
    }
    format!("[{}]", parts.join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_point_diff(s: &str) -> Result<LasPointDiff, String> {
    let inner = strip_brackets(s)?;
    let mut d = LasPointDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("point diff: bad entry {entry:?}"))?;
        match tag {
            "X" => d.x = Some(parse_f64(val)?),
            "Y" => d.y = Some(parse_f64(val)?),
            "Z" => d.z = Some(parse_f64(val)?),
            "I" => d.intensity = Some(parse_u16(val)?),
            "R" => d.return_number = Some(parse_u8(val)?),
            "N" => d.number_of_returns = Some(parse_u8(val)?),
            "D" => d.scan_direction_flag = Some(val == "1"),
            "E" => d.edge_of_flight_line = Some(val == "1"),
            "C" => d.classification = Some(parse_u8(val)?),
            "A" => d.scan_angle_rank = Some(parse_i8(val)?),
            "U" => d.user_data = Some(parse_u8(val)?),
            "P" => d.point_source_id = Some(parse_u16(val)?),
            "G" => d.gps_time = Some(decode_option(val, parse_f64)?),
            "B" => d.rgb = Some(decode_option(val, dec_rgb)?),
            other => return Err(format!("point diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}

/// 🧭️ Generic-shaped 3-section `[removed];[modified];[added]` collection-triple printer/parser
/// (identical shape to gif 89a's copy — own copy per artifact, no cross-artifact type sharing).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_collection_triple(name: &str, removed: &[usize], modified: &[(usize, String)], added: &[(usize, String)]) -> String {
    let removed = removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = modified.iter().map(|(i, v)| format!("{i}:{v}")).collect::<Vec<_>>().join(",");
    let added = added.iter().map(|(i, v)| format!("{i}:{v}")).collect::<Vec<_>>().join(",");
    format!("{name}{{[{removed}];[{modified}];[{added}]}}")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_collection_triple(body: &str) -> Result<(Vec<usize>, Vec<(usize, String)>, Vec<(usize, String)>), String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("collection: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let parse_entries = |s: &str| -> Result<Vec<(usize, String)>, String> {
        split_top_level(strip_brackets(s)?, ',')
            .into_iter()
            .filter(|s| !s.is_empty())
            .map(|entry| {
                let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("collection entry: bad entry {entry:?}"))?;
                Ok((parse_usize(idx)?, rest.to_string()))
            })
            .collect()
    };
    Ok((removed, parse_entries(modified_s)?, parse_entries(added_s)?))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_vlrs_diff(d: &LasVlrsDiff) -> String {
    enc_collection_triple("vlrs", &d.removed, &d.modified.iter().map(|m| (m.index, enc_vlr_diff(&m.diff))).collect::<Vec<_>>(), &d.added.iter().map(|a| (a.index, enc_vlr(&a.vlr))).collect::<Vec<_>>())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_vlrs_diff(body: &str) -> Result<LasVlrsDiff, String> {
    let (removed, modified, added) = dec_collection_triple(body)?;
    Ok(LasVlrsDiff {
        removed,
        modified: modified.into_iter().map(|(index, enc)| Ok(LasVlrModified { index, diff: dec_vlr_diff(&enc)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, enc)| Ok(LasVlrAdded { index, vlr: dec_vlr(&enc)? })).collect::<Result<Vec<_>, String>>()?,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_points_diff(d: &LasPointsDiff) -> String {
    enc_collection_triple("points", &d.removed, &d.modified.iter().map(|m| (m.index, enc_point_diff(&m.diff))).collect::<Vec<_>>(), &d.added.iter().map(|a| (a.index, enc_point(&a.point))).collect::<Vec<_>>())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_points_diff(body: &str) -> Result<LasPointsDiff, String> {
    let (removed, modified, added) = dec_collection_triple(body)?;
    Ok(LasPointsDiff {
        removed,
        modified: modified.into_iter().map(|(index, enc)| Ok(LasPointModified { index, diff: dec_point_diff(&enc)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, enc)| Ok(LasPointAdded { index, point: dec_point(&enc)? })).collect::<Result<Vec<_>, String>>()?,
    })
}
//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_las_diff(d: &LasDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = d.version_major {
        tokens.push(format!("version-major={v}"));
    }
    if let Some(v) = d.version_minor {
        tokens.push(format!("version-minor={v}"));
    }
    if let Some(v) = &d.system_identifier {
        tokens.push(format!("system-identifier={}", hex_encode(v.as_bytes())));
    }
    if let Some(v) = &d.generating_software {
        tokens.push(format!("generating-software={}", hex_encode(v.as_bytes())));
    }
    if let Some(v) = d.creation_day_of_year {
        tokens.push(format!("creation-day-of-year={v}"));
    }
    if let Some(v) = d.creation_year {
        tokens.push(format!("creation-year={v}"));
    }
    if let Some(v) = d.header_size {
        tokens.push(format!("header-size={v}"));
    }
    if let Some(v) = d.offset_to_point_data {
        tokens.push(format!("offset-to-point-data={v}"));
    }
    if let Some(v) = d.number_of_vlrs {
        tokens.push(format!("number-of-vlrs={v}"));
    }
    if let Some(v) = d.point_data_format_id {
        tokens.push(format!("point-data-format-id={v}"));
    }
    if let Some(v) = d.point_data_record_length {
        tokens.push(format!("point-data-record-length={v}"));
    }
    if let Some(v) = d.number_of_point_records {
        tokens.push(format!("number-of-point-records={v}"));
    }
    if let Some(v) = d.points_by_return {
        tokens.push(format!("points-by-return={}", enc_u32x5(&v)));
    }
    if let Some(v) = d.x_scale {
        tokens.push(format!("x-scale={v}"));
    }
    if let Some(v) = d.y_scale {
        tokens.push(format!("y-scale={v}"));
    }
    if let Some(v) = d.z_scale {
        tokens.push(format!("z-scale={v}"));
    }
    if let Some(v) = d.x_offset {
        tokens.push(format!("x-offset={v}"));
    }
    if let Some(v) = d.y_offset {
        tokens.push(format!("y-offset={v}"));
    }
    if let Some(v) = d.z_offset {
        tokens.push(format!("z-offset={v}"));
    }
    if let Some(v) = d.max_x {
        tokens.push(format!("max-x={v}"));
    }
    if let Some(v) = d.min_x {
        tokens.push(format!("min-x={v}"));
    }
    if let Some(v) = d.max_y {
        tokens.push(format!("max-y={v}"));
    }
    if let Some(v) = d.min_y {
        tokens.push(format!("min-y={v}"));
    }
    if let Some(v) = d.max_z {
        tokens.push(format!("max-z={v}"));
    }
    if let Some(v) = d.min_z {
        tokens.push(format!("min-z={v}"));
    }
    if let Some(v) = &d.vlrs {
        tokens.push(enc_vlrs_diff(v));
    }
    if let Some(v) = &d.points {
        tokens.push(enc_points_diff(v));
    }
    tokens.join(" ")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_las_diff(line: &str) -> Result<LasDiff, String> {
    let mut d = LasDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("version-major=") {
            d.version_major = Some(parse_u8(rest)?);
        } else if let Some(rest) = token.strip_prefix("version-minor=") {
            d.version_minor = Some(parse_u8(rest)?);
        } else if let Some(rest) = token.strip_prefix("system-identifier=") {
            d.system_identifier = Some(String::from_utf8(hex_decode(rest)?).map_err(|e| e.to_string())?);
        } else if let Some(rest) = token.strip_prefix("generating-software=") {
            d.generating_software = Some(String::from_utf8(hex_decode(rest)?).map_err(|e| e.to_string())?);
        } else if let Some(rest) = token.strip_prefix("creation-day-of-year=") {
            d.creation_day_of_year = Some(parse_u16(rest)?);
        } else if let Some(rest) = token.strip_prefix("creation-year=") {
            d.creation_year = Some(parse_u16(rest)?);
        } else if let Some(rest) = token.strip_prefix("header-size=") {
            d.header_size = Some(parse_u16(rest)?);
        } else if let Some(rest) = token.strip_prefix("offset-to-point-data=") {
            d.offset_to_point_data = Some(parse_u32(rest)?);
        } else if let Some(rest) = token.strip_prefix("number-of-vlrs=") {
            d.number_of_vlrs = Some(parse_u32(rest)?);
        } else if let Some(rest) = token.strip_prefix("point-data-format-id=") {
            d.point_data_format_id = Some(parse_u8(rest)?);
        } else if let Some(rest) = token.strip_prefix("point-data-record-length=") {
            d.point_data_record_length = Some(parse_u16(rest)?);
        } else if let Some(rest) = token.strip_prefix("number-of-point-records=") {
            d.number_of_point_records = Some(parse_u32(rest)?);
        } else if let Some(rest) = token.strip_prefix("points-by-return=") {
            d.points_by_return = Some(dec_u32x5(rest)?);
        } else if let Some(rest) = token.strip_prefix("x-scale=") {
            d.x_scale = Some(parse_f64(rest)?);
        } else if let Some(rest) = token.strip_prefix("y-scale=") {
            d.y_scale = Some(parse_f64(rest)?);
        } else if let Some(rest) = token.strip_prefix("z-scale=") {
            d.z_scale = Some(parse_f64(rest)?);
        } else if let Some(rest) = token.strip_prefix("x-offset=") {
            d.x_offset = Some(parse_f64(rest)?);
        } else if let Some(rest) = token.strip_prefix("y-offset=") {
            d.y_offset = Some(parse_f64(rest)?);
        } else if let Some(rest) = token.strip_prefix("z-offset=") {
            d.z_offset = Some(parse_f64(rest)?);
        } else if let Some(rest) = token.strip_prefix("max-x=") {
            d.max_x = Some(parse_f64(rest)?);
        } else if let Some(rest) = token.strip_prefix("min-x=") {
            d.min_x = Some(parse_f64(rest)?);
        } else if let Some(rest) = token.strip_prefix("max-y=") {
            d.max_y = Some(parse_f64(rest)?);
        } else if let Some(rest) = token.strip_prefix("min-y=") {
            d.min_y = Some(parse_f64(rest)?);
        } else if let Some(rest) = token.strip_prefix("max-z=") {
            d.max_z = Some(parse_f64(rest)?);
        } else if let Some(rest) = token.strip_prefix("min-z=") {
            d.min_z = Some(parse_f64(rest)?);
        } else if let Some(rest) = token.strip_prefix("vlrs{") {
            d.vlrs = Some(dec_vlrs_diff(rest.strip_suffix('}').ok_or_else(|| "vlrs: missing closing brace".to_string())?)?);
        } else if let Some(rest) = token.strip_prefix("points{") {
            d.points = Some(dec_points_diff(rest.strip_suffix('}').ok_or_else(|| "points: missing closing brace".to_string())?)?);
        } else {
            return Err(format!("las diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

//#region 🔖️BinaryDiffCodec
/// 🧪️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: REAL
/// LEB128-varint-framed binary twins of the hex-text codecs above, backing the upgraded
/// `DiffCodec::encode_diff`/`decode_diff` below (`#region 🔖️TopLevel`) — replaces the old F6
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().await.map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).await.map_err(|e| e.to_string())?.to_vec())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️RecordBinaryCodec
/// 🧭️ Full (non-sparse) `LasHeader` binary record — every field always present, declaration
/// order — reused by `🧬️mutations/🦀️component.rs`'s `SetSnapshot` binary payload (a whole
/// `LasSnapshot` embeds a whole `LasHeader`, never a sparse patch).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_header_bin(h: &LasHeader, out: &mut Vec<u8>) {
    out.push(h.version_major);
    out.push(h.version_minor);
    write_str_lp(out, &h.system_identifier);
    write_str_lp(out, &h.generating_software);
    store::pack_rt::write_varint_u64(out, h.creation_day_of_year as u64);
    store::pack_rt::write_varint_u64(out, h.creation_year as u64);
    store::pack_rt::write_varint_u64(out, h.header_size as u64);
    store::pack_rt::write_varint_u64(out, h.offset_to_point_data as u64);
    store::pack_rt::write_varint_u64(out, h.number_of_vlrs as u64);
    out.push(h.point_data_format_id);
    store::pack_rt::write_varint_u64(out, h.point_data_record_length as u64);
    store::pack_rt::write_varint_u64(out, h.number_of_point_records as u64);
    for v in h.points_by_return {
        store::pack_rt::write_varint_u64(out, v as u64);
    }
    for v in [h.x_scale, h.y_scale, h.z_scale, h.x_offset, h.y_offset, h.z_offset, h.max_x, h.min_x, h.max_y, h.min_y, h.max_z, h.min_z] {
        out.extend_from_slice(&v.to_le_bytes());
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_header_bin(reader: &mut store::ByteReader<'_>) -> Result<LasHeader, String> {
    let version_major = reader.read_u8().await.map_err(|e| e.to_string())?;
    let version_minor = reader.read_u8().await.map_err(|e| e.to_string())?;
    let system_identifier = read_str_lp(reader)?;
    let generating_software = read_str_lp(reader)?;
    let creation_day_of_year = reader.read_varint_u64().await.map_err(|e| e.to_string())? as u16;
    let creation_year = reader.read_varint_u64().await.map_err(|e| e.to_string())? as u16;
    let header_size = reader.read_varint_u64().await.map_err(|e| e.to_string())? as u16;
    let offset_to_point_data = reader.read_varint_u64().await.map_err(|e| e.to_string())? as u32;
    let number_of_vlrs = reader.read_varint_u64().await.map_err(|e| e.to_string())? as u32;
    let point_data_format_id = reader.read_u8().await.map_err(|e| e.to_string())?;
    let point_data_record_length = reader.read_varint_u64().await.map_err(|e| e.to_string())? as u16;
    let number_of_point_records = reader.read_varint_u64().await.map_err(|e| e.to_string())? as u32;
    let mut points_by_return = [0u32; 5];
    for slot in points_by_return.iter_mut() {
        *slot = reader.read_varint_u64().await.map_err(|e| e.to_string())? as u32;
    }
    let mut floats = [0.0f64; 12];
    for slot in floats.iter_mut() {
        *slot = reader.read_f64_le().await.map_err(|e| e.to_string())?;
    }
    let [x_scale, y_scale, z_scale, x_offset, y_offset, z_offset, max_x, min_x, max_y, min_y, max_z, min_z] = floats;
    Ok(LasHeader {
        version_major,
        version_minor,
        system_identifier,
        generating_software,
        creation_day_of_year,
        creation_year,
        header_size,
        offset_to_point_data,
        number_of_vlrs,
        point_data_format_id,
        point_data_record_length,
        number_of_point_records,
        points_by_return,
        x_scale,
        y_scale,
        z_scale,
        x_offset,
        y_offset,
        z_offset,
        max_x,
        min_x,
        max_y,
        min_y,
        max_z,
        min_z,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_vlr_bin(v: &LasVlr, out: &mut Vec<u8>) {
    write_str_lp(out, &v.user_id);
    store::pack_rt::write_varint_u64(out, v.record_id as u64);
    write_str_lp(out, &v.description);
    write_bytes_lp(out, &v.data);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_vlr_bin(reader: &mut store::ByteReader<'_>) -> Result<LasVlr, String> {
    Ok(LasVlr { user_id: read_str_lp(reader)?, record_id: reader.read_varint_u64().await.map_err(|e| e.to_string())? as u16, description: read_str_lp(reader)?, data: read_bytes_lp(reader)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_point_bin(p: &LasPoint, out: &mut Vec<u8>) {
    out.extend_from_slice(&p.x.to_le_bytes());
    out.extend_from_slice(&p.y.to_le_bytes());
    out.extend_from_slice(&p.z.to_le_bytes());
    store::pack_rt::write_varint_u64(out, p.intensity as u64);
    out.push(p.return_number);
    out.push(p.number_of_returns);
    out.push(p.scan_direction_flag as u8);
    out.push(p.edge_of_flight_line as u8);
    out.push(p.classification);
    out.push(p.scan_angle_rank as u8);
    out.push(p.user_data);
    store::pack_rt::write_varint_u64(out, p.point_source_id as u64);
    match p.gps_time {
        None => out.push(0),
        Some(v) => {
            out.push(1);
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    match p.rgb {
        None => out.push(0),
        Some((r, g, b)) => {
            out.push(1);
            store::pack_rt::write_varint_u64(out, r as u64);
            store::pack_rt::write_varint_u64(out, g as u64);
            store::pack_rt::write_varint_u64(out, b as u64);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_point_bin(reader: &mut store::ByteReader<'_>) -> Result<LasPoint, String> {
    let x = reader.read_f64_le().await.map_err(|e| e.to_string())?;
    let y = reader.read_f64_le().await.map_err(|e| e.to_string())?;
    let z = reader.read_f64_le().await.map_err(|e| e.to_string())?;
    let intensity = reader.read_varint_u64().await.map_err(|e| e.to_string())? as u16;
    let return_number = reader.read_u8().await.map_err(|e| e.to_string())?;
    let number_of_returns = reader.read_u8().await.map_err(|e| e.to_string())?;
    let scan_direction_flag = reader.read_u8().await.map_err(|e| e.to_string())? != 0;
    let edge_of_flight_line = reader.read_u8().await.map_err(|e| e.to_string())? != 0;
    let classification = reader.read_u8().await.map_err(|e| e.to_string())?;
    let scan_angle_rank = reader.read_u8().await.map_err(|e| e.to_string())? as i8;
    let user_data = reader.read_u8().await.map_err(|e| e.to_string())?;
    let point_source_id = reader.read_varint_u64().await.map_err(|e| e.to_string())? as u16;
    let gps_time = match reader.read_u8().await.map_err(|e| e.to_string())? {
        0 => None,
        1 => Some(reader.read_f64_le().await.map_err(|e| e.to_string())?),
        other => return Err(format!("bad option tag {other}")),
    };
    let rgb = match reader.read_u8().await.map_err(|e| e.to_string())? {
        0 => None,
        1 => Some((reader.read_varint_u64().await.map_err(|e| e.to_string())? as u16, reader.read_varint_u64().await.map_err(|e| e.to_string())? as u16, reader.read_varint_u64().await.map_err(|e| e.to_string())? as u16)),
        other => return Err(format!("bad option tag {other}")),
    };
    Ok(LasPoint { x, y, z, intensity, return_number, number_of_returns, scan_direction_flag, edge_of_flight_line, classification, scan_angle_rank, user_data, point_source_id, gps_time, rgb })
}
//#endregion 🔖️RecordBinaryCodec

//#region 🔖️SparseDiffBinaryCodec
const VDF_USER_ID: u8 = 1 << 0;
const VDF_RECORD_ID: u8 = 1 << 1;
const VDF_DESCRIPTION: u8 = 1 << 2;
const VDF_DATA: u8 = 1 << 3;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_vlr_diff_bin(d: &LasVlrDiff, out: &mut Vec<u8>) {
    let mut mask = 0u8;
    if d.user_id.is_some() {
        mask |= VDF_USER_ID;
    }
    if d.record_id.is_some() {
        mask |= VDF_RECORD_ID;
    }
    if d.description.is_some() {
        mask |= VDF_DESCRIPTION;
    }
    if d.data.is_some() {
        mask |= VDF_DATA;
    }
    out.push(mask);
    if let Some(v) = &d.user_id {
        write_str_lp(out, v);
    }
    if let Some(v) = d.record_id {
        store::pack_rt::write_varint_u64(out, v as u64);
    }
    if let Some(v) = &d.description {
        write_str_lp(out, v);
    }
    if let Some(v) = &d.data {
        write_bytes_lp(out, v);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_vlr_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<LasVlrDiff, String> {
    let mask = reader.read_u8().await.map_err(|e| e.to_string())?;
    let mut d = LasVlrDiff::default();
    if mask & VDF_USER_ID != 0 {
        d.user_id = Some(read_str_lp(reader)?);
    }
    if mask & VDF_RECORD_ID != 0 {
        d.record_id = Some(reader.read_varint_u64().await.map_err(|e| e.to_string())? as u16);
    }
    if mask & VDF_DESCRIPTION != 0 {
        d.description = Some(read_str_lp(reader)?);
    }
    if mask & VDF_DATA != 0 {
        d.data = Some(read_bytes_lp(reader)?);
    }
    Ok(d)
}

const PDF_X: u16 = 1 << 0;
const PDF_Y: u16 = 1 << 1;
const PDF_Z: u16 = 1 << 2;
const PDF_INTENSITY: u16 = 1 << 3;
const PDF_RETURN_NUMBER: u16 = 1 << 4;
const PDF_NUMBER_OF_RETURNS: u16 = 1 << 5;
const PDF_SCAN_DIRECTION_FLAG: u16 = 1 << 6;
const PDF_EDGE_OF_FLIGHT_LINE: u16 = 1 << 7;
const PDF_CLASSIFICATION: u16 = 1 << 8;
const PDF_SCAN_ANGLE_RANK: u16 = 1 << 9;
const PDF_USER_DATA: u16 = 1 << 10;
const PDF_POINT_SOURCE_ID: u16 = 1 << 11;
const PDF_GPS_TIME: u16 = 1 << 12;
const PDF_RGB: u16 = 1 << 13;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_point_diff_bin(d: &LasPointDiff, out: &mut Vec<u8>) {
    let mut mask = 0u16;
    if d.x.is_some() {
        mask |= PDF_X;
    }
    if d.y.is_some() {
        mask |= PDF_Y;
    }
    if d.z.is_some() {
        mask |= PDF_Z;
    }
    if d.intensity.is_some() {
        mask |= PDF_INTENSITY;
    }
    if d.return_number.is_some() {
        mask |= PDF_RETURN_NUMBER;
    }
    if d.number_of_returns.is_some() {
        mask |= PDF_NUMBER_OF_RETURNS;
    }
    if d.scan_direction_flag.is_some() {
        mask |= PDF_SCAN_DIRECTION_FLAG;
    }
    if d.edge_of_flight_line.is_some() {
        mask |= PDF_EDGE_OF_FLIGHT_LINE;
    }
    if d.classification.is_some() {
        mask |= PDF_CLASSIFICATION;
    }
    if d.scan_angle_rank.is_some() {
        mask |= PDF_SCAN_ANGLE_RANK;
    }
    if d.user_data.is_some() {
        mask |= PDF_USER_DATA;
    }
    if d.point_source_id.is_some() {
        mask |= PDF_POINT_SOURCE_ID;
    }
    if d.gps_time.is_some() {
        mask |= PDF_GPS_TIME;
    }
    if d.rgb.is_some() {
        mask |= PDF_RGB;
    }
    out.extend_from_slice(&mask.to_le_bytes());
    if let Some(v) = d.x {
        out.extend_from_slice(&v.to_le_bytes());
    }
    if let Some(v) = d.y {
        out.extend_from_slice(&v.to_le_bytes());
    }
    if let Some(v) = d.z {
        out.extend_from_slice(&v.to_le_bytes());
    }
    if let Some(v) = d.intensity {
        store::pack_rt::write_varint_u64(out, v as u64);
    }
    if let Some(v) = d.return_number {
        out.push(v);
    }
    if let Some(v) = d.number_of_returns {
        out.push(v);
    }
    if let Some(v) = d.scan_direction_flag {
        out.push(v as u8);
    }
    if let Some(v) = d.edge_of_flight_line {
        out.push(v as u8);
    }
    if let Some(v) = d.classification {
        out.push(v);
    }
    if let Some(v) = d.scan_angle_rank {
        out.push(v as u8);
    }
    if let Some(v) = d.user_data {
        out.push(v);
    }
    if let Some(v) = d.point_source_id {
        store::pack_rt::write_varint_u64(out, v as u64);
    }
    if let Some(v) = d.gps_time {
        match v {
            None => out.push(0),
            Some(x) => {
                out.push(1);
                out.extend_from_slice(&x.to_le_bytes());
            }
        }
    }
    if let Some(v) = d.rgb {
        match v {
            None => out.push(0),
            Some((r, g, b)) => {
                out.push(1);
                store::pack_rt::write_varint_u64(out, r as u64);
                store::pack_rt::write_varint_u64(out, g as u64);
                store::pack_rt::write_varint_u64(out, b as u64);
            }
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_point_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<LasPointDiff, String> {
    let mask = reader.read_u16_le().await.map_err(|e| e.to_string())?;
    let mut d = LasPointDiff::default();
    if mask & PDF_X != 0 {
        d.x = Some(reader.read_f64_le().await.map_err(|e| e.to_string())?);
    }
    if mask & PDF_Y != 0 {
        d.y = Some(reader.read_f64_le().await.map_err(|e| e.to_string())?);
    }
    if mask & PDF_Z != 0 {
        d.z = Some(reader.read_f64_le().await.map_err(|e| e.to_string())?);
    }
    if mask & PDF_INTENSITY != 0 {
        d.intensity = Some(reader.read_varint_u64().await.map_err(|e| e.to_string())? as u16);
    }
    if mask & PDF_RETURN_NUMBER != 0 {
        d.return_number = Some(reader.read_u8().await.map_err(|e| e.to_string())?);
    }
    if mask & PDF_NUMBER_OF_RETURNS != 0 {
        d.number_of_returns = Some(reader.read_u8().await.map_err(|e| e.to_string())?);
    }
    if mask & PDF_SCAN_DIRECTION_FLAG != 0 {
        d.scan_direction_flag = Some(reader.read_u8().await.map_err(|e| e.to_string())? != 0);
    }
    if mask & PDF_EDGE_OF_FLIGHT_LINE != 0 {
        d.edge_of_flight_line = Some(reader.read_u8().await.map_err(|e| e.to_string())? != 0);
    }
    if mask & PDF_CLASSIFICATION != 0 {
        d.classification = Some(reader.read_u8().await.map_err(|e| e.to_string())?);
    }
    if mask & PDF_SCAN_ANGLE_RANK != 0 {
        d.scan_angle_rank = Some(reader.read_u8().await.map_err(|e| e.to_string())? as i8);
    }
    if mask & PDF_USER_DATA != 0 {
        d.user_data = Some(reader.read_u8().await.map_err(|e| e.to_string())?);
    }
    if mask & PDF_POINT_SOURCE_ID != 0 {
        d.point_source_id = Some(reader.read_varint_u64().await.map_err(|e| e.to_string())? as u16);
    }
    if mask & PDF_GPS_TIME != 0 {
        d.gps_time = Some(match reader.read_u8().await.map_err(|e| e.to_string())? {
            0 => None,
            1 => Some(reader.read_f64_le().await.map_err(|e| e.to_string())?),
            other => return Err(format!("bad option tag {other}")),
        });
    }
    if mask & PDF_RGB != 0 {
        d.rgb = Some(match reader.read_u8().await.map_err(|e| e.to_string())? {
            0 => None,
            1 => Some((reader.read_varint_u64().await.map_err(|e| e.to_string())? as u16, reader.read_varint_u64().await.map_err(|e| e.to_string())? as u16, reader.read_varint_u64().await.map_err(|e| e.to_string())? as u16)),
            other => return Err(format!("bad option tag {other}")),
        });
    }
    Ok(d)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_vlrs_diff_bin(d: &LasVlrsDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    for i in &d.removed {
        store::pack_rt::write_varint_u64(out, *i as u64);
    }
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for m in &d.modified {
        store::pack_rt::write_varint_u64(out, m.index as u64);
        enc_vlr_diff_bin(&m.diff, out);
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for a in &d.added {
        store::pack_rt::write_varint_u64(out, a.index as u64);
        enc_vlr_bin(&a.vlr, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_vlrs_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<LasVlrsDiff, String> {
    let removed_count = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
    let removed = (0..removed_count).map(|_| Ok(semio_framework_plugin::resolve_ready(reader.read_varint_u64()).map_err(|e| e.to_string())? as usize)).collect::<Result<Vec<_>, String>>()?;
    let modified_count = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
    let modified = (0..modified_count)
        .map(|_| -> Result<LasVlrModified, String> {
            let index = semio_framework_plugin::resolve_ready(reader.read_varint_u64()).map_err(|e| e.to_string())? as usize;
            let diff = dec_vlr_diff_bin(reader)?;
            Ok(LasVlrModified { index, diff })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added_count = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
    let added = (0..added_count)
        .map(|_| -> Result<LasVlrAdded, String> {
            let index = semio_framework_plugin::resolve_ready(reader.read_varint_u64()).map_err(|e| e.to_string())? as usize;
            let vlr = dec_vlr_bin(reader)?;
            Ok(LasVlrAdded { index, vlr })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(LasVlrsDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_points_diff_bin(d: &LasPointsDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    for i in &d.removed {
        store::pack_rt::write_varint_u64(out, *i as u64);
    }
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for m in &d.modified {
        store::pack_rt::write_varint_u64(out, m.index as u64);
        enc_point_diff_bin(&m.diff, out);
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for a in &d.added {
        store::pack_rt::write_varint_u64(out, a.index as u64);
        enc_point_bin(&a.point, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_points_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<LasPointsDiff, String> {
    let removed_count = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
    let removed = (0..removed_count).map(|_| Ok(semio_framework_plugin::resolve_ready(reader.read_varint_u64()).map_err(|e| e.to_string())? as usize)).collect::<Result<Vec<_>, String>>()?;
    let modified_count = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
    let modified = (0..modified_count)
        .map(|_| -> Result<LasPointModified, String> {
            let index = semio_framework_plugin::resolve_ready(reader.read_varint_u64()).map_err(|e| e.to_string())? as usize;
            let diff = dec_point_diff_bin(reader)?;
            Ok(LasPointModified { index, diff })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added_count = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
    let added = (0..added_count)
        .map(|_| -> Result<LasPointAdded, String> {
            let index = semio_framework_plugin::resolve_ready(reader.read_varint_u64()).map_err(|e| e.to_string())? as usize;
            let point = dec_point_bin(reader)?;
            Ok(LasPointAdded { index, point })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(LasPointsDiff { removed, modified, added })
}
//#endregion 🔖️SparseDiffBinaryCodec

//#region 🔖️HeaderMaskBits
const HDR_VERSION_MAJOR: u32 = 1 << 0;
const HDR_VERSION_MINOR: u32 = 1 << 1;
const HDR_SYSTEM_IDENTIFIER: u32 = 1 << 2;
const HDR_GENERATING_SOFTWARE: u32 = 1 << 3;
const HDR_CREATION_DAY_OF_YEAR: u32 = 1 << 4;
const HDR_CREATION_YEAR: u32 = 1 << 5;
const HDR_HEADER_SIZE: u32 = 1 << 6;
const HDR_OFFSET_TO_POINT_DATA: u32 = 1 << 7;
const HDR_NUMBER_OF_VLRS: u32 = 1 << 8;
const HDR_POINT_DATA_FORMAT_ID: u32 = 1 << 9;
const HDR_POINT_DATA_RECORD_LENGTH: u32 = 1 << 10;
const HDR_NUMBER_OF_POINT_RECORDS: u32 = 1 << 11;
const HDR_POINTS_BY_RETURN: u32 = 1 << 12;
const HDR_X_SCALE: u32 = 1 << 13;
const HDR_Y_SCALE: u32 = 1 << 14;
const HDR_Z_SCALE: u32 = 1 << 15;
const HDR_X_OFFSET: u32 = 1 << 16;
const HDR_Y_OFFSET: u32 = 1 << 17;
const HDR_Z_OFFSET: u32 = 1 << 18;
const HDR_MAX_X: u32 = 1 << 19;
const HDR_MIN_X: u32 = 1 << 20;
const HDR_MAX_Y: u32 = 1 << 21;
const HDR_MIN_Y: u32 = 1 << 22;
const HDR_MAX_Z: u32 = 1 << 23;
const HDR_MIN_Z: u32 = 1 << 24;
const HDR_VLRS: u32 = 1 << 25;
const HDR_POINTS: u32 = 1 << 26;
//#endregion 🔖️HeaderMaskBits
//#endregion 🔖️BinaryDiffCodec

impl DiffCodec for LasDiff {
    async fn print_diff(&self) -> String {
        print_las_diff(self)
    }
    async fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_las_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// ⚡️ REAL binary frame (`format u8 | header_mask u32 | <present header fields> | <vlrs diff
    /// if present> | <points diff if present>`), matching `../💾️binary/📡️component.protocol.
    /// semio`'s `format`/`header_mask` leading fields exactly — upgraded from F6's
    /// `print_diff().into_bytes()` text-as-binary shortcut (see `#region 🔖️BinaryDiffCodec`'s doc
    /// comment for the full rationale).
    async fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut mask = 0u32;
        if self.version_major.is_some() {
            mask |= HDR_VERSION_MAJOR;
        }
        if self.version_minor.is_some() {
            mask |= HDR_VERSION_MINOR;
        }
        if self.system_identifier.is_some() {
            mask |= HDR_SYSTEM_IDENTIFIER;
        }
        if self.generating_software.is_some() {
            mask |= HDR_GENERATING_SOFTWARE;
        }
        if self.creation_day_of_year.is_some() {
            mask |= HDR_CREATION_DAY_OF_YEAR;
        }
        if self.creation_year.is_some() {
            mask |= HDR_CREATION_YEAR;
        }
        if self.header_size.is_some() {
            mask |= HDR_HEADER_SIZE;
        }
        if self.offset_to_point_data.is_some() {
            mask |= HDR_OFFSET_TO_POINT_DATA;
        }
        if self.number_of_vlrs.is_some() {
            mask |= HDR_NUMBER_OF_VLRS;
        }
        if self.point_data_format_id.is_some() {
            mask |= HDR_POINT_DATA_FORMAT_ID;
        }
        if self.point_data_record_length.is_some() {
            mask |= HDR_POINT_DATA_RECORD_LENGTH;
        }
        if self.number_of_point_records.is_some() {
            mask |= HDR_NUMBER_OF_POINT_RECORDS;
        }
        if self.points_by_return.is_some() {
            mask |= HDR_POINTS_BY_RETURN;
        }
        if self.x_scale.is_some() {
            mask |= HDR_X_SCALE;
        }
        if self.y_scale.is_some() {
            mask |= HDR_Y_SCALE;
        }
        if self.z_scale.is_some() {
            mask |= HDR_Z_SCALE;
        }
        if self.x_offset.is_some() {
            mask |= HDR_X_OFFSET;
        }
        if self.y_offset.is_some() {
            mask |= HDR_Y_OFFSET;
        }
        if self.z_offset.is_some() {
            mask |= HDR_Z_OFFSET;
        }
        if self.max_x.is_some() {
            mask |= HDR_MAX_X;
        }
        if self.min_x.is_some() {
            mask |= HDR_MIN_X;
        }
        if self.max_y.is_some() {
            mask |= HDR_MAX_Y;
        }
        if self.min_y.is_some() {
            mask |= HDR_MIN_Y;
        }
        if self.max_z.is_some() {
            mask |= HDR_MAX_Z;
        }
        if self.min_z.is_some() {
            mask |= HDR_MIN_Z;
        }
        if self.vlrs.is_some() {
            mask |= HDR_VLRS;
        }
        if self.points.is_some() {
            mask |= HDR_POINTS;
        }

        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT];
        out.extend_from_slice(&mask.to_le_bytes());
        if let Some(v) = self.version_major {
            out.push(v);
        }
        if let Some(v) = self.version_minor {
            out.push(v);
        }
        if let Some(v) = &self.system_identifier {
            write_str_lp(&mut out, v);
        }
        if let Some(v) = &self.generating_software {
            write_str_lp(&mut out, v);
        }
        if let Some(v) = self.creation_day_of_year {
            store::pack_rt::write_varint_u64(&mut out, v as u64);
        }
        if let Some(v) = self.creation_year {
            store::pack_rt::write_varint_u64(&mut out, v as u64);
        }
        if let Some(v) = self.header_size {
            store::pack_rt::write_varint_u64(&mut out, v as u64);
        }
        if let Some(v) = self.offset_to_point_data {
            store::pack_rt::write_varint_u64(&mut out, v as u64);
        }
        if let Some(v) = self.number_of_vlrs {
            store::pack_rt::write_varint_u64(&mut out, v as u64);
        }
        if let Some(v) = self.point_data_format_id {
            out.push(v);
        }
        if let Some(v) = self.point_data_record_length {
            store::pack_rt::write_varint_u64(&mut out, v as u64);
        }
        if let Some(v) = self.number_of_point_records {
            store::pack_rt::write_varint_u64(&mut out, v as u64);
        }
        if let Some(v) = self.points_by_return {
            for x in v {
                store::pack_rt::write_varint_u64(&mut out, x as u64);
            }
        }
        if let Some(v) = self.x_scale {
            out.extend_from_slice(&v.to_le_bytes());
        }
        if let Some(v) = self.y_scale {
            out.extend_from_slice(&v.to_le_bytes());
        }
        if let Some(v) = self.z_scale {
            out.extend_from_slice(&v.to_le_bytes());
        }
        if let Some(v) = self.x_offset {
            out.extend_from_slice(&v.to_le_bytes());
        }
        if let Some(v) = self.y_offset {
            out.extend_from_slice(&v.to_le_bytes());
        }
        if let Some(v) = self.z_offset {
            out.extend_from_slice(&v.to_le_bytes());
        }
        if let Some(v) = self.max_x {
            out.extend_from_slice(&v.to_le_bytes());
        }
        if let Some(v) = self.min_x {
            out.extend_from_slice(&v.to_le_bytes());
        }
        if let Some(v) = self.max_y {
            out.extend_from_slice(&v.to_le_bytes());
        }
        if let Some(v) = self.min_y {
            out.extend_from_slice(&v.to_le_bytes());
        }
        if let Some(v) = self.max_z {
            out.extend_from_slice(&v.to_le_bytes());
        }
        if let Some(v) = self.min_z {
            out.extend_from_slice(&v.to_le_bytes());
        }
        if let Some(v) = &self.vlrs {
            enc_vlrs_diff_bin(v, &mut out);
        }
        if let Some(v) = &self.points {
            enc_points_diff_bin(v, &mut out);
        }
        Ok(out)
    }
    async fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        async fn go(bytes: &[u8]) -> Result<LasDiff, String> {
            let mut reader = store::ByteReader::new(bytes).await;
            let format = reader.read_u8().await.map_err(|e| e.to_string())?;
            if format != store::pack_rt::OP_BINARY_FORMAT {
                return Err(format!("bad diff format byte {format}"));
            }
            let mask = reader.read_u32_le().await.map_err(|e| e.to_string())?;
            let mut d = LasDiff::default();
            if mask & HDR_VERSION_MAJOR != 0 {
                d.version_major = Some(reader.read_u8().await.map_err(|e| e.to_string())?);
            }
            if mask & HDR_VERSION_MINOR != 0 {
                d.version_minor = Some(reader.read_u8().await.map_err(|e| e.to_string())?);
            }
            if mask & HDR_SYSTEM_IDENTIFIER != 0 {
                d.system_identifier = Some(read_str_lp(&mut reader)?);
            }
            if mask & HDR_GENERATING_SOFTWARE != 0 {
                d.generating_software = Some(read_str_lp(&mut reader)?);
            }
            if mask & HDR_CREATION_DAY_OF_YEAR != 0 {
                d.creation_day_of_year = Some(reader.read_varint_u64().await.map_err(|e| e.to_string())? as u16);
            }
            if mask & HDR_CREATION_YEAR != 0 {
                d.creation_year = Some(reader.read_varint_u64().await.map_err(|e| e.to_string())? as u16);
            }
            if mask & HDR_HEADER_SIZE != 0 {
                d.header_size = Some(reader.read_varint_u64().await.map_err(|e| e.to_string())? as u16);
            }
            if mask & HDR_OFFSET_TO_POINT_DATA != 0 {
                d.offset_to_point_data = Some(reader.read_varint_u64().await.map_err(|e| e.to_string())? as u32);
            }
            if mask & HDR_NUMBER_OF_VLRS != 0 {
                d.number_of_vlrs = Some(reader.read_varint_u64().await.map_err(|e| e.to_string())? as u32);
            }
            if mask & HDR_POINT_DATA_FORMAT_ID != 0 {
                d.point_data_format_id = Some(reader.read_u8().await.map_err(|e| e.to_string())?);
            }
            if mask & HDR_POINT_DATA_RECORD_LENGTH != 0 {
                d.point_data_record_length = Some(reader.read_varint_u64().await.map_err(|e| e.to_string())? as u16);
            }
            if mask & HDR_NUMBER_OF_POINT_RECORDS != 0 {
                d.number_of_point_records = Some(reader.read_varint_u64().await.map_err(|e| e.to_string())? as u32);
            }
            if mask & HDR_POINTS_BY_RETURN != 0 {
                let mut a = [0u32; 5];
                for slot in a.iter_mut() {
                    *slot = reader.read_varint_u64().await.map_err(|e| e.to_string())? as u32;
                }
                d.points_by_return = Some(a);
            }
            if mask & HDR_X_SCALE != 0 {
                d.x_scale = Some(reader.read_f64_le().await.map_err(|e| e.to_string())?);
            }
            if mask & HDR_Y_SCALE != 0 {
                d.y_scale = Some(reader.read_f64_le().await.map_err(|e| e.to_string())?);
            }
            if mask & HDR_Z_SCALE != 0 {
                d.z_scale = Some(reader.read_f64_le().await.map_err(|e| e.to_string())?);
            }
            if mask & HDR_X_OFFSET != 0 {
                d.x_offset = Some(reader.read_f64_le().await.map_err(|e| e.to_string())?);
            }
            if mask & HDR_Y_OFFSET != 0 {
                d.y_offset = Some(reader.read_f64_le().await.map_err(|e| e.to_string())?);
            }
            if mask & HDR_Z_OFFSET != 0 {
                d.z_offset = Some(reader.read_f64_le().await.map_err(|e| e.to_string())?);
            }
            if mask & HDR_MAX_X != 0 {
                d.max_x = Some(reader.read_f64_le().await.map_err(|e| e.to_string())?);
            }
            if mask & HDR_MIN_X != 0 {
                d.min_x = Some(reader.read_f64_le().await.map_err(|e| e.to_string())?);
            }
            if mask & HDR_MAX_Y != 0 {
                d.max_y = Some(reader.read_f64_le().await.map_err(|e| e.to_string())?);
            }
            if mask & HDR_MIN_Y != 0 {
                d.min_y = Some(reader.read_f64_le().await.map_err(|e| e.to_string())?);
            }
            if mask & HDR_MAX_Z != 0 {
                d.max_z = Some(reader.read_f64_le().await.map_err(|e| e.to_string())?);
            }
            if mask & HDR_MIN_Z != 0 {
                d.min_z = Some(reader.read_f64_le().await.map_err(|e| e.to_string())?);
            }
            if mask & HDR_VLRS != 0 {
                d.vlrs = Some(dec_vlrs_diff_bin(&mut reader)?);
            }
            if mask & HDR_POINTS != 0 {
                d.points = Some(dec_points_diff_bin(&mut reader)?);
            }
            Ok(d)
        }
        go(bytes).await.map_err(|e| protocol::ProtocolError::Malformed { what: "las diff binary", offset: 0, detail: e })
    }
}
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
/// `⚙️engine/🦀️component.rs`'s `diff_grammar_conformance_law`/`protocol_walk_law` conformance
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
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn invalid_collection_targets_are_rejected_before_mutation() {
        let base = LasSnapshot::default();
        let diff = LasDiff { vlrs: Some(LasVlrsDiff { removed: vec![0], ..Default::default() }), ..Default::default() };
        let error = diff.apply(&base).await.expect_err("missing VLR target must be rejected");
        assert_eq!(error.code, "invalid-remove-index");
        assert_eq!(error.target, vec!["vlrs", "0"]);
        assert_eq!(base, LasSnapshot::default());
    }

    /// 🧪️ `DiffCodec` round-trip law for the hand-rolled `LasDiff` text/binary grammar —
    /// exercises every header scalar, both `LasPointDiff` tri-states (`gps_time`/`rgb`, both
    /// `Some(None)` and `Some(Some(_))` transitions), and both collection triples (`vlrs`/`points`,
    /// `removed`/`modified`/`added`) simultaneously via `demo_diff_cases()`'s real `between()`
    /// results — the single source of truth also reused by `⚙️engine/🦀️component.rs`'s
    /// `diff_grammar_conformance_law`/`protocol_walk_law`.
    #[semio_framework_async_macros::async_test]
    async fn diff_codec_text_binary_roundtrip_law() {
        for d in demo_diff_cases() {
            let printed = d.print_diff();
            assert!(!printed.await.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = LasDiff::parse_diff(&printed).await.unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().await.unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = LasDiff::decode_diff(&encoded).await.unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
}
//#endregion Tests
