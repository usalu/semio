use super::*;
use protocol::SemanticMutation;

#[test]
fn direct_owners_descriptors_surfaces_and_catalog_correspond() {
    let mutation_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
    let descriptor_kinds: Vec<_> = RewriteRuleMutation::kinds().iter().map(|descriptor| descriptor.kind).collect();
    let catalog_source = std::fs::read_to_string(mutation_root.join("../../🔮️oracles/🔣️.json")).expect("language-neutral oracle catalog");
    let catalog: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&catalog_source, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("language-neutral oracle catalog must be valid JSON");
    let mutation_catalog = &catalog["mutationCatalogs"][0];
    let catalog_kinds: Vec<_> = mutation_catalog["kinds"].as_array().expect("catalog kinds").iter().map(|kind| kind.as_str().expect("string kind")).collect();
    assert_eq!(descriptor_kinds, catalog_kinds);
    let vectors = mutation_catalog["vectors"].as_array().expect("catalog vectors");
    {
        let kind = "change-parameter-binding";
        let aggregate_variant = "ChangeParameterBinding";
        let directory = "🔧️change-parameter";
        let binary_tag = 3;
        let owner = mutation_root.join("🔧️change-parameter");
        let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
        let descriptor_source = std::fs::read_to_string(owner.join("🔣️.json")).expect("direct language-neutral descriptor");
        let descriptor: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&descriptor_source, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("direct descriptor must be valid JSON");
        assert!(descriptor_kinds.contains(&kind), "direct owner {directory} must have a derived descriptor");
        assert!(source.contains("protocol::MutationKind"), "direct owner {directory} must implement its payload");
        assert!(!source.contains(concat!("::", "mutation::")), "direct owner {directory} must not route through a nested mutation module");
        assert_eq!(descriptor["semanticKind"], kind);
        assert_eq!(descriptor["aggregateVariant"], aggregate_variant);
        assert_eq!(descriptor["payloadSchema"], "🧬️schema/🔣️.json");
        assert_eq!(descriptor["textOpcode"], kind);
        assert_eq!(descriptor["binaryTag"], binary_tag);
        assert_eq!(descriptor["invertibility"], "explicit-mutation");
        assert_eq!(descriptor["diffParticipation"], "detect");
        assert_eq!(descriptor["composition"], "atomic");
        assert_eq!(descriptor["requiredLanguageSurfaces"], semio_framework_pack_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
        assert!(descriptor["owner"].as_str().expect("descriptor owner").ends_with(&format!("/🧬️mutations/{directory}")));
        assert!(!descriptor["outcomeClasses"].as_array().expect("descriptor outcome classes").is_empty());
        let payload_schema_source = std::fs::read_to_string(owner.join("🧬️schema/🔣️.json")).expect("direct JSON payload schema");
        let payload_schema: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&payload_schema_source, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("direct payload schema must be valid JSON");
        assert_eq!(payload_schema["title"], aggregate_variant);
        {
            let surface_source = std::fs::read_to_string(owner.join("🟦️.ts")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("🔗️.graphql")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("🛰️.proto")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("📝️text/🦀️.rs")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("💾️binary/🦀️.rs")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory), "direct owner {directory} must correspond to the JSON catalog");
    }
    {
        let kind = "change-rule-layout-point";
        let aggregate_variant = "ChangeRuleLayoutPoint";
        let directory = "📐️change-rule-layout";
        let binary_tag = 5;
        let owner = mutation_root.join("📐️change-rule-layout");
        let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
        let descriptor_source = std::fs::read_to_string(owner.join("🔣️.json")).expect("direct language-neutral descriptor");
        let descriptor: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&descriptor_source, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("direct descriptor must be valid JSON");
        assert!(descriptor_kinds.contains(&kind), "direct owner {directory} must have a derived descriptor");
        assert!(source.contains("protocol::MutationKind"), "direct owner {directory} must implement its payload");
        assert!(!source.contains(concat!("::", "mutation::")), "direct owner {directory} must not route through a nested mutation module");
        assert_eq!(descriptor["semanticKind"], kind);
        assert_eq!(descriptor["aggregateVariant"], aggregate_variant);
        assert_eq!(descriptor["payloadSchema"], "🧬️schema/🔣️.json");
        assert_eq!(descriptor["textOpcode"], kind);
        assert_eq!(descriptor["binaryTag"], binary_tag);
        assert_eq!(descriptor["invertibility"], "explicit-mutation");
        assert_eq!(descriptor["diffParticipation"], "detect");
        assert_eq!(descriptor["composition"], "atomic");
        assert_eq!(descriptor["requiredLanguageSurfaces"], semio_framework_pack_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
        assert!(descriptor["owner"].as_str().expect("descriptor owner").ends_with(&format!("/🧬️mutations/{directory}")));
        assert!(!descriptor["outcomeClasses"].as_array().expect("descriptor outcome classes").is_empty());
        let payload_schema_source = std::fs::read_to_string(owner.join("🧬️schema/🔣️.json")).expect("direct JSON payload schema");
        let payload_schema: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&payload_schema_source, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("direct payload schema must be valid JSON");
        assert_eq!(payload_schema["title"], aggregate_variant);
        {
            let surface_source = std::fs::read_to_string(owner.join("🟦️.ts")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("🔗️.graphql")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("🛰️.proto")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("📝️text/🦀️.rs")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("💾️binary/🦀️.rs")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory), "direct owner {directory} must correspond to the JSON catalog");
    }
    {
        let kind = "edit-working-graph";
        let aggregate_variant = "EditWorkingGraph";
        let directory = "🖼️edit-working-graph";
        let binary_tag = 0;
        let owner = mutation_root.join("🖼️edit-working-graph");
        let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
        let descriptor_source = std::fs::read_to_string(owner.join("🔣️.json")).expect("direct language-neutral descriptor");
        let descriptor: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&descriptor_source, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("direct descriptor must be valid JSON");
        assert!(descriptor_kinds.contains(&kind), "direct owner {directory} must have a derived descriptor");
        assert!(source.contains("protocol::MutationKind"), "direct owner {directory} must implement its payload");
        assert!(!source.contains(concat!("::", "mutation::")), "direct owner {directory} must not route through a nested mutation module");
        assert_eq!(descriptor["semanticKind"], kind);
        assert_eq!(descriptor["aggregateVariant"], aggregate_variant);
        assert_eq!(descriptor["payloadSchema"], "🧬️schema/🔣️.json");
        assert_eq!(descriptor["textOpcode"], kind);
        assert_eq!(descriptor["binaryTag"], binary_tag);
        assert_eq!(descriptor["invertibility"], "explicit-mutation");
        assert_eq!(descriptor["diffParticipation"], "detect");
        assert_eq!(descriptor["composition"], "atomic");
        assert_eq!(descriptor["requiredLanguageSurfaces"], semio_framework_pack_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
        assert!(descriptor["owner"].as_str().expect("descriptor owner").ends_with(&format!("/🧬️mutations/{directory}")));
        assert!(!descriptor["outcomeClasses"].as_array().expect("descriptor outcome classes").is_empty());
        let payload_schema_source = std::fs::read_to_string(owner.join("🧬️schema/🔣️.json")).expect("direct JSON payload schema");
        let payload_schema: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&payload_schema_source, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("direct payload schema must be valid JSON");
        assert_eq!(payload_schema["title"], aggregate_variant);
        {
            let surface_source = std::fs::read_to_string(owner.join("🟦️.ts")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("🔗️.graphql")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("🛰️.proto")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("📝️text/🦀️.rs")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("💾️binary/🦀️.rs")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory), "direct owner {directory} must correspond to the JSON catalog");
    }
    {
        let kind = "edit-lhs";
        let aggregate_variant = "EditLhs";
        let directory = "👈️edit-lhs";
        let binary_tag = 1;
        let owner = mutation_root.join("👈️edit-lhs");
        let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
        let descriptor_source = std::fs::read_to_string(owner.join("🔣️.json")).expect("direct language-neutral descriptor");
        let descriptor: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&descriptor_source, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("direct descriptor must be valid JSON");
        assert!(descriptor_kinds.contains(&kind), "direct owner {directory} must have a derived descriptor");
        assert!(source.contains("protocol::MutationKind"), "direct owner {directory} must implement its payload");
        assert!(!source.contains(concat!("::", "mutation::")), "direct owner {directory} must not route through a nested mutation module");
        assert_eq!(descriptor["semanticKind"], kind);
        assert_eq!(descriptor["aggregateVariant"], aggregate_variant);
        assert_eq!(descriptor["payloadSchema"], "🧬️schema/🔣️.json");
        assert_eq!(descriptor["textOpcode"], kind);
        assert_eq!(descriptor["binaryTag"], binary_tag);
        assert_eq!(descriptor["invertibility"], "explicit-mutation");
        assert_eq!(descriptor["diffParticipation"], "detect");
        assert_eq!(descriptor["composition"], "atomic");
        assert_eq!(descriptor["requiredLanguageSurfaces"], semio_framework_pack_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
        assert!(descriptor["owner"].as_str().expect("descriptor owner").ends_with(&format!("/🧬️mutations/{directory}")));
        assert!(!descriptor["outcomeClasses"].as_array().expect("descriptor outcome classes").is_empty());
        let payload_schema_source = std::fs::read_to_string(owner.join("🧬️schema/🔣️.json")).expect("direct JSON payload schema");
        let payload_schema: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&payload_schema_source, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("direct payload schema must be valid JSON");
        assert_eq!(payload_schema["title"], aggregate_variant);
        {
            let surface_source = std::fs::read_to_string(owner.join("🟦️.ts")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("🔗️.graphql")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("🛰️.proto")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("📝️text/🦀️.rs")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("💾️binary/🦀️.rs")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory), "direct owner {directory} must correspond to the JSON catalog");
    }
    {
        let kind = "edit-rhs";
        let aggregate_variant = "EditRhs";
        let directory = "👉️edit-rhs";
        let binary_tag = 2;
        let owner = mutation_root.join("👉️edit-rhs");
        let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
        let descriptor_source = std::fs::read_to_string(owner.join("🔣️.json")).expect("direct language-neutral descriptor");
        let descriptor: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&descriptor_source, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("direct descriptor must be valid JSON");
        assert!(descriptor_kinds.contains(&kind), "direct owner {directory} must have a derived descriptor");
        assert!(source.contains("protocol::MutationKind"), "direct owner {directory} must implement its payload");
        assert!(!source.contains(concat!("::", "mutation::")), "direct owner {directory} must not route through a nested mutation module");
        assert_eq!(descriptor["semanticKind"], kind);
        assert_eq!(descriptor["aggregateVariant"], aggregate_variant);
        assert_eq!(descriptor["payloadSchema"], "🧬️schema/🔣️.json");
        assert_eq!(descriptor["textOpcode"], kind);
        assert_eq!(descriptor["binaryTag"], binary_tag);
        assert_eq!(descriptor["invertibility"], "explicit-mutation");
        assert_eq!(descriptor["diffParticipation"], "detect");
        assert_eq!(descriptor["composition"], "atomic");
        assert_eq!(descriptor["requiredLanguageSurfaces"], semio_framework_pack_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
        assert!(descriptor["owner"].as_str().expect("descriptor owner").ends_with(&format!("/🧬️mutations/{directory}")));
        assert!(!descriptor["outcomeClasses"].as_array().expect("descriptor outcome classes").is_empty());
        let payload_schema_source = std::fs::read_to_string(owner.join("🧬️schema/🔣️.json")).expect("direct JSON payload schema");
        let payload_schema: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&payload_schema_source, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("direct payload schema must be valid JSON");
        assert_eq!(payload_schema["title"], aggregate_variant);
        {
            let surface_source = std::fs::read_to_string(owner.join("🟦️.ts")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("🔗️.graphql")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("🛰️.proto")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("📝️text/🦀️.rs")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("💾️binary/🦀️.rs")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory), "direct owner {directory} must correspond to the JSON catalog");
    }
    {
        let kind = "remove-parameter-binding";
        let aggregate_variant = "RemoveParameterBinding";
        let directory = "🧹️remove-parameter-binding";
        let binary_tag = 4;
        let owner = mutation_root.join("🧹️remove-parameter-binding");
        let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
        let descriptor_source = std::fs::read_to_string(owner.join("🔣️.json")).expect("direct language-neutral descriptor");
        let descriptor: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&descriptor_source, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("direct descriptor must be valid JSON");
        assert!(descriptor_kinds.contains(&kind), "direct owner {directory} must have a derived descriptor");
        assert!(source.contains("protocol::MutationKind"), "direct owner {directory} must implement its payload");
        assert!(!source.contains(concat!("::", "mutation::")), "direct owner {directory} must not route through a nested mutation module");
        assert_eq!(descriptor["semanticKind"], kind);
        assert_eq!(descriptor["aggregateVariant"], aggregate_variant);
        assert_eq!(descriptor["payloadSchema"], "🧬️schema/🔣️.json");
        assert_eq!(descriptor["textOpcode"], kind);
        assert_eq!(descriptor["binaryTag"], binary_tag);
        assert_eq!(descriptor["invertibility"], "explicit-mutation");
        assert_eq!(descriptor["diffParticipation"], "detect");
        assert_eq!(descriptor["composition"], "atomic");
        assert_eq!(descriptor["requiredLanguageSurfaces"], semio_framework_pack_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
        assert!(descriptor["owner"].as_str().expect("descriptor owner").ends_with(&format!("/🧬️mutations/{directory}")));
        assert!(!descriptor["outcomeClasses"].as_array().expect("descriptor outcome classes").is_empty());
        let payload_schema_source = std::fs::read_to_string(owner.join("🧬️schema/🔣️.json")).expect("direct JSON payload schema");
        let payload_schema: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&payload_schema_source, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("direct payload schema must be valid JSON");
        assert_eq!(payload_schema["title"], aggregate_variant);
        {
            let surface_source = std::fs::read_to_string(owner.join("🟦️.ts")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("🔗️.graphql")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("🛰️.proto")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("📝️text/🦀️.rs")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("💾️binary/🦀️.rs")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory), "direct owner {directory} must correspond to the JSON catalog");
    }
    {
        let kind = "remove-rule-layout-point";
        let aggregate_variant = "RemoveRuleLayoutPoint";
        let directory = "🗑️remove-rule-layout";
        let binary_tag = 6;
        let owner = mutation_root.join("🗑️remove-rule-layout");
        let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
        let descriptor_source = std::fs::read_to_string(owner.join("🔣️.json")).expect("direct language-neutral descriptor");
        let descriptor: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&descriptor_source, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("direct descriptor must be valid JSON");
        assert!(descriptor_kinds.contains(&kind), "direct owner {directory} must have a derived descriptor");
        assert!(source.contains("protocol::MutationKind"), "direct owner {directory} must implement its payload");
        assert!(!source.contains(concat!("::", "mutation::")), "direct owner {directory} must not route through a nested mutation module");
        assert_eq!(descriptor["semanticKind"], kind);
        assert_eq!(descriptor["aggregateVariant"], aggregate_variant);
        assert_eq!(descriptor["payloadSchema"], "🧬️schema/🔣️.json");
        assert_eq!(descriptor["textOpcode"], kind);
        assert_eq!(descriptor["binaryTag"], binary_tag);
        assert_eq!(descriptor["invertibility"], "explicit-mutation");
        assert_eq!(descriptor["diffParticipation"], "detect");
        assert_eq!(descriptor["composition"], "atomic");
        assert_eq!(descriptor["requiredLanguageSurfaces"], semio_framework_pack_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
        assert!(descriptor["owner"].as_str().expect("descriptor owner").ends_with(&format!("/🧬️mutations/{directory}")));
        assert!(!descriptor["outcomeClasses"].as_array().expect("descriptor outcome classes").is_empty());
        let payload_schema_source = std::fs::read_to_string(owner.join("🧬️schema/🔣️.json")).expect("direct JSON payload schema");
        let payload_schema: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&payload_schema_source, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("direct payload schema must be valid JSON");
        assert_eq!(payload_schema["title"], aggregate_variant);
        {
            let surface_source = std::fs::read_to_string(owner.join("🟦️.ts")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("🔗️.graphql")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("🛰️.proto")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("📝️text/🦀️.rs")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("💾️binary/🦀️.rs")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory), "direct owner {directory} must correspond to the JSON catalog");
    }
    {
        let kind = "drag-rule-nodes";
        let aggregate_variant = "DragRuleNodes";
        let directory = "🫳️drag-rule";
        let binary_tag = 7;
        let owner = mutation_root.join("🫳️drag-rule");
        let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
        let descriptor_source = std::fs::read_to_string(owner.join("🔣️.json")).expect("direct language-neutral descriptor");
        let descriptor: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&descriptor_source, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("direct descriptor must be valid JSON");
        assert!(descriptor_kinds.contains(&kind), "direct owner {directory} must have a derived descriptor");
        assert!(source.contains("protocol::MutationKind"), "direct owner {directory} must implement its payload");
        assert!(!source.contains(concat!("::", "mutation::")), "direct owner {directory} must not route through a nested mutation module");
        assert_eq!(descriptor["semanticKind"], kind);
        assert_eq!(descriptor["aggregateVariant"], aggregate_variant);
        assert_eq!(descriptor["payloadSchema"], "🧬️schema/🔣️.json");
        assert_eq!(descriptor["textOpcode"], kind);
        assert_eq!(descriptor["binaryTag"], binary_tag);
        assert_eq!(descriptor["invertibility"], "explicit-mutation");
        assert_eq!(descriptor["diffParticipation"], "detect");
        assert_eq!(descriptor["composition"], "atomic");
        assert_eq!(descriptor["requiredLanguageSurfaces"], semio_framework_pack_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
        assert!(descriptor["owner"].as_str().expect("descriptor owner").ends_with(&format!("/🧬️mutations/{directory}")));
        assert!(!descriptor["outcomeClasses"].as_array().expect("descriptor outcome classes").is_empty());
        let payload_schema_source = std::fs::read_to_string(owner.join("🧬️schema/🔣️.json")).expect("direct JSON payload schema");
        let payload_schema: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&payload_schema_source, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("direct payload schema must be valid JSON");
        assert_eq!(payload_schema["title"], aggregate_variant);
        {
            let surface_source = std::fs::read_to_string(owner.join("🟦️.ts")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("🔗️.graphql")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("🛰️.proto")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("📝️text/🦀️.rs")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("💾️binary/🦀️.rs")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory), "direct owner {directory} must correspond to the JSON catalog");
    }
    {
        let kind = "set-rule-layout-points";
        let aggregate_variant = "SetRuleLayoutPoints";
        let directory = "📍️set-rule-layout";
        let binary_tag = 8;
        let owner = mutation_root.join("📍️set-rule-layout");
        let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
        let descriptor_source = std::fs::read_to_string(owner.join("🔣️.json")).expect("direct language-neutral descriptor");
        let descriptor: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&descriptor_source, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("direct descriptor must be valid JSON");
        assert!(descriptor_kinds.contains(&kind), "direct owner {directory} must have a derived descriptor");
        assert!(source.contains("protocol::MutationKind"), "direct owner {directory} must implement its payload");
        assert!(!source.contains(concat!("::", "mutation::")), "direct owner {directory} must not route through a nested mutation module");
        assert_eq!(descriptor["semanticKind"], kind);
        assert_eq!(descriptor["aggregateVariant"], aggregate_variant);
        assert_eq!(descriptor["payloadSchema"], "🧬️schema/🔣️.json");
        assert_eq!(descriptor["textOpcode"], kind);
        assert_eq!(descriptor["binaryTag"], binary_tag);
        assert_eq!(descriptor["invertibility"], "explicit-mutation");
        assert_eq!(descriptor["diffParticipation"], "detect");
        assert_eq!(descriptor["composition"], "atomic");
        assert_eq!(descriptor["requiredLanguageSurfaces"], semio_framework_pack_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
        assert!(descriptor["owner"].as_str().expect("descriptor owner").ends_with(&format!("/🧬️mutations/{directory}")));
        assert!(!descriptor["outcomeClasses"].as_array().expect("descriptor outcome classes").is_empty());
        let payload_schema_source = std::fs::read_to_string(owner.join("🧬️schema/🔣️.json")).expect("direct JSON payload schema");
        let payload_schema: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&payload_schema_source, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("direct payload schema must be valid JSON");
        assert_eq!(payload_schema["title"], aggregate_variant);
        {
            let surface_source = std::fs::read_to_string(owner.join("🟦️.ts")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("🔗️.graphql")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("🛰️.proto")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("📝️text/🦀️.rs")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        {
            let surface_source = std::fs::read_to_string(owner.join("💾️binary/🦀️.rs")).expect("direct mutation surface");
            assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
        }
        assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory), "direct owner {directory} must correspond to the JSON catalog");
    }
    assert!(!catalog_kinds.contains(&"set-state") && !catalog_kinds.contains(&"set-snapshot"));
}

#[test]
fn rewriting_declared_json_mutations_and_sparse_deltas_preserve_roles(){
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any");
 let catalog:serde_json::Value=serde_json::from_str(&std::fs::read_to_string(root.join("🔮️oracles/🔣️.json")).unwrap()).unwrap();
 let vectors=catalog["mutationCatalogs"][0]["vectors"].as_array().unwrap();assert_eq!(vectors.len(),9);
 let mut scenarios=0;
 for vector in vectors{
  let directory=vector["mutationDirectoryName"].as_str().unwrap();
  for scenario in vector["scenarios"].as_array().unwrap(){
   let case=root.join("🧫️fixtures/🧬️mutations").join(directory).join(scenario["directoryName"].as_str().unwrap());
   let mutation_json=std::fs::read_to_string(case.join("🦠️mutation/🔣️.json")).unwrap();
   let mutation=crate::standards::v1::subsets::any::io::text::mutations::decode_rewriting_mutation_json(&mutation_json).unwrap();
   let encoded=crate::standards::v1::subsets::any::io::text::mutations::encode_rewriting_mutation_json(&mutation).unwrap();
   assert_eq!(serde_json::from_str::<serde_json::Value>(&encoded).unwrap(),serde_json::from_str::<serde_json::Value>(&mutation_json).unwrap(),"{directory}: mutation declared roles");
   let diff_json=std::fs::read_to_string(case.join("🔺️diff/🔣️.json")).unwrap();
   let diff=crate::standards::v1::subsets::any::schema::diff::decode_rewriting_diff_json(&diff_json).unwrap();
   let encoded=crate::standards::v1::subsets::any::schema::diff::encode_rewriting_diff_json(&diff).unwrap();
   assert_eq!(serde_json::from_str::<serde_json::Value>(&encoded).unwrap(),serde_json::from_str::<serde_json::Value>(&diff_json).unwrap(),"{directory}: sparse declared roles");
   scenarios+=1;
  }
 }
 assert_eq!(scenarios,9);
 for text in[r#"{"mutation":"editLhs","newLhs":"serialized carrier"}"#,r#"{"mutation":"editRhs","newRhs":"serialized carrier"}"#,r#"{"mutation":"editWorkingGraph","newWorkingGraph":"serialized carrier"}"#]{
  let refusal=crate::standards::v1::subsets::any::io::text::mutations::decode_rewriting_mutation_json(text).unwrap_err();
  assert_eq!(refusal.kind,semio_framework_value::ValueRefusalKind::InvalidValue);
 }
}
