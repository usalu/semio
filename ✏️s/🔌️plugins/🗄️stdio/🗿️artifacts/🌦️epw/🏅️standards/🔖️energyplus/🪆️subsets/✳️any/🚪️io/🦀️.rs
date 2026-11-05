//! 🚪️ IO — real EnergyPlus Weather (EPW) text codec (LOSSLESS: all 8 header lines + all 35
//! per-record columns, https://bigladdersoftware.com/epx/docs/9-6/auxiliary-programs/energyplus-
//! weather-file-epw-data-dictionary.html — see `…/schema/snapshot` module doc for the full
//! rationale) plus composition/registration. 🦑 Codec + registration dissolved out of the former
//! `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
#[cfg(test)] use crate::apply_mutation;

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::energyplus::subsets::any::schema::snapshot::EpwSnapshot;
    use crate::standards::energyplus::subsets::any::io::EpwAnalyzer;
    use {semio_plugin::io::AnalyzeSource,semio_plugin::ArtifactComposition,semio_plugin::io::ComposeError,semio_plugin::io::ComposeSource,semio_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.epw", standard: StandardId("energyplus"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct EpwComposerComposition;

    impl ArtifactComposition for EpwComposerComposition {
        type Snapshot = EpwSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "EpwComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = EpwAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "EpwComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, inferences, document codec. Called from
    /// this artifact's root-level `register()` (former standard-level `engine::register()`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        ::semio_framework_schema_registry::register_artifact_schema_descriptor(crate::standards::energyplus::subsets::any::schema::epw_artifact_schema_descriptor()).expect("schema descriptor publication");
        register_artifact_inferences();
        semio_plugin::io::register_native_document_codec(Dialect { artifact_kind: "s.stdio.epw", standard: StandardId("energyplus"), subset: SubsetId("*") }, store::bare_artifact_codec::<EpwSnapshot, crate::standards::energyplus::subsets::any::schema::mutations::EpwMutation>(crate::standards::energyplus::subsets::any::schema::snapshot::STDIO_EPW_DOCUMENT_SCHEMA))
            .expect("static Stdio registration must be available and conflict-free");
    }

    /// 💡️ Registers `s.stdio.epw.inference`'s facet leaves into the OS-wide inference catalog —
    /// sibling to the artifact schema descriptor above (separate registry, ticket
    /// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::standards::energyplus::subsets::any::schema::inferences::epw_artifact_inference_descriptor()).expect("schema descriptor publication");
    }
    //#endregion 🔖️Register
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

