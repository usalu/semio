//! 🪐️ S Space semantic mutation aggregate.
//!
//! Every variant wraps the payload owned by its direct `<mutation>/🦀️component.rs` leaf.

use crate::artifacts::space::standards::v1::subsets::any::schema::diff::SSpaceDiff;
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use serde::{Deserialize, Serialize};

pub use super::create_artifact::{create_artifact, CreateArtifact};
pub use super::delete_artifact::{delete_artifact, DeleteArtifact};
pub use super::rename_artifact::{rename_artifact, RenameArtifact};
pub use super::touch_artifact::{touch_artifact, TouchArtifact};
pub use crate::artifacts::space::standards::v1::subsets::any::schema::operations::*;

//#region 🔖️Aggregate
/// 🧮️ Semantic S Space index mutation vocabulary.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = SSpaceSnapshot, diff = SSpaceDiff, schema = "s.space.space")]
pub enum SSpaceMutation {
    CreateArtifact(CreateArtifact),
    DeleteArtifact(DeleteArtifact),
    RenameArtifact(RenameArtifact),
    TouchArtifact(TouchArtifact),
}
//#endregion 🔖️Aggregate

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
mod structural_correspondence_tests {
    use super::*;

    #[test]
    fn direct_owners_descriptors_surfaces_and_catalog_correspond() {
        let mutation_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
        let descriptor_kinds: Vec<_> = <SSpaceMutation as protocol::SemanticMutation<SSpaceSnapshot>>::kinds().iter().map(|descriptor| descriptor.kind).collect();
        let catalog_source = std::fs::read_to_string(mutation_root.join("../../🧪️oracle/🔣️.json")).expect("language-neutral oracle catalog");
        let catalog: serde_json::Value = serde_json::from_str(&catalog_source).expect("valid language-neutral oracle catalog");
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
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️component.json")).expect("direct descriptor")).expect("valid direct descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️payload.schema.json");
            assert_eq!(descriptor["textOpcode"], kind);
            assert_eq!(descriptor["binaryTag"], tag);
            assert_eq!(descriptor["outcomeClasses"], serde_json::json!(outcomes));
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "json-schema", "text", "binary"]));
            let payload: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️payload.schema.json")).expect("direct payload schema")).expect("valid direct payload schema");
            assert_eq!(payload["title"], variant);
            {
                let surface_source = std::fs::read_to_string(owner.join("🟦️component.ts")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("📝️text/🦀️component.rs")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("💾️binary/🦀️component.rs")).expect("direct language surface");
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
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️component.json")).expect("direct descriptor")).expect("valid direct descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️payload.schema.json");
            assert_eq!(descriptor["textOpcode"], kind);
            assert_eq!(descriptor["binaryTag"], tag);
            assert_eq!(descriptor["outcomeClasses"], serde_json::json!(outcomes));
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "json-schema", "text", "binary"]));
            let payload: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️payload.schema.json")).expect("direct payload schema")).expect("valid direct payload schema");
            assert_eq!(payload["title"], variant);
            {
                let surface_source = std::fs::read_to_string(owner.join("🟦️component.ts")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("📝️text/🦀️component.rs")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("💾️binary/🦀️component.rs")).expect("direct language surface");
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
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️component.json")).expect("direct descriptor")).expect("valid direct descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️payload.schema.json");
            assert_eq!(descriptor["textOpcode"], kind);
            assert_eq!(descriptor["binaryTag"], tag);
            assert_eq!(descriptor["outcomeClasses"], serde_json::json!(outcomes));
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "json-schema", "text", "binary"]));
            let payload: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️payload.schema.json")).expect("direct payload schema")).expect("valid direct payload schema");
            assert_eq!(payload["title"], variant);
            {
                let surface_source = std::fs::read_to_string(owner.join("🟦️component.ts")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("📝️text/🦀️component.rs")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("💾️binary/🦀️component.rs")).expect("direct language surface");
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
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️component.json")).expect("direct descriptor")).expect("valid direct descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️payload.schema.json");
            assert_eq!(descriptor["textOpcode"], kind);
            assert_eq!(descriptor["binaryTag"], tag);
            assert_eq!(descriptor["outcomeClasses"], serde_json::json!(outcomes));
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "json-schema", "text", "binary"]));
            let payload: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️payload.schema.json")).expect("direct payload schema")).expect("valid direct payload schema");
            assert_eq!(payload["title"], variant);
            {
                let surface_source = std::fs::read_to_string(owner.join("🟦️component.ts")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("📝️text/🦀️component.rs")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            {
                let surface_source = std::fs::read_to_string(owner.join("💾️binary/🦀️component.rs")).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
        }
    }
}
//#endregion 🧪️StructuralCorrespondence
