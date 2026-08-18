//! 💡️ Draw inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use crate::artifacts::draw::DrawSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::topology::{compute_draw_topology, DrawTopology};

//#region 🔖️Inference
/// 💡️ Everything inferable from a draw snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir) — `layers` is a
/// real tree (`Group.children: Vec<DrawLayerNode>`), so `topology` here is a real pre-order
/// traversal of that structural nesting: `topoOrder`/`depth`/`nodeCount` plus `cycleFree`, which is
/// always `true` — a Rust `Vec<Self>` embedded by value cannot express a structural cycle.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.draw.draw.inference")]
pub struct DrawInference {
    #[derived]
    pub topology: DrawTopology,
}

impl protocol::Inference<DrawSnapshot> for DrawInference {
    fn infer(snapshot: &DrawSnapshot) -> Self {
        Self { topology: compute_draw_topology(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `DrawSnapshot::default()`'s `layers` field ever stops being empty.
impl Default for DrawInference {
    fn default() -> Self {
        <Self as protocol::Inference<DrawSnapshot>>::infer(&DrawSnapshot::default())
    }
}

impl protocol::InferenceSpec<DrawSnapshot> for DrawInference {
    fn inference_schema_id() -> &'static str {
        "s.draw.draw.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.draw.draw.inference.topology", reads: &["layers"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::draw::standards::v1::subsets::any::schema::DrawInferrer {
    type Snapshot = DrawSnapshot;
    type Inference = DrawInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.draw.draw.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `draw_artifact_schema_descriptor`'s registration.
pub fn draw_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.draw.draw.inference",
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

    #[test]
    fn inference_determinism_law() {
        let snapshot = DrawSnapshot::default();
        assert_eq!(DrawInference::infer(&snapshot), DrawInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(DrawInference::infer(&DrawSnapshot::default()), DrawInference::default());
    }
}
//#endregion 🧪️Tests
