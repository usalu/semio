//! 🔺️ StlDiff — sparse per-field diff. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: replaces the old
//! `StlDiff{snapshot: Option<StlSnapshot>}` full-replace template with a real per-field patch —
//! `solid_name` plus an index-keyed `triangles` triple (`removed`/`modified`/`added`); each
//! `StlTriangle`'s `normal`/`vertices` fields are whole-value replaced (fixed-size arrays, no
//! sub-diffing per the recipe's weak-field rule).
//!
//! 🧪️ F6 (OpText/OpBinary + DiffCodec wave): **HAND-ROLL path** — `StlDiff`'s field tree has zero
//! `pub enum` nodes and zero `Option<Option<_>>` tri-state fields, so neither §3a nor §3b of
//! `f6-recon-report.md`'s documented decision rule applies, and `#[derive(dsl::DslDiff)]` was
//! first attempted and DID compile cleanly (every nested type — `StlTriangle`,
//! `StlTriangleDiff`/`Modified`/`Added`/`StlTrianglesDiff` — derived `dsl::DslRecord` with zero
//! errors). It was reverted after a real `cargo test` run found a THIRD, undocumented blocker:
//! `vertices: [[f64; 3]; 3]` is a doubly-nested fixed-size array, and `dsl`'s grammar engine
//! prints every `Shape::Tuple` level as a flat, unbracketed comma-join with no depth marker, so
//! `parse_diff` cannot tell where the outer 3-tuple ends and the inner 3-tuples begin — confirmed
//! verbatim: `parse_diff("… vertices=1,2,3,4,5,6,7,8,9 …")` → `"tuple expects 3 elements, found
//! 9"`. Full root-cause citation on `StlTriangle`'s own doc comment in `📸️snapshot::component`
//! (`dsl` is a shared framework module, out of this artifact's ownership boundary to fix). The
//! grammar below sidesteps the bug entirely: `enc_vec3`/`enc_vertices` wrap EVERY array level in
//! its own `[...]` (bracket-depth-aware `split_top_level`, same primitive `gif`89a's/`svg`'s
//! hand-rolled `DiffBinary,DiffCodec,DiffText` use), so nesting is unambiguous.

/// 🧩 Ordered removed keys, modified values, and inserted items.
pub(crate) type IndexedDiffParts<D, T> = (Vec<usize>, Vec<(usize, D)>, Vec<(usize, T)>);

use crate::schema::snapshot::StlTriangle;
use crate::StlSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{BTreeSet, HashMap, HashSet};

//#region 🔖️TriangleDiff
/// 🔺️ Sparse per-field patch for one `StlTriangle`. Both fields are fixed-size arrays — whole-
/// value replace, never sub-diffed (matches the recipe's weak-entity rule for value structs).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct StlTriangleDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub normal: Option<[f64; 3]>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub vertices: Option<[[f64; 3]; 3]>,
}

/// ▶️ Applies a per-field triangle patch, returning the patched triangle.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_triangle_diff(base: &StlTriangle, diff: &StlTriangleDiff) -> StlTriangle {
    StlTriangle { normal: diff.normal.unwrap_or(base.normal), vertices: diff.vertices.unwrap_or(base.vertices) }
}

/// 🧭️ Field-by-field state delta between two triangles occupying the same index slot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn triangle_between(a: &StlTriangle, b: &StlTriangle) -> StlTriangleDiff {
    StlTriangleDiff { normal: (a.normal != b.normal).then_some(b.normal), vertices: (a.vertices != b.vertices).then_some(b.vertices) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn triangle_diff_is_empty(d: &StlTriangleDiff) -> bool {
    d.normal.is_none() && d.vertices.is_none()
}

/// ➕️ LWW field-by-field absorb of one triangle patch into another.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_triangle_diff(base: &mut StlTriangleDiff, other: &StlTriangleDiff) {
    if other.normal.is_some() {
        base.normal = other.normal;
    }
    if other.vertices.is_some() {
        base.vertices = other.vertices;
    }
}
//#endregion 🔖️TriangleDiff

//#region 🔖️TrianglesTriple
/// 📦️ One `triangles.modified[]` entity — `index` is the triangle's position **in BASE**.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct StlTriangleModified {
    pub index: usize,
    pub diff: StlTriangleDiff,
}

/// 📦️ One `triangles.added[]` entity — `index` is the triangle's position in the FINAL sequence
/// (apply semantics: `added` indices refer to final state, inserted ascending at `min(index,
/// len)`; see the recipe's `## Absorb` section for the full apply/absorb contract).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct StlTriangleAdded {
    pub index: usize,
    pub triangle: StlTriangle,
}