pub use semio_s_artifact_stdio_epw_snapshot::standards::energyplus::subsets::any::io::natural::{decode_epw, encode_epw, parse_location_line, parse_record_line, sniff_real_bytes};

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::energyplus::subsets::any::io::EpwComposer as EpwRawAnyComposer;
    use semio_plugin::{composer_entry_of, io::ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<EpwRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

pub use semio_s_artifact_stdio_epw_editor::standards::energyplus::subsets::any::io::binary;

pub use semio_s_artifact_stdio_epw_editor::standards::energyplus::subsets::any::io::text;

pub use semio_s_artifact_stdio_epw_schema::standards::energyplus::subsets::any::io::sqlite;

pub mod derived_construction {
    use crate::standards::energyplus::subsets::any::schema::diff::EpwDiff;
    use crate::standards::energyplus::subsets::any::schema::mutations::{EpwMutation};

    use crate::standards::energyplus::subsets::any::schema::snapshot::EpwSnapshot;
    use semio_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct EpwBuilderConstruction {
        snapshot: EpwSnapshot,
    }

    impl ArtifactBuilder for EpwBuilderConstruction {
        type Snapshot = EpwSnapshot;
        type Mutation = EpwMutation;
        type Diff = EpwDiff;
        fn empty() -> Self {
            Self { snapshot: EpwSnapshot::default() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<EpwSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<EpwSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::apply_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = protocol::apply_diff_cold(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            Ok(self.snapshot)
        }
    }
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::standards::energyplus::subsets::any::io;
    use crate::standards::energyplus::subsets::any::schema::snapshot::{EpwSnapshot, STDIO_EPW_DOCUMENT_SCHEMA};
    use {semio_plugin::io::Analysis,semio_plugin::io::AnalyzeSource,semio_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct EpwParts {
        pub snapshot: Option<EpwSnapshot>,
    }

    pub struct EpwAnalyzerAnalysis;

    impl ArtifactAnalysis for EpwAnalyzerAnalysis {
        type Parts = EpwParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.epw", standard: StandardId("energyplus"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> semio_plugin::io::Confidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if io::sniff_real_bytes(bytes) {
                        return semio_plugin::io::Confidence::High;
                    }
                    let marker = STDIO_EPW_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        semio_plugin::io::Confidence::High
                    } else {
                        semio_plugin::io::Confidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if io::sniff_real_bytes(text.as_bytes()) || text.contains(STDIO_EPW_DOCUMENT_SCHEMA) {
                        semio_plugin::io::Confidence::High
                    } else {
                        semio_plugin::io::Confidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = EpwParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <EpwSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <EpwSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;

semio_plugin::derive_artifact_facets!(
    pub spec EpwBuilderFacets {
        construction: EpwBuilderConstruction,
        analysis: EpwAnalyzerAnalysis,
        composition: EpwComposerComposition,
    }
    builder: EpwBuilder,
    analyzer: EpwAnalyzer,
    composer: EpwComposer,
);

//#region 🧩️ArtifactItems3
pub mod artifact_items_3 {
/// 📜 Schema-owned package definition.
pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("../../../../../📜️artifact-definition.json");
}
//#endregion 🧩️ArtifactItems3

//#region 🧩️ArtifactItems4
pub mod artifact_items_4 {
/// 📜 EPW-owned MIME policy for every authored artifact definition.
pub const ARTIFACT_DEFINITION_CONSTRAINT: &str = include_str!("../../../../../🧬️schema/🔣️.json");
}
//#endregion 🧩️ArtifactItems4

//#region 🧩️ArtifactItems5
pub mod artifact_items_5 {
use crate::{ARTIFACT_DEFINITION_CONSTRAINT};

/// 🌦️ Validates EPW ownership and its unregistered MIME policy.
pub fn validate_definition_schema(schema: &str) -> Result<(), semio_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::validate_definition_constraint(schema, ARTIFACT_DEFINITION_CONSTRAINT)
}
}
//#endregion 🧩️ArtifactItems5

//#region 🧩️ArtifactItems6
pub mod artifact_items_6 {
use crate::{validate_definition_schema};
use crate::{ARTIFACT_DEFINITION_SCHEMA};

pub fn definition() -> Result<semio_plugin::ArtifactDefinition, semio_plugin::PluginAssemblyError> {
    validate_definition_schema(ARTIFACT_DEFINITION_SCHEMA)?;
    semio_s_artifact_stdio_contract::definition_from_schema(ARTIFACT_DEFINITION_SCHEMA)
}
}
//#endregion 🧩️ArtifactItems6

//#region 🧩️ArtifactItems7
pub mod artifact_items_7 {
use crate::{validate_definition_schema};
use crate::{ARTIFACT_DEFINITION_SCHEMA};

pub fn formats() -> Result<Vec<semio_plugin::io::FormatDescriptor>, semio_plugin::ArtifactDefinitionError> {
    validate_definition_schema(ARTIFACT_DEFINITION_SCHEMA).map_err(|error| semio_plugin::ArtifactDefinitionError::new("epw.definition", error.to_string()))?;
    semio_s_artifact_stdio_contract::format_descriptors(ARTIFACT_DEFINITION_SCHEMA)
}
}
//#endregion 🧩️ArtifactItems7

//#region 🧩️ArtifactItems9
pub mod artifact_items_9 {
use crate::{definition};
use crate::{formats};
use crate::{STDIO_EPW_DOCUMENT_SCHEMA};
use semio_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
pub use crate::standards::energyplus::subsets::any::schema::mutations::EpwMutation;
pub use crate::standards::energyplus::subsets::any::schema::snapshot::EpwSnapshot;

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("epw", definition()?, declaration)
}

/// 🧾️ The runtime `epw` declares: its schema, format, inference descriptor, composers and document codec.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_plugin::ArtifactDefinition) -> Result<semio_plugin::ArtifactDeclaration, semio_plugin::ArtifactDefinitionError> {
    let formats = formats()?;
    semio_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::standards::energyplus::subsets::any::schema::epw_artifact_schema_descriptor())
        .formats(formats)
        .inferences([crate::standards::energyplus::subsets::any::schema::inferences::epw_artifact_inference_descriptor()])
        .composers(crate::standards::energyplus::subsets::any::io::io_registry::entries())
        .document_codec_bare::<EpwSnapshot, EpwMutation>(
            STDIO_EPW_DOCUMENT_SCHEMA,
            semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.epw", standard: semio_framework_artifact_reference::StandardId("energyplus"), subset: semio_framework_artifact_reference::SubsetId("*") },
        )
        .try_build()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "s.stdio.epw".into(),
        label: semio_ui_locale::LocalizedLabel::native("Epw", "Epw"),
        source_format: STDIO_EPW_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_EPW_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
}
//#endregion 🧩️ArtifactItems9

//#region 🧩️ArtifactItems10
pub mod artifact_items_10 {
use crate::{ARTIFACT_DEFINITION_CONSTRAINT};
use crate::{ARTIFACT_DEFINITION_SCHEMA};
use crate::{definition};
use crate::{assembly};
use crate::{formats};
use crate::{native_codecs};

pub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {
    semio_s_artifact_stdio_contract::ArtifactContribution { definition_constraint: Some(ARTIFACT_DEFINITION_CONSTRAINT), identity: "epw", schema: ARTIFACT_DEFINITION_SCHEMA, definition, assembly, formats, native_codecs }
}
}
//#endregion 🧩️ArtifactItems10

//#region 🧩️ArtifactItems11
pub mod artifact_items_11 {
//#region 🔖️Register
/// 🗂️ Registers this artifact's IO and its handcrafted grammar/protocol `LanguageSpec` imperatively, outside any plugin
/// assembly — the twin of [`declaration`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {
    crate::standards::energyplus::subsets::any::io::register();
    register_pilot_languages();
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_pilot_languages() {
    use crate::standards::energyplus::subsets::any::io::{text,binary};
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.epw",
        extension: Some("epw"),
        role: semio_framework_dsl::LanguageRole::Document,
        grammar: Some(text::snapshot::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(text::snapshot::COMPONENT_GRAMMAR_PATH),
        protocol: Some(binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(binary::snapshot::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.epw"),
    });
}
//#endregion 🔖️Register
}
//#endregion 🧩️ArtifactItems11
