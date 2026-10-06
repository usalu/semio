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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: IfcSnapshot) {
        self.schema = snapshot.schema;
        self.header = snapshot.header;
        self.entities = snapshot.entities;
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

//#region 🔖️Register
/// 🗂️ **Deliberately left imperative and callable** (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-
/// APP-STATE-MACHINES, per the ticket's own explicit instruction: "leave ifc's registration
/// alone" — see the artifact root `🦀️.rs`'s own doc comment for why `ArtifactDeclaration`
/// structurally cannot hold both `4`'s and `2x3`'s independent descriptors/codecs at once). Only
/// physically dissolved out of `⚙️engine`; reached as `crate::standards::v4::
/// engine::register()` through the `engine` barrel shim below, which is exactly the path
/// `🦀️.rs`'s root `ifc::engine::register()` override calls explicitly (alongside `v2x3::
/// engine::register()`) — and, since the root shim's `pub use super::standards::v4::engine::*;`
/// glob otherwise re-exports this standard, also the plugin root's own `crate::
/// engine::register()` entry point before that override's `fn register()` shadows it.
///
/// Registers codecs and the artifact schema descriptor.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {
    crate::io_registry::register();
    register_artifact_schema();
    register_artifact_inferences();
    register_pilot_languages();
    semio_framework_plugin::io::register_native_document_codec(semio_framework_plugin::Dialect { artifact_kind: "s.stdio.ifc", standard: semio_framework_plugin::StandardId("4"), subset: semio_framework_plugin::SubsetId("*") }, store::ArtifactCodec::bare::<IfcSnapshot, IfcMutation>(STDIO_IFC_DOCUMENT_SCHEMA)).expect("static Stdio registration must be available and conflict-free");
}

/// 📌️ P2-FG1: 5-role `LanguageSpec` registration (Document/Ops/Diff/Pack/Spr), per the recipe's
/// json exemplar — `stdio.ifc`/`.op`/`.diff`/`.pack`/`.spr`, all `dsl::passthrough_hooks`. `diff`'s
/// `protocol` slot stays `None` matching the exemplar's own shape exactly (the 5-role scheme has no
/// dedicated "diff binary" role even though `🔺️diff/💾️binary/📡️.protocol.semio` is a
/// real, conformance-tested file — its binary form is exercised directly by `protocol_walk_law`
/// below, just not wired through a 6th `LanguageRole`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_pilot_languages() {
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.ifc",
        extension: Some("ifc"),
        role: semio_framework_dsl::LanguageRole::Document,
        grammar: Some(crate::standards::v4::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::standards::v4::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::standards::v4::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::standards::v4::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.ifc"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.ifc.op",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Ops,
        grammar: Some(crate::standards::v4::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::standards::v4::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::standards::v4::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::standards::v4::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.ifc.op"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.ifc.diff",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Diff,
        grammar: Some(crate::standards::v4::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::standards::v4::subsets::any::io::text::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: semio_framework_dsl::passthrough_hooks("stdio.ifc.diff"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.ifc.pack",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::standards::v4::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::standards::v4::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.ifc.pack"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.ifc.spr",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::standards::v4::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::standards::v4::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.ifc.spr"),
    });
}

/// 📌️ P2-FG1: `dsl::registry::register_schema_spec` is intentionally NOT called here — `IfcValue`
/// (a genuine data-carrying enum) has no `DslField` impl, so no `fn() -> RecordSpec` exists for
/// `IfcSnapshot`/`IfcDiff` at all (real `cargo check` confirmed, see `🔺️diff/🦀️.rs`'s own
/// doc comment) — filed as the `register-schema-spec-needs-recordspec` mechanism gap rather than
/// fabricating an unrelated spec, per the recipe's own instruction.
/// 📌️ Registers schema leaves for `s.stdio.ifc`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_artifact_schema() {
    ::semio_framework_schema_registry::register_artifact_schema_descriptor(ifc_artifact_schema_descriptor()).expect("schema descriptor publication");
}

/// 💡️ Registers `s.stdio.ifc.inference`'s facet leaves into the OS-wide inference catalog —
/// sibling to `register_artifact_schema()` (separate registry, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_artifact_inferences() {
    ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::standards::v4::subsets::any::schema::inferences::ifc_artifact_inference_descriptor()).expect("schema descriptor publication");
}
//#endregion 🔖️Register
