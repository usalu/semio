//! 💡️ Puzzle3d inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📍️flat-position/`).

use crate::standards::v1::subsets::any::schema::inferences::flatten::{flatten_snapshot, plane_to_orientation};
use crate::Puzzle3dSnapshot;
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use std::collections::BTreeMap;

use super::flat_position::{Puzzle3dFlatCenter, Puzzle3dFlatPlane};
//#region 🔖️Inference
/// 💡️ Everything inferable from a puzzle3d snapshot. One field per named inference under
/// `💡️inferences/` (currently: `flatPositions`, backed by the `📍️flat-position/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
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
impl ArtifactInferrer for crate::standards::v1::subsets::any::schema::Puzzle3dBuilder {
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
pub fn puzzle3d_artifact_inference_descriptor() -> ::semio_framework_schema::ArtifactInferenceDescriptor {
    ::semio_framework_schema::ArtifactInferenceDescriptor {
        id: "s.puzzle.puzzle3d.inference",
        inference: ::semio_framework_schema::FacetLeaves {
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::standards::v1::subsets::any::schema::inferences::flatten::FlattenPose;
//#endregion 🔁️Re-exports
