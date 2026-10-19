//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! 💡️ Puzzle3d inference schema — the fourth schema family alongside snapshot/diff/mutations
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📍️flat-position/`).

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

use crate::flatten::flatten_snapshot;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;
use ::semio_framework_schema::ArtifactSchema;
use std::collections::BTreeMap;

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
    fn infer(snapshot: &Puzzle3dSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { flat_positions: flatten_snapshot(snapshot).into_iter().collect() }
    
        })
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

//#region 🔖️Descriptor
/// 💡️ Registers `s.puzzle.puzzle3d.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `puzzle3d_artifact_schema_descriptor`'s registration.
pub fn puzzle3d_artifact_inference_descriptor() -> ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
    ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.puzzle.puzzle3d.inference",
        inference: ::semio_framework_schema_registry::FacetLeaves {
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
pub use crate::flatten::FlattenPose;
//#endregion 🔁️Re-exports

//#region 🌳️Modules
#[path = "."]
pub mod flat_position {
    #[path = "📍️flat-position/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod flatten {
    #[path = "🗜️flatten/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod io {
    #[path = "."]
    pub mod binary {
        #[path = "../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
        pub mod inferences;
    }
    #[path = "."]
    pub mod text {
        #[path = "../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
        pub mod inferences;
    }
}
//#endregion 🌳️Modules
