//! 🧬️ Ifc2x3Artifact schema — full artifact state for the `2x3` standard (buildingSMART
//! Coordination View 2.0 era, ISO/PAS 16739:2005 schema). Sibling of `🔖️4`'s `IfcArtifact`, own
//! distinct schema id `s.stdio.ifc.2x3` so the two standards' descriptors never collide in the
//! flat `::schema::register_artifact_schema_descriptor` registry.

use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::{Ifc2x3EdmPreamble, Ifc2x3Snapshot};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ifc.2x3")]
pub struct Ifc2x3Artifact {
    #[state(artifact)]
    pub schema: String,
    /// 📦️ The full, lossless generic Part-21 graph, wrapped in this standard's own
    /// [`Ifc2x3Snapshot`] type — the actual persisted state.
    #[state(artifact)]
    #[serde(default)]
    pub document: crate::artifacts::step::engine::part21::Part21Document,
    #[state(artifact)]
    #[serde(default)]
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
    pub fn to_snapshot(&self) -> Ifc2x3Snapshot {
        Ifc2x3Snapshot { schema: self.schema.clone(), document: self.document.clone(), edm_preamble: self.edm_preamble.clone() }
    }

    pub fn from_snapshot(snapshot: Ifc2x3Snapshot) -> Self {
        Self { schema: snapshot.schema, document: snapshot.document, edm_preamble: snapshot.edm_preamble }
    }

