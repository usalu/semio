//! 🧬️ BinaryMutation — document mutation dispatch. Every variant's `diff()`/`inverse()` is
//! handcrafted directly against `BinaryDiff`/`ByteSplice` -- no apply-and-capture.

use crate::artifacts::binary::schema::diff::{diff_set_snapshot, BinaryDiff, ByteSplice};
use crate::artifacts::binary::BinarySnapshot;
use protocol::Mutation;
use protocol::{OpBinary, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.binary`.
//#region 🔖️Leaves
#[path = "📄set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "✂replace-byte-range/🦀️.rs"]
pub mod replace_byte_range;
#[path = "➕append-bytes/🦀️.rs"]
pub mod append_bytes;
#[path = "🔪truncate-at/🦀️.rs"]
pub mod truncate_at;
//#endregion 🔖️Leaves

/// 🧭️ `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires every variant to wrap exactly
/// one leaf payload (a unit variant wraps none) and asserts `is_approved_verb(SEMANTICS.verb)`,
/// and `no` is not an approved verb.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = BinarySnapshot, diff = BinaryDiff, schema = "BinaryMutation")]
pub enum BinaryMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    /// ✂️ Replaces `[offset, offset+remove_len)` with `insert`. The variant is named for its
    /// approved verb (`replace`) because `#[derive(dsl::Mutations)]` asserts the leaf descriptor's
    /// `semanticKind` equals `to_kebab(VariantIdent)` and rejects a single-word kind; the wire tag
    /// stays `splice`, which is what the catalog, the feature file and the committed fixtures speak.
    #[serde(rename = "splice")]
    ReplaceByteRange(replace_byte_range::ReplaceByteRange),
    /// ➕️ Appends `data` at the end of the buffer.
    AppendBytes(append_bytes::AppendBytes),
    /// ✂️ Drops everything at/after `offset` (a no-op if `offset >= len`).
    TruncateAt(truncate_at::TruncateAt),
}
//#endregion 🔖️Mutations

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every `BinaryMutation` variant, in declaration order — the vocabulary
/// the `binary-raw-any` mutation catalog (`../../🧪️oracle/🔣️.json`) declares and the
/// exhaustive mutate/inverse test case measures itself against. `kinds_cover_every_variant` below
/// is what keeps this list honest against the enum it names, since the framework never parses Rust.
pub const KINDS: &[&str] = &["set-snapshot", "splice", "append-bytes", "truncate-at"];
//#endregion 🔖️Kinds

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`. Diff is the single semantics source.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_binary_mutation(snapshot: &mut BinarySnapshot, mutation: &BinaryMutation) -> protocol::MutationOutcome<BinaryDiff> {
    let outcome = <BinaryMutation as Mutation<BinarySnapshot>>::diff(mutation, &*snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &BinaryMutation, base: &BinarySnapshot) -> protocol::MutationOutcome<BinaryDiff> {
    protocol::MutationOutcome::new(match this {
        BinaryMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),
        BinaryMutation::ReplaceByteRange(replace_byte_range::ReplaceByteRange { offset, remove_len, insert }) => BinaryDiff { splices: vec![ByteSplice { offset: *offset, remove_len: *remove_len, insert: insert.clone() }] },
        BinaryMutation::AppendBytes(append_bytes::AppendBytes { data }) => BinaryDiff { splices: vec![ByteSplice { offset: base.bytes.len(), remove_len: 0, insert: data.clone() }] },
        BinaryMutation::TruncateAt(truncate_at::TruncateAt { offset }) => {
            if *offset >= base.bytes.len() {
                BinaryDiff::default()
            } else {
                BinaryDiff { splices: vec![ByteSplice { offset: *offset, remove_len: base.bytes.len() - offset, insert: vec![] }] }
            }
        }
    })
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &BinaryMutation, base: &BinarySnapshot) -> Vec<BinaryMutation> {
    match this {
        BinaryMutation::SetSnapshot(_) => vec![BinaryMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        BinaryMutation::ReplaceByteRange(replace_byte_range::ReplaceByteRange { offset, remove_len, insert }) => {
            let start = (*offset).min(base.bytes.len());
            let end = (*offset + *remove_len).min(base.bytes.len());
            let removed_bytes = base.bytes[start..end].to_vec();
            vec![BinaryMutation::ReplaceByteRange(replace_byte_range::ReplaceByteRange { offset: start, remove_len: insert.len(), insert: removed_bytes })]
        }
        BinaryMutation::AppendBytes(_) => {
            // ↩️ Undo an append by truncating back to the pre-append length.
            vec![BinaryMutation::TruncateAt(truncate_at::TruncateAt { offset: base.bytes.len() })]
        }
        BinaryMutation::TruncateAt(truncate_at::TruncateAt { offset }) => {
            if *offset >= base.bytes.len() {
                // 🧭️ Nothing was actually dropped (offset was already past the end), so there is
                // no real forward step to undo — the same empty-inverse idiom the migrated `tiff`
                // pilot uses for its own dropped-`NoMutation` fallback arms (`RemoveTileTags`'s
                // "was already absent" case, `../../🖼️tiff/…/✳️baseline/🧬️schema/🧬️mutations/
                // 🦀️.rs`), rather than reinstating a unit `NoMutation` variant the derive forbids.
                return Vec::new();
            } else {
                vec![BinaryMutation::ReplaceByteRange(replace_byte_range::ReplaceByteRange { offset: *offset, remove_len: 0, insert: base.bytes[*offset..].to_vec() })]
            }
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🎙️ Handcrafted `OpText`/`OpBinary` via plain `serde_json` (one line of compact JSON per op) —
/// mirrors the `stdio.mp3` pilot's own hand-rolled bridge
/// (`../../../../../🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`).
/// P6's `#[derive(dsl::DslOps)]` cannot apply post-migration: its `DslVariants` codegen only
/// delegates a single-field tuple variant to the inner type's OWN `dsl::DslField` impl, and a
/// `#[derive(dsl::MutationLeaf)]` payload does not carry one — so the prior `DslOps`-derived
/// grammar bridge is replaced by this JSON one, not preserved byte-for-byte.
impl OpText for BinaryMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

