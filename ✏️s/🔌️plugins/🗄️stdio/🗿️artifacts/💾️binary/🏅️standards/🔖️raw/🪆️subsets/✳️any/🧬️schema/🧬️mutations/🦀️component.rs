//! 🧬️ BinaryMutation — document mutation dispatch. Every variant's `diff()`/`inverse()` is
//! handcrafted directly against `BinaryDiff`/`ByteSplice` -- no apply-and-capture.

use crate::artifacts::binary::schema::diff::{diff_set_snapshot, BinaryDiff, ByteSplice};
use crate::artifacts::binary::BinarySnapshot;
use protocol::Mutation;
use protocol::{OpBinary, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.binary`.
///
/// 🧪️ F6-PILOT: `dsl::DslOps` derive added — emits `dsl::DslVariants` only (P6: `OpText`/
/// `OpBinary` are always handcrafted, see the `OpCodecs` region below). `#[dsl(base64)]` on the
/// two `Vec<u8>` payload fields keeps the printed op a compact one-liner instead of a bracketed
/// list of decimal byte values.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum BinaryMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        #[dsl(block)]
        snapshot: BinarySnapshot,
    },
    /// ✂️ Replaces `[offset, offset+remove_len)` with `insert`.
    Splice {
        offset: usize,
        remove_len: usize,
        #[dsl(base64)]
        insert: Vec<u8>,
    },
    /// ➕️ Appends `data` at the end of the buffer.
    AppendBytes {
        #[dsl(base64)]
        data: Vec<u8>,
    },
    /// ✂️ Drops everything at/after `offset` (a no-op if `offset >= len`).
    TruncateAt { offset: usize },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`. Diff is the single semantics source.
pub async fn apply_binary_mutation(snapshot: &mut BinarySnapshot, mutation: &BinaryMutation) -> protocol::MutationOutcome<BinaryDiff> {
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
impl Mutation<BinarySnapshot> for BinaryMutation {
    type Diff = BinaryDiff;

    async fn diff(&self, base: &BinarySnapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(match self {
            BinaryMutation::NoMutation => BinaryDiff::default(),
            BinaryMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
            BinaryMutation::Splice { offset, remove_len, insert } => BinaryDiff { splices: vec![ByteSplice { offset: *offset, remove_len: *remove_len, insert: insert.clone() }] },
            BinaryMutation::AppendBytes { data } => BinaryDiff { splices: vec![ByteSplice { offset: base.bytes.len(), remove_len: 0, insert: data.clone() }] },
            BinaryMutation::TruncateAt { offset } => {
                if *offset >= base.bytes.len() {
                    BinaryDiff::default()
                } else {
                    BinaryDiff { splices: vec![ByteSplice { offset: *offset, remove_len: base.bytes.len() - offset, insert: vec![] }] }
                }
            }
        })
    }

    async fn inverse(&self, base: &BinarySnapshot) -> Vec<Self> {
        match self {
            BinaryMutation::NoMutation => vec![BinaryMutation::NoMutation],
            BinaryMutation::SetSnapshot { .. } => vec![BinaryMutation::SetSnapshot { snapshot: base.clone() }],
            BinaryMutation::Splice { offset, remove_len, insert } => {
                let start = (*offset).min(base.bytes.len());
                let end = (*offset + *remove_len).min(base.bytes.len());
                let removed_bytes = base.bytes[start..end].to_vec();
                vec![BinaryMutation::Splice { offset: start, remove_len: insert.len(), insert: removed_bytes }]
            }
            BinaryMutation::AppendBytes { .. } => {
                // ↩️ Undo an append by truncating back to the pre-append length.
                vec![BinaryMutation::TruncateAt { offset: base.bytes.len() }]
            }
            BinaryMutation::TruncateAt { offset } => {
                if *offset >= base.bytes.len() {
                    vec![BinaryMutation::NoMutation]
                } else {
                    vec![BinaryMutation::Splice { offset: *offset, remove_len: 0, insert: base.bytes[*offset..].to_vec() }]
                }
            }
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🎙️ Handcrafted `OpText` (P6: `dsl::DslOps` emits `DslVariants` only) — one-line grammar via
/// the derived `RecordSpec`/`DslVariants`. Body is the same ~15-line shape every `DslOps`-derived
/// enum's `OpText` impl uses (see `SpaceMutation`, `FlowMutationDsl` for the framework-side
/// precedent this copies verbatim).
impl OpText for BinaryMutation {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    async fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// ⚡️ Handcrafted `OpBinary` (P6) — pure forward to `dsl::variants_binary`, the generic
/// `format u8 (=1) | variant ordinal varint | record body` layout shared by every `DslVariants`
/// type. Zero per-artifact logic.
impl OpBinary for BinaryMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion OpCodecs

/// 🧪️ P2-P3: representative `BinaryMutation` cases, one per variant, against [`tests::base`]'s
/// canonical `[1,2,3,4,5]` snapshot -- single source of truth shared by the round-trip/law tests
/// below AND the new `ops_grammar_conformance_law`/`protocol_walk_law` conformance tests in
/// `⚙️engine/🦀️component.rs`, per CLAUDE.md (no duplicated literal case lists).
#[cfg(test)]
pub(crate) async fn demo_mutation_cases() -> Vec<BinaryMutation> {
    vec![
        BinaryMutation::NoMutation,
        BinaryMutation::SetSnapshot { snapshot: BinarySnapshot { bytes: vec![9, 9], ..Default::default() } },
        BinaryMutation::Splice { offset: 1, remove_len: 2, insert: vec![0xAA, 0xBB, 0xCC] },
        BinaryMutation::AppendBytes { data: vec![0xEE, 0xFF] },
        BinaryMutation::TruncateAt { offset: 4 },
    ]
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::os_spr::command::DiffAlgebra;
    use protocol::MutationDiff;

    pub(crate) async fn base() -> BinarySnapshot {
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

    /// 🧪️ F6-PILOT: `OpText`/`OpBinary` round-trip laws (handcrafted impls over the
    /// `dsl::DslOps`-derived `DslVariants`).
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
}
//#endregion 🧪️Tests
