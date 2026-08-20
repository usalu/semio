//! 🔺️ TxtDiff — handcrafted sparse diff: line-ending/trailing-newline scalars + an index-keyed
//! `lines` triple. No `snapshot: Option<TxtSnapshot>` full-replace slot anywhere, incl. SetSnapshot
//! (its diff is `TxtDiff::between(base, next)`, field-by-field, same as every other mutation).

use crate::artifacts::txt::schema::snapshot::LineEnding;
use crate::artifacts::txt::TxtSnapshot;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
// 🧭️ `DiffAlgebra` isn't yet on the `protocol` facade's curated re-export list (S1 added the
// trait but the facade wasn't updated — see s1-spine-report.md) so it's reached via the
// still-public `os_spr::command` path instead of touching that framework facade file.
use protocol::os_spr::command::DiffAlgebra;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

//#region 🔖️LinesDiff
/// ➕️ One line added at `index` (a FINAL-state index, per the recipe's `CAdded{index,item}`
/// shape) carrying its full text (lines are opaque strings -- a weak leaf value with no
/// sub-fields of its own, so "diff" of a line is just its replacement text).
///
/// 🧪️ F6: `dsl::DslRecord` — gives this `DslField` so `Vec<TxtLineAdded>` can sit inside a
/// `#[derive(dsl::DslDiff)]` struct's list field (`TxtLinesDiff::added` below).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct TxtLineAdded {
    pub index: usize,
    pub text: String,
}

/// ✏️ Line at BASE index `index` whose text changed to `text`.
///
/// 🧪️ F6: `dsl::DslRecord` — see [`TxtLineAdded`]'s doc comment, same reason.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct TxtLineModified {
    pub index: usize,
    pub text: String,
}

/// 🧮 Index-keyed triple over `TxtSnapshot::lines`. `removed`/`modified` indices refer to the
/// BASE array; `added` indices refer to the FINAL array (apply order: modified, then removed
/// descending, then added ascending clamped to `min(index, len)` -- normative per the recipe).
///
/// 🧪️ F6: `dsl::DslRecord` — gives this `DslField` so `Option<TxtLinesDiff>` can sit inside
/// `TxtDiff` below (`Vec<usize>`/`Vec<TxtLineModified>`/`Vec<TxtLineAdded>` all bind via the
/// `dsl` crate's blanket `Vec<T>` impl, `TxtLineModified`/`TxtLineAdded` via their own
/// `DslRecord` derive just above).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct TxtLinesDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<TxtLineModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<TxtLineAdded>,
}

impl TxtLinesDiff {
    /// 🕳️ No removed/modified/added entries.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }

    /// ▶️ Applies this triple to a base line array. See module docs for apply order.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply(&self, base: &[String]) -> Vec<String> {
        let mut items: Vec<Option<String>> = base.iter().cloned().map(Some).collect();
        for m in &self.modified {
            if let Some(slot) = items.get_mut(m.index) {
                *slot = Some(m.text.clone());
            }
        }
        let removed: HashSet<usize> = self.removed.iter().copied().collect();
        let mut survivors: Vec<String> = items.into_iter().enumerate().filter(|(i, _)| !removed.contains(i)).filter_map(|(_, v)| v).collect();
        let mut added = self.added.clone();
        added.sort_by_key(|a| a.index);
        for a in added {
            let pos = a.index.min(survivors.len());
            survivors.insert(pos, a.text.clone());
        }
        survivors
    }

    /// 🧭️ State delta between two line arrays: pairwise-by-position over `0..min(len)`
    /// (`modified`), base tail (`removed`), other tail (`added`) -- the recipe's "index keys
    /// pairwise by position" `between` rule.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn between(base: &[String], next: &[String]) -> Self {
        let min_len = base.len().min(next.len());
        let mut modified = Vec::new();
        for i in 0..min_len {
            if base[i] != next[i] {
                modified.push(TxtLineModified { index: i, text: next[i].clone() });
            }
        }
        let removed: Vec<usize> = (next.len()..base.len()).collect();
        let added: Vec<TxtLineAdded> = (base.len()..next.len()).map(|i| TxtLineAdded { index: i, text: next[i].clone() }).collect();
        TxtLinesDiff { removed, modified, added }
    }
}

