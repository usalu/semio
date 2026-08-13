//! 💡️ AviInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `⏱duration/`, derived from the real
//! `avih` MainAVIHeader's `dwTotalFrames`/`dwMicroSecPerFrame` fields).

use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::AviSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::duration::{compute_avi_duration, AviDuration};

//#region 🔖️Inference
/// 💡️ Everything inferable from an avi snapshot. One field per named inference under
/// `💡️inferences/` (currently: `duration`, backed by the `⏱duration/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.avi.inference")]
pub struct AviInference {
    #[derived]
    pub duration: AviDuration,
}

impl protocol::Inference<AviSnapshot> for AviInference {
    fn infer(snapshot: &AviSnapshot) -> Self {
        Self { duration: compute_avi_duration(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — `AviSnapshot` itself derives `Default` (all
/// header fields zero, `streams` empty), so this is belt-and-suspenders rather than a strict
/// necessity here, but keeps the same hand-rolled convention every sibling family uses (safer
/// against a future `AviMainHeader` default that stops being all-zero).
impl Default for AviInference {
    fn default() -> Self {
        <Self as protocol::Inference<AviSnapshot>>::infer(&AviSnapshot::default())
    }
}

impl protocol::InferenceSpec<AviSnapshot> for AviInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.avi.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.avi.inference.duration", reads: &["mainHeader", "streams"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here — `duration` is a fixed-field read off `main_header` plus a single
/// `streams.len()` count, already O(1)/O(streams) with no honest per-entity incremental
/// decomposition worth a merkle dep-chain — the default `infer_cached` passthrough is exact.
impl ArtifactInferrer for crate::artifacts::avi::standards::v1_0::subsets::any::schema::AviBuilder {
    type Snapshot = AviSnapshot;
    type Inference = AviInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.avi.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `avi_artifact_schema_descriptor`'s registration.
pub fn avi_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.avi.inference",
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
        let snapshot = AviSnapshot::default();
        assert_eq!(AviInference::infer(&snapshot), AviInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(AviInference::infer(&AviSnapshot::default()), AviInference::default());
    }
}
//#endregion 🧪️Tests
