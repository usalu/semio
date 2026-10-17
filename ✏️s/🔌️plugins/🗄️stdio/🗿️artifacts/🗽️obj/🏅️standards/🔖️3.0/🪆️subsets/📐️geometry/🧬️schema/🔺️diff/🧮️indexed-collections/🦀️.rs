//! 🧮️ The shared index-collection algebra and the vertex, texture coordinate, normal and face diffs built on it.

use super::*;

//#region IndexCollectionCore
/// 🧮 Per-item sparse-diff behavior shared by the four flat, position-keyed collections. `Diff`
/// is that item's own PUBLIC sparse-patch type (`ObjVertexDiff`, …).
pub(super) trait ObjIndexElem: Clone + PartialEq {
    type Diff: Clone + PartialEq;
    fn diff_is_empty(d: &Self::Diff) -> bool;
    fn field_changes(a: &Self, b: &Self) -> Self::Diff;
    fn diff_inverse(d: &Self::Diff, base: &Self) -> Self::Diff;
    fn diff_apply(d: &Self::Diff, item: &mut Self);
    fn diff_absorb(base: &mut Self::Diff, other: Self::Diff);
}

/// ▶️ Applies a `(removed, modified, added)` triple to a base array — modified on BASE
/// positions first, then removed descending, then added ascending clamped to `min(index,len)`
/// (recipe's normative apply order).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn generic_apply<T: ObjIndexElem>(base: &[T], removed: &[usize], modified: &[(usize, T::Diff)], added: &[(usize, T)]) -> Vec<T> {
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

/// ↩️ Negative rows of a `(removed, modified, added)` triple against its BASE items: added rows become removals at their final
/// index, removed rows return at their base index, and each modified row restores its base fields at the index the row has after
/// the diff. Every list comes back ascending, the normal form [`generic_absorb_pair`] emits.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn generic_inverse<T: ObjIndexElem>(removed: &[usize], modified: &[(usize, T::Diff)], added: &[(usize, T)], base: &[T]) -> IndexedDiffParts<T::Diff, T> {
    let removed_sorted = semio_s_artifact_stdio_contract::ordered_unique(&removed);
    let mut added_final: Vec<usize> = added.iter().map(|(index, _)| *index).collect();
    added_final.sort_unstable();
    let after_index = |index: usize| {
        let survivor = index - removed_sorted.iter().filter(|dropped| **dropped < index).count();
        added_final.iter().fold(survivor, |position, inserted| if *inserted <= position { position + 1 } else { position })
    };
    let mut inverse_modified: Vec<(usize, T::Diff)> = modified.iter().filter_map(|(index, diff)| base.get(*index).map(|item| (after_index(*index), T::diff_inverse(diff, item)))).collect();
    inverse_modified.sort_by_key(|(index, _)| *index);
    let inverse_added = removed_sorted.iter().filter_map(|index| base.get(*index).map(|item| (*index, item.clone()))).collect();
    (added_final, inverse_modified, inverse_added)
}

/// 🏷️ A structural, base-free label used only inside [`generic_absorb_pair`] to simulate the
/// two-step position transform (base→mid via `d1`, mid→after via `d2`) without ever looking at
/// real item content. Mirrors `stdio.txt`'s proven `Lbl`/`simulate_labels`/`absorb_pair` shape.
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
    let added_sorted = semio_s_artifact_stdio_contract::ordered_by_key(added, |(idx, _)| *idx);
    for (idx, label) in added_sorted {
        let pos = idx.min(survivors.len());
        survivors.insert(pos, label);
    }
    survivors
}

