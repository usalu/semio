//! 💡️ DxfInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`, honestly derivable from
//! `entities` and `blocks[].entities` alone — DXF R12 is a real vector CAD format with decoded
//! point-bearing entities, unlike DWG's undecoded byte payload).

use crate::artifacts::dxf::DxfSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::bounds::{compute_dxf_bounds, DxfBounds};

//#region 🔖️Inference
/// 💡️ Everything inferable from a dxf snapshot. One field per named inference under
/// `💡️inferences/` (currently: `bounds`, backed by the `📦bounds/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.dxf.inference")]
pub struct DxfInference {
    #[derived]
    pub bounds: DxfBounds,
}

impl protocol::Inference<DxfSnapshot> for DxfInference {
    async fn infer(snapshot: &DxfSnapshot) -> Self {
        Self { bounds: compute_dxf_bounds(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `DxfSnapshot::default()`'s `entities`/`blocks` ever stop being empty.
impl Default for DxfInference {
    fn default() -> Self {
        <Self as protocol::Inference<DxfSnapshot>>::infer(&DxfSnapshot::default())
    }
}

impl protocol::InferenceSpec<DxfSnapshot> for DxfInference {
    async fn inference_schema_id() -> &'static str {
        "s.stdio.dxf.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.dxf.inference.bounds", reads: &["entities", "blocks"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here — `bounds` is a single min/max fold over every entity's own
/// point fields (top-level `entities` plus every block's nested `entities`), already O(n) in
/// total entity count with no honest per-entity incremental decomposition (a merkle dep-chain
/// over this flat entity list costs more than the fold it would cache) — the default
/// `infer_cached` passthrough is exact.
impl ArtifactInferrer for crate::artifacts::dxf::standards::v_r12::subsets::any::schema::DxfBuilder {
    type Snapshot = DxfSnapshot;
    type Inference = DxfInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.dxf.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `dxf_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dxf_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.dxf.inference",
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

    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = DxfSnapshot::default();
        assert_eq!(DxfInference::infer(&snapshot), DxfInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(DxfInference::infer(&DxfSnapshot::default()), DxfInference::default());
    }
}
//#endregion 🧪️Tests
