use super::*;
use protocol::SemanticMutation;

/// 🏷️ `KINDS` must name every declared variant, in the exact order and spelling
/// `#[derive(dsl::Mutations)]` assigns, and every entry must also appear in the committed
/// catalog — the framework reads the manifest and never parses Rust, so this test is the only
/// thing that keeps the two in step. A plain `#[test]`: it suspends on nothing.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    let descriptors = DrawingMutation::kinds();
    assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared DrawingMutation variant");
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
    }
    let manifests: Vec<serde_json::Value> =
        [include_str!("../../../../../🎨️style/🔮️oracle/🔣️.json"), include_str!("../../../../../🏷️metadata/🔮️oracle/🔣️.json"), include_str!("../../../../../🔀️transform/🔮️oracle/🔣️.json"), include_str!("../../../../../🧱️structure/🔮️oracle/🔣️.json")]
            .into_iter()
            .map(|text| serde_json::from_str(text).expect("language-neutral subset oracle"))
            .collect();
    let entries: Vec<_> = manifests.iter().flat_map(|manifest| manifest["mutationManifests"].as_array().unwrap()).flat_map(|manifest| manifest["mutations"].as_array().unwrap()).collect();
    let leaf_descriptors = <DrawingMutation as protocol::Mutation<DrawingSnapshot>>::DESCRIPTORS;
    assert_eq!(entries.len(), leaf_descriptors.len());
    for (index, descriptor) in leaf_descriptors.iter().enumerate() {
        let entry = entries.iter().find(|entry| entry["id"] == descriptor.semantic_kind).expect("every leaf has a declared catalog entry");
        assert_eq!(entry["productionDispatch"]["variant"], descriptor.aggregate_variant);
        assert!(entry["payloadSchema"].as_str().unwrap().ends_with(descriptor.payload_schema));
        assert_eq!(descriptor.text_opcode, Some(KINDS[index]));
        assert_eq!(descriptor.binary_tag, Some(index as u32));
    }
}
