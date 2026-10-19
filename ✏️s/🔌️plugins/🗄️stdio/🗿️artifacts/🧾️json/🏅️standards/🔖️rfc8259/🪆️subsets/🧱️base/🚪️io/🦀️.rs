//! 🚪️ IO stdio.json (rfc8259/✳️any) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register().
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v_rfc8259::subsets::base::io::JsonAnalyzer;
    use crate::JsonSnapshot;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct JsonComposerComposition;

    impl ArtifactComposition for JsonComposerComposition {
        type Snapshot = JsonSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_TXT)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "JsonComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = JsonAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "JsonComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️DerivedIoRegistry
/// 🦑 Dissolved out of the former `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — pure `ComposerEntry` aggregation, no
/// engine needed. NOTE: always reach this via a fully-qualified path
/// (`standards::v_rfc8259::subsets::any::io::io_registry::entries()`) — the artifact root's OWN
/// `io_registry` (`🗿️artifacts/🧾️json/🦀️.rs`) shadows this name with a DIFFERENT return
/// type (`&'static [&'static ComposerEntry]` vs this module's `&'static [ComposerEntry]`); a bare
/// `io_registry::entries()` silently rebinds to the wrong one.
pub mod io_registry {
    use crate::standards::v_rfc8259::subsets::base::io::JsonComposer as JsonRawAnyComposer;
    use crate::standards::v_rfc8259::subsets::i_json::io::JsonIJsonComposer;
    use crate::standards::v_rfc8259::subsets::geojson::io::JsonGeoJsonComposer;
    use semio_framework_plugin::{composer_entry_of, io::ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<JsonRawAnyComposer>(), composer_entry_of::<JsonIJsonComposer>(), composer_entry_of::<JsonGeoJsonComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

pub mod text {
    pub use semio_s_artifact_stdio_json_mutations_2::standards::v_rfc8259::subsets::base::io::text::*;
    pub use semio_s_artifact_stdio_json_number::standards::v_rfc8259::subsets::base::io::text::inferences;
}



pub mod derived_construction {
    use crate::{JsonDiff, JsonMutation, JsonSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.json` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct JsonBuilderConstruction {
        snapshot: JsonSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for JsonBuilderConstruction {
        type Snapshot = JsonSnapshot;
        type Mutation = JsonMutation;
        type Diff = JsonDiff;
        fn empty() -> Self {
            Self { snapshot: JsonSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<JsonSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<JsonSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::apply_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
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

pub mod derived_analysis {
    use crate::JsonSnapshot;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.json` parts.
    #[derive(Clone, Debug, Default)]
    pub struct JsonParts {
        pub snapshot: Option<JsonSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.json` (rfc8259/✳️any) sources.
    pub struct JsonAnalyzerAnalysis;

    impl ArtifactAnalysis for JsonAnalyzerAnalysis {
        type Parts = JsonParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            match source {
                AnalyzeSource::Text(text) => if <JsonSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).is_ok() { semio_framework_plugin::io::Confidence::High } else { semio_framework_plugin::io::Confidence::Low },
                AnalyzeSource::Binary(bytes) => if <JsonSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).is_ok() { semio_framework_plugin::io::Confidence::High } else { semio_framework_plugin::io::Confidence::Low },
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = JsonParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <JsonSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <JsonSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
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
    include!("🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
    //#endregion 🧪️Tests
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec JsonBuilderFacets {
        construction: JsonBuilderConstruction,
        analysis: JsonAnalyzerAnalysis,
        composition: JsonComposerComposition,
    }
    builder: JsonBuilder,
    analyzer: JsonAnalyzer,
    composer: JsonComposer,
);

//#region 🧩️ArtifactItems3
pub mod artifact_items_3 {
/// 📜 Schema-owned package definition.
pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("../../../../../📜️artifact-definition.json");
}
//#endregion 🧩️ArtifactItems3

//#region 🧩️ArtifactItems4
pub mod artifact_items_4 {
use crate::{STDIO_JSON_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "s.stdio.json".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Json", "Json"),
        source_format: STDIO_JSON_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_JSON_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
}
//#endregion 🧩️ArtifactItems4

//#region 🧩️ArtifactItems5
pub mod artifact_items_5 {
use crate::{STDIO_JSON_DOCUMENT_SCHEMA};
pub use crate::schema::mutations::JsonMutation;
pub use crate::schema::snapshot::JsonSnapshot;

pub fn native_codec() -> store::os_store::serialization::ArtifactCodec {
    let mut codec = store::artifact_codec::<JsonSnapshot, JsonMutation>(STDIO_JSON_DOCUMENT_SCHEMA);
    codec.extension = "json";
    codec
}
}
//#endregion 🧩️ArtifactItems5

//#region 🧩️ArtifactItems6
pub mod artifact_items_6 {
use crate::{artifact_kind};
use crate::{native_codec};

pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    vec![semio_s_artifact_stdio_contract::NativeCodecFactory { id: "stdio.native.json.v1", artifact: "json", kind: artifact_kind, codec: native_codec }]
}
}
//#endregion 🧩️ArtifactItems6

//#region 🧩️ArtifactItems7
pub mod artifact_items_7 {
use crate::{native_codecs};
use crate::{ARTIFACT_DEFINITION_SCHEMA};

pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::PluginAssemblyError> {
    let factories = native_codecs();
    let executables = semio_s_artifact_stdio_contract::native_codec_executables(ARTIFACT_DEFINITION_SCHEMA, &factories)?;
    semio_s_artifact_stdio_contract::definition_from_schema_with_executables(ARTIFACT_DEFINITION_SCHEMA, executables)
}
}
//#endregion 🧩️ArtifactItems7

//#region 🧩️ArtifactItems8
pub mod artifact_items_8 {
use crate::{ARTIFACT_DEFINITION_SCHEMA};

pub fn formats() -> Result<Vec<semio_framework_plugin::io::FormatDescriptor>, semio_framework_plugin::ArtifactDefinitionError> {
    semio_s_artifact_stdio_contract::format_descriptors(ARTIFACT_DEFINITION_SCHEMA)
}
}
//#endregion 🧩️ArtifactItems8

//#region 🧩️ArtifactItems9
pub mod artifact_items_9 {
use crate::{definition};
use crate::{formats};
use crate::{STDIO_JSON_DOCUMENT_SCHEMA};
pub use crate::schema::mutations::JsonMutation;
pub use crate::schema::snapshot::JsonSnapshot;

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6) — replaces
/// stdio's plugin root calling an imperative `register()` before `Plugin::builder` was even
/// constructed, mirroring the `🔋️energy`/`🗒️note` exemplars. Call order, in `.builder()` order below:
/// `.composers(...)` from `standards::v_rfc8259::subsets::any::io::io_registry::entries()` (dissolved
/// out of the former `⚙️engine`, ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES — that
/// list already carries BOTH the `✳️any` raw composer and the `🛜️i-json` composer, so `.composers()`
/// alone covers both); `.schema(...)`/`.inferences(...)` from the schema/inferences descriptors
/// directly; `.languages(...)` from `pilot_languages()` below (same `OnceLock`-leak shape `🔋️energy`'s
/// own `pilot_languages()` uses, since `dsl::LanguageSpec` isn't `const fn`-constructible);
/// `.document_codec_bare::<JsonSnapshot, JsonMutation>(...)`; and
/// `crate::standards::v_rfc8259::subsets::i_json::io::register()` — the 🛜️i-json
/// subset's own `register_subset_validator` call, living in `🚪️io/` → `.subset_validators(...)`
/// below, built fresh via `subset_validator_entry_of::<JsonIJsonValidator>()` rather than reaching
/// into that module's private `validator_entry()` OnceLock (left untouched — first artifact in the
/// repo to populate this field, see its own builder doc).
/// 🧩️ Binds this executable root to its sole schema-owned definition.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("json", definition()?, declaration)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = formats()?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::schema::json_artifact_schema_descriptor())
        .formats(formats)
        .inferences([crate::standards::v_rfc8259::subsets::base::schema::inferences::json_artifact_inference_descriptor()])
        .composers(crate::standards::v_rfc8259::subsets::base::io::io_registry::entries())
        .subset_validators(pilot_subset_validators())
        .languages(pilot_languages())
        .document_codec_bare::<JsonSnapshot, JsonMutation>(STDIO_JSON_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.json", standard: semio_framework_artifact_reference::StandardId("rfc8259"), subset: semio_framework_artifact_reference::SubsetId("*") })
        .document_codec_bare::<JsonSnapshot, JsonMutation>(STDIO_JSON_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.json", standard: semio_framework_artifact_reference::StandardId("rfc8259"), subset: semio_framework_artifact_reference::SubsetId("i-json") })
        .document_codec_bare::<JsonSnapshot, JsonMutation>(STDIO_JSON_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.json", standard: semio_framework_artifact_reference::StandardId("rfc8259"), subset: semio_framework_artifact_reference::SubsetId("geojson") })
        .try_build()
}

/// 🛡️ The 🛜️i-json and 🌍️geojson subsets' `SubsetValidatorEntry`s, built once — see `declaration()`'s own doc for
/// why this is a fresh `subset_validator_entry_of::<JsonIJsonValidator>()` call rather than a reuse
/// of `subsets::i_json::io::derived_composition`'s private `validator_entry()`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn pilot_subset_validators() -> &'static [semio_framework_plugin::io::SubsetValidatorEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<semio_framework_plugin::io::SubsetValidatorEntry>> = std::sync::OnceLock::new();
    ENTRIES.get_or_init(|| vec![semio_framework_plugin::io::subset_validator_entry_of::<crate::standards::v_rfc8259::subsets::i_json::io::JsonIJsonValidator>(), semio_framework_plugin::io::subset_validator_entry_of::<crate::standards::v_rfc8259::subsets::geojson::io::JsonGeoJsonValidator>()]).as_slice()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `🔋️energy` exemplar's helper of the same shape. Verbatim copy of
/// `standards::v_rfc8259::subsets::any::engine::register_pilot_languages()`'s five `LanguageSpec`s.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.json",
                    extension: Some("json"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(crate::standards::v_rfc8259::subsets::base::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v_rfc8259::subsets::base::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v_rfc8259::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v_rfc8259::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.json"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.json.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(crate::standards::v_rfc8259::subsets::base::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v_rfc8259::subsets::base::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v_rfc8259::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v_rfc8259::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.json.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.json.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(crate::standards::v_rfc8259::subsets::base::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v_rfc8259::subsets::base::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.json.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.json.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v_rfc8259::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v_rfc8259::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.json.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.json.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v_rfc8259::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v_rfc8259::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.json.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration
}
//#endregion 🧩️ArtifactItems9

//#region 🧩️ArtifactItems10
pub mod artifact_items_10 {
use crate::{ARTIFACT_DEFINITION_SCHEMA};
use crate::{definition};
use crate::{assembly};
use crate::{formats};
use crate::{native_codecs};

pub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {
    semio_s_artifact_stdio_contract::ArtifactContribution { definition_constraint: None, identity: "json", schema: ARTIFACT_DEFINITION_SCHEMA, definition, assembly, formats, native_codecs }
}
}
//#endregion 🧩️ArtifactItems10
