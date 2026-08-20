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
//! hand-rolled `DiffCodec` use), so nesting is unambiguous.

use crate::artifacts::stl::schema::snapshot::StlTriangle;
use crate::artifacts::stl::StlSnapshot;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap, HashSet};

//#region 🔖️TriangleDiff
/// 🔺️ Sparse per-field patch for one `StlTriangle`. Both fields are fixed-size arrays — whole-
/// value replace, never sub-diffed (matches the recipe's weak-entity rule for value structs).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StlTriangleDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normal: Option<[f64; 3]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vertices: Option<[[f64; 3]; 3]>,
}

/// ▶️ Applies a per-field triangle patch, returning the patched triangle.
async fn apply_triangle_diff(base: &StlTriangle, diff: &StlTriangleDiff) -> StlTriangle {
    StlTriangle { normal: diff.normal.unwrap_or(base.normal), vertices: diff.vertices.unwrap_or(base.vertices) }
}

/// 🧭️ Field-by-field state delta between two triangles occupying the same index slot.
async fn triangle_between(a: &StlTriangle, b: &StlTriangle) -> StlTriangleDiff {
    StlTriangleDiff { normal: (a.normal != b.normal).then_some(b.normal), vertices: (a.vertices != b.vertices).then_some(b.vertices) }
}

async fn triangle_diff_is_empty(d: &StlTriangleDiff) -> bool {
    d.normal.is_none() && d.vertices.is_none()
}

/// ➕️ LWW field-by-field absorb of one triangle patch into another.
async fn absorb_triangle_diff(base: &mut StlTriangleDiff, other: StlTriangleDiff) {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StlTriangleModified {
    pub index: usize,
    pub diff: StlTriangleDiff,
}

/// 📦️ One `triangles.added[]` entity — `index` is the triangle's position in the FINAL sequence
/// (apply semantics: `added` indices refer to final state, inserted ascending at `min(index,
/// len)`; see the recipe's `## Absorb` section for the full apply/absorb contract).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StlTriangleAdded {
    pub index: usize,
    pub triangle: StlTriangle,
}

/// 📦️ Sparse index-keyed `triangles` triple.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StlTrianglesDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<StlTriangleModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<StlTriangleAdded>,
}

