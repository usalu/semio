//! 📜️ Imperative semantic mutation aggregate.
//!
//! Every variant wraps the payload owned by its direct `<mutation>/🦀️.rs` leaf.

use crate::artifacts::procedure::diff::ProcedureDiff;
use crate::artifacts::procedure::ProcedureSnapshot;

pub use super::create_step::{create_step, CreateStep};
pub use super::delete_step::{delete_step, DeleteStep};
pub use super::edit_step_params::{edit_step_params, EditStepParams};
pub use super::reorder_steps::{reorder_steps, ReorderSteps};
pub use crate::artifacts::procedure::standards::v1::subsets::any::schema::operations::*;

//#region 🔖️Aggregate
/// 🧮️ Semantic Imperative document mutation vocabulary.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = ProcedureSnapshot, diff = ProcedureDiff, schema = "imperative.imperative")]
pub enum ProcedureMutation {
    CreateStep(CreateStep),
    DeleteStep(DeleteStep),
    ReorderSteps(ReorderSteps),
    EditStepParams(EditStepParams),
}
//#endregion 🔖️Aggregate

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
mod structural_correspondence_tests {
    use super::*;

    #[test]
    fn direct_owners_descriptors_surfaces_and_catalog_correspond() {
        let mutation_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
        let descriptor_kinds: Vec<_> = <ProcedureMutation as protocol::SemanticMutation<ProcedureSnapshot>>::kinds().iter().map(|descriptor| descriptor.kind).collect();
        let catalog_source = std::fs::read_to_string(mutation_root.join("../../🔣️oracle.json")).expect("language-neutral oracle catalog");
        let catalog: serde_json::Value = serde_json::from_str(&catalog_source).expect("valid language-neutral oracle catalog");
        let mutation_catalog = &catalog["mutationCatalogs"][0];
        let catalog_kinds: Vec<_> = mutation_catalog["kinds"].as_array().expect("catalog kinds").iter().map(|kind| kind.as_str().expect("string kind")).collect();
        assert_eq!(descriptor_kinds, catalog_kinds);
        let vectors = mutation_catalog["vectors"].as_array().expect("catalog vectors");
        {
            let kind = "create-step";
            let variant = "CreateStep";
            let directory = "🌱create-step";
            let tag = 0;
            let outcomes = &["applied", "fatal"][..];
            let owner = mutation_root.join("🌱create-step");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid direct descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️.schema.json");
            assert_eq!(descriptor["textOpcode"], kind);
            assert_eq!(descriptor["binaryTag"], tag);
            assert_eq!(descriptor["outcomeClasses"], serde_json::json!(outcomes));
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
            let payload: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.schema.json")).expect("direct payload schema")).expect("valid direct payload schema");
            assert_eq!(payload["title"], variant);
            {
                let surface_source = std::fs::read_to_string(owner.join("🟦️.ts")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🔗️.graphql")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🛰️.proto")).expect("direct language surface");
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
            let kind = "delete-step";
            let variant = "DeleteStep";
            let directory = "🗑️delete-step";
            let tag = 1;
            let outcomes = &["applied", "error"][..];
            let owner = mutation_root.join("🗑️delete-step");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid direct descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️.schema.json");
            assert_eq!(descriptor["textOpcode"], kind);
            assert_eq!(descriptor["binaryTag"], tag);
            assert_eq!(descriptor["outcomeClasses"], serde_json::json!(outcomes));
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
            let payload: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.schema.json")).expect("direct payload schema")).expect("valid direct payload schema");
            assert_eq!(payload["title"], variant);
            {
                let surface_source = std::fs::read_to_string(owner.join("🟦️.ts")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🔗️.graphql")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🛰️.proto")).expect("direct language surface");
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
            let kind = "reorder-steps";
            let variant = "ReorderSteps";
            let directory = "🔀reorder-steps";
            let tag = 2;
            let outcomes = &["applied", "warning", "error"][..];
            let owner = mutation_root.join("🔀reorder-steps");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid direct descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️.schema.json");
            assert_eq!(descriptor["textOpcode"], kind);
            assert_eq!(descriptor["binaryTag"], tag);
            assert_eq!(descriptor["outcomeClasses"], serde_json::json!(outcomes));
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
            let payload: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.schema.json")).expect("direct payload schema")).expect("valid direct payload schema");
            assert_eq!(payload["title"], variant);
            {
                let surface_source = std::fs::read_to_string(owner.join("🟦️.ts")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🔗️.graphql")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🛰️.proto")).expect("direct language surface");
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
            let kind = "edit-step-params";
            let variant = "EditStepParams";
            let directory = "🔧edit-step-params";
            let tag = 3;
            let outcomes = &["applied", "warning", "error"][..];
            let owner = mutation_root.join("🔧edit-step-params");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid direct descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️.schema.json");
            assert_eq!(descriptor["textOpcode"], kind);
            assert_eq!(descriptor["binaryTag"], tag);
            assert_eq!(descriptor["outcomeClasses"], serde_json::json!(outcomes));
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
            let payload: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.schema.json")).expect("direct payload schema")).expect("valid direct payload schema");
            assert_eq!(payload["title"], variant);
            {
                let surface_source = std::fs::read_to_string(owner.join("🟦️.ts")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🔗️.graphql")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("🛰️.proto")).expect("direct language surface");
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
}
//#endregion 🧪️StructuralCorrespondence
