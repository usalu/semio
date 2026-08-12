//! 💡️ Puzzle2d inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🎛flat-position/`, the same
//! graph-BFS-derived positioning concept `🧊️3d`'s own `🎛flat-position/` and `🔱️trinity/🔌️jack`'s own
//! `🎛flat-position/` carry for their artifacts — here reusing the existing
//! `⚙️engine/📐️layout::fastened_layout_snapshot` compose-parity math directly rather than
//! duplicating it, a plain whole-snapshot BFS pass, so no `InferredField`/incremental caching is
//! needed, matching both siblings' own "simple whole-snapshot scalars" rationale).

use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
use artifact_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::flat_position::{compute_flat_position, Puzzle2dFlatPosition};

//#region 🔖️Inference
/// 💡️ Everything inferable from a puzzle2d snapshot. One field per named inference under
/// `💡️inferences/` (currently: `flatPosition`, backed by the `🎛flat-position/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.puzzle.puzzle2d.inference")]
pub struct Puzzle2dInference {
    #[state(inferred)]
    pub flat_position: Puzzle2dFlatPosition,
}

impl protocol::Inference<Puzzle2dSnapshot> for Puzzle2dInference {
    fn infer(snapshot: &Puzzle2dSnapshot) -> Self {
        Self { flat_position: compute_flat_position(snapshot) }
    }
}

impl protocol::InferenceSpec<Puzzle2dSnapshot> for Puzzle2dInference {
    fn inference_schema_id() -> &'static str {
        "s.puzzle.puzzle2d.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.puzzle.puzzle2d.inference.flatPosition", reads: &["nodes", "edges"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 🧠️ Uncached: the underlying `fastened_layout_snapshot` BFS re-runs in one pass over the whole
/// graph — the default `infer_cached` passthrough (just calls `infer`) is exactly right here, no
/// `InferredField` chain needed (mirrors jack's own `🎛flat-position`/`🧭topology` rationale).
impl ArtifactInferrer for crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::Puzzle2dBuilder {
    type Snapshot = Puzzle2dSnapshot;
    type Inference = Puzzle2dInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.puzzle.puzzle2d.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `puzzle2d_artifact_schema_descriptor`'s registration.
pub fn puzzle2d_artifact_inference_descriptor() -> artifact_schema::ArtifactInferenceDescriptor {
    artifact_schema::ArtifactInferenceDescriptor {
        id: "s.puzzle.puzzle2d.inference",
        inference: artifact_schema::FacetLeaves {
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
    use crate::artifacts::puzzle2d::{Puzzle2dEdge, Puzzle2dHandle, Puzzle2dNode, Puzzle2dNodeAnchor};
    use protocol::Inference;

    //#region 🧸️Fixtures
    fn parent_child_snapshot() -> Puzzle2dSnapshot {
        // p (Fixed, off-origin) --e-- c (Derived): edge x/y offsets place c relative to p.
        let p = Puzzle2dNode {
            id: "p".into(),
            x: 5.0,
            y: 7.0,
            anchor: Puzzle2dNodeAnchor::Fixed,
            handles: vec![Puzzle2dHandle { id: "h".into(), ..Default::default() }],
            ..Default::default()
        };
        let c = Puzzle2dNode {
            id: "c".into(),
            anchor: Puzzle2dNodeAnchor::Derived,
            handles: vec![Puzzle2dHandle { id: "h".into(), ..Default::default() }],
            ..Default::default()
        };
        let e = Puzzle2dEdge { id: "e".into(), source: "p:h".into(), target: "c:h".into(), x: 3.0, y: -2.0, ..Default::default() };
        Puzzle2dSnapshot { schema: crate::artifacts::puzzle2d::PUZZLE_2D_SCHEMA.to_string(), camera: Default::default(), nodes: vec![p, c], edges: vec![e], meta: Default::default() }
    }
    //#endregion 🧸️Fixtures

    //#region 🧪️InferenceLaws
    #[test]
    fn inference_determinism_law() {
        let snapshot = parent_child_snapshot();
        assert_eq!(Puzzle2dInference::infer(&snapshot), Puzzle2dInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(Puzzle2dInference::infer(&Puzzle2dSnapshot::default()), Puzzle2dInference::default());
    }

    #[test]
    fn inference_matches_compute_flat_position_directly() {
        let snapshot = parent_child_snapshot();
        let inferred = Puzzle2dInference::infer(&snapshot);
        assert_eq!(inferred.flat_position, compute_flat_position(&snapshot));
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
