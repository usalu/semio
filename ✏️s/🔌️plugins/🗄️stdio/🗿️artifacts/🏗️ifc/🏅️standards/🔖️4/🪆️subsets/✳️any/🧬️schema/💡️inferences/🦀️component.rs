//! 💡️ IfcInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`, honestly derivable by
//! folding every real `IFCCARTESIANPOINT((x,y,z));` entity in `entities` — IFC4's own EXPRESS
//! schema keyword for a 3D point, riding the same ISO 10303-21 Part-21 syntax `📐️step` AP214
//! uses under its distinct `CARTESIAN_POINT` vocabulary).

use crate::artifacts::ifc::IfcSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::bounds::{compute_ifc_bounds, IfcBounds};

//#region 🔖️Inference
/// 💡️ Everything inferable from an IFC4 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `bounds`, backed by the `📦bounds/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ifc.inference")]
pub struct IfcInference {
    #[derived]
    pub bounds: IfcBounds,
}

impl protocol::Inference<IfcSnapshot> for IfcInference {
    fn infer(snapshot: &IfcSnapshot) -> Self {
        Self { bounds: compute_ifc_bounds(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `IfcSnapshot::default()`'s `entities` ever stops being empty.
impl Default for IfcInference {
    fn default() -> Self {
        <Self as protocol::Inference<IfcSnapshot>>::infer(&IfcSnapshot::default())
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

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here — `bounds` is a single min/max fold over every
/// `IFCCARTESIANPOINT` entity in `entities`, already O(n) in total entity count with no honest
/// per-entity incremental decomposition (a merkle dep-chain over this flat entity list costs more
/// than the fold it would cache) — the default `infer_cached` passthrough is exact.
impl ArtifactInferrer for crate::artifacts::ifc::standards::v4::subsets::any::schema::IfcBuilder {
    type Snapshot = IfcSnapshot;
    type Inference = IfcInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.ifc.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `ifc_artifact_schema_descriptor`'s registration.
pub fn ifc_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.ifc.inference",
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
        let snapshot = IfcSnapshot::default();
        assert_eq!(IfcInference::infer(&snapshot), IfcInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(IfcInference::infer(&IfcSnapshot::default()), IfcInference::default());
    }
}
//#endregion 🧪️Tests
