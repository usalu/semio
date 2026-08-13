//! 💡️ SemioModelInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/` — a REAL POSITION
//! envelope over every `SpatialNode`/`SemioModelElement` PLACEMENT this subset owns outright.
//! `GeometryRef` only resolves BY ID into the sibling `brep`/`mesh` subsets' own snapshots — never
//! inlined here (this file's own module doc comment) — so a true geometry bounding box is not
//! honestly derivable from `model` alone; the placement translations ARE owned data).

use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::bounds::{compute_semio_model_bounds, SemioModelBounds};

//#region 🔖️Inference
/// 💡️ Everything inferable from a semio model snapshot. One field per named inference under
/// `💡️inferences/` (currently: `bounds`, backed by the `📦bounds/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.model.inference")]
pub struct SemioModelInference {
    #[derived]
    pub bounds: SemioModelBounds,
}

impl protocol::Inference<SemioModelSnapshot> for SemioModelInference {
    fn infer(snapshot: &SemioModelSnapshot) -> Self {
        Self { bounds: compute_semio_model_bounds(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — `SemioModelSnapshot::default()` happens to be
/// all-empty today (no spatial nodes, no elements), so a naive derive would happen to agree, but
/// tying `Default` to `infer` keeps the law correct even if that default ever stops being
/// all-empty (the same defensive pattern raster's `RasterInference` documents).
impl Default for SemioModelInference {
    fn default() -> Self {
        <Self as protocol::Inference<SemioModelSnapshot>>::infer(&SemioModelSnapshot::default())
    }
}

impl protocol::InferenceSpec<SemioModelSnapshot> for SemioModelInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.semio.model.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.semio.model.inference.bounds", reads: &["spatial", "elements"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here (a position-envelope fold is a single whole-snapshot pass over
/// already-flat `spatial`/`elements` collections, no per-entity incremental decomposition applies)
/// — the default `infer_cached` passthrough (`ArtifactInferrer::infer_cached`) is exact.
impl ArtifactInferrer for crate::artifacts::semio::standards::v1::subsets::model::schema::SemioModelBuilder {
    type Snapshot = SemioModelSnapshot;
    type Inference = SemioModelInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.semio.model.inference`'s facet leaves into the OS-wide inference catalog
/// — call once at plugin init, alongside `semio_model_artifact_schema_descriptor`'s registration.
pub fn semio_model_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.semio.model.inference",
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
        let snapshot = SemioModelSnapshot::default();
        assert_eq!(SemioModelInference::infer(&snapshot), SemioModelInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(SemioModelInference::infer(&SemioModelSnapshot::default()), SemioModelInference::default());
    }
}
//#endregion 🧪️Tests
