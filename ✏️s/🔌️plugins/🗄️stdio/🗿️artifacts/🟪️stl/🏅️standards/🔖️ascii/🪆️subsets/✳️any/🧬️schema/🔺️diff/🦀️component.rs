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
use protocol::MutationDiff;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

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
fn apply_triangle_diff(base: &StlTriangle, diff: &StlTriangleDiff) -> StlTriangle {
    StlTriangle {
        normal: diff.normal.unwrap_or(base.normal),
        vertices: diff.vertices.unwrap_or(base.vertices),
    }
}

/// 🧭️ Field-by-field state delta between two triangles occupying the same index slot.
fn triangle_between(a: &StlTriangle, b: &StlTriangle) -> StlTriangleDiff {
    StlTriangleDiff {
        normal: (a.normal != b.normal).then_some(b.normal),
        vertices: (a.vertices != b.vertices).then_some(b.vertices),
    }
}

fn triangle_diff_is_empty(d: &StlTriangleDiff) -> bool {
    d.normal.is_none() && d.vertices.is_none()
}

/// ➕️ LWW field-by-field absorb of one triangle patch into another.
fn absorb_triangle_diff(base: &mut StlTriangleDiff, other: StlTriangleDiff) {
    if other.normal.is_some() { base.normal = other.normal; }
    if other.vertices.is_some() { base.vertices = other.vertices; }
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
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

/// ▶️ Applies the triangles triple: (1) `modified` by BASE index, (2) `removed` by BASE index
/// processed descending so earlier removals don't shift later indices, (3) `added` by FINAL
/// index, ascending, `insert(min(index, len))`.
fn apply_triangles_diff(triangles: &[StlTriangle], diff: &StlTrianglesDiff) -> Vec<StlTriangle> {
    let mut result = triangles.to_vec();
    for m in &diff.modified {
        if let Some(t) = result.get_mut(m.index) {
            *t = apply_triangle_diff(t, &m.diff);
        }
    }
    let mut removed_sorted = diff.removed.clone();
    removed_sorted.sort_unstable_by(|a, b| b.cmp(a));
    removed_sorted.dedup();
    for idx in removed_sorted {
        if idx < result.len() {
            result.remove(idx);
        }
    }
    let mut added_sorted: Vec<&StlTriangleAdded> = diff.added.iter().collect();
    added_sorted.sort_by_key(|a| a.index);
    for a in added_sorted {
        let at = a.index.min(result.len());
        result.insert(at, a.triangle);
    }
    result
}

/// 🧭️ Index-keyed state delta between two triangle lists: pairwise-compared over
/// `0..min(len)` (→ `modified`), whichever side is longer supplies the tail (`removed` if BASE is
/// longer, `added` if OTHER is longer) — structurally only one tail kind can ever be non-empty
/// from a single call (the known flat/unkeyed-collection limitation; `field_sweep` below exercises
/// both directions to prove both tail kinds, per the ticket's documented fix pattern).
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
    let added: Vec<StlTriangleAdded> = (min_len..b.len())
        .map(|i| StlTriangleAdded { index: i, triangle: b[i] })
        .collect();
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
fn simulate_labels(labels: Vec<Lbl>, removed: &[usize], added: &[(usize, Lbl)]) -> Vec<Lbl> {
    let removed_set: HashSet<usize> = removed.iter().copied().collect();
    let mut survivors: Vec<Lbl> = labels
        .into_iter()
        .enumerate()
        .filter(|(i, _)| !removed_set.contains(i))
        .map(|(_, l)| l)
        .collect();
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
fn absorb_pair(d1: &StlTrianglesDiff, d2: &StlTrianglesDiff) -> StlTrianglesDiff {
    let max_ref = d1.removed.iter().copied()
        .chain(d1.modified.iter().map(|m| m.index))
        .chain(d1.added.iter().map(|a| a.index))
        .chain(d2.removed.iter().copied())
        .chain(d2.modified.iter().map(|m| m.index))
        .chain(d2.added.iter().map(|a| a.index))
        .max();
    let l1 = max_ref.map(|m| m + 2).unwrap_or(0);

    let base_labels: Vec<Lbl> = (0..l1).map(Lbl::Base).collect();
    let d1_added: Vec<(usize, Lbl)> = d1.added.iter().enumerate().map(|(j, a)| (a.index, Lbl::Added1(j))).collect();
    let mut mid_labels = simulate_labels(base_labels, &d1.removed, &d1_added);

    // 🔍️ Record each label's MID position — exactly the φ(base_index)/mid_index_of(Added1(j))
    // transport the recipe calls for.
    let mut mid_pos_of_base: HashMap<usize, usize> = HashMap::new();
    let mut mid_pos_of_added1: HashMap<usize, usize> = HashMap::new();
    for (pos, l) in mid_labels.iter().enumerate() {
        match l {
            Lbl::Base(i) => { mid_pos_of_base.insert(*i, pos); }
            Lbl::Added1(j) => { mid_pos_of_added1.insert(*j, pos); }
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
fn absorb_triangles(d1: Option<StlTrianglesDiff>, d2: Option<StlTrianglesDiff>) -> Option<StlTrianglesDiff> {
    let merged = match (d1, d2) {
        (None, None) => return None,
        (Some(d1), None) => return Some(d1),
        (None, Some(d2)) => return Some(d2),
        (Some(d1), Some(d2)) => absorb_pair(&d1, &d2),
    };
    if merged.is_empty() { None } else { Some(merged) }
}
//#endregion 🔖️AbsorbLabels
//#endregion 🔖️TrianglesTriple

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.stl`. `schema` is an identity field and never appears here.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.stl.diff")]
pub struct StlDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solid_name: Option<String>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub triangles: Option<StlTrianglesDiff>,
}

impl MutationDiff<StlSnapshot> for StlDiff {
    fn apply(&self, base: &StlSnapshot) -> StlSnapshot {
        let triangles = match &self.triangles {
            Some(td) => apply_triangles_diff(&base.triangles, td),
            None => base.triangles.clone(),
        };
        StlSnapshot {
            schema: base.schema.clone(),
            solid_name: self.solid_name.clone().unwrap_or_else(|| base.solid_name.clone()),
            triangles,
        }
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
        let mutated = self.apply(base);
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
        self.solid_name.is_none() && self.triangles.as_ref().map_or(true, StlTrianglesDiff::is_empty)
    }
}
//#endregion 🔖️Diff

//#region 🔖️MutationDiffBuilders
/// 🧩 `SetSnapshot`'s diff is the sparse field-by-field `between(base, next)` — no full-replace
/// slot exists on `StlDiff` to short-circuit into.
pub fn diff_set_snapshot(base: &StlSnapshot, next: &StlSnapshot) -> StlDiff {
    <StlDiff as DiffAlgebra<StlSnapshot>>::between(base, next)
}
pub fn diff_set_solid_name(name: &str) -> StlDiff {
    StlDiff { solid_name: Some(name.to_string()), triangles: None }
}
pub fn diff_insert_triangle(index: usize, triangle: StlTriangle) -> StlDiff {
    StlDiff {
        solid_name: None,
        triangles: Some(StlTrianglesDiff { removed: vec![], modified: vec![], added: vec![StlTriangleAdded { index, triangle }] }),
    }
}
pub fn diff_remove_triangle(index: usize) -> StlDiff {
    StlDiff {
        solid_name: None,
        triangles: Some(StlTrianglesDiff { removed: vec![index], modified: vec![], added: vec![] }),
    }
}
fn diff_triangle_field(index: usize, field: StlTriangleDiff) -> StlDiff {
    StlDiff {
        solid_name: None,
        triangles: Some(StlTrianglesDiff { removed: vec![], modified: vec![StlTriangleModified { index, diff: field }], added: vec![] }),
    }
}
pub fn diff_set_triangle_normal(index: usize, normal: [f64; 3]) -> StlDiff {
    diff_triangle_field(index, StlTriangleDiff { normal: Some(normal), vertices: None })
}
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
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
pub(crate) fn hex_encode_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
pub(crate) fn hex_decode_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
pub(crate) fn parse_f64(s: &str) -> Result<f64, String> { s.parse().map_err(|e: std::num::ParseFloatError| e.to_string()) }
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> { s.parse().map_err(|e: std::num::ParseIntError| e.to_string()) }

/// 🧭️ Bracket-depth-aware split (tracks `[`/`]` only): a top-level `sep` inside nested brackets is
/// never mistaken for a field separator — the whole hand-rolled grammar's parsing primitive (same
/// technique `gif`89a's/`svg`'s hand-rolled `DiffCodec` use).
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
pub(crate) fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}
//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs
/// 📐️ One `[f64; 3]` level — the depth marker `dsl`'s own `Shape::Tuple` printer is missing.
pub(crate) fn enc_vec3(v: &[f64; 3]) -> String {
    format!("[{},{},{}]", v[0], v[1], v[2])
}
pub(crate) fn dec_vec3(s: &str) -> Result<[f64; 3], String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z] = parts.as_slice() else { return Err(format!("vec3: expected 3 fields, got {}", parts.len())) };
    Ok([parse_f64(x)?, parse_f64(y)?, parse_f64(z)?])
}
/// 📐️ The outer `[[f64; 3]; 3]` level — 3 `enc_vec3`-bracketed vertices inside one more `[...]`.
pub(crate) fn enc_vertices(vs: &[[f64; 3]; 3]) -> String {
    format!("[{}]", vs.iter().map(enc_vec3).collect::<Vec<_>>().join(","))
}
pub(crate) fn dec_vertices(s: &str) -> Result<[[f64; 3]; 3], String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [v0, v1, v2] = parts.as_slice() else { return Err(format!("vertices: expected 3 fields, got {}", parts.len())) };
    Ok([dec_vec3(v0)?, dec_vec3(v1)?, dec_vec3(v2)?])
}
pub(crate) fn enc_triangle(t: &StlTriangle) -> String {
    format!("[{},{}]", enc_vec3(&t.normal), enc_vertices(&t.vertices))
}
pub(crate) fn dec_triangle(s: &str) -> Result<StlTriangle, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [normal, vertices] = parts.as_slice() else { return Err(format!("triangle: expected 2 fields, got {}", parts.len())) };
    Ok(StlTriangle { normal: dec_vec3(normal)?, vertices: dec_vertices(vertices)? })
}
//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs
fn enc_triangle_diff(d: &StlTriangleDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = &d.normal { parts.push(format!("N:{}", enc_vec3(v))); }
    if let Some(v) = &d.vertices { parts.push(format!("V:{}", enc_vertices(v))); }
    format!("[{}]", parts.join(","))
}
fn dec_triangle_diff(s: &str) -> Result<StlTriangleDiff, String> {
    let inner = strip_brackets(s)?;
    let mut d = StlTriangleDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() { continue; }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("triangle diff: bad entry {entry:?}"))?;
        match tag {
            "N" => d.normal = Some(dec_vec3(val)?),
            "V" => d.vertices = Some(dec_vertices(val)?),
            other => return Err(format!("triangle diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}

/// 🧭️ Generic-shaped 3-section `[removed];[modified];[added]` collection-triple printer/parser
/// (same shape `gif`89a's hand-roll uses, ported here for `triangles`).
pub(crate) fn enc_collection_triple(name: &str, removed: &[usize], modified: &[(usize, String)], added: &[(usize, String)]) -> String {
    let removed = removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = modified.iter().map(|(i, v)| format!("{i}:{v}")).collect::<Vec<_>>().join(",");
    let added = added.iter().map(|(i, v)| format!("{i}:{v}")).collect::<Vec<_>>().join(",");
    format!("{name}{{[{removed}];[{modified}];[{added}]}}")
}
pub(crate) fn dec_collection_triple(body: &str) -> Result<(Vec<usize>, Vec<(usize, String)>, Vec<(usize, String)>), String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("collection: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let parse_entries = |s: &str| -> Result<Vec<(usize, String)>, String> {
        split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("collection entry: bad entry {entry:?}"))?;
            Ok((parse_usize(idx)?, rest.to_string()))
        }).collect()
    };
    Ok((removed, parse_entries(modified_s)?, parse_entries(added_s)?))
}

fn enc_triangles_diff(d: &StlTrianglesDiff) -> String {
    enc_collection_triple(
        "triangles",
        &d.removed,
        &d.modified.iter().map(|m| (m.index, enc_triangle_diff(&m.diff))).collect::<Vec<_>>(),
        &d.added.iter().map(|a| (a.index, enc_triangle(&a.triangle))).collect::<Vec<_>>(),
    )
}
fn dec_triangles_diff(body: &str) -> Result<StlTrianglesDiff, String> {
    let (removed, modified, added) = dec_collection_triple(body)?;
    Ok(StlTrianglesDiff {
        removed,
        modified: modified.into_iter().map(|(index, enc)| Ok(StlTriangleModified { index, diff: dec_triangle_diff(&enc)? })).collect::<Result<Vec<_>, String>>()?,
        added: added.into_iter().map(|(index, enc)| Ok(StlTriangleAdded { index, triangle: dec_triangle(&enc)? })).collect::<Result<Vec<_>, String>>()?,
    })
}
//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel
fn print_stl_diff(d: &StlDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.solid_name { tokens.push(format!("solid-name={}", hex_encode_str(v))); }
    if let Some(v) = &d.triangles { tokens.push(enc_triangles_diff(v)); }
    tokens.join(" ")
}
fn parse_stl_diff(line: &str) -> Result<StlDiff, String> {
    let mut d = StlDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("solid-name=") { d.solid_name = Some(hex_decode_str(rest)?); }
        else if let Some(rest) = token.strip_prefix("triangles{") { d.triangles = Some(dec_triangles_diff(rest.strip_suffix('}').ok_or_else(|| "triangles: missing closing brace".to_string())?)?); }
        else { return Err(format!("stl diff: unknown token {token:?}")); }
    }
    Ok(d)
}

impl protocol::DiffCodec for StlDiff {
    fn print_diff(&self) -> String {
        print_stl_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_stl_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// ⚡️ Binary = the text bytes verbatim (same simplification `WriterDiff`'s/`gif`89a's/`svg`'s
    /// hand-rolled `DiffCodec` use): satisfies every `DiffCodec` law (round-trips, deterministic)
    /// without inventing a second, denser wire format.
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(self.print_diff().into_bytes())
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "diff utf8", offset: 0, detail: e.to_string() })?;
        Self::parse_diff(line).map_err(|e| protocol::ProtocolError::Malformed { what: "diff text", offset: 0, detail: e.to_string() })
    }
}
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec
