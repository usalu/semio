use super::*;

/// 🧪️ F6/P2-FG3: `OpText`/`OpBinary` round-trip laws for the hand-rolled `PlyMutation`
/// grammar — `OpBinary` is now a REAL binary frame, no longer text-as-bytes.
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    for mutation in demo_mutation_cases() {
        let printed = mutation.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = PlyMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

        let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
        let decoded = PlyMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
    }
}

//#region 🔖️KindsCoverageLaw
/// 🏷️ `KINDS` must name exactly the enum's variants (kebab-case), one entry each — an
/// exhaustive `match` so the compiler itself fails the moment a variant is added, renamed or
/// removed without this list being updated alongside it. The manifest side of the same claim
/// (`../../🔣️oracle.json`'s `ply-1-0-any` catalog `kinds`) is checked by the
/// mutate/inverse test case's own contract gate, which fails if the two lists ever diverge.
#[semio_framework_async_macros::async_test]
async fn kinds_cover_every_variant() {
    fn kind_of(mutation: &PlyMutation) -> &'static str {
        match mutation {
            PlyMutation::SetSnapshot(..) => "set-snapshot",
            PlyMutation::SetFormat(..) => "set-format",
            PlyMutation::InsertComment(..) => "insert-comment",
            PlyMutation::RemoveComment(..) => "remove-comment",
            PlyMutation::AddElement(..) => "add-element",
            PlyMutation::RemoveElement(..) => "remove-element",
            PlyMutation::InsertRow(..) => "insert-row",
            PlyMutation::RemoveRow(..) => "remove-row",
            PlyMutation::SetRowProperty(..) => "set-row-property",
        }
    }
    let mut exercised: Vec<&str> = demo_mutation_cases().iter().map(kind_of).collect();
    exercised.sort_unstable();
    exercised.dedup();
    let mut declared: Vec<&str> = KINDS.to_vec();
    declared.sort_unstable();
    assert_eq!(exercised, declared, "KINDS must name exactly the variants demo_mutation_cases() exercises");
    assert_eq!(KINDS.len(), 9, "ply-1-0-any declares 9 PlyMutation variants");
}
//#endregion 🔖️KindsCoverageLaw
