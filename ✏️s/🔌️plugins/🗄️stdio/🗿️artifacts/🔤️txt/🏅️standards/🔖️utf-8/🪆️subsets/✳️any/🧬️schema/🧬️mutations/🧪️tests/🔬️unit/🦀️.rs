
use super::*;
use dsl::FromValue;
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
    let good = dsl::DslValue::object([
        ("mutation".to_string(), dsl::DslValue::String("set-line".to_string())),
        ("payload".to_string(), dsl::DslValue::object([("index".to_string(), dsl::DslValue::uint(1)), ("text".to_string(), dsl::DslValue::String("a".to_string()))])),
    ]);
    assert_eq!(TxtMutation::from_value(good), Ok(TxtMutation::SetLine(SetLineMutation { index: 1, text: "a".to_string() })));
    let bad = dsl::DslValue::object([
        ("mutation".to_string(), dsl::DslValue::String("set-line".to_string())),
        ("payload".to_string(), dsl::DslValue::object([("index".to_string(), dsl::DslValue::uint(1)), ("text".to_string(), dsl::DslValue::String("a".to_string()))])),
        ("extra".to_string(), dsl::DslValue::Bool(true)),
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
    ];
    assert_eq!(TxtMutation::kinds().iter().map(|semantic| semantic.kind).collect::<Vec<_>>(), roster.map(|(kind, _, _)| kind));
    for (kind, variant, descriptor) in roster {
        assert_eq!(descriptor.semantic_kind, kind);
        assert_eq!(descriptor.aggregate_variant, variant);
    }
}
