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
        let direct_owners = [
            ("create-artifact", "CreateArtifact", "🌱create-artifact", 0, &["applied", "fatal"][..]),
            ("delete-artifact", "DeleteArtifact", "🗑️delete-artifact", 1, &["applied", "error"][..]),
            ("rename-artifact", "RenameArtifact", "🏷️rename-artifact", 2, &["applied", "warning", "error", "fatal"][..]),
            ("touch-artifact", "TouchArtifact", "🕒touch-artifact", 3, &["applied", "error"][..]),
        ];
        let mutation_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
        let descriptor_kinds: Vec<_> = <SSpaceMutation as protocol::SemanticMutation<SSpaceSnapshot>>::kinds().iter().map(|descriptor| descriptor.kind).collect();
        let catalog_source = std::fs::read_to_string(mutation_root.join("../../🧪️oracle/🔣️component.json")).expect("language-neutral oracle catalog");
        let catalog: serde_json::Value = serde_json::from_str(&catalog_source).expect("valid language-neutral oracle catalog");
        let mutation_catalog = &catalog["mutationCatalogs"][0];
        let catalog_kinds: Vec<_> = mutation_catalog["kinds"].as_array().expect("catalog kinds").iter().map(|kind| kind.as_str().expect("string kind")).collect();
        assert_eq!(descriptor_kinds, catalog_kinds);
        let vectors = mutation_catalog["vectors"].as_array().expect("catalog vectors");
        for (kind, variant, directory, tag, outcomes) in direct_owners {
            let owner = mutation_root.join(directory);
            let source = std::fs::read_to_string(owner.join("🦀️component.rs")).expect("direct Rust owner");
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
            for surface in ["🟦️component.ts", "📝️text/🦀️component.rs", "💾️binary/🦀️component.rs"] {
                let surface_source = std::fs::read_to_string(owner.join(surface)).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
        }
    }
}
//#endregion 🧪️StructuralCorrespondence
