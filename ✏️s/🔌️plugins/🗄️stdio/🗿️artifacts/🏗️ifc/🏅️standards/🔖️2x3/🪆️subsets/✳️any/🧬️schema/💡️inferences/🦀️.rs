//! 💡️ Ifc2x3Inference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`, honestly derivable by
//! folding every real `IFCCARTESIANPOINT((x,y,z));` instance in `document.instances` — the same
//! buildingSMART Coordination View 2.0-era Part-21 entity keyword IFC4 uses, since 2x3 rides the
//! identical ISO 10303-21 syntax with an older EXPRESS schema).

use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use super::bounds::{compute_ifc2x3_bounds, Ifc2x3Bounds};

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
    fn infer(snapshot: &Ifc2x3Snapshot) -> Self {
        Self { bounds: compute_ifc2x3_bounds(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `Ifc2x3Snapshot::default()`'s `document` ever stops being empty.
impl Default for Ifc2x3Inference {
    fn default() -> Self {
        <Self as protocol::Inference<Ifc2x3Snapshot>>::infer(&Ifc2x3Snapshot::default())
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

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here — `bounds` is a single min/max fold over every
/// `IFCCARTESIANPOINT` instance in `document.instances`, already O(n) in total instance count
/// with no honest per-entity incremental decomposition (a merkle dep-chain over this flat
/// instance list costs more than the fold it would cache) — the default `infer_cached`
/// passthrough is exact.
impl ArtifactInferrer for crate::artifacts::ifc::standards::v2x3::subsets::any::schema::Ifc2x3Builder {
    type Snapshot = Ifc2x3Snapshot;
    type Inference = Ifc2x3Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.ifc.2x3.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `ifc2x3_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn ifc2x3_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.ifc.2x3.inference",
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
        let snapshot = Ifc2x3Snapshot::default();
        assert_eq!(Ifc2x3Inference::infer(&snapshot), Ifc2x3Inference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(Ifc2x3Inference::infer(&Ifc2x3Snapshot::default()), Ifc2x3Inference::default());
    }
}
//#endregion 🧪️Tests
