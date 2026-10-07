//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    #[cfg(feature = "conversion-presentation")]
    use super::super::export::serializers::artifacts::pptx::v_ecma_376::any::SemioPresentationToPptx;
    #[cfg(feature = "conversion-presentation")]
    use super::super::import::deserializers::artifacts::pptx::v_ecma_376::any::SemioPresentationFromPptx;
    use crate::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot;
    use crate::standards::v1::subsets::presentation::io::SemioPresentationAnalyzer;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::TextSpan;
    #[cfg(feature = "conversion-presentation")]
    use semio_framework_plugin::{deserializer_entry_of, register_composer_entries, serializer_entry_of, ComposerEntry};
    use {semio_framework_plugin::register_subset_validator,semio_framework_plugin::subset_validator_entry_of,semio_framework_plugin::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::ComposeError,semio_framework_plugin::ComposeSource,semio_framework_plugin::Composition,semio_framework_artifact_reference::Dialect,semio_framework_plugin::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_plugin::SubsetValidator,semio_framework_plugin::SubsetValidatorEntry};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("presentation") };

    //#region 🔖️Composer
    pub struct SemioPresentationComposerComposition;

    impl ArtifactComposition for SemioPresentationComposerComposition {
        type Snapshot = SemioPresentationSnapshot;
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
                return Err(ComposeError { message: "SemioPresentationComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioPresentationAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "SemioPresentationComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ Referential-invariant checks over a decoded `SemioPresentationSnapshot`: every
    /// `layout.master_id` must resolve to a real `masters` entry, every `slide.layout_id` (when set)
    /// must resolve to a real `layouts` entry, and `masters`/`layouts` ids must each be unique (both
    /// collections are name-keyed in the diff facet — a duplicate id would silently corrupt any future
    /// `between()`/`apply()` on this snapshot). Real structural checks, not a decode-only stub.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_presentation_referential_integrity(snapshot: &SemioPresentationSnapshot) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        let mut seen_master_ids = std::collections::HashSet::new();
        for master in &snapshot.masters {
            if !seen_master_ids.insert(master.id.as_str()) {
                diagnostics.push(Diagnostic::error("stdio.semio_presentation.duplicate-master-id", TextSpan::at(1, 1), format!("duplicate master id {:?}", master.id)));
            }
        }
        let mut seen_layout_ids = std::collections::HashSet::new();
        for layout in &snapshot.layouts {
            if !seen_layout_ids.insert(layout.id.as_str()) {
                diagnostics.push(Diagnostic::error("stdio.semio_presentation.duplicate-layout-id", TextSpan::at(1, 1), format!("duplicate layout id {:?}", layout.id)));
            }
            if !seen_master_ids.contains(layout.master_id.as_str()) {
                diagnostics.push(Diagnostic::error("stdio.semio_presentation.dangling-layout-master", TextSpan::at(1, 1), format!("layout {:?} references unknown master {:?}", layout.id, layout.master_id)));
            }
        }
        for slide in &snapshot.slides {
            if let Some(layout_id) = &slide.layout_id {
                if !seen_layout_ids.contains(layout_id.as_str()) {
                    diagnostics.push(Diagnostic::error("stdio.semio_presentation.dangling-slide-layout", TextSpan::at(1, 1), format!("slide {:?} references unknown layout {:?}", slide.id, layout_id)));
                }
            }
        }
        diagnostics
    }

    pub struct SemioPresentationValidator;

    impl SubsetValidator for SemioPresentationValidator {
        const DIALECT: Dialect = DIALECT;
        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioPresentationSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioPresentationSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_presentation_referential_integrity(&snapshot),
                None => vec![Diagnostic::error("stdio.semio_presentation.validate-decode-failed", TextSpan::at(1, 1), "SemioPresentationValidator: payload did not decode as a SemioPresentationSnapshot".to_string())],
            }
        }
    }

    static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioPresentationValidator>)
    }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️IoEntries
    /// 🚪️ presentation<->pptx bridge row (W4 G6) — one `deserializer_entry_of` (pptx -> semio) +
    /// one `serializer_entry_of` (semio -> pptx); `register_composer_entries` derives all 4 `IoKey`s
    /// from these 2 rows (see `document`'s own composer for the fuller doc comment on this mechanism).
    #[cfg(feature = "conversion-presentation")]
    static IO_ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    #[cfg(feature = "conversion-presentation")]
    fn io_entries() -> &'static [ComposerEntry] {
        IO_ENTRIES.get_or_init(|| vec![deserializer_entry_of::<SemioPresentationFromPptx>(), serializer_entry_of::<SemioPresentationToPptx>()])
    }
    //#endregion 🔖️IoEntries

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec, SubsetValidator, and the
    /// presentation<->pptx io bridge row. Called from this artifact's standard-level `engine::register()`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        ::semio_framework_schema_registry::register_artifact_schema_descriptor(crate::standards::v1::subsets::presentation::schema::semio_presentation_artifact_schema_descriptor()).expect("schema descriptor publication");
        semio_framework_plugin::io::register_native_document_codec(semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.semio", standard: semio_framework_artifact_reference::StandardId("v1"), subset: semio_framework_artifact_reference::SubsetId("presentation") }, store::ArtifactCodec::bare::<SemioPresentationSnapshot, crate::standards::v1::subsets::presentation::schema::mutations::SemioPresentationMutation>(
            crate::standards::v1::subsets::presentation::schema::snapshot::STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA,
        ))
        .expect("static Stdio registration must be available and conflict-free");
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
        #[cfg(feature = "conversion-presentation")]
        register_composer_entries(io_entries()).expect("static Stdio registration must be available and conflict-free");
        register_artifact_inferences();
    }

    /// 🧾️ The declarative twin of [`register`]: this subset's schema, document codec, `SubsetValidator`, composers
    /// (those writing semio, [`crate::semio_written`]) and inference descriptor as rows of [`crate::declaration`].
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn declare(builder: semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady>) -> semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady> {
        let builder = builder
            .schemas([crate::standards::v1::subsets::presentation::schema::semio_presentation_artifact_schema_descriptor()])
            .document_codec_bare::<SemioPresentationSnapshot, crate::standards::v1::subsets::presentation::schema::mutations::SemioPresentationMutation>(crate::standards::v1::subsets::presentation::schema::snapshot::STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.semio", standard: semio_framework_artifact_reference::StandardId("v1"), subset: semio_framework_artifact_reference::SubsetId("presentation") })
            .subset_validators(std::slice::from_ref(validator_entry()))
            .inferences([crate::standards::v1::subsets::presentation::schema::inferences::semio_presentation_artifact_inference_descriptor()]);
        #[cfg(feature = "conversion-presentation")]
        let builder = {
            static COMPOSERS: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
            builder.composers(crate::semio_written(io_entries(), &COMPOSERS))
        };
        builder
    }

    /// 💡️ Registers `s.stdio.semio.presentation.inference`'s facet leaves into the OS-wide
    /// inference catalog — sibling to `register_artifact_schema_descriptor` above (separate
    /// registry, ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::standards::v1::subsets::presentation::schema::inferences::semio_presentation_artifact_inference_descriptor()).expect("schema descriptor publication");
    }
    //#endregion 🔖️Register

    //#region 🧪️Tests
    #[cfg(all(test, feature = "conversion-presentation"))]
    include!("🧪️tests/🔬️derived-composition-unit/🦀️.rs");
    //#endregion 🧪️Tests
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use crate::standards::v1::subsets::presentation::schema::diff::SemioPresentationDiff;
    #[cfg(test)]
    use crate::standards::v1::subsets::presentation::schema::mutations::insert_master;
    use crate::standards::v1::subsets::presentation::schema::mutations::{apply_semio_presentation_mutation, SemioPresentationMutation};
    use crate::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct SemioPresentationBuilderConstruction {
        snapshot: SemioPresentationSnapshot,
    }

    impl ArtifactBuilder for SemioPresentationBuilderConstruction {
        type Snapshot = SemioPresentationSnapshot;
        type Mutation = SemioPresentationMutation;
        type Diff = SemioPresentationDiff;
        fn empty() -> Self {
            Self { snapshot: SemioPresentationSnapshot::default() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<SemioPresentationSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioPresentationSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_semio_presentation_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <SemioPresentationDiff as protocol::MutationDiff<SemioPresentationSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            Ok(self.snapshot)
        }
    }

    //#region 🧪️Tests
    #[cfg(test)]
    include!("../🧬️schema/🧪️tests/🔬️derived-construction-unit/🦀️.rs");
    //#endregion 🧪️Tests
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::standards::v1::subsets::presentation::schema::snapshot::{SemioPresentationSnapshot, STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA};
    use {semio_framework_plugin::Analysis,semio_framework_plugin::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_plugin::IoConfidence,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct SemioPresentationParts {
        pub snapshot: Option<SemioPresentationSnapshot>,
    }

    pub struct SemioPresentationAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioPresentationAnalyzerAnalysis {
        type Parts = SemioPresentationParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("presentation") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioPresentationParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioPresentationSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioPresentationSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }

    //#region 🧪️Tests
    #[cfg(test)]
    include!("../🧬️schema/🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
    //#endregion 🧪️Tests
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec SemioPresentationBuilderFacets {
        construction: SemioPresentationBuilderConstruction,
        analysis: SemioPresentationAnalyzerAnalysis,
        composition: crate::standards::v1::subsets::presentation::io::derived_composition::SemioPresentationComposerComposition,
    }
    builder: SemioPresentationBuilder,
    analyzer: SemioPresentationAnalyzer,
    composer: SemioPresentationComposer,
);