/// 📦️ Sparse index-keyed `triangles` triple.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct StlTrianglesDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<StlTriangleModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<StlTriangleAdded>,
}

impl StlTrianglesDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

/// ▶️ Applies the triangles triple: (1) `modified` by BASE index, (2) `removed` by BASE index
/// processed descending so earlier removals don't shift later indices, (3) `added` by FINAL
/// index, ascending, `insert(min(index, len))`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_triangles_diff(triangles: &[StlTriangle], diff: &StlTrianglesDiff) -> Vec<StlTriangle> {
    let mut result = triangles.to_vec();
    for m in &diff.modified {
        result[m.index] = apply_triangle_diff(&result[m.index], &m.diff);
    }
    let mut removed_sorted = diff.removed.clone();
    removed_sorted.sort_unstable_by(|a, b| b.cmp(a));
    for idx in removed_sorted {
        result.remove(idx);
    }
    let mut added_sorted: Vec<&StlTriangleAdded> = diff.added.iter().collect();
    added_sorted.sort_by_key(|a| a.index);
    for a in added_sorted {
        result.insert(a.index, a.triangle);
    }
    result
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_triangles_diff(base_len: usize, diff: &StlTrianglesDiff) -> MutationApplyResult<()> {
    let mut removed = BTreeSet::new();
    for &index in &diff.removed {
        if index >= base_len || !removed.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.invalid-remove-index", "triangle removal target must exist exactly once").at(["triangles", &index.to_string()]));
        }
    }
    let mut modified = BTreeSet::new();
    for entry in &diff.modified {
        if entry.index >= base_len || removed.contains(&entry.index) || !modified.insert(entry.index) {
            return Err(MutationApplyError::new("mutation.apply.invalid-modify-index", "triangle modification target must exist exactly once and remain present").at(["triangles", &entry.index.to_string()]));
        }
    }
    let mut additions: Vec<usize> = diff.added.iter().map(|entry| entry.index).collect();
    additions.sort_unstable();
    let mut previous = None;
    for (length, index) in (base_len - removed.len()..).zip(additions) {
        if index > length || previous == Some(index) {
            return Err(MutationApplyError::new("mutation.apply.invalid-add-index", "triangle addition target must be unique and within the evolving sequence").at(["triangles", &index.to_string()]));
        }
        previous = Some(index);
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_stl_diff_unchecked(diff: &StlDiff, base: &StlSnapshot) -> StlSnapshot {
    let triangles = diff.triangles.as_ref().map_or_else(|| base.triangles.clone(), |value| apply_triangles_diff(&base.triangles, value));
    StlSnapshot { schema: base.schema.clone(), solid_name: diff.solid_name.clone().unwrap_or_else(|| base.solid_name.clone()), triangles }
}

/// 🧭️ Index-keyed state delta between two triangle lists: pairwise-compared over
/// `0..min(len)` (→ `modified`), whichever side is longer supplies the tail (`removed` if BASE is
/// longer, `added` if OTHER is longer) — structurally only one tail kind can ever be non-empty
/// from a single call (the known flat/unkeyed-collection limitation; `field_sweep` below exercises
/// both directions to prove both tail kinds, per the ticket's documented fix pattern).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn triangles_between(a: &[StlTriangle], b: &[StlTriangle]) -> StlTrianglesDiff {
    let min_len = a.len().min(b.len());
    let mut modified = Vec::new();
    for i in 0..min_len {
        let d = triangle_between(&a[i], &b[i]);
        if !triangle_diff_is_empty(&d) {
            modified.push(StlTriangleModified { index: i, diff: d });
        }
    }
    let removed: Vec<usize> = (min_len..a.len()).collect();
    let added: Vec<StlTriangleAdded> = (min_len..b.len()).map(|i| StlTriangleAdded { index: i, triangle: b[i] }).collect();
    StlTrianglesDiff { removed, modified, added }
}

//#region 🔖️AbsorbLabels
/// 🏷️ A structural, base-free label used only inside [`absorb_pair`] to simulate the two-step
/// position transform (base→mid via `d1`, mid→after via `d2`) without ever looking at real
/// triangle content — absorb's normative contract is "structural" and "base-free". `Base(i)`
/// traces an original base-array index; `Added1`/`Added2` trace a still-alive entry from
/// `d1.added`/`d2.added` (by its position in that Vec, so we can look its payload back up). Same
/// technique as `txt`'s `TxtLinesDiff` absorb (this ticket's other flat/index-keyed collection).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Lbl {
    Base(usize),
    Added1(usize),
    Added2(usize),
}

