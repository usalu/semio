//! 📜️ Imperative semantic mutation aggregate.
//!
//! Every variant wraps the payload owned by its direct `<mutation>/🦀️component.rs` leaf.

use crate::artifacts::imperative::diff::ImperativeDiff;
use crate::artifacts::imperative::ImperativeSnapshot;
use serde::{Deserialize, Serialize};

pub use super::create_step::{create_step, CreateStep};
pub use super::delete_step::{delete_step, DeleteStep};
pub use super::edit_step_params::{edit_step_params, EditStepParams};
pub use super::reorder_steps::{reorder_steps, ReorderSteps};
pub use crate::artifacts::imperative::standards::v1::subsets::any::schema::operations::*;

//#region 🔖️Aggregate
/// 🧮️ Semantic Imperative document mutation vocabulary.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = ImperativeSnapshot, diff = ImperativeDiff, schema = "imperative.imperative")]
pub enum ImperativeMutation {
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
        let direct_owners = [
            ("create-step", "CreateStep", "🌱create-step", 0, &["applied", "fatal"][..]),
            ("delete-step", "DeleteStep", "🗑️delete-step", 1, &["applied", "error"][..]),
            ("reorder-steps", "ReorderSteps", "🔀reorder-steps", 2, &["applied", "warning", "error"][..]),
            ("edit-step-params", "EditStepParams", "🔧edit-step-params", 3, &["applied", "warning", "error"][..]),
        ];
        let mutation_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
        let descriptor_kinds: Vec<_> = <ImperativeMutation as protocol::SemanticMutation<ImperativeSnapshot>>::kinds().iter().map(|descriptor| descriptor.kind).collect();
        let catalog_source = std::fs::read_to_string(mutation_root.join("../../🧪️oracle/🔣️.json")).expect("language-neutral oracle catalog");
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
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
            let payload: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️payload.schema.json")).expect("direct payload schema")).expect("valid direct payload schema");
            assert_eq!(payload["title"], variant);
            for surface in ["🟦️component.ts", "🔗️component.graphql", "🛰️component.proto", "📝️text/🦀️component.rs", "💾️binary/🦀️component.rs"] {
                let surface_source = std::fs::read_to_string(owner.join(surface)).expect("direct language surface");
                assert!(surface_source.contains(kind) || surface_source.contains(variant));
            }
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
        }
    }
}
//#endregion 🧪️StructuralCorrespondence
