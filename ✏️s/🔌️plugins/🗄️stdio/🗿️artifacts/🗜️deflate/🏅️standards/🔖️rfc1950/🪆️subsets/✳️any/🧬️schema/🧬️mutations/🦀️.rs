//! 🧬️ DeflateMutation — document mutation dispatch over the typed RFC1950 container fields.

use crate::artifacts::deflate::schema::diff::{diff_set_compression_params, diff_set_payload, diff_set_preset_dictionary, diff_set_snapshot, DeflateDiff};
use crate::artifacts::deflate::schema::snapshot::DeflateLevelHint;
use crate::artifacts::deflate::DeflateSnapshot;
use protocol::Mutation;
use protocol::{OpBinary, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.deflate`.
//#region 🔖️Leaves
#[path = "📄set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🧮set-compression-params/🦀️.rs"]
pub mod set_compression_params;
#[path = "📖set-preset-dictionary/🦀️.rs"]
pub mod set_preset_dictionary;
#[path = "📦set-payload/🦀️.rs"]
pub mod set_payload;
//#endregion 🔖️Leaves

/// 🧭️ `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires every variant to wrap exactly
/// one leaf payload (a unit variant wraps none) and asserts `is_approved_verb(SEMANTICS.verb)`,
/// and `no` is not an approved verb.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = DeflateSnapshot, diff = DeflateDiff, schema = "DeflateMutation")]
pub enum DeflateMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    /// 🧮️ Sets CMF's compression method/window bits and FLG's compression-level hint together
    /// (they're written to the same two-byte header, so one mutation covers all three).
    SetCompressionParams(set_compression_params::SetCompressionParams),
    /// 📖️ Sets or clears (via `None`) the preset-dictionary id (FLG.FDICT + DICTID).
    SetPresetDictionary(set_preset_dictionary::SetPresetDictionary),
    /// 📦️ Replaces the decompressed payload wholesale.
    SetPayload(set_payload::SetPayload),
}
//#endregion 🔖️Mutations

