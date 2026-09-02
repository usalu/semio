//! 💡️ Puzzle3d inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🎛flat-position/`).

use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::inferences::flatten::{flatten_snapshot, plane_to_orientation, FlattenPose};
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;
use artifact_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use std::collections::BTreeMap;

use super::flat_position::{Puzzle3dFlatCenter, Puzzle3dFlatPlane};

//#region 🔖️Inference
/// 💡️ Everything inferable from a puzzle3d snapshot. One field per named inference under
/// `💡️inferences/` (currently: `flatPositions`, backed by the `🎛flat-position/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.puzzle.puzzle3d.inference")]
pub struct Puzzle3dInference {
    #[derived]
    pub flat_positions: BTreeMap<String, FlattenPose>,
}

impl protocol::Inference<Puzzle3dSnapshot> for Puzzle3dInference {
    fn infer(snapshot: &Puzzle3dSnapshot) -> Self {
        Self { flat_positions: flatten_snapshot(snapshot).into_iter().collect() }
    }
}

impl protocol::InferenceSpec<Puzzle3dSnapshot> for Puzzle3dInference {
    fn inference_schema_id() -> &'static str {
        "s.puzzle.puzzle3d.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[
            protocol::InferenceFieldSpec { id: "s.puzzle.puzzle3d.inference.flatPosition.plane", reads: &["objects", "attractions"] },
            protocol::InferenceFieldSpec { id: "s.puzzle.puzzle3d.inference.flatPosition.center", reads: &["objects", "attractions"] },
        ]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::Puzzle3dBuilder {
    type Snapshot = Puzzle3dSnapshot;
    type Inference = Puzzle3dInference;

    async fn infer_cached(snapshot: &Self::Snapshot, cache: &mut store::InferenceCache, session: &mut store::InferenceSession) -> Self::Inference {
        let _ = session;
        let planes = store::infer_field::<Puzzle3dSnapshot, Puzzle3dFlatPlane>(snapshot, Some(cache));
        let centers = store::infer_field::<Puzzle3dSnapshot, Puzzle3dFlatCenter>(snapshot, Some(cache));
        let flat_positions = planes
            .into_iter()
            .map(|(id, plane)| {
                let center = centers.get(&id).copied().unwrap_or([0.0, 0.0]);
                let orientation = plane_to_orientation(plane);
                (id, FlattenPose { plane, center, orientation })
            })
            .collect();
        Puzzle3dInference { flat_positions }
    }
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.puzzle.puzzle3d.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `puzzle3d_artifact_schema_descriptor`'s registration.
pub fn puzzle3d_artifact_inference_descriptor() -> artifact_schema::ArtifactInferenceDescriptor {
    artifact_schema::ArtifactInferenceDescriptor {
        id: "s.puzzle.puzzle3d.inference",
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
    use crate::artifacts::puzzle3d::{Puzzle3dAttraction, Puzzle3dObject, Puzzle3dObjectAnchor, Puzzle3dVortex};
    use protocol::Inference;

    //#region 🧸️Fixtures
    fn vortex(id: &str, position: [f64; 3], direction: [f64; 3]) -> Puzzle3dVortex {
        Puzzle3dVortex { id: id.into(), vortex_kind: None, label: None, position, direction: Some(direction), radius: None, hidden: false, locked: false }
    }

    fn object(id: &str, origin: [f64; 3], anchor: Puzzle3dObjectAnchor, vortices: Vec<Puzzle3dVortex>) -> Puzzle3dObject {
        Puzzle3dObject { id: id.into(), label: None, object_kind: None, anchor, origin, orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, mesh_url: None, vortices, hidden: false, locked: false }
    }

    fn chain_snapshot() -> Puzzle3dSnapshot {
        // root -A- mid -B- leaf: a 3-object chain — same shape as the flat-position slug's own
        // fixture, kept independent per-file per this repo's inline-fixture convention.
        let root = object("root", [0.0, 0.0, 0.0], Puzzle3dObjectAnchor::Fixed, vec![vortex("top", [0.0, 0.0, 1.0], [0.0, 0.0, 1.0])]);
        let mid = object("mid", [0.0, 0.0, 0.0], Puzzle3dObjectAnchor::Derived, vec![vortex("bottom", [0.0, 0.0, -1.0], [0.0, 0.0, -1.0]), vortex("top", [0.0, 0.0, 1.0], [0.0, 0.0, 1.0])]);
        let leaf = object("leaf", [0.0, 0.0, 0.0], Puzzle3dObjectAnchor::Derived, vec![vortex("bottom", [0.0, 0.0, -1.0], [0.0, 0.0, -1.0])]);
        let attraction_a = Puzzle3dAttraction { id: "a1".into(), attracting: "root:top".into(), attracted: "mid:bottom".into(), gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 1.0, y: 0.0 };
        let attraction_b = Puzzle3dAttraction { id: "a2".into(), attracting: "mid:top".into(), attracted: "leaf:bottom".into(), gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 1.0 };
        Puzzle3dSnapshot {
            schema: crate::artifacts::puzzle3d::PUZZLE_3D_SCHEMA.to_string(),
            domain: "architecture".into(),
            meta: Default::default(),
            objects: vec![root, mid, leaf],
            attractions: vec![attraction_a, attraction_b],
            target_volumes: Vec::new(),
            references: Vec::new(),
        }
    }
    //#endregion 🧸️Fixtures

    //#region 🧪️InferenceLaws
    #[test]
    fn inference_determinism_law() {
        let snapshot = chain_snapshot();
        assert_eq!(Puzzle3dInference::infer(&snapshot), Puzzle3dInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(Puzzle3dInference::infer(&Puzzle3dSnapshot::default()), Puzzle3dInference::default());
    }

    #[test]
    fn inference_matches_flatten_snapshot_directly() {
        let snapshot = chain_snapshot();
        let inferred = Puzzle3dInference::infer(&snapshot);
        let direct = flatten_snapshot(&snapshot);
        for (id, pose) in &direct {
            assert_eq!(inferred.flat_positions.get(id), Some(pose), "inference must match flatten_snapshot exactly for {id}");
        }
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
