//! 💡️ DeflateInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🪟window/`). Deliberately NOT a
//! `🗃entries` census like `🎒️zip`'s: RFC1950 wraps exactly one deflate-compressed member, not a
//! multi-entry container — forcing an "entries" shape onto a single-stream zlib payload would be
//! dishonest, so this facet instead derives real RFC1950 zlib HEADER semantics (CMF window size,
//! FLG.FLEVEL, FDICT) that zip has no equivalent of at all.

use crate::standards::v_rfc1950::subsets::any::schema::snapshot::DeflateSnapshot;
use framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use super::window::{compute_deflate_window};
//#region 🔖️Inference
/// 💡️ Everything inferable from a deflate snapshot. One field per named inference under
/// `💡️inferences/` (currently: `window`, backed by the `🪟window/` slug dir).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.deflate.inference")]
pub struct DeflateInference {
    #[derived]
    pub window: DeflateWindow,
}

impl protocol::Inference<DeflateSnapshot> for DeflateInference {
    fn infer(snapshot: &DeflateSnapshot) -> Self {
        Self { window: compute_deflate_window(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — `DeflateSnapshot::default()` is a real RFC1950
/// normal form (`compression_method: 8`, `window_bits: 7`), not a zeroed struct, so a derived
/// all-zero `Default` would disagree with the honest compute and break the law.
impl Default for DeflateInference {
    fn default() -> Self {
        <Self as protocol::Inference<DeflateSnapshot>>::infer(&DeflateSnapshot::default())
    }
}

impl protocol::InferenceSpec<DeflateSnapshot> for DeflateInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.deflate.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.deflate.inference.window", reads: &["windowBits", "compressionLevelHint", "dictId", "payload"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here — `window` is a fixed-field header read plus a single fold over
/// `payload`, already O(n) in payload size with no honest per-entity incremental decomposition (a
/// merkle dep-chain over one flat `Vec<u8>` payload costs more than the fold it would cache) —
/// the default `infer_cached` passthrough is exact.
impl ArtifactInferrer for crate::standards::v_rfc1950::subsets::any::schema::DeflateBuilder {
    type Snapshot = DeflateSnapshot;
    type Inference = DeflateInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.deflate.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `deflate_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deflate_artifact_inference_descriptor() -> framework_schema::ArtifactInferenceDescriptor {
    framework_schema::ArtifactInferenceDescriptor {
        id: "s.stdio.deflate.inference",
        inference: framework_schema::FacetLeaves {
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use super::window::DeflateWindow;
//#endregion 🔁️Re-exports
