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

use std::collections::{BTreeSet, HashMap, HashSet};

use crate::artifacts::obj::schema::snapshot::{ObjFace, ObjFaceVertex, ObjGroup, ObjNormal, ObjObject, ObjSmoothingRange, ObjTexCoord, ObjUnknownStatement, ObjUsemtlRange, ObjVertex};
use crate::artifacts::obj::ObjSnapshot;
use protocol::command::DiffAlgebra;
use protocol::DiffCodec;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region IndexCollectionCore
/// 🧮 Per-item sparse-diff behavior shared by the four flat, position-keyed collections. `Diff`
/// is that item's own PUBLIC sparse-patch type (`ObjVertexDiff`, …).
trait ObjIndexElem: Clone + PartialEq {
    type Diff: Clone + PartialEq;
    async fn diff_is_empty(d: &Self::Diff) -> bool;
    async fn diff_between(a: &Self, b: &Self) -> Self::Diff;
    async fn diff_apply(d: &Self::Diff, item: &mut Self);
    async fn diff_absorb(base: &mut Self::Diff, other: Self::Diff);
}

/// ▶️ Applies a `(removed, modified, added)` triple to a base array — modified on BASE
/// positions first, then removed descending, then added ascending clamped to `min(index,len)`
/// (recipe's normative apply order).
async fn generic_apply<T: ObjIndexElem>(base: &[T], removed: &[usize], modified: &[(usize, T::Diff)], added: &[(usize, T)]) -> Vec<T> {
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
async fn generic_between<T: ObjIndexElem>(base: &[T], other: &[T]) -> (Vec<usize>, Vec<(usize, T::Diff)>, Vec<(usize, T)>) {
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

async fn simulate_labels(labels: Vec<Lbl>, removed: &[usize], added: &[(usize, Lbl)]) -> Vec<Lbl> {
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
async fn generic_absorb_pair<T: ObjIndexElem>(
    d1_removed: &[usize],
    d1_modified: &[(usize, T::Diff)],
    d1_added: &[(usize, T)],
    d2_removed: &[usize],
    d2_modified: &[(usize, T::Diff)],
    d2_added: &[(usize, T)],
) -> (Vec<usize>, Vec<(usize, T::Diff)>, Vec<(usize, T)>) {
    let max_ref =
        d1_removed.iter().copied().chain(d1_modified.iter().map(|(i, _)| *i)).chain(d1_added.iter().map(|(i, _)| *i)).chain(d2_removed.iter().copied()).chain(d2_modified.iter().map(|(i, _)| *i)).chain(d2_added.iter().map(|(i, _)| *i)).max();
    let l1 = max_ref.map(|m| m + 2).unwrap_or(0);

    let base_labels: Vec<Lbl> = (0..l1).map(Lbl::Base).collect();
    let d1_added_lbl: Vec<(usize, Lbl)> = d1_added.iter().enumerate().map(|(j, (idx, _))| (*idx, Lbl::Added1(j))).collect();
    let mut mid_labels = simulate_labels(base_labels, d1_removed, &d1_added_lbl).await;

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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjVertexDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub z: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub w: Option<Option<f64>>,
}

impl ObjIndexElem for ObjVertex {
    type Diff = ObjVertexDiff;
    async fn diff_is_empty(d: &ObjVertexDiff) -> bool {
        d == &ObjVertexDiff::default()
    }
    async fn diff_between(a: &ObjVertex, b: &ObjVertex) -> ObjVertexDiff {
        ObjVertexDiff { x: (a.x != b.x).then_some(b.x), y: (a.y != b.y).then_some(b.y), z: (a.z != b.z).then_some(b.z), w: (a.w != b.w).then_some(b.w) }
    }
    async fn diff_apply(d: &ObjVertexDiff, item: &mut ObjVertex) {
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
    async fn diff_absorb(base: &mut ObjVertexDiff, other: ObjVertexDiff) {
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

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjVertexModified {
    pub index: usize,
    pub diff: ObjVertexDiff,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjVertexAdded {
    pub index: usize,
    pub vertex: ObjVertex,
}
/// 🔺️ Index-keyed removed/modified/added triple over `ObjSnapshot::vertices`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjVerticesDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<ObjVertexModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<ObjVertexAdded>,
}
impl ObjVerticesDiff {
    pub async fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
    async fn apply(&self, base: &[ObjVertex]) -> Vec<ObjVertex> {
        let modified: Vec<(usize, ObjVertexDiff)> = self.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
        let added: Vec<(usize, ObjVertex)> = self.added.iter().map(|a| (a.index, a.vertex.clone())).collect();
        generic_apply(base, &self.removed, &modified, &added).await
    }
    async fn between(base: &[ObjVertex], other: &[ObjVertex]) -> Option<Self> {
        let (removed, modified, added) = generic_between(base, other).await;
        let d = Self { removed, modified: modified.into_iter().map(|(index, diff)| ObjVertexModified { index, diff }).collect(), added: added.into_iter().map(|(index, vertex)| ObjVertexAdded { index, vertex }).collect() };
        if d.is_empty().await {
            None
        } else {
            Some(d)
        }
    }
    async fn absorb(d1: Self, d2: Self) -> Option<Self> {
        let d1m: Vec<(usize, ObjVertexDiff)> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();
        let d1a: Vec<(usize, ObjVertex)> = d1.added.into_iter().map(|a| (a.index, a.vertex)).collect();
        let d2m: Vec<(usize, ObjVertexDiff)> = d2.modified.into_iter().map(|m| (m.index, m.diff)).collect();
        let d2a: Vec<(usize, ObjVertex)> = d2.added.into_iter().map(|a| (a.index, a.vertex)).collect();
        let (removed, modified, added) = generic_absorb_pair::<ObjVertex>(&d1.removed, &d1m, &d1a, &d2.removed, &d2m, &d2a).await;
        let d = Self { removed, modified: modified.into_iter().map(|(index, diff)| ObjVertexModified { index, diff }).collect(), added: added.into_iter().map(|(index, vertex)| ObjVertexAdded { index, vertex }).collect() };
        if d.is_empty().await {
            None
        } else {
            Some(d)
        }
    }
}
//#endregion 🔖️VertexDiff

//#region 🔖️TexCoordDiff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjTexCoordDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub v: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub w: Option<Option<f64>>,
}
impl ObjIndexElem for ObjTexCoord {
    type Diff = ObjTexCoordDiff;
    async fn diff_is_empty(d: &ObjTexCoordDiff) -> bool {
        d == &ObjTexCoordDiff::default()
    }
    async fn diff_between(a: &ObjTexCoord, b: &ObjTexCoord) -> ObjTexCoordDiff {
        ObjTexCoordDiff { u: (a.u != b.u).then_some(b.u), v: (a.v != b.v).then_some(b.v), w: (a.w != b.w).then_some(b.w) }
    }
    async fn diff_apply(d: &ObjTexCoordDiff, item: &mut ObjTexCoord) {
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
    async fn diff_absorb(base: &mut ObjTexCoordDiff, other: ObjTexCoordDiff) {
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjTexCoordModified {
    pub index: usize,
    pub diff: ObjTexCoordDiff,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjTexCoordAdded {
    pub index: usize,
    pub texcoord: ObjTexCoord,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjTexCoordsDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<ObjTexCoordModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<ObjTexCoordAdded>,
}
impl ObjTexCoordsDiff {
    pub async fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
    async fn apply(&self, base: &[ObjTexCoord]) -> Vec<ObjTexCoord> {
        let modified: Vec<(usize, ObjTexCoordDiff)> = self.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
        let added: Vec<(usize, ObjTexCoord)> = self.added.iter().map(|a| (a.index, a.texcoord.clone())).collect();
        generic_apply(base, &self.removed, &modified, &added).await
    }
    async fn between(base: &[ObjTexCoord], other: &[ObjTexCoord]) -> Option<Self> {
        let (removed, modified, added) = generic_between(base, other).await;
        let d = Self { removed, modified: modified.into_iter().map(|(index, diff)| ObjTexCoordModified { index, diff }).collect(), added: added.into_iter().map(|(index, texcoord)| ObjTexCoordAdded { index, texcoord }).collect() };
        if d.is_empty().await {
            None
        } else {
            Some(d)
        }
    }
    async fn absorb(d1: Self, d2: Self) -> Option<Self> {
        let d1m: Vec<(usize, ObjTexCoordDiff)> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();
        let d1a: Vec<(usize, ObjTexCoord)> = d1.added.into_iter().map(|a| (a.index, a.texcoord)).collect();
        let d2m: Vec<(usize, ObjTexCoordDiff)> = d2.modified.into_iter().map(|m| (m.index, m.diff)).collect();
        let d2a: Vec<(usize, ObjTexCoord)> = d2.added.into_iter().map(|a| (a.index, a.texcoord)).collect();
        let (removed, modified, added) = generic_absorb_pair::<ObjTexCoord>(&d1.removed, &d1m, &d1a, &d2.removed, &d2m, &d2a).await;
        let d = Self { removed, modified: modified.into_iter().map(|(index, diff)| ObjTexCoordModified { index, diff }).collect(), added: added.into_iter().map(|(index, texcoord)| ObjTexCoordAdded { index, texcoord }).collect() };
        if d.is_empty().await {
            None
        } else {
            Some(d)
        }
    }
}
//#endregion 🔖️TexCoordDiff

//#region 🔖️NormalDiff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjNormalDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub z: Option<f64>,
}
impl ObjIndexElem for ObjNormal {
    type Diff = ObjNormalDiff;
    async fn diff_is_empty(d: &ObjNormalDiff) -> bool {
        d == &ObjNormalDiff::default()
    }
    async fn diff_between(a: &ObjNormal, b: &ObjNormal) -> ObjNormalDiff {
        ObjNormalDiff { x: (a.x != b.x).then_some(b.x), y: (a.y != b.y).then_some(b.y), z: (a.z != b.z).then_some(b.z) }
    }
    async fn diff_apply(d: &ObjNormalDiff, item: &mut ObjNormal) {
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
    async fn diff_absorb(base: &mut ObjNormalDiff, other: ObjNormalDiff) {
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjNormalModified {
    pub index: usize,
    pub diff: ObjNormalDiff,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjNormalAdded {
    pub index: usize,
    pub normal: ObjNormal,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjNormalsDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<ObjNormalModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<ObjNormalAdded>,
}
impl ObjNormalsDiff {
    pub async fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
    async fn apply(&self, base: &[ObjNormal]) -> Vec<ObjNormal> {
        let modified: Vec<(usize, ObjNormalDiff)> = self.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
        let added: Vec<(usize, ObjNormal)> = self.added.iter().map(|a| (a.index, a.normal.clone())).collect();
        generic_apply(base, &self.removed, &modified, &added).await
    }
    async fn between(base: &[ObjNormal], other: &[ObjNormal]) -> Option<Self> {
        let (removed, modified, added) = generic_between(base, other).await;
        let d = Self { removed, modified: modified.into_iter().map(|(index, diff)| ObjNormalModified { index, diff }).collect(), added: added.into_iter().map(|(index, normal)| ObjNormalAdded { index, normal }).collect() };
        if d.is_empty().await {
            None
        } else {
            Some(d)
        }
    }
    async fn absorb(d1: Self, d2: Self) -> Option<Self> {
        let d1m: Vec<(usize, ObjNormalDiff)> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();
        let d1a: Vec<(usize, ObjNormal)> = d1.added.into_iter().map(|a| (a.index, a.normal)).collect();
        let d2m: Vec<(usize, ObjNormalDiff)> = d2.modified.into_iter().map(|m| (m.index, m.diff)).collect();
        let d2a: Vec<(usize, ObjNormal)> = d2.added.into_iter().map(|a| (a.index, a.normal)).collect();
        let (removed, modified, added) = generic_absorb_pair::<ObjNormal>(&d1.removed, &d1m, &d1a, &d2.removed, &d2m, &d2a).await;
        let d = Self { removed, modified: modified.into_iter().map(|(index, diff)| ObjNormalModified { index, diff }).collect(), added: added.into_iter().map(|(index, normal)| ObjNormalAdded { index, normal }).collect() };
        if d.is_empty().await {
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjFaceDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vertices: Option<Vec<ObjFaceVertex>>,
}
impl ObjIndexElem for ObjFace {
    type Diff = ObjFaceDiff;
    async fn diff_is_empty(d: &ObjFaceDiff) -> bool {
        d == &ObjFaceDiff::default()
    }
    async fn diff_between(a: &ObjFace, b: &ObjFace) -> ObjFaceDiff {
        ObjFaceDiff { vertices: (a.vertices != b.vertices).then(|| b.vertices.clone()) }
    }
    async fn diff_apply(d: &ObjFaceDiff, item: &mut ObjFace) {
        if let Some(v) = &d.vertices {
            item.vertices = v.clone();
        }
    }
    async fn diff_absorb(base: &mut ObjFaceDiff, other: ObjFaceDiff) {
        if other.vertices.is_some() {
            base.vertices = other.vertices;
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjFaceModified {
    pub index: usize,
    pub diff: ObjFaceDiff,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjFaceAdded {
    pub index: usize,
    pub face: ObjFace,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjFacesDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<ObjFaceModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<ObjFaceAdded>,
}
impl ObjFacesDiff {
    pub async fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
    async fn apply(&self, base: &[ObjFace]) -> Vec<ObjFace> {
        let modified: Vec<(usize, ObjFaceDiff)> = self.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
        let added: Vec<(usize, ObjFace)> = self.added.iter().map(|a| (a.index, a.face.clone())).collect();
        generic_apply(base, &self.removed, &modified, &added).await
    }
    async fn between(base: &[ObjFace], other: &[ObjFace]) -> Option<Self> {
        let (removed, modified, added) = generic_between(base, other).await;
        let d = Self { removed, modified: modified.into_iter().map(|(index, diff)| ObjFaceModified { index, diff }).collect(), added: added.into_iter().map(|(index, face)| ObjFaceAdded { index, face }).collect() };
        if d.is_empty().await {
            None
        } else {
            Some(d)
        }
    }
    async fn absorb(d1: Self, d2: Self) -> Option<Self> {
        let d1m: Vec<(usize, ObjFaceDiff)> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();
        let d1a: Vec<(usize, ObjFace)> = d1.added.into_iter().map(|a| (a.index, a.face)).collect();
        let d2m: Vec<(usize, ObjFaceDiff)> = d2.modified.into_iter().map(|m| (m.index, m.diff)).collect();
        let d2a: Vec<(usize, ObjFace)> = d2.added.into_iter().map(|a| (a.index, a.face)).collect();
        let (removed, modified, added) = generic_absorb_pair::<ObjFace>(&d1.removed, &d1m, &d1a, &d2.removed, &d2m, &d2a).await;
        let d = Self { removed, modified: modified.into_iter().map(|(index, diff)| ObjFaceModified { index, diff }).collect(), added: added.into_iter().map(|(index, face)| ObjFaceAdded { index, face }).collect() };
        if d.is_empty().await {
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjGroupDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub faces: Option<Vec<usize>>,
}
async fn group_diff_is_empty(d: &ObjGroupDiff) -> bool {
    d == &ObjGroupDiff::default()
}

/// 🧮 `ObjGroup` and `ObjObject` are structurally identical (`name`/`faces`) but distinct named
/// types per the recipe — this tiny local trait lets `apply_group_diff`/membership helpers work
/// over either without merging the two public types back into one shared type.
trait HasFaces {
    async fn faces_mut(&mut self) -> &mut Vec<usize>;
}
impl HasFaces for ObjGroup {
    async fn faces_mut(&mut self) -> &mut Vec<usize> {
        &mut self.faces
    }
}
impl HasFaces for ObjObject {
    async fn faces_mut(&mut self) -> &mut Vec<usize> {
        &mut self.faces
    }
}

async fn group_between(a_faces: &[usize], b_faces: &[usize]) -> ObjGroupDiff {
    ObjGroupDiff { faces: (a_faces != b_faces).then(|| b_faces.to_vec()) }
}
async fn apply_group_diff<T: HasFaces>(item: &mut T, d: &ObjGroupDiff) {
    if let Some(f) = &d.faces {
        *item.faces_mut() = f.clone();
    }
}
async fn absorb_group_diff(base: &mut ObjGroupDiff, other: ObjGroupDiff) {
    if other.faces.is_some() {
        base.faces = other.faces;
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjGroupModified {
    pub name: String,
    pub diff: ObjGroupDiff,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjGroupAdded {
    pub index: usize,
    pub group: ObjGroup,
}
/// 🔺️ Name-keyed removed/modified/added triple over `ObjSnapshot::groups` (same shape as
/// `stdio.zip`'s `ZipEntriesDiff` — no rename tracking needed here, `g` statements only ever
/// add/remove named groups, never rename one in place).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjGroupsDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<ObjGroupModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<ObjGroupAdded>,
}
impl ObjGroupsDiff {
    pub async fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

/// ▶️ Applies a name-keyed groups/objects triple in place (shared shape; parameterized over the
/// `name`/`faces` accessor pair since `ObjGroup`/`ObjObject` are structurally identical but
/// distinct named types per the recipe).
async fn apply_named_membership<T: Clone>(base: &[T], removed: &[String], modified: &[(String, ObjGroupDiff)], added: &[(usize, T)], name_of: impl Fn(&T) -> &str, patch: impl Fn(&mut T, &ObjGroupDiff)) -> Vec<T> {
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
async fn absorb_named_membership<T: Clone>(
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
    if merged.is_empty().await {
        None
    } else {
        Some(merged)
    }
}
//#endregion 🔖️GroupDiff

//#region 🔖️ObjectsDiff
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjObjectAdded {
    pub index: usize,
    pub object: ObjObject,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjObjectsDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<ObjGroupModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<ObjObjectAdded>,
}
impl ObjObjectsDiff {
    pub async fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}
//#endregion 🔖️ObjectsDiff

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.obj`. `schema` is an identity field and never appears here.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.obj.diff")]
pub struct ObjDiff {
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vertices: Option<ObjVerticesDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub texcoords: Option<ObjTexCoordsDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normals: Option<ObjNormalsDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub faces: Option<ObjFacesDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<ObjGroupsDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub objects: Option<ObjObjectsDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mtllib: Option<Option<String>>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usemtl: Option<Vec<ObjUsemtlRange>>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smoothing_groups: Option<Vec<ObjSmoothingRange>>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unknown_statements: Option<Vec<ObjUnknownStatement>>,
}

async fn validate_indexed_targets(base_len: usize, removed_indices: &[usize], modified_indices: impl IntoIterator<Item = usize>, added_indices: impl IntoIterator<Item = usize>, target: &str) -> MutationApplyResult<()> {
    let mut removed = BTreeSet::new();
    for &index in removed_indices {
        if index >= base_len || !removed.insert(index) {
            return Err(MutationApplyError::new("invalid-remove-index", "removal target must exist exactly once").await.at([target, &index.to_string()]).await);
        }
    }
    let mut modified = BTreeSet::new();
    for index in modified_indices {
        if index >= base_len || removed.contains(&index) || !modified.insert(index) {
            return Err(MutationApplyError::new("invalid-modify-index", "modification target must exist exactly once and remain present").await.at([target, &index.to_string()]).await);
        }
    }
    let mut length = base_len - removed.len();
    let mut additions: Vec<usize> = added_indices.into_iter().collect();
    additions.sort_unstable();
    let mut previous = None;
    for index in additions {
        if index > length || previous == Some(index) {
            return Err(MutationApplyError::new("invalid-add-index", "addition target must be unique and within the evolving sequence").await.at([target, &index.to_string()]).await);
        }
        previous = Some(index);
        length += 1;
    }
    Ok(())
}

async fn validate_named_targets<'a>(
    base_names: impl IntoIterator<Item = &'a str>,
    removed_names: impl IntoIterator<Item = &'a str>,
    modified_names: impl IntoIterator<Item = &'a str>,
    added: impl IntoIterator<Item = (usize, &'a str)>,
    target: &str,
) -> MutationApplyResult<()> {
    let mut base = BTreeSet::new();
    for name in base_names {
        if !base.insert(name) {
            return Err(MutationApplyError::new("duplicate-base-target", "base names must be unique").await.at([target, name]).await);
        }
    }
    let mut removed = BTreeSet::new();
    for name in removed_names {
        if !base.contains(name) || !removed.insert(name) {
            return Err(MutationApplyError::new("invalid-remove-target", "removal target must exist exactly once").await.at([target, name]).await);
        }
    }
    let mut modified = BTreeSet::new();
    for name in modified_names {
        if !base.contains(name) || removed.contains(name) || !modified.insert(name) {
            return Err(MutationApplyError::new("invalid-modify-target", "modification target must exist exactly once and remain present").await.at([target, name]).await);
        }
    }
    let mut length = base.len() - removed.len();
    let mut additions: Vec<(usize, &str)> = added.into_iter().collect();
    additions.sort_by_key(|(index, _)| *index);
    let mut added_names = BTreeSet::new();
    let mut previous = None;
    for (index, name) in additions {
        if base.contains(name) || !added_names.insert(name) || index > length || previous == Some(index) {
            return Err(MutationApplyError::new("invalid-add-target", "addition name and position must be unique and valid").await.at([target, name]).await);
        }
        previous = Some(index);
        length += 1;
    }
    Ok(())
}

async fn validate_obj_diff(diff: &ObjDiff, base: &ObjSnapshot) -> MutationApplyResult<()> {
    if let Some(value) = &diff.vertices {
        validate_indexed_targets(base.vertices.len(), &value.removed, value.modified.iter().map(|entry| entry.index), value.added.iter().map(|entry| entry.index), "vertices").await?;
    }
    if let Some(value) = &diff.texcoords {
        validate_indexed_targets(base.texcoords.len(), &value.removed, value.modified.iter().map(|entry| entry.index), value.added.iter().map(|entry| entry.index), "texcoords").await?;
    }
    if let Some(value) = &diff.normals {
        validate_indexed_targets(base.normals.len(), &value.removed, value.modified.iter().map(|entry| entry.index), value.added.iter().map(|entry| entry.index), "normals").await?;
    }
    if let Some(value) = &diff.faces {
        validate_indexed_targets(base.faces.len(), &value.removed, value.modified.iter().map(|entry| entry.index), value.added.iter().map(|entry| entry.index), "faces").await?;
    }
    if let Some(value) = &diff.groups {
        validate_named_targets(
            base.groups.iter().map(|entry| entry.name.as_str()),
            value.removed.iter().map(String::as_str),
            value.modified.iter().map(|entry| entry.name.as_str()),
            value.added.iter().map(|entry| (entry.index, entry.group.name.as_str())),
            "groups",
        ).await?;
    }
    if let Some(value) = &diff.objects {
        validate_named_targets(
            base.objects.iter().map(|entry| entry.name.as_str()),
            value.removed.iter().map(String::as_str),
            value.modified.iter().map(|entry| entry.name.as_str()),
            value.added.iter().map(|entry| (entry.index, entry.object.name.as_str())),
            "objects",
        ).await?;
    }
    Ok(())
}

async fn apply_obj_diff_unchecked(diff: &ObjDiff, base: &ObjSnapshot) -> ObjSnapshot {
    ObjSnapshot {
        schema: base.schema.clone(),
        vertices: match &diff.vertices {
            Some(d) => d.apply(&base.vertices).await,
            None => base.vertices.clone(),
        },
        texcoords: match &diff.texcoords {
            Some(d) => d.apply(&base.texcoords).await,
            None => base.texcoords.clone(),
        },
        normals: match &diff.normals {
            Some(d) => d.apply(&base.normals).await,
            None => base.normals.clone(),
        },
        faces: match &diff.faces {
            Some(d) => d.apply(&base.faces).await,
            None => base.faces.clone(),
        },
        groups: match &diff.groups {
            Some(d) => {
                let modified: Vec<(String, ObjGroupDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
                let added: Vec<(usize, ObjGroup)> = d.added.iter().map(|a| (a.index, a.group.clone())).collect();
                apply_named_membership(&base.groups, &d.removed, &modified, &added, |g| g.name.as_str(), apply_group_diff).await
            }
            None => base.groups.clone(),
        },
        objects: match &diff.objects {
            Some(d) => {
                let modified: Vec<(String, ObjGroupDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
                let added: Vec<(usize, ObjObject)> = d.added.iter().map(|a| (a.index, a.object.clone())).collect();
                apply_named_membership(&base.objects, &d.removed, &modified, &added, |o| o.name.as_str(), apply_group_diff).await
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
    async fn apply(&self, base: &ObjSnapshot) -> MutationApplyResult<ObjSnapshot> {
        validate_obj_diff(self, base).await?;
        Ok(apply_obj_diff_unchecked(self, base).await)
    }

    /// ➕️ Structural, total, base-free sequential-coalesce (`## Absorb` contract). Index-keyed
    /// collections use the label-simulation transport (`generic_absorb_pair`); name-keyed
    /// collections use `absorb_named_membership`; every other field is LWW.
    async fn absorb(&mut self, other: Self) {
        self.vertices = match (self.vertices.take(), other.vertices) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => ObjVerticesDiff::absorb(a, b).await,
        };
        self.texcoords = match (self.texcoords.take(), other.texcoords) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => ObjTexCoordsDiff::absorb(a, b).await,
        };
        self.normals = match (self.normals.take(), other.normals) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => ObjNormalsDiff::absorb(a, b).await,
        };
        self.faces = match (self.faces.take(), other.faces) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => ObjFacesDiff::absorb(a, b).await,
        };
        self.groups = match (self.groups.take(), other.groups) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => absorb_named_membership(a, b, |g: &ObjGroup| g.name.as_str(), apply_group_diff, |added: &ObjGroupAdded| added.group.clone(), |index, group| ObjGroupAdded { index, group }).await,
        };
        self.objects = match (self.objects.take(), other.objects) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => {
                let a2 = ObjGroupsDiff { removed: a.removed, modified: a.modified, added: a.added.into_iter().map(|x| ObjGroupAdded { index: x.index, group: ObjGroup { name: x.object.name, faces: x.object.faces } }).collect() };
                let b2 = ObjGroupsDiff { removed: b.removed, modified: b.modified, added: b.added.into_iter().map(|x| ObjGroupAdded { index: x.index, group: ObjGroup { name: x.object.name, faces: x.object.faces } }).collect() };
                absorb_named_membership(a2, b2, |g: &ObjGroup| g.name.as_str(), apply_group_diff, |added: &ObjGroupAdded| added.group.clone(), |index, group| ObjGroupAdded { index, group }).await.map(|merged| ObjObjectsDiff {
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
    async fn inverse(&self, base: &ObjSnapshot) -> Self {
        let mutated = apply_obj_diff_unchecked(self, base);
        Self::between(&mutated, base).await
    }

    async fn between(base: &ObjSnapshot, other: &ObjSnapshot) -> Self {
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
                        modified.push(ObjGroupModified { name: bg.name.clone(), diff: d.await });
                    }
                }
            }
            let added: Vec<ObjGroupAdded> = other.groups.iter().enumerate().filter(|(_, g)| !base_names.contains(g.name.as_str())).map(|(index, g)| ObjGroupAdded { index, group: g.clone() }).collect();
            let d = ObjGroupsDiff { removed, modified, added };
            if d.is_empty().await {
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
                        modified.push(ObjGroupModified { name: bo.name.clone(), diff: d.await });
                    }
                }
            }
            let added: Vec<ObjObjectAdded> = other.objects.iter().enumerate().filter(|(_, o)| !base_names.contains(o.name.as_str())).map(|(index, o)| ObjObjectAdded { index, object: o.clone() }).collect();
            let d = ObjObjectsDiff { removed, modified, added };
            if d.is_empty().await {
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

    async fn is_empty(&self) -> bool {
        self.vertices.as_ref().map_or(true, ObjVerticesDiff::is_empty)
            && self.texcoords.as_ref().map_or(true, ObjTexCoordsDiff::is_empty)
            && self.normals.as_ref().map_or(true, ObjNormalsDiff::is_empty)
            && self.faces.as_ref().map_or(true, ObjFacesDiff::is_empty)
            && self.groups.as_ref().map_or(true, ObjGroupsDiff::is_empty)
            && self.objects.as_ref().map_or(true, ObjObjectsDiff::is_empty)
            && self.mtllib.is_none()
            && self.usemtl.is_none()
            && self.smoothing_groups.is_none()
            && self.unknown_statements.is_none()
    }
}

/// 🧩 `SetSnapshot`'s diff is the sparse field-by-field `between(base, next)` — no full-replace
/// slot exists on `ObjDiff` to short-circuit into.
pub async fn diff_set_snapshot(base: &ObjSnapshot, next: &ObjSnapshot) -> ObjDiff {
    ObjDiff::between(base, next).await
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
async fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
async fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
async fn hex_encode_str(s: &str) -> String {
    hex_encode(s.as_bytes()).await
}
async fn hex_decode_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s).await?).map_err(|e| e.to_string())
}
async fn fmt_f64(v: f64) -> String {
    v.to_string()
}
async fn parse_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}
async fn parse_u32(s: &str) -> Result<u32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
async fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

/// 🧭️ Bracket-depth-aware split (tracks `[`/`]` only): a top-level `sep` inside nested brackets is
/// never mistaken for a field separator — the whole hand-rolled grammar's parsing primitive.
async fn split_top_level(s: &str, sep: char) -> Vec<&str> {
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
async fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}
async fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
async fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s).await?;
    match split_top_level(inner, ',').await.as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}
//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs
async fn enc_vertex(v: &ObjVertex) -> String {
    format!("[{},{},{},{}]", fmt_f64(v.x), fmt_f64(v.y), fmt_f64(v.z), encode_option(&v.w, |w| fmt_f64(*w)))
}
async fn dec_vertex(s: &str) -> Result<ObjVertex, String> {
    let parts = split_top_level(strip_brackets(s).await?, ',').await;
    let [x, y, z, w] = parts.as_slice() else { return Err(format!("vertex: expected 4 fields, got {}", parts.len())) };
    Ok(ObjVertex { x: parse_f64(x).await?, y: parse_f64(y).await?, z: parse_f64(z).await?, w: decode_option(w, parse_f64).await? })
}
async fn enc_texcoord(t: &ObjTexCoord) -> String {
    format!("[{},{},{}]", fmt_f64(t.u), fmt_f64(t.v), encode_option(&t.w, |w| fmt_f64(*w)))
}
async fn dec_texcoord(s: &str) -> Result<ObjTexCoord, String> {
    let parts = split_top_level(strip_brackets(s).await?, ',').await;
    let [u, v, w] = parts.as_slice() else { return Err(format!("texcoord: expected 3 fields, got {}", parts.len())) };
    Ok(ObjTexCoord { u: parse_f64(u).await?, v: parse_f64(v).await?, w: decode_option(w, parse_f64).await? })
}
async fn enc_normal(n: &ObjNormal) -> String {
    format!("[{},{},{}]", fmt_f64(n.x), fmt_f64(n.y), fmt_f64(n.z))
}
async fn dec_normal(s: &str) -> Result<ObjNormal, String> {
    let parts = split_top_level(strip_brackets(s).await?, ',').await;
    let [x, y, z] = parts.as_slice() else { return Err(format!("normal: expected 3 fields, got {}", parts.len())) };
    Ok(ObjNormal { x: parse_f64(x).await?, y: parse_f64(y).await?, z: parse_f64(z).await? })
}
async fn enc_face_vertex(fv: &ObjFaceVertex) -> String {
    format!("[{},{},{}]", fv.vertex, encode_option(&fv.texcoord, |v| v.to_string()), encode_option(&fv.normal, |v| v.to_string()))
}
async fn dec_face_vertex(s: &str) -> Result<ObjFaceVertex, String> {
    let parts = split_top_level(strip_brackets(s).await?, ',').await;
    let [vertex, texcoord, normal] = parts.as_slice() else { return Err(format!("face vertex: expected 3 fields, got {}", parts.len())) };
    Ok(ObjFaceVertex { vertex: parse_u32(vertex).await?, texcoord: decode_option(texcoord, parse_u32).await?, normal: decode_option(normal, parse_u32).await? })
}
async fn enc_face(f: &ObjFace) -> String {
    format!("[{}]", f.vertices.iter().map(enc_face_vertex).collect::<Vec<_>>().join(","))
}
async fn dec_face(s: &str) -> Result<ObjFace, String> {
    let inner = strip_brackets(s).await?;
    let vertices = split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_face_vertex).collect::<Result<Vec<_>, String>>()?;
    Ok(ObjFace { vertices })
}
async fn enc_group(g: &ObjGroup) -> String {
    format!("[{},[{}]]", hex_encode_str(&g.name), g.faces.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","))
}
async fn dec_group(s: &str) -> Result<ObjGroup, String> {
    let parts = split_top_level(strip_brackets(s).await?, ',').await;
    let [name_hex, faces_s] = parts.as_slice() else { return Err(format!("group: expected 2 fields, got {}", parts.len())) };
    let faces = split_top_level(strip_brackets(faces_s).await?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    Ok(ObjGroup { name: hex_decode_str(name_hex).await?, faces })
}
async fn enc_object(o: &ObjObject) -> String {
    format!("[{},[{}]]", hex_encode_str(&o.name), o.faces.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","))
}
async fn dec_object(s: &str) -> Result<ObjObject, String> {
    let parts = split_top_level(strip_brackets(s).await?, ',').await;
    let [name_hex, faces_s] = parts.as_slice() else { return Err(format!("object: expected 2 fields, got {}", parts.len())) };
    let faces = split_top_level(strip_brackets(faces_s).await?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    Ok(ObjObject { name: hex_decode_str(name_hex).await?, faces })
}
async fn enc_usemtl(u: &ObjUsemtlRange) -> String {
    format!("[{},{}]", u.face_index_from, hex_encode_str(&u.material))
}
async fn dec_usemtl(s: &str) -> Result<ObjUsemtlRange, String> {
    let parts = split_top_level(strip_brackets(s).await?, ',').await;
    let [idx, mat] = parts.as_slice() else { return Err(format!("usemtl: expected 2 fields, got {}", parts.len())) };
    Ok(ObjUsemtlRange { face_index_from: parse_usize(idx).await?, material: hex_decode_str(mat).await? })
}
async fn enc_smoothing(sg: &ObjSmoothingRange) -> String {
    format!("[{},{}]", sg.face_index_from, encode_option(&sg.group, |g| g.to_string()))
}
async fn dec_smoothing(s: &str) -> Result<ObjSmoothingRange, String> {
    let parts = split_top_level(strip_brackets(s).await?, ',').await;
    let [idx, grp] = parts.as_slice() else { return Err(format!("smoothing: expected 2 fields, got {}", parts.len())) };
    Ok(ObjSmoothingRange { face_index_from: parse_usize(idx).await?, group: decode_option(grp, parse_u32).await? })
}
async fn enc_unknown(u: &ObjUnknownStatement) -> String {
    format!("[{},{}]", u.line_index, hex_encode_str(&u.raw))
}
async fn dec_unknown(s: &str) -> Result<ObjUnknownStatement, String> {
    let parts = split_top_level(strip_brackets(s).await?, ',').await;
    let [idx, raw] = parts.as_slice() else { return Err(format!("unknown: expected 2 fields, got {}", parts.len())) };
    Ok(ObjUnknownStatement { line_index: parse_usize(idx).await?, raw: hex_decode_str(raw).await? })
}
//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs
async fn enc_vertex_diff(d: &ObjVertexDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = d.x {
        parts.push(format!("X:{}", fmt_f64(v)));
    }
    if let Some(v) = d.y {
        parts.push(format!("Y:{}", fmt_f64(v)));
    }
    if let Some(v) = d.z {
        parts.push(format!("Z:{}", fmt_f64(v)));
    }
    if let Some(v) = d.w {
        parts.push(format!("W:{}", encode_option(&v, |w| fmt_f64(*w))));
    }
    format!("[{}]", parts.join(","))
}
async fn dec_vertex_diff(s: &str) -> Result<ObjVertexDiff, String> {
    let inner = strip_brackets(s).await?;
    let mut d = ObjVertexDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("vertex diff: bad entry {entry:?}"))?;
        match tag {
            "X" => d.x = Some(parse_f64(val).await?),
            "Y" => d.y = Some(parse_f64(val).await?),
            "Z" => d.z = Some(parse_f64(val).await?),
            "W" => d.w = Some(decode_option(val, parse_f64).await?),
            other => return Err(format!("vertex diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}
async fn enc_texcoord_diff(d: &ObjTexCoordDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = d.u {
        parts.push(format!("U:{}", fmt_f64(v)));
    }
    if let Some(v) = d.v {
        parts.push(format!("V:{}", fmt_f64(v)));
    }
    if let Some(v) = d.w {
        parts.push(format!("W:{}", encode_option(&v, |w| fmt_f64(*w))));
    }
    format!("[{}]", parts.join(","))
}
async fn dec_texcoord_diff(s: &str) -> Result<ObjTexCoordDiff, String> {
    let inner = strip_brackets(s).await?;
    let mut d = ObjTexCoordDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("texcoord diff: bad entry {entry:?}"))?;
        match tag {
            "U" => d.u = Some(parse_f64(val).await?),
            "V" => d.v = Some(parse_f64(val).await?),
            "W" => d.w = Some(decode_option(val, parse_f64).await?),
            other => return Err(format!("texcoord diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}
async fn enc_normal_diff(d: &ObjNormalDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = d.x {
        parts.push(format!("X:{}", fmt_f64(v)));
    }
    if let Some(v) = d.y {
        parts.push(format!("Y:{}", fmt_f64(v)));
    }
    if let Some(v) = d.z {
        parts.push(format!("Z:{}", fmt_f64(v)));
    }
    format!("[{}]", parts.join(","))
}
async fn dec_normal_diff(s: &str) -> Result<ObjNormalDiff, String> {
    let inner = strip_brackets(s).await?;
    let mut d = ObjNormalDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("normal diff: bad entry {entry:?}"))?;
        match tag {
            "X" => d.x = Some(parse_f64(val).await?),
            "Y" => d.y = Some(parse_f64(val).await?),
            "Z" => d.z = Some(parse_f64(val).await?),
            other => return Err(format!("normal diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}
async fn enc_face_diff(d: &ObjFaceDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = &d.vertices {
        parts.push(format!("V:[{}]", v.iter().map(enc_face_vertex).collect::<Vec<_>>().join(",")));
    }
    format!("[{}]", parts.join(","))
}
async fn dec_face_diff(s: &str) -> Result<ObjFaceDiff, String> {
    let inner = strip_brackets(s).await?;
    let mut d = ObjFaceDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("face diff: bad entry {entry:?}"))?;
        match tag {
            "V" => {
                let items = split_top_level(strip_brackets(val).await?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_face_vertex).collect::<Result<Vec<_>, String>>()?;
                d.vertices = Some(items);
            }
            other => return Err(format!("face diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}
async fn enc_group_diff(d: &ObjGroupDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = &d.faces {
        parts.push(format!("F:[{}]", v.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(",")));
    }
    format!("[{}]", parts.join(","))
}
async fn dec_group_diff(s: &str) -> Result<ObjGroupDiff, String> {
    let inner = strip_brackets(s).await?;
    let mut d = ObjGroupDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("group diff: bad entry {entry:?}"))?;
        match tag {
            "F" => {
                let faces = split_top_level(strip_brackets(val).await?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
                d.faces = Some(faces);
            }
            other => return Err(format!("group diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}
//#endregion 🔖️DiffValueCodecs

//#region 🔖️CollectionCodecs
/// 🧭️ Generic-shaped 3-section `[removed];[modified];[added]` index-keyed collection-triple
/// printer/parser (mirrors gif89a's `enc_collection_triple`/`dec_collection_triple`), hand-
/// instantiated per item type below.
async fn enc_index_triple(name: &str, removed: &[usize], modified: &[(usize, String)], added: &[(usize, String)]) -> String {
    let removed = removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = modified.iter().map(|(i, v)| format!("{i}:{v}")).collect::<Vec<_>>().join(",");
    let added = added.iter().map(|(i, v)| format!("{i}:{v}")).collect::<Vec<_>>().join(",");
    format!("{name}{{[{removed}];[{modified}];[{added}]}}")
}
async fn dec_index_triple(body: &str) -> Result<(Vec<usize>, Vec<(usize, String)>, Vec<(usize, String)>), String> {
    let three = split_top_level(body, ';').await;
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("collection: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s).await?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
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

/// 🧭️ Same shape, name-keyed (`removed: Vec<String>`) — for `groups`/`objects`.
async fn enc_named_triple(name: &str, removed: &[String], modified: &[(String, String)], added: &[(usize, String)]) -> String {
    let removed = removed.iter().map(|n| hex_encode_str(n)).collect::<Vec<_>>().join(",");
    let modified = modified.iter().map(|(n, v)| format!("{}:{v}", hex_encode_str(n))).collect::<Vec<_>>().join(",");
    let added = added.iter().map(|(i, v)| format!("{i}:{v}")).collect::<Vec<_>>().join(",");
    format!("{name}{{[{removed}];[{modified}];[{added}]}}")
}
async fn dec_named_triple(body: &str) -> Result<(Vec<String>, Vec<(String, String)>, Vec<(usize, String)>), String> {
    let three = split_top_level(body, ';').await;
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("named collection: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s).await?, ',').into_iter().filter(|s| !s.is_empty()).map(hex_decode_str).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s).await?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (name_hex, rest) = entry.split_once(':').ok_or_else(|| format!("named collection modified: bad entry {entry:?}"))?;
            Ok((hex_decode_str(name_hex)?, rest.to_string()))
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s).await?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("named collection added: bad entry {entry:?}"))?;
            Ok((parse_usize(idx)?, rest.to_string()))
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok((removed, modified, added))
}

async fn enc_vertices_diff(d: &ObjVerticesDiff) -> String {
    enc_index_triple("vertices", &d.removed, &d.modified.iter().map(|m| (m.index, enc_vertex_diff(&m.diff))).collect::<Vec<_>>(), &d.added.iter().map(|a| (a.index, enc_vertex(&a.vertex))).collect::<Vec<_>>()).await
}
async fn dec_vertices_diff(body: &str) -> Result<ObjVerticesDiff, String> {
    let (removed, modified, added) = dec_index_triple(body).await?;
    Ok(ObjVerticesDiff {
        removed,
        modified: modified.into_iter().map(|(index, enc)| Ok(ObjVertexModified { index, diff: dec_vertex_diff(&enc)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, enc)| Ok(ObjVertexAdded { index, vertex: dec_vertex(&enc)? })).collect::<Result<Vec<_>, String>>()?,
    })
}
async fn enc_texcoords_diff(d: &ObjTexCoordsDiff) -> String {
    enc_index_triple("texcoords", &d.removed, &d.modified.iter().map(|m| (m.index, enc_texcoord_diff(&m.diff))).collect::<Vec<_>>(), &d.added.iter().map(|a| (a.index, enc_texcoord(&a.texcoord))).collect::<Vec<_>>()).await
}
async fn dec_texcoords_diff(body: &str) -> Result<ObjTexCoordsDiff, String> {
    let (removed, modified, added) = dec_index_triple(body).await?;
    Ok(ObjTexCoordsDiff {
        removed,
        modified: modified.into_iter().map(|(index, enc)| Ok(ObjTexCoordModified { index, diff: dec_texcoord_diff(&enc)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, enc)| Ok(ObjTexCoordAdded { index, texcoord: dec_texcoord(&enc)? })).collect::<Result<Vec<_>, String>>()?,
    })
}
async fn enc_normals_diff(d: &ObjNormalsDiff) -> String {
    enc_index_triple("normals", &d.removed, &d.modified.iter().map(|m| (m.index, enc_normal_diff(&m.diff))).collect::<Vec<_>>(), &d.added.iter().map(|a| (a.index, enc_normal(&a.normal))).collect::<Vec<_>>()).await
}
async fn dec_normals_diff(body: &str) -> Result<ObjNormalsDiff, String> {
    let (removed, modified, added) = dec_index_triple(body).await?;
    Ok(ObjNormalsDiff {
        removed,
        modified: modified.into_iter().map(|(index, enc)| Ok(ObjNormalModified { index, diff: dec_normal_diff(&enc)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, enc)| Ok(ObjNormalAdded { index, normal: dec_normal(&enc)? })).collect::<Result<Vec<_>, String>>()?,
    })
}
async fn enc_faces_diff(d: &ObjFacesDiff) -> String {
    enc_index_triple("faces", &d.removed, &d.modified.iter().map(|m| (m.index, enc_face_diff(&m.diff))).collect::<Vec<_>>(), &d.added.iter().map(|a| (a.index, enc_face(&a.face))).collect::<Vec<_>>()).await
}
async fn dec_faces_diff(body: &str) -> Result<ObjFacesDiff, String> {
    let (removed, modified, added) = dec_index_triple(body).await?;
    Ok(ObjFacesDiff {
        removed,
        modified: modified.into_iter().map(|(index, enc)| Ok(ObjFaceModified { index, diff: dec_face_diff(&enc)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, enc)| Ok(ObjFaceAdded { index, face: dec_face(&enc)? })).collect::<Result<Vec<_>, String>>()?,
    })
}
async fn enc_groups_diff(d: &ObjGroupsDiff) -> String {
    enc_named_triple("groups", &d.removed, &d.modified.iter().map(|m| (m.name.clone(), enc_group_diff(&m.diff))).collect::<Vec<_>>(), &d.added.iter().map(|a| (a.index, enc_group(&a.group))).collect::<Vec<_>>()).await
}
async fn dec_groups_diff(body: &str) -> Result<ObjGroupsDiff, String> {
    let (removed, modified, added) = dec_named_triple(body).await?;
    Ok(ObjGroupsDiff {
        removed,
        modified: modified.into_iter().map(|(name, enc)| Ok(ObjGroupModified { name, diff: dec_group_diff(&enc)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, enc)| Ok(ObjGroupAdded { index, group: dec_group(&enc)? })).collect::<Result<Vec<_>, String>>()?,
    })
}
async fn enc_objects_diff(d: &ObjObjectsDiff) -> String {
    enc_named_triple("objects", &d.removed, &d.modified.iter().map(|m| (m.name.clone(), enc_group_diff(&m.diff))).collect::<Vec<_>>(), &d.added.iter().map(|a| (a.index, enc_object(&a.object))).collect::<Vec<_>>()).await
}
async fn dec_objects_diff(body: &str) -> Result<ObjObjectsDiff, String> {
    let (removed, modified, added) = dec_named_triple(body).await?;
    Ok(ObjObjectsDiff {
        removed,
        modified: modified.into_iter().map(|(name, enc)| Ok(ObjGroupModified { name, diff: dec_group_diff(&enc)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, enc)| Ok(ObjObjectAdded { index, object: dec_object(&enc)? })).collect::<Result<Vec<_>, String>>()?,
    })
}
//#endregion 🔖️CollectionCodecs

//#region 🔖️BinaryPrimitives
/// 🧪️ P2-FG1: real LEB128-varint-framed binary primitives (length-prefixed strings, fixed-8-byte
/// `f64` little-endian, varint-encoded `u32`/`usize`, tag-byte `Option<T>`/tri-state
/// `Option<Option<T>>` wrappers) backing the upgraded `DiffCodec::encode_diff`/`decode_diff` frame
/// below — mirrors dxf/md's own `#region 🔖️BinaryPrimitives`, reusing
/// `store::pack_rt::write_varint_u64`/`store::ByteReader` rather than reinventing varint encode/
/// decode. `obj`'s whole diff tree is flat structs/`Vec`/`Option<T>` (module doc comment, §3b) — no
/// `Prim::Ref`-recursion gap applies to any SINGLE value here, only to the collection-triple SHAPE
/// itself (see the `.protocol.semio` sibling's comment), so every value below gets a full,
/// genuinely field-by-field binary frame, never an opaque byte-chain.
async fn write_f64_bin(out: &mut Vec<u8>, v: f64) {
    out.extend_from_slice(&v.to_le_bytes());
}
async fn read_f64_bin(reader: &mut store::ByteReader<'_>) -> Result<f64, String> {
    reader.read_f64_le().await.map_err(|e| e.to_string())
}
async fn write_str_bin(out: &mut Vec<u8>, s: &str) {
    store::pack_rt::write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}
async fn read_str_bin(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    let len = reader.read_varint_u64().await.map_err(|e| e.to_string())? as usize;
    String::from_utf8(reader.read_bytes(len).await.map_err(|e| e.to_string())?.to_vec()).map_err(|e| e.to_string())
}
async fn write_u32_bin(out: &mut Vec<u8>, v: u32) {
    store::pack_rt::write_varint_u64(out, v as u64);
}
async fn read_u32_bin(reader: &mut store::ByteReader<'_>) -> Result<u32, String> {
    Ok(reader.read_varint_u64().await.map_err(|e| e.to_string())? as u32)
}
async fn write_usize_bin(out: &mut Vec<u8>, v: usize) {
    store::pack_rt::write_varint_u64(out, v as u64);
}
async fn read_usize_bin(reader: &mut store::ByteReader<'_>) -> Result<usize, String> {
    Ok(reader.read_varint_u64().await.map_err(|e| e.to_string())? as usize)
}
async fn write_option_bin<T>(out: &mut Vec<u8>, opt: &Option<T>, enc: impl Fn(&T, &mut Vec<u8>)) {
    match opt {
        None => out.push(0),
        Some(v) => {
            out.push(1);
            enc(v, out);
        }
    }
}
async fn read_option_bin<T>(reader: &mut store::ByteReader<'_>, dec: impl Fn(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<Option<T>, String> {
    match reader.read_u8().await.map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(dec(reader)?)),
        other => Err(format!("option binary: unknown tag {other}")),
    }
}
/// 🏳️ Tri-state `Option<Option<T>>` binary wrapper (`ObjVertexDiff::w`/`ObjTexCoordDiff::w`'s own
/// diff field, NOT `ObjDiff::mtllib` — that top-level field's OUTER `Option` layer is already
/// carried by `encode_diff`'s presence flags, so its payload only needs [`write_option_bin`] over
/// the remaining `Option<String>`) — `0`=unchanged (`None`), `1`=cleared (`Some(None)`),
/// `2`=set (`Some(Some(v))`).
async fn write_tristate_bin<T>(out: &mut Vec<u8>, opt: &Option<Option<T>>, enc: impl Fn(&T, &mut Vec<u8>)) {
    match opt {
        None => out.push(0),
        Some(None) => out.push(1),
        Some(Some(v)) => {
            out.push(2);
            enc(v, out);
        }
    }
}
async fn read_tristate_bin<T>(reader: &mut store::ByteReader<'_>, dec: impl Fn(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<Option<Option<T>>, String> {
    match reader.read_u8().await.map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(None)),
        2 => Ok(Some(Some(dec(reader)?))),
        other => Err(format!("tristate binary: unknown tag {other}")),
    }
}
async fn write_vec_bin<T>(out: &mut Vec<u8>, items: &[T], enc: impl Fn(&T, &mut Vec<u8>)) {
    store::pack_rt::write_varint_u64(out, items.len() as u64);
    for item in items {
        enc(item, out);
    }
}
async fn read_vec_bin<T>(reader: &mut store::ByteReader<'_>, dec: impl Fn(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<Vec<T>, String> {
    let count = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(dec(reader)?);
    }
    Ok(out)
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️ValueBinaryCodecs
/// 🧪️ P2-FG1: real field-by-field binary twins of `#region 🔖️ValueCodecs` above.
async fn enc_vertex_bin(v: &ObjVertex, out: &mut Vec<u8>) {
    write_f64_bin(out, v.x);
    write_f64_bin(out, v.y);
    write_f64_bin(out, v.z);
    write_option_bin(out, &v.w, |w, o| write_f64_bin(o, *w));
}
async fn dec_vertex_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjVertex, String> {
    let x = read_f64_bin(reader).await?;
    let y = read_f64_bin(reader).await?;
    let z = read_f64_bin(reader).await?;
    let w = read_option_bin(reader, read_f64_bin).await?;
    Ok(ObjVertex { x, y, z, w })
}
async fn enc_texcoord_bin(t: &ObjTexCoord, out: &mut Vec<u8>) {
    write_f64_bin(out, t.u);
    write_f64_bin(out, t.v);
    write_option_bin(out, &t.w, |w, o| write_f64_bin(o, *w));
}
async fn dec_texcoord_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjTexCoord, String> {
    let u = read_f64_bin(reader).await?;
    let v = read_f64_bin(reader).await?;
    let w = read_option_bin(reader, read_f64_bin).await?;
    Ok(ObjTexCoord { u, v, w })
}
async fn enc_normal_bin(n: &ObjNormal, out: &mut Vec<u8>) {
    write_f64_bin(out, n.x);
    write_f64_bin(out, n.y);
    write_f64_bin(out, n.z);
}
async fn dec_normal_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjNormal, String> {
    let x = read_f64_bin(reader).await?;
    let y = read_f64_bin(reader).await?;
    let z = read_f64_bin(reader).await?;
    Ok(ObjNormal { x, y, z })
}
async fn enc_face_vertex_bin(fv: &ObjFaceVertex, out: &mut Vec<u8>) {
    write_u32_bin(out, fv.vertex);
    write_option_bin(out, &fv.texcoord, |v, o| write_u32_bin(o, *v));
    write_option_bin(out, &fv.normal, |v, o| write_u32_bin(o, *v));
}
async fn dec_face_vertex_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjFaceVertex, String> {
    let vertex = read_u32_bin(reader).await?;
    let texcoord = read_option_bin(reader, read_u32_bin).await?;
    let normal = read_option_bin(reader, read_u32_bin).await?;
    Ok(ObjFaceVertex { vertex, texcoord, normal })
}
async fn enc_face_bin(f: &ObjFace, out: &mut Vec<u8>) {
    write_vec_bin(out, &f.vertices, enc_face_vertex_bin);
}
async fn dec_face_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjFace, String> {
    Ok(ObjFace { vertices: read_vec_bin(reader, dec_face_vertex_bin).await? })
}
async fn enc_group_bin(g: &ObjGroup, out: &mut Vec<u8>) {
    write_str_bin(out, &g.name);
    write_vec_bin(out, &g.faces, |f, o| write_usize_bin(o, *f));
}
async fn dec_group_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjGroup, String> {
    let name = read_str_bin(reader).await?;
    let faces = read_vec_bin(reader, read_usize_bin).await?;
    Ok(ObjGroup { name, faces })
}
async fn enc_object_bin(o: &ObjObject, out: &mut Vec<u8>) {
    write_str_bin(out, &o.name);
    write_vec_bin(out, &o.faces, |f, out| write_usize_bin(out, *f));
}
async fn dec_object_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjObject, String> {
    let name = read_str_bin(reader).await?;
    let faces = read_vec_bin(reader, read_usize_bin).await?;
    Ok(ObjObject { name, faces })
}
async fn enc_usemtl_bin(u: &ObjUsemtlRange, out: &mut Vec<u8>) {
    write_usize_bin(out, u.face_index_from);
    write_str_bin(out, &u.material);
}
async fn dec_usemtl_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjUsemtlRange, String> {
    let face_index_from = read_usize_bin(reader).await?;
    let material = read_str_bin(reader).await?;
    Ok(ObjUsemtlRange { face_index_from, material })
}
async fn enc_smoothing_bin(sg: &ObjSmoothingRange, out: &mut Vec<u8>) {
    write_usize_bin(out, sg.face_index_from);
    write_option_bin(out, &sg.group, |g, o| write_u32_bin(o, *g));
}
async fn dec_smoothing_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjSmoothingRange, String> {
    let face_index_from = read_usize_bin(reader).await?;
    let group = read_option_bin(reader, read_u32_bin).await?;
    Ok(ObjSmoothingRange { face_index_from, group })
}
async fn enc_unknown_bin(u: &ObjUnknownStatement, out: &mut Vec<u8>) {
    write_usize_bin(out, u.line_index);
    write_str_bin(out, &u.raw);
}
async fn dec_unknown_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjUnknownStatement, String> {
    let line_index = read_usize_bin(reader).await?;
    let raw = read_str_bin(reader).await?;
    Ok(ObjUnknownStatement { line_index, raw })
}
//#endregion 🔖️ValueBinaryCodecs

//#region 🔖️DiffValueBinaryCodecs
/// 🧪️ P2-FG1: real field-by-field binary twins of `#region 🔖️DiffValueCodecs` above — each
/// per-item sparse patch encodes its fields in fixed declaration order via
/// [`write_option_bin`]/[`write_tristate_bin`] (no tag byte needed per field, unlike an enum
/// variant: the field ORDER itself is the schema, exactly [`enc_vertex_diff_bin`]'s shape
/// mirroring md's own `MdBlockDiff::Heading` arm).
async fn enc_vertex_diff_bin(d: &ObjVertexDiff, out: &mut Vec<u8>) {
    write_option_bin(out, &d.x, |v, o| write_f64_bin(o, *v));
    write_option_bin(out, &d.y, |v, o| write_f64_bin(o, *v));
    write_option_bin(out, &d.z, |v, o| write_f64_bin(o, *v));
    write_tristate_bin(out, &d.w, |v, o| write_f64_bin(o, *v));
}
async fn dec_vertex_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjVertexDiff, String> {
    let x = read_option_bin(reader, read_f64_bin).await?;
    let y = read_option_bin(reader, read_f64_bin).await?;
    let z = read_option_bin(reader, read_f64_bin).await?;
    let w = read_tristate_bin(reader, read_f64_bin).await?;
    Ok(ObjVertexDiff { x, y, z, w })
}
async fn enc_texcoord_diff_bin(d: &ObjTexCoordDiff, out: &mut Vec<u8>) {
    write_option_bin(out, &d.u, |v, o| write_f64_bin(o, *v));
    write_option_bin(out, &d.v, |v, o| write_f64_bin(o, *v));
    write_tristate_bin(out, &d.w, |v, o| write_f64_bin(o, *v));
}
async fn dec_texcoord_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjTexCoordDiff, String> {
    let u = read_option_bin(reader, read_f64_bin).await?;
    let v = read_option_bin(reader, read_f64_bin).await?;
    let w = read_tristate_bin(reader, read_f64_bin).await?;
    Ok(ObjTexCoordDiff { u, v, w })
}
async fn enc_normal_diff_bin(d: &ObjNormalDiff, out: &mut Vec<u8>) {
    write_option_bin(out, &d.x, |v, o| write_f64_bin(o, *v));
    write_option_bin(out, &d.y, |v, o| write_f64_bin(o, *v));
    write_option_bin(out, &d.z, |v, o| write_f64_bin(o, *v));
}
async fn dec_normal_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjNormalDiff, String> {
    let x = read_option_bin(reader, read_f64_bin).await?;
    let y = read_option_bin(reader, read_f64_bin).await?;
    let z = read_option_bin(reader, read_f64_bin).await?;
    Ok(ObjNormalDiff { x, y, z })
}
async fn enc_face_diff_bin(d: &ObjFaceDiff, out: &mut Vec<u8>) {
    write_option_bin(out, &d.vertices, |v, o| write_vec_bin(o, v, enc_face_vertex_bin));
}
async fn dec_face_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjFaceDiff, String> {
    let vertices = read_option_bin(reader, |r| read_vec_bin(r, dec_face_vertex_bin)).await?;
    Ok(ObjFaceDiff { vertices })
}
async fn enc_group_diff_bin(d: &ObjGroupDiff, out: &mut Vec<u8>) {
    write_option_bin(out, &d.faces, |v, o| write_vec_bin(o, v, |f, oo| write_usize_bin(oo, *f)));
}
async fn dec_group_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjGroupDiff, String> {
    let faces = read_option_bin(reader, |r| read_vec_bin(r, read_usize_bin)).await?;
    Ok(ObjGroupDiff { faces })
}
//#endregion 🔖️DiffValueBinaryCodecs

//#region 🔖️CollectionBinaryCodecs
/// 🧭️ Generic-shaped 3-section index-keyed/name-keyed collection-triple binary
/// encoder/decoder (mirrors dxf's own `enc_index_triple_bin`/`enc_name_triple_bin`), hand-
/// instantiated per collection below.
async fn enc_index_triple_bin<T, D>(removed: &[usize], modified: &[(usize, D)], added: &[(usize, T)], out: &mut Vec<u8>, enc_diff: impl Fn(&D, &mut Vec<u8>), enc_item: impl Fn(&T, &mut Vec<u8>)) {
    write_vec_bin(out, removed, |idx, o| write_usize_bin(o, *idx));
    store::pack_rt::write_varint_u64(out, modified.len() as u64);
    for (idx, d) in modified {
        write_usize_bin(out, *idx);
        enc_diff(d, out);
    }
    store::pack_rt::write_varint_u64(out, added.len() as u64);
    for (idx, item) in added {
        write_usize_bin(out, *idx);
        enc_item(item, out);
    }
}
async fn dec_index_triple_bin<T, D>(
    reader: &mut store::ByteReader<'_>,
    dec_diff: impl Fn(&mut store::ByteReader<'_>) -> Result<D, String>,
    dec_item: impl Fn(&mut store::ByteReader<'_>) -> Result<T, String>,
) -> Result<(Vec<usize>, Vec<(usize, D)>, Vec<(usize, T)>), String> {
    let removed = read_vec_bin(reader, read_usize_bin).await?;
    let mc = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(mc as usize);
    for _ in 0..mc {
        let idx = read_usize_bin(reader).await?;
        let d = dec_diff(reader)?;
        modified.push((idx, d));
    }
    let ac = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(ac as usize);
    for _ in 0..ac {
        let idx = read_usize_bin(reader).await?;
        let item = dec_item(reader)?;
        added.push((idx, item));
    }
    Ok((removed, modified, added))
}
async fn enc_named_triple_bin<T, D>(removed: &[String], modified: &[(String, D)], added: &[(usize, T)], out: &mut Vec<u8>, enc_diff: impl Fn(&D, &mut Vec<u8>), enc_item: impl Fn(&T, &mut Vec<u8>)) {
    write_vec_bin(out, removed, |name, o| write_str_bin(o, name));
    store::pack_rt::write_varint_u64(out, modified.len() as u64);
    for (name, d) in modified {
        write_str_bin(out, name);
        enc_diff(d, out);
    }
    store::pack_rt::write_varint_u64(out, added.len() as u64);
    for (idx, item) in added {
        write_usize_bin(out, *idx);
        enc_item(item, out);
    }
}
async fn dec_named_triple_bin<T, D>(
    reader: &mut store::ByteReader<'_>,
    dec_diff: impl Fn(&mut store::ByteReader<'_>) -> Result<D, String>,
    dec_item: impl Fn(&mut store::ByteReader<'_>) -> Result<T, String>,
) -> Result<(Vec<String>, Vec<(String, D)>, Vec<(usize, T)>), String> {
    let removed = read_vec_bin(reader, read_str_bin).await?;
    let mc = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(mc as usize);
    for _ in 0..mc {
        let name = read_str_bin(reader).await?;
        let d = dec_diff(reader)?;
        modified.push((name, d));
    }
    let ac = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(ac as usize);
    for _ in 0..ac {
        let idx = read_usize_bin(reader).await?;
        let item = dec_item(reader)?;
        added.push((idx, item));
    }
    Ok((removed, modified, added))
}

async fn enc_vertices_diff_bin(d: &ObjVerticesDiff, out: &mut Vec<u8>) {
    let modified: Vec<(usize, ObjVertexDiff)> = d.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
    let added: Vec<(usize, ObjVertex)> = d.added.iter().map(|a| (a.index, a.vertex.clone())).collect();
    enc_index_triple_bin(&d.removed, &modified, &added, out, enc_vertex_diff_bin, enc_vertex_bin);
}
async fn dec_vertices_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjVerticesDiff, String> {
    let (removed, modified, added) = dec_index_triple_bin(reader, dec_vertex_diff_bin, dec_vertex_bin).await?;
    Ok(ObjVerticesDiff { removed, modified: modified.into_iter().map(|(index, diff)| ObjVertexModified { index, diff }).collect(), added: added.into_iter().map(|(index, vertex)| ObjVertexAdded { index, vertex }).collect() })
}
async fn enc_texcoords_diff_bin(d: &ObjTexCoordsDiff, out: &mut Vec<u8>) {
    let modified: Vec<(usize, ObjTexCoordDiff)> = d.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
    let added: Vec<(usize, ObjTexCoord)> = d.added.iter().map(|a| (a.index, a.texcoord.clone())).collect();
    enc_index_triple_bin(&d.removed, &modified, &added, out, enc_texcoord_diff_bin, enc_texcoord_bin);
}
async fn dec_texcoords_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjTexCoordsDiff, String> {
    let (removed, modified, added) = dec_index_triple_bin(reader, dec_texcoord_diff_bin, dec_texcoord_bin).await?;
    Ok(ObjTexCoordsDiff { removed, modified: modified.into_iter().map(|(index, diff)| ObjTexCoordModified { index, diff }).collect(), added: added.into_iter().map(|(index, texcoord)| ObjTexCoordAdded { index, texcoord }).collect() })
}
async fn enc_normals_diff_bin(d: &ObjNormalsDiff, out: &mut Vec<u8>) {
    let modified: Vec<(usize, ObjNormalDiff)> = d.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
    let added: Vec<(usize, ObjNormal)> = d.added.iter().map(|a| (a.index, a.normal.clone())).collect();
    enc_index_triple_bin(&d.removed, &modified, &added, out, enc_normal_diff_bin, enc_normal_bin);
}
async fn dec_normals_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjNormalsDiff, String> {
    let (removed, modified, added) = dec_index_triple_bin(reader, dec_normal_diff_bin, dec_normal_bin).await?;
    Ok(ObjNormalsDiff { removed, modified: modified.into_iter().map(|(index, diff)| ObjNormalModified { index, diff }).collect(), added: added.into_iter().map(|(index, normal)| ObjNormalAdded { index, normal }).collect() })
}
async fn enc_faces_diff_bin(d: &ObjFacesDiff, out: &mut Vec<u8>) {
    let modified: Vec<(usize, ObjFaceDiff)> = d.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
    let added: Vec<(usize, ObjFace)> = d.added.iter().map(|a| (a.index, a.face.clone())).collect();
    enc_index_triple_bin(&d.removed, &modified, &added, out, enc_face_diff_bin, enc_face_bin);
}
async fn dec_faces_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjFacesDiff, String> {
    let (removed, modified, added) = dec_index_triple_bin(reader, dec_face_diff_bin, dec_face_bin).await?;
    Ok(ObjFacesDiff { removed, modified: modified.into_iter().map(|(index, diff)| ObjFaceModified { index, diff }).collect(), added: added.into_iter().map(|(index, face)| ObjFaceAdded { index, face }).collect() })
}
async fn enc_groups_diff_bin(d: &ObjGroupsDiff, out: &mut Vec<u8>) {
    let modified: Vec<(String, ObjGroupDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
    let added: Vec<(usize, ObjGroup)> = d.added.iter().map(|a| (a.index, a.group.clone())).collect();
    enc_named_triple_bin(&d.removed, &modified, &added, out, enc_group_diff_bin, enc_group_bin);
}
async fn dec_groups_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjGroupsDiff, String> {
    let (removed, modified, added) = dec_named_triple_bin(reader, dec_group_diff_bin, dec_group_bin).await?;
    Ok(ObjGroupsDiff { removed, modified: modified.into_iter().map(|(name, diff)| ObjGroupModified { name, diff }).collect(), added: added.into_iter().map(|(index, group)| ObjGroupAdded { index, group }).collect() })
}
async fn enc_objects_diff_bin(d: &ObjObjectsDiff, out: &mut Vec<u8>) {
    let modified: Vec<(String, ObjGroupDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
    let added: Vec<(usize, ObjObject)> = d.added.iter().map(|a| (a.index, a.object.clone())).collect();
    enc_named_triple_bin(&d.removed, &modified, &added, out, enc_group_diff_bin, enc_object_bin);
}
async fn dec_objects_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<ObjObjectsDiff, String> {
    let (removed, modified, added) = dec_named_triple_bin(reader, dec_group_diff_bin, dec_object_bin).await?;
    Ok(ObjObjectsDiff { removed, modified: modified.into_iter().map(|(name, diff)| ObjGroupModified { name, diff }).collect(), added: added.into_iter().map(|(index, object)| ObjObjectAdded { index, object }).collect() })
}
//#endregion 🔖️CollectionBinaryCodecs

//#region 🔖️TopLevel
async fn print_obj_diff(d: &ObjDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.vertices {
        tokens.push(enc_vertices_diff(v).await);
    }
    if let Some(v) = &d.texcoords {
        tokens.push(enc_texcoords_diff(v).await);
    }
    if let Some(v) = &d.normals {
        tokens.push(enc_normals_diff(v).await);
    }
    if let Some(v) = &d.faces {
        tokens.push(enc_faces_diff(v).await);
    }
    if let Some(v) = &d.groups {
        tokens.push(enc_groups_diff(v).await);
    }
    if let Some(v) = &d.objects {
        tokens.push(enc_objects_diff(v).await);
    }
    if let Some(v) = &d.mtllib {
        tokens.push(format!("mtllib={}", encode_option(v, |s| hex_encode_str(s))));
    }
    if let Some(v) = &d.usemtl {
        tokens.push(format!("usemtl=[{}]", v.iter().map(enc_usemtl).collect::<Vec<_>>().join(",")));
    }
    if let Some(v) = &d.smoothing_groups {
        tokens.push(format!("smoothing=[{}]", v.iter().map(enc_smoothing).collect::<Vec<_>>().join(",")));
    }
    if let Some(v) = &d.unknown_statements {
        tokens.push(format!("unknown=[{}]", v.iter().map(enc_unknown).collect::<Vec<_>>().join(",")));
    }
    tokens.join(" ")
}
async fn parse_obj_diff(line: &str) -> Result<ObjDiff, String> {
    let mut d = ObjDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("vertices{") {
            d.vertices = Some(dec_vertices_diff(rest.strip_suffix('}').ok_or_else(|| "vertices: missing closing brace".to_string())?).await?);
        } else if let Some(rest) = token.strip_prefix("texcoords{") {
            d.texcoords = Some(dec_texcoords_diff(rest.strip_suffix('}').ok_or_else(|| "texcoords: missing closing brace".to_string())?).await?);
        } else if let Some(rest) = token.strip_prefix("normals{") {
            d.normals = Some(dec_normals_diff(rest.strip_suffix('}').ok_or_else(|| "normals: missing closing brace".to_string())?).await?);
        } else if let Some(rest) = token.strip_prefix("faces{") {
            d.faces = Some(dec_faces_diff(rest.strip_suffix('}').ok_or_else(|| "faces: missing closing brace".to_string())?).await?);
        } else if let Some(rest) = token.strip_prefix("groups{") {
            d.groups = Some(dec_groups_diff(rest.strip_suffix('}').ok_or_else(|| "groups: missing closing brace".to_string())?).await?);
        } else if let Some(rest) = token.strip_prefix("objects{") {
            d.objects = Some(dec_objects_diff(rest.strip_suffix('}').ok_or_else(|| "objects: missing closing brace".to_string())?).await?);
        } else if let Some(rest) = token.strip_prefix("mtllib=") {
            d.mtllib = Some(decode_option(rest, hex_decode_str).await?);
        } else if let Some(rest) = token.strip_prefix("usemtl=") {
            d.usemtl = Some(split_top_level(strip_brackets(rest).await?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_usemtl).collect::<Result<Vec<_>, String>>()?);
        } else if let Some(rest) = token.strip_prefix("smoothing=") {
            d.smoothing_groups = Some(split_top_level(strip_brackets(rest).await?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_smoothing).collect::<Result<Vec<_>, String>>()?);
        } else if let Some(rest) = token.strip_prefix("unknown=") {
            d.unknown_statements = Some(split_top_level(strip_brackets(rest).await?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_unknown).collect::<Result<Vec<_>, String>>()?);
        } else {
            return Err(format!("obj diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl DiffCodec for ObjDiff {
    async fn print_diff(&self) -> String {
        print_obj_diff(self).await
    }
    async fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_obj_diff(line).await.map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// 🧪️ P2-FG1: REAL binary frame (`format u8 | flags_lo u8 | flags_hi u8 | per-present-field
    /// payload`), matching `../💾️binary/📡️component.protocol.semio`'s `header fixed 3` + `chain
    /// payload bytes` shape — upgraded from F6's `print_diff().into_bytes()` text-as-binary
    /// shortcut (the FG1 fixup wave's own finding: `obj` was one of 4 stdio standards left on that
    /// shortcut this same wave, despite md/xml/dxf's equally-recursive/flat types already proving
    /// the real upgrade achievable). `ObjDiff` has TEN independently optional top-level fields
    /// (`vertices`/`texcoords`/`normals`/`faces`/`groups`/`objects`/`mtllib`/`usemtl`/
    /// `smoothing_groups`/`unknown_statements`) — one more bit than a single `u8` flags byte can
    /// hold, so two flags bytes carry the presence mask (`flags_lo` bits 0-7 = vertices..usemtl,
    /// `flags_hi` bit 0 = smoothing_groups, bit 1 = unknown_statements) — same bitmask-over-`u8`
    /// device dxf's own `DxfDiff` (4 fields, one `flags` byte) upgrade introduced, just needing a
    /// second byte here. Every PRESENT field's own real, field-by-field binary payload follows
    /// (`#region 🔖️CollectionBinaryCodecs`/`#region 🔖️BinaryPrimitives` above) — `obj`'s whole diff
    /// tree is flat structs/`Vec`/`Option<T>` (module doc comment), so unlike md/dxf's recursive
    /// node types, NOTHING here falls back to an opaque byte-chain at the value layer; the ONLY
    /// thing still described as an opaque `chain` in the sibling `.protocol.semio` file is the
    /// collection-triple SHAPE itself (`Prim::Ref` cannot express a `Vec<Modified{index,diff}>`
    /// record-array in the protocol grammar — the same wall every collection-triple diff in this
    /// wave hit, documented in that file), never any individual scalar/struct value.
    async fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut flags_lo: u8 = 0;
        let mut flags_hi: u8 = 0;
        if self.vertices.is_some() {
            flags_lo |= 0b0000_0001;
        }
        if self.texcoords.is_some() {
            flags_lo |= 0b0000_0010;
        }
        if self.normals.is_some() {
            flags_lo |= 0b0000_0100;
        }
        if self.faces.is_some() {
            flags_lo |= 0b0000_1000;
        }
        if self.groups.is_some() {
            flags_lo |= 0b0001_0000;
        }
        if self.objects.is_some() {
            flags_lo |= 0b0010_0000;
        }
        if self.mtllib.is_some() {
            flags_lo |= 0b0100_0000;
        }
        if self.usemtl.is_some() {
            flags_lo |= 0b1000_0000;
        }
        if self.smoothing_groups.is_some() {
            flags_hi |= 0b0000_0001;
        }
        if self.unknown_statements.is_some() {
            flags_hi |= 0b0000_0010;
        }
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, flags_lo, flags_hi];
        if let Some(v) = &self.vertices {
            enc_vertices_diff_bin(v, &mut out);
        }
        if let Some(v) = &self.texcoords {
            enc_texcoords_diff_bin(v, &mut out);
        }
        if let Some(v) = &self.normals {
            enc_normals_diff_bin(v, &mut out);
        }
        if let Some(v) = &self.faces {
            enc_faces_diff_bin(v, &mut out);
        }
        if let Some(v) = &self.groups {
            enc_groups_diff_bin(v, &mut out);
        }
        if let Some(v) = &self.objects {
            enc_objects_diff_bin(v, &mut out);
        }
        if let Some(v) = &self.mtllib {
            write_option_bin(&mut out, v, |s, o| write_str_bin(o, s));
        }
        if let Some(v) = &self.usemtl {
            write_vec_bin(&mut out, v, enc_usemtl_bin);
        }
        if let Some(v) = &self.smoothing_groups {
            write_vec_bin(&mut out, v, enc_smoothing_bin);
        }
        if let Some(v) = &self.unknown_statements {
            write_vec_bin(&mut out, v, enc_unknown_bin);
        }
        Ok(out)
    }
    async fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes).await;
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().await.map_err(|e| malformed("diff format", 0, e.to_string()))?;
        let flags_lo = reader.read_u8().await.map_err(|e| malformed("diff flags_lo", 1, e.to_string()))?;
        let flags_hi = reader.read_u8().await.map_err(|e| malformed("diff flags_hi", 2, e.to_string()))?;
        let vertices = if flags_lo & 0b0000_0001 != 0 { Some(dec_vertices_diff_bin(&mut reader).await.map_err(|e| malformed("diff vertices", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None };
        let texcoords = if flags_lo & 0b0000_0010 != 0 { Some(dec_texcoords_diff_bin(&mut reader).await.map_err(|e| malformed("diff texcoords", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None };
        let normals = if flags_lo & 0b0000_0100 != 0 { Some(dec_normals_diff_bin(&mut reader).await.map_err(|e| malformed("diff normals", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None };
        let faces = if flags_lo & 0b0000_1000 != 0 { Some(dec_faces_diff_bin(&mut reader).await.map_err(|e| malformed("diff faces", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None };
        let groups = if flags_lo & 0b0001_0000 != 0 { Some(dec_groups_diff_bin(&mut reader).await.map_err(|e| malformed("diff groups", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None };
        let objects = if flags_lo & 0b0010_0000 != 0 { Some(dec_objects_diff_bin(&mut reader).await.map_err(|e| malformed("diff objects", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None };
        let mtllib = if flags_lo & 0b0100_0000 != 0 { Some(read_option_bin(&mut reader, read_str_bin).await.map_err(|e| malformed("diff mtllib", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None };
        let usemtl = if flags_lo & 0b1000_0000 != 0 { Some(read_vec_bin(&mut reader, dec_usemtl_bin).await.map_err(|e| malformed("diff usemtl", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None };
        let smoothing_groups = if flags_hi & 0b0000_0001 != 0 { Some(read_vec_bin(&mut reader, dec_smoothing_bin).await.map_err(|e| malformed("diff smoothing_groups", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None };
        let unknown_statements = if flags_hi & 0b0000_0010 != 0 { Some(read_vec_bin(&mut reader, dec_unknown_bin).await.map_err(|e| malformed("diff unknown_statements", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None };
        Ok(ObjDiff { vertices, texcoords, normals, faces, groups, objects, mtllib, usemtl, smoothing_groups, unknown_statements })
    }
}
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️MutationDiffBuilders
// 🧮 Item-level `between` wrappers, exposed to `🧬️mutations` so `SetVertex`/`SetTexCoord`/
// `SetNormal`/`SetFace`'s `diff()` can compute a sparse per-field patch without the private
// `ObjIndexElem` trait itself leaving this module.
pub async fn vertex_diff_between(a: &ObjVertex, b: &ObjVertex) -> ObjVertexDiff {
    <ObjVertex as ObjIndexElem>::diff_between(a, b).await
}
pub async fn texcoord_diff_between(a: &ObjTexCoord, b: &ObjTexCoord) -> ObjTexCoordDiff {
    <ObjTexCoord as ObjIndexElem>::diff_between(a, b).await
}
pub async fn normal_diff_between(a: &ObjNormal, b: &ObjNormal) -> ObjNormalDiff {
    <ObjNormal as ObjIndexElem>::diff_between(a, b).await
}
pub async fn face_diff_between(a: &ObjFace, b: &ObjFace) -> ObjFaceDiff {
    <ObjFace as ObjIndexElem>::diff_between(a, b).await
}

pub async fn diff_insert_vertex(index: usize, vertex: ObjVertex) -> ObjDiff {
    ObjDiff { vertices: Some(ObjVerticesDiff { removed: vec![], modified: vec![], added: vec![ObjVertexAdded { index, vertex }] }), ..Default::default() }
}
pub async fn diff_remove_vertex(index: usize) -> ObjDiff {
    ObjDiff { vertices: Some(ObjVerticesDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }
}
pub async fn diff_set_vertex(index: usize, diff: ObjVertexDiff) -> ObjDiff {
    ObjDiff { vertices: Some(ObjVerticesDiff { removed: vec![], modified: vec![ObjVertexModified { index, diff }], added: vec![] }), ..Default::default() }
}
pub async fn diff_insert_texcoord(index: usize, texcoord: ObjTexCoord) -> ObjDiff {
    ObjDiff { texcoords: Some(ObjTexCoordsDiff { removed: vec![], modified: vec![], added: vec![ObjTexCoordAdded { index, texcoord }] }), ..Default::default() }
}
pub async fn diff_remove_texcoord(index: usize) -> ObjDiff {
    ObjDiff { texcoords: Some(ObjTexCoordsDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }
}
pub async fn diff_set_texcoord(index: usize, diff: ObjTexCoordDiff) -> ObjDiff {
    ObjDiff { texcoords: Some(ObjTexCoordsDiff { removed: vec![], modified: vec![ObjTexCoordModified { index, diff }], added: vec![] }), ..Default::default() }
}
pub async fn diff_insert_normal(index: usize, normal: ObjNormal) -> ObjDiff {
    ObjDiff { normals: Some(ObjNormalsDiff { removed: vec![], modified: vec![], added: vec![ObjNormalAdded { index, normal }] }), ..Default::default() }
}
pub async fn diff_remove_normal(index: usize) -> ObjDiff {
    ObjDiff { normals: Some(ObjNormalsDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }
}
pub async fn diff_set_normal(index: usize, diff: ObjNormalDiff) -> ObjDiff {
    ObjDiff { normals: Some(ObjNormalsDiff { removed: vec![], modified: vec![ObjNormalModified { index, diff }], added: vec![] }), ..Default::default() }
}
pub async fn diff_insert_face(index: usize, face: ObjFace) -> ObjDiff {
    ObjDiff { faces: Some(ObjFacesDiff { removed: vec![], modified: vec![], added: vec![ObjFaceAdded { index, face }] }), ..Default::default() }
}
pub async fn diff_remove_face(index: usize) -> ObjDiff {
    ObjDiff { faces: Some(ObjFacesDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }
}
pub async fn diff_set_face(index: usize, diff: ObjFaceDiff) -> ObjDiff {
    ObjDiff { faces: Some(ObjFacesDiff { removed: vec![], modified: vec![ObjFaceModified { index, diff }], added: vec![] }), ..Default::default() }
}
pub async fn diff_set_group(index: usize, name: &str, faces: Vec<usize>, existed: bool) -> ObjDiff {
    if existed {
        ObjDiff { groups: Some(ObjGroupsDiff { removed: vec![], modified: vec![ObjGroupModified { name: name.to_string(), diff: ObjGroupDiff { faces: Some(faces) } }], added: vec![] }), ..Default::default() }
    } else {
        ObjDiff { groups: Some(ObjGroupsDiff { removed: vec![], modified: vec![], added: vec![ObjGroupAdded { index, group: ObjGroup { name: name.to_string(), faces } }] }), ..Default::default() }
    }
}
pub async fn diff_remove_group(name: &str) -> ObjDiff {
    ObjDiff { groups: Some(ObjGroupsDiff { removed: vec![name.to_string()], modified: vec![], added: vec![] }), ..Default::default() }
}
pub async fn diff_set_object(index: usize, name: &str, faces: Vec<usize>, existed: bool) -> ObjDiff {
    if existed {
        ObjDiff { objects: Some(ObjObjectsDiff { removed: vec![], modified: vec![ObjGroupModified { name: name.to_string(), diff: ObjGroupDiff { faces: Some(faces) } }], added: vec![] }), ..Default::default() }
    } else {
        ObjDiff { objects: Some(ObjObjectsDiff { removed: vec![], modified: vec![], added: vec![ObjObjectAdded { index, object: ObjObject { name: name.to_string(), faces } }] }), ..Default::default() }
    }
}
pub async fn diff_remove_object(name: &str) -> ObjDiff {
    ObjDiff { objects: Some(ObjObjectsDiff { removed: vec![name.to_string()], modified: vec![], added: vec![] }), ..Default::default() }
}
pub async fn diff_set_mtllib(mtllib: Option<String>) -> ObjDiff {
    ObjDiff { mtllib: Some(mtllib), ..Default::default() }
}
pub async fn diff_set_usemtl(usemtl: Vec<ObjUsemtlRange>) -> ObjDiff {
    ObjDiff { usemtl: Some(usemtl), ..Default::default() }
}
pub async fn diff_set_smoothing_groups(smoothing_groups: Vec<ObjSmoothingRange>) -> ObjDiff {
    ObjDiff { smoothing_groups: Some(smoothing_groups), ..Default::default() }
}
pub async fn diff_set_unknown_statements(unknown_statements: Vec<ObjUnknownStatement>) -> ObjDiff {
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
pub(crate) async fn sweep_a() -> ObjSnapshot {
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
pub(crate) async fn sweep_b() -> ObjSnapshot {
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
/// `../../⚙️engine/🦀️component.rs`'s `diff_grammar_conformance_law`/`protocol_walk_law`
/// conformance tests, same convention P2-P1's json/zip pilots established.
#[cfg(test)]
pub(crate) async fn demo_diff_cases() -> Vec<ObjDiff> {
    let a = sweep_a();
    let b = sweep_b();
    vec![ObjDiff::default(), <ObjDiff as DiffAlgebra<ObjSnapshot>>::between(&a, &b), <ObjDiff as DiffAlgebra<ObjSnapshot>>::between(&b, &a)]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn invalid_collection_targets_are_rejected_before_mutation() {
        let base = ObjSnapshot::default();
        let diff = ObjDiff { vertices: Some(ObjVerticesDiff { removed: vec![0], ..Default::default() }), ..Default::default() };
        let error = diff.apply(&base).expect_err("missing vertex target must be rejected");
        assert_eq!(error.code, "invalid-remove-index");
        assert_eq!(error.target, vec!["vertices", "0"]);
        assert_eq!(base, ObjSnapshot::default());
    }

    /// 🧪️ F6: `DiffCodec` round-trip laws for the hand-rolled `ObjDiff` text/binary grammar —
    /// exercises every scalar, both tri-states (`mtllib` at the top level, `texcoords[1].w`
    /// inside a modified item), and all three collection-triple kinds — index-keyed
    /// (`vertices`/`texcoords`/`normals`/`faces`) AND name-keyed (`groups`/`objects`) — via a real
    /// `between()` result in both directions.
    #[semio_framework_async_macros::async_test]
    async fn diff_codec_text_binary_roundtrip_law() {
        let a = sweep_a();
        let b = sweep_b();
        let ab = <ObjDiff as DiffAlgebra<ObjSnapshot>>::between(&a, &b);
        assert_eq!(ab.mtllib, Some(None), "mtllib tri-state must exercise Some(None)");
        let td = ab.texcoords.as_ref().expect("texcoords diff populated");
        assert_eq!(td.modified[0].diff.w, Some(None), "texcoord w tri-state must exercise Some(None)");
        assert!(!ab.groups.as_ref().unwrap().removed.is_empty() && !ab.groups.as_ref().unwrap().modified.is_empty() && !ab.groups.as_ref().unwrap().added.is_empty(), "groups triple must exercise all 3 kinds");

        let cases = vec![ObjDiff::default(), ab, <ObjDiff as DiffAlgebra<ObjSnapshot>>::between(&b, &a)];
        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = ObjDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = ObjDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
}
//#endregion 🧪️Tests
