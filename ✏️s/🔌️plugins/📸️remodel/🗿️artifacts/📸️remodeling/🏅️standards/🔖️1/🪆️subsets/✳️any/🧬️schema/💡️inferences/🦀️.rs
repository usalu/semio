//! 💡️ Remodeling inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (`📦bounds/`, `🔄relative-pose/`).
//!
//! `results.mesh.mesh` (`MeshData`) is the only field on this snapshot with real 3D geometry
//! (`positions`/`indices`), so the honest whole-snapshot derivation is the reconstructed mesh's
//! axis-aligned bounding box plus vertex/face counts — `bounds` stays a plain whole-snapshot
//! `protocol::Inference` leaf for that reason. `relative_camera_poses` is the other kind: a real
//! per-entity `store::InferredField<RemodelingSnapshot>` DAG over `results.trajectory.poses`, added
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave M3d as this artifact's
//! first genuine `DepHash`-chained CQRS path (see `🔄relative-pose/🦀️.rs`).

use crate::artifacts::remodeling::RemodelingSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;

use super::bounds::compute_remodeling_bounds;
use super::relative_pose::RemodelingRelativeCameraPose;

pub use super::bounds::{RemodelingBoundingBox, RemodelingBounds};
pub use super::relative_pose::RemodelingPoseDelta;

//#region 🔖️Inference
/// 💡️ Everything inferable from a remodeling snapshot. One field per named inference under
/// `💡️inferences/` (currently: `bounds`, a whole-snapshot `protocol::Inference` leaf backed by
/// `📦bounds/`; `relative_camera_poses`, a real per-entity `store::InferredField` DAG backed by
/// `🔄relative-pose/` — see that leaf's own docstring for why it, not `bounds`, is this artifact's
/// genuine `DepHash`-chained CQRS path).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.remodeling.remodeling.inference")]
pub struct RemodelingInference {
    #[derived]
    pub bounds: RemodelingBounds,
    #[derived]
    pub relative_camera_poses: BTreeMap<String, RemodelingPoseDelta>,
}

impl protocol::Inference<RemodelingSnapshot> for RemodelingInference {
    async fn infer(snapshot: &RemodelingSnapshot) -> Self {
        Self { bounds: compute_remodeling_bounds(snapshot), relative_camera_poses: store::infer_field::<RemodelingSnapshot, RemodelingRelativeCameraPose>(snapshot, None) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `RemodelingSnapshot::default()`'s mesh ever stops being empty. Same "match `infer` of the real
/// default, don't derive structurally" trick `AddInference` uses in `📡️spr/🎮️command/🦀️.rs`.
impl Default for RemodelingInference {
    fn default() -> Self {
        <Self as protocol::Inference<RemodelingSnapshot>>::infer(&RemodelingSnapshot::default())
    }
}

impl protocol::InferenceSpec<RemodelingSnapshot> for RemodelingInference {
    async fn inference_schema_id() -> &'static str {
        "s.remodeling.remodeling.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.remodeling.remodeling.inference.bounds", reads: &["results"] }, protocol::InferenceFieldSpec { id: "s.remodeling.remodeling.inference.relative_camera_pose", reads: &["results"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::remodeling::standards::v1::subsets::any::schema::RemodelingBuilder {
    type Snapshot = RemodelingSnapshot;
    type Inference = RemodelingInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.remodeling.remodeling.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `remodeling_artifact_schema_descriptor`'s registration.
pub async fn remodeling_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.remodeling.remodeling.inference",
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
    use crate::artifacts::remodeling::mint_and_stash_mesh;
    use protocol::Inference;
    use semio_framework::MeshData;

    async fn triangle_snapshot() -> RemodelingSnapshot {
        let mut snapshot = RemodelingSnapshot::default();
        snapshot.results.mesh.mesh = mint_and_stash_mesh(MeshData { positions: vec![0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 3.0, 0.0], indices: vec![0, 1, 2], ..MeshData::default() });
        snapshot
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = triangle_snapshot();
        assert_eq!(RemodelingInference::infer(&snapshot), RemodelingInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(RemodelingInference::infer(&RemodelingSnapshot::default()), RemodelingInference::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn bounds_covers_the_mesh_vertices_and_counts_it_exactly() {
        let inferred = RemodelingInference::infer(&triangle_snapshot());
        assert_eq!(inferred.bounds.vertex_count, 3);
        assert_eq!(inferred.bounds.face_count, 1);
        assert_eq!(inferred.bounds.bounding_box.min, [0.0, 0.0, 0.0]);
        assert_eq!(inferred.bounds.bounding_box.max, [2.0, 3.0, 0.0]);
    }
}
//#endregion 🧪️Tests