/// ➕️ Absorbs `d1` (base→mid) then `d2` (mid→after) into a single base→after triple. See
/// `stdio.txt`'s `absorb_pair` doc comment for the full label-walk rationale — this is the same
/// algorithm, generic over `T: ObjIndexElem` instead of `String`.
#[allow(clippy::type_complexity)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn generic_absorb_pair<T: ObjIndexElem>(d1_removed: &[usize], d1_modified: &[(usize, T::Diff)], d1_added: &[(usize, T)], d2_removed: &[usize], d2_modified: &[(usize, T::Diff)], d2_added: &[(usize, T)]) -> IndexedDiffParts<T::Diff, T> {
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
//#endregion IndexCollectionCore

//#region 🔖️VertexDiff
/// 🔺️ Sparse per-field patch for one [`ObjVertex`].
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct ObjVertexDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub z: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub w: Option<Option<f64>>,
}

impl ObjIndexElem for ObjVertex {
    type Diff = ObjVertexDiff;
    fn diff_is_empty(d: &ObjVertexDiff) -> bool {
        d == &ObjVertexDiff::default()
    }
    fn field_changes(a: &ObjVertex, b: &ObjVertex) -> ObjVertexDiff {
        ObjVertexDiff { x: (a.x != b.x).then_some(b.x), y: (a.y != b.y).then_some(b.y), z: (a.z != b.z).then_some(b.z), w: (a.w != b.w).then_some(b.w) }
    }
    fn diff_inverse(d: &ObjVertexDiff, base: &ObjVertex) -> ObjVertexDiff {
        ObjVertexDiff { x: d.x.map(|_| base.x), y: d.y.map(|_| base.y), z: d.z.map(|_| base.z), w: d.w.map(|_| base.w) }
    }
    fn diff_apply(d: &ObjVertexDiff, item: &mut ObjVertex) {
        if let Some(v) = d.x {
            item.x = v;
        }
        if let Some(v) = d.y {
            item.y = v;
        }
        if let Some(v) = d.z {
            item.z = v;
        }
        if let Some(v) = d.w {
            item.w = v;
        }
    }
    fn diff_absorb(base: &mut ObjVertexDiff, other: ObjVertexDiff) {
        if other.x.is_some() {
            base.x = other.x;
        }
        if other.y.is_some() {
            base.y = other.y;
        }
        if other.z.is_some() {
            base.z = other.z;
        }
        if other.w.is_some() {
            base.w = other.w;
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct ObjVertexModified {
    pub index: usize,
    pub diff: ObjVertexDiff,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct ObjVertexAdded {
    pub index: usize,
    pub vertex: ObjVertex,
}
/// 🔺️ Index-keyed removed/modified/added triple over `ObjSnapshot::vertices`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct ObjVerticesDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<ObjVertexModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<ObjVertexAdded>,
}
impl ObjVerticesDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn apply(&self, base: &[ObjVertex]) -> Vec<ObjVertex> {
        let modified: Vec<(usize, ObjVertexDiff)> = self.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
        let added: Vec<(usize, ObjVertex)> = self.added.iter().map(|a| (a.index, a.vertex.clone())).collect();
        generic_apply(base, &self.removed, &modified, &added)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn inverse(&self, base: &[ObjVertex]) -> Option<Self> {
        let modified: Vec<(usize, ObjVertexDiff)> = self.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
        let added: Vec<(usize, ObjVertex)> = self.added.iter().map(|a| (a.index, a.vertex.clone())).collect();
        let (removed, modified, added) = generic_inverse(&self.removed, &modified, &added, base);
        let d = Self { removed, modified: modified.into_iter().map(|(index, diff)| ObjVertexModified { index, diff }).collect(), added: added.into_iter().map(|(index, vertex)| ObjVertexAdded { index, vertex }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn absorb(d1: Self, d2: Self) -> Option<Self> {
        let d1m: Vec<(usize, ObjVertexDiff)> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();
        let d1a: Vec<(usize, ObjVertex)> = d1.added.into_iter().map(|a| (a.index, a.vertex)).collect();
        let d2m: Vec<(usize, ObjVertexDiff)> = d2.modified.into_iter().map(|m| (m.index, m.diff)).collect();
        let d2a: Vec<(usize, ObjVertex)> = d2.added.into_iter().map(|a| (a.index, a.vertex)).collect();
        let (removed, modified, added) = generic_absorb_pair::<ObjVertex>(&d1.removed, &d1m, &d1a, &d2.removed, &d2m, &d2a);
        let d = Self { removed, modified: modified.into_iter().map(|(index, diff)| ObjVertexModified { index, diff }).collect(), added: added.into_iter().map(|(index, vertex)| ObjVertexAdded { index, vertex }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
}
//#endregion 🔖️VertexDiff

//#region 🔖️TexCoordDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct ObjTexCoordDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub u: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub v: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub w: Option<Option<f64>>,
}
impl ObjIndexElem for ObjTexCoord {
    type Diff = ObjTexCoordDiff;
    fn diff_is_empty(d: &ObjTexCoordDiff) -> bool {
        d == &ObjTexCoordDiff::default()
    }
    fn field_changes(a: &ObjTexCoord, b: &ObjTexCoord) -> ObjTexCoordDiff {
        ObjTexCoordDiff { u: (a.u != b.u).then_some(b.u), v: (a.v != b.v).then_some(b.v), w: (a.w != b.w).then_some(b.w) }
    }
    fn diff_inverse(d: &ObjTexCoordDiff, base: &ObjTexCoord) -> ObjTexCoordDiff {
        ObjTexCoordDiff { u: d.u.map(|_| base.u), v: d.v.map(|_| base.v), w: d.w.map(|_| base.w) }
    }
    fn diff_apply(d: &ObjTexCoordDiff, item: &mut ObjTexCoord) {
        if let Some(v) = d.u {
            item.u = v;
        }
        if let Some(v) = d.v {
            item.v = v;
        }
        if let Some(v) = d.w {
            item.w = v;
        }
    }
    fn diff_absorb(base: &mut ObjTexCoordDiff, other: ObjTexCoordDiff) {
        if other.u.is_some() {
            base.u = other.u;
        }
        if other.v.is_some() {
            base.v = other.v;
        }
        if other.w.is_some() {
            base.w = other.w;
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct ObjTexCoordModified {
    pub index: usize,
    pub diff: ObjTexCoordDiff,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct ObjTexCoordAdded {
    pub index: usize,
    pub texcoord: ObjTexCoord,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct ObjTexCoordsDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<ObjTexCoordModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<ObjTexCoordAdded>,
}
impl ObjTexCoordsDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn apply(&self, base: &[ObjTexCoord]) -> Vec<ObjTexCoord> {
        let modified: Vec<(usize, ObjTexCoordDiff)> = self.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
        let added: Vec<(usize, ObjTexCoord)> = self.added.iter().map(|a| (a.index, a.texcoord.clone())).collect();
        generic_apply(base, &self.removed, &modified, &added)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn inverse(&self, base: &[ObjTexCoord]) -> Option<Self> {
        let modified: Vec<(usize, ObjTexCoordDiff)> = self.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
        let added: Vec<(usize, ObjTexCoord)> = self.added.iter().map(|a| (a.index, a.texcoord.clone())).collect();
        let (removed, modified, added) = generic_inverse(&self.removed, &modified, &added, base);
        let d = Self { removed, modified: modified.into_iter().map(|(index, diff)| ObjTexCoordModified { index, diff }).collect(), added: added.into_iter().map(|(index, texcoord)| ObjTexCoordAdded { index, texcoord }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn absorb(d1: Self, d2: Self) -> Option<Self> {
        let d1m: Vec<(usize, ObjTexCoordDiff)> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();
        let d1a: Vec<(usize, ObjTexCoord)> = d1.added.into_iter().map(|a| (a.index, a.texcoord)).collect();
        let d2m: Vec<(usize, ObjTexCoordDiff)> = d2.modified.into_iter().map(|m| (m.index, m.diff)).collect();
        let d2a: Vec<(usize, ObjTexCoord)> = d2.added.into_iter().map(|a| (a.index, a.texcoord)).collect();
        let (removed, modified, added) = generic_absorb_pair::<ObjTexCoord>(&d1.removed, &d1m, &d1a, &d2.removed, &d2m, &d2a);
        let d = Self { removed, modified: modified.into_iter().map(|(index, diff)| ObjTexCoordModified { index, diff }).collect(), added: added.into_iter().map(|(index, texcoord)| ObjTexCoordAdded { index, texcoord }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
}
//#endregion 🔖️TexCoordDiff

//#region 🔖️NormalDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct ObjNormalDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub z: Option<f64>,
}
impl ObjIndexElem for ObjNormal {
    type Diff = ObjNormalDiff;
    fn diff_is_empty(d: &ObjNormalDiff) -> bool {
        d == &ObjNormalDiff::default()
    }
    fn field_changes(a: &ObjNormal, b: &ObjNormal) -> ObjNormalDiff {
        ObjNormalDiff { x: (a.x != b.x).then_some(b.x), y: (a.y != b.y).then_some(b.y), z: (a.z != b.z).then_some(b.z) }
    }
    fn diff_inverse(d: &ObjNormalDiff, base: &ObjNormal) -> ObjNormalDiff {
        ObjNormalDiff { x: d.x.map(|_| base.x), y: d.y.map(|_| base.y), z: d.z.map(|_| base.z) }
    }
    fn diff_apply(d: &ObjNormalDiff, item: &mut ObjNormal) {
        if let Some(v) = d.x {
            item.x = v;
        }
        if let Some(v) = d.y {
            item.y = v;
        }
        if let Some(v) = d.z {
            item.z = v;
        }
    }
    fn diff_absorb(base: &mut ObjNormalDiff, other: ObjNormalDiff) {
        if other.x.is_some() {
            base.x = other.x;
        }
        if other.y.is_some() {
            base.y = other.y;
        }
        if other.z.is_some() {
            base.z = other.z;
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct ObjNormalModified {
    pub index: usize,
    pub diff: ObjNormalDiff,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct ObjNormalAdded {
    pub index: usize,
    pub normal: ObjNormal,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct ObjNormalsDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<ObjNormalModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<ObjNormalAdded>,
}
impl ObjNormalsDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn apply(&self, base: &[ObjNormal]) -> Vec<ObjNormal> {
        let modified: Vec<(usize, ObjNormalDiff)> = self.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
        let added: Vec<(usize, ObjNormal)> = self.added.iter().map(|a| (a.index, a.normal.clone())).collect();
        generic_apply(base, &self.removed, &modified, &added)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn inverse(&self, base: &[ObjNormal]) -> Option<Self> {
        let modified: Vec<(usize, ObjNormalDiff)> = self.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
        let added: Vec<(usize, ObjNormal)> = self.added.iter().map(|a| (a.index, a.normal.clone())).collect();
        let (removed, modified, added) = generic_inverse(&self.removed, &modified, &added, base);
        let d = Self { removed, modified: modified.into_iter().map(|(index, diff)| ObjNormalModified { index, diff }).collect(), added: added.into_iter().map(|(index, normal)| ObjNormalAdded { index, normal }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn absorb(d1: Self, d2: Self) -> Option<Self> {
        let d1m: Vec<(usize, ObjNormalDiff)> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();
        let d1a: Vec<(usize, ObjNormal)> = d1.added.into_iter().map(|a| (a.index, a.normal)).collect();
        let d2m: Vec<(usize, ObjNormalDiff)> = d2.modified.into_iter().map(|m| (m.index, m.diff)).collect();
        let d2a: Vec<(usize, ObjNormal)> = d2.added.into_iter().map(|a| (a.index, a.normal)).collect();
        let (removed, modified, added) = generic_absorb_pair::<ObjNormal>(&d1.removed, &d1m, &d1a, &d2.removed, &d2m, &d2a);
        let d = Self { removed, modified: modified.into_iter().map(|(index, diff)| ObjNormalModified { index, diff }).collect(), added: added.into_iter().map(|(index, normal)| ObjNormalAdded { index, normal }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
}
//#endregion 🔖️NormalDiff

//#region 🔖️FaceDiff
/// 🔺️ `vertices` is a weak leaf value (a face's own v/vt/vn reference list) — whole-vec
/// replaced, never sub-diffed (recipe's weak-entity rule; a face's index list has no stable
/// per-slot identity worth tracking below the face itself).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct ObjFaceDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub vertices: Option<Vec<ObjFaceVertex>>,
}
impl ObjIndexElem for ObjFace {
    type Diff = ObjFaceDiff;
    fn diff_is_empty(d: &ObjFaceDiff) -> bool {
        d == &ObjFaceDiff::default()
    }
    fn field_changes(a: &ObjFace, b: &ObjFace) -> ObjFaceDiff {
        ObjFaceDiff { vertices: (a.vertices != b.vertices).then(|| b.vertices.clone()) }
    }
    fn diff_inverse(d: &ObjFaceDiff, base: &ObjFace) -> ObjFaceDiff {
        ObjFaceDiff { vertices: d.vertices.as_ref().map(|_| base.vertices.clone()) }
    }
    fn diff_apply(d: &ObjFaceDiff, item: &mut ObjFace) {
        if let Some(v) = &d.vertices {
            item.vertices = v.clone();
        }
    }
    fn diff_absorb(base: &mut ObjFaceDiff, other: ObjFaceDiff) {
        if other.vertices.is_some() {
            base.vertices = other.vertices;
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct ObjFaceModified {
    pub index: usize,
    pub diff: ObjFaceDiff,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct ObjFaceAdded {
    pub index: usize,
    pub face: ObjFace,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct ObjFacesDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<ObjFaceModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<ObjFaceAdded>,
}
impl ObjFacesDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn apply(&self, base: &[ObjFace]) -> Vec<ObjFace> {
        let modified: Vec<(usize, ObjFaceDiff)> = self.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
        let added: Vec<(usize, ObjFace)> = self.added.iter().map(|a| (a.index, a.face.clone())).collect();
        generic_apply(base, &self.removed, &modified, &added)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn inverse(&self, base: &[ObjFace]) -> Option<Self> {
        let modified: Vec<(usize, ObjFaceDiff)> = self.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
        let added: Vec<(usize, ObjFace)> = self.added.iter().map(|a| (a.index, a.face.clone())).collect();
        let (removed, modified, added) = generic_inverse(&self.removed, &modified, &added, base);
        let d = Self { removed, modified: modified.into_iter().map(|(index, diff)| ObjFaceModified { index, diff }).collect(), added: added.into_iter().map(|(index, face)| ObjFaceAdded { index, face }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn absorb(d1: Self, d2: Self) -> Option<Self> {
        let d1m: Vec<(usize, ObjFaceDiff)> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();
        let d1a: Vec<(usize, ObjFace)> = d1.added.into_iter().map(|a| (a.index, a.face)).collect();
        let d2m: Vec<(usize, ObjFaceDiff)> = d2.modified.into_iter().map(|m| (m.index, m.diff)).collect();
        let d2a: Vec<(usize, ObjFace)> = d2.added.into_iter().map(|a| (a.index, a.face)).collect();
        let (removed, modified, added) = generic_absorb_pair::<ObjFace>(&d1.removed, &d1m, &d1a, &d2.removed, &d2m, &d2a);
        let d = Self { removed, modified: modified.into_iter().map(|(index, diff)| ObjFaceModified { index, diff }).collect(), added: added.into_iter().map(|(index, face)| ObjFaceAdded { index, face }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
}
//#endregion 🔖️FaceDiff
