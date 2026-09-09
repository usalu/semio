use super::*;
use protocol::SemanticMutation;

/// 🧭️ Every declared kind owns a leaf directory whose descriptor, payload schema and behaviour
/// facets agree with the aggregate enum AND with the subset's language-neutral oracle catalog —
/// which lives at `✳️any/🔮️oracle/🔣️.json`, never at a flat `🔣️oracle.json`.
#[test]
fn direct_owner_descriptors_and_catalog_correspond() {
    let mutation_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
    let catalog_source = std::fs::read_to_string(mutation_root.join("../../🔮️oracle/🔣️.json")).expect("language-neutral oracle catalog");
    let catalog: pack::json::Value = pack::json::parse(&catalog_source).expect("language-neutral oracle catalog must be valid JSON");
    let catalog_kinds: Vec<String> = catalog["mutationCatalogs"][0]["kinds"].as_array().expect("catalog kinds").iter().map(|kind| kind.as_str().expect("catalog kind is a string").to_string()).collect();
    let descriptors = EnergyModelMutation::kinds();
    assert_eq!(descriptors.len(), KINDS.len());
    assert_eq!(descriptors.iter().map(|descriptor| descriptor.kind).collect::<Vec<_>>(), KINDS.to_vec());
    assert_eq!(catalog_kinds, KINDS.iter().map(|kind| (*kind).to_string()).collect::<Vec<_>>());
    assert_eq!(DIRECTORIES.len(), KINDS.len());
    for (kind, directory) in DIRECTORIES {
        let owner = mutation_root.join(directory);
        let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
        let descriptor_source = std::fs::read_to_string(owner.join("🔣️.json")).expect("direct language-neutral descriptor");
        let descriptor: pack::json::Value = pack::json::parse(&descriptor_source).expect("direct descriptor must be valid JSON");
        let payload_schema_source = std::fs::read_to_string(owner.join("🧬️schema/🔣️.json")).expect("direct payload schema");
        let payload_schema: pack::json::Value = pack::json::parse(&payload_schema_source).expect("direct payload schema must be valid JSON");
        assert!(source.contains("protocol::MutationKind"), "{kind} owns no MutationKind impl");
        assert!(!source.contains(concat!("::", "mutation::")));
        assert_eq!(descriptor["semanticKind"].as_str(), Some(*kind));
        assert_eq!(descriptor["textOpcode"].as_str(), Some(*kind));
        assert_eq!(descriptor["payloadSchema"].as_str(), Some("🧬️schema/🔣️.json"));
        assert_eq!(payload_schema["title"].as_str(), descriptor["aggregateVariant"].as_str());
        assert!(owner.join("🔺️diff/🦀️.rs").exists(), "{kind} owns no diff leaf");
        assert!(owner.join("↩️inverse/🦀️.rs").exists(), "{kind} owns no inverse leaf");
    }
}
