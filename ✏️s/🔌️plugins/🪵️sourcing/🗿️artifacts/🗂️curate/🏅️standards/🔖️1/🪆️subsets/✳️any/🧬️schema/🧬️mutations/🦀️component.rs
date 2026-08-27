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
        let direct_owners = [
            ("create-curated-item", "CreateCuratedItem", "🌱create-curated-item", 0, &["applied", "fatal"][..]),
            ("delete-curated-item", "DeleteCuratedItem", "🗑️delete-curated-item", 1, &["applied", "error"][..]),
            ("change-curated-item-count", "ChangeCuratedItemCount", "🔢change-curated-item-count", 2, &["applied", "warning", "error"][..]),
        ];
        let mutation_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
        let descriptor_kinds: Vec<_> = <SourcingMutation as protocol::SemanticMutation<CurateSnapshot>>::kinds().iter().map(|descriptor| descriptor.kind).collect();
        let catalog: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(mutation_root.join("../../🧪️oracle/🔣️component.json")).expect("language-neutral catalog")).expect("valid catalog");
        let mutation_catalog = &catalog["mutationCatalogs"][0];
        let catalog_kinds: Vec<_> = mutation_catalog["kinds"].as_array().expect("catalog kinds").iter().map(|kind| kind.as_str().expect("string kind")).collect();
        assert_eq!(descriptor_kinds, catalog_kinds);
        let vectors = mutation_catalog["vectors"].as_array().expect("catalog vectors");
        for (kind, variant, directory, tag, outcomes) in direct_owners {
            let owner = mutation_root.join(directory);
            let source = std::fs::read_to_string(owner.join("🦀️component.rs")).expect("direct Rust owner");
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️component.json")).expect("direct descriptor")).expect("valid descriptor");
            assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(descriptor["semanticKind"], kind);
            assert_eq!(descriptor["aggregateVariant"], variant);
            assert_eq!(descriptor["payloadSchema"], "🔣️payload.schema.json");
            assert_eq!(descriptor["textOpcode"], kind);
            assert_eq!(descriptor["binaryTag"], tag);
            assert_eq!(descriptor["outcomeClasses"], serde_json::json!(outcomes));
            assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
            for surface in ["🔣️payload.schema.json", "🟦️component.ts", "🔗️component.graphql", "🛰️component.proto", "📝️text/🦀️component.rs", "💾️binary/🦀️component.rs"] {
                assert!(owner.join(surface).is_file(), "missing direct surface {surface}");
            }
            assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
        }
    }
}
//#endregion 🧪️StructuralCorrespondence
