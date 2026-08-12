//! 💡️ Block2d inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`).
//!
//! Like block3d, block2d has no parent/child object graph — it is a single kind DEFINITION (one
//! `NodeKind` plus a catalog of rim `Block2dHandleTemplate`s placed by polar `angle`/`radius`), so
//! the honest whole-snapshot inference here is a geometric bounding box + vertex count over the
//! handle templates' rim positions (converted from polar to cartesian), expressed as a plain
//! `Inference` impl (no per-entity `InferredField` caching needed).

use crate::artifacts::block2d::Block2dSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::bounds::{compute_block2d_bounds, Block2dBounds};

//#region 🔖️Inference
/// 💡️ Everything inferable from a block2d snapshot. One field per named inference under
/// `💡️inferences/` (currently: `bounds`, backed by the `📦bounds/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.block.block2d.inference")]
pub struct Block2dInference {
    #[state(inferred)]
    pub bounds: Block2dBounds,
}

impl protocol::Inference<Block2dSnapshot> for Block2dInference {
    fn infer(snapshot: &Block2dSnapshot) -> Self {
        Self { bounds: compute_block2d_bounds(snapshot) }
    }
}

impl protocol::InferenceSpec<Block2dSnapshot> for Block2dInference {
    fn inference_schema_id() -> &'static str {
        "s.block.block2d.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.block.block2d.inference.bounds", reads: &["handles"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::block2d::standards::v1::subsets::any::schema::Block2dBuilder {
    type Snapshot = Block2dSnapshot;
    type Inference = Block2dInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.block.block2d.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `block2d_artifact_schema_descriptor`'s registration.
pub fn block2d_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.block.block2d.inference",
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
    use crate::artifacts::block2d::Block2dHandleTemplate;
    use crate::BlockKindIdentity;
    use protocol::Inference;
    use std::f64::consts::FRAC_PI_2;

    //#region 🧸️Fixtures
    fn handle(id: &str, angle: f64, radius: f64) -> Block2dHandleTemplate {
        Block2dHandleTemplate { id: id.into(), handle_kind: "wire".into(), angle, radius }
    }

    fn snapshot_with_handles(handles: Vec<Block2dHandleTemplate>) -> Block2dSnapshot {
        Block2dSnapshot {
            node_kind: BlockKindIdentity { id: "square".into(), name: "square".into(), label: "Square".into(), ..Default::default() },
            handles,
            ..Block2dSnapshot::default()
        }
    }
    //#endregion 🧸️Fixtures

    //#region 🧪️InferenceLaws
    #[test]
    fn inference_determinism_law() {
        let snapshot = snapshot_with_handles(vec![handle("h0", 0.0, 1.0), handle("h1", FRAC_PI_2, 2.0)]);
        assert_eq!(Block2dInference::infer(&snapshot), Block2dInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(Block2dInference::infer(&Block2dSnapshot::default()), Block2dInference::default());
    }

    #[test]
    fn bounds_convert_polar_handles_to_cartesian() {
        let snapshot = snapshot_with_handles(vec![handle("h0", 0.0, 1.0), handle("h1", FRAC_PI_2, 2.0)]);
        let inferred = Block2dInference::infer(&snapshot);
        let bounds = inferred.bounds.bounding_box.expect("non-empty handles produce a bounding box");
        assert!((bounds.min[0] - 0.0).abs() < 1e-9, "min x should be 0.0 (h0 at angle 0)");
        assert!((bounds.max[0] - 1.0).abs() < 1e-9, "max x should be 1.0 (h0 at angle 0, radius 1)");
        assert!((bounds.min[1] - 0.0).abs() < 1e-9, "min y should be 0.0 (h0 at angle 0)");
        assert!((bounds.max[1] - 2.0).abs() < 1e-9, "max y should be 2.0 (h1 at angle pi/2, radius 2)");
        assert_eq!(inferred.bounds.vertex_count, 2);
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
