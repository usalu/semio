//! 🏠️ Space Home semantic mutation aggregate.

use crate::artifacts::home::{SHomeDiff, SHomeSnapshot};
use serde::{Deserialize, Serialize};

pub use super::change_catalog_generation::{change_catalog_generation, ChangeCatalogGeneration};
pub use crate::artifacts::home::schema::operations::*;

//#region 🔖️Aggregate
/// 🧮️ Home launcher mutation vocabulary backed by its direct semantic owner.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = SHomeSnapshot, diff = SHomeDiff, schema = "s.space.home")]
pub enum SHomeMutation {
    ChangeCatalogGeneration(ChangeCatalogGeneration),
}
//#endregion 🔖️Aggregate

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
mod structural_correspondence_tests {
    use super::*;

    #[test]
    fn direct_owner_descriptor_surfaces_and_catalog_correspond() {
        let mutation_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
        let owner = mutation_root.join("🔢️change-catalog-generation");
        let source = std::fs::read_to_string(owner.join("🦀️component.rs")).expect("direct Rust owner");
        let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️component.json")).expect("direct descriptor")).expect("valid descriptor");
        let catalog: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(mutation_root.join("../../🧪️oracle/🔣️component.json")).expect("language-neutral catalog")).expect("valid catalog");
        let mutation_catalog = &catalog["mutationCatalogs"][0];
        assert_eq!(<SHomeMutation as protocol::SemanticMutation<SHomeSnapshot>>::kinds()[0].kind, "change-catalog-generation");
        assert_eq!(mutation_catalog["kinds"], serde_json::json!(["change-catalog-generation"]));
        assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
        assert!(!source.contains(concat!("::", "mutation::")));
        assert_eq!(descriptor["semanticKind"], "change-catalog-generation");
        assert_eq!(descriptor["aggregateVariant"], "ChangeCatalogGeneration");
        assert_eq!(descriptor["payloadSchema"], "🔣️payload.schema.json");
        assert_eq!(descriptor["textOpcode"], "change-catalog-generation");
        assert_eq!(descriptor["binaryTag"], 0);
        assert_eq!(descriptor["outcomeClasses"], serde_json::json!(["applied", "warning"]));
        assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
        for surface in ["🔣️payload.schema.json", "🟦️component.ts", "🔗️component.graphql", "🛰️component.proto", "📝️text/🦀️component.rs", "💾️binary/🦀️component.rs"] {
            assert!(owner.join(surface).is_file(), "missing direct surface {surface}");
        }
        assert!(mutation_catalog["vectors"].as_array().expect("catalog vectors").iter().any(|vector| vector["mutationId"] == "change-catalog-generation" && vector["mutationDirectoryName"] == "🔢️change-catalog-generation"));
    }
}
//#endregion 🧪️StructuralCorrespondence
