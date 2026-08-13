//! 💡️ Remodel inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (`📦bounds/`, `🔄relative-pose/`).
//!
//! `results.mesh.mesh` (`MeshData`) is the only field on this snapshot with real 3D geometry
//! (`positions`/`indices`), so the honest whole-snapshot derivation is the reconstructed mesh's
//! axis-aligned bounding box plus vertex/face counts — `bounds` stays a plain whole-snapshot
//! `protocol::Inference` leaf for that reason. `relative_camera_poses` is the other kind: a real
//! per-entity `store::InferredField<RemodelSnapshot>` DAG over `results.trajectory.poses`, added
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave M3d as this artifact's
//! first genuine `DepHash`-chained CQRS path (see `🔄relative-pose/🦀️component.rs`).

use crate::artifacts::remodel::RemodelSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::bounds::compute_remodel_bounds;
use super::relative_pose::RemodelRelativeCameraPose;

pub use super::bounds::{RemodelBoundingBox, RemodelBounds};
pub use super::relative_pose::RemodelPoseDelta;

//#region 🔖️Inference
/// 💡️ Everything inferable from a remodel snapshot. One field per named inference under
/// `💡️inferences/` (currently: `bounds`, a whole-snapshot `protocol::Inference` leaf backed by
/// `📦bounds/`; `relative_camera_poses`, a real per-entity `store::InferredField` DAG backed by
/// `🔄relative-pose/` — see that leaf's own docstring for why it, not `bounds`, is this artifact's
/// genuine `DepHash`-chained CQRS path).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.remodel.remodel.inference")]
pub struct RemodelInference {
    #[derived]
    pub bounds: RemodelBounds,
    #[derived]
    pub relative_camera_poses: BTreeMap<String, RemodelPoseDelta>,
}

impl protocol::Inference<RemodelSnapshot> for RemodelInference {
    fn infer(snapshot: &RemodelSnapshot) -> Self {
        Self { bounds: compute_remodel_bounds(snapshot), relative_camera_poses: store::infer_field::<RemodelSnapshot, RemodelRelativeCameraPose>(snapshot, None) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `RemodelSnapshot::default()`'s mesh ever stops being empty. Same "match `infer` of the real
/// default, don't derive structurally" trick `AddInference` uses in `📡️spr/🎮️command/🦀️component.rs`.
impl Default for RemodelInference {
    fn default() -> Self {
        <Self as protocol::Inference<RemodelSnapshot>>::infer(&RemodelSnapshot::default())
    }
}

impl protocol::InferenceSpec<RemodelSnapshot> for RemodelInference {
    fn inference_schema_id() -> &'static str {
        "s.remodel.remodel.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[
            protocol::InferenceFieldSpec { id: "s.remodel.remodel.inference.bounds", reads: &["results"] },
            protocol::InferenceFieldSpec { id: "s.remodel.remodel.inference.relative_camera_pose", reads: &["results"] },
        ]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::remodel::standards::v1::subsets::any::schema::RemodelBuilder {
    type Snapshot = RemodelSnapshot;
    type Inference = RemodelInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.remodel.remodel.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `remodel_artifact_schema_descriptor`'s registration.
pub fn remodel_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.remodel.remodel.inference",
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
    use crate::artifacts::remodel::mint_and_stash_mesh;
    use protocol::Inference;
    use semio_framework::MeshData;

    fn triangle_snapshot() -> RemodelSnapshot {
        let mut snapshot = RemodelSnapshot::default();
        snapshot.results.mesh.mesh = mint_and_stash_mesh(MeshData { positions: vec![0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 3.0, 0.0], indices: vec![0, 1, 2], ..MeshData::default() });
        snapshot
    }

    #[test]
    fn inference_determinism_law() {
        let snapshot = triangle_snapshot();
        assert_eq!(RemodelInference::infer(&snapshot), RemodelInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(RemodelInference::infer(&RemodelSnapshot::default()), RemodelInference::default());
    }

    #[test]
    fn bounds_covers_the_mesh_vertices_and_counts_it_exactly() {
        let inferred = RemodelInference::infer(&triangle_snapshot());
        assert_eq!(inferred.bounds.vertex_count, 3);
        assert_eq!(inferred.bounds.face_count, 1);
        assert_eq!(inferred.bounds.bounding_box.min, [0.0, 0.0, 0.0]);
        assert_eq!(inferred.bounds.bounding_box.max, [2.0, 3.0, 0.0]);
    }
}
//#endregion 🧪️Tests