//#region 🔖️AbsorbLabels
/// 🏷️ A structural, base-free label used only inside [`absorb_pair`] to simulate the two-step
/// position transform (base→mid via `d1`, mid→after via `d2`) without ever looking at real line
/// content -- absorb's normative contract is "structural" and "base-free". `Base(i)` traces an
/// original base-array index; `Added1`/`Added2` trace a still-alive entry from `d1.added`/
/// `d2.added` (by its position in that Vec, so we can look its text back up).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Lbl {
    Base(usize),
    Added1(usize),
    Added2(usize),
}

/// ➡️ Structural simulate of [`TxtLinesDiff::apply`]'s position algebra over an abstract label
/// array: remove the given indices, then insert `added` labels ascending at
/// `min(index, current_len)`. Mirrors `apply`'s exact algorithm but carries labels, not text, so
/// it can run without any real snapshot.
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
/// [`TxtLinesDiff`]. Implements the recipe's normative absorb algorithm for an index-keyed
/// collection via label simulation: a virtual base of `Lbl::Base(0..l1)` (large enough to cover
/// every index either diff references -- extra headroom is harmless, `apply`'s own
/// out-of-range indices are graceful no-ops) is walked through `d1`'s remove/insert, then
/// `d2`'s, and the resulting label array is read back into `removed`/`modified`/`added` by
/// which base indices survived (present ⇒ kept, absent ⇒ `r1 ∪ φ⁻¹(r2)`), which `Added1`
/// entries survived `d2` (a `d2`-removal of a `d1`-added item "annihilates the add" -- it's
/// simply absent from the walk, never re-emitted), and each survivor's final position.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_pair(d1: &TxtLinesDiff, d2: &TxtLinesDiff) -> TxtLinesDiff {
    // 🧭️ `l1` (the virtual base's assumed size) must cover every index EITHER diff references,
    // not just `d1`'s -- a `d1` that's empty/a no-op must not collapse the virtual base to zero
    // elements when `d2` still references real base positions `d1` never touched (a real bug
    // this exact formula had until byte-splice fuzz-testing caught its twin in `BinaryDiff`).
    let max_ref =
        d1.removed.iter().copied().chain(d1.modified.iter().map(|m| m.index)).chain(d1.added.iter().map(|a| a.index)).chain(d2.removed.iter().copied()).chain(d2.modified.iter().map(|m| m.index)).chain(d2.added.iter().map(|a| a.index)).max();
    let l1 = max_ref.map(|m| m + 2).unwrap_or(0);

    let base_labels: Vec<Lbl> = (0..l1).map(Lbl::Base).collect();
    let d1_added: Vec<(usize, Lbl)> = d1.added.iter().enumerate().map(|(j, a)| (a.index, Lbl::Added1(j))).collect();
    let mut mid_labels = simulate_labels(base_labels, &d1.removed, &d1_added);

    // 🔍️ Record each label's MID position (before any d2-triggered padding) -- this is exactly
    // the φ(base_index)/mid_index_of(Added1(j)) transport the recipe calls for.
    let mut mid_pos_of_base: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
    let mut mid_pos_of_added1: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
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

    // 📦 `l1` already covers `d2`'s own max reference (computed above); pad is appended at the
    // tail only -- `Vec::push` never disturbs earlier positions, so `mid_pos_of_*` stay valid.
    while mid_labels.len() < l1 {
        mid_labels.push(Lbl::Base(usize::MAX)); // inert padding index, never referenced by mid_pos_of_base
    }

    let d2_added: Vec<(usize, Lbl)> = d2.added.iter().enumerate().map(|(k, a)| (a.index, Lbl::Added2(k))).collect();
    let after_labels = simulate_labels(mid_labels, &d2.removed, &d2_added);

    let d2_modified_at: std::collections::HashMap<usize, &str> = d2.modified.iter().map(|m| (m.index, m.text.as_str())).collect();
    let d1_modified_at: std::collections::HashMap<usize, &str> = d1.modified.iter().map(|m| (m.index, m.text.as_str())).collect();

    let mut present_base: HashSet<usize> = HashSet::new();
    let mut modified = Vec::new();
    let mut added = Vec::new();

    for (pos, l) in after_labels.into_iter().enumerate() {
        match l {
            Lbl::Base(i) if i != usize::MAX => {
                present_base.insert(i);
                let mid_pos = mid_pos_of_base.get(&i).copied();
                let text = mid_pos.and_then(|m| d2_modified_at.get(&m).copied()).or_else(|| d1_modified_at.get(&i).copied());
                if let Some(text) = text {
                    modified.push(TxtLineModified { index: i, text: text.to_string() });
                }
            }
            Lbl::Base(_) => { /* padding survived untouched -- never real, ignore */ }
            Lbl::Added1(j) => {
                let mid_pos = mid_pos_of_added1.get(&j).copied();
                let base_text = &d1.added[j].text;
                let text = mid_pos.and_then(|m| d2_modified_at.get(&m).copied()).unwrap_or(base_text.as_str());
                added.push(TxtLineAdded { index: pos, text: text.to_string() });
            }
            Lbl::Added2(k) => {
                added.push(TxtLineAdded { index: pos, text: d2.added[k].text.clone() });
            }
        }
    }

    let removed: Vec<usize> = (0..l1).filter(|i| !present_base.contains(i)).collect();
    TxtLinesDiff { removed, modified, added }
}
//#endregion 🔖️AbsorbLabels
//#endregion 🔖️LinesDiff

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.txt`. Every mutable field is `Option<T>` (present = changed); `lines`
/// is the one owned collection, an `Option<TxtLinesDiff>` triple.
///
/// 🧪️ F6: `dsl::DslDiff` derive added — emits `protocol::DiffCodec` (print_diff/parse_diff/
/// encode_diff/decode_diff) from the same `RecordSpec` machinery `DslRecord` uses. Classified
/// DERIVE per `f6-recon-report.md` §3's unified decision rule: no field here is `Option<Option<
/// _>>` (every field is a single-layer `Option<T>` meaning "changed", never a tri-state
/// nullable — `lines` composes VIA a triple, it does not itself carry removal-vs-absence), and
/// the only enum in the walk (`LineEnding`) is unit-variant-only, so it binds via `DslScalar`
/// (see the snapshot module) rather than blocking the derive like a data-carrying enum would.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema, dsl::DslDiff)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.txt.diff")]
pub struct TxtDiff {
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trailing_newline: Option<bool>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_ending: Option<LineEnding>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lines: Option<TxtLinesDiff>,
}

