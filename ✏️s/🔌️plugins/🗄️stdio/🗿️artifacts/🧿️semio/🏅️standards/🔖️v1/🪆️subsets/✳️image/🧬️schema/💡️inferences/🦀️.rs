//! 💡️ SemioImageInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📐dimensions/`, honestly derivable
//! from `width`/`height`/`colorspace`/`bitDepth`/`frames` alone).

use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use super::dimensions::{compute_semio_image_dimensions, SemioImageDimensions};

//#region 🔖️Inference
/// 💡️ Everything inferable from a semio image snapshot. One field per named inference under
/// `💡️inferences/` (currently: `dimensions`, backed by the `📐dimensions/` slug dir).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.image.inference")]
pub struct SemioImageInference {
    #[derived]
    pub dimensions: SemioImageDimensions,
}

impl protocol::Inference<SemioImageSnapshot> for SemioImageInference {
    fn infer(snapshot: &SemioImageSnapshot) -> Self {
        Self { dimensions: compute_semio_image_dimensions(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — `SemioImageSnapshot::default()` is all-zero
/// (`width`/`height`/`bitDepth` all `0`, no frames), so a naive derive would happen to agree
/// today, but tying `Default` to `infer` keeps the law correct even if that default ever stops
/// being all-zero (the same defensive pattern raster's `RasterInference` documents).
impl Default for SemioImageInference {
    fn default() -> Self {
        <Self as protocol::Inference<SemioImageSnapshot>>::infer(&SemioImageSnapshot::default())
    }
}

impl protocol::InferenceSpec<SemioImageSnapshot> for SemioImageInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.semio.image.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.semio.image.inference.dimensions", reads: &["width", "height", "colorspace", "bitDepth", "frames"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here (a header-field read + a single frame-count fold is already O(n)
/// in `frames.len()`, no per-entity incremental decomposition applies) — the default
/// `infer_cached` passthrough (`ArtifactInferrer::infer_cached`) is exact.
impl ArtifactInferrer for crate::artifacts::semio::standards::v1::subsets::image::schema::SemioImageBuilder {
    type Snapshot = SemioImageSnapshot;
    type Inference = SemioImageInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.semio.image.inference`'s facet leaves into the OS-wide inference catalog
/// — call once at plugin init, alongside `semio_image_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_image_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.semio.image.inference",
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
        let snapshot = SemioImageSnapshot::default();
        assert_eq!(SemioImageInference::infer(&snapshot), SemioImageInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(SemioImageInference::infer(&SemioImageSnapshot::default()), SemioImageInference::default());
    }
}
//#endregion 🧪️Tests
