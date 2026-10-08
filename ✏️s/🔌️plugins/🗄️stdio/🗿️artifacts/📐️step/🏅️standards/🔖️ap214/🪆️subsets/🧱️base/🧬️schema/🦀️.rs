//! 🧬️ StepArtifact schema — full artifact state, mirrors `StepSnapshot`'s own typed HEADER +
//! id-keyed entity graph (never a raw `Part21Document` — same specific-code mandate as the
//! snapshot itself).

use crate::schema::snapshot::{StepEntity, StepHeader};
use crate::{StepSnapshot, STDIO_STEP_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.step")]
pub struct StepArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub header: StepHeader,
    #[state(artifact)]
    #[value(default)]
    pub entities: Vec<StepEntity>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for StepArtifact {
    fn default() -> Self {
        Self::from_snapshot(StepSnapshot::default())
    }
}

impl StepArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> StepSnapshot {
        StepSnapshot { schema: self.schema.clone(), header: self.header.clone(), entities: self.entities.clone() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: StepSnapshot) -> Self {
        Self { schema: snapshot.schema, header: snapshot.header, entities: snapshot.entities }
    }

    /// 🧐️ Derived BrepMesh analyzer view — computed on demand from the typed entity graph via
    /// `StepSnapshot::to_part21_document`, never stored.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn brep_mesh(&self) -> crate::engine::brep::BrepMeshView {
        crate::engine::brep::analyze_brep_mesh(&self.to_snapshot().to_part21_document())
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn step_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.step",
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
/// `crate::engine::empty_step_snapshot` through the `engine` barrel shim.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_step_snapshot() -> StepSnapshot {
    StepSnapshot::default()
}

/// 📄️ P2-FG1: the demo `stdio.step` document — a real, minimal AP214 exchange structure (typed
/// HEADER triple + two `CARTESIAN_POINT` entities). The single source of truth for
/// `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio` (both are literally
/// this snapshot's `print_dsl`/`encode_pack` output, asserted equal by `fixture_honesty_law`) and
/// for `mutations::demo_mutation_cases()`/`diff::demo_diff_cases()`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_step_snapshot() -> StepSnapshot {
    use crate::schema::snapshot::{StepEntity as _StepEntity, StepFileDescription, StepFileName, StepFileSchema, StepHeader as _StepHeader, StepValue};
    StepSnapshot {
        schema: STDIO_STEP_DOCUMENT_SCHEMA.into(),
        header: _StepHeader {
            file_description: StepFileDescription { description: vec!["".into()], implementation_level: "2;1".into() },
            file_name: StepFileName {
                name: "semio.step".into(),
                timestamp: "2026-08-11T00:00:00".into(),
                author: vec!["Ueli".into()],
                organization: vec!["semio".into()],
                preprocessor_version: "semio".into(),
                originating_system: "".into(),
                authorization: "".into(),
            },
            file_schema: StepFileSchema { schemas: vec!["AUTOMOTIVE_DESIGN".into()] },
        },
        entities: vec![
            _StepEntity { id: 1, name: "CARTESIAN_POINT".into(), args: vec![StepValue::String("".into()), StepValue::Aggregate(vec![StepValue::Real(0.0), StepValue::Real(0.0), StepValue::Real(0.0)])], complex: Vec::new() },
            _StepEntity { id: 2, name: "CARTESIAN_POINT".into(), args: vec![StepValue::String("".into()), StepValue::Aggregate(vec![StepValue::Real(10.0), StepValue::Real(0.0), StepValue::Real(0.0)])], complex: Vec::new() },
        ],
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
