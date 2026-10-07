//! 🧬️ BinaryMutation — document mutation dispatch. Every variant's `diff()`/`inverse()` is
//! handcrafted directly against `BinaryDiff`/`ByteSplice` -- no apply-and-capture.

use crate::schema::diff::{diff_set_snapshot, BinaryDiff, ByteSplice};
use crate::BinarySnapshot;
use protocol::Mutation;


//#region 🔖️Mutations
#[path = "➕️append-bytes/🦀️.rs"]
pub mod append_bytes;
#[path = "✂️replace-byte-range/🦀️.rs"]
pub mod replace_byte_range;
/// 📐️ Typed content mutation for `stdio.binary`.
//#region 🔖️Leaves
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🔪️truncate-at/🦀️.rs"]
pub mod truncate_at;
//#endregion 🔖️Leaves

/// 🧭️ `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires every variant to wrap exactly
/// one leaf payload (a unit variant wraps none) and asserts `is_approved_verb(SEMANTICS.verb)`,
/// and `no` is not an approved verb.
///
/// 🧪️ `#[derive(semio_framework_dsl_record_derive::DslEnum)]` is kept ALONGSIDE `#[derive(dsl::Mutations)]`: every variant below
/// is a single-field newtype wrapping its own mutation leaf, and `dsl_variants_codegen`'s
/// "single-field tuple variant" branch (`✨️derive/🦀️.rs`) delegates `DslVariants`
/// straight through to that leaf's own `#[derive(semio_framework_dsl_record_derive::DslRecord)]`-provided `DslField` impl — the
/// SAME `record_codegen` output the fields produced when they lived inline in the enum, so the
/// committed `crate::standards::v_raw::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO`/`crate::standards::v_raw::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO`
/// facets and this `OpText`/`OpBinary` pair are unaffected by the leaf split.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = BinarySnapshot, diff = BinaryDiff, schema = "BinaryMutation")]
pub enum BinaryMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    /// ✂️ Replaces `[offset, offset+remove_len)` with `insert`.
    ReplaceByteRange(replace_byte_range::ReplaceByteRange),
    /// ➕️ Appends `data` at the end of the buffer.
    AppendBytes(append_bytes::AppendBytes),
    /// ✂️ Drops everything at/after `offset` (a no-op if `offset >= len`).
    TruncateAt(truncate_at::TruncateAt),
}
//#endregion 🔖️Mutations

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every `BinaryMutation` variant, in declaration order — the vocabulary
/// the `binary-raw-any` mutation catalog (`../../🔣️oracle.json`) declares and the
/// exhaustive mutate/inverse test case measures itself against. `kinds_cover_every_variant` below
/// is what keeps this list honest against the enum it names, since the framework never parses Rust.
pub const KINDS: &[&str] = &["set-snapshot", "replace-byte-range", "append-bytes", "truncate-at"];
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
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
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
pub(crate) fn agg_inverse(this: &BinaryMutation, base: &BinarySnapshot) -> Result<Vec<BinaryMutation>, semio_framework_value::ValueError> {
    Ok((|| {
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
                // "was already absent" case, `../../🖼️tiff/…/🧱️baseline/🧬️schema/🧬️mutations/
                // 🦀️.rs`), rather than reinstating a unit `NoMutation` variant the derive forbids.
                Vec::new()
            } else {
                vec![BinaryMutation::ReplaceByteRange(replace_byte_range::ReplaceByteRange { offset: *offset, remove_len: 0, insert: base.bytes[*offset..].to_vec() })]
            }
        }
    }

    })())
}
//#endregion 🔖️MutationTrait

//#region OpCodecs



//#endregion OpCodecs

/// 🧪️ P2-P3: representative `BinaryMutation` cases, one per variant, against [`tests::base`]'s
/// canonical `[1,2,3,4,5]` snapshot -- single source of truth shared by the round-trip/law tests
/// below AND the new `ops_grammar_conformance_law`/`protocol_walk_law` conformance tests in
/// `⚙️engine/🦀️.rs`, per CLAUDE.md (no duplicated literal case lists).
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧪️FixtureTests
// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION),
// one case per mutation leaf. Wired HERE and not in `🦀️.rs`: that file is shared with the
// agents migrating the other stdio artifacts, so the production mounts there stay untouched while
// this artifact owns its own test mount. `#[path = "."]` re-bases the children on this file's own
// directory, which is what makes the leaf-relative path below resolve.
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
//#endregion 🧪️FixtureTests

#[cfg(test)]
use protocol::{OpBinary,OpText};
