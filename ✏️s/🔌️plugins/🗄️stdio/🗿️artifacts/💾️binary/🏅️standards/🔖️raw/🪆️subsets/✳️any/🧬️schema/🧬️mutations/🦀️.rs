//! 🧬️ BinaryMutation — document mutation dispatch. Every variant's `diff()`/`inverse()` is
//! handcrafted directly against `BinaryDiff`/`ByteSplice` -- no apply-and-capture.

use crate::schema::diff::{BinaryDiff, ByteSplice};
use crate::BinarySnapshot;
use protocol::Mutation;


//#region 🔖️Mutations
#[path = "➕️append-bytes/🦀️.rs"]
pub mod append_bytes;
#[path = "✂️replace-byte-range/🦀️.rs"]
pub mod replace_byte_range;
#[path = "🔪️truncate-at/🦀️.rs"]
pub mod truncate_at;

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
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = BinarySnapshot, diff = BinaryDiff, schema = "BinaryMutation")]
pub enum BinaryMutation {
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
pub const KINDS: &[&str] = &["replace-byte-range", "append-bytes", "truncate-at"];
//#endregion 🔖️Kinds


//#endregion 🔖️Apply

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

#[cfg(test)]
use protocol::{OpBinary,OpText};