/// ➡️ Structural simulate of [`apply_triangles_diff`]'s position algebra over an abstract label
/// array: remove the given indices, then insert `added` labels ascending at
/// `min(index, current_len)`. Mirrors `apply`'s exact algorithm but carries labels, not
/// triangles, so it can run without any real snapshot.
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

/// ➕️ Absorbs `d1` (base→mid) then `d2` (mid→after) into a single base→after
/// [`StlTrianglesDiff`] (`## Absorb` contract; structural, total, base-free sequential-coalesce).
/// A virtual base of `Lbl::Base(0..l1)`, large enough to cover every index either diff
/// references, is walked through `d1`'s remove/insert then `d2`'s; the resulting label array is
/// read back into `removed`/`modified`/`added`: a base index present in `after_labels` ⇒ kept
/// (its `modified` patch is `d1`'s own patch field-by-field absorbed with whatever `d2` patch
/// lands at its mid-position — "d2 patch on a surviving base item recursively absorbs into the
/// matching m1 entry"); absent ⇒ `removed`. An `Added1` entry that a `d2`-removal targets is
/// simply absent from the walk ("annihilates the add", never re-emitted); one a `d2`-modify
/// targets gets that patch applied directly into its carried payload ("Add+SetField").
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_pair(d1: &StlTrianglesDiff, d2: &StlTrianglesDiff) -> StlTrianglesDiff {
    let max_ref =
        d1.removed.iter().copied().chain(d1.modified.iter().map(|m| m.index)).chain(d1.added.iter().map(|a| a.index)).chain(d2.removed.iter().copied()).chain(d2.modified.iter().map(|m| m.index)).chain(d2.added.iter().map(|a| a.index)).max();
    let l1 = max_ref.map_or(0, |m| m + 2);

    let base_labels: Vec<Lbl> = (0..l1).map(Lbl::Base).collect();
    let d1_added: Vec<(usize, Lbl)> = d1.added.iter().enumerate().map(|(j, a)| (a.index, Lbl::Added1(j))).collect();
    let mut mid_labels = simulate_labels(base_labels, &d1.removed, &d1_added);

    // 🔍️ Record each label's MID position — exactly the φ(base_index)/mid_index_of(Added1(j))
    // transport the recipe calls for.
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

    // 📦 `l1` already covers `d2`'s own max reference; pad is appended at the tail only —
    // `Vec::push` never disturbs earlier positions, so `mid_pos_of_*` stay valid.
    while mid_labels.len() < l1 {
        mid_labels.push(Lbl::Base(usize::MAX)); // inert padding index, never referenced by mid_pos_of_base
    }

    let d2_added: Vec<(usize, Lbl)> = d2.added.iter().enumerate().map(|(k, a)| (a.index, Lbl::Added2(k))).collect();
    let after_labels = simulate_labels(mid_labels, &d2.removed, &d2_added);

    let d2_modified_at: HashMap<usize, &StlTriangleDiff> = d2.modified.iter().map(|m| (m.index, &m.diff)).collect();
    let d1_modified_at: HashMap<usize, &StlTriangleDiff> = d1.modified.iter().map(|m| (m.index, &m.diff)).collect();

    let mut present_base: HashSet<usize> = HashSet::new();
    let mut modified = Vec::new();
    let mut added = Vec::new();

    for (pos, l) in after_labels.into_iter().enumerate() {
        match l {
            Lbl::Base(i) if i != usize::MAX => {
                present_base.insert(i);
                let mid_pos = mid_pos_of_base.get(&i).copied();
                let mut combined = d1_modified_at.get(&i).map(|d| (*d).clone()).unwrap_or_default();
                if let Some(mp) = mid_pos {
                    if let Some(d2d) = d2_modified_at.get(&mp) {
                        absorb_triangle_diff(&mut combined, d2d);
                    }
                }
                if !triangle_diff_is_empty(&combined) {
                    modified.push(StlTriangleModified { index: i, diff: combined });
                }
            }
            Lbl::Base(_) => { /* padding survived untouched — never real, ignore */ }
            Lbl::Added1(j) => {
                let mid_pos = mid_pos_of_added1.get(&j).copied();
                let mut triangle = d1.added[j].triangle;
                if let Some(mp) = mid_pos {
                    if let Some(d2d) = d2_modified_at.get(&mp) {
                        triangle = apply_triangle_diff(&triangle, d2d);
                    }
                }
                added.push(StlTriangleAdded { index: pos, triangle });
            }
            Lbl::Added2(k) => {
                added.push(StlTriangleAdded { index: pos, triangle: d2.added[k].triangle });
            }
        }
    }

    let removed: Vec<usize> = (0..l1).filter(|i| !present_base.contains(i)).collect();
    StlTrianglesDiff { removed, modified, added }
}

