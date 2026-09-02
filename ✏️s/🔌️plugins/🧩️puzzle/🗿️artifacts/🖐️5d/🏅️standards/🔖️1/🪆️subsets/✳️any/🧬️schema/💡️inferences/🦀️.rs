//! 💡️ Puzzle5d inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors puzzle3d's own `💡️inferences/` (this artifact's exemplar): this file is the
//! family-root assembly (never mod's/includes the slug dirs directly — `🦀️.rs` is the sole
//! mounting mechanism, same as mutations); each named inference gets its own `<emoji><slug>/` child
//! (currently: `🎛flat-position/`).
//!
//! 🎛 Puzzle5d's flatten math lives at `💡️inferences/🎛flat-position` (`flatten_snapshot`, which maps
//! parts/grips/fasteners onto the 3d object/vortex/attraction graph and runs puzzle3d's own solver) —
//! unlike puzzle3d's own inference, there is no separate low-level per-edge decomposition exposed
//! here to drive an incremental `InferredField` chain, so this inference is a plain whole-snapshot
//! `Inference` impl (per the family root's own "simple whole-snapshot scalars" guidance) that calls
//! that sibling slug's function directly; `ArtifactInferrer::infer_cached`'s default passthrough
//! (just calls `infer`) is used as-is, uncached.

use crate::artifacts::puzzle3d::schema::inferences::flatten::FlattenPose;
use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::inferences::flat_position::flatten_snapshot;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;
use artifact_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Inference
/// 💡️ Everything inferable from a puzzle5d snapshot. One field per named inference under
/// `💡️inferences/` (currently: `flatPositions`, backed by the `🎛flat-position/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.puzzle.puzzle5d.inference")]
pub struct Puzzle5dInference {
    #[derived]
    pub flat_positions: BTreeMap<String, FlattenPose>,
}

impl protocol::Inference<Puzzle5dSnapshot> for Puzzle5dInference {
    fn infer(snapshot: &Puzzle5dSnapshot) -> Self {
        Self { flat_positions: flatten_snapshot(snapshot).into_iter().collect() }
    }
}

impl protocol::InferenceSpec<Puzzle5dSnapshot> for Puzzle5dInference {
    fn inference_schema_id() -> &'static str {
        "s.puzzle.puzzle5d.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.puzzle.puzzle5d.inference.flatPosition", reads: &["parts", "fasteners"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 🧠️ Uncached: `flatten_snapshot` recomputes the whole graph in one pass and puzzle5d's engine
/// exposes no per-edge decomposition to key an `InferredField` chain off of (see the module doc) —
/// the default `infer_cached` passthrough (just calls `infer`) is exactly right here.
impl ArtifactInferrer for crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::Puzzle5dBuilder {
    type Snapshot = Puzzle5dSnapshot;
    type Inference = Puzzle5dInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.puzzle.puzzle5d.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `puzzle5d_artifact_schema_descriptor`'s registration.
pub fn puzzle5d_artifact_inference_descriptor() -> artifact_schema::ArtifactInferenceDescriptor {
    artifact_schema::ArtifactInferenceDescriptor {
        id: "s.puzzle.puzzle5d.inference",
        inference: artifact_schema::FacetLeaves {
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
    use crate::artifacts::puzzle5d::{Puzzle5dFastener, Puzzle5dGrip, Puzzle5dGrip2d, Puzzle5dGrip3d, Puzzle5dMeta, Puzzle5dPart, Puzzle5dPart2d, Puzzle5dPart3d, Puzzle5dPartAnchor};
    use protocol::Inference;

    //#region 🧸️Fixtures
    fn chain_snapshot() -> Puzzle5dSnapshot {
        // p -f- c: a 2-part chain — same shape as puzzle3d's own inference fixture, kept
        // independent per-file per this repo's inline-fixture convention.
        let parent = Puzzle5dPart {
            id: "p".into(),
            part_kind: None,
            anchor: Puzzle5dPartAnchor::Fixed,
            part_2d: Puzzle5dPart2d { x: 10.0, y: 20.0, shape: None, radius: None, width: None, height: None, text: None, icon_kind: None, hidden: None, locked: None },
            part_3d: Puzzle5dPart3d { origin: [0.0, 0.0, 0.0], mesh_url: None, orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, label: None },
            grips: vec![Puzzle5dGrip {
                id: "top".into(),
                grip_kind: None,
                grip_2d: Puzzle5dGrip2d { angle: 0.0, grip_kind: None, radius: None },
                grip_3d: Puzzle5dGrip3d { position: [0.0, 0.0, 1.0], direction: Some([0.0, 0.0, 1.0]), radius: None, label: None },
            }],
        };
        let child = Puzzle5dPart {
            id: "c".into(),
            part_kind: None,
            anchor: Puzzle5dPartAnchor::Derived,
            part_2d: Puzzle5dPart2d { x: 0.0, y: 0.0, shape: None, radius: None, width: None, height: None, text: None, icon_kind: None, hidden: None, locked: None },
            part_3d: Puzzle5dPart3d { origin: [0.0, 0.0, 0.0], mesh_url: None, orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, label: None },
            grips: vec![Puzzle5dGrip {
                id: "bottom".into(),
                grip_kind: None,
                grip_2d: Puzzle5dGrip2d { angle: 0.0, grip_kind: None, radius: None },
                grip_3d: Puzzle5dGrip3d { position: [0.0, 0.0, -1.0], direction: Some([0.0, 0.0, -1.0]), radius: None, label: None },
            }],
        };
        let fastener = Puzzle5dFastener { id: "f".into(), source: "p:top".into(), target: "c:bottom".into(), fastener_kind: None, gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 1.5, y: 2.5 };
        Puzzle5dSnapshot {
            schema: "puzzle.5d".into(),
            domain: "architecture".into(),
            label: None,
            meta: Puzzle5dMeta::default(),
            kind_catalogs: None,
            kind_catalogs_extra: None,
            kind_compatibility: Vec::new(),
            parts: vec![parent, child],
            fasteners: vec![fastener],
        }
    }
    //#endregion 🧸️Fixtures

    //#region 🧪️InferenceLaws
    #[test]
    fn inference_determinism_law() {
        let snapshot = chain_snapshot();
        assert_eq!(Puzzle5dInference::infer(&snapshot), Puzzle5dInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(Puzzle5dInference::infer(&Puzzle5dSnapshot::default()), Puzzle5dInference::default());
    }

    #[test]
    fn inference_matches_flatten_snapshot_directly() {
        let snapshot = chain_snapshot();
        let inferred = Puzzle5dInference::infer(&snapshot);
        let direct = flatten_snapshot(&snapshot);
        for (id, pose) in &direct {
            assert_eq!(inferred.flat_positions.get(id), Some(pose), "inference must match flatten_snapshot exactly for {id}");
        }
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
