//! ⚡️ `trinity.graph` semantic mutation aggregate.
//!
//! Every variant wraps the payload owned by its direct `<mutation>/🦀️component.rs` leaf.

use crate::artifacts::jack::diff::JackDiff;
use crate::artifacts::jack::JackSnapshot;
use serde::{Deserialize, Serialize};

pub use super::change_data_property::{change_data_property, ChangeDataProperty};
pub use super::create_edge::{create_edge, CreateEdge};
pub use super::create_node::{create_node, CreateNode};
pub use super::delete_edge::{delete_edge, DeleteEdge};
pub use super::delete_node::{delete_node, DeleteNode};
pub use super::move_node::{move_node, MoveNode};
pub use super::remove_data_property::{remove_data_property, RemoveDataProperty};
pub use super::rename_node::{rename_node, RenameNode};

//#region 🔖️Aggregate
/// 🧮️ Semantic trinity graph mutation vocabulary.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = JackSnapshot, diff = JackDiff, schema = "s.trinity.jack")]
pub enum TrinityGraphMutation {
    CreateNode(CreateNode),
    DeleteNode(DeleteNode),
    CreateEdge(CreateEdge),
    DeleteEdge(DeleteEdge),
    RenameNode(RenameNode),
    MoveNode(MoveNode),
    ChangeDataProperty(ChangeDataProperty),
    RemoveDataProperty(RemoveDataProperty),
}
//#endregion 🔖️Aggregate

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
mod structural_correspondence_tests {
    use super::*;
    use protocol::SemanticMutation;

    #[test]
    fn direct_owners_descriptors_and_language_neutral_catalog_correspond() {
        let direct_owners = [
            ("change-data-property", "ChangeDataProperty", "🔧️change-data-property", 6),
            ("create-edge", "CreateEdge", "🔗️create-edge", 2),
            ("create-node", "CreateNode", "🌱️create-node", 0),
            ("delete-edge", "DeleteEdge", "✂️delete-edge", 3),
            ("delete-node", "DeleteNode", "🗑️delete-node", 1),
            ("move-node", "MoveNode", "📍️move-node", 5),
            ("remove-data-property", "RemoveDataProperty", "🧹️remove-data-property", 7),
            ("rename-node", "RenameNode", "✏️rename-node", 4),
        ];
        let mutation_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
        let descriptor_kinds: Vec<_> = TrinityGraphMutation::kinds().iter().map(|descriptor| descriptor.kind).collect();
        let catalog_source = std::fs::read_to_string(mutation_root.join("../../🧪️oracle/🔣️component.json")).expect("language-neutral oracle catalog");
        let catalog: serde_json::Value = serde_json::from_str(&catalog_source).expect("language-neutral oracle catalog must be valid JSON");
        let mutation_catalog = &catalog["mutationCatalogs"][0];
        let catalog_kinds: Vec<_> = mutation_catalog["kinds"].as_array().expect("catalog kinds").iter().map(|kind| kind.as_str().expect("string kind")).collect();
        assert_eq!(descriptor_kinds, catalog_kinds);
        let vectors = mutation_catalog["vectors"].as_array().expect("catalog vectors");
        for (kind, aggregate_variant, directory, binary_tag) in direct_owners {
            let owner = mutation_root.join(directory);
            let source = std::fs::read_to_string(owner.join("🦀️component.rs")).expect("direct Rust owner");
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
            for surface in ["🟦️component.ts", "🔗️component.graphql", "🛰️component.proto", "📝️text/🦀️component.rs", "💾️binary/🦀️component.rs"] {
                let surface_source = std::fs::read_to_string(owner.join(surface)).expect("direct mutation surface");
                assert!(surface_source.contains(kind) || surface_source.contains(aggregate_variant), "direct surface {directory}/{surface} must identify its semantic mutation");
            }
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory), "direct owner {directory} must correspond to the JSON catalog");
        }
        assert!(!catalog_kinds.contains(&"set-snapshot"));
    }
}
//#endregion 🧪️StructuralCorrespondence
