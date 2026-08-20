//! 💡️ ObjInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`, honestly derivable from
//! `vertices`/`faces`/`groups` alone).

use crate::artifacts::obj::schema::snapshot::ObjSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::bounds::{compute_obj_bounds, ObjBounds};

//#region 🔖️Inference
/// 💡️ Everything inferable from an obj snapshot. One field per named inference under
/// `💡️inferences/` (currently: `bounds`, backed by the `📦bounds/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.obj.inference")]
pub struct ObjInference {
    #[derived]
    pub bounds: ObjBounds,
}

impl protocol::Inference<ObjSnapshot> for ObjInference {
    async fn infer(snapshot: &ObjSnapshot) -> Self {
        Self { bounds: compute_obj_bounds(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `ObjSnapshot::default()`'s `vertices`/`faces`/`groups` ever stop being empty.
impl Default for ObjInference {
    fn default() -> Self {
        <Self as protocol::Inference<ObjSnapshot>>::infer(&ObjSnapshot::default())
    }
}

impl protocol::InferenceSpec<ObjSnapshot> for ObjInference {
    async fn inference_schema_id() -> &'static str {
        "s.stdio.obj.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.obj.inference.bounds", reads: &["vertices", "faces", "groups"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here — `bounds` is a single min/max fold over `vertices` plus direct
/// `faces`/`groups` tallies, already O(n) in total vertex count with no honest per-entity
/// incremental decomposition (a merkle dep-chain over this flat vertex list costs more than the
/// fold it would cache) — the default `infer_cached` passthrough is exact.
impl ArtifactInferrer for crate::artifacts::obj::standards::v3_0::subsets::any::schema::ObjBuilder {
    type Snapshot = ObjSnapshot;
    type Inference = ObjInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.obj.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `obj_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn obj_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.obj.inference",
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
        let snapshot = ObjSnapshot::default();
        assert_eq!(ObjInference::infer(&snapshot), ObjInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(ObjInference::infer(&ObjSnapshot::default()), ObjInference::default());
    }
}
//#endregion 🧪️Tests
