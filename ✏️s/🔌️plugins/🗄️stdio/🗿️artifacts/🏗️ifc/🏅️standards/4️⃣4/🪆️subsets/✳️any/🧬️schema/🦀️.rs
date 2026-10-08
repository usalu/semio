//! 🧬️ IfcArtifact schema — full artifact state. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: this used to duplicate
//! `IfcSnapshot`'s prior worst-offender defect (`document: semio_s_artifact_stdio_contract::part21::Part21Document`
//! verbatim) — now mirrors `IfcSnapshot`'s own typed `header`/`entities` fields.

use crate::schema::snapshot::{IfcEntity, IfcHeader};
use crate::{IfcMutation, IfcSnapshot, STDIO_IFC_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ifc")]
pub struct IfcArtifact {
    #[state(artifact)]
    pub schema: String,
    /// 📦️ The full, lossless IFC4 graph in IFC's own typed model — the actual persisted state.
    #[state(artifact)]
    #[value(default)]
    pub header: IfcHeader,
    #[state(artifact)]
    #[value(default)]
    pub entities: Vec<IfcEntity>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for IfcArtifact {
    fn default() -> Self {
        Self::from_snapshot(IfcSnapshot::default())
    }
}

impl IfcArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> IfcSnapshot {
        IfcSnapshot { schema: self.schema.clone(), header: self.header.clone(), entities: self.entities.clone() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: IfcSnapshot) -> Self {
        Self { schema: snapshot.schema, header: snapshot.header, entities: snapshot.entities }
    }

    /// 🏛️ Derived spatial-structure/placement/pset analyzer view — computed on demand, never
    /// stored; builds the shared generic Part-21 graph on the fly via `to_part21_document`
    /// (the analyzer's own relationship-graph traversal still walks that generic shape).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn spatial(&self) -> crate::engine::spatial::SpatialAnalysis {
        let document = crate::schema::snapshot::to_part21_document(&self.to_snapshot());
        crate::engine::spatial::analyze_spatial(&document)
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn ifc_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.ifc",
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
/// `crate::standards::v4::engine::empty_ifc_snapshot` through the `engine` barrel
/// shim, and (via the root `crate::engine` shim, glob-imported from v4) as
/// `crate::engine::empty_ifc_snapshot` too.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_ifc_snapshot() -> IfcSnapshot {
    IfcSnapshot::default()
}

/// 📄️ P2-FG1: the demo `stdio.ifc` document — a real, minimal IFC4 exchange structure (raw HEADER
/// value tuples + three real entities incl. an `IFCOWNERHISTORY` reference chain). The single
/// source of truth for `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio`
/// (both are literally this snapshot's `print_dsl`/`encode_pack` output, asserted equal by
/// `fixture_honesty_law`, now in `../🚪️io/🦀️.rs`) and for `mutations::
/// demo_mutation_cases()`/`diff::demo_diff_cases()`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_ifc_snapshot() -> IfcSnapshot {
    use crate::schema::snapshot::{IfcEntity as _IfcEntity, IfcHeader as _IfcHeader, IfcValue};
    IfcSnapshot {
        schema: STDIO_IFC_DOCUMENT_SCHEMA.into(),
        header: _IfcHeader {
            file_description: vec![IfcValue::Aggregate(vec![IfcValue::String(String::new())]), IfcValue::String("2;1".into())],
            file_name: vec![
                IfcValue::String("semio.ifc".into()),
                IfcValue::String("2026-08-11T00:00:00".into()),
                IfcValue::Aggregate(vec![IfcValue::String("Ueli".into())]),
                IfcValue::Aggregate(vec![IfcValue::String("semio".into())]),
                IfcValue::String("semio".into()),
                IfcValue::String("".into()),
                IfcValue::String("".into()),
            ],
            file_schema: vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4".into())])],
        },
        entities: vec![
            _IfcEntity { id: 1, name: "IFCPROJECT".into(), args: vec![IfcValue::String("gid-project".into()), IfcValue::Reference(2), IfcValue::String("Demo Project".into())], complex: Vec::new() },
            _IfcEntity { id: 2, name: "IFCOWNERHISTORY".into(), args: vec![IfcValue::Unset, IfcValue::Integer(0)], complex: Vec::new() },
        ],
    }
}
//#endregion 🔖️DocumentHelpers


