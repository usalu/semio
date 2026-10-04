//! 💡️ DxfInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`, honestly derivable from
//! `entities` and `blocks[].entities` alone — DXF R12 is a real vector CAD format with decoded
//! point-bearing entities, unlike DWG's undecoded byte payload).

use crate::DxfSnapshot;
use framework_schema::ArtifactSchema;

use super::bounds::compute_dxf_bounds;
//#region 🔖️Inference
/// 💡️ Everything inferable from a dxf snapshot. One field per named inference under
/// `💡️inferences/` (currently: `bounds`, backed by the `📦bounds/` slug dir).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.dxf.inference")]
pub struct DxfInference {
    #[derived]
    pub bounds: DxfBounds,
}

impl protocol::Inference<DxfSnapshot> for DxfInference {
    fn infer(snapshot: &DxfSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { bounds: compute_dxf_bounds(snapshot) }
    
        })
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `DxfSnapshot::default()`'s `entities`/`blocks` ever stop being empty.
impl Default for DxfInference {
    fn default() -> Self {
        let snapshot = &DxfSnapshot::default();

        Self { bounds: compute_dxf_bounds(snapshot) }
    }
}

impl protocol::InferenceSpec<DxfSnapshot> for DxfInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.dxf.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.dxf.inference.bounds", reads: &["entities", "blocks"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.dxf.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `dxf_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dxf_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.stdio.dxf.inference",
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
pub use super::bounds::DxfBounds;
//#endregion 🔁️Re-exports

//#region 🪢️TaxonomyMounts
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion 🪢️TaxonomyMounts
