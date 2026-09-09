use super::*;
use protocol::SemanticMutation;

#[test]
fn aggregate_descriptor_roster_is_exactly_the_direct_leaf_roster() {
    assert_eq!(GltfMutation::kinds().len(), 120);
    assert_eq!(GltfMutation::kinds().iter().map(|descriptor| descriptor.kind).collect::<std::collections::BTreeSet<_>>().len(), 120);
}

#[test]
fn mutation_rejection_messages_match_the_language_neutral_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📨️mutation-carriers/🔣️.json")).unwrap();
    for case in fixture["rejections"].as_array().unwrap() {
        let outcome = crate::schema::modules::mutation_support::top_level::rejection_outcome(case["code"].as_str().unwrap(), case["path"].as_str().unwrap(), case["detail"].as_str().unwrap().to_owned());
        let encoded: serde_json::Value = serde_json::from_str(&dsl::json::to_json_string(&outcome)).unwrap();
        assert_eq!(encoded, case["outcome"]);
    }
    println!("[DEBUG] glTF rejection oracle: six classifications preserve severity, code, detail and target.");
}

#[test]
fn mutation_restore_preserves_the_language_neutral_wire_and_inverse() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📨️mutation-carriers/🔣️.json")).unwrap();
    let mutation: GltfMutation = dsl::json::from_json_str(&fixture["apply"].to_string()).unwrap();
    let mut base = GltfSnapshot::default();
    base.document.scene = Some(0);
    base.document.scenes = vec![Default::default(), Default::default()];
    let outcome = <GltfMutation as protocol::Mutation<GltfSnapshot>>::diff(&mutation, &base);
    assert!(outcome.messages().is_empty());
    let next = protocol::MutationDiff::apply(outcome.diff(), &base).unwrap();
    assert_eq!(next.document.scene, Some(1));
    let inverse = <GltfMutation as protocol::Mutation<GltfSnapshot>>::inverse(&mutation, &base);
    assert_eq!(inverse.len(), 1);
    let encoded: serde_json::Value = serde_json::from_str(&dsl::json::to_json_string(&inverse[0])).unwrap();
    assert_eq!(encoded, fixture["restore"]);
    let restored: GltfMutation = dsl::json::from_json_str(&encoded.to_string()).unwrap();
    let outcome = <GltfMutation as protocol::Mutation<GltfSnapshot>>::diff(&restored, &next);
    assert!(outcome.messages().is_empty());
    assert_eq!(protocol::MutationDiff::apply(outcome.diff(), &next).unwrap(), base);
    assert!(size_of::<GltfMutation>() <= fixture["maximumInlineBytes"].as_u64().unwrap() as usize);
    println!("[DEBUG] glTF mutation carrier: committed JSON, forward apply and restored inverse agree.");
}