//#region 🔖️Kinds
/// 🗂️ Kebab-case spelling of every `DeflateMutation` variant, declaration order, mirrored by this
/// subset's `🧪️oracle/🔣️.json` mutation catalog (`deflate-rfc1950-any`). The completeness
/// gate reads that JSON catalog, never this enum, so `kinds_match_enum_variants_and_catalog` below
/// is what keeps the two lists honest.
pub const KINDS: &[&str] = &["set-snapshot", "set-compression-params", "set-preset-dictionary", "set-payload"];
//#endregion 🔖️Kinds

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`; the diff is the single semantics source (never
/// apply-and-capture).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_deflate_mutation(snapshot: &mut DeflateSnapshot, mutation: &DeflateMutation) -> protocol::MutationOutcome<DeflateDiff> {
    let outcome = <DeflateMutation as Mutation<DeflateSnapshot>>::diff(mutation, &*snapshot);
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
pub(crate) fn agg_diff(this: &DeflateMutation, base: &DeflateSnapshot) -> protocol::MutationOutcome<DeflateDiff> {
    protocol::MutationOutcome::new(match this {
        DeflateMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),
        DeflateMutation::SetCompressionParams(set_compression_params::SetCompressionParams { method, window_bits, level_hint }) => diff_set_compression_params(*method, *window_bits, *level_hint),
        DeflateMutation::SetPresetDictionary(set_preset_dictionary::SetPresetDictionary { dict_id }) => diff_set_preset_dictionary(*dict_id),
        DeflateMutation::SetPayload(set_payload::SetPayload { payload }) => diff_set_payload(payload.clone()),
    })
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &DeflateMutation, base: &DeflateSnapshot) -> Vec<DeflateMutation> {
    match this {
        DeflateMutation::SetSnapshot(_) => vec![DeflateMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        DeflateMutation::SetCompressionParams(_) => vec![DeflateMutation::SetCompressionParams(set_compression_params::SetCompressionParams { method: base.compression_method, window_bits: base.window_bits, level_hint: base.compression_level_hint })],
        DeflateMutation::SetPresetDictionary(_) => {
            vec![DeflateMutation::SetPresetDictionary(set_preset_dictionary::SetPresetDictionary { dict_id: base.dict_id })]
        }
        DeflateMutation::SetPayload(_) => vec![DeflateMutation::SetPayload(set_payload::SetPayload { payload: base.payload.clone() })],
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
impl OpText for DeflateMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

impl OpBinary for DeflateMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|e| protocol::ProtocolError::Io(e.to_string()))
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|e| protocol::ProtocolError::Io(e.to_string()))
    }
}
//#endregion OpCodecs

//#region 🔖️DemoCases
/// 🧪️ P2-FG2: representative `DeflateMutation` values (every variant, incl. both
/// `SetPresetDictionary` arms and both `SetPayload` empty/non-empty arms) — the single source of
/// truth reused by `op_text_binary_roundtrip_law` below AND by `⚙️engine/🦀️component.rs`'s
/// `ops_grammar_conformance_law`/`protocol_walk_law` conformance tests.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<DeflateMutation> {
    use crate::artifacts::deflate::STDIO_DEFLATE_DOCUMENT_SCHEMA;

    let snapshot =
        DeflateSnapshot { schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(), compression_method: 8, window_bits: 7, compression_level_hint: DeflateLevelHint::Default, dict_id: Some(0x1234_5678), payload: b"demo-mutation-snapshot-payload".to_vec() };
    vec![
        DeflateMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: snapshot.clone() }),
        DeflateMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: DeflateSnapshot { dict_id: None, compression_level_hint: DeflateLevelHint::Fastest, ..snapshot } }),
        DeflateMutation::SetCompressionParams(set_compression_params::SetCompressionParams { method: 8, window_bits: 5, level_hint: DeflateLevelHint::Maximum }),
        DeflateMutation::SetPresetDictionary(set_preset_dictionary::SetPresetDictionary { dict_id: Some(7) }),
        DeflateMutation::SetPresetDictionary(set_preset_dictionary::SetPresetDictionary { dict_id: None }),
        DeflateMutation::SetPayload(set_payload::SetPayload { payload: b"demo-payload".to_vec() }),
        DeflateMutation::SetPayload(set_payload::SetPayload { payload: Vec::new() }),
    ]
}
//#endregion 🔖️DemoCases

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::deflate::STDIO_DEFLATE_DOCUMENT_SCHEMA;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn base_snapshot() -> DeflateSnapshot {
        DeflateSnapshot { schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(), compression_method: 8, window_bits: 7, compression_level_hint: DeflateLevelHint::Fastest, dict_id: None, payload: b"op-text-binary-fixture".to_vec() }
    }

    /// 🧪️ `op_text_binary_roundtrip_law`: every variant (incl. both `SetPresetDictionary` arms,
    /// `Some`/`None`, and the `SetSnapshot` leaf payload) round-trips through `print_op`/`parse_op`
    /// (one line, no `\n`) AND `encode_op`/`decode_op`.
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        let base = base_snapshot();
        for mutation in [
            DeflateMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: DeflateSnapshot { dict_id: Some(0xDEAD_BEEF), ..base.clone() } }),
            DeflateMutation::SetCompressionParams(set_compression_params::SetCompressionParams { method: 8, window_bits: 5, level_hint: DeflateLevelHint::Maximum }),
            DeflateMutation::SetPresetDictionary(set_preset_dictionary::SetPresetDictionary { dict_id: Some(7) }),
            DeflateMutation::SetPresetDictionary(set_preset_dictionary::SetPresetDictionary { dict_id: None }),
            DeflateMutation::SetPayload(set_payload::SetPayload { payload: b"mutation-op-text-binary".to_vec() }),
            DeflateMutation::SetPayload(set_payload::SetPayload { payload: Vec::new() }),
        ] {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = DeflateMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
            let decoded = DeflateMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
        }
    }

    /// 🧪️ `kinds_match_enum_variants_and_catalog`: `KINDS` lists every `DeflateMutation` variant
    /// exactly once (the `match` below has no wildcard arm, so a new variant fails to compile
    /// here first). The manifest side is a containment check (mirrors the `tiff`/`mp3` migrated
    /// pilots' own `kinds_match_the_committed_catalog`/`kinds_matches_every_variant_and_the_catalog`
    /// tests): the oracle manifest's catalog is a DIFFERENT concern's file (still lists the dropped
    /// `no-mutation` kind, same as both pilots' own manifests), so this only asserts every KINDS
    /// entry is declared there, never that the manifest has nothing else.
    #[semio_framework_async_macros::async_test]
    async fn kinds_match_enum_variants_and_catalog() {
        // 🚫️async: E1 pure inherent helper, no I/O — see R9
        fn kebab_of(mutation: &DeflateMutation) -> &'static str {
            match mutation {
                DeflateMutation::SetSnapshot(_) => "set-snapshot",
                DeflateMutation::SetCompressionParams(_) => "set-compression-params",
                DeflateMutation::SetPresetDictionary(_) => "set-preset-dictionary",
                DeflateMutation::SetPayload(_) => "set-payload",
            }
        }
        let variant_kinds: std::collections::BTreeSet<&str> = demo_mutation_cases().iter().map(kebab_of).collect();
        let declared_kinds: std::collections::BTreeSet<&str> = KINDS.iter().copied().collect();
        assert_eq!(variant_kinds, declared_kinds, "KINDS must list every DeflateMutation variant exactly once");

        let manifest = include_str!("../../🧪️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
    }
}
//#endregion Tests

//#region 🧪️FixtureCases
/// 🧪️ Handcrafted `📄set-snapshot` fixture cases, wired from this tree's own mutations root so
/// `📦️glue.rs` stays untouched (`#[path]` on a non-inline module resolves against this file's own
/// directory).
#[cfg(test)]
#[path = "📄set-snapshot/🧪️tests/raises-the-flevel-hint-and-extends-the-payload/🦀️component.rs"]
mod set_snapshot_raises_the_flevel_hint_and_extends_the_payload;
//#endregion 🧪️FixtureCases
