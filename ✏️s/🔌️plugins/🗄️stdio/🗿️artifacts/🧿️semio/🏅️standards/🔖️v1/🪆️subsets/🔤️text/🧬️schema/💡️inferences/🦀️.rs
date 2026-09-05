//! 💡️ SemioTextInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📊profile/` — this subset owns runs
//! standalone, not nested inside block structure (this subset's own module doc comment), so it has
//! no heading hierarchy the way `document`'s does; the honest structural summary is a word/mark
//! census plus the distinct BCP-47 languages actually used).

use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use super::profile::{compute_semio_text_profile, SemioTextProfile};

//#region 🔖️Inference
/// 💡️ Everything inferable from a semio text snapshot. One field per named inference under
/// `💡️inferences/` (currently: `profile`, backed by the `📊profile/` slug dir).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.text.inference")]
pub struct SemioTextInference {
    #[derived]
    pub profile: SemioTextProfile,
}

impl protocol::Inference<SemioTextSnapshot> for SemioTextInference {
    fn infer(snapshot: &SemioTextSnapshot) -> Self {
        Self { profile: compute_semio_text_profile(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — `SemioTextSnapshot::default()` happens to be
/// all-empty today (no runs), so a naive derive would happen to agree, but tying `Default` to
/// `infer` keeps the law correct even if that default ever stops being all-empty (the same
/// defensive pattern raster's `RasterInference` documents).
impl Default for SemioTextInference {
    fn default() -> Self {
        <Self as protocol::Inference<SemioTextSnapshot>>::infer(&SemioTextSnapshot::default())
    }
}

impl protocol::InferenceSpec<SemioTextSnapshot> for SemioTextInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.semio.text.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.semio.text.inference.profile", reads: &["runs"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here (a word/mark/language census is a single whole-snapshot fold over
/// already-flat `runs`) — the default `infer_cached` passthrough (`ArtifactInferrer::infer_cached`)
/// is exact.
impl ArtifactInferrer for crate::artifacts::semio::standards::v1::subsets::text::schema::SemioTextBuilder {
    type Snapshot = SemioTextSnapshot;
    type Inference = SemioTextInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.semio.text.inference`'s facet leaves into the OS-wide inference catalog
/// — call once at plugin init, alongside `semio_text_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_text_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.semio.text.inference",
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
        let snapshot = SemioTextSnapshot::default();
        assert_eq!(SemioTextInference::infer(&snapshot), SemioTextInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(SemioTextInference::infer(&SemioTextSnapshot::default()), SemioTextInference::default());
    }
}
//#endregion 🧪️Tests
