//! ♻️ `trinity.rewrite.rule` semantic mutation aggregate.
//!
//! Every variant wraps the payload owned by its direct `<mutation>/🦀️component.rs` leaf.

use crate::artifacts::rewrite::diff::RewriteDiff;
use crate::artifacts::rewrite::RewriteSnapshot;
use serde::{Deserialize, Serialize};

pub use super::change_parameter_binding::{change_parameter_binding, ChangeParameterBinding};
pub use super::change_rule_layout_point::{change_rule_layout_point, ChangeRuleLayoutPoint};
pub use super::edit_before_fixture::{edit_before_fixture, EditBeforeFixture};
pub use super::edit_lhs::{edit_lhs, EditLhs};
pub use super::edit_rhs::{edit_rhs, EditRhs};
pub use super::remove_parameter_binding::{remove_parameter_binding, RemoveParameterBinding};
pub use super::remove_rule_layout_point::{remove_rule_layout_point, RemoveRuleLayoutPoint};

//#region 🔖️Aggregate
/// 🧮️ Semantic rewrite-rule mutation vocabulary.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = RewriteSnapshot, diff = RewriteDiff, schema = "s.trinity.rewrite")]
pub enum RewriteRuleMutation {
    EditBeforeFixture(EditBeforeFixture),
    EditLhs(EditLhs),
    EditRhs(EditRhs),
    ChangeParameterBinding(ChangeParameterBinding),
    RemoveParameterBinding(RemoveParameterBinding),
    ChangeRuleLayoutPoint(ChangeRuleLayoutPoint),
    RemoveRuleLayoutPoint(RemoveRuleLayoutPoint),
}
//#endregion 🔖️Aggregate

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
mod structural_correspondence_tests {
    use super::*;
    use protocol::SemanticMutation;