/// ➕️ Structural, total, base-free sequential-coalesce of two `Option<StlTrianglesDiff>`s
/// (`## Absorb` contract) — the `None`-collapsing wrapper around [`absorb_pair`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_triangles(d1: Option<StlTrianglesDiff>, d2: Option<StlTrianglesDiff>) -> Option<StlTrianglesDiff> {
    let merged = match (d1, d2) {
        (None, None) => return None,
        (Some(d1), None) => return Some(d1),
        (None, Some(d2)) => return Some(d2),
        (Some(d1), Some(d2)) => absorb_pair(&d1, &d2),
    };
    if merged.is_empty() {
        None
    } else {
        Some(merged)
    }
}
//#endregion 🔖️AbsorbLabels
//#endregion 🔖️TrianglesTriple

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.stl`. `schema` is an identity field and never appears here.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.stl.diff")]
pub struct StlDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub solid_name: Option<String>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub triangles: Option<StlTrianglesDiff>,
}

impl MutationDiff<StlSnapshot> for StlDiff {
    fn apply(&self, base: &StlSnapshot) -> MutationApplyResult<StlSnapshot> {
        if let Some(diff) = &self.triangles {
            validate_triangles_diff(base.triangles.len(), diff)?;
        }
        Ok(apply_stl_diff_unchecked(self, base))
    }

    /// ➕️ Structural, total, base-free sequential-coalesce (`## Absorb` contract). Scalar
    /// `solid_name`: LWW. `triangles`: see `absorb_triangles`.
    fn absorb(&mut self, other: Self) {
        if other.solid_name.is_some() {
            self.solid_name = other.solid_name;
        }
        self.triangles = absorb_triangles(self.triangles.take(), other.triangles);
    }
}

impl DiffAlgebra<StlSnapshot> for StlDiff {
    /// 🔁️ Diff-level undo, derived generically (correct by construction): the state delta from
    /// `self.apply(base)` back to `base` — `between` is the single source of truth for turning a
    /// state pair into a diff, so `inverse` doesn't duplicate its per-field logic (same pattern
    /// as this ticket's zip/xml precedent).
    fn inverse(&self, base: &StlSnapshot) -> Self {
        let mutated = apply_stl_diff_unchecked(self, base);
        <Self as DiffAlgebra<StlSnapshot>>::between(&mutated, base)
    }

    /// 🧭️ State delta (compose `GetXDiff`): `triangles` uses index-pairwise matching (see
    /// `triangles_between`'s doc comment for the single-tail-kind-per-call caveat).
    fn between(base: &StlSnapshot, other: &StlSnapshot) -> Self {
        let solid_name = (base.solid_name != other.solid_name).then(|| other.solid_name.clone());
        let td = triangles_between(&base.triangles, &other.triangles);
        let triangles = if td.is_empty() { None } else { Some(td) };
        StlDiff { solid_name, triangles }
    }

    fn is_empty(&self) -> bool {
        self.solid_name.is_none() && self.triangles.as_ref().is_none_or(StlTrianglesDiff::is_empty)
    }
}
//#endregion 🔖️Diff

