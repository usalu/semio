//! 🧬️ Ifc2x3Artifact schema — full artifact state for the `2x3` standard (buildingSMART
//! Coordination View 2.0 era, ISO/PAS 16739:2005 schema). Sibling of `4️⃣4`'s `IfcArtifact`, own
//! distinct schema id `s.stdio.ifc.2x3` so the two standards' descriptors never collide in the
//! flat `::semio_framework_schema_registry::register_artifact_schema_descriptor` registry.

use crate::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ifc.2x3")]
pub struct Ifc2x3Artifact {
    #[state(artifact)]
    pub schema: String,
    /// 📦️ The full, lossless generic Part-21 graph, wrapped in this standard's own
    /// [`Ifc2x3Snapshot`] type — the actual persisted state.
    #[state(artifact)]
    #[value(default)]
    pub document: semio_s_artifact_stdio_contract::part21::Part21Document,
    #[state(artifact)]
    #[value(default)]
    pub edm_preamble: Option<Ifc2x3EdmPreamble>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for Ifc2x3Artifact {
    fn default() -> Self {
        Self::from_snapshot(Ifc2x3Snapshot::default())
    }
}

impl Ifc2x3Artifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> Ifc2x3Snapshot {
        Ifc2x3Snapshot { schema: self.schema.clone(), document: self.document.clone(), edm_preamble: self.edm_preamble.clone() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: Ifc2x3Snapshot) -> Self {
        Self { schema: snapshot.schema, document: snapshot.document, edm_preamble: snapshot.edm_preamble }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: Ifc2x3Snapshot) {
        self.schema = snapshot.schema;
        self.document = snapshot.document;
        self.edm_preamble = snapshot.edm_preamble;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn ifc2x3_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.ifc.2x3",
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
/// `crate::standards::v2x3::engine::empty_ifc2x3_snapshot` through the `engine`
/// barrel shim.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_ifc2x3_snapshot() -> Ifc2x3Snapshot {
    Ifc2x3Snapshot::default()
}

/// 📄️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: the demo
/// `stdio.ifc.2x3` document — a real, minimal IFC2X3 exchange structure (raw HEADER value tuples +
/// two real entities incl. an `IFCOWNERHISTORY` reference chain), matching `4`'s own
/// `demo_ifc_snapshot()` shape but declaring `FILE_SCHEMA(('IFC2X3'))` so `decode_ifc2x3`'s own
/// schema gate accepts it. Fodder for `mutations::demo_mutation_cases()`/`diff::demo_diff_cases()`
/// and this standard's own `conformance_laws` tests (a non-empty snapshot, unlike the prior
/// `empty_ifc2x3_snapshot()` stub, so every recognizer/walk law actually exercises real content).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_ifc2x3_snapshot() -> Ifc2x3Snapshot {
    use semio_s_artifact_stdio_contract::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};
    let document = Part21Document {
        header: Part21Header {
            file_description: vec![Part21Value::List(vec![Part21Value::Str(String::new())]), Part21Value::Str("2;1".into())],
            file_name: vec![
                Part21Value::Str("semio.ifc".into()),
                Part21Value::Str("2026-08-11T00:00:00".into()),
                Part21Value::List(vec![Part21Value::Str("Ueli".into())]),
                Part21Value::List(vec![Part21Value::Str("semio".into())]),
                Part21Value::Str("semio".into()),
                Part21Value::Str("".into()),
                Part21Value::Str("".into()),
            ],
            file_schema: vec![Part21Value::List(vec![Part21Value::Str("IFC2X3".into())])],
        },
        instances: vec![
            Part21Instance { id: 1, entities: vec![("IFCPROJECT".into(), vec![Part21Value::Str("gid-project".into()), Part21Value::Ref(2), Part21Value::Str("Demo Project".into())])] },
            Part21Instance { id: 2, entities: vec![("IFCOWNERHISTORY".into(), vec![Part21Value::Unset, Part21Value::Int(0)])] },
        ],
    };

    Ifc2x3Snapshot { schema: crate::standards::v2x3::subsets::base::schema::snapshot::STDIO_IFC2X3_DOCUMENT_SCHEMA.into(), document, edm_preamble: None }
}
//#endregion 🔖️DocumentHelpers



//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3EdmPreamble;
//#endregion 🔁️Re-exports
