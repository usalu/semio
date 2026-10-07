//! 🔺️ ObjDiff — handcrafted sparse diff. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: replaces the old
//! `ObjDiff{snapshot: Option<ObjSnapshot>}` full-replace template with real per-field patches —
//! four index-keyed recursive triples (`vertices`/`texcoords`/`normals`/`faces`, per the recipe's
//! "index keys pairwise by position" rule), two name-keyed triples (`groups`/`objects`), a
//! tri-state scalar (`mtllib`), and three whole-vec-replace scalars for the range-tagged/
//! position-retained weak-value lists (`usemtl`/`smoothingGroups`/`unknownStatements`).
//!
//! The four index-keyed collections share IDENTICAL position algebra (apply order: modified on
//! BASE positions, then removed descending, then added ascending clamped — and the matching
//! label-simulation absorb from the recipe's `## Absorb`, same shape as `stdio.txt`'s proven
//! `TxtLinesDiff`/`absorb_pair`). That algebra is written ONCE in `#[region IndexCollectionCore]`
//! below via a small intra-file `ObjIndexElem` trait — this is pure code reuse WITHIN this one
//! artifact's own diff file (never exported, never shared with another artifact's types); every
//! PUBLIC diff type below (`ObjVerticesDiff`, `ObjTexCoordsDiff`, …) stays a fully concrete,
//! per-artifact named type with its own field names, matching the recipe's "specific code, not
//! generic" mandate at the type-shape level.
//!
//! 🧪️ F6: `protocol::DiffCodec` for `ObjDiff` is **hand-rolled** (`#region HandcraftedDiffCodec`
//! below), NOT `#[derive(dsl::DslDiff)]` — ticket `f6-recon-report.md` §3b's tri-state blocker:
//! `ObjVertexDiff::w`/`ObjTexCoordDiff::w` and `ObjDiff::mtllib` are all `Option<Option<T>>`, and
//! the derive's `classify_field` can only ever peel exactly one `Option<..>` layer before needing
//! `Option<T>: DslField` — a blanket impl that does not exist anywhere in the `dsl` crate (confirmed
//! empirically, same failure shape as `GifFrameDiff`/`GifDiff` in the recon pilot: `the trait bound
//! std::option::Option<f64>: DslField is not satisfied`). No enum anywhere in this file's own types
//! (3a doesn't apply here — `obj`'s whole model is plain structs/`Vec`/`Option<T>`), so this is a
//! pure 3b case, unlike svg's combined 3a+3b. The grammar below reuses §5's primitive set
//! (`hex_encode`/`split_top_level`/`encode_option`/…) verbatim per artifact convention; `f64` fields
//! use Rust's own round-trippable `Display`/`FromStr` (no external float-formatting dep needed).

#[cfg(test)]
use crate::standards::v3_0::subsets::any::io::binary::diff::dec_unknown_bin;
#[cfg(test)]
use crate::standards::v3_0::subsets::any::io::binary::diff::enc_unknown_bin;
/// 🧩 Ordered removed keys, modified values, and inserted items.
pub(crate) type IndexedDiffParts<D, T> = (Vec<usize>, Vec<(usize, D)>, Vec<(usize, T)>);

/// 🧩 Ordered removed keys, modified values, and inserted items.
pub(crate) type NamedDiffParts<D, T> = (Vec<String>, Vec<(String, D)>, Vec<(usize, T)>);

use crate::schema::snapshot::{ObjFace, ObjGroup, ObjNormal, ObjObject, ObjTexCoord, ObjVertex};
use crate::ObjSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{DiffCodec};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{BTreeSet, HashMap, HashSet};

