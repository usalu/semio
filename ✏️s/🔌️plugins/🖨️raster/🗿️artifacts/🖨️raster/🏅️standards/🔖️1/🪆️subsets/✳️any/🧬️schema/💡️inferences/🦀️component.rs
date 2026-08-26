//! 💡️ Raster inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use crate::artifacts::raster::RasterSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::topology::{compute_raster_topology, RasterTopology};

//#region 🔖️Inference
/// 💡️ Everything inferable from a raster snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir) — `layers` is a
/// real tree (`RasterLayerNode::Group.children: Vec<RasterLayerNode>` is a genuine tree, owned by
/// value), so `topology` here is a real pre-order traversal of that structural nesting:
/// `topoOrder`/`depth`/`nodeCount` plus `cycleFree`, which is always `true` — a Rust `Vec<Self>`
/// embedded by value cannot express a structural cycle.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.raster.raster.inference")]
pub struct RasterInference {
    #[derived]
    pub topology: RasterTopology,
}

impl protocol::Inference<RasterSnapshot> for RasterInference {
    fn infer(snapshot: &RasterSnapshot) -> Self {
        Self { topology: compute_raster_topology(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `RasterSnapshot::default()`'s `layers` field ever stops being empty.
impl Default for RasterInference {
    fn default() -> Self {
        <Self as protocol::Inference<RasterSnapshot>>::infer(&RasterSnapshot::default())
    }
}

impl protocol::InferenceSpec<RasterSnapshot> for RasterInference {
    fn inference_schema_id() -> &'static str {
        "s.raster.raster.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.raster.raster.inference.topology", reads: &["layers"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::raster::standards::v1::subsets::any::schema::RasterBuilder {
    type Snapshot = RasterSnapshot;
    type Inference = RasterInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.raster.raster.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `raster_artifact_schema_descriptor`'s registration.
pub fn raster_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.raster.raster.inference",
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

    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = RasterSnapshot::default();
        assert_eq!(RasterInference::infer(&snapshot), RasterInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(RasterInference::infer(&RasterSnapshot::default()), RasterInference::default());
    }
}
//#endregion 🧪️Tests
