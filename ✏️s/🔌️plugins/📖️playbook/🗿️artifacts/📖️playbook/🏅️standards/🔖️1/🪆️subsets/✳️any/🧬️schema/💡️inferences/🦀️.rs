//! 💡️ Playbook inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use crate::artifacts::playbook::PlaybookSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use semio_framework_value_derive::{FromValue, ToValue};

use super::topology::{compute_playbook_topology, PlaybookTopology};

//#region 🔖️Inference
/// 💡️ Everything inferable from a playbook snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
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
        Self { topology: compute_playbook_topology(&snapshot.steps()) }
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
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
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
    /// 🧸️ Built via `PlaybookStep`/`PlaybookBlock` construction (not raw JSON with a `"steps"` key —
    /// `PlaybookSnapshot` no longer has that field; it composes `document`/`flow` children instead)
    /// and minted through `playbook_snapshot_with_steps` so the working-scene cache is seeded.
    fn step_with_conditional_block() -> PlaybookSnapshot {
        use crate::artifacts::playbook::PlaybookBlock;

        fn block(id: &str, kind: &str, condition: Option<crate::artifacts::playbook::PlaybookExpr>) -> PlaybookBlock {
            PlaybookBlock {
                id: id.into(),
                label: id.into(),
                kind: kind.into(),
                description: None,
                required: None,
                placeholder: None,
                default: None,
                min: None,
                max: None,
                step: None,
                unit: None,
                text: None,
                options: None,
                fields: None,
                schema: None,
                src: None,
                accept: None,
                fixture_slug: None,
                params: None,
                condition,
            }
        }

        let steps = vec![crate::artifacts::playbook::PlaybookStep {
            id: "s1".into(),
            title: "Step 1".into(),
            description: None,
            blocks: vec![block("material", "single", None), block("finish", "text", Some(crate::artifacts::playbook::PlaybookExpr::Truthy { expr: Box::new(crate::artifacts::playbook::PlaybookExpr::Var { name: "material".into() }) }))],
        }];
        crate::artifacts::playbook::playbook_snapshot_with_steps("playbook.playbook", "playbook", "1", None, steps)
    }
    //#endregion 🧸️Fixtures

    //#region 🧪️InferenceLaws
    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = step_with_conditional_block();
        assert_eq!(PlaybookInference::infer(&snapshot), PlaybookInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(PlaybookInference::infer(&PlaybookSnapshot::default()), PlaybookInference::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn topology_orders_the_conditioned_block_after_its_dependency() {
        let snapshot = step_with_conditional_block();
        let inferred = PlaybookInference::infer(&snapshot);
        let material_index = inferred.topology.topo_order.iter().position(|id| id == "material").unwrap();
        let finish_index = inferred.topology.topo_order.iter().position(|id| id == "finish").unwrap();
        assert!(material_index < finish_index, "finish's condition reads material, so material must precede it");
        assert!(inferred.topology.cycle_free);
    }

    #[semio_framework_async_macros::async_test]
    async fn default_snapshot_has_one_empty_step() {
        let inferred = PlaybookInference::infer(&PlaybookSnapshot::default());
        assert_eq!(inferred.topology.node_count, 1);
        assert!(inferred.topology.cycle_free);
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
