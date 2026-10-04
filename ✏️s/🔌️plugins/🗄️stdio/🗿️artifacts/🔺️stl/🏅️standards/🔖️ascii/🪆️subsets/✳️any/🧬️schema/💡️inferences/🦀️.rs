//! 💡️ StlInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`, honestly derivable from
//! `triangles` alone — real STL has no shared vertex index space, so this is a direct fold over
//! every triangle's own 3 vertices).

use crate::StlSnapshot;
use framework_schema::ArtifactSchema;

use super::bounds::compute_stl_bounds;
//#region 🔖️Inference
/// 💡️ Everything inferable from a stl snapshot. One field per named inference under
/// `💡️inferences/` (currently: `bounds`, backed by the `📦bounds/` slug dir).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.stl.inference")]
pub struct StlInference {
    #[derived]
    pub bounds: StlBounds,
}

impl protocol::Inference<StlSnapshot> for StlInference {
    fn infer(snapshot: &StlSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { bounds: compute_stl_bounds(snapshot) }
    
        })
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `StlSnapshot::default()`'s `triangles` ever stops being empty.
impl Default for StlInference {
    fn default() -> Self {
        let snapshot = &StlSnapshot::default();

        Self { bounds: compute_stl_bounds(snapshot) }
    }
}

impl protocol::InferenceSpec<StlSnapshot> for StlInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.stl.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.stl.inference.bounds", reads: &["triangles"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.stl.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `stl_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn stl_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.stdio.stl.inference",
        inference: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
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
pub use super::bounds::StlBounds;
//#endregion 🔁️Re-exports