impl MutationDiff<TxtSnapshot> for TxtDiff {
    async fn apply(&self, base: &TxtSnapshot) -> MutationApplyResult<TxtSnapshot> {
        if let Some(lines) = &self.lines {
            validate_txt_lines(base.lines.len(), lines)?;
        }
        Ok(TxtSnapshot {
            schema: base.schema.clone(),
            lines: match &self.lines {
                Some(ld) => ld.apply(&base.lines),
                None => base.lines.clone(),
            },
            trailing_newline: self.trailing_newline.unwrap_or(base.trailing_newline),
            line_ending: self.line_ending.unwrap_or(base.line_ending),
        })
    }

    /// ➕️ Sequential-coalesce only (see trait docs): `self` is base→mid, `other` is mid→after.
    /// Scalars are LWW; `lines` composes via [`absorb_pair`]'s structural index-transport.
    async fn absorb(&mut self, other: Self) {
        if let Some(tn) = other.trailing_newline {
            self.trailing_newline = Some(tn);
        }
        if let Some(le) = other.line_ending {
            self.line_ending = Some(le);
        }
        self.lines = match (self.lines.take(), other.lines) {
            (None, None) => None,
            (Some(l1), None) => Some(l1),
            (None, Some(l2)) => Some(l2),
            (Some(l1), Some(l2)) => {
                let merged = absorb_pair(&l1, &l2);
                if merged.is_empty() {
                    None
                } else {
                    Some(merged)
                }
            }
        };
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_txt_lines(base_len: usize, diff: &TxtLinesDiff) -> MutationApplyResult<()> {
    let mut removed = HashSet::new();
    for &index in &diff.removed {
        if index >= base_len {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "removed line index is outside the base snapshot").await.at(["lines", "removed"]));
        }
        if !removed.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "line is removed more than once").await.at(["lines", "removed"]));
        }
    }
    let mut modified = HashSet::new();
    for entry in &diff.modified {
        if entry.index >= base_len {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "modified line does not exist").await.at(["lines", "modified"]));
        }
        if !modified.insert(entry.index) || removed.contains(&entry.index) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "line cannot be both removed and modified").await.at(["lines", "modified"]));
        }
    }
    let final_len = base_len.saturating_sub(diff.removed.len()).saturating_add(diff.added.len());
    let mut additions = HashSet::new();
    for entry in &diff.added {
        if entry.index > final_len || !additions.insert(entry.index) {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "added line index is invalid or duplicated").await.at(["lines", "added"]));
        }
    }
    Ok(())
}

