//! 💡️ Playbook inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use crate::artifacts::playbook::PlaybookSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::topology::{compute_playbook_topology, PlaybookTopology};

//#region 🔖️Inference
/// 💡️ Everything inferable from a playbook snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.playbook.playbook.inference")]
pub struct PlaybookInference {
    #[derived]
    pub topology: PlaybookTopology,
}

impl Default for PlaybookInference {
    fn default() -> Self {
        <Self as protocol::Inference<PlaybookSnapshot>>::infer(&PlaybookSnapshot::default())
    }
}

impl protocol::Inference<PlaybookSnapshot> for PlaybookInference {
    fn infer(snapshot: &PlaybookSnapshot) -> Self {
        Self { topology: compute_playbook_topology(&snapshot.steps) }
    }
}

impl protocol::InferenceSpec<PlaybookSnapshot> for PlaybookInference {
    fn inference_schema_id() -> &'static str {
        "s.playbook.playbook.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.playbook.playbook.inference.topology", reads: &["steps"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::playbook::standards::v1::subsets::any::schema::PlaybookBuilder {
    type Snapshot = PlaybookSnapshot;
    type Inference = PlaybookInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.playbook.playbook.inference`'s facet leaves into the OS-wide inference catalog
/// — call once at plugin init, alongside `playbook_artifact_schema_descriptor`'s registration.
pub fn playbook_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.playbook.playbook.inference",
        inference: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use protocol::Inference;

    //#region 🧸️Fixtures
    fn step_with_conditional_block() -> PlaybookSnapshot {
        let json = r#"{
            "schema": "playbook.playbook",
            "id": "playbook",
            "version": "1",
            "title": null,
            "steps": [
                {
                    "id": "s1",
                    "title": "Step 1",
                    "blocks": [
                        { "id": "material", "label": "Material", "kind": "single" },
                        {
                            "id": "finish",
                            "label": "Finish",
                            "kind": "text",
                            "condition": { "kind": "truthy", "expr": { "kind": "var", "name": "material" } }
                        }
                    ]
                }
            ]
        }"#;
        serde_json::from_str::<PlaybookSnapshot>(json).expect("valid playbook snapshot json")
    }
    //#endregion 🧸️Fixtures

    //#region 🧪️InferenceLaws
    #[test]
    fn inference_determinism_law() {
        let snapshot = step_with_conditional_block();
        assert_eq!(PlaybookInference::infer(&snapshot), PlaybookInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(PlaybookInference::infer(&PlaybookSnapshot::default()), PlaybookInference::default());
    }

    #[test]
    fn topology_orders_the_conditioned_block_after_its_dependency() {
        let snapshot = step_with_conditional_block();
        let inferred = PlaybookInference::infer(&snapshot);
        let material_index = inferred.topology.topo_order.iter().position(|id| id == "material").unwrap();
        let finish_index = inferred.topology.topo_order.iter().position(|id| id == "finish").unwrap();
        assert!(material_index < finish_index, "finish's condition reads material, so material must precede it");
        assert!(inferred.topology.cycle_free);
    }

    #[test]
    fn default_snapshot_has_one_empty_step() {
        let inferred = PlaybookInference::infer(&PlaybookSnapshot::default());
        assert_eq!(inferred.topology.node_count, 1);
        assert!(inferred.topology.cycle_free);
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
