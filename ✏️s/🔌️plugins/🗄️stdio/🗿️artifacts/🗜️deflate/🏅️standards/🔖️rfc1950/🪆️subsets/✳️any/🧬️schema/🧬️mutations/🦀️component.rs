//! 🧬️ DeflateMutation — document mutation dispatch over the typed RFC1950 container fields.

use crate::artifacts::deflate::schema::diff::{diff_set_compression_params, diff_set_payload, diff_set_preset_dictionary, diff_set_snapshot, DeflateDiff};
use crate::artifacts::deflate::schema::snapshot::DeflateLevelHint;
use crate::artifacts::deflate::DeflateSnapshot;
use protocol::Mutation;
use protocol::{OpBinary, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.deflate`.
///
/// 🧪️ F6: `dsl::DslOps` derive — `DeflateSnapshot`'s field tree has no data-carrying enum
/// anywhere (`DeflateLevelHint` is unit-variant-only, `dsl::DslScalar`-derived), so every
/// variant's payload binds cleanly (confirmed via real `cargo check`, no mirror-enum needed).
/// `#[dsl(block)]` on the struct-valued `snapshot` payload matches the `SpaceMutation`/
/// `GifMutation` framework precedent's formatting convention; `#[dsl(base64)]` on the two bare
/// `Vec<u8>`/`insert`-shaped payloads keeps the printed op compact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum DeflateMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        #[dsl(block)]
        snapshot: DeflateSnapshot,
    },
    /// 🧮️ Sets CMF's compression method/window bits and FLG's compression-level hint together
    /// (they're written to the same two-byte header, so one mutation covers all three).
    SetCompressionParams { method: u8, window_bits: u8, level_hint: DeflateLevelHint },
    /// 📖️ Sets or clears (via `None`) the preset-dictionary id (FLG.FDICT + DICTID).
    SetPresetDictionary { dict_id: Option<u32> },
    /// 📦️ Replaces the decompressed payload wholesale.
    SetPayload {
        #[dsl(base64)]
        payload: Vec<u8>,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Kinds
/// 🗂️ Kebab-case spelling of every `DeflateMutation` variant, declaration order, mirrored by this
/// subset's `🧪️oracle/🔣️component.json` mutation catalog (`deflate-rfc1950-any`). The completeness
/// gate reads that JSON catalog, never this enum, so `kinds_match_enum_variants_and_catalog` below
/// is what keeps the two lists honest.
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-compression-params", "set-preset-dictionary", "set-payload"];
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
impl Mutation<DeflateSnapshot> for DeflateMutation {
    type Diff = DeflateDiff;

    fn diff(&self, base: &DeflateSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(match self {
            DeflateMutation::NoMutation => DeflateDiff::default(),
            DeflateMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
            DeflateMutation::SetCompressionParams { method, window_bits, level_hint } => diff_set_compression_params(*method, *window_bits, *level_hint),
            DeflateMutation::SetPresetDictionary { dict_id } => diff_set_preset_dictionary(*dict_id),
            DeflateMutation::SetPayload { payload } => diff_set_payload(payload.clone()),
        })
    }

    fn inverse(&self, base: &DeflateSnapshot) -> Vec<Self> {
        match self {
            DeflateMutation::NoMutation => vec![DeflateMutation::NoMutation],
            DeflateMutation::SetSnapshot { .. } => vec![DeflateMutation::SetSnapshot { snapshot: base.clone() }],
            DeflateMutation::SetCompressionParams { .. } => vec![DeflateMutation::SetCompressionParams { method: base.compression_method, window_bits: base.window_bits, level_hint: base.compression_level_hint }],
            DeflateMutation::SetPresetDictionary { .. } => {
                vec![DeflateMutation::SetPresetDictionary { dict_id: base.dict_id }]
            }
            DeflateMutation::SetPayload { .. } => vec![DeflateMutation::SetPayload { payload: base.payload.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🎙️ Handcrafted `OpText` (P6: `dsl::DslOps` emits `DslVariants` only, never `OpText`/
/// `OpBinary` themselves) — the same ~15-line body every `DslOps`-derived enum's `OpText` impl
/// uses (`FlowMutationDsl`/`SpaceMutation`/`BinaryMutation`/`GifMutation` precedent). Replaces
/// the prior `serde_json` stub.
impl OpText for DeflateMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
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
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// ⚡️ Handcrafted `OpBinary` (P6) — pure forward to `dsl::variants_binary`. Replaces the prior
/// `serde_json` stub.
impl OpBinary for DeflateMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
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
        DeflateMutation::NoMutation,
        DeflateMutation::SetSnapshot { snapshot: snapshot.clone() },
        DeflateMutation::SetSnapshot { snapshot: DeflateSnapshot { dict_id: None, compression_level_hint: DeflateLevelHint::Fastest, ..snapshot } },
        DeflateMutation::SetCompressionParams { method: 8, window_bits: 5, level_hint: DeflateLevelHint::Maximum },
        DeflateMutation::SetPresetDictionary { dict_id: Some(7) },
        DeflateMutation::SetPresetDictionary { dict_id: None },
        DeflateMutation::SetPayload { payload: b"demo-payload".to_vec() },
        DeflateMutation::SetPayload { payload: Vec::new() },
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
    /// `Some`/`None`, and the `SetSnapshot` struct-payload variant) round-trips through
    /// `print_op`/`parse_op` (one line, no `\n`) AND `encode_op`/`decode_op`.
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        let base = base_snapshot();
        for mutation in [
            DeflateMutation::NoMutation,
            DeflateMutation::SetSnapshot { snapshot: DeflateSnapshot { dict_id: Some(0xDEAD_BEEF), ..base.clone() } },
            DeflateMutation::SetCompressionParams { method: 8, window_bits: 5, level_hint: DeflateLevelHint::Maximum },
            DeflateMutation::SetPresetDictionary { dict_id: Some(7) },
            DeflateMutation::SetPresetDictionary { dict_id: None },
            DeflateMutation::SetPayload { payload: b"mutation-op-text-binary".to_vec() },
            DeflateMutation::SetPayload { payload: Vec::new() },
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
    /// here first) AND matches the mutation catalog this subset's `🧪️oracle/🔣️component.json`
    /// declares, in the same order — the framework's completeness gate reads that JSON, never this
    /// enum, so this test is the only thing tying the two declarations together.
    #[semio_framework_async_macros::async_test]
    async fn kinds_match_enum_variants_and_catalog() {
        // 🚫️async: E1 pure inherent helper, no I/O — see R9
        fn kebab_of(mutation: &DeflateMutation) -> &'static str {
            match mutation {
                DeflateMutation::NoMutation => "no-mutation",
                DeflateMutation::SetSnapshot { .. } => "set-snapshot",
                DeflateMutation::SetCompressionParams { .. } => "set-compression-params",
                DeflateMutation::SetPresetDictionary { .. } => "set-preset-dictionary",
                DeflateMutation::SetPayload { .. } => "set-payload",
            }
        }
        let variant_kinds: std::collections::BTreeSet<&str> = demo_mutation_cases().iter().map(kebab_of).collect();
        let declared_kinds: std::collections::BTreeSet<&str> = KINDS.iter().copied().collect();
        assert_eq!(variant_kinds, declared_kinds, "KINDS must list every DeflateMutation variant exactly once");

        let manifest: serde_json::Value = serde_json::from_str(include_str!("../../🧪️oracle/🔣️component.json")).expect("valid catalog JSON");
        let catalog_kinds: Vec<&str> = manifest["mutationCatalogs"][0]["kinds"].as_array().expect("mutationCatalogs[0].kinds array").iter().map(|value| value.as_str().expect("kind is a string")).collect();
        assert_eq!(catalog_kinds, KINDS.to_vec(), "the manifest's mutationCatalogs[0].kinds must match KINDS exactly, declaration order included");
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