impl StlTrianglesDiff {
    pub async fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

/// ▶️ Applies the triangles triple: (1) `modified` by BASE index, (2) `removed` by BASE index
/// processed descending so earlier removals don't shift later indices, (3) `added` by FINAL
/// index, ascending, `insert(min(index, len))`.
async fn apply_triangles_diff(triangles: &[StlTriangle], diff: &StlTrianglesDiff) -> Vec<StlTriangle> {
    let mut result = triangles.to_vec();
    for m in &diff.modified {
        result[m.index] = apply_triangle_diff(&result[m.index], &m.diff).await;
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

async fn validate_triangles_diff(base_len: usize, diff: &StlTrianglesDiff) -> MutationApplyResult<()> {
    let mut removed = BTreeSet::new();
    for &index in &diff.removed {
        if index >= base_len || !removed.insert(index) {
            return Err(MutationApplyError::new("invalid-remove-index", "triangle removal target must exist exactly once").await.at(["triangles", &index.to_string()]).await);
        }
    }
    let mut modified = BTreeSet::new();
    for entry in &diff.modified {
        if entry.index >= base_len || removed.contains(&entry.index) || !modified.insert(entry.index) {
            return Err(MutationApplyError::new("invalid-modify-index", "triangle modification target must exist exactly once and remain present").await.at(["triangles", &entry.index.to_string()]).await);
        }
    }
    let mut length = base_len - removed.len();
    let mut additions: Vec<usize> = diff.added.iter().map(|entry| entry.index).collect();
    additions.sort_unstable();
    let mut previous = None;
    for index in additions {
        if index > length || previous == Some(index) {
            return Err(MutationApplyError::new("invalid-add-index", "triangle addition target must be unique and within the evolving sequence").await.at(["triangles", &index.to_string()]).await);
        }
        previous = Some(index);
        length += 1;
    }
    Ok(())
}

async fn apply_stl_diff_unchecked(diff: &StlDiff, base: &StlSnapshot) -> StlSnapshot {
    let triangles = diff.triangles.as_ref().map_or_else(|| base.triangles.clone(), |value| apply_triangles_diff(&base.triangles, value));
    StlSnapshot { schema: base.schema.clone(), solid_name: diff.solid_name.clone().unwrap_or_else(|| base.solid_name.clone()), triangles }
}

/// 🧭️ Index-keyed state delta between two triangle lists: pairwise-compared over
/// `0..min(len)` (→ `modified`), whichever side is longer supplies the tail (`removed` if BASE is
/// longer, `added` if OTHER is longer) — structurally only one tail kind can ever be non-empty
/// from a single call (the known flat/unkeyed-collection limitation; `field_sweep` below exercises
/// both directions to prove both tail kinds, per the ticket's documented fix pattern).
async fn triangles_between(a: &[StlTriangle], b: &[StlTriangle]) -> StlTrianglesDiff {
    let min_len = a.len().min(b.len());
    let mut modified = Vec::new();
    for i in 0..min_len {
        let d = triangle_between(&a[i], &b[i]);
        if !triangle_diff_is_empty(&d) {
            modified.push(StlTriangleModified { index: i, diff: d.await });
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
async fn absorb_pair(d1: &StlTrianglesDiff, d2: &StlTrianglesDiff) -> StlTrianglesDiff {
    let max_ref =
        d1.removed.iter().copied().chain(d1.modified.iter().map(|m| m.index)).chain(d1.added.iter().map(|a| a.index)).chain(d2.removed.iter().copied()).chain(d2.modified.iter().map(|m| m.index)).chain(d2.added.iter().map(|a| a.index)).max();
    let l1 = max_ref.map(|m| m + 2).unwrap_or(0);

    let base_labels: Vec<Lbl> = (0..l1).map(Lbl::Base).collect();
    let d1_added: Vec<(usize, Lbl)> = d1.added.iter().enumerate().map(|(j, a)| (a.index, Lbl::Added1(j))).collect();
    let mut mid_labels = simulate_labels(base_labels, &d1.removed, &d1_added).await;

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
                        absorb_triangle_diff(&mut combined, (*d2d).clone());
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
                        triangle = apply_triangle_diff(&triangle, d2d).await;
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
async fn absorb_triangles(d1: Option<StlTrianglesDiff>, d2: Option<StlTrianglesDiff>) -> Option<StlTrianglesDiff> {
    let merged = match (d1, d2) {
        (None, None) => return None,
        (Some(d1), None) => return Some(d1),
        (None, Some(d2)) => return Some(d2),
        (Some(d1), Some(d2)) => absorb_pair(&d1, &d2),
    }.await;
    if merged.is_empty().await {
        None
    } else {
        Some(merged)
    }
}
//#endregion 🔖️AbsorbLabels
//#endregion 🔖️TrianglesTriple

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.stl`. `schema` is an identity field and never appears here.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.stl.diff")]
pub struct StlDiff {
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solid_name: Option<String>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub triangles: Option<StlTrianglesDiff>,
}

impl MutationDiff<StlSnapshot> for StlDiff {
    async fn apply(&self, base: &StlSnapshot) -> MutationApplyResult<StlSnapshot> {
        if let Some(diff) = &self.triangles {
            validate_triangles_diff(base.triangles.len(), diff).await?;
        }
        Ok(apply_stl_diff_unchecked(self, base).await)
    }

    /// ➕️ Structural, total, base-free sequential-coalesce (`## Absorb` contract). Scalar
    /// `solid_name`: LWW. `triangles`: see `absorb_triangles`.
    async fn absorb(&mut self, other: Self) {
        if other.solid_name.is_some() {
            self.solid_name = other.solid_name;
        }
        self.triangles = absorb_triangles(self.triangles.take(), other.triangles).await;
    }
}

impl DiffAlgebra<StlSnapshot> for StlDiff {
    /// 🔁️ Diff-level undo, derived generically (correct by construction): the state delta from
    /// `self.apply(base)` back to `base` — `between` is the single source of truth for turning a
    /// state pair into a diff, so `inverse` doesn't duplicate its per-field logic (same pattern
    /// as this ticket's zip/xml precedent).
    async fn inverse(&self, base: &StlSnapshot) -> Self {
        let mutated = apply_stl_diff_unchecked(self, base);
        <Self as DiffAlgebra<StlSnapshot>>::between(&mutated, base).await
    }

    /// 🧭️ State delta (compose `GetXDiff`): `triangles` uses index-pairwise matching (see
    /// `triangles_between`'s doc comment for the single-tail-kind-per-call caveat).
    async fn between(base: &StlSnapshot, other: &StlSnapshot) -> Self {
        let solid_name = (base.solid_name != other.solid_name).then(|| other.solid_name.clone());
        let td = triangles_between(&base.triangles, &other.triangles).await;
        let triangles = if td.is_empty().await { None } else { Some(td) };
        StlDiff { solid_name, triangles }
    }

    async fn is_empty(&self) -> bool {
        self.solid_name.is_none() && self.triangles.as_ref().map_or(true, StlTrianglesDiff::is_empty)
    }
}
//#endregion 🔖️Diff

//#region 🔖️MutationDiffBuilders
/// 🧩 `SetSnapshot`'s diff is the sparse field-by-field `between(base, next)` — no full-replace
/// slot exists on `StlDiff` to short-circuit into.
pub async fn diff_set_snapshot(base: &StlSnapshot, next: &StlSnapshot) -> StlDiff {
    <StlDiff as DiffAlgebra<StlSnapshot>>::between(base, next).await
}
pub async fn diff_set_solid_name(name: &str) -> StlDiff {
    StlDiff { solid_name: Some(name.to_string()), triangles: None }
}
pub async fn diff_insert_triangle(index: usize, triangle: StlTriangle) -> StlDiff {
    StlDiff { solid_name: None, triangles: Some(StlTrianglesDiff { removed: vec![], modified: vec![], added: vec![StlTriangleAdded { index, triangle }] }) }
}
pub async fn diff_remove_triangle(index: usize) -> StlDiff {
    StlDiff { solid_name: None, triangles: Some(StlTrianglesDiff { removed: vec![index], modified: vec![], added: vec![] }) }
}
async fn diff_triangle_field(index: usize, field: StlTriangleDiff) -> StlDiff {
    StlDiff { solid_name: None, triangles: Some(StlTrianglesDiff { removed: vec![], modified: vec![StlTriangleModified { index, diff: field }], added: vec![] }) }
}
pub async fn diff_set_triangle_normal(index: usize, normal: [f64; 3]) -> StlDiff {
    diff_triangle_field(index, StlTriangleDiff { normal: Some(normal), vertices: None }).await
}
pub async fn diff_set_triangle_vertices(index: usize, vertices: [[f64; 3]; 3]) -> StlDiff {
    diff_triangle_field(index, StlTriangleDiff { normal: None, vertices: Some(vertices) }).await
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
pub(crate) async fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
pub(crate) async fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
pub(crate) async fn hex_encode_str(s: &str) -> String {
    hex_encode(s.as_bytes()).await
}
pub(crate) async fn hex_decode_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s).await?).map_err(|e| e.to_string())
}
pub(crate) async fn parse_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}
pub(crate) async fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

/// 🧭️ Bracket-depth-aware split (tracks `[`/`]` only): a top-level `sep` inside nested brackets is
/// never mistaken for a field separator — the whole hand-rolled grammar's parsing primitive (same
/// technique `gif`89a's/`svg`'s hand-rolled `DiffCodec` use).
pub(crate) async fn split_top_level(s: &str, sep: char) -> Vec<&str> {
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
pub(crate) async fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}
//#endregion 🔖️Primitives

//#region 🔖️BinaryPrimitives
/// 🧪️ P2-FG1-FIX: real LEB128-varint-framed binary primitives backing the upgraded `OpBinary`
/// (`../🧬️mutations/🦀️component.rs`) and `DiffCodec` (below) frames — reuses
/// `store::pack_rt::write_varint_u64`/`store::ByteReader` rather than reinventing varint encode/
/// decode (same shape `dxf`'s own `BinaryPrimitives`/`ItemBinaryCodecs` regions use).
/// `pub(crate)` so the mutations sibling reuses these rather than duplicating them a second time.
pub(crate) async fn write_str_bin(out: &mut Vec<u8>, s: &str) {
    store::pack_rt::write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}
pub(crate) async fn read_str_bin(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    let len = reader.read_varint_u64().await.map_err(|e| e.to_string())? as usize;
    String::from_utf8(reader.read_bytes(len).await.map_err(|e| e.to_string())?.to_vec()).map_err(|e| e.to_string())
}
pub(crate) async fn write_f64_bin(out: &mut Vec<u8>, v: f64) {
    out.extend_from_slice(&v.to_le_bytes());
}
pub(crate) async fn read_f64_bin(reader: &mut store::ByteReader<'_>) -> Result<f64, String> {
    reader.read_f64_le().await.map_err(|e| e.to_string())
}
pub(crate) async fn write_option_bin<T>(out: &mut Vec<u8>, opt: &Option<T>, enc: impl FnOnce(&T, &mut Vec<u8>)) {
    match opt {
        None => out.push(0),
        Some(v) => {
            out.push(1);
            enc(v, out);
        }
    }
}
pub(crate) async fn read_option_bin<T>(reader: &mut store::ByteReader<'_>, dec: impl FnOnce(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<Option<T>, String> {
    match reader.read_u8().await.map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(dec(reader)?)),
        other => Err(format!("option binary: unknown tag {other}")),
    }
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️ValueCodecs
/// 📐️ One `[f64; 3]` level — the depth marker `dsl`'s own `Shape::Tuple` printer is missing.
pub(crate) async fn enc_vec3(v: &[f64; 3]) -> String {
    format!("[{},{},{}]", v[0], v[1], v[2])
}
pub(crate) async fn dec_vec3(s: &str) -> Result<[f64; 3], String> {
    let parts = split_top_level(strip_brackets(s).await?, ',').await;
    let [x, y, z] = parts.as_slice() else { return Err(format!("vec3: expected 3 fields, got {}", parts.len())) };
    Ok([parse_f64(x).await?, parse_f64(y).await?, parse_f64(z).await?])
}
/// 📐️ The outer `[[f64; 3]; 3]` level — 3 `enc_vec3`-bracketed vertices inside one more `[...]`.
pub(crate) async fn enc_vertices(vs: &[[f64; 3]; 3]) -> String {
    format!("[{}]", vs.iter().map(enc_vec3).collect::<Vec<_>>().join(","))
}
pub(crate) async fn dec_vertices(s: &str) -> Result<[[f64; 3]; 3], String> {
    let parts = split_top_level(strip_brackets(s).await?, ',').await;
    let [v0, v1, v2] = parts.as_slice() else { return Err(format!("vertices: expected 3 fields, got {}", parts.len())) };
    Ok([dec_vec3(v0).await?, dec_vec3(v1).await?, dec_vec3(v2).await?])
}
pub(crate) async fn enc_triangle(t: &StlTriangle) -> String {
    format!("[{},{}]", enc_vec3(&t.normal), enc_vertices(&t.vertices))
}
pub(crate) async fn dec_triangle(s: &str) -> Result<StlTriangle, String> {
    let parts = split_top_level(strip_brackets(s).await?, ',').await;
    let [normal, vertices] = parts.as_slice() else { return Err(format!("triangle: expected 2 fields, got {}", parts.len())) };
    Ok(StlTriangle { normal: dec_vec3(normal).await?, vertices: dec_vertices(vertices).await? })
}
//#endregion 🔖️ValueCodecs

//#region 🔖️ValueBinaryCodecs
/// 🧪️ P2-FG1-FIX: real recursive binary twins of [`enc_vec3`]/[`enc_vertices`]/[`enc_triangle`]
/// above — genuinely flat (no self-recursion, `StlTriangle` never references itself), so every
/// level is real fixed/varint-framed binary, never an opaque byte-chain.
pub(crate) async fn enc_vec3_bin(v: &[f64; 3], out: &mut Vec<u8>) {
    write_f64_bin(out, v[0]);
    write_f64_bin(out, v[1]);
    write_f64_bin(out, v[2]);
}
pub(crate) async fn dec_vec3_bin(reader: &mut store::ByteReader<'_>) -> Result<[f64; 3], String> {
    Ok([read_f64_bin(reader).await?, read_f64_bin(reader).await?, read_f64_bin(reader).await?])
}
pub(crate) async fn enc_vertices_bin(vs: &[[f64; 3]; 3], out: &mut Vec<u8>) {
    for v in vs {
        enc_vec3_bin(v, out);
    }
}
pub(crate) async fn dec_vertices_bin(reader: &mut store::ByteReader<'_>) -> Result<[[f64; 3]; 3], String> {
    Ok([dec_vec3_bin(reader).await?, dec_vec3_bin(reader).await?, dec_vec3_bin(reader).await?])
}
pub(crate) async fn enc_triangle_bin(t: &StlTriangle, out: &mut Vec<u8>) {
    enc_vec3_bin(&t.normal, out);
    enc_vertices_bin(&t.vertices, out);
}
pub(crate) async fn dec_triangle_bin(reader: &mut store::ByteReader<'_>) -> Result<StlTriangle, String> {
    Ok(StlTriangle { normal: dec_vec3_bin(reader).await?, vertices: dec_vertices_bin(reader).await? })
}
//#endregion 🔖️ValueBinaryCodecs

//#region 🔖️DiffValueCodecs
async fn enc_triangle_diff(d: &StlTriangleDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = &d.normal {
        parts.push(format!("N:{}", enc_vec3(v)));
    }
    if let Some(v) = &d.vertices {
        parts.push(format!("V:{}", enc_vertices(v)));
    }
    format!("[{}]", parts.join(","))
}
async fn dec_triangle_diff(s: &str) -> Result<StlTriangleDiff, String> {
    let inner = strip_brackets(s).await?;
    let mut d = StlTriangleDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("triangle diff: bad entry {entry:?}"))?;
        match tag {
            "N" => d.normal = Some(dec_vec3(val).await?),
            "V" => d.vertices = Some(dec_vertices(val).await?),
            other => return Err(format!("triangle diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}

/// 🧭️ Generic-shaped 3-section `[removed];[modified];[added]` collection-triple printer/parser
/// (same shape `gif`89a's hand-roll uses, ported here for `triangles`).
pub(crate) async fn enc_collection_triple(name: &str, removed: &[usize], modified: &[(usize, String)], added: &[(usize, String)]) -> String {
    let removed = removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = modified.iter().map(|(i, v)| format!("{i}:{v}")).collect::<Vec<_>>().join(",");
    let added = added.iter().map(|(i, v)| format!("{i}:{v}")).collect::<Vec<_>>().join(",");
    format!("{name}{{[{removed}];[{modified}];[{added}]}}")
}
pub(crate) async fn dec_collection_triple(body: &str) -> Result<(Vec<usize>, Vec<(usize, String)>, Vec<(usize, String)>), String> {
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

async fn enc_triangles_diff(d: &StlTrianglesDiff) -> String {
    enc_collection_triple("triangles", &d.removed, &d.modified.iter().map(|m| (m.index, enc_triangle_diff(&m.diff))).collect::<Vec<_>>(), &d.added.iter().map(|a| (a.index, enc_triangle(&a.triangle))).collect::<Vec<_>>()).await
}
async fn dec_triangles_diff(body: &str) -> Result<StlTrianglesDiff, String> {
    let (removed, modified, added) = dec_collection_triple(body).await?;
    Ok(StlTrianglesDiff {
        removed,
        modified: modified.into_iter().map(|(index, enc)| Ok(StlTriangleModified { index, diff: dec_triangle_diff(&enc)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, enc)| Ok(StlTriangleAdded { index, triangle: dec_triangle(&enc)? })).collect::<Result<Vec<_>, String>>()?,
    })
}
//#endregion 🔖️DiffValueCodecs

//#region 🔖️DiffValueBinaryCodecs
/// 🧪️ P2-FG1-FIX: real recursive binary twin of [`enc_triangle_diff`]/[`dec_triangle_diff`] and
/// [`enc_triangles_diff`]/[`dec_triangles_diff`] above — `StlTriangleDiff` has no enum/tri-state
/// field (both `normal`/`vertices` are plain `Option<T>`), so [`write_option_bin`]/
/// [`read_option_bin`] cover both. `StlTrianglesDiff` is the one genuinely variable-length,
/// collection-of-records part of this frame (`removed: Vec<usize>`, `modified: Vec<{index,
/// diff}>`, `added: Vec<{index, triangle}>`) — real varint-counted, recursively-encoded lists,
/// same shape `md`'s own `enc_blocks_diff_bin`/`dec_blocks_diff_bin` uses for its collection
/// triple.
pub(crate) async fn enc_triangle_diff_bin(d: &StlTriangleDiff, out: &mut Vec<u8>) {
    write_option_bin(out, &d.normal, |v, o| enc_vec3_bin(v, o));
    write_option_bin(out, &d.vertices, |v, o| enc_vertices_bin(v, o));
}
pub(crate) async fn dec_triangle_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<StlTriangleDiff, String> {
    let normal = read_option_bin(reader, dec_vec3_bin).await?;
    let vertices = read_option_bin(reader, dec_vertices_bin).await?;
    Ok(StlTriangleDiff { normal, vertices })
}
pub(crate) async fn enc_triangles_diff_bin(d: &StlTrianglesDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    for idx in &d.removed {
        store::pack_rt::write_varint_u64(out, *idx as u64);
    }
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for entry in &d.modified {
        store::pack_rt::write_varint_u64(out, entry.index as u64);
        enc_triangle_diff_bin(&entry.diff, out);
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for entry in &d.added {
        store::pack_rt::write_varint_u64(out, entry.index as u64);
        enc_triangle_bin(&entry.triangle, out);
    }
}
pub(crate) async fn dec_triangles_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<StlTrianglesDiff, String> {
    let removed_count = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(reader.read_varint_u64().await.map_err(|e| e.to_string())? as usize);
    }
    let modified_count = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let index = reader.read_varint_u64().await.map_err(|e| e.to_string())? as usize;
        let diff = dec_triangle_diff_bin(reader).await?;
        modified.push(StlTriangleModified { index, diff });
    }
    let added_count = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().await.map_err(|e| e.to_string())? as usize;
        let triangle = dec_triangle_bin(reader).await?;
        added.push(StlTriangleAdded { index, triangle });
    }
    Ok(StlTrianglesDiff { removed, modified, added })
}
//#endregion 🔖️DiffValueBinaryCodecs

//#region 🔖️TopLevel
async fn print_stl_diff(d: &StlDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.solid_name {
        tokens.push(format!("solid-name={}", hex_encode_str(v)));
    }
    if let Some(v) = &d.triangles {
        tokens.push(enc_triangles_diff(v).await);
    }
    tokens.join(" ")
}
async fn parse_stl_diff(line: &str) -> Result<StlDiff, String> {
    let mut d = StlDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("solid-name=") {
            d.solid_name = Some(hex_decode_str(rest).await?);
        } else if let Some(rest) = token.strip_prefix("triangles{") {
            d.triangles = Some(dec_triangles_diff(rest.strip_suffix('}').ok_or_else(|| "triangles: missing closing brace".to_string())?).await?);
        } else {
            return Err(format!("stl diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffCodec for StlDiff {
    async fn print_diff(&self) -> String {
        print_stl_diff(self).await
    }
    async fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_stl_diff(line).await.map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// 🧪️ P2-FG1-FIX: REAL binary frame (`format u8 | flags u8 | [solid_name] | [triangles]`),
    /// matching `../💾️binary/📡️component.protocol.semio`'s `header fixed 2` + `chain payload
    /// bytes` shape — upgraded from the prior `print_diff().into_bytes()` text-as-binary
    /// shortcut. `flags` is a 2-bit presence mask (bit0=`solid_name`, bit1=`triangles`) since
    /// `StlDiff` has TWO independently optional top-level fields (unlike `MdDiff`'s single
    /// `blocks`, which only needed one `has_value` byte). `StlDiff`'s own field tree has ZERO
    /// self-recursion (`StlTriangleDiff`/`StlTrianglesDiff` never reference `StlDiff` or
    /// themselves) — every present field is real field-by-field binary all the way down
    /// (`enc_triangles_diff_bin` → `enc_triangle_diff_bin`/`enc_triangle_bin` →
    /// `enc_vertices_bin`/`enc_vec3_bin`/`write_f64_bin`), never an opaque byte-chain at the Rust
    /// layer. Only the protocol-DIALECT file (not the Rust code) still frames `triangles`'
    /// payload as one opaque trailing `chain payload bytes`: `removed`/`modified`/`added` are
    /// variable-length VECTORS OF RECORDS, which hits the same `protocol-array-of-records`
    /// `walk_protocol` gap this wave's `dxf`/`md` upgrades independently document (the dialect's
    /// `array-prim`/`record`-block constructs are unexercised anywhere in this codebase and
    /// `Prim::Ref`-adjacent array-of-records framing is the documented, non-blocking
    /// `mechanism_gaps` entry every collection-triple diff hits this wave).
    async fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut flags = 0u8;
        if self.solid_name.is_some() {
            flags |= 0b01;
        }
        if self.triangles.is_some() {
            flags |= 0b10;
        }
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, flags];
        if let Some(name) = &self.solid_name {
            write_str_bin(&mut out, name);
        }
        if let Some(triangles) = &self.triangles {
            enc_triangles_diff_bin(triangles, &mut out);
        }
        Ok(out)
    }
    async fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes).await;
        let _format = reader.read_u8().await.map_err(|e| protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: e.to_string() })?;
        let flags = reader.read_u8().await.map_err(|e| protocol::ProtocolError::Malformed { what: "diff flags", offset: 1, detail: e.to_string() })?;
        let solid_name = if flags & 0b01 != 0 { Some(read_str_bin(&mut reader).await.map_err(|e| protocol::ProtocolError::Malformed { what: "diff solid_name", offset: semio_framework_plugin::resolve_ready(reader.position()) as u64, detail: e })?) } else { None };
        let triangles = if flags & 0b10 != 0 { Some(dec_triangles_diff_bin(&mut reader).await.map_err(|e| protocol::ProtocolError::Malformed { what: "diff triangles", offset: semio_framework_plugin::resolve_ready(reader.position()) as u64, detail: e })?) } else { None };
        Ok(StlDiff { solid_name, triangles })
    }
}
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
pub(crate) async fn demo_diff_cases() -> Vec<StlDiff> {
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
