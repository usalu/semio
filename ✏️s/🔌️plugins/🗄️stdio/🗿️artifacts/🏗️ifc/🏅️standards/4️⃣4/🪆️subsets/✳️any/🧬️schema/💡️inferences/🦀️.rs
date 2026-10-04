//! 💡️ IfcInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`, honestly derivable by
//! folding every real `IFCCARTESIANPOINT((x,y,z));` entity in `entities` — IFC4's own EXPRESS
//! schema keyword for a 3D point, riding the same ISO 10303-21 Part-21 syntax `📐️step` AP214
//! uses under its distinct `CARTESIAN_POINT` vocabulary).

use crate::IfcSnapshot;
use framework_schema::ArtifactSchema;

use super::bounds::compute_ifc_bounds;
//#region 🔖️Inference
/// 💡️ Everything inferable from an IFC4 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `bounds`, backed by the `📦bounds/` slug dir).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ifc.inference")]
pub struct IfcInference {
    #[derived]
    pub bounds: IfcBounds,
}

impl protocol::Inference<IfcSnapshot> for IfcInference {
    fn infer(snapshot: &IfcSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { bounds: compute_ifc_bounds(snapshot) }
    
        })
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `IfcSnapshot::default()`'s `entities` ever stops being empty.
impl Default for IfcInference {
    fn default() -> Self {
        let snapshot = &IfcSnapshot::default();

        Self { bounds: compute_ifc_bounds(snapshot) }
    }
}

impl protocol::InferenceSpec<IfcSnapshot> for IfcInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.ifc.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.ifc.inference.bounds", reads: &["entities"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.ifc.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `ifc_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn ifc_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.stdio.ifc.inference",
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
pub use super::bounds::IfcBounds;
//#endregion 🔁️Re-exports
