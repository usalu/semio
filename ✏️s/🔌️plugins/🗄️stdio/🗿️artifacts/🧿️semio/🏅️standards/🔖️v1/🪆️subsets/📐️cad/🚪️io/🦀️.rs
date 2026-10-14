//! 🚪️ IO — structure only; registration flows through 🎹️composer::register (matching the
//! repo-wide convention — see gif's own io leaf doc comment). W4 adds the real semio↔dxf/dwg/step
//! import/export leaves under 📥️import/🧩️deserializers and 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::schema::snapshot::{CadEntity, SemioCadSnapshot};
    use crate::io::SemioCadAnalyzer;
    use {semio_framework_plugin::io::register_subset_validator,semio_framework_plugin::io::subset_validator_entry_of,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_plugin::io::SubsetValidator,semio_framework_plugin::io::SubsetValidatorEntry};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("cad") };

    //#region 🔖️Composer
    pub struct SemioCadComposerComposition;

    impl ArtifactComposition for SemioCadComposerComposition {
        type Snapshot = SemioCadSnapshot;
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
                return Err(ComposeError { message: "SemioCadComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioCadAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "SemioCadComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ Real referential-invariant checks over the subset's OWN collections: every
    /// `CadEntityRecord.layer` (top-level AND nested inside a block) must name a real `CadLayer`;
    /// every `CadEntity::Insert.block_name` must name a real `CadBlock` and must not name its OWN
    /// containing block (a self-referential insert is an infinite-recursion cycle, not valid content).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn cad_referential_diagnostics(snapshot: &SemioCadSnapshot) -> Vec<semio_framework_diagnostic::Diagnostic> {
        let mut diagnostics = Vec::new();
        let layer_names: std::collections::BTreeSet<&str> = snapshot.layers.iter().map(|l| l.name.as_str()).collect();
        let block_names: std::collections::BTreeSet<&str> = snapshot.blocks.iter().map(|b| b.name.as_str()).collect();

        let check_record = |diagnostics: &mut Vec<semio_framework_diagnostic::Diagnostic>, owning_block: Option<&str>, rec: &crate::schema::snapshot::CadEntityRecord| {
            if !layer_names.contains(rec.layer.as_str()) {
                diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.semio_cad.dangling-layer", semio_framework_diagnostic::TextSpan::at(1, 1), format!("entity {:?} (handle {:?}) references undefined layer {:?}", owning_block.unwrap_or("<top-level>"), rec.handle, rec.layer)));
            }
            if let CadEntity::Insert { block_name, .. } = &rec.entity {
                if !block_names.contains(block_name.as_str()) {
                    diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.semio_cad.dangling-block-insert", semio_framework_diagnostic::TextSpan::at(1, 1), format!("entity handle {:?} inserts undefined block {:?}", rec.handle, block_name)));
                }
                if owning_block == Some(block_name.as_str()) {
                    diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.semio_cad.self-referential-insert", semio_framework_diagnostic::TextSpan::at(1, 1), format!("block {:?} contains an Insert of itself (handle {:?}) -- infinite recursion", block_name, rec.handle)));
                }
            }
        };

        for rec in &snapshot.entities {
            check_record(&mut diagnostics, None, rec);
        }
        for block in &snapshot.blocks {
            for rec in &block.entities {
                check_record(&mut diagnostics, Some(block.name.as_str()), rec);
            }
        }
        diagnostics
    }

    pub struct SemioCadValidator;

    impl SubsetValidator for SemioCadValidator {
        const DIALECT: Dialect = DIALECT;
        async fn validate(payload: &IoPayload) -> Vec<semio_framework_diagnostic::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioCadSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioCadSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => cad_referential_diagnostics(&snapshot),
                None => vec![semio_framework_diagnostic::Diagnostic::error("stdio.semio_cad.validate-decode-failed", semio_framework_diagnostic::TextSpan::at(1, 1), "SemioCadValidator: payload did not decode as a SemioCadSnapshot".to_string())],
            }
        }
    }

    static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioCadValidator>)
    }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️IoEntries
    //#endregion 🔖️IoEntries

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec, SubsetValidator, and (W4) its
    /// semio↔format io bridges. Called from this artifact's standard-level `engine::register()`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        ::semio_framework_schema_registry::register_artifact_schema_descriptor(crate::schema::semio_cad_artifact_schema_descriptor()).expect("schema descriptor publication");
        semio_framework_plugin::io::register_native_document_codec(Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("cad") }, store::bare_artifact_codec::<SemioCadSnapshot, crate::schema::mutations::SemioCadMutation>(crate::schema::snapshot::STDIO_SEMIOCAD_DOCUMENT_SCHEMA))
            .expect("static Stdio registration must be available and conflict-free");
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
        register_artifact_inferences();
    }

    /// 🧾️ The declarative twin of [`register`]: this subset's schema, document codec, `SubsetValidator`, composers
    /// (those writing semio, [`crate::semio_written`]) and inference descriptor as rows of [`crate::declaration`].
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn declare(builder: semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady>) -> semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady> {
        let builder = builder
            .schemas([crate::schema::semio_cad_artifact_schema_descriptor()])
            .document_codec_bare::<SemioCadSnapshot, crate::schema::mutations::SemioCadMutation>(crate::schema::snapshot::STDIO_SEMIOCAD_DOCUMENT_SCHEMA, Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("cad") })
            .subset_validators(std::slice::from_ref(validator_entry()))
            .inferences([crate::schema::inferences::semio_cad_artifact_inference_descriptor()]);
        builder
    }

    /// 💡️ Registers `s.stdio.semio.cad.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::schema::inferences::semio_cad_artifact_inference_descriptor()).expect("schema descriptor publication");
    }
    //#endregion 🔖️Register

    //#region 🔖️Tests
    #[cfg(test)]
    include!("🧪️tests/🔬️derived-composition-unit/🦀️.rs");
    //#endregion 🔖️Tests
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
    use crate::schema::diff::SemioCadDiff;
    use crate::schema::mutations::SemioCadMutation;
    use crate::schema::snapshot::SemioCadSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct SemioCadBuilderConstruction {
        snapshot: SemioCadSnapshot,
    }

    impl ArtifactBuilder for SemioCadBuilderConstruction {
        type Snapshot = SemioCadSnapshot;
        type Mutation = SemioCadMutation;
        type Diff = SemioCadDiff;
        fn empty() -> Self {
            Self { snapshot: SemioCadSnapshot::default() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<SemioCadSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioCadSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <SemioCadMutation as protocol::Mutation<SemioCadSnapshot>>::diff(&mutation, &self.snapshot);
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
    use crate::schema::snapshot::{SemioCadSnapshot, STDIO_SEMIOCAD_DOCUMENT_SCHEMA};
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct SemioCadParts {
        pub snapshot: Option<SemioCadSnapshot>,
    }

    pub struct SemioCadAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioCadAnalyzerAnalysis {
        type Parts = SemioCadParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("cad") };

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIOCAD_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIOCAD_DOCUMENT_SCHEMA) {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioCadParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioCadSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioCadSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes) {
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
    pub spec SemioCadBuilderFacets {
        construction: SemioCadBuilderConstruction,
        analysis: SemioCadAnalyzerAnalysis,
        composition: SemioCadComposerComposition,
    }
    builder: SemioCadBuilder,
    analyzer: SemioCadAnalyzer,
    composer: SemioCadComposer,
);