impl OpBinary for BinaryMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|e| protocol::ProtocolError::Io(e.to_string()))
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|e| protocol::ProtocolError::Io(e.to_string()))
    }
}
//#endregion OpCodecs

/// 🧪️ P2-P3: representative `BinaryMutation` cases, one per variant, against [`tests::base`]'s
/// canonical `[1,2,3,4,5]` snapshot -- single source of truth shared by the round-trip/law tests
/// below AND the new `ops_grammar_conformance_law`/`protocol_walk_law` conformance tests in
/// `⚙️engine/🦀️component.rs`, per CLAUDE.md (no duplicated literal case lists).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<BinaryMutation> {
    vec![
        BinaryMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: BinarySnapshot { bytes: vec![9, 9], ..Default::default() } }),
        BinaryMutation::ReplaceByteRange(replace_byte_range::ReplaceByteRange { offset: 1, remove_len: 2, insert: vec![0xAA, 0xBB, 0xCC] }),
        BinaryMutation::AppendBytes(append_bytes::AppendBytes { data: vec![0xEE, 0xFF] }),
        BinaryMutation::TruncateAt(truncate_at::TruncateAt { offset: 4 }),
    ]
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::os_spr::command::DiffAlgebra;
    use protocol::MutationDiff;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn base() -> BinarySnapshot {
        BinarySnapshot { bytes: vec![1, 2, 3, 4, 5], ..Default::default() }
    }

    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law() {
        let b = base();
        for m in demo_mutation_cases() {
            let mut via_apply = b.clone();
            let returned = apply_binary_mutation(&mut via_apply, &m);
            let expected_diff = m.diff(&b);
            assert_eq!(returned, expected_diff, "returned diff mismatch for {m:?}");
            assert_eq!(via_apply, expected_diff.diff().apply(&b).unwrap(), "apply mismatch for {m:?}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn inverse_law() {
        let b = base();
        for m in demo_mutation_cases() {
            let mut mutated = b.clone();
            apply_binary_mutation(&mut mutated, &m);
            for undo in m.inverse(&b) {
                apply_binary_mutation(&mut mutated, &undo);
            }
            assert_eq!(mutated, b, "mutation-level inverse round-trip failed for {m:?}");
        }
        for m in demo_mutation_cases() {
            let d = m.diff(&b);
            let next = d.diff().apply(&b).unwrap();
            let inv = d.diff().inverse(&b);
            assert_eq!(inv.apply(&next).unwrap(), b, "diff-level inverse round-trip failed for {m:?}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_law_cartesian() {
        let b = base();
        let variants = demo_mutation_cases();
        for m1 in &variants {
            let d1 = m1.diff(&b);
            let mid = d1.diff().apply(&b).unwrap();
            for m2 in &variants {
                let d2 = m2.diff(&mid);
                let after = d2.diff().apply(&mid).unwrap();
                let mut merged = d1.diff().clone();
                merged.absorb(d2.diff().clone());
                assert_eq!(merged.apply(&b).unwrap(), after, "absorb({m1:?}, {m2:?}) mismatch");
            }
        }
    }

    /// 🧪️ `OpText`/`OpBinary` round-trip laws, hand-rolled over `serde_json` (see the `OpCodecs`
    /// region's doc comment for why this replaced the prior `dsl::DslOps`-derived bridge).
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        for m in demo_mutation_cases() {
            let printed = m.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = BinaryMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?} (printed {printed:?})");

            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
            let decoded = BinaryMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
        }
    }

    //#region 🔖️KindsCoverageLaw
    /// 🏷️ `KINDS` must name exactly the enum's variants (kebab-case), one entry each — an
    /// exhaustive `match` so the compiler itself fails the moment a variant is added, renamed or
    /// removed without this list being updated alongside it. The manifest side of the same claim
    /// (`../../🧪️oracle/🔣️.json`'s `binary-raw-any` catalog `kinds`) is checked by the
    /// mutate/inverse test case's own contract gate, which fails if the two lists ever diverge.
    #[semio_framework_async_macros::async_test]
    async fn kinds_cover_every_variant() {
        fn kind_of(mutation: &BinaryMutation) -> &'static str {
            match mutation {
                BinaryMutation::SetSnapshot(_) => "set-snapshot",
                BinaryMutation::ReplaceByteRange(_) => "splice",
                BinaryMutation::AppendBytes(_) => "append-bytes",
                BinaryMutation::TruncateAt(_) => "truncate-at",
            }
        }
        let mut exercised: Vec<&str> = demo_mutation_cases().iter().map(kind_of).collect();
        exercised.sort_unstable();
        exercised.dedup();
        let mut declared: Vec<&str> = KINDS.to_vec();
        declared.sort_unstable();
        assert_eq!(exercised, declared, "KINDS must name exactly the variants demo_mutation_cases() exercises");
        assert_eq!(KINDS.len(), 4, "binary-raw-any declares 4 BinaryMutation variants");
    }
    //#endregion 🔖️KindsCoverageLaw
}
//#endregion 🧪️Tests

//#region 🧪️FixtureTests
// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION),
// one case per mutation leaf. Wired HERE and not in `📦️glue.rs`: that file is shared with the
// agents migrating the other stdio artifacts, so the production mounts there stay untouched while
// this artifact owns its own test mount. `#[path = "."]` re-bases the children on this file's own
// directory, which is what makes the leaf-relative path below resolve.
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "📄set-snapshot/🧪️tests/rewrites-the-two-middle-bytes/🦀️component.rs"]
    mod tests_set_snapshot_rewrites_the_two_middle_bytes;
}
//#endregion 🧪️FixtureTests