impl DiffAlgebra<TxtSnapshot> for TxtDiff {
    /// 🔁️ `self`'s pre-image undo, expressed via `apply`+`between` (both already proven
    /// correct against `TxtSnapshot`): `next = self.apply(base)`, so `between(next, base)` is
    /// by definition the diff that restores `base` from `next`.
    async fn inverse(&self, base: &TxtSnapshot) -> Self {
        let next = self.apply(base).await.unwrap();
        Self::between(&next, base).await
    }

    async fn between(base: &TxtSnapshot, other: &TxtSnapshot) -> Self {
        let trailing_newline = if base.trailing_newline != other.trailing_newline { Some(other.trailing_newline) } else { None };
        let line_ending = if base.line_ending != other.line_ending { Some(other.line_ending) } else { None };
        let lines_diff = TxtLinesDiff::between(&base.lines, &other.lines);
        let lines = if lines_diff.is_empty() { None } else { Some(lines_diff) };
        TxtDiff { trailing_newline, line_ending, lines }
    }

    async fn is_empty(&self) -> bool {
        self.trailing_newline.is_none() && self.line_ending.is_none() && self.lines.as_ref().map_or(true, TxtLinesDiff::is_empty)
    }
}

/// 🧩 Builds the sparse field-by-field diff for a `SetSnapshot` mutation -- no full-replace
/// slot, same `between` machinery every other mutation's diff ultimately composes from.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &TxtSnapshot, snapshot: &TxtSnapshot) -> TxtDiff {
    TxtDiff::between(base, snapshot)
}
//#endregion 🔖️Diff

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn lines(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[semio_framework_async_macros::async_test]
    async fn insert_then_remove_before_matches_canonical_shape() {
        // Insert("f") at 2, then Remove(0) — canonical case: {removed:[0], added:[(1,f)]}.
        let d1 = TxtDiff { lines: Some(TxtLinesDiff { removed: vec![], modified: vec![], added: vec![TxtLineAdded { index: 2, text: "f".into() }] }), ..Default::default() };
        let d2 = TxtDiff { lines: Some(TxtLinesDiff { removed: vec![0], modified: vec![], added: vec![] }), ..Default::default() };
        let mut merged = d1.clone();
        merged.absorb(d2.clone());
        let ld = merged.lines.clone().expect("lines diff present");
        assert_eq!(ld.removed, vec![0]);
        assert_eq!(ld.added, vec![TxtLineAdded { index: 1, text: "f".into() }]);
        assert!(ld.modified.is_empty());

        let base = TxtSnapshot { lines: lines(&["a", "b", "c", "d"]), ..Default::default() };
        let sequential = {
            let mid = d1.apply(&base).await.unwrap();
            d2.apply(&mid).await.unwrap()
        };
        assert_eq!(merged.apply(&base).await.unwrap(), sequential);
    }

    #[semio_framework_async_macros::async_test]
    async fn insert_insert_same_index_both_survive() {
        let d1 = TxtDiff { lines: Some(TxtLinesDiff { removed: vec![], modified: vec![], added: vec![TxtLineAdded { index: 2, text: "f".into() }] }), ..Default::default() };
        let d2 = TxtDiff { lines: Some(TxtLinesDiff { removed: vec![], modified: vec![], added: vec![TxtLineAdded { index: 2, text: "g".into() }] }), ..Default::default() };
        let mut merged = d1.clone();
        merged.absorb(d2.clone());
        let base = TxtSnapshot { lines: lines(&["a", "b", "c", "d"]), ..Default::default() };
        let sequential = {
            let mid = d1.apply(&base).await.unwrap();
            d2.apply(&mid).await.unwrap()
        };
        assert_eq!(merged.apply(&base).await.unwrap(), sequential);
        assert!(sequential.lines.contains(&"f".to_string()) && sequential.lines.contains(&"g".to_string()));
    }

    #[semio_framework_async_macros::async_test]
    async fn add_then_set_field_patches_into_added() {
        let d1 = TxtDiff { lines: Some(TxtLinesDiff { removed: vec![], modified: vec![], added: vec![TxtLineAdded { index: 1, text: "f".into() }] }), ..Default::default() };
        let d2 = TxtDiff { lines: Some(TxtLinesDiff { removed: vec![], modified: vec![TxtLineModified { index: 1, text: "v".into() }], added: vec![] }), ..Default::default() };
        let mut merged = d1.clone();
        merged.absorb(d2.clone());
        let ld = merged.lines.clone().expect("lines diff present");
        assert!(ld.modified.is_empty(), "patched value should live in the added entry, not a separate modified entry");
        assert_eq!(ld.added, vec![TxtLineAdded { index: 1, text: "v".into() }]);

        let base = TxtSnapshot { lines: lines(&["a", "b", "c"]), ..Default::default() };
        let sequential = {
            let mid = d1.apply(&base).await.unwrap();
            d2.apply(&mid).await.unwrap()
        };
        assert_eq!(merged.apply(&base).await.unwrap(), sequential);
    }

    #[semio_framework_async_macros::async_test]
    async fn modify_then_remove_drops_the_modify() {
        let d1 = TxtDiff { lines: Some(TxtLinesDiff { removed: vec![], modified: vec![TxtLineModified { index: 0, text: "m".into() }], added: vec![] }), ..Default::default() };
        let d2 = TxtDiff { lines: Some(TxtLinesDiff { removed: vec![0], modified: vec![], added: vec![] }), ..Default::default() };
        let mut merged = d1.clone();
        merged.absorb(d2.clone());
        let ld = merged.lines.clone().expect("lines diff present");
        assert_eq!(ld.removed, vec![0]);
        assert!(ld.modified.is_empty());

        let base = TxtSnapshot { lines: lines(&["a", "b"]), ..Default::default() };
        let sequential = {
            let mid = d1.apply(&base).await.unwrap();
            d2.apply(&mid).await.unwrap()
        };
        assert_eq!(merged.apply(&base).await.unwrap(), sequential);
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_associative_over_a_triple() {
        let base = TxtSnapshot { lines: lines(&["a", "b", "c"]), ..Default::default() };
        let d1 = TxtDiff { lines: Some(TxtLinesDiff { removed: vec![1], modified: vec![], added: vec![] }), ..Default::default() };
        let d2 = TxtDiff { lines: Some(TxtLinesDiff { removed: vec![], modified: vec![], added: vec![TxtLineAdded { index: 0, text: "x".into() }] }), ..Default::default() };
        let d3 = TxtDiff { trailing_newline: Some(true), ..Default::default() };

        let mut left = d1.clone();
        left.absorb(d2.clone());
        left.absorb(d3.clone());

        let mut mid = d2.clone();
        mid.absorb(d3.clone());
        let mut right = d1.clone();
        right.absorb(mid);

        assert_eq!(left.apply(&base).await.unwrap(), right.apply(&base).await.unwrap());
        let sequential = {
            let s1 = d1.apply(&base).await.unwrap();
            let s2 = d2.apply(&s1).await.unwrap();
            d3.apply(&s2).await.unwrap()
        };
        assert_eq!(left.apply(&base).await.unwrap(), sequential);
    }

    #[semio_framework_async_macros::async_test]
    async fn between_roundtrip_synthetic() {
        let a = TxtSnapshot { lines: lines(&["a", "b", "c"]), trailing_newline: true, line_ending: LineEnding::Lf, ..Default::default() };
        let b = TxtSnapshot { lines: lines(&["a", "x", "c", "d"]), trailing_newline: false, line_ending: LineEnding::CrLf, ..Default::default() };
        assert_eq!(TxtDiff::between(&a, &b).apply(&a).unwrap(), b);
        assert_eq!(TxtDiff::between(&b, &a).apply(&b).unwrap(), a);
        assert!(TxtDiff::between(&a, &a).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn inverse_diff_level_roundtrip() {
        let base = TxtSnapshot { lines: lines(&["a", "b"]), trailing_newline: false, line_ending: LineEnding::Lf, ..Default::default() };
        let d = TxtDiff { lines: Some(TxtLinesDiff { removed: vec![0], modified: vec![], added: vec![TxtLineAdded { index: 0, text: "z".into() }] }), trailing_newline: Some(true), line_ending: Some(LineEnding::CrLf) };
        let next = d.apply(&base).await.unwrap();
        let inv = d.inverse(&base);
        assert_eq!(inv.apply(&next).unwrap(), base);
    }

    /// 🧪️ F6: `DiffCodec` round-trip laws (derived via `dsl::DslDiff`) — exercises the empty
    /// diff, scalar-only changes, and every one of `TxtLinesDiff`'s `removed`/`modified`/`added`
    /// sections populated simultaneously (a real `between()` result can only ever populate
    /// `modified`+`removed` OR `modified`+`added`, per `TxtLinesDiff::between`'s own base-tail/
    /// other-tail algorithm above -- so this test also directly constructs one diff exercising
    /// all three sections at once, plus a genuine `between()` result for good measure).
    #[semio_framework_async_macros::async_test]
    async fn diff_codec_text_binary_roundtrip_law() {
        use protocol::DiffCodec;
        let a = TxtSnapshot { lines: lines(&["a", "b", "c"]), trailing_newline: true, line_ending: LineEnding::Lf, ..Default::default() };
        let b = TxtSnapshot { lines: lines(&["a", "x", "c", "d"]), trailing_newline: false, line_ending: LineEnding::CrLf, ..Default::default() };
        let cases = vec![
            TxtDiff::default(),
            TxtDiff { trailing_newline: Some(true), line_ending: Some(LineEnding::CrLf), lines: None },
            TxtDiff {
                trailing_newline: Some(false),
                line_ending: Some(LineEnding::Lf),
                lines: Some(TxtLinesDiff {
                    removed: vec![0, 2],
                    modified: vec![TxtLineModified { index: 1, text: "changed".into() }],
                    added: vec![TxtLineAdded { index: 0, text: "new-head".into() }, TxtLineAdded { index: 3, text: "new-tail".into() }],
                }),
            },
            TxtDiff::between(&a, &b).await,
        ];
        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.await.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = TxtDiff::parse_diff(&printed).await.unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch for {d:?} (printed {printed:?})");

            let encoded = d.encode_diff().await.unwrap_or_else(|e| panic!("encode_diff({d:?}) failed: {e}"));
            let decoded = TxtDiff::decode_diff(&encoded).await.unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch for {d:?}");
        }
    }

    //#region 🔖️DiffGrammarConformanceLaw
    /// 🧪️ P2-P3: `dsl::parse_grammar` + `dsl::Recognizer::compile` + `.recognize` against REAL
    /// `print_diff` output -- the empty diff, a scalar-only diff, and a diff exercising every one
    /// of `TxtLinesDiff`'s `removed`/`modified`/`added` sections at once (a real `between()`
    /// result can only ever populate `modified`+`removed` OR `modified`+`added` at a time, per
    /// `TxtLinesDiff::between`'s own base-tail/other-tail algorithm above, so this also directly
    /// constructs one diff exercising all three sections simultaneously, plus a genuine
    /// `between()` result for good measure -- same case list `diff_codec_text_binary_roundtrip_law`
    /// already uses).
    #[semio_framework_async_macros::async_test]
    async fn diff_grammar_conformance_law() {
        use protocol::DiffCodec;
        let grammar_text = crate::artifacts::txt::schema::diff::text::COMPONENT_GRAMMAR_SEMIO;
        let grammar = dsl::parse_grammar(grammar_text).expect("parse diff grammar");
        let recognizer = dsl::Recognizer::compile(&grammar);

        let a = TxtSnapshot { lines: lines(&["a", "b", "c"]), trailing_newline: true, line_ending: LineEnding::Lf, ..Default::default() };
        let b = TxtSnapshot { lines: lines(&["a", "x", "c", "d"]), trailing_newline: false, line_ending: LineEnding::CrLf, ..Default::default() };
        let cases = vec![
            TxtDiff::default(),
            TxtDiff { trailing_newline: Some(true), line_ending: Some(LineEnding::CrLf), lines: None },
            TxtDiff {
                trailing_newline: Some(false),
                line_ending: Some(LineEnding::Lf),
                lines: Some(TxtLinesDiff {
                    removed: vec![0, 2],
                    modified: vec![TxtLineModified { index: 1, text: "changed".into() }],
                    added: vec![TxtLineAdded { index: 0, text: "new-head".into() }, TxtLineAdded { index: 3, text: "new-tail".into() }],
                }),
            },
            TxtDiff::between(&a, &b).await,
        ];
        for d in cases {
            let printed = d.print_diff();
            let ok = recognizer.recognize(&printed).unwrap_or_else(|e| panic!("recognize({printed:?}) errored: {e:?}"));
            assert!(ok, "diff grammar must recognize real print_diff output {printed:?} for {d:?}");
        }
    }
    //#endregion 🔖️DiffGrammarConformanceLaw
}
//#endregion 🧪️Tests