//#region 🔖️MutationDiffBuilders
/// 🧩 `SetSnapshot`'s diff is the sparse field-by-field `between(base, next)` — no full-replace
/// slot exists on `StlDiff` to short-circuit into.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &StlSnapshot, next: &StlSnapshot) -> StlDiff {
    <StlDiff as DiffAlgebra<StlSnapshot>>::between(base, next)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_solid_name(name: &str) -> StlDiff {
    StlDiff { solid_name: Some(name.to_string()), triangles: None }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_triangle(index: usize, triangle: StlTriangle) -> StlDiff {
    StlDiff { solid_name: None, triangles: Some(StlTrianglesDiff { removed: vec![], modified: vec![], added: vec![StlTriangleAdded { index, triangle }] }) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_triangle(index: usize) -> StlDiff {
    StlDiff { solid_name: None, triangles: Some(StlTrianglesDiff { removed: vec![index], modified: vec![], added: vec![] }) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_triangle_field(index: usize, field: StlTriangleDiff) -> StlDiff {
    StlDiff { solid_name: None, triangles: Some(StlTrianglesDiff { removed: vec![], modified: vec![StlTriangleModified { index, diff: field }], added: vec![] }) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_triangle_normal(index: usize, normal: [f64; 3]) -> StlDiff {
    diff_triangle_field(index, StlTriangleDiff { normal: Some(normal), vertices: None })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_triangle_vertices(index: usize, vertices: [[f64; 3]; 3]) -> StlDiff {
    diff_triangle_field(index, StlTriangleDiff { normal: None, vertices: Some(vertices) })
}
//#endregion 🔖️MutationDiffBuilders

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: hand-rolled `protocol::DiffCodec` for `StlDiff` — see this file's top doc comment and
/// `StlTriangle`'s doc comment (`📸️snapshot::component`) for the real, reproduced `dsl`-derive
/// bug that forced this path despite `StlDiff` having no enum and no tri-state field.
///
/// **Grammar**: one space-separated `name=value` token per changed top-level field (a field
/// absent from the line = unchanged); `triangles` prints as `triangles{[removed];[modified];[added]}`
/// (same collection-triple shape `gif`89a's hand-roll uses). `solid_name` is lowercase hex (no
/// external base64 dep, matches this artifact family's own `ArtifactDsl` idiom). Every array level
/// (`normal: [f64;3]`, each `vertices[i]: [f64;3]`, the outer `vertices: [[f64;3];3]`) gets its
/// own `[...]` bracket — the depth marker `dsl`'s own `Shape::Tuple` printer is missing — so
/// `split_top_level`'s bracket-depth-aware comma split recovers nesting unambiguously. `f64`
/// values print via Rust's `Display` (`{}`), which round-trips exactly (shortest-round-trippable
/// representation, guaranteed since Rust 1.0) — `str::parse::<f64>()` on the other end recovers
/// the identical bit pattern.
//#region 🔖️Primitives









//#endregion 🔖️Primitives

//#region 🔖️BinaryPrimitives






//#endregion 🔖️BinaryPrimitives

//#region 🔖️ValueCodecs






//#endregion 🔖️ValueCodecs

//#region 🔖️ValueBinaryCodecs






//#endregion 🔖️ValueBinaryCodecs

//#region 🔖️DiffValueCodecs








//#endregion 🔖️DiffValueCodecs

//#region 🔖️DiffValueBinaryCodecs




//#endregion 🔖️DiffValueBinaryCodecs

//#region 🔖️TopLevel




//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️DemoCases
/// 🎯 FG1: representative `StlDiff` cases — the empty default, a full triple (`removed`+`modified`+
/// `added` simultaneously, incl. the doubly-nested `vertices` field), and a `modified`-only case
/// exercising the sparse `V`-tag-without-`N`-tag path of `triangle-diff-value`'s own permissive
/// grammar. Shared by this file's own `⚙️engine::conformance_laws`'s `diff_grammar_conformance_law`/
/// `protocol_walk_law` AND `🧬️mutations::component`'s `diff_codec_text_binary_roundtrip_law` (same
/// reuse pattern `binary`'s own `demo_diff_cases` establishes).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<StlDiff> {
    vec![
        StlDiff::default(),
        StlDiff {
            solid_name: Some("after".into()),
            triangles: Some(StlTrianglesDiff {
                removed: vec![2],
                modified: vec![StlTriangleModified { index: 0, diff: StlTriangleDiff { normal: Some([0.0, 0.0, 1.0]), vertices: Some([[5.0, 0.0, 0.0], [6.0, 0.0, 0.0], [5.0, 1.0, 0.0]]) } }],
                added: vec![StlTriangleAdded { index: 1, triangle: StlTriangle { normal: [-1.0, 0.0, 0.0], vertices: [[20.0, 0.0, 0.0], [21.0, 0.0, 0.0], [20.0, 1.0, 0.0]] } }],
            }),
        },
        StlDiff {
            solid_name: None,
            triangles: Some(StlTrianglesDiff { removed: vec![], modified: vec![StlTriangleModified { index: 0, diff: StlTriangleDiff { normal: None, vertices: Some([[1.0, 1.0, 1.0], [2.0, 2.0, 2.0], [3.0, 3.0, 3.0]]) } }], added: vec![] }),
        },
    ]
}
//#endregion 🔖️DemoCases
