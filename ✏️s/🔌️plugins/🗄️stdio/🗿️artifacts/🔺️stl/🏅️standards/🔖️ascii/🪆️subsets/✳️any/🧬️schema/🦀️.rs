//! 🧬️ StlArtifact schema — full artifact state.

use crate::{StlSnapshot, STDIO_STL_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full `stdio.stl` artifact state.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.stl")]
pub struct StlArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub solid_name: String,
    #[state(artifact)]
    #[value(default)]
    pub triangles: Vec<StlTriangle>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for StlArtifact {
    fn default() -> Self {
        Self::from_snapshot(StlSnapshot::default())
    }
}

impl StlArtifact {
    /// 📸️ Persisted subset.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> StlSnapshot {
        StlSnapshot { schema: self.schema.clone(), solid_name: self.solid_name.clone(), triangles: self.triangles.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: StlSnapshot) -> Self {
        Self { schema: snapshot.schema, solid_name: snapshot.solid_name, triangles: snapshot.triangles }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: StlSnapshot) {
        self.schema = snapshot.schema;
        self.solid_name = snapshot.solid_name;
        self.triangles = snapshot.triangles;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.stl`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn stl_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.stl",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot. Dissolved out of `⚙️engine`
/// (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — reached as
/// `crate::engine::empty_stl_snapshot` through the `engine` barrel shim.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_stl_snapshot() -> StlSnapshot {
    StlSnapshot::default()
}

/// 📄️ FG1: the demo `stdio.stl` document — a non-degenerate, non-empty `solid_name` plus two
/// distinct-normal triangles, matching the companion real-format fixture assets
/// (`📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio`, both literally this
/// snapshot's `print_dsl`/`encode_pack` output, asserted equal by `conformance_laws::
/// fixture_honesty_law`). Deliberately avoids the empty-`solid_name` degenerate case: the
/// grammar's `LINE` raw-span terminal captures rest-of-physical-line starting at the NEXT real
/// token after `"solid"`/`"endsolid"` — when the name is empty, that next token is on a LATER
/// line (whitespace/newlines are lexer trivia), which would swallow that later line's content as
/// if it were the name. This is a real, narrow edge of the `LINE` primitive itself (shared,
/// framework-level — `📖️grammar/🦀️.rs`'s `match_raw_span`), not a bug in this artifact;
/// every other pilot's own demo/fixture picks similarly avoid degenerate corners their grammar's
/// primitives don't cleanly cover (same "model realistically" convention this ticket's recipe
/// documents throughout, not a `mechanism_gaps` entry of its own since it's a strict subset of the
/// already-documented `protocol-prim-ref-recursion`-adjacent raw-span family).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_stl_snapshot() -> StlSnapshot {
    StlSnapshot {
        schema: STDIO_STL_DOCUMENT_SCHEMA.into(),
        solid_name: "demo".into(),
        triangles: vec![StlTriangle { normal: [0.0, 0.0, 1.0], vertices: [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]] }, StlTriangle { normal: [0.0, 0.0, -1.0], vertices: [[0.0, 0.0, 1.0], [1.0, 0.0, 1.0], [0.0, 1.0, 1.0]] }],
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::schema::snapshot::StlTriangle;
//#endregion 🔁️Re-exports
