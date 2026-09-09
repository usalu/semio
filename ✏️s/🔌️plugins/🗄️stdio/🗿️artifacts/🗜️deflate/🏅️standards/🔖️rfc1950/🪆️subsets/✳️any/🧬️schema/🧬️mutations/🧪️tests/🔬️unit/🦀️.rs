use super::*;
use crate::STDIO_DEFLATE_DOCUMENT_SCHEMA;

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

    let manifest = include_str!("../../../../🔮️oracle/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}
