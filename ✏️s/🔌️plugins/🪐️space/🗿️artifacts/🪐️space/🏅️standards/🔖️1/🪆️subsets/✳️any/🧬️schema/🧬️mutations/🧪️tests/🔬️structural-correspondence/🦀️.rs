
use super::*;

fn outcome_classes(outcomes: &[&str]) -> pack::JsonValue {
    pack::JsonValue::Array(outcomes.iter().map(|outcome| pack::JsonValue::from(*outcome)).collect())
}

#[test]
fn direct_owners_descriptors_surfaces_and_catalog_correspond() {
    let mutation_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
    let descriptor_kinds: Vec<_> = <SSpaceMutation as protocol::SemanticMutation<SSpaceSnapshot>>::kinds().iter().map(|descriptor| descriptor.kind).collect();
    let catalog_source = std::fs::read_to_string(mutation_root.join("../../🔣️oracle.json")).expect("language-neutral oracle catalog");
    let catalog: pack::JsonValue = pack::parse_json(&catalog_source).expect("valid language-neutral oracle catalog");
    let mutation_catalog = &catalog["mutationCatalogs"][0];
    let catalog_kinds: Vec<_> = mutation_catalog["kinds"].as_array().expect("catalog kinds").iter().map(|kind| kind.as_str().expect("string kind")).collect();
    assert_eq!(descriptor_kinds, catalog_kinds);
    let vectors = mutation_catalog["vectors"].as_array().expect("catalog vectors");
    {
        let kind = "create-artifact";
        let variant = "CreateArtifact";
        let directory = "🌱create-artifact";
        let tag = 0;
        let outcomes = &["applied", "fatal"][..];
        let owner = mutation_root.join("🌱create-artifact");
        let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
        let descriptor: pack::JsonValue = pack::parse_json(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid direct descriptor");
        assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
        assert!(!source.contains(concat!("::", "mutation::")));
        assert_eq!(descriptor["semanticKind"], kind);
        assert_eq!(descriptor["aggregateVariant"], variant);
        assert_eq!(descriptor["payloadSchema"], "🧬️schema/🔣️.json");
        assert_eq!(descriptor["textOpcode"], kind);
        assert_eq!(descriptor["binaryTag"], tag);
        assert_eq!(descriptor["outcomeClasses"], outcome_classes(outcomes));
        assert_eq!(descriptor["requiredLanguageSurfaces"], pack::json!(["rust", "typescript", "json-schema", "text", "binary"]));
        let payload: pack::JsonValue = pack::parse_json(&std::fs::read_to_string(owner.join("🧬️schema/🔣️.json")).expect("direct payload schema")).expect("valid direct payload schema");
        assert_eq!(payload["title"], variant);
        {
            let surface_source = std::fs::read_to_string(owner.join("🟦️.ts")).expect("direct language surface");
            assert!(surface_source.contains(kind) || surface_source.contains(variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("📝️text/🦀️.rs")).expect("direct language surface");
            assert!(surface_source.contains(kind) || surface_source.contains(variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("💾️binary/🦀️.rs")).expect("direct language surface");
            assert!(surface_source.contains(kind) || surface_source.contains(variant));
        }
        assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
    }
    {
        let kind = "delete-artifact";
        let variant = "DeleteArtifact";
        let directory = "🗑️delete-artifact";
        let tag = 1;
        let outcomes = &["applied", "error"][..];
        let owner = mutation_root.join("🗑️delete-artifact");
        let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
        let descriptor: pack::JsonValue = pack::parse_json(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid direct descriptor");
        assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
        assert!(!source.contains(concat!("::", "mutation::")));
        assert_eq!(descriptor["semanticKind"], kind);
        assert_eq!(descriptor["aggregateVariant"], variant);
        assert_eq!(descriptor["payloadSchema"], "🧬️schema/🔣️.json");
        assert_eq!(descriptor["textOpcode"], kind);
        assert_eq!(descriptor["binaryTag"], tag);
        assert_eq!(descriptor["outcomeClasses"], outcome_classes(outcomes));
        assert_eq!(descriptor["requiredLanguageSurfaces"], pack::json!(["rust", "typescript", "json-schema", "text", "binary"]));
        let payload: pack::JsonValue = pack::parse_json(&std::fs::read_to_string(owner.join("🧬️schema/🔣️.json")).expect("direct payload schema")).expect("valid direct payload schema");
        assert_eq!(payload["title"], variant);
        {
            let surface_source = std::fs::read_to_string(owner.join("🟦️.ts")).expect("direct language surface");
            assert!(surface_source.contains(kind) || surface_source.contains(variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("📝️text/🦀️.rs")).expect("direct language surface");
            assert!(surface_source.contains(kind) || surface_source.contains(variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("💾️binary/🦀️.rs")).expect("direct language surface");
            assert!(surface_source.contains(kind) || surface_source.contains(variant));
        }
        assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
    }
    {
        let kind = "rename-artifact";
        let variant = "RenameArtifact";
        let directory = "🏷️rename-artifact";
        let tag = 2;
        let outcomes = &["applied", "warning", "error", "fatal"][..];
        let owner = mutation_root.join("🏷️rename-artifact");
        let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
        let descriptor: pack::JsonValue = pack::parse_json(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid direct descriptor");
        assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
        assert!(!source.contains(concat!("::", "mutation::")));
        assert_eq!(descriptor["semanticKind"], kind);
        assert_eq!(descriptor["aggregateVariant"], variant);
        assert_eq!(descriptor["payloadSchema"], "🧬️schema/🔣️.json");
        assert_eq!(descriptor["textOpcode"], kind);
        assert_eq!(descriptor["binaryTag"], tag);
        assert_eq!(descriptor["outcomeClasses"], outcome_classes(outcomes));
        assert_eq!(descriptor["requiredLanguageSurfaces"], pack::json!(["rust", "typescript", "json-schema", "text", "binary"]));
        let payload: pack::JsonValue = pack::parse_json(&std::fs::read_to_string(owner.join("🧬️schema/🔣️.json")).expect("direct payload schema")).expect("valid direct payload schema");
        assert_eq!(payload["title"], variant);
        {
            let surface_source = std::fs::read_to_string(owner.join("🟦️.ts")).expect("direct language surface");
            assert!(surface_source.contains(kind) || surface_source.contains(variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("📝️text/🦀️.rs")).expect("direct language surface");
            assert!(surface_source.contains(kind) || surface_source.contains(variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("💾️binary/🦀️.rs")).expect("direct language surface");
            assert!(surface_source.contains(kind) || surface_source.contains(variant));
        }
        assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
    }
    {
        let kind = "touch-artifact";
        let variant = "TouchArtifact";
        let directory = "🕒touch-artifact";
        let tag = 3;
        let outcomes = &["applied", "error"][..];
        let owner = mutation_root.join("🕒touch-artifact");
        let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
        let descriptor: pack::JsonValue = pack::parse_json(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid direct descriptor");
        assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
        assert!(!source.contains(concat!("::", "mutation::")));
        assert_eq!(descriptor["semanticKind"], kind);
        assert_eq!(descriptor["aggregateVariant"], variant);
        assert_eq!(descriptor["payloadSchema"], "🧬️schema/🔣️.json");
        assert_eq!(descriptor["textOpcode"], kind);
        assert_eq!(descriptor["binaryTag"], tag);
        assert_eq!(descriptor["outcomeClasses"], outcome_classes(outcomes));
        assert_eq!(descriptor["requiredLanguageSurfaces"], pack::json!(["rust", "typescript", "json-schema", "text", "binary"]));
        let payload: pack::JsonValue = pack::parse_json(&std::fs::read_to_string(owner.join("🧬️schema/🔣️.json")).expect("direct payload schema")).expect("valid direct payload schema");
        assert_eq!(payload["title"], variant);
        {
            let surface_source = std::fs::read_to_string(owner.join("🟦️.ts")).expect("direct language surface");
            assert!(surface_source.contains(kind) || surface_source.contains(variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("📝️text/🦀️.rs")).expect("direct language surface");
            assert!(surface_source.contains(kind) || surface_source.contains(variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("💾️binary/🦀️.rs")).expect("direct language surface");
            assert!(surface_source.contains(kind) || surface_source.contains(variant));
        }
        assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
    }
}
