use super::*;
use protocol::SemanticMutation;

#[test]
fn aggregate_descriptor_roster_is_exactly_the_direct_leaf_roster() {
    let schema: serde_json::Value = serde_json::from_str(include_str!("../../🔣️.json")).unwrap();
    let expected = schema["oneOf"].as_array().unwrap().iter().map(|leaf| leaf["properties"]["mutation"]["const"].as_str().unwrap().to_owned()).collect::<std::collections::BTreeSet<_>>();
    let actual = GltfMutation::kinds().iter().map(|descriptor| descriptor.kind.split('-').enumerate().map(|(index, word)| if index == 0 { word.to_owned() } else { word[..1].to_uppercase() + &word[1..] }).collect::<String>()).collect::<std::collections::BTreeSet<_>>();
    assert_eq!(GltfMutation::kinds().len(), expected.len());
    assert_eq!(actual, expected);
}

#[test]
fn mutation_rejection_messages_match_the_language_neutral_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📨️mutation-carriers/🔣️.json")).unwrap();
    for case in fixture["rejections"].as_array().unwrap() {
        let outcome = crate::schema::modules::mutation_support::top_level::rejection_outcome(case["code"].as_str().unwrap(), case["path"].as_str().unwrap(), case["detail"].as_str().unwrap().to_owned());
        let encoded: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&outcome)).unwrap();
        assert_eq!(encoded, case["outcome"]);
    }
}

#[test]
fn mutation_restore_preserves_the_language_neutral_wire_and_inverse() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📨️mutation-carriers/🔣️.json")).unwrap();
    let mutation: GltfMutation = semio_framework_pack_json::from_json_str(&fixture["apply"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mut base = GltfSnapshot::default();
    base.document.scene = Some(0);
    base.document.scenes = vec![Default::default(), Default::default()];
    let outcome = <GltfMutation as protocol::Mutation<GltfSnapshot>>::diff(&mutation, &base);
    assert!(outcome.messages().is_empty());
    let next = protocol::MutationDiff::apply(outcome.diff(), &base).unwrap();
    assert_eq!(next.document.scene, Some(1));
    let inverse = <GltfMutation as protocol::Mutation<GltfSnapshot>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse.len(), 1);
    let encoded: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&inverse[0])).unwrap();
    assert_eq!(encoded, fixture["restore"]);
    let restored: GltfMutation = semio_framework_pack_json::from_json_str(&encoded.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let outcome = <GltfMutation as protocol::Mutation<GltfSnapshot>>::diff(&restored, &next);
    assert!(outcome.messages().is_empty());
    assert_eq!(protocol::MutationDiff::apply(outcome.diff(), &next).unwrap(), base);
    assert!(size_of::<GltfMutation>() <= fixture["maximumInlineBytes"].as_u64().unwrap() as usize);
}
