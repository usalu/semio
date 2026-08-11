//! 🔺️ BinaryDiff — a splice list, not a full-replace. `bytes: Vec<u8>` stays as the snapshot
//! (the recipe's one documented "format IS bytes" exception), but the diff is structural: a
//! list of `ByteSplice{offset, remove_len, insert}` ranges, each range treated as its own
//! index-transport "key" (the recipe's own novel design point for this one artifact, since its
//! "collection" is raw byte ranges rather than typed items).

use crate::artifacts::binary::BinarySnapshot;
use protocol::{DiffAlgebra, MutationDiff};
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;
use std::collections::HashMap;

//#region 🔖️Splice
/// ✂️ One byte-range edit against the BASE array: replace `[offset, offset+remove_len)` with
/// `insert`. `offset`/`remove_len` are BASE-relative (never re-based against a prior splice in
/// the same diff -- see [`apply`](BinaryDiff::apply)'s normative processing order).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ByteSplice {
    pub offset: usize,
    pub remove_len: usize,
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
/// Out-of-range offsets/lengths clamp to the buffer's current bounds (graceful, never panics).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.binary.diff")]
pub struct BinaryDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub splices: Vec<ByteSplice>,
}

impl MutationDiff<BinarySnapshot> for BinaryDiff {
    fn apply(&self, base: &BinarySnapshot) -> BinarySnapshot {
        let mut bytes = base.bytes.clone();
        let mut splices = self.splices.clone();
        splices.sort_by(|a, b| b.offset.cmp(&a.offset));
        for s in splices {
            let start = s.offset.min(bytes.len());
            let end = (s.offset + s.remove_len).min(bytes.len()).max(start);
            bytes.splice(start..end, s.insert.iter().copied());
        }
        BinarySnapshot { schema: base.schema.clone(), bytes }
    }

    /// ➕️ Sequential-coalesce absorb via [`absorb_splices`]'s byte-range index-transport.
    fn absorb(&mut self, other: Self) {
        self.splices = absorb_splices(&self.splices, &other.splices);
    }
}

