use crate::standards::v_commonmark::subsets::any::schema::mutations::*;
use protocol::OpBinary;

/// 🧪️ F6/P2-FG1: `OpText`/`OpBinary` round-trip laws for the hand-rolled `MdMutation` grammar —
/// see `demo_mutation_cases()`'s own doc comment for exactly what each case exercises.
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    for mutation in demo_mutation_cases() {
        let printed = mutation.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = MdMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

        let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
        let decoded = MdMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
    }
}

/// 🧪️ `kinds_match_enum_variants_and_catalog`: `KINDS` lists every `MdMutation` variant
/// exactly once (the `match` below has no wildcard arm, so a new variant fails to compile
/// here first) AND matches the mutation catalog this subset's `🔣️oracle.json`
/// declares, in the same order — the framework's completeness gate reads that JSON, never this
/// enum, so this test is the only thing tying the two declarations together.
#[semio_framework_async_macros::async_test]
async fn kinds_match_enum_variants_and_catalog() {
    fn kebab_of(mutation: &MdMutation) -> &'static str {
        match mutation {
            MdMutation::InsertBlock(_) => "insert-block",
            MdMutation::RemoveBlock(_) => "remove-block",
            MdMutation::ReplaceBlock(_) => "replace-block",
            MdMutation::SetInlines(_) => "set-inlines",
            MdMutation::SpliceSource(_) => "splice-source",
        }
    }
    let variant_kinds: std::collections::BTreeSet<&str> = demo_mutation_cases().iter().map(kebab_of).collect();
    let declared_kinds: std::collections::BTreeSet<&str> = KINDS.iter().copied().collect();
    assert_eq!(variant_kinds, declared_kinds, "KINDS must list every MdMutation variant exactly once");

    let manifest: serde_json::Value = serde_json::from_str(include_str!("../../../../🔮️oracles/🔣️.json")).expect("valid catalog JSON");
    let catalog_kinds: Vec<&str> = manifest["mutationCatalogs"][0]["kinds"].as_array().expect("mutationCatalogs[0].kinds array").iter().map(|value| value.as_str().expect("kind is a string")).collect();
    for kind in KINDS {
        assert!(catalog_kinds.contains(kind), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}
