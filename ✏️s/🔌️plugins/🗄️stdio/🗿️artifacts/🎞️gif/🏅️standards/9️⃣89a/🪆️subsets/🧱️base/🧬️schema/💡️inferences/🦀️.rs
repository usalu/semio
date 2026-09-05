//! 💡️ Gif (89a) inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📐dimensions/`). Sibling to (but a
//! SEPARATE inference schema id from) `7️⃣87a`'s own `GifInference` — same shape as those two
//! standards' own `GifSnapshot`/`GifBuilder` reuse of the same Rust type name in different modules.

use crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::GifSnapshot;
use schema::ArtifactSchema;

use super::dimensions::{compute_gif_dimensions, GifDimensions};

//#region 🔖️Inference
/// 💡️ Everything inferable from a gif89a snapshot. One field per named inference under
/// `💡️inferences/` (currently: `dimensions`, backed by the `📐dimensions/` slug dir).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gif.89a.inference")]
pub struct GifInference {
    #[derived]
    pub dimensions: GifDimensions,
}

impl protocol::Inference<GifSnapshot> for GifInference {
    fn infer(snapshot: &GifSnapshot) -> Self {
        Self { dimensions: compute_gif_dimensions(snapshot) }
    }
}

/// 🌱 Hand-fixed to agree with `infer(&GifSnapshot::default())` rather than a naive
/// `#[derive(Default)]` — the no-GCT `bitDepth: 8` fallback disagrees with a structurally-derived
/// all-zero `GifDimensions`, the same "match `infer` of the real default, don't derive
/// structurally" trick as `AddInference`'s hand-written `Default` in
/// `📡️spr/🎮️command/🦀️.rs`.
impl Default for GifInference {
    fn default() -> Self {
        <Self as protocol::Inference<GifSnapshot>>::infer(&GifSnapshot::default())
    }
}

impl protocol::InferenceSpec<GifSnapshot> for GifInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.gif.89a.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.gif.89a.inference.dimensions", reads: &["width", "height", "gct", "frames"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here (a screen-descriptor/GCT/GCE read is already O(frames)) — the
/// default `infer_cached` passthrough (`ArtifactInferrer::infer_cached`) is exact.
impl semio_framework_plugin::ArtifactInferrer for crate::artifacts::gif::standards::v89a::subsets::any::schema::GifBuilder {
    type Snapshot = GifSnapshot;
    type Inference = GifInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.gif.89a.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside 89a's own artifact schema descriptor registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn gif89a_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.gif.89a.inference",
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
        let snapshot = GifSnapshot::default();
        assert_eq!(GifInference::infer(&snapshot), GifInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(GifInference::infer(&GifSnapshot::default()), GifInference::default());
    }
}
//#endregion 🧪️Tests
