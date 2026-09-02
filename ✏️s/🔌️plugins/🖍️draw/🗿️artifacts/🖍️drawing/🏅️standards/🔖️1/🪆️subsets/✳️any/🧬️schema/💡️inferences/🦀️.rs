//! 💡️ Drawing inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use crate::artifacts::drawing::DrawingSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use super::topology::{compute_drawing_topology, DrawingTopology};

//#region 🔖️Inference
/// 💡️ Everything inferable from a drawing snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir) — `layers` is a
/// real tree (`Group.children: Vec<DrawingLayerNode>`), so `topology` here is a real pre-order
/// traversal of that structural nesting: `topoOrder`/`depth`/`nodeCount` plus `cycleFree`, which is
/// always `true` — a Rust `Vec<Self>` embedded by value cannot express a structural cycle.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.draw.drawing.inference")]
pub struct DrawingInference {
    #[derived]
    pub topology: DrawingTopology,
}

impl protocol::Inference<DrawingSnapshot> for DrawingInference {
    fn infer(snapshot: &DrawingSnapshot) -> Self {
        Self { topology: compute_drawing_topology(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `DrawingSnapshot::default()`'s `layers` field ever stops being empty.
impl Default for DrawingInference {
    fn default() -> Self {
        <Self as protocol::Inference<DrawingSnapshot>>::infer(&DrawingSnapshot::default())
    }
}

impl protocol::InferenceSpec<DrawingSnapshot> for DrawingInference {
    fn inference_schema_id() -> &'static str {
        "s.draw.drawing.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.draw.drawing.inference.topology", reads: &["layers"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::drawing::standards::v1::subsets::any::schema::DrawingInferrer {
    type Snapshot = DrawingSnapshot;
    type Inference = DrawingInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.draw.drawing.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `drawing_artifact_schema_descriptor`'s registration.
pub fn drawing_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.draw.drawing.inference",
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

    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = DrawingSnapshot::default();
        assert_eq!(DrawingInference::infer(&snapshot), DrawingInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(DrawingInference::infer(&DrawingSnapshot::default()), DrawingInference::default());
    }
}
//#endregion 🧪️Tests