    pub fn set_snapshot(&mut self, snapshot: Ifc2x3Snapshot) {
        self.schema = snapshot.schema;
        self.document = snapshot.document;
        self.edm_preamble = snapshot.edm_preamble;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
pub fn ifc2x3_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.ifc.2x3",
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
    use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::diff::Ifc2x3Diff;
    use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::mutations::{apply_ifc2x3_mutation, Ifc2x3Mutation};
    use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct Ifc2x3BuilderConstruction {
        snapshot: Ifc2x3Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for Ifc2x3BuilderConstruction {
        type Snapshot = Ifc2x3Snapshot;
        type Mutation = Ifc2x3Mutation;
        type Diff = Ifc2x3Diff;
        fn empty() -> Self {
            Self { snapshot: Ifc2x3Snapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<Ifc2x3Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Ifc2x3Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_ifc2x3_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <Ifc2x3Diff as protocol::MutationDiff<Ifc2x3Snapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
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
    use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.ifc.2x3` parts.
    #[derive(Clone, Debug, Default)]
    pub struct Ifc2x3Parts {
        pub snapshot: Option<Ifc2x3Snapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Sniff
    /// 🔍️ Real, honest confidence probe: `High` when the text/bytes look like a Part-21 envelope AND
    /// declare `IFC2X3` in `FILE_SCHEMA`; `Medium` for a Part-21 envelope of an unknown schema (could
    /// still decode -- IFC2X3 is layered on the same generic tokenizer); `Low` otherwise.
    fn sniff_text(body: &str) -> IoConfidence {
        let trimmed = body.trim_start();
        if trimmed.starts_with("ISO-10303-21") {
            if trimmed.contains("IFC2X3") {
                IoConfidence::High
            } else {
                IoConfidence::Medium
            }
        } else {
            IoConfidence::Low
        }
    }
    //#endregion 🔖️Sniff

    //#region 🔖️Analyzer
    pub struct Ifc2x3AnalyzerAnalysis;

    impl ArtifactAnalysis for Ifc2x3AnalyzerAnalysis {
        type Parts = Ifc2x3Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Text(text) => {
                    let body = match store::semio_format::split_text_preamble(text) {
                        Ok((_, rest)) => rest,
                        Err(_) => text,
                    };
                    sniff_text(body)
                }
                AnalyzeSource::Binary(bytes) => match std::str::from_utf8(bytes) {
                    Ok(text) => sniff_text(text),
                    Err(_) => IoConfidence::Low,
                },
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Ifc2x3Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match if text.trim_start().starts_with("ISO-10303-21") {
                        crate::artifacts::ifc::standards::v2x3::engine::decode_ifc2x3(text.as_bytes()).map_err(|error| store::TextError::new(error, dsl::TextSpan::at(1, 1)))
                    } else {
                        <Ifc2x3Snapshot as store::ArtifactDsl>::parse_dsl(text)
                    } {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Ifc2x3Snapshot as store::ArtifactPack>::decode_pack(bytes).or_else(|_| crate::artifacts::ifc::standards::v2x3::engine::decode_ifc2x3(bytes).map_err(store::PackError::Schema)) {
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

    //#region 🧪️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn sniff_high_confidence_for_ifc2x3_envelope() {
            let text = "ISO-10303-21;\nHEADER;\nFILE_SCHEMA(('IFC2X3'));\nENDSEC;\nDATA;\nENDSEC;\nEND-ISO-10303-21;\n";
            assert_eq!(Ifc2x3AnalyzerAnalysis::sniff(&AnalyzeSource::Text(text)), IoConfidence::High);
        }

        #[test]
        fn sniff_medium_confidence_for_other_part21_schema() {
            let text = "ISO-10303-21;\nHEADER;\nFILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\nENDSEC;\nEND-ISO-10303-21;\n";
            assert_eq!(Ifc2x3AnalyzerAnalysis::sniff(&AnalyzeSource::Text(text)), IoConfidence::Medium);
        }

        #[test]
        fn sniff_low_confidence_for_non_part21_input() {
            assert_eq!(Ifc2x3AnalyzerAnalysis::sniff(&AnalyzeSource::Text("not a step file at all")), IoConfidence::Low);
            assert_eq!(Ifc2x3AnalyzerAnalysis::sniff(&AnalyzeSource::Binary(&[0xFF, 0xD8, 0xFF])), IoConfidence::Low);
        }
    }
    //#endregion 🧪️Tests
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec Ifc2x3BuilderFacets {
        construction: Ifc2x3BuilderConstruction,
        analysis: Ifc2x3AnalyzerAnalysis,
        composition: super::super::io::derived_composition::Ifc2x3ComposerComposition,
    }
    builder: Ifc2x3Builder,
    analyzer: Ifc2x3Analyzer,
    composer: Ifc2x3Composer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot. Dissolved out of `⚙️engine`
/// (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — reached as
/// `crate::artifacts::ifc::standards::v2x3::engine::empty_ifc2x3_snapshot` through the `engine`
/// barrel shim.
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
pub fn demo_ifc2x3_snapshot() -> Ifc2x3Snapshot {
    use crate::artifacts::step::engine::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};
    let document = Part21Document {
        header: Part21Header {
            file_description: vec![Part21Value::List(vec![]), Part21Value::Str("2;1".into())],
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
    let snapshot = Ifc2x3Snapshot { schema: crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::STDIO_IFC2X3_DOCUMENT_SCHEMA.into(), document, edm_preamble: None };
    snapshot
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🗂️ **Deliberately left imperative and callable** (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-
/// APP-STATE-MACHINES, per the ticket's own explicit instruction: "leave ifc's registration
/// alone" — `ArtifactDeclaration` has exactly one `.schema()`/`.document_codec()` slot and
/// cannot hold both `4`'s and `2x3`'s independent descriptors/codecs at once, see the artifact
/// root `🦀️component.rs`'s own doc comment). Only physically dissolved out of `⚙️engine`; reached
/// as `crate::artifacts::ifc::standards::v2x3::engine::register()` through the `engine` barrel
/// shim, which is exactly the path `📦️glue.rs`'s root `ifc::engine::register()` override calls
/// explicitly (alongside `v4::engine::register()`).
///
/// Registers this standard's schema descriptor, document codec, 5-role `LanguageSpec`s, and (via
/// each real subset's own composer) its `SubsetValidator`s. Does NOT call the artifact-level
/// `ifc::composer::register()` (that union is already invoked once from `4`'s own
/// `engine::register()`, extended by this ticket to also union `v2x3::composer::entries()` —
/// calling it a second time here would be a redundant registration, same reasoning gif's
/// `89a::engine::register` doc comment gives).
pub fn register() {
    ::schema::register_artifact_schema_descriptor(ifc2x3_artifact_schema_descriptor());
    register_artifact_inferences();
    register_pilot_languages();
    store::register_document_codec(store::ArtifactCodec::of::<Ifc2x3Snapshot, crate::artifacts::ifc::standards::v2x3::subsets::any::schema::mutations::Ifc2x3Mutation>(
        crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::STDIO_IFC2X3_DOCUMENT_SCHEMA,
    ));
    // 🛡️ D5's generic validate-on-build hook: registers each real subset's `SubsetValidator` so
    // `io_dispatch`/`wire_artifact_compose` re-check them for free. Each subset's `ComposerEntry`
    // is registered separately via this standard's own `composer::entries()` aggregation.
    crate::artifacts::ifc::standards::v2x3::subsets::cv20::io::register();
    crate::artifacts::ifc::standards::v2x3::subsets::sav::io::register();
    crate::artifacts::ifc::standards::v2x3::subsets::cobie::io::register();
}

/// 💡️ Registers `s.stdio.ifc.2x3.inference`'s facet leaves into the OS-wide inference catalog —
/// sibling to the schema descriptor registration above (separate registry, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
pub fn register_artifact_inferences() {
    ::schema::register_artifact_inference_descriptor(crate::artifacts::ifc::standards::v2x3::subsets::any::schema::inferences::ifc2x3_artifact_inference_descriptor());
}

/// 📌️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: 5-role
/// `LanguageSpec` registration (Document/Ops/Diff/Pack/Spr), per the recipe's json exemplar —
/// `stdio.ifc.2x3`/`.op`/`.diff`/`.pack`/`.spr`, all `dsl::passthrough_hooks`. `diff`'s `protocol`
/// slot stays `None` matching the exemplar's own shape exactly (the 5-role scheme has no dedicated
/// "diff binary" role even though `🔺️diff/💾️binary/📡️component.protocol.semio` is a real,
/// conformance-tested file — its binary form is exercised directly by `protocol_walk_law` below,
/// just not wired through a 6th `LanguageRole`), same precedent `4`'s own
/// `register_pilot_languages` established.
pub fn register_pilot_languages() {
    use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::{diff, mutations, snapshot};
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.ifc.2x3",
        extension: Some("ifc"),
        role: dsl::LanguageRole::Document,
        grammar: Some(snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.ifc.2x3"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.ifc.2x3.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(mutations::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(mutations::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.ifc.2x3.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.ifc.2x3.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(diff::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(diff::text::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("stdio.ifc.2x3.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.ifc.2x3.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.ifc.2x3.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.ifc.2x3.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.ifc.2x3.spr"),
    });
}

// 📌️ `dsl::registry::register_schema_spec` is intentionally NOT called here — `Part21Value` (a
// genuine data-carrying enum) has no `DslField` impl, so no `fn() -> RecordSpec` exists for
// `Ifc2x3Snapshot`/`Ifc2x3Diff` at all (same `register-schema-spec-needs-recordspec` mechanism gap
// `4`'s own `IfcSnapshot`/`IfcDiff` doc comment documents for the isomorphic shape) — filed as a
// `mechanism_gaps` entry rather than fabricating an unrelated spec.
//#endregion 🔖️Register