    #[test]
    fn direct_owners_descriptors_surfaces_and_catalog_correspond() {
        let mutation_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
        let descriptor_kinds: Vec<_> = RewriteRuleMutation::kinds().iter().map(|descriptor| descriptor.kind).collect();
        let catalog_source = std::fs::read_to_string(mutation_root.join("../../🧪️oracle/🔣️.json")).expect("language-neutral oracle catalog");
        let catalog: serde_json::Value = serde_json::from_str(&catalog_source).expect("language-neutral oracle catalog must be valid JSON");
        let mutation_catalog = &catalog["mutationCatalogs"][0];
        let catalog_kinds: Vec<_> = mutation_catalog["kinds"].as_array().expect("catalog kinds").iter().map(|kind| kind.as_str().expect("string kind")).collect();
        assert_eq!(descriptor_kinds, catalog_kinds);
        let vectors = mutation_catalog["vectors"].as_array().expect("catalog vectors");
        {
            let kind = "change-parameter-binding";
            let aggregate_variant = "ChangeParameterBinding";
            let directory = "🔧️change-parameter-binding";
            let binary_tag = 3;
            let owner = mutation_root.join("🔧️change-parameter-binding");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor_source = std::fs::read_to_string(owner.join("🔣️component.json")).expect("direct language-neutral descriptor");
            let descriptor: serde_json::Value = serde_json::from_str(&descriptor_source).expect("direct descriptor must be valid JSON");
            assert!(descriptor_kinds.contains(&kind), "direct owner {directory} must have a derived descriptor");
            assert!(source.contains("protocol::MutationKind"), "direct owner {directory} must implement its payload");
            assert!(!source.contains(concat!("::", "mutation")), "direct owner {directory} must not route through a nested mutation module");
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], aggregate_variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️payload.schema.json");
            assert_eq!(descriptor["textOpcode"], kind);
            assert_eq!(descriptor["binaryTag"], binary_tag);
            assert_eq!(descriptor["invertibility"], "explicit-mutation");
            assert_eq!(descriptor["diffParticipation"], "detect");
            assert_eq!(descriptor["composition"], "atomic");
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
            assert!(descriptor["owner"].as_str().expect("descriptor owner").ends_with(&format!("/🧬️mutations/{directory}")));
            assert!(!descriptor["outcomeClasses"].as_array().expect("descriptor outcome classes").is_empty());
            let payload_schema_source = std::fs::read_to_string(owner.join("🔣️payload.schema.json")).expect("direct JSON payload schema");
            let payload_schema: serde_json::Value = serde_json::from_str(&payload_schema_source).expect("direct payload schema must be valid JSON");
            assert_eq!(payload_schema["title"], aggregate_variant);
            {
                let surface_source = std::fs::read_to_string(owner.join("🟦️component.ts")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🔗️component.graphql")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🛰️component.proto")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("📝️text/🦀️component.rs")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("💾️binary/🦀️component.rs")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory), "direct owner {directory} must correspond to the JSON catalog");
        }
        {
            let kind = "change-rule-layout-point";
            let aggregate_variant = "ChangeRuleLayoutPoint";
            let directory = "📐️change-rule-layout-point";
            let binary_tag = 5;
            let owner = mutation_root.join("📐️change-rule-layout-point");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor_source = std::fs::read_to_string(owner.join("🔣️component.json")).expect("direct language-neutral descriptor");
            let descriptor: serde_json::Value = serde_json::from_str(&descriptor_source).expect("direct descriptor must be valid JSON");
            assert!(descriptor_kinds.contains(&kind), "direct owner {directory} must have a derived descriptor");
            assert!(source.contains("protocol::MutationKind"), "direct owner {directory} must implement its payload");
            assert!(!source.contains(concat!("::", "mutation")), "direct owner {directory} must not route through a nested mutation module");
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], aggregate_variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️payload.schema.json");
            assert_eq!(descriptor["textOpcode"], kind);
            assert_eq!(descriptor["binaryTag"], binary_tag);
            assert_eq!(descriptor["invertibility"], "explicit-mutation");
            assert_eq!(descriptor["diffParticipation"], "detect");
            assert_eq!(descriptor["composition"], "atomic");
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
            assert!(descriptor["owner"].as_str().expect("descriptor owner").ends_with(&format!("/🧬️mutations/{directory}")));
            assert!(!descriptor["outcomeClasses"].as_array().expect("descriptor outcome classes").is_empty());
            let payload_schema_source = std::fs::read_to_string(owner.join("🔣️payload.schema.json")).expect("direct JSON payload schema");
            let payload_schema: serde_json::Value = serde_json::from_str(&payload_schema_source).expect("direct payload schema must be valid JSON");
            assert_eq!(payload_schema["title"], aggregate_variant);
            {
                let surface_source = std::fs::read_to_string(owner.join("🟦️component.ts")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🔗️component.graphql")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🛰️component.proto")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("📝️text/🦀️component.rs")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("💾️binary/🦀️component.rs")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory), "direct owner {directory} must correspond to the JSON catalog");
        }
        {
            let kind = "edit-before-fixture";
            let aggregate_variant = "EditBeforeFixture";
            let directory = "🖼️edit-before-fixture";
            let binary_tag = 0;
            let owner = mutation_root.join("🖼️edit-before-fixture");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor_source = std::fs::read_to_string(owner.join("🔣️component.json")).expect("direct language-neutral descriptor");
            let descriptor: serde_json::Value = serde_json::from_str(&descriptor_source).expect("direct descriptor must be valid JSON");
            assert!(descriptor_kinds.contains(&kind), "direct owner {directory} must have a derived descriptor");
            assert!(source.contains("protocol::MutationKind"), "direct owner {directory} must implement its payload");
            assert!(!source.contains(concat!("::", "mutation")), "direct owner {directory} must not route through a nested mutation module");
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], aggregate_variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️payload.schema.json");
            assert_eq!(descriptor["textOpcode"], kind);
            assert_eq!(descriptor["binaryTag"], binary_tag);
            assert_eq!(descriptor["invertibility"], "explicit-mutation");
            assert_eq!(descriptor["diffParticipation"], "detect");
            assert_eq!(descriptor["composition"], "atomic");
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
            assert!(descriptor["owner"].as_str().expect("descriptor owner").ends_with(&format!("/🧬️mutations/{directory}")));
            assert!(!descriptor["outcomeClasses"].as_array().expect("descriptor outcome classes").is_empty());
            let payload_schema_source = std::fs::read_to_string(owner.join("🔣️payload.schema.json")).expect("direct JSON payload schema");
            let payload_schema: serde_json::Value = serde_json::from_str(&payload_schema_source).expect("direct payload schema must be valid JSON");
            assert_eq!(payload_schema["title"], aggregate_variant);
            {
                let surface_source = std::fs::read_to_string(owner.join("🟦️component.ts")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🔗️component.graphql")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🛰️component.proto")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("📝️text/🦀️component.rs")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("💾️binary/🦀️component.rs")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory), "direct owner {directory} must correspond to the JSON catalog");
        }
        {
            let kind = "edit-lhs";
            let aggregate_variant = "EditLhs";
            let directory = "🔍️edit-lhs";
            let binary_tag = 1;
            let owner = mutation_root.join("🔍️edit-lhs");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor_source = std::fs::read_to_string(owner.join("🔣️component.json")).expect("direct language-neutral descriptor");
            let descriptor: serde_json::Value = serde_json::from_str(&descriptor_source).expect("direct descriptor must be valid JSON");
            assert!(descriptor_kinds.contains(&kind), "direct owner {directory} must have a derived descriptor");
            assert!(source.contains("protocol::MutationKind"), "direct owner {directory} must implement its payload");
            assert!(!source.contains(concat!("::", "mutation")), "direct owner {directory} must not route through a nested mutation module");
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], aggregate_variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️payload.schema.json");
            assert_eq!(descriptor["textOpcode"], kind);
            assert_eq!(descriptor["binaryTag"], binary_tag);
            assert_eq!(descriptor["invertibility"], "explicit-mutation");
            assert_eq!(descriptor["diffParticipation"], "detect");
            assert_eq!(descriptor["composition"], "atomic");
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
            assert!(descriptor["owner"].as_str().expect("descriptor owner").ends_with(&format!("/🧬️mutations/{directory}")));
            assert!(!descriptor["outcomeClasses"].as_array().expect("descriptor outcome classes").is_empty());
            let payload_schema_source = std::fs::read_to_string(owner.join("🔣️payload.schema.json")).expect("direct JSON payload schema");
            let payload_schema: serde_json::Value = serde_json::from_str(&payload_schema_source).expect("direct payload schema must be valid JSON");
            assert_eq!(payload_schema["title"], aggregate_variant);
            {
                let surface_source = std::fs::read_to_string(owner.join("🟦️component.ts")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🔗️component.graphql")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🛰️component.proto")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("📝️text/🦀️component.rs")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("💾️binary/🦀️component.rs")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory), "direct owner {directory} must correspond to the JSON catalog");
        }
        {
            let kind = "edit-rhs";
            let aggregate_variant = "EditRhs";
            let directory = "🎯️edit-rhs";
            let binary_tag = 2;
            let owner = mutation_root.join("🎯️edit-rhs");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor_source = std::fs::read_to_string(owner.join("🔣️component.json")).expect("direct language-neutral descriptor");
            let descriptor: serde_json::Value = serde_json::from_str(&descriptor_source).expect("direct descriptor must be valid JSON");
            assert!(descriptor_kinds.contains(&kind), "direct owner {directory} must have a derived descriptor");
            assert!(source.contains("protocol::MutationKind"), "direct owner {directory} must implement its payload");
            assert!(!source.contains(concat!("::", "mutation")), "direct owner {directory} must not route through a nested mutation module");
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], aggregate_variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️payload.schema.json");
            assert_eq!(descriptor["textOpcode"], kind);
            assert_eq!(descriptor["binaryTag"], binary_tag);
            assert_eq!(descriptor["invertibility"], "explicit-mutation");
            assert_eq!(descriptor["diffParticipation"], "detect");
            assert_eq!(descriptor["composition"], "atomic");
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
            assert!(descriptor["owner"].as_str().expect("descriptor owner").ends_with(&format!("/🧬️mutations/{directory}")));
            assert!(!descriptor["outcomeClasses"].as_array().expect("descriptor outcome classes").is_empty());
            let payload_schema_source = std::fs::read_to_string(owner.join("🔣️payload.schema.json")).expect("direct JSON payload schema");
            let payload_schema: serde_json::Value = serde_json::from_str(&payload_schema_source).expect("direct payload schema must be valid JSON");
            assert_eq!(payload_schema["title"], aggregate_variant);
            {
                let surface_source = std::fs::read_to_string(owner.join("🟦️component.ts")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🔗️component.graphql")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🛰️component.proto")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("📝️text/🦀️component.rs")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("💾️binary/🦀️component.rs")).expect("direct mutation surface");
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
            let descriptor_source = std::fs::read_to_string(owner.join("🔣️component.json")).expect("direct language-neutral descriptor");
            let descriptor: serde_json::Value = serde_json::from_str(&descriptor_source).expect("direct descriptor must be valid JSON");
            assert!(descriptor_kinds.contains(&kind), "direct owner {directory} must have a derived descriptor");
            assert!(source.contains("protocol::MutationKind"), "direct owner {directory} must implement its payload");
            assert!(!source.contains(concat!("::", "mutation")), "direct owner {directory} must not route through a nested mutation module");
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], aggregate_variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️payload.schema.json");
            assert_eq!(descriptor["textOpcode"], kind);
            assert_eq!(descriptor["binaryTag"], binary_tag);
            assert_eq!(descriptor["invertibility"], "explicit-mutation");
            assert_eq!(descriptor["diffParticipation"], "detect");
            assert_eq!(descriptor["composition"], "atomic");
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
            assert!(descriptor["owner"].as_str().expect("descriptor owner").ends_with(&format!("/🧬️mutations/{directory}")));
            assert!(!descriptor["outcomeClasses"].as_array().expect("descriptor outcome classes").is_empty());
            let payload_schema_source = std::fs::read_to_string(owner.join("🔣️payload.schema.json")).expect("direct JSON payload schema");
            let payload_schema: serde_json::Value = serde_json::from_str(&payload_schema_source).expect("direct payload schema must be valid JSON");
            assert_eq!(payload_schema["title"], aggregate_variant);
            {
                let surface_source = std::fs::read_to_string(owner.join("🟦️component.ts")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🔗️component.graphql")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🛰️component.proto")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("📝️text/🦀️component.rs")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("💾️binary/🦀️component.rs")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory), "direct owner {directory} must correspond to the JSON catalog");
        }
        {
            let kind = "remove-rule-layout-point";
            let aggregate_variant = "RemoveRuleLayoutPoint";
            let directory = "🗑️remove-rule-layout-point";
            let binary_tag = 6;
            let owner = mutation_root.join("🗑️remove-rule-layout-point");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor_source = std::fs::read_to_string(owner.join("🔣️component.json")).expect("direct language-neutral descriptor");
            let descriptor: serde_json::Value = serde_json::from_str(&descriptor_source).expect("direct descriptor must be valid JSON");
            assert!(descriptor_kinds.contains(&kind), "direct owner {directory} must have a derived descriptor");
            assert!(source.contains("protocol::MutationKind"), "direct owner {directory} must implement its payload");
            assert!(!source.contains(concat!("::", "mutation")), "direct owner {directory} must not route through a nested mutation module");
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], aggregate_variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️payload.schema.json");
            assert_eq!(descriptor["textOpcode"], kind);
            assert_eq!(descriptor["binaryTag"], binary_tag);
            assert_eq!(descriptor["invertibility"], "explicit-mutation");
            assert_eq!(descriptor["diffParticipation"], "detect");
            assert_eq!(descriptor["composition"], "atomic");
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
            assert!(descriptor["owner"].as_str().expect("descriptor owner").ends_with(&format!("/🧬️mutations/{directory}")));
            assert!(!descriptor["outcomeClasses"].as_array().expect("descriptor outcome classes").is_empty());
            let payload_schema_source = std::fs::read_to_string(owner.join("🔣️payload.schema.json")).expect("direct JSON payload schema");
            let payload_schema: serde_json::Value = serde_json::from_str(&payload_schema_source).expect("direct payload schema must be valid JSON");
            assert_eq!(payload_schema["title"], aggregate_variant);
            {
                let surface_source = std::fs::read_to_string(owner.join("🟦️component.ts")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🔗️component.graphql")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🛰️component.proto")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("📝️text/🦀️component.rs")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("💾️binary/🦀️component.rs")).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant));
            }
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory), "direct owner {directory} must correspond to the JSON catalog");
        }
        assert!(!catalog_kinds.contains(&"set-state") && !catalog_kinds.contains(&"set-snapshot"));
    }
}
//#endregion 🧪️StructuralCorrespondence
