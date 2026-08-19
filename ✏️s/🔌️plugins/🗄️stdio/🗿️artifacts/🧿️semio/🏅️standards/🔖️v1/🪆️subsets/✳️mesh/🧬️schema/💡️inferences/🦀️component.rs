//! 💡️ SemioMeshInference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING, authored here
//! by DKM per the standing exclusion: IIF's inference fan-out explicitly excludes `✳️brep`/
//! `✳️drawing`/`✳️mesh` and defers them). Directory shape mirrors `🧬️mutations/`: this file is the
//! family-root assembly (never mod's/includes the slug dirs directly — `📦️glue.rs` is the sole
//! mounting mechanism, same as mutations); each named inference gets its own `<emoji><slug>/` child
//! (currently: `📦aabb/`).
//!
//! `computed-normals`/`tessellation-preview` are deliberately NOT fields here — see
//! `📦aabb/🦀️component.rs`'s module doc comment for why (a real chain would either shadow this
//! subset's own authored `normals` field with a competing definition, or merely copy
//! already-authoritative snapshot data under a different name; faking either was rejected, not
//! merely deferred).

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::aabb::SemioAabb;
#[cfg(test)]
use super::aabb::aabb_key;

//#region 🔖️Inference
/// 💡️ Everything inferable from a mesh snapshot. One field per named inference under
/// `💡️inferences/` (currently: `aabb`, backed by the `📦aabb/` slug dir), keyed per
/// `"{meshId}:{primitiveId}"`. `BTreeMap` (not `HashMap`) — `store::infer_field`'s own real
/// return type, ordered/deterministic by key.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.mesh.inference")]
pub struct SemioMeshInference {
    #[derived]
    pub aabb: BTreeMap<String, SemioAabb>,
}

impl protocol::Inference<SemioMeshSnapshot> for SemioMeshInference {
    async fn infer(snapshot: &SemioMeshSnapshot) -> Self {
        Self { aabb: store::infer_field::<SemioMeshSnapshot, super::aabb::MeshAabb>(snapshot, None) }
    }
}

impl protocol::InferenceSpec<SemioMeshSnapshot> for SemioMeshInference {
    async fn inference_schema_id() -> &'static str {
        "s.stdio.semio.mesh.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.semio.mesh.inference.aabb", reads: &["meshes"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::semio::standards::v1::subsets::mesh::schema::SemioMeshBuilder {
    type Snapshot = SemioMeshSnapshot;
    type Inference = SemioMeshInference;

    async fn infer_cached(snapshot: &Self::Snapshot, cache: &mut store::InferenceCache, session: &mut store::InferenceSession) -> Self::Inference {
        let _ = session;
        SemioMeshInference { aabb: store::infer_field::<SemioMeshSnapshot, super::aabb::MeshAabb>(snapshot, Some(cache)) }
    }
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.semio.mesh.inference`'s facet leaves into the OS-wide inference catalog.
/// The `register_artifact_inferences()` call site itself lives in the SHARED
/// `🏅️standards/🔖️v1/⚙️engine/🦀️component.rs` (aggregates all 14 `s.stdio.semio.*` subsets'
/// `register()` calls) — out of this ticket's `✳️mesh/`-only edit scope, same boundary brep's own
/// wave already flagged. Flagged under `## sharedFileRequests` in the wave report, not wired here.
pub async fn semio_mesh_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.semio.mesh.inference",
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
        let snapshot = SemioMeshSnapshot::default();
        assert_eq!(SemioMeshInference::infer(&snapshot), SemioMeshInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(SemioMeshInference::infer(&SemioMeshSnapshot::default()), SemioMeshInference::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_covers_every_primitive_by_composite_key() {
        use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint3;
        use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioPrimitive};
        let snapshot = SemioMeshSnapshot { meshes: vec![SemioMesh { id: "m1".into(), primitives: vec![SemioPrimitive { id: "p1".into(), positions: vec![SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 }], ..Default::default() }] }], ..Default::default() };
        let inference = SemioMeshInference::infer(&snapshot);
        assert!(inference.aabb.contains_key(&aabb_key("m1", "p1")));
    }
}
//#endregion 🧪️Tests
