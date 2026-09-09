//! 💡️ Json inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::JsonSnapshot;
use framework_schema::ArtifactSchema;
use protocol::Inference;
use semio_framework_plugin::ArtifactInferrer;

//#region 🔖️Inference
/// 💡️ Everything inferable from a json snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.json.inference")]
pub struct JsonInference {
    #[derived]
    pub outline: JsonOutline,
}

impl Inference<JsonSnapshot> for JsonInference {
    fn infer(snapshot: &JsonSnapshot) -> Self {
        Self { outline: JsonOutline::compute(snapshot) }
    }
}

/// 🪞️ Hand impl (not derived): `JsonSnapshot::default()`'s root is `Null`, a real value that
/// `JsonOutline::compute` reports on, so the derived all-zero `JsonOutline::default()` disagrees
/// with it and breaks `inference_default_law`. Defining default as "infer the default snapshot"
/// makes the two definitionally equal.
impl Default for JsonInference {
    fn default() -> Self {
        Self::infer(&JsonSnapshot::default())
    }
}

impl protocol::InferenceSpec<JsonSnapshot> for JsonInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.json.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.json.inference.outline", reads: &["value"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::standards::v_rfc8259::subsets::base::schema::JsonBuilder {
    type Snapshot = JsonSnapshot;
    type Inference = JsonInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.json.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `json_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn json_artifact_inference_descriptor() -> framework_schema::ArtifactInferenceDescriptor {
    framework_schema::ArtifactInferenceDescriptor {
        id: "s.stdio.json.inference",
        inference: framework_schema::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
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
pub use super::outline::JsonOutline;
//#endregion 🔁️Re-exports
