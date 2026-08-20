//! 💡️ SemioObjectInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧩composition/`).
//!
//! ⚠️ RENAME TRAP: this `object` is the brand-new SPATIAL subset (transform + owned brep/mesh/
//! value children) — unrelated to the old value-tree `object`, renamed to `✳️value` earlier in
//! this ticket. `brep`/`mesh`/`properties` are owned CHILD slots (handles only, never embedded
//! content — this subset's own module doc comment forbids it), so the only honest inference is a
//! composition census (which children are present) plus the object's own real `transform.
//! translation`, never a fabricated geometry bounding box.

use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::composition::{compute_semio_object_composition, SemioObjectComposition};

//#region 🔖️Inference
/// 💡️ Everything inferable from a semio object snapshot. One field per named inference under
/// `💡️inferences/` (currently: `composition`, backed by the `🧩composition/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.object.inference")]
pub struct SemioObjectInference {
    #[derived]
    pub composition: SemioObjectComposition,
}

impl protocol::Inference<SemioObjectSnapshot> for SemioObjectInference {
    async fn infer(snapshot: &SemioObjectSnapshot) -> Self {
        Self { composition: compute_semio_object_composition(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — `SemioObjectSnapshot::default()` (identity
/// transform, no children) happens to agree with a naive derive today, but tying `Default` to
/// `infer` keeps the law correct even if that default ever stops agreeing (the same defensive
/// pattern raster's `RasterInference` documents).
impl Default for SemioObjectInference {
    fn default() -> Self {
        <Self as protocol::Inference<SemioObjectSnapshot>>::infer(&SemioObjectSnapshot::default())
    }
}

impl protocol::InferenceSpec<SemioObjectSnapshot> for SemioObjectInference {
    async fn inference_schema_id() -> &'static str {
        "s.stdio.semio.object.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.semio.object.inference.composition", reads: &["transform", "brep", "mesh", "properties"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here (a 3-flag child-presence census + one transform-field read is O(1))
/// — the default `infer_cached` passthrough (`ArtifactInferrer::infer_cached`) is exact.
impl ArtifactInferrer for crate::artifacts::semio::standards::v1::subsets::object::schema::SemioObjectBuilder {
    type Snapshot = SemioObjectSnapshot;
    type Inference = SemioObjectInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.semio.object.inference`'s facet leaves into the OS-wide inference catalog
/// — call once at plugin init, alongside `semio_object_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_object_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.semio.object.inference",
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
        let snapshot = SemioObjectSnapshot::default();
        assert_eq!(SemioObjectInference::infer(&snapshot), SemioObjectInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(SemioObjectInference::infer(&SemioObjectSnapshot::default()), SemioObjectInference::default());
    }
}
//#endregion 🧪️Tests
