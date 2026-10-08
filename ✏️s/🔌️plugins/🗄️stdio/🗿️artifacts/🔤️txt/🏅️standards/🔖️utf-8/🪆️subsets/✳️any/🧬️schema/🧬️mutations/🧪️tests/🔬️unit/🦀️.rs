use super::*;
use semio_framework_value::FromValue;
use protocol::SemanticMutation;

/// 🛡️ One of the two production-enum proofs required by ticket
/// `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`'s
/// enum `deny_unknown_fields` fix (the other is `FlowMutation`, see
/// `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🧪️test/🦀️s.rs`). `TxtMutation` is
/// adjacently tagged (`tag = "mutation", content = "payload"`) — an unknown key sitting
/// alongside `mutation`/`payload` at the OUTER level must now be rejected by the derive's own
/// `FromValue`, not just by a `serde_json` sibling.
#[test]
fn aggregate_denies_unknown_outer_key_via_first_party_from_value() {
    let good = semio_framework_value::DslValue::object([
        ("mutation".to_string(), semio_framework_value::DslValue::String("set-line".to_string())),
        ("payload".to_string(), semio_framework_value::DslValue::object([("index".to_string(), semio_framework_value::DslValue::uint(1)), ("text".to_string(), semio_framework_value::DslValue::String("a".to_string()))])),
    ]);
    assert_eq!(TxtMutation::from_value(good), Ok(TxtMutation::SetLine(SetLineMutation { index: 1, text: "a".to_string() })));
    let bad = semio_framework_value::DslValue::object([
        ("mutation".to_string(), semio_framework_value::DslValue::String("set-line".to_string())),
        ("payload".to_string(), semio_framework_value::DslValue::object([("index".to_string(), semio_framework_value::DslValue::uint(1)), ("text".to_string(), semio_framework_value::DslValue::String("a".to_string()))])),
        ("extra".to_string(), semio_framework_value::DslValue::Bool(true)),
    ]);
    assert!(TxtMutation::from_value(bad).is_err());
}

#[test]
fn aggregate_roster_is_exact() {
    let roster = [
        ("set-trailing-newline", "SetTrailingNewline", <SetTrailingNewlineMutation as protocol::MutationLeaf>::DESCRIPTOR),
        ("set-line-ending", "SetLineEnding", <SetLineEndingMutation as protocol::MutationLeaf>::DESCRIPTOR),
        ("insert-line", "InsertLine", <InsertLineMutation as protocol::MutationLeaf>::DESCRIPTOR),
        ("remove-line", "RemoveLine", <RemoveLineMutation as protocol::MutationLeaf>::DESCRIPTOR),
        ("set-line", "SetLine", <SetLineMutation as protocol::MutationLeaf>::DESCRIPTOR),
        ("splice-text", "SpliceText", <SpliceTextMutation as protocol::MutationLeaf>::DESCRIPTOR),
    ];
    assert_eq!(TxtMutation::kinds().iter().map(|semantic| semantic.kind).collect::<Vec<_>>(), roster.map(|(kind, _, _)| kind));
    for (kind, variant, descriptor) in roster {
        assert_eq!(descriptor.semantic_kind, kind);
        assert_eq!(descriptor.aggregate_variant, variant);
    }
}

/// ⚖️ `mutation_inverse_sum_law`: for every leaf the inverse diffs sum to the negative forward diff.
#[semio_framework_async_macros::async_test]
async fn mutation_inverse_sum_law_holds_for_every_leaf() {
    let base = crate::TxtSnapshot::from_body("alpha\nbeta\ngamma\n");
    for mutation in [
        TxtMutation::SetTrailingNewline(SetTrailingNewlineMutation { value: false }),
        TxtMutation::SetLineEnding(SetLineEndingMutation { value: crate::schema::snapshot::LineEnding::CrLf }),
        TxtMutation::InsertLine(InsertLineMutation { index: 1, text: "inserted".into() }),
        TxtMutation::InsertLine(InsertLineMutation { index: 3, text: "appended".into() }),
        TxtMutation::RemoveLine(RemoveLineMutation { index: 0 }),
        TxtMutation::SetLine(SetLineMutation { index: 2, text: "changed".into() }),
        TxtMutation::SpliceText(SpliceTextMutation { splices: vec![TextSplice { offset: 6, delete: 4, insert: "BETA".into() }, TextSplice { offset: 11, delete: 0, insert: "new\n".into() }] }),
        TxtMutation::SpliceText(SpliceTextMutation { splices: vec![TextSplice { offset: 5, delete: 1, insert: String::new() }] }),
        TxtMutation::SpliceText(SpliceTextMutation { splices: vec![TextSplice { offset: 17, delete: 0, insert: "tail".into() }] }),
    ] {
        protocol::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
}
