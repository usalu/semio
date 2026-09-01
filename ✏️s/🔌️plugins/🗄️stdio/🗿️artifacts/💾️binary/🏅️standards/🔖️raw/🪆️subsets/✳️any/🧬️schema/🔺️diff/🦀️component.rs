//! 🔺️ BinaryDiff — a splice list, not a full-replace. `bytes: Vec<u8>` stays as the snapshot
//! (the recipe's one documented "format IS bytes" exception), but the diff is structural: a
//! list of `ByteSplice{offset, remove_len, insert}` ranges, each range treated as its own
//! index-transport "key" (the recipe's own novel design point for this one artifact, since its
//! "collection" is raw byte ranges rather than typed items).

use crate::artifacts::binary::BinarySnapshot;
use protocol::MutationDiff;
// 🧭️ `DiffAlgebra` isn't yet on the `protocol` facade's curated re-export list (S1 added the
// trait but the facade wasn't updated — see s1-spine-report.md) so it's reached via the
// still-public `os_spr::command` path instead of touching that framework facade file.
use protocol::os_spr::command::DiffAlgebra;
use schema::ArtifactSchema;

//#region 🔖️Splice
/// ✂️ One byte-range edit against the BASE array: replace `[offset, offset+remove_len)` with
/// `insert`. `offset`/`remove_len` are BASE-relative (never re-based against a prior splice in
/// the same diff -- see [`apply`](BinaryDiff::apply)'s normative processing order).
///
/// 🧪️ F6-PILOT: `dsl::DslRecord` — gives this `DslField` so `Vec<ByteSplice>` can sit inside a
/// `#[derive(dsl::DslDiff)]` struct's list field (`BinaryDiff::splices` below).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct ByteSplice {
    pub offset: usize,
    pub remove_len: usize,
    #[dsl(base64)]
    pub insert: Vec<u8>,
}
//#endregion 🔖️Splice

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.binary`: an ordered splice list.
///
/// **Apply order (normative):** splices are processed in DESCENDING `offset` order against the
/// SAME base buffer. Editing at a higher offset never shifts byte positions below it (an
/// insert/remove at position P only moves bytes at/after P), so processing high-to-low means
/// every splice's `offset` is still valid against the base at the moment it's applied --
/// `offset`/`remove_len` are never re-interpreted against a partially-mutated buffer.
/// Out-of-range offsets/lengths are rejected by `MutationDiff::apply`; the unchecked helper is
/// reserved for the algebra's total absorb/inverse machinery after a valid diff is established.
/// 🧪️ F6-PILOT: `dsl::DslDiff` derive added — emits `protocol::DiffCodec` (print_diff/parse_diff/
/// encode_diff/decode_diff) from the same `RecordSpec` machinery `DslRecord` uses. `BinaryDiff`
/// is a plain struct with one `Vec<ByteSplice>` field (`ByteSplice` itself `DslRecord`-derived
/// above) — the derive's struct-only restriction is satisfied trivially here.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, dsl::DslDiff)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.binary.diff")]
pub struct BinaryDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub splices: Vec<ByteSplice>,
}

impl MutationDiff<BinarySnapshot> for BinaryDiff {
    fn apply(&self, base: &BinarySnapshot) -> protocol::MutationApplyResult<BinarySnapshot> {
        validate_binary_diff(self, base)?;
        Ok(apply_binary_diff_unchecked(self, base))
    }

    /// ➕️ Sequential-coalesce absorb via [`absorb_splices`]'s byte-range index-transport.
    fn absorb(&mut self, other: Self) {
        self.splices = absorb_splices(&self.splices, &other.splices);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_binary_diff(diff: &BinaryDiff, base: &BinarySnapshot) -> protocol::MutationApplyResult<()> {
    let mut previous = None;
    for (position, splice) in diff.splices.iter().enumerate() {
        if splice.offset > base.bytes.len() {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-index", "byte splice offset is outside the base buffer"));
        }
        if splice.remove_len > base.bytes.len() - splice.offset {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-range", "byte splice removal exceeds the base buffer"));
        }
        if previous == Some(splice.offset) || diff.splices[..position].iter().any(|prior| prior.offset == splice.offset) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "byte splice offset is repeated"));
        }
        previous = Some(splice.offset);
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_binary_diff_unchecked(diff: &BinaryDiff, base: &BinarySnapshot) -> BinarySnapshot {
    let mut bytes = base.bytes.clone();
    let mut splices = diff.splices.clone();
    splices.sort_by(|a, b| b.offset.cmp(&a.offset));
    for s in splices {
        let start = s.offset;
        let end = s.offset + s.remove_len;
        bytes.splice(start..end, s.insert.iter().copied());
    }
    BinarySnapshot { schema: base.schema.clone(), bytes }
}

impl DiffAlgebra<BinarySnapshot> for BinaryDiff {
    fn inverse(&self, base: &BinarySnapshot) -> Self {
        let next = apply_binary_diff_unchecked(self, base);
        Self::between(&next, base)
    }

    /// 🧭️ Minimal common-prefix/common-suffix splice: a single `ByteSplice` covering exactly
    /// the differing middle region (empty splice list iff `base.bytes == other.bytes`).
    fn between(base: &BinarySnapshot, other: &BinarySnapshot) -> Self {
        let a = &base.bytes;
        let b = &other.bytes;
        let mut prefix = 0usize;
        while prefix < a.len() && prefix < b.len() && a[prefix] == b[prefix] {
            prefix += 1;
        }
        let mut suffix = 0usize;
        while suffix < a.len() - prefix && suffix < b.len() - prefix && a[a.len() - 1 - suffix] == b[b.len() - 1 - suffix] {
            suffix += 1;
        }
        let remove_len = a.len() - prefix - suffix;
        let insert = b[prefix..b.len() - suffix].to_vec();
        let splices = if remove_len == 0 && insert.is_empty() { vec![] } else { vec![ByteSplice { offset: prefix, remove_len, insert }] };
        BinaryDiff { splices }
    }

    fn is_empty(&self) -> bool {
        self.splices.is_empty()
    }
}

//#region 🔖️AbsorbLabels
/// 🏷️ Structural, base-free per-BYTE label used only inside [`absorb_splices`] to simulate the
/// two-step position transform, exactly analogous to `TxtLinesDiff`'s label simulation but at
/// byte (not line) granularity -- each splice's affected `[offset, offset+remove_len)` range is
/// decomposed into individual `Base(i)` labels (one per covered index) so overlapping/adjacent
/// splices across `d1`/`d2` compose correctly.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Lbl {
    Base(usize),
    New(u8),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn simulate_labels(labels: Vec<Lbl>, removed: &[usize], added: &[(usize, Lbl)]) -> Vec<Lbl> {
    let removed_set: std::collections::HashSet<usize> = removed.iter().copied().collect();
    let mut survivors: Vec<Lbl> = labels.into_iter().enumerate().filter(|(i, _)| !removed_set.contains(i)).map(|(_, l)| l).collect();
    let mut added_sorted = added.to_vec();
    added_sorted.sort_by_key(|(idx, _)| *idx);
    for (idx, label) in added_sorted {
        let pos = idx.min(survivors.len());
        survivors.insert(pos, label);
    }
    survivors
}

/// 🗑️ All BASE-relative indices removed by a splice list (order-independent; each splice's
/// range is always relative to the shared base, never to a prior splice's result in the same
/// list -- that's the whole point of `apply`'s descending-offset processing order).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn splice_removed_indices(splices: &[ByteSplice]) -> Vec<usize> {
    let mut out = Vec::new();
    for s in splices {
        for i in s.offset..(s.offset + s.remove_len) {
            out.push(i);
        }
    }
    out
}

/// ➕️ Per-byte insert targets in FINAL-space (positions in the array that results from applying
/// every splice in `splices` to its base) -- NOT the raw `offset` values. Splices are walked
/// ascending by offset, tracking a running `delta` (net length change from every EARLIER
/// splice's `insert.len() - remove_len`) so a later splice's insert lands after everything an
/// earlier sibling splice already inserted, exactly mirroring how `apply`'s own descending
/// `Vec::splice` calls accumulate. Using bare `offset + k` here silently reorders sibling
/// inserts within one absorbed diff whenever it has ≥2 splices (caught by fuzz-testing in the
/// scratch crate this diff's tests were validated against — see `deviations` in the F1 report).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn splice_added_targets(splices: &[ByteSplice]) -> Vec<(usize, Lbl)> {
    let mut sorted: Vec<&ByteSplice> = splices.iter().collect();
    sorted.sort_by_key(|s| s.offset);
    let mut out = Vec::new();
    let mut delta: i64 = 0;
    for s in sorted {
        let base_target = (s.offset as i64 + delta).max(0) as usize;
        for (k, byte) in s.insert.iter().enumerate() {
            out.push((base_target + k, Lbl::New(*byte)));
        }
        delta += s.insert.len() as i64 - s.remove_len as i64;
    }
    out
}

/// ➕️ Absorbs `d1` (base→mid) then `d2` (mid→after) splice lists into a single base→after
/// splice list, simulated exactly like the line-diff case (per-byte instead of per-line
/// labels), then the resulting label array is run-length-encoded back into a minimal ordered
/// `ByteSplice` list.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_splices(d1: &[ByteSplice], d2: &[ByteSplice]) -> Vec<ByteSplice> {
    // 🧭️ `l1` (the virtual base's assumed size) must cover every index EITHER diff references,
    // not just `d1`'s -- a `d1` that's empty/a no-op must not collapse the virtual base to zero
    // elements when `d2` still references real base positions `d1` never touched.
    let max_ref = d1.iter().chain(d2.iter()).map(|s| s.offset + s.remove_len.max(s.insert.len())).max();
    let l1 = max_ref.map(|m| m + 8).unwrap_or(0);

    let base_labels: Vec<Lbl> = (0..l1).map(Lbl::Base).collect();
    let d1_removed = splice_removed_indices(d1);
    let d1_added = splice_added_targets(d1);
    let mut mid_labels = simulate_labels(base_labels, &d1_removed, &d1_added);
    while mid_labels.len() < l1 {
        mid_labels.push(Lbl::Base(usize::MAX)); // inert padding, tail-appended
    }

    let d2_removed = splice_removed_indices(d2);
    let d2_added = splice_added_targets(d2);
    let after_labels = simulate_labels(mid_labels, &d2_removed, &d2_added);

    // 🧵️ Run-length-encode the after-label array back into a minimal ordered splice list: walk
    // left to right, tracking the next expected surviving base index; a gap means those base
    // indices were removed, and any `New` labels seen since the last `Base` are the insert run
    // anchored at that gap (or appended at the very end).
    let mut splices = Vec::new();
    let mut expected = 0usize;
    let mut pending_insert: Vec<u8> = Vec::new();
    for l in after_labels {
        match l {
            Lbl::Base(i) if i != usize::MAX => {
                if i > expected || !pending_insert.is_empty() {
                    splices.push(ByteSplice { offset: expected, remove_len: i.saturating_sub(expected), insert: std::mem::take(&mut pending_insert) });
                }
                expected = i + 1;
            }
            Lbl::Base(_) => { /* padding, never real */ }
            Lbl::New(byte) => pending_insert.push(byte),
        }
    }
    if expected < l1 || !pending_insert.is_empty() {
        splices.push(ByteSplice { offset: expected, remove_len: l1.saturating_sub(expected), insert: pending_insert });
    }
    splices
}
//#endregion 🔖️AbsorbLabels
//#endregion 🔖️Diff

/// 🧩 Builds the sparse field-by-field diff for a `SetSnapshot` mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &BinarySnapshot, snapshot: &BinarySnapshot) -> BinaryDiff {
    BinaryDiff::between(base, snapshot)
}

/// 🧪️ P2-P3: representative `BinaryDiff` cases (empty, single-splice, multi-splice incl. a
/// zero-length no-op splice) -- single source of truth shared by `diff_codec_text_binary_
/// roundtrip_law` below AND the new `diff_grammar_conformance_law`/`protocol_walk_law`
/// conformance tests in `⚙️engine/🦀️component.rs`, per CLAUDE.md (no duplicated literal case lists).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<BinaryDiff> {
    vec![
        BinaryDiff::default(),
        BinaryDiff { splices: vec![ByteSplice { offset: 1, remove_len: 2, insert: vec![9, 9, 9] }] },
        BinaryDiff { splices: vec![ByteSplice { offset: 0, remove_len: 0, insert: vec![] }, ByteSplice { offset: 5, remove_len: 1, insert: vec![0xAA, 0xBB] }] },
    ]
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn insert_then_remove_before_matches_canonical_shape() {
        // Insert(0xAA) at offset 2, then Remove 1 byte at offset 0 -- byte-level analog of the
        // line-diff canonical case: {removed:[0], added:[(1,0xAA)]}.
        let d1 = vec![ByteSplice { offset: 2, remove_len: 0, insert: vec![0xAA] }];
        let d2 = vec![ByteSplice { offset: 0, remove_len: 1, insert: vec![] }];
        let merged = absorb_splices(&d1, &d2);

        let base = BinarySnapshot { bytes: vec![1, 2, 3, 4], ..Default::default() };
        let mid = BinaryDiff { splices: d1.clone() }.apply(&base).unwrap();
        let after = BinaryDiff { splices: d2.clone() }.apply(&mid).unwrap();
        assert_eq!(BinaryDiff { splices: merged }.apply(&base).unwrap(), after);
    }

    #[semio_framework_async_macros::async_test]
    async fn insert_insert_same_offset_both_survive() {
        let d1 = vec![ByteSplice { offset: 2, remove_len: 0, insert: vec![0xAA] }];
        let d2 = vec![ByteSplice { offset: 2, remove_len: 0, insert: vec![0xBB] }];
        let merged = absorb_splices(&d1, &d2);

        let base = BinarySnapshot { bytes: vec![1, 2, 3, 4], ..Default::default() };
        let mid = BinaryDiff { splices: d1.clone() }.apply(&base).unwrap();
        let after = BinaryDiff { splices: d2.clone() }.apply(&mid).unwrap();
        assert_eq!(BinaryDiff { splices: merged }.apply(&base).unwrap(), after);
        assert!(after.bytes.windows(2).any(|w| w == [0xBB, 0xAA]) || after.bytes.contains(&0xAA) && after.bytes.contains(&0xBB));
    }

    #[semio_framework_async_macros::async_test]
    async fn modify_then_remove_drops_the_modify() {
        let d1 = vec![ByteSplice { offset: 0, remove_len: 1, insert: vec![0xFF] }];
        let d2 = vec![ByteSplice { offset: 0, remove_len: 1, insert: vec![] }];
        let merged = absorb_splices(&d1, &d2);

        let base = BinarySnapshot { bytes: vec![1, 2, 3], ..Default::default() };
        let mid = BinaryDiff { splices: d1.clone() }.apply(&base).unwrap();
        let after = BinaryDiff { splices: d2.clone() }.apply(&mid).unwrap();
        assert_eq!(BinaryDiff { splices: merged }.apply(&base).unwrap(), after);
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_associative_over_a_triple() {
        let base = BinarySnapshot { bytes: vec![10, 20, 30, 40, 50], ..Default::default() };
        let d1 = BinaryDiff { splices: vec![ByteSplice { offset: 1, remove_len: 1, insert: vec![] }] };
        let d2 = BinaryDiff { splices: vec![ByteSplice { offset: 0, remove_len: 0, insert: vec![99] }] };
        let d3 = BinaryDiff { splices: vec![ByteSplice { offset: 2, remove_len: 1, insert: vec![7, 8] }] };

        let mut left = d1.clone();
        left.absorb(d2.clone());
        left.absorb(d3.clone());

        let mut mid = d2.clone();
        mid.absorb(d3.clone());
        let mut right = d1.clone();
        right.absorb(mid);

        assert_eq!(left.apply(&base).unwrap(), right.apply(&base).unwrap());
        let sequential = {
            let s1 = d1.apply(&base).unwrap();
            let s2 = d2.apply(&s1).unwrap();
            d3.apply(&s2).unwrap()
        };
        assert_eq!(left.apply(&base).unwrap(), sequential);
    }

    #[semio_framework_async_macros::async_test]
    async fn between_roundtrip_synthetic() {
        let a = BinarySnapshot { bytes: vec![1, 2, 3, 4, 5], ..Default::default() };
        let b = BinarySnapshot { bytes: vec![1, 9, 9, 4, 5, 6], ..Default::default() };
        assert_eq!(BinaryDiff::between(&a, &b).apply(&a).unwrap(), b);
        assert_eq!(BinaryDiff::between(&b, &a).apply(&b).unwrap(), a);
        assert!(BinaryDiff::between(&a, &a).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn inverse_diff_level_roundtrip() {
        let base = BinarySnapshot { bytes: vec![1, 2, 3, 4], ..Default::default() };
        let d = BinaryDiff { splices: vec![ByteSplice { offset: 1, remove_len: 2, insert: vec![9, 9, 9] }] };
        let next = d.apply(&base).unwrap();
        let inv = d.inverse(&base);
        assert_eq!(inv.apply(&next).unwrap(), base);
    }

    #[semio_framework_async_macros::async_test]
    async fn apply_rejects_invalid_splice_without_mutating_base() {
        let base = BinarySnapshot { bytes: vec![1, 2, 3], ..Default::default() };
        let diff = BinaryDiff { splices: vec![ByteSplice { offset: 2, remove_len: 2, insert: vec![9] }] };
        assert!(diff.apply(&base).is_err());
        assert_eq!(base.bytes, vec![1, 2, 3]);
    }

    /// 🧪️ F6-PILOT: `DiffCodec` round-trip laws (derived via `dsl::DslDiff`).
    #[semio_framework_async_macros::async_test]
    async fn diff_codec_text_binary_roundtrip_law() {
        use protocol::DiffCodec;
        for d in demo_diff_cases() {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = BinaryDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch for {d:?} (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff({d:?}) failed: {e}"));
            let decoded = BinaryDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch for {d:?}");
        }
    }
}
//#endregion 🧪️Tests
