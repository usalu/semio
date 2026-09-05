//! 💡️ SemioKitInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🗃️entries/`, a real census over the
//! catalog's own collections — the only thing honestly inferable from a kit alone, since its child
//! slots (`objects`/`models`/`properties`) and link slot (`representations`) are handles, never
//! embedded content).

use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use super::entries::{compute_semio_kit_entries, SemioKitEntries};

//#region 🔖️Inference
/// 💡️ Everything inferable from a semio kit snapshot. One field per named inference under
/// `💡️inferences/` (currently: `entries`, backed by the `🗃️entries/` slug dir).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.kit.inference")]
pub struct SemioKitInference {
    #[derived]
    pub entries: SemioKitEntries,
}

impl protocol::Inference<SemioKitSnapshot> for SemioKitInference {
    fn infer(snapshot: &SemioKitSnapshot) -> Self {
        Self { entries: compute_semio_kit_entries(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — `SemioKitSnapshot::default()` happens to be
/// all-empty today (no types/designs/children/representations), so a naive derive would happen to
/// agree, but tying `Default` to `infer` keeps the law correct even if that default ever stops
/// being all-empty (the same defensive pattern raster's `RasterInference` documents).
impl Default for SemioKitInference {
    fn default() -> Self {
        <Self as protocol::Inference<SemioKitSnapshot>>::infer(&SemioKitSnapshot::default())
    }
}

impl protocol::InferenceSpec<SemioKitSnapshot> for SemioKitInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.semio.kit.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.semio.kit.inference.entries", reads: &["types", "designs", "objects", "models", "properties", "representations"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here (a catalog census is a single whole-snapshot fold over already-flat
/// collections, no per-entity incremental decomposition applies) — the default `infer_cached`
/// passthrough (`ArtifactInferrer::infer_cached`) is exact.
impl ArtifactInferrer for crate::artifacts::semio::standards::v1::subsets::kit::schema::SemioKitBuilder {
    type Snapshot = SemioKitSnapshot;
    type Inference = SemioKitInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.semio.kit.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `semio_kit_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_kit_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.semio.kit.inference",
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
        let snapshot = SemioKitSnapshot::default();
        assert_eq!(SemioKitInference::infer(&snapshot), SemioKitInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(SemioKitInference::infer(&SemioKitSnapshot::default()), SemioKitInference::default());
    }
}
//#endregion 🧪️Tests
