//! 🏠️ Space Home semantic mutation aggregate.

use crate::artifacts::home::{SHomeDiff, SHomeSnapshot};

pub use super::change_catalog_generation::{change_catalog_generation, ChangeCatalogGeneration};
pub use crate::artifacts::home::schema::operations::*;

//#region 🔖️Aggregate
/// 🧮️ Home launcher mutation vocabulary backed by its direct semantic owner.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslEnum, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
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
        let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
        let descriptor: pack::JsonValue = pack::parse_json(&std::fs::read_to_string(owner.join("🔣️component.json")).expect("direct descriptor")).expect("valid descriptor");
        let catalog: pack::JsonValue = pack::parse_json(&std::fs::read_to_string(mutation_root.join("../../🧪️oracle/🔣️.json")).expect("language-neutral catalog")).expect("valid catalog");
        let mutation_catalog = &catalog["mutationCatalogs"][0];
        assert_eq!(<SHomeMutation as protocol::SemanticMutation<SHomeSnapshot>>::kinds()[0].kind, "change-catalog-generation");
        assert_eq!(mutation_catalog["kinds"], pack::json!(["change-catalog-generation"]));
        assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
        assert!(!source.contains(concat!("::", "mutation::")));
        assert_eq!(descriptor["semanticKind"], "change-catalog-generation");
        assert_eq!(descriptor["aggregateVariant"], "ChangeCatalogGeneration");
        assert_eq!(descriptor["payloadSchema"], "🔣️payload.schema.json");
        assert_eq!(descriptor["textOpcode"], "change-catalog-generation");
        assert_eq!(descriptor["binaryTag"], 0);
        assert_eq!(descriptor["outcomeClasses"], pack::json!(["applied", "warning"]));
        assert_eq!(descriptor["requiredLanguageSurfaces"], pack::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
        {
            assert!(owner.join("🔣️payload.schema.json").is_file());
        }
        {
            assert!(owner.join("🟦️component.ts").is_file());
        }
        {
            assert!(owner.join("🔗️component.graphql").is_file());
        }
        {
            assert!(owner.join("🛰️component.proto").is_file());
        }
        {
            assert!(owner.join("📝️text/🦀️component.rs").is_file());
        }
        {
            assert!(owner.join("💾️binary/🦀️component.rs").is_file());
        }
        assert!(mutation_catalog["vectors"].as_array().expect("catalog vectors").iter().any(|vector| vector["mutationId"] == "change-catalog-generation" && vector["mutationDirectoryName"] == "🔢️change-catalog-generation"));
    }
}
//#endregion 🧪️StructuralCorrespondence
