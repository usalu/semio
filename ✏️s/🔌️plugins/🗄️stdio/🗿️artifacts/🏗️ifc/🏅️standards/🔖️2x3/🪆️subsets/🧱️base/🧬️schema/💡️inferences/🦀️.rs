//! 💡️ Ifc2x3Inference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`, honestly derivable by
//! folding every real `IFCCARTESIANPOINT((x,y,z));` instance in `document.instances` — the same
//! buildingSMART Coordination View 2.0-era Part-21 entity keyword IFC4 uses, since 2x3 rides the
//! identical ISO 10303-21 syntax with an older EXPRESS schema).

use crate::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
use framework_schema::ArtifactSchema;

use super::bounds::compute_ifc2x3_bounds;
//#region 🔖️Inference
/// 💡️ Everything inferable from an IFC2X3 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `bounds`, backed by the `📦bounds/` slug dir).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ifc.2x3.inference")]
pub struct Ifc2x3Inference {
    #[derived]
    pub bounds: Ifc2x3Bounds,
}

impl protocol::Inference<Ifc2x3Snapshot> for Ifc2x3Inference {
    fn infer(snapshot: &Ifc2x3Snapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { bounds: compute_ifc2x3_bounds(snapshot) }
    
        })
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `Ifc2x3Snapshot::default()`'s `document` ever stops being empty.
impl Default for Ifc2x3Inference {
    fn default() -> Self {
        let snapshot = &Ifc2x3Snapshot::default();

        Self { bounds: compute_ifc2x3_bounds(snapshot) }
    }
}

impl protocol::InferenceSpec<Ifc2x3Snapshot> for Ifc2x3Inference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.ifc.2x3.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.ifc.2x3.inference.bounds", reads: &["document"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.ifc.2x3.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `ifc2x3_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn ifc2x3_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.stdio.ifc.2x3.inference",
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
pub use super::bounds::Ifc2x3Bounds;
//#endregion 🔁️Re-exports
