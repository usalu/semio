//! 🗂️ Sourcing curate semantic mutation aggregate.

use crate::artifacts::curate::{CurateDiff, CurateSnapshot};
use serde::{Deserialize, Serialize};

pub use super::change_curated_item_count::{change_curated_item_count, ChangeCuratedItemCount};
pub use super::create_curated_item::{create_curated_item, CreateCuratedItem};
pub use super::delete_curated_item::{delete_curated_item, DeleteCuratedItem};
pub use crate::artifacts::curate::schema::operations::*;

//#region 🔖️Aggregate
/// 🧮️ Closed curated-selection mutation vocabulary backed by direct semantic owners.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = CurateSnapshot, diff = CurateDiff, schema = "sourcing.curate")]
pub enum SourcingMutation {
    CreateCuratedItem(CreateCuratedItem),
    DeleteCuratedItem(DeleteCuratedItem),
    ChangeCuratedItemCount(ChangeCuratedItemCount),
}
//#endregion 🔖️Aggregate

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
mod structural_correspondence_tests {
    use super::*;

    #[test]
    fn direct_owners_descriptors_surfaces_and_catalog_correspond() {
        let mutation_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
        let descriptor_kinds: Vec<_> = <SourcingMutation as protocol::SemanticMutation<CurateSnapshot>>::kinds().iter().map(|descriptor| descriptor.kind).collect();
        let catalog: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(mutation_root.join("../../🔣️oracle.json")).expect("language-neutral catalog")).expect("valid catalog");
        let mutation_catalog = &catalog["mutationCatalogs"][0];
        let catalog_kinds: Vec<_> = mutation_catalog["kinds"].as_array().expect("catalog kinds").iter().map(|kind| kind.as_str().expect("string kind")).collect();
        assert_eq!(descriptor_kinds, catalog_kinds);
        let vectors = mutation_catalog["vectors"].as_array().expect("catalog vectors");
        {
            let kind = "create-curated-item";
            let variant = "CreateCuratedItem";
            let directory = "🌱create-curated-item";
            let tag = 0;
            let outcomes = &["applied", "fatal"][..];
            let owner = mutation_root.join("🌱create-curated-item");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️.schema.json");
            assert_eq!(descriptor["textOpcode"], kind);
            assert_eq!(descriptor["binaryTag"], tag);
            assert_eq!(descriptor["outcomeClasses"], serde_json::json!(outcomes));
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
            {
                assert!(owner.join("🔣️.schema.json").is_file());
            }
            {
                assert!(owner.join("🟦️.ts").is_file());
            }
            {
                assert!(owner.join("🔗️.graphql").is_file());
            }
            {
                assert!(owner.join("🛰️.proto").is_file());
            }
            {
                assert!(owner.join("📝️text/🦀️.rs").is_file());
            }
            {
                assert!(owner.join("💾️binary/🦀️.rs").is_file());
            }
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
        }
        {
            let kind = "delete-curated-item";
            let variant = "DeleteCuratedItem";
            let directory = "🗑️delete-curated-item";
            let tag = 1;
            let outcomes = &["applied", "error"][..];
            let owner = mutation_root.join("🗑️delete-curated-item");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️.schema.json");
            assert_eq!(descriptor["textOpcode"], kind);
            assert_eq!(descriptor["binaryTag"], tag);
            assert_eq!(descriptor["outcomeClasses"], serde_json::json!(outcomes));
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
            {
                assert!(owner.join("🔣️.schema.json").is_file());
            }
            {
                assert!(owner.join("🟦️.ts").is_file());
            }
            {
                assert!(owner.join("🔗️.graphql").is_file());
            }
            {
                assert!(owner.join("🛰️.proto").is_file());
            }
            {
                assert!(owner.join("📝️text/🦀️.rs").is_file());
            }
            {
                assert!(owner.join("💾️binary/🦀️.rs").is_file());
            }
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
        }
        {
            let kind = "change-curated-item-count";
            let variant = "ChangeCuratedItemCount";
            let directory = "🔢change-curated-item-count";
            let tag = 2;
            let outcomes = &["applied", "warning", "error"][..];
            let owner = mutation_root.join("🔢change-curated-item-count");
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️.schema.json");
            assert_eq!(descriptor["textOpcode"], kind);
            assert_eq!(descriptor["binaryTag"], tag);
            assert_eq!(descriptor["outcomeClasses"], serde_json::json!(outcomes));
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
            {
                assert!(owner.join("🔣️.schema.json").is_file());
            }
            {
                assert!(owner.join("🟦️.ts").is_file());
            }
            {
                assert!(owner.join("🔗️.graphql").is_file());
            }
            {
                assert!(owner.join("🛰️.proto").is_file());
            }
            {
                assert!(owner.join("📝️text/🦀️.rs").is_file());
            }
            {
                assert!(owner.join("💾️binary/🦀️.rs").is_file());
            }
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
        }
    }
}
//#endregion 🧪️StructuralCorrespondence
