//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::schema::snapshot::SemioImageSnapshot;
    use crate::io::SemioImageAnalyzer;
    use {semio_framework_plugin::io::register_subset_validator,semio_framework_plugin::io::subset_validator_entry_of,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_plugin::io::SubsetValidator,semio_framework_plugin::io::SubsetValidatorEntry};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("image") };

    //#region 🔖️Composer
    pub struct SemioImageComposerComposition;

    impl ArtifactComposition for SemioImageComposerComposition {
        type Snapshot = SemioImageSnapshot;
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
                return Err(ComposeError { message: "SemioImageComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioImageAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "SemioImageComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ 🚧 scaffolded by W1b — decode-only validator (no referential-invariant diagnostics yet;
    /// W2 adds real cross-reference checks).
    pub struct SemioImageValidator;

    impl SubsetValidator for SemioImageValidator {
        const DIALECT: Dialect = DIALECT;
        async fn validate(payload: &IoPayload) -> Vec<semio_framework_diagnostic::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioImageSnapshot as store::os_store::serialization::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioImageSnapshot as store::os_store::serialization::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(_) => Vec::new(),
                None => vec![semio_framework_diagnostic::Diagnostic::error("stdio.semio_image.validate-decode-failed", semio_framework_diagnostic::TextSpan::at(1, 1), "SemioImageValidator: payload did not decode as a SemioImageSnapshot".to_string())],
            }
        }
    }

    static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioImageValidator>)
    }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️IoEntries
    //#endregion 🔖️IoEntries

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec, SubsetValidator, and (W4) its
    /// semio↔format io bridges. Called from this artifact's standard-level `engine::register()`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        ::semio_framework_schema_registry::register_artifact_schema_descriptor(crate::schema::semio_image_artifact_schema_descriptor()).expect("schema descriptor publication");
        semio_framework_plugin::io::register_native_snapshot_codec(Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("image") }, store::artifact_codec::<SemioImageSnapshot, crate::schema::mutations::SemioImageMutation>(
            crate::schema::snapshot::STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA,
        ))
        .expect("static Stdio registration must be available and conflict-free");
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
        register_artifact_inferences();
    }

    /// 🧾️ The declarative twin of [`register`]: this subset's schema, document codec, `SubsetValidator`, composers
    /// (those writing semio, [`crate::semio_written`]) and inference descriptor as rows of [`crate::declaration`].
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn declare(builder: semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady>) -> semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady> {
        let builder = builder
            .schemas([crate::schema::semio_image_artifact_schema_descriptor()])
            .document_codec_bare::<SemioImageSnapshot, crate::schema::mutations::SemioImageMutation>(crate::schema::snapshot::STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA, Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("image") })
            .subset_validators(std::slice::from_ref(validator_entry()))
            .inferences([crate::schema::inferences::semio_image_artifact_inference_descriptor()]);
        builder
    }

    /// 💡️ Registers `s.stdio.semio.image.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::schema::inferences::semio_image_artifact_inference_descriptor()).expect("schema descriptor publication");
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
    use crate::schema::diff::SemioImageDiff;
    use crate::schema::mutations::SemioImageMutation;
    use crate::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry, SemioImageSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct SemioImageBuilderConstruction {
        snapshot: SemioImageSnapshot,
    }

    //#region 🔖️TypedConstructors
    impl SemioImageBuilderConstruction {
        /// 🏗️ Starts a fresh image at the given pixel dimensions.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn new(width: u32, height: u32) -> Self {
            Self { snapshot: SemioImageSnapshot { width, height, ..SemioImageSnapshot::default() } }
        }
        /// 🏗️ Sets the source colorspace.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn set_colorspace(mut self, colorspace: SemioColorspace) -> Self {
            self.snapshot.colorspace = colorspace;
            self
        }
        /// 🏗️ Sets the bit depth.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn set_bit_depth(mut self, bit_depth: u8) -> Self {
            self.snapshot.bit_depth = bit_depth;
            self
        }
        /// 🏗️ Appends one frame, in order.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_frame(mut self, frame: SemioImageFrame) -> Self {
            self.snapshot.frames.push(frame);
            self
        }
        /// 🏗️ Sets the embedded ICC profile (`None` clears it).
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn set_icc(mut self, icc: Option<Vec<u8>>) -> Self {
            self.snapshot.icc = icc;
            self
        }
        /// 🏗️ Appends one metadata entry.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
            self.snapshot.metadata.push(SemioImageMetadataEntry { key: key.into(), value: value.into() });
            self
        }
    }
    //#endregion 🔖️TypedConstructors

    impl ArtifactBuilder for SemioImageBuilderConstruction {
        type Snapshot = SemioImageSnapshot;
        type Mutation = SemioImageMutation;
        type Diff = SemioImageDiff;
        fn empty() -> Self {
            Self { snapshot: SemioImageSnapshot::default() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<SemioImageSnapshot as store::os_store::serialization::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioImageSnapshot as store::os_store::serialization::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <SemioImageMutation as protocol::Mutation<SemioImageSnapshot>>::diff(&mutation, &self.snapshot);
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

    //#region 🔖️Tests
    #[cfg(test)]
    include!("../🧬️schema/🧪️tests/🔬️derived-construction-unit/🦀️.rs");
    //#endregion 🔖️Tests
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::schema::snapshot::{SemioImageSnapshot, STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA};
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct SemioImageParts {
        pub snapshot: Option<SemioImageSnapshot>,
    }

    pub struct SemioImageAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioImageAnalyzerAnalysis {
        type Parts = SemioImageParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("image") };

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA) {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioImageParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioImageSnapshot as store::os_store::serialization::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioImageSnapshot as store::os_store::serialization::ArtifactPack>::decode_pack(bytes) {
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
    pub spec SemioImageBuilderFacets {
        construction: SemioImageBuilderConstruction,
        analysis: SemioImageAnalyzerAnalysis,
        composition: SemioImageComposerComposition,
    }
    builder: SemioImageBuilder,
    analyzer: SemioImageAnalyzer,
    composer: SemioImageComposer,
);
