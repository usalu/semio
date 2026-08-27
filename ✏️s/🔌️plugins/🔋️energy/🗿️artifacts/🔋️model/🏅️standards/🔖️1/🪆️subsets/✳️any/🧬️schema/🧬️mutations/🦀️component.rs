//! 🧬️ Transparent energy-model semantic mutation aggregate.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::EnergyModelSnapshot;
use serde::{Deserialize, Serialize};

pub use super::replace_model::{energy_model_mutation_report_json, ReplaceModel, KINDS};

//#region 🔖️Aggregate
/// 🧬️ Closed semantic mutation vocabulary for an energy model.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = EnergyModelSnapshot, diff = EnergyModelDiff, schema = "energy.model")]
pub enum EnergyModelMutation {
    ReplaceModel(ReplaceModel),
}
//#endregion 🔖️Aggregate

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
mod structural_correspondence_tests {
    use super::*;
    use protocol::SemanticMutation;

    #[test]
    fn direct_owner_descriptor_surfaces_and_catalog_correspond() {
        let mutation_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
        let owner = mutation_root.join("♻️replace-model");
        let source = std::fs::read_to_string(owner.join("🦀️component.rs")).expect("direct Rust owner");
        let descriptor_source = std::fs::read_to_string(owner.join("🔣️component.json")).expect("direct language-neutral descriptor");
        let descriptor: serde_json::Value = serde_json::from_str(&descriptor_source).expect("direct descriptor must be valid JSON");
        let payload_schema_source = std::fs::read_to_string(owner.join("🔣️payload.schema.json")).expect("direct payload schema");
        let payload_schema: serde_json::Value = serde_json::from_str(&payload_schema_source).expect("direct payload schema must be valid JSON");
        let catalog_source = std::fs::read_to_string(mutation_root.join("../../🧪️oracle/🔣️.json")).expect("language-neutral oracle catalog");
        let catalog: serde_json::Value = serde_json::from_str(&catalog_source).expect("language-neutral oracle catalog must be valid JSON");
        let descriptors = EnergyModelMutation::kinds();

        assert_eq!(descriptors.len(), 1);
        assert_eq!(descriptors[0].kind, "replace-model");
        assert!(source.contains("protocol::MutationKind"));
        assert!(!source.contains(concat!("::", "mutation::")));
        assert_eq!(descriptor["semanticKind"], "replace-model");
        assert_eq!(descriptor["aggregateVariant"], "ReplaceModel");
        assert_eq!(owner.join(descriptor["payloadSchema"].as_str().expect("payload schema pointer")), owner.join("🔣️payload.schema.json"));
        assert_eq!(payload_schema["title"], "ReplaceModel");
        for surface in ["🟦️component.ts", "🔗️component.graphql", "🛰️component.proto", "📝️text/🦀️component.rs", "💾️binary/🦀️component.rs"] {
            let surface_source = std::fs::read_to_string(owner.join(surface)).expect("direct mutation surface");
            assert!(surface_source.contains("replace-model") || surface_source.contains("ReplaceModel"));
        }
        assert!(catalog["mutationCatalogs"][0]["kinds"].as_array().expect("catalog kinds").iter().any(|kind| kind == "replace-model"));
    }
}
//#endregion 🧪️StructuralCorrespondence
