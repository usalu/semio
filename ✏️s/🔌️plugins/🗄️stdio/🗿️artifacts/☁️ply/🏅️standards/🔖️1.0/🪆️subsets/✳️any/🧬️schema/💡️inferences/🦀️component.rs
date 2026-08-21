//! 💡️ PlyInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`, honestly derivable from
//! `elements` alone — PLY's own convention names the vertex-carrying element `"vertex"` with
//! `x`/`y`/`z` scalar properties, and the face-carrying element `"face"`).

use crate::artifacts::ply::PlySnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::bounds::{compute_ply_bounds, PlyBounds};

//#region 🔖️Inference
/// 💡️ Everything inferable from a ply snapshot. One field per named inference under
/// `💡️inferences/` (currently: `bounds`, backed by the `📦bounds/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ply.inference")]
pub struct PlyInference {
    #[derived]
    pub bounds: PlyBounds,
}

impl protocol::Inference<PlySnapshot> for PlyInference {
    fn infer(snapshot: &PlySnapshot) -> Self {
        Self { bounds: compute_ply_bounds(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `PlySnapshot::default()`'s `elements` ever stops being empty.
impl Default for PlyInference {
    fn default() -> Self {
        <Self as protocol::Inference<PlySnapshot>>::infer(&PlySnapshot::default())
    }
}

impl protocol::InferenceSpec<PlySnapshot> for PlyInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.ply.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.ply.inference.bounds", reads: &["elements"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here — `bounds` is a single min/max fold over the `"vertex"` element's
/// own `x`/`y`/`z` property columns plus a row count of the `"face"` element, already O(n) in
/// total row count with no honest per-row incremental decomposition (a merkle dep-chain over this
/// flat row list costs more than the fold it would cache) — the default `infer_cached`
/// passthrough is exact.
impl ArtifactInferrer for crate::artifacts::ply::standards::v1_0::subsets::any::schema::PlyBuilder {
    type Snapshot = PlySnapshot;
    type Inference = PlyInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.ply.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `ply_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn ply_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.ply.inference",
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
        let snapshot = PlySnapshot::default();
        assert_eq!(PlyInference::infer(&snapshot), PlyInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(PlyInference::infer(&PlySnapshot::default()), PlyInference::default());
    }
}
//#endregion 🧪️Tests
