//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use semio_s_artifact_stdio_semio_animation::io::SemioAnimationValidator;
    use semio_s_artifact_stdio_semio_audio::io::SemioAudioValidator;
    use crate::schema::snapshot::{SemioSnapshot, SemioSubsetSnapshot};
    use crate::io::SemioAnalyzer;
    use semio_s_artifact_stdio_semio_brep::io::SemioBrepValidator;
    use semio_s_artifact_stdio_semio_cad::io::SemioCadValidator;
    use semio_s_artifact_stdio_semio_document::io::SemioDocumentValidator;
    use semio_s_artifact_stdio_semio_drawing::io::SemioDrawingValidator;
    use semio_s_artifact_stdio_semio_flow::io::SemioFlowValidator;
    use semio_s_artifact_stdio_semio_graph::io::SemioGraphValidator;
    use semio_s_artifact_stdio_semio_image::io::SemioImageValidator;
    use semio_s_artifact_stdio_semio_kit::io::SemioKitValidator;
    use semio_s_artifact_stdio_semio_mesh::io::SemioMeshValidator;
    use semio_s_artifact_stdio_semio_model::io::SemioModelValidator;
    use semio_s_artifact_stdio_semio_object::io::SemioObjectValidator;
    use semio_s_artifact_stdio_semio_presentation::io::SemioPresentationValidator;
    use semio_s_artifact_stdio_semio_table::io::SemioTableValidator;
    use semio_s_artifact_stdio_semio_text::io::SemioTextValidator;
    use semio_s_artifact_stdio_semio_value::io::SemioValueValidator;
    use semio_s_artifact_stdio_semio_video::io::SemioVideoValidator;
    use semio_framework_diagnostic::Diagnostic;
    use {semio_framework_plugin::io::register_subset_validator,semio_framework_plugin::io::subset_validator_entry_of,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_plugin::io::SubsetValidator,semio_framework_plugin::io::SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct SemioComposerComposition;

    impl ArtifactComposition for SemioComposerComposition {
        type Snapshot = SemioSnapshot;
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
                return Err(ComposeError { message: "SemioComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "SemioComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ The envelope's own `SubsetValidator` for the `"*"` dialect (D5's generic
    /// validate-on-build hook — required by policy, same as every one of the 13 subsets' own
    /// validators, `pdf`'s `✳️a` composer is the copy template). Decodes the payload as a
    /// `SemioSnapshot`, then DELEGATES to whichever one of the 13 subsets' OWN, already-real
    /// `SubsetValidator`s matches the decoded snapshot's active kind — this validator owns zero
    /// invariant logic itself, only the envelope-level decode + dispatch, exactly mirroring how
    /// `SemioDiff`/`SemioMutation` themselves only own routing, never re-derived per-subset rules.
    pub struct SemioValidator;

    /// 🔎️ Real dispatch: re-encodes the decoded inner snapshot through ITS OWN subset's
    /// `ArtifactPack`, then calls that subset's own registered `SubsetValidator::validate` — genuine
    /// reuse of all 13 already-tested invariant checks, never duplicated here.
    async fn dispatch_validate(snapshot: &SemioSnapshot) -> Vec<Diagnostic> {
        match &snapshot.subset {
            SemioSubsetSnapshot::Brep(s) => SemioBrepValidator::validate(&IoPayload::Binary(<semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s))).await,
            SemioSubsetSnapshot::Mesh(s) => SemioMeshValidator::validate(&IoPayload::Binary(<semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMeshSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s))).await,
            SemioSubsetSnapshot::Model(s) => SemioModelValidator::validate(&IoPayload::Binary(<semio_s_artifact_stdio_semio_model::schema::snapshot::SemioModelSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s))).await,
            SemioSubsetSnapshot::Value(s) => SemioValueValidator::validate(&IoPayload::Binary(<semio_s_artifact_stdio_semio_value::schema::snapshot::SemioValueSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s))).await,
            SemioSubsetSnapshot::Document(s) => SemioDocumentValidator::validate(&IoPayload::Binary(<semio_s_artifact_stdio_semio_document::schema::snapshot::SemioDocumentSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s))).await,
            SemioSubsetSnapshot::Cad(s) => SemioCadValidator::validate(&IoPayload::Binary(<semio_s_artifact_stdio_semio_cad::schema::snapshot::SemioCadSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s))).await,
            SemioSubsetSnapshot::Drawing(s) => SemioDrawingValidator::validate(&IoPayload::Binary(<semio_s_artifact_stdio_semio_drawing::schema::snapshot::SemioDrawingSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s))).await,
            SemioSubsetSnapshot::Image(s) => SemioImageValidator::validate(&IoPayload::Binary(<semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s))).await,
            SemioSubsetSnapshot::Video(s) => SemioVideoValidator::validate(&IoPayload::Binary(<semio_s_artifact_stdio_semio_video::schema::snapshot::SemioVideoSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s))).await,
            SemioSubsetSnapshot::Audio(s) => SemioAudioValidator::validate(&IoPayload::Binary(<semio_s_artifact_stdio_semio_audio::schema::snapshot::SemioAudioSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s))).await,
            SemioSubsetSnapshot::Animation(s) => SemioAnimationValidator::validate(&IoPayload::Binary(<semio_s_artifact_stdio_semio_animation::schema::snapshot::SemioAnimationSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s))).await,
            SemioSubsetSnapshot::Presentation(s) => SemioPresentationValidator::validate(&IoPayload::Binary(<semio_s_artifact_stdio_semio_presentation::schema::snapshot::SemioPresentationSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s))).await,
            SemioSubsetSnapshot::Flow(s) => SemioFlowValidator::validate(&IoPayload::Binary(<semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s))).await,
            SemioSubsetSnapshot::Text(s) => SemioTextValidator::validate(&IoPayload::Binary(<semio_s_artifact_stdio_semio_text::schema::snapshot::SemioTextSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s))).await,
            SemioSubsetSnapshot::Table(s) => SemioTableValidator::validate(&IoPayload::Binary(<semio_s_artifact_stdio_semio_table::schema::snapshot::SemioTableSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s))).await,
            SemioSubsetSnapshot::Graph(s) => SemioGraphValidator::validate(&IoPayload::Binary(<semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s))).await,
            SemioSubsetSnapshot::Object(s) => SemioObjectValidator::validate(&IoPayload::Binary(<semio_s_artifact_stdio_semio_object::schema::snapshot::SemioObjectSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s))).await,
            SemioSubsetSnapshot::Kit(s) => SemioKitValidator::validate(&IoPayload::Binary(<semio_s_artifact_stdio_semio_kit::schema::snapshot::SemioKitSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s))).await,
        }
    }

    impl SubsetValidator for SemioValidator {
        const DIALECT: Dialect = DIALECT;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => dispatch_validate(&snapshot).await,
                None => vec![Diagnostic {
                    code: semio_framework_diagnostic::FaultCode::new("stdio.semio.any.validate-decode-failed"),
                    severity: semio_framework_diagnostic::Severity::Warning,
                    span: semio_framework_diagnostic::TextSpan::at(1, 1),
                    message: "SemioValidator: payload did not decode as a SemioSnapshot — skipped".into(),
                    expected: None,
                    scope: semio_framework_diagnostic::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioValidator>)
    }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec, and its `SubsetValidator`.
    /// Called from this artifact's standard-level `engine::register()`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        ::semio_framework_schema_registry::register_artifact_schema_descriptor(crate::schema::semio_artifact_schema_descriptor()).expect("schema descriptor publication");
        semio_framework_plugin::io::register_native_document_codec(Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("*") }, store::bare_artifact_codec::<SemioSnapshot, crate::schema::mutations::SemioMutation>(crate::schema::snapshot::STDIO_SEMIO_DOCUMENT_SCHEMA))
            .expect("static Stdio registration must be available and conflict-free");
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
        register_artifact_inferences();
    }

    /// 🧾️ The declarative twin of [`register`]: this envelope's document codec, `SubsetValidator` and inference
    /// descriptor (its schema opens the declaration) as rows of [`crate::declaration`].
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn declare(builder: semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady>) -> semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady> {
        builder
            .document_codec_bare::<SemioSnapshot, crate::schema::mutations::SemioMutation>(crate::schema::snapshot::STDIO_SEMIO_DOCUMENT_SCHEMA, Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("*") })
            .subset_validators(std::slice::from_ref(validator_entry()))
            .inferences([crate::schema::inferences::semio_artifact_inference_descriptor()])
    }

    /// 💡️ Registers `s.stdio.semio.inference`'s facet leaves into the OS-wide inference catalog —
    /// sibling to `register_artifact_schema_descriptor` above (separate registry, ticket
    /// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::schema::inferences::semio_artifact_inference_descriptor()).expect("schema descriptor publication");
    }
    //#endregion 🔖️Register

    //#region 🔖️Tests
    #[cfg(test)]
    include!("🧪️tests/🔬️derived-composition-unit/🦀️.rs");
    //#endregion 🔖️Tests
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️DerivedIoRegistry
/// 🦑 Dissolved out of the former standard-level `⚙️engine` (ticket 26/08/12/ENGINELESS-
/// ARTIFACTS-AND-APP-STATE-MACHINES) — pure `ComposerEntry` aggregation across all 19 subsets
/// (the 13 domain subsets + `text` + this `✉️base` envelope's own), no engine needed.
pub mod io_registry {
    use semio_s_artifact_stdio_semio_animation::io::SemioAnimationComposer;
    use semio_s_artifact_stdio_semio_audio::io::SemioAudioComposer;
    use crate::io::SemioComposer as SemioRawAnyComposer;
    use semio_s_artifact_stdio_semio_brep::io::SemioBrepComposer;
    use semio_s_artifact_stdio_semio_cad::io::SemioCadComposer;
    use semio_s_artifact_stdio_semio_document::io::SemioDocumentComposer;
    use semio_s_artifact_stdio_semio_drawing::io::SemioDrawingComposer;
    use semio_s_artifact_stdio_semio_flow::io::SemioFlowComposer;
    use semio_s_artifact_stdio_semio_graph::io::SemioGraphComposer;
    use semio_s_artifact_stdio_semio_image::io::SemioImageComposer;
    use semio_s_artifact_stdio_semio_kit::io::SemioKitComposer;
    use semio_s_artifact_stdio_semio_mesh::io::SemioMeshComposer;
    use semio_s_artifact_stdio_semio_model::io::SemioModelComposer;
    use semio_s_artifact_stdio_semio_object::io::SemioObjectComposer;
    use semio_s_artifact_stdio_semio_presentation::io::SemioPresentationComposer;
    use semio_s_artifact_stdio_semio_table::io::SemioTableComposer;
    use semio_s_artifact_stdio_semio_text::io::SemioTextComposer;
    use semio_s_artifact_stdio_semio_value::io::SemioValueComposer;
    use semio_s_artifact_stdio_semio_video::io::SemioVideoComposer;
    use semio_framework_plugin::{composer_entry_of, io::ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES
            .get_or_init(|| {
                vec![
                    composer_entry_of::<SemioBrepComposer>(),
                    composer_entry_of::<SemioMeshComposer>(),
                    composer_entry_of::<SemioModelComposer>(),
                    composer_entry_of::<SemioValueComposer>(),
                    composer_entry_of::<SemioDocumentComposer>(),
                    composer_entry_of::<SemioCadComposer>(),
                    composer_entry_of::<SemioDrawingComposer>(),
                    composer_entry_of::<SemioImageComposer>(),
                    composer_entry_of::<SemioVideoComposer>(),
                    composer_entry_of::<SemioAudioComposer>(),
                    composer_entry_of::<SemioAnimationComposer>(),
                    composer_entry_of::<SemioPresentationComposer>(),
                    composer_entry_of::<SemioFlowComposer>(),
                    composer_entry_of::<SemioTextComposer>(),
                    composer_entry_of::<SemioTableComposer>(),
                    composer_entry_of::<SemioGraphComposer>(),
                    composer_entry_of::<SemioObjectComposer>(),
                    composer_entry_of::<SemioKitComposer>(),
                    composer_entry_of::<SemioRawAnyComposer>(),
                ]
            })
            .as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use crate::schema::diff::SemioDiff;
    use crate::schema::mutations::SemioMutation;
    use crate::schema::snapshot::SemioSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct SemioBuilderConstruction {
        snapshot: SemioSnapshot,
    }

    impl ArtifactBuilder for SemioBuilderConstruction {
        type Snapshot = SemioSnapshot;
        type Mutation = SemioMutation;
        type Diff = SemioDiff;
        fn empty() -> Self {
            Self { snapshot: SemioSnapshot::default() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<SemioSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <SemioMutation as protocol::Mutation<SemioSnapshot>>::diff(&mutation, &self.snapshot);
            match protocol::apply_diff(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => (Self { snapshot, ..self }, outcome),
                Err(error) => (self, protocol::MutationOutcome::fatal(error.code, error.message, error.target)),
            }
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            Ok(self.snapshot)
        }
    }
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::schema::snapshot::{SemioSnapshot, STDIO_SEMIO_DOCUMENT_SCHEMA};
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct SemioParts {
        pub snapshot: Option<SemioSnapshot>,
    }

    pub struct SemioAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioAnalyzerAnalysis {
        type Parts = SemioParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIO_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIO_DOCUMENT_SCHEMA) {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes) {
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
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec SemioBuilderFacets {
        construction: SemioBuilderConstruction,
        analysis: SemioAnalyzerAnalysis,
        composition: SemioComposerComposition,
    }
    builder: SemioBuilder,
    analyzer: SemioAnalyzer,
    composer: SemioComposer,
);