//#region IndexCollectionCore
/// 🧮 Per-item sparse-diff behavior shared by the four flat, position-keyed collections. `Diff`
/// is that item's own PUBLIC sparse-patch type (`ObjVertexDiff`, …).
trait ObjIndexElem: Clone + PartialEq {
    type Diff: Clone + PartialEq;
    fn diff_is_empty(d: &Self::Diff) -> bool;
    fn diff_between(a: &Self, b: &Self) -> Self::Diff;
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

/// 🧭️ Pairwise-by-position state delta: `modified` over `0..min(len)`, base tail `removed`,
/// other tail `added` (recipe's "index keys pairwise by position" `between` rule).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn generic_between<T: ObjIndexElem>(base: &[T], other: &[T]) -> IndexedDiffParts<T::Diff, T> {
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
    let mut added_sorted = added.to_vec();
    added_sorted.sort_by_key(|(idx, _)| *idx);
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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
    fn diff_between(a: &ObjVertex, b: &ObjVertex) -> ObjVertexDiff {
        ObjVertexDiff { x: (a.x != b.x).then_some(b.x), y: (a.y != b.y).then_some(b.y), z: (a.z != b.z).then_some(b.z), w: (a.w != b.w).then_some(b.w) }
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

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ObjVertexModified {
    pub index: usize,
    pub diff: ObjVertexDiff,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ObjVertexAdded {
    pub index: usize,
    pub vertex: ObjVertex,
}
/// 🔺️ Index-keyed removed/modified/added triple over `ObjSnapshot::vertices`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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
    fn apply(&self, base: &[ObjVertex]) -> Vec<ObjVertex> {
        let modified: Vec<(usize, ObjVertexDiff)> = self.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
        let added: Vec<(usize, ObjVertex)> = self.added.iter().map(|a| (a.index, a.vertex.clone())).collect();
        generic_apply(base, &self.removed, &modified, &added)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn between(base: &[ObjVertex], other: &[ObjVertex]) -> Option<Self> {
        let (removed, modified, added) = generic_between(base, other);
        let d = Self { removed, modified: modified.into_iter().map(|(index, diff)| ObjVertexModified { index, diff }).collect(), added: added.into_iter().map(|(index, vertex)| ObjVertexAdded { index, vertex }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn absorb(d1: Self, d2: Self) -> Option<Self> {
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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
    fn diff_between(a: &ObjTexCoord, b: &ObjTexCoord) -> ObjTexCoordDiff {
        ObjTexCoordDiff { u: (a.u != b.u).then_some(b.u), v: (a.v != b.v).then_some(b.v), w: (a.w != b.w).then_some(b.w) }
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ObjTexCoordModified {
    pub index: usize,
    pub diff: ObjTexCoordDiff,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ObjTexCoordAdded {
    pub index: usize,
    pub texcoord: ObjTexCoord,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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
    fn apply(&self, base: &[ObjTexCoord]) -> Vec<ObjTexCoord> {
        let modified: Vec<(usize, ObjTexCoordDiff)> = self.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
        let added: Vec<(usize, ObjTexCoord)> = self.added.iter().map(|a| (a.index, a.texcoord.clone())).collect();
        generic_apply(base, &self.removed, &modified, &added)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn between(base: &[ObjTexCoord], other: &[ObjTexCoord]) -> Option<Self> {
        let (removed, modified, added) = generic_between(base, other);
        let d = Self { removed, modified: modified.into_iter().map(|(index, diff)| ObjTexCoordModified { index, diff }).collect(), added: added.into_iter().map(|(index, texcoord)| ObjTexCoordAdded { index, texcoord }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn absorb(d1: Self, d2: Self) -> Option<Self> {
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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
    fn diff_between(a: &ObjNormal, b: &ObjNormal) -> ObjNormalDiff {
        ObjNormalDiff { x: (a.x != b.x).then_some(b.x), y: (a.y != b.y).then_some(b.y), z: (a.z != b.z).then_some(b.z) }
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ObjNormalModified {
    pub index: usize,
    pub diff: ObjNormalDiff,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ObjNormalAdded {
    pub index: usize,
    pub normal: ObjNormal,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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
    fn apply(&self, base: &[ObjNormal]) -> Vec<ObjNormal> {
        let modified: Vec<(usize, ObjNormalDiff)> = self.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
        let added: Vec<(usize, ObjNormal)> = self.added.iter().map(|a| (a.index, a.normal.clone())).collect();
        generic_apply(base, &self.removed, &modified, &added)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn between(base: &[ObjNormal], other: &[ObjNormal]) -> Option<Self> {
        let (removed, modified, added) = generic_between(base, other);
        let d = Self { removed, modified: modified.into_iter().map(|(index, diff)| ObjNormalModified { index, diff }).collect(), added: added.into_iter().map(|(index, normal)| ObjNormalAdded { index, normal }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn absorb(d1: Self, d2: Self) -> Option<Self> {
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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
    fn diff_between(a: &ObjFace, b: &ObjFace) -> ObjFaceDiff {
        ObjFaceDiff { vertices: (a.vertices != b.vertices).then(|| b.vertices.clone()) }
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ObjFaceModified {
    pub index: usize,
    pub diff: ObjFaceDiff,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ObjFaceAdded {
    pub index: usize,
    pub face: ObjFace,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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
    fn apply(&self, base: &[ObjFace]) -> Vec<ObjFace> {
        let modified: Vec<(usize, ObjFaceDiff)> = self.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
        let added: Vec<(usize, ObjFace)> = self.added.iter().map(|a| (a.index, a.face.clone())).collect();
        generic_apply(base, &self.removed, &modified, &added)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn between(base: &[ObjFace], other: &[ObjFace]) -> Option<Self> {
        let (removed, modified, added) = generic_between(base, other);
        let d = Self { removed, modified: modified.into_iter().map(|(index, diff)| ObjFaceModified { index, diff }).collect(), added: added.into_iter().map(|(index, face)| ObjFaceAdded { index, face }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn absorb(d1: Self, d2: Self) -> Option<Self> {
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

//#region 🔖️GroupDiff
/// 🔺️ Sparse patch for one [`ObjGroup`]/[`ObjObject`] — `faces` is a weak whole-list value
/// (membership set), replaced wholesale, never sub-diffed.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ObjGroupDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub faces: Option<Vec<u64>>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn group_diff_is_empty(d: &ObjGroupDiff) -> bool {
    d == &ObjGroupDiff::default()
}

/// 🧮 `ObjGroup` and `ObjObject` are structurally identical (`name`/`faces`) but distinct named
/// types per the recipe — this tiny local trait lets `apply_group_diff`/membership helpers work
/// over either without merging the two public types back into one shared type.
trait HasFaces {
    fn faces_mut(&mut self) -> &mut Vec<u64>;
}
impl HasFaces for ObjGroup {
    fn faces_mut(&mut self) -> &mut Vec<u64> {
        &mut self.faces
    }
}
impl HasFaces for ObjObject {
    fn faces_mut(&mut self) -> &mut Vec<u64> {
        &mut self.faces
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn group_between(a_faces: &[u64], b_faces: &[u64]) -> ObjGroupDiff {
    ObjGroupDiff { faces: (a_faces != b_faces).then(|| b_faces.to_vec()) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_group_diff<T: HasFaces>(item: &mut T, d: &ObjGroupDiff) {
    if let Some(f) = &d.faces {
        *item.faces_mut() = f.clone();
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_group_diff(base: &mut ObjGroupDiff, other: ObjGroupDiff) {
    if other.faces.is_some() {
        base.faces = other.faces;
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ObjGroupModified {
    pub name: String,
    pub diff: ObjGroupDiff,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ObjGroupAdded {
    pub index: usize,
    pub group: ObjGroup,
}
/// 🔺️ Name-keyed removed/modified/added triple over `ObjSnapshot::groups` (same shape as
/// `stdio.zip`'s `ZipEntriesDiff` — no rename tracking needed here, `g` statements only ever
/// add/remove named groups, never rename one in place).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ObjGroupsDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<ObjGroupModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<ObjGroupAdded>,
}
impl ObjGroupsDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

/// ▶️ Applies a name-keyed groups/objects triple in place (shared shape; parameterized over the
/// `name`/`faces` accessor pair since `ObjGroup`/`ObjObject` are structurally identical but
/// distinct named types per the recipe).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_named_membership<T: Clone>(base: &[T], removed: &[String], modified: &[(String, ObjGroupDiff)], added: &[(usize, T)], name_of: impl Fn(&T) -> &str, patch: impl Fn(&mut T, &ObjGroupDiff)) -> Vec<T> {
    let mut items = base.to_vec();
    for (name, d) in modified {
        for item in &mut items {
            if name_of(item) == name {
                patch(item, d);
            }
        }
    }
    let removed_set: HashSet<&str> = removed.iter().map(String::as_str).collect();
    items.retain(|it| !removed_set.contains(name_of(it)));
    let mut adds: Vec<&(usize, T)> = added.iter().collect();
    adds.sort_by_key(|(i, _)| *i);
    for (idx, item) in adds {
        items.insert(*idx, item.clone());
    }
    items
}

/// ➕️ Structural, total, base-free absorb for a name-keyed groups/objects triple — same
/// algorithm as `stdio.zip`'s `absorb_entries` minus rename tracking (φ is the identity on
/// names here since nothing renames a group/object in place).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_named_membership<T: Clone>(
    d1: ObjGroupsDiff,
    d2: ObjGroupsDiff,
    name_of: impl Fn(&T) -> &str,
    patch: impl Fn(&mut T, &ObjGroupDiff),
    added_item: impl Fn(&ObjGroupAdded) -> T,
    wrap_added: impl Fn(usize, T) -> ObjGroupAdded,
) -> Option<ObjGroupsDiff> {
    let added_names: HashSet<String> = d1.added.iter().map(|a| name_of(&added_item(a)).to_string()).collect();

    let mut merged_removed: Vec<String> = d1.removed;
    let mut annihilated: HashSet<String> = HashSet::new();
    for name in &d2.removed {
        if added_names.contains(name) {
            annihilated.insert(name.clone());
        } else if !merged_removed.contains(name) {
            merged_removed.push(name.clone());
        }
    }
    let mut d1_modified = d1.modified;
    d1_modified.retain(|m| !merged_removed.contains(&m.name));

    let mut merged_added: Vec<ObjGroupAdded> = d1.added.into_iter().filter(|a| !annihilated.contains(name_of(&added_item(a)))).collect();
    let mut merged_modified: Vec<ObjGroupModified> = d1_modified;

    for dm in &d2.modified {
        if added_names.contains(&dm.name) {
            if annihilated.contains(&dm.name) {
                continue;
            }
            if let Some(a) = merged_added.iter_mut().find(|a| name_of(&added_item(a)) == dm.name) {
                let mut item = added_item(a);
                patch(&mut item, &dm.diff);
                *a = wrap_added(a.index, item);
            }
        } else {
            if merged_removed.contains(&dm.name) {
                continue;
            }
            if let Some(existing) = merged_modified.iter_mut().find(|m| m.name == dm.name) {
                absorb_group_diff(&mut existing.diff, dm.diff.clone());
            } else {
                merged_modified.push(ObjGroupModified { name: dm.name.clone(), diff: dm.diff.clone() });
            }
        }
    }

    merged_added.extend(d2.added);
    let merged = ObjGroupsDiff { removed: merged_removed, modified: merged_modified, added: merged_added };
    if merged.is_empty() {
        None
    } else {
        Some(merged)
    }
}
//#endregion 🔖️GroupDiff

//#region 🔖️ObjectsDiff
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ObjObjectAdded {
    pub index: usize,
    pub object: ObjObject,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ObjObjectsDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<ObjGroupModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<ObjObjectAdded>,
}
impl ObjObjectsDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}
//#endregion 🔖️ObjectsDiff

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.obj`. `schema` is an identity field and never appears here.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.obj.diff")]
pub struct ObjDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub vertices: Option<ObjVerticesDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub texcoords: Option<ObjTexCoordsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub normals: Option<ObjNormalsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub faces: Option<ObjFacesDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<ObjGroupsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub objects: Option<ObjObjectsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mtllib: Option<Option<String>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub usemtl: Option<Vec<ObjUsemtlRange>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub smoothing_groups: Option<Vec<ObjSmoothingRange>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub unknown_statements: Option<Vec<ObjUnknownStatement>>,
}

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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_named_targets<'a>(
    base_names: impl IntoIterator<Item = &'a str>,
    removed_names: impl IntoIterator<Item = &'a str>,
    modified_names: impl IntoIterator<Item = &'a str>,
    added: impl IntoIterator<Item = (usize, &'a str)>,
    target: &str,
) -> MutationApplyResult<()> {
    let mut base = BTreeSet::new();
    for name in base_names {
        if !base.insert(name) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-base-target", "base names must be unique").at([target, name]));
        }
    }
    let mut removed = BTreeSet::new();
    for name in removed_names {
        if !base.contains(name) || !removed.insert(name) {
            return Err(MutationApplyError::new("mutation.apply.invalid-remove-target", "removal target must exist exactly once").at([target, name]));
        }
    }
    let mut modified = BTreeSet::new();
    for name in modified_names {
        if !base.contains(name) || removed.contains(name) || !modified.insert(name) {
            return Err(MutationApplyError::new("mutation.apply.invalid-modify-target", "modification target must exist exactly once and remain present").at([target, name]));
        }
    }
    let mut additions: Vec<(usize, &str)> = added.into_iter().collect();
    additions.sort_by_key(|(index, _)| *index);
    let mut added_names = BTreeSet::new();
    let mut previous = None;
    for (length, (index, name)) in (base.len() - removed.len()..).zip(additions) {
        if base.contains(name) || !added_names.insert(name) || index > length || previous == Some(index) {
            return Err(MutationApplyError::new("mutation.apply.invalid-add-target", "addition name and position must be unique and valid").at([target, name]));
        }
        previous = Some(index);
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_obj_diff(diff: &ObjDiff, base: &ObjSnapshot) -> MutationApplyResult<()> {
    if let Some(value) = &diff.vertices {
        validate_indexed_targets(base.vertices.len(), &value.removed, value.modified.iter().map(|entry| entry.index), value.added.iter().map(|entry| entry.index), "vertices")?;
    }
    if let Some(value) = &diff.texcoords {
        validate_indexed_targets(base.texcoords.len(), &value.removed, value.modified.iter().map(|entry| entry.index), value.added.iter().map(|entry| entry.index), "texcoords")?;
    }
    if let Some(value) = &diff.normals {
        validate_indexed_targets(base.normals.len(), &value.removed, value.modified.iter().map(|entry| entry.index), value.added.iter().map(|entry| entry.index), "normals")?;
    }
    if let Some(value) = &diff.faces {
        validate_indexed_targets(base.faces.len(), &value.removed, value.modified.iter().map(|entry| entry.index), value.added.iter().map(|entry| entry.index), "faces")?;
    }
    if let Some(value) = &diff.groups {
        validate_named_targets(
            base.groups.iter().map(|entry| entry.name.as_str()),
            value.removed.iter().map(String::as_str),
            value.modified.iter().map(|entry| entry.name.as_str()),
            value.added.iter().map(|entry| (entry.index, entry.group.name.as_str())),
            "groups",
        )?;
    }
    if let Some(value) = &diff.objects {
        validate_named_targets(
            base.objects.iter().map(|entry| entry.name.as_str()),
            value.removed.iter().map(String::as_str),
            value.modified.iter().map(|entry| entry.name.as_str()),
            value.added.iter().map(|entry| (entry.index, entry.object.name.as_str())),
            "objects",
        )?;
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_obj_diff_unchecked(diff: &ObjDiff, base: &ObjSnapshot) -> ObjSnapshot {
    ObjSnapshot {
        schema: base.schema.clone(),
        vertices: match &diff.vertices {
            Some(d) => d.apply(&base.vertices),
            None => base.vertices.clone(),
        },
        texcoords: match &diff.texcoords {
            Some(d) => d.apply(&base.texcoords),
            None => base.texcoords.clone(),
        },
        normals: match &diff.normals {
            Some(d) => d.apply(&base.normals),
            None => base.normals.clone(),
        },
        faces: match &diff.faces {
            Some(d) => d.apply(&base.faces),
            None => base.faces.clone(),
        },
        groups: match &diff.groups {
            Some(d) => {
                let modified: Vec<(String, ObjGroupDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
                let added: Vec<(usize, ObjGroup)> = d.added.iter().map(|a| (a.index, a.group.clone())).collect();
                apply_named_membership(&base.groups, &d.removed, &modified, &added, |g| g.name.as_str(), apply_group_diff)
            }
            None => base.groups.clone(),
        },
        objects: match &diff.objects {
            Some(d) => {
                let modified: Vec<(String, ObjGroupDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
                let added: Vec<(usize, ObjObject)> = d.added.iter().map(|a| (a.index, a.object.clone())).collect();
                apply_named_membership(&base.objects, &d.removed, &modified, &added, |o| o.name.as_str(), apply_group_diff)
            }
            None => base.objects.clone(),
        },
        mtllib: diff.mtllib.clone().unwrap_or_else(|| base.mtllib.clone()),
        usemtl: diff.usemtl.clone().unwrap_or_else(|| base.usemtl.clone()),
        smoothing_groups: diff.smoothing_groups.clone().unwrap_or_else(|| base.smoothing_groups.clone()),
        unknown_statements: diff.unknown_statements.clone().unwrap_or_else(|| base.unknown_statements.clone()),
    }
}

impl MutationDiff<ObjSnapshot> for ObjDiff {
    fn apply(&self, base: &ObjSnapshot) -> MutationApplyResult<ObjSnapshot> {
        validate_obj_diff(self, base)?;
        Ok(apply_obj_diff_unchecked(self, base))
    }

    /// ➕️ Structural, total, base-free sequential-coalesce (`## Absorb` contract). Index-keyed
    /// collections use the label-simulation transport (`generic_absorb_pair`); name-keyed
    /// collections use `absorb_named_membership`; every other field is LWW.
    fn absorb(&mut self, other: Self) {
        self.vertices = match (self.vertices.take(), other.vertices) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => ObjVerticesDiff::absorb(a, b),
        };
        self.texcoords = match (self.texcoords.take(), other.texcoords) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => ObjTexCoordsDiff::absorb(a, b),
        };
        self.normals = match (self.normals.take(), other.normals) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => ObjNormalsDiff::absorb(a, b),
        };
        self.faces = match (self.faces.take(), other.faces) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => ObjFacesDiff::absorb(a, b),
        };
        self.groups = match (self.groups.take(), other.groups) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => absorb_named_membership(a, b, |g: &ObjGroup| g.name.as_str(), apply_group_diff, |added: &ObjGroupAdded| added.group.clone(), |index, group| ObjGroupAdded { index, group }),
        };
        self.objects = match (self.objects.take(), other.objects) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => {
                let a2 = ObjGroupsDiff { removed: a.removed, modified: a.modified, added: a.added.into_iter().map(|x| ObjGroupAdded { index: x.index, group: ObjGroup { name: x.object.name, faces: x.object.faces } }).collect() };
                let b2 = ObjGroupsDiff { removed: b.removed, modified: b.modified, added: b.added.into_iter().map(|x| ObjGroupAdded { index: x.index, group: ObjGroup { name: x.object.name, faces: x.object.faces } }).collect() };
                absorb_named_membership(a2, b2, |g: &ObjGroup| g.name.as_str(), apply_group_diff, |added: &ObjGroupAdded| added.group.clone(), |index, group| ObjGroupAdded { index, group }).map(|merged| ObjObjectsDiff {
                    removed: merged.removed,
                    modified: merged.modified,
                    added: merged.added.into_iter().map(|x| ObjObjectAdded { index: x.index, object: ObjObject { name: x.group.name, faces: x.group.faces } }).collect(),
                })
            }
        };
        if other.mtllib.is_some() {
            self.mtllib = other.mtllib;
        }
        if other.usemtl.is_some() {
            self.usemtl = other.usemtl;
        }
        if other.smoothing_groups.is_some() {
            self.smoothing_groups = other.smoothing_groups;
        }
        if other.unknown_statements.is_some() {
            self.unknown_statements = other.unknown_statements;
        }
    }
}

impl DiffAlgebra<ObjSnapshot> for ObjDiff {
    /// 🔁️ Diff-level undo, derived generically (correct by construction) via `apply` + `between`.
    fn inverse(&self, base: &ObjSnapshot) -> Self {
        let mutated = apply_obj_diff_unchecked(self, base);
        Self::between(&mutated, base)
    }

    fn between(base: &ObjSnapshot, other: &ObjSnapshot) -> Self {
        let vertices = ObjVerticesDiff::between(&base.vertices, &other.vertices);
        let texcoords = ObjTexCoordsDiff::between(&base.texcoords, &other.texcoords);
        let normals = ObjNormalsDiff::between(&base.normals, &other.normals);
        let faces = ObjFacesDiff::between(&base.faces, &other.faces);

        let groups = {
            let base_names: HashSet<&str> = base.groups.iter().map(|g| g.name.as_str()).collect();
            let other_names: HashSet<&str> = other.groups.iter().map(|g| g.name.as_str()).collect();
            let removed: Vec<String> = base.groups.iter().filter(|g| !other_names.contains(g.name.as_str())).map(|g| g.name.clone()).collect();
            let mut modified = Vec::new();
            for bg in &base.groups {
                if let Some(og) = other.groups.iter().find(|o| o.name == bg.name) {
                    let d = group_between(&bg.faces, &og.faces);
                    if !group_diff_is_empty(&d) {
                        modified.push(ObjGroupModified { name: bg.name.clone(), diff: d });
                    }
                }
            }
            let added: Vec<ObjGroupAdded> = other.groups.iter().enumerate().filter(|(_, g)| !base_names.contains(g.name.as_str())).map(|(index, g)| ObjGroupAdded { index, group: g.clone() }).collect();
            let d = ObjGroupsDiff { removed, modified, added };
            if d.is_empty() {
                None
            } else {
                Some(d)
            }
        };

        let objects = {
            let base_names: HashSet<&str> = base.objects.iter().map(|o| o.name.as_str()).collect();
            let other_names: HashSet<&str> = other.objects.iter().map(|o| o.name.as_str()).collect();
            let removed: Vec<String> = base.objects.iter().filter(|o| !other_names.contains(o.name.as_str())).map(|o| o.name.clone()).collect();
            let mut modified = Vec::new();
            for bo in &base.objects {
                if let Some(oo) = other.objects.iter().find(|o| o.name == bo.name) {
                    let d = group_between(&bo.faces, &oo.faces);
                    if !group_diff_is_empty(&d) {
                        modified.push(ObjGroupModified { name: bo.name.clone(), diff: d });
                    }
                }
            }
            let added: Vec<ObjObjectAdded> = other.objects.iter().enumerate().filter(|(_, o)| !base_names.contains(o.name.as_str())).map(|(index, o)| ObjObjectAdded { index, object: o.clone() }).collect();
            let d = ObjObjectsDiff { removed, modified, added };
            if d.is_empty() {
                None
            } else {
                Some(d)
            }
        };

        ObjDiff {
            vertices,
            texcoords,
            normals,
            faces,
            groups,
            objects,
            mtllib: (base.mtllib != other.mtllib).then(|| other.mtllib.clone()),
            usemtl: (base.usemtl != other.usemtl).then(|| other.usemtl.clone()),
            smoothing_groups: (base.smoothing_groups != other.smoothing_groups).then(|| other.smoothing_groups.clone()),
            unknown_statements: (base.unknown_statements != other.unknown_statements).then(|| other.unknown_statements.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.vertices.as_ref().is_none_or(ObjVerticesDiff::is_empty)
            && self.texcoords.as_ref().is_none_or(ObjTexCoordsDiff::is_empty)
            && self.normals.as_ref().is_none_or(ObjNormalsDiff::is_empty)
            && self.faces.as_ref().is_none_or(ObjFacesDiff::is_empty)
            && self.groups.as_ref().is_none_or(ObjGroupsDiff::is_empty)
            && self.objects.as_ref().is_none_or(ObjObjectsDiff::is_empty)
            && self.mtllib.is_none()
            && self.usemtl.is_none()
            && self.smoothing_groups.is_none()
            && self.unknown_statements.is_none()
    }
}

/// 🧩 `SetSnapshot`'s diff is the sparse field-by-field `between(base, next)` — no full-replace
/// slot exists on `ObjDiff` to short-circuit into.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &ObjSnapshot, next: &ObjSnapshot) -> ObjDiff {
    ObjDiff::between(base, next)
}
//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: hand-rolled `protocol::DiffCodec` for `ObjDiff` — see the module doc comment at the top
/// of this file for the confirmed 3b (tri-state) compile-error citation. **Grammar** (real, not
/// `serde_json`, mirrors gif89a/svg's §5 template): one space-separated token per changed
/// top-level field (absent token = unchanged); the four index-keyed collections
/// (`vertices`/`texcoords`/`normals`/`faces`) and the two name-keyed collections
/// (`groups`/`objects`) print as `name{[removed];[modified];[added]}`; strings (group/object/
/// material names, `mtllib`, retained raw source lines) are lowercase hex (no external base64
/// dep, no escaping needed — this artifact's `unknown_statements` can contain arbitrary source
/// text); `f64` fields use Rust's own round-trippable `to_string`/`parse` (no external float-fmt
/// dep); `Option<T>` values (both real optional snapshot fields like `ObjVertex::w` AND diff
/// tri-states like `ObjDiff::mtllib`) use the uniform `[0]`=None / `[1,<T>]`=Some(T) tag; per-item
/// sparse diffs (`ObjVertexDiff`, `ObjTexCoordDiff`, `ObjNormalDiff`, `ObjFaceDiff`,
/// `ObjGroupDiff`) print as single-uppercase-letter `TAG:value` pairs inside their own `[...]`,
/// same convention as gif89a's `GifFrameDiff`.
//#region 🔖️Primitives













//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs




















//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs










//#endregion 🔖️DiffValueCodecs

//#region 🔖️CollectionCodecs


















//#endregion 🔖️CollectionCodecs

//#region 🔖️BinaryPrimitives














//#endregion 🔖️BinaryPrimitives

//#region 🔖️ValueBinaryCodecs





















#[cfg(test)]
#[test]
fn sqlite_snapshot_obj_unsigned_source_positions_have_lossless_diff_twins() {
    let value = ObjUnknownStatement { line_index: u64::MAX, raw: "# Quelle 🎨".into() };
    assert_eq!(dec_unknown(&enc_unknown(&value)).unwrap(), value);
    let mut bytes = Vec::new();
    enc_unknown_bin(&value, &mut bytes);
    assert_eq!(dec_unknown_bin(&mut store::ByteReader::new(&bytes)).unwrap(), value);
}
//#endregion 🔖️ValueBinaryCodecs

//#region 🔖️DiffValueBinaryCodecs










//#endregion 🔖️DiffValueBinaryCodecs

//#region 🔖️CollectionBinaryCodecs

















//#endregion 🔖️CollectionBinaryCodecs

//#region 🔖️TopLevel




//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️MutationDiffBuilders
// 🧮 Item-level `between` wrappers, exposed to `🧬️mutations` so `SetVertex`/`SetTexcoord`/
// `SetNormal`/`SetFace`'s `diff()` can compute a sparse per-field patch without the private
// `ObjIndexElem` trait itself leaving this module.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn vertex_diff_between(a: &ObjVertex, b: &ObjVertex) -> ObjVertexDiff {
    <ObjVertex as ObjIndexElem>::diff_between(a, b)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn texcoord_diff_between(a: &ObjTexCoord, b: &ObjTexCoord) -> ObjTexCoordDiff {
    <ObjTexCoord as ObjIndexElem>::diff_between(a, b)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn normal_diff_between(a: &ObjNormal, b: &ObjNormal) -> ObjNormalDiff {
    <ObjNormal as ObjIndexElem>::diff_between(a, b)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn face_diff_between(a: &ObjFace, b: &ObjFace) -> ObjFaceDiff {
    <ObjFace as ObjIndexElem>::diff_between(a, b)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_vertex(index: usize, vertex: ObjVertex) -> ObjDiff {
    ObjDiff { vertices: Some(ObjVerticesDiff { removed: vec![], modified: vec![], added: vec![ObjVertexAdded { index, vertex }] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_vertex(index: usize) -> ObjDiff {
    ObjDiff { vertices: Some(ObjVerticesDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_vertex(index: usize, diff: ObjVertexDiff) -> ObjDiff {
    ObjDiff { vertices: Some(ObjVerticesDiff { removed: vec![], modified: vec![ObjVertexModified { index, diff }], added: vec![] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_texcoord(index: usize, texcoord: ObjTexCoord) -> ObjDiff {
    ObjDiff { texcoords: Some(ObjTexCoordsDiff { removed: vec![], modified: vec![], added: vec![ObjTexCoordAdded { index, texcoord }] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_texcoord(index: usize) -> ObjDiff {
    ObjDiff { texcoords: Some(ObjTexCoordsDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_texcoord(index: usize, diff: ObjTexCoordDiff) -> ObjDiff {
    ObjDiff { texcoords: Some(ObjTexCoordsDiff { removed: vec![], modified: vec![ObjTexCoordModified { index, diff }], added: vec![] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_normal(index: usize, normal: ObjNormal) -> ObjDiff {
    ObjDiff { normals: Some(ObjNormalsDiff { removed: vec![], modified: vec![], added: vec![ObjNormalAdded { index, normal }] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_normal(index: usize) -> ObjDiff {
    ObjDiff { normals: Some(ObjNormalsDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_normal(index: usize, diff: ObjNormalDiff) -> ObjDiff {
    ObjDiff { normals: Some(ObjNormalsDiff { removed: vec![], modified: vec![ObjNormalModified { index, diff }], added: vec![] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_face(index: usize, face: ObjFace) -> ObjDiff {
    ObjDiff { faces: Some(ObjFacesDiff { removed: vec![], modified: vec![], added: vec![ObjFaceAdded { index, face }] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_face(index: usize) -> ObjDiff {
    ObjDiff { faces: Some(ObjFacesDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_face(index: usize, diff: ObjFaceDiff) -> ObjDiff {
    ObjDiff { faces: Some(ObjFacesDiff { removed: vec![], modified: vec![ObjFaceModified { index, diff }], added: vec![] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_group(index: usize, name: &str, faces: Vec<u64>, existed: bool) -> ObjDiff {
    if existed {
        ObjDiff { groups: Some(ObjGroupsDiff { removed: vec![], modified: vec![ObjGroupModified { name: name.to_string(), diff: ObjGroupDiff { faces: Some(faces) } }], added: vec![] }), ..Default::default() }
    } else {
        ObjDiff { groups: Some(ObjGroupsDiff { removed: vec![], modified: vec![], added: vec![ObjGroupAdded { index, group: ObjGroup { name: name.to_string(), faces } }] }), ..Default::default() }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_group(name: &str) -> ObjDiff {
    ObjDiff { groups: Some(ObjGroupsDiff { removed: vec![name.to_string()], modified: vec![], added: vec![] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_object(index: usize, name: &str, faces: Vec<u64>, existed: bool) -> ObjDiff {
    if existed {
        ObjDiff { objects: Some(ObjObjectsDiff { removed: vec![], modified: vec![ObjGroupModified { name: name.to_string(), diff: ObjGroupDiff { faces: Some(faces) } }], added: vec![] }), ..Default::default() }
    } else {
        ObjDiff { objects: Some(ObjObjectsDiff { removed: vec![], modified: vec![], added: vec![ObjObjectAdded { index, object: ObjObject { name: name.to_string(), faces } }] }), ..Default::default() }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_object(name: &str) -> ObjDiff {
    ObjDiff { objects: Some(ObjObjectsDiff { removed: vec![name.to_string()], modified: vec![], added: vec![] }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_mtllib(mtllib: Option<String>) -> ObjDiff {
    ObjDiff { mtllib: Some(mtllib), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_usemtl(usemtl: Vec<ObjUsemtlRange>) -> ObjDiff {
    ObjDiff { usemtl: Some(usemtl), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_smoothing_groups(smoothing_groups: Vec<ObjSmoothingRange>) -> ObjDiff {
    ObjDiff { smoothing_groups: Some(smoothing_groups), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_unknown_statements(unknown_statements: Vec<ObjUnknownStatement>) -> ObjDiff {
    ObjDiff { unknown_statements: Some(unknown_statements), ..Default::default() }
}
//#endregion 🔖️MutationDiffBuilders

//#region 🔖️DemoCases
/// 🧬️ Canonical "differs in every mutable field" snapshot A — every index-keyed collection
/// has 2 items (a stable prefix item + one that will be modified); every name-keyed
/// collection has 2 named entries (one that will be removed, one that will be modified).
/// Mirrors `🧬️mutations`' own `sweep_a`/`sweep_b` fixtures (kept local to this file, same
/// per-file-independent-fixture convention `stdio.zip`'s own diff/mutations pair uses).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn sweep_a() -> ObjSnapshot {
    ObjSnapshot {
        schema: "stdio.obj".into(),
        vertices: vec![ObjVertex { x: 0.0, y: 0.0, z: 0.0, w: None }, ObjVertex { x: 1.0, y: 1.0, z: 1.0, w: None }],
        texcoords: vec![ObjTexCoord { u: 0.0, v: 0.0, w: None }, ObjTexCoord { u: 1.0, v: 1.0, w: Some(5.0) }],
        normals: vec![ObjNormal { x: 0.0, y: 0.0, z: 1.0 }, ObjNormal { x: 1.0, y: 1.0, z: 1.0 }],
        faces: vec![ObjFace { vertices: vec![ObjFaceVertex { vertex: 0, texcoord: None, normal: None }] }, ObjFace { vertices: vec![ObjFaceVertex { vertex: 0, texcoord: None, normal: None }] }],
        groups: vec![ObjGroup { name: "G1".into(), faces: vec![0] }, ObjGroup { name: "G2".into(), faces: vec![1] }],
        objects: vec![ObjObject { name: "O1".into(), faces: vec![0] }, ObjObject { name: "O2".into(), faces: vec![1] }],
        mtllib: Some("a.mtl".into()),
        usemtl: vec![ObjUsemtlRange { face_index_from: 0, material: "Red".into() }],
        smoothing_groups: vec![ObjSmoothingRange { face_index_from: 0, group: Some(1) }],
        unknown_statements: vec![ObjUnknownStatement { line_index: 0, raw: "# a".into() }],
    }
}

/// 🧬️ Sweep B: every index-keyed collection's index-0 item is UNCHANGED, its index-1 item is
/// MODIFIED in every field (incl. a tri-state `Some(None)` on `texcoords[1].w`), and gains a
/// brand-new item at index 2. Name-keyed `groups`/`objects` show removed+modified+added
/// simultaneously from ONE `between(a,b)` call. `mtllib` exercises `Some->None` tri-state.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn sweep_b() -> ObjSnapshot {
    ObjSnapshot {
        schema: "stdio.obj".into(),
        vertices: vec![ObjVertex { x: 0.0, y: 0.0, z: 0.0, w: None }, ObjVertex { x: 9.0, y: 9.0, z: 9.0, w: Some(0.5) }, ObjVertex { x: 5.0, y: 5.0, z: 5.0, w: Some(1.0) }],
        texcoords: vec![ObjTexCoord { u: 0.0, v: 0.0, w: None }, ObjTexCoord { u: 2.0, v: 2.0, w: None }, ObjTexCoord { u: 5.0, v: 5.0, w: None }],
        normals: vec![ObjNormal { x: 0.0, y: 0.0, z: 1.0 }, ObjNormal { x: -1.0, y: -1.0, z: -1.0 }, ObjNormal { x: 0.0, y: 1.0, z: 0.0 }],
        faces: vec![
            ObjFace { vertices: vec![ObjFaceVertex { vertex: 0, texcoord: None, normal: None }] },
            ObjFace { vertices: vec![ObjFaceVertex { vertex: 1, texcoord: Some(0), normal: Some(0) }] },
            ObjFace { vertices: vec![ObjFaceVertex { vertex: 2, texcoord: None, normal: None }] },
        ],
        groups: vec![ObjGroup { name: "G2".into(), faces: vec![1, 2] }, ObjGroup { name: "G3".into(), faces: vec![3] }],
        objects: vec![ObjObject { name: "O2".into(), faces: vec![1, 2] }, ObjObject { name: "O3".into(), faces: vec![3] }],
        mtllib: None,
        usemtl: vec![ObjUsemtlRange { face_index_from: 0, material: "Blue".into() }, ObjUsemtlRange { face_index_from: 2, material: "Green".into() }],
        smoothing_groups: vec![ObjSmoothingRange { face_index_from: 0, group: None }],
        unknown_statements: vec![ObjUnknownStatement { line_index: 5, raw: "# b".into() }, ObjUnknownStatement { line_index: 6, raw: "weird".into() }],
    }
}

/// 🧪️ P2-FG1: representative `ObjDiff` values (empty, a forward `between`, and its reverse) —
/// exercises every scalar, both tri-states (`mtllib` at the top level, `texcoords[1].w` inside a
/// modified item), and all three collection-triple kinds — index-keyed
/// (`vertices`/`texcoords`/`normals`/`faces`) AND name-keyed (`groups`/`objects`). Single source
/// of truth reused by `diff_codec_text_binary_roundtrip_law` below AND by
/// `../../⚙️engine/🦀️.rs`'s `diff_grammar_conformance_law`/`protocol_walk_law`
/// conformance tests, same convention P2-P1's json/zip pilots established.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<ObjDiff> {
    let a = sweep_a();
    let b = sweep_b();
    vec![ObjDiff::default(), <ObjDiff as DiffAlgebra<ObjSnapshot>>::between(&a, &b), <ObjDiff as DiffAlgebra<ObjSnapshot>>::between(&b, &a)]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::schema::snapshot::ObjFaceVertex;
pub use crate::schema::snapshot::ObjSmoothingRange;
pub use crate::schema::snapshot::ObjUnknownStatement;
pub use crate::schema::snapshot::ObjUsemtlRange;
//#endregion 🔁️Re-exports





#[cfg(test)]
use protocol::{DiffBinary,DiffText};