impl DiffAlgebra<BinarySnapshot> for BinaryDiff {
    fn inverse(&self, base: &BinarySnapshot) -> Self {
        let next = self.apply(base);
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

fn simulate_labels(labels: Vec<Lbl>, removed: &[usize], added: &[(usize, Lbl)]) -> Vec<Lbl> {
    let removed_set: std::collections::HashSet<usize> = removed.iter().copied().collect();
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

/// ➕️ Absorbs `d1` (base→mid) then `d2` (mid→after) splice lists into a single base→after
/// splice list. Each splice is decomposed into per-byte remove/insert label ops (a multi-byte
/// insert at `offset` becomes `offset, offset+1, offset+2, …` targets so the bytes land in
/// their original relative order -- inserting several items at literally the SAME target index
/// would otherwise reverse them, the same subtlety `TxtLinesDiff`'s absorb documents for
/// same-index inserts), simulated exactly like the line-diff case, then the resulting label
/// array is run-length-encoded back into a minimal ordered `ByteSplice` list.
fn absorb_splices(d1: &[ByteSplice], d2: &[ByteSplice]) -> Vec<ByteSplice> {
    let max_ref1 = d1.iter().map(|s| s.offset + s.remove_len.max(s.insert.len())).max();
    let l1 = max_ref1.map(|m| m + 8).unwrap_or(0);

    let base_labels: Vec<Lbl> = (0..l1).map(Lbl::Base).collect();
    let mut d1_removed = Vec::new();
    let mut d1_added: Vec<(usize, Lbl)> = Vec::new();
    for s in d1 {
        for i in s.offset..(s.offset + s.remove_len) {
            d1_removed.push(i);
        }
        for (k, byte) in s.insert.iter().enumerate() {
            d1_added.push((s.offset + k, Lbl::New(*byte)));
        }
    }
    let mut mid_labels = simulate_labels(base_labels, &d1_removed, &d1_added);

    let mut mid_pos_of_base: HashMap<usize, usize> = HashMap::new();
    for (pos, l) in mid_labels.iter().enumerate() {
        if let Lbl::Base(i) = l {
            mid_pos_of_base.insert(*i, pos);
        }
    }

    let max_ref2 = d2.iter().map(|s| s.offset + s.remove_len.max(s.insert.len())).max();
    let needed_len = max_ref2.map(|m| (m + 8).max(mid_labels.len())).unwrap_or(mid_labels.len());
    while mid_labels.len() < needed_len {
        mid_labels.push(Lbl::Base(usize::MAX)); // inert padding, tail-appended, never in mid_pos_of_base
    }

    let mut d2_removed = Vec::new();
    let mut d2_added: Vec<(usize, Lbl)> = Vec::new();
    for s in d2 {
        for i in s.offset..(s.offset + s.remove_len) {
            d2_removed.push(i);
        }
        for (k, byte) in s.insert.iter().enumerate() {
            d2_added.push((s.offset + k, Lbl::New(*byte)));
        }
    }
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
pub fn diff_set_snapshot(base: &BinarySnapshot, snapshot: &BinarySnapshot) -> BinaryDiff {
    BinaryDiff::between(base, snapshot)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_then_remove_before_matches_canonical_shape() {
        // Insert(0xAA) at offset 2, then Remove 1 byte at offset 0 -- byte-level analog of the
        // line-diff canonical case: {removed:[0], added:[(1,0xAA)]}.
        let d1 = vec![ByteSplice { offset: 2, remove_len: 0, insert: vec![0xAA] }];
        let d2 = vec![ByteSplice { offset: 0, remove_len: 1, insert: vec![] }];
        let merged = absorb_splices(&d1, &d2);

        let base = BinarySnapshot { bytes: vec![1, 2, 3, 4], ..Default::default() };
        let mid = BinaryDiff { splices: d1.clone() }.apply(&base);
        let after = BinaryDiff { splices: d2.clone() }.apply(&mid);
        assert_eq!(BinaryDiff { splices: merged }.apply(&base), after);
    }

    #[test]
    fn insert_insert_same_offset_both_survive() {
        let d1 = vec![ByteSplice { offset: 2, remove_len: 0, insert: vec![0xAA] }];
        let d2 = vec![ByteSplice { offset: 2, remove_len: 0, insert: vec![0xBB] }];
        let merged = absorb_splices(&d1, &d2);

        let base = BinarySnapshot { bytes: vec![1, 2, 3, 4], ..Default::default() };
        let mid = BinaryDiff { splices: d1.clone() }.apply(&base);
        let after = BinaryDiff { splices: d2.clone() }.apply(&mid);
        assert_eq!(BinaryDiff { splices: merged }.apply(&base), after);
        assert!(after.bytes.windows(2).any(|w| w == [0xBB, 0xAA]) || after.bytes.contains(&0xAA) && after.bytes.contains(&0xBB));
    }

    #[test]
    fn modify_then_remove_drops_the_modify() {
        let d1 = vec![ByteSplice { offset: 0, remove_len: 1, insert: vec![0xFF] }];
        let d2 = vec![ByteSplice { offset: 0, remove_len: 1, insert: vec![] }];
        let merged = absorb_splices(&d1, &d2);

        let base = BinarySnapshot { bytes: vec![1, 2, 3], ..Default::default() };
        let mid = BinaryDiff { splices: d1.clone() }.apply(&base);
        let after = BinaryDiff { splices: d2.clone() }.apply(&mid);
        assert_eq!(BinaryDiff { splices: merged }.apply(&base), after);
    }

    #[test]
    fn absorb_associative_over_a_triple() {
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

        assert_eq!(left.apply(&base), right.apply(&base));
        let sequential = { let s1 = d1.apply(&base); let s2 = d2.apply(&s1); d3.apply(&s2) };
        assert_eq!(left.apply(&base), sequential);
    }

    #[test]
    fn between_roundtrip_synthetic() {
        let a = BinarySnapshot { bytes: vec![1, 2, 3, 4, 5], ..Default::default() };
        let b = BinarySnapshot { bytes: vec![1, 9, 9, 4, 5, 6], ..Default::default() };
        assert_eq!(BinaryDiff::between(&a, &b).apply(&a), b);
        assert_eq!(BinaryDiff::between(&b, &a).apply(&b), a);
        assert!(BinaryDiff::between(&a, &a).is_empty());
    }

    #[test]
    fn inverse_diff_level_roundtrip() {
        let base = BinarySnapshot { bytes: vec![1, 2, 3, 4], ..Default::default() };
        let d = BinaryDiff { splices: vec![ByteSplice { offset: 1, remove_len: 2, insert: vec![9, 9, 9] }] };
        let next = d.apply(&base);
        let inv = d.inverse(&base);
        assert_eq!(inv.apply(&next), base);
    }
}
//#endregion 🧪️Tests
