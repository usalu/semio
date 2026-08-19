//! 🧬️ IfcArtifact schema — full artifact state. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: this used to duplicate
//! `IfcSnapshot`'s prior worst-offender defect (`document: step::engine::part21::Part21Document`
//! verbatim) — now mirrors `IfcSnapshot`'s own typed `header`/`entities` fields.

use crate::artifacts::ifc::schema::snapshot::{IfcEntity, IfcHeader};
use crate::artifacts::ifc::{IfcMutation, IfcSnapshot, STDIO_IFC_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ifc")]
pub struct IfcArtifact {
    #[state(artifact)]
    pub schema: String,
    /// 📦️ The full, lossless IFC4 graph in IFC's own typed model — the actual persisted state.
    #[state(artifact)]
    #[serde(default)]
    pub header: IfcHeader,
    #[state(artifact)]
    #[serde(default)]
    pub entities: Vec<IfcEntity>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for IfcArtifact {
    async fn default() -> Self {
        Self::from_snapshot(IfcSnapshot::default())
    }
}

impl IfcArtifact {
    pub async fn to_snapshot(&self) -> IfcSnapshot {
        IfcSnapshot { schema: self.schema.clone(), header: self.header.clone(), entities: self.entities.clone() }
    }

    pub async fn from_snapshot(snapshot: IfcSnapshot) -> Self {
        Self { schema: snapshot.schema, header: snapshot.header, entities: snapshot.entities }
    }

    pub async fn set_snapshot(&mut self, snapshot: IfcSnapshot) {
        self.schema = snapshot.schema;
        self.header = snapshot.header;
        self.entities = snapshot.entities;
    }

    /// 🏛️ Derived spatial-structure/placement/pset analyzer view — computed on demand, never
    /// stored; builds the shared generic Part-21 graph on the fly via `to_part21_document`
    /// (the analyzer's own relationship-graph traversal still walks that generic shape).
    pub async fn spatial(&self) -> crate::artifacts::ifc::engine::spatial::SpatialAnalysis {
        let document = crate::artifacts::ifc::schema::snapshot::to_part21_document(&self.to_snapshot());
        crate::artifacts::ifc::engine::spatial::analyze_spatial(&document)
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
pub async fn ifc_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.ifc",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::ifc::{IfcDiff, IfcMutation, IfcSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.ifc` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct IfcBuilderConstruction {
        snapshot: IfcSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for IfcBuilderConstruction {
        type Snapshot = IfcSnapshot;
        type Mutation = IfcMutation;
        type Diff = IfcDiff;
        async fn empty() -> Self {
            Self { snapshot: IfcSnapshot::default(), diagnostics: Vec::new() }
        }
        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<IfcSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<IfcSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::artifacts::ifc::schema::mutations::apply_ifc_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        async fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <IfcDiff as protocol::MutationDiff<IfcSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        async fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
    //#endregion 🔖️Builder
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::ifc::IfcSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.ifc` parts.
    #[derive(Clone, Debug, Default)]
    pub struct IfcParts {
        pub snapshot: Option<IfcSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.ifc` (4/✳️any) sources.
    pub struct IfcAnalyzerAnalysis;

    impl ArtifactAnalysis for IfcAnalyzerAnalysis {
        type Parts = IfcParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("4"), subset: SubsetId("*") };

        async fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = IfcParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <IfcSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <IfcSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec IfcBuilderFacets {
        construction: IfcBuilderConstruction,
        analysis: IfcAnalyzerAnalysis,
        composition: super::super::io::derived_composition::IfcComposerComposition,
    }
    builder: IfcBuilder,
    analyzer: IfcAnalyzer,
    composer: IfcComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot. Dissolved out of `⚙️engine`
/// (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — reached as
/// `crate::artifacts::ifc::standards::v4::engine::empty_ifc_snapshot` through the `engine` barrel
/// shim, and (via the root `crate::artifacts::ifc::engine` shim, glob-imported from v4) as
/// `crate::artifacts::ifc::engine::empty_ifc_snapshot` too.
pub async fn empty_ifc_snapshot() -> IfcSnapshot {
    IfcSnapshot::default()
}

/// 📄️ P2-FG1: the demo `stdio.ifc` document — a real, minimal IFC4 exchange structure (raw HEADER
/// value tuples + three real entities incl. an `IFCOWNERHISTORY` reference chain). The single
/// source of truth for `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio`
/// (both are literally this snapshot's `print_dsl`/`encode_pack` output, asserted equal by
/// `fixture_honesty_law`, now in `../🚪️io/🦀️component.rs`) and for `mutations::
/// demo_mutation_cases()`/`diff::demo_diff_cases()`.
pub async fn demo_ifc_snapshot() -> IfcSnapshot {
    use crate::artifacts::ifc::schema::snapshot::{IfcEntity as _IfcEntity, IfcHeader as _IfcHeader, IfcValue};
    IfcSnapshot {
        schema: STDIO_IFC_DOCUMENT_SCHEMA.into(),
        header: _IfcHeader {
            file_description: vec![IfcValue::Aggregate(vec![]), IfcValue::String("2;1".into())],
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
/// alone" — see the artifact root `🦀️component.rs`'s own doc comment for why `ArtifactDeclaration`
/// structurally cannot hold both `4`'s and `2x3`'s independent descriptors/codecs at once). Only
/// physically dissolved out of `⚙️engine`; reached as `crate::artifacts::ifc::standards::v4::
/// engine::register()` through the `engine` barrel shim below, which is exactly the path
/// `📦️glue.rs`'s root `ifc::engine::register()` override calls explicitly (alongside `v2x3::
/// engine::register()`) — and, since the root shim's `pub use super::standards::v4::engine::*;`
/// glob otherwise re-exports this standard, also the plugin root's own `crate::artifacts::ifc::
/// engine::register()` entry point before that override's `fn register()` shadows it.
///
/// Registers codecs and the artifact schema descriptor.
pub async fn register() {
    crate::artifacts::ifc::io_registry::register();
    register_artifact_schema();
    register_artifact_inferences();
    register_pilot_languages();
    let _ = store::register_document_codec(store::ArtifactCodec::of::<IfcSnapshot, IfcMutation>(STDIO_IFC_DOCUMENT_SCHEMA));
}

/// 📌️ P2-FG1: 5-role `LanguageSpec` registration (Document/Ops/Diff/Pack/Spr), per the recipe's
/// json exemplar — `stdio.ifc`/`.op`/`.diff`/`.pack`/`.spr`, all `dsl::passthrough_hooks`. `diff`'s
/// `protocol` slot stays `None` matching the exemplar's own shape exactly (the 5-role scheme has no
/// dedicated "diff binary" role even though `🔺️diff/💾️binary/📡️component.protocol.semio` is a
/// real, conformance-tested file — its binary form is exercised directly by `protocol_walk_law`
/// below, just not wired through a 6th `LanguageRole`).
pub async fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.ifc",
        extension: Some("ifc"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::ifc::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::ifc::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::ifc::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::ifc::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.ifc"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.ifc.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::ifc::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::ifc::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::ifc::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::ifc::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.ifc.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.ifc.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::ifc::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::ifc::schema::diff::text::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("stdio.ifc.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.ifc.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::ifc::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::ifc::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.ifc.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.ifc.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::ifc::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::ifc::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.ifc.spr"),
    });
}

/// 📌️ P2-FG1: `dsl::registry::register_schema_spec` is intentionally NOT called here — `IfcValue`
/// (a genuine data-carrying enum) has no `DslField` impl, so no `fn() -> RecordSpec` exists for
/// `IfcSnapshot`/`IfcDiff` at all (real `cargo check` confirmed, see `🔺️diff/🦀️component.rs`'s own
/// doc comment) — filed as the `register-schema-spec-needs-recordspec` mechanism gap rather than
/// fabricating an unrelated spec, per the recipe's own instruction.

/// 📌️ Registers schema leaves for `s.stdio.ifc`.
pub async fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(ifc_artifact_schema_descriptor());
}

/// 💡️ Registers `s.stdio.ifc.inference`'s facet leaves into the OS-wide inference catalog —
/// sibling to `register_artifact_schema()` (separate registry, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
pub async fn register_artifact_inferences() {
    ::schema::register_artifact_inference_descriptor(crate::artifacts::ifc::standards::v4::subsets::any::schema::inferences::ifc_artifact_inference_descriptor());
}
//#endregion 🔖️Register
