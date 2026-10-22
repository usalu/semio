//! 🚪️ IO `s.stdio.semio` (v1/video) — real cross-format bridge leaves (W4): typed
//! `ArtifactDeserializer`/`ArtifactSerializer` impls, one pair per bridged format
//! (`video↔mp4`, `video↔avi` per the master plan's io lattice). Each leaf module is mounted here
//! (not in `🦀️.rs`, a closer-only hot file) via `#[path=...]`, resolved relative to this
//! file's own directory — the same mechanism `🦀️.rs` itself uses one level up. Registration
//! flows through `🎹️composer::register` (see that module), matching the repo-wide convention.

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use semio_s_artifact_stdio_semio_video_model::schema::snapshot::SemioVideoSnapshot;
    use semio_s_artifact_stdio_semio_video_model::schema::snapshot::SemioVideoStreamKind;
    use crate::io::SemioVideoAnalyzer;
    use {semio_framework_plugin::io::register_subset_validator,semio_framework_plugin::io::subset_validator_entry_of,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_plugin::io::SubsetValidator,semio_framework_plugin::io::SubsetValidatorEntry};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("video") };

    //#region 🔖️Composer
    pub struct SemioVideoComposerComposition;

    impl ArtifactComposition for SemioVideoComposerComposition {
        type Snapshot = SemioVideoSnapshot;
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
                return Err(ComposeError { message: "SemioVideoComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioVideoAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "SemioVideoComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ Real referential-invariant checks (decode-only was the W1b scaffold; this is the
    /// per-subset check D5's validate-on-build hook is FOR): `rate.den` must never be zero (it is a
    /// divisor everywhere frame timing is computed downstream); a `Video`-kind stream's `width`/
    /// `height` must be nonzero (a video stream with a zero raster dimension is not decodable by any
    /// real container reader); a stream's `samples` should carry monotonically nondecreasing `pts`
    /// (soft — real containers legitimately reorder decode order vs. presentation order for B-frames,
    /// so this is a `Warning`, never a hard `Error`, honestly reflecting that this subset cannot tell
    /// decode order from presentation order from the metadata alone).
    pub struct SemioVideoValidator;

    /// 🧮️ Runs this subset's real referential-invariant checks against an already-decoded snapshot —
    /// shared by the registered `SubsetValidator` (wire-payload recheck) and this file's own unit
    /// tests (which exercise it directly against hand-built snapshots).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_semio_video_invariants(snapshot: &SemioVideoSnapshot) -> Vec<semio_framework_diagnostic::Diagnostic> {
        let mut out = Vec::new();
        for (stream_index, stream) in snapshot.streams.iter().enumerate() {
            if stream.rate.den == 0 {
                out.push(semio_framework_diagnostic::Diagnostic::error("stdio.semio_video.rate-zero-denominator", semio_framework_diagnostic::TextSpan::at(1, 1), format!("stream {stream_index}: rate denominator is 0 (rate.num={})", stream.rate.num)));
            }
            if stream.kind == SemioVideoStreamKind::Video && (stream.width == 0 || stream.height == 0) {
                out.push(semio_framework_diagnostic::Diagnostic::error("stdio.semio_video.video-stream-zero-dimension", semio_framework_diagnostic::TextSpan::at(1, 1), format!("stream {stream_index}: kind=Video but width={} height={}", stream.width, stream.height)));
            }
            let mut prev_pts: Option<u64> = None;
            for (sample_index, sample) in stream.samples.iter().enumerate() {
                if let Some(prev) = prev_pts {
                    if sample.pts < prev {
                        out.push(semio_framework_diagnostic::Diagnostic {
                            code: semio_framework_diagnostic::FaultCode::new("stdio.semio_video.pts-non-monotonic"),
                            severity: semio_framework_diagnostic::Severity::Warning,
                            span: semio_framework_diagnostic::TextSpan::at(1, 1),
                            message: format!("stream {stream_index} sample {sample_index}: pts {} < previous pts {prev} (allowed — decode order may legitimately differ from presentation order)", sample.pts),
                            expected: None,
                            scope: semio_framework_diagnostic::FaultScope::default(),
                        });
                    }
                }
                prev_pts = Some(sample.pts);
            }
        }
        out
    }

    impl SubsetValidator for SemioVideoValidator {
        const DIALECT: Dialect = DIALECT;
        async fn validate(payload: &IoPayload) -> Vec<semio_framework_diagnostic::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioVideoSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioVideoSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_semio_video_invariants(&snapshot),
                None => vec![semio_framework_diagnostic::Diagnostic::error("stdio.semio_video.validate-decode-failed", semio_framework_diagnostic::TextSpan::at(1, 1), "SemioVideoValidator: payload did not decode as a SemioVideoSnapshot".to_string())],
            }
        }
    }

    static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioVideoValidator>)
    }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec (`"s.stdio.semio.video"` — the
    /// repo-wide-unique id `policyDocumentCodecDuplicateIds` checks statically), and SubsetValidator.
    /// Called from this artifact's standard-level `engine::register()`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        ::semio_framework_schema_registry::register_artifact_schema_descriptor(semio_s_artifact_stdio_semio_video_descriptor::schema::descriptor::semio_video_artifact_schema_descriptor()).expect("schema descriptor publication");
        semio_framework_plugin::io::register_native_snapshot_codec(Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("video") }, store::artifact_codec::<SemioVideoSnapshot, semio_s_artifact_stdio_semio_video_mutations::schema::mutations::SemioVideoMutation>(
            semio_s_artifact_stdio_semio_video_model::schema::snapshot::STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA,
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
            .schemas([semio_s_artifact_stdio_semio_video_descriptor::schema::descriptor::semio_video_artifact_schema_descriptor()])
            .document_codec_bare::<SemioVideoSnapshot, semio_s_artifact_stdio_semio_video_mutations::schema::mutations::SemioVideoMutation>(semio_s_artifact_stdio_semio_video_model::schema::snapshot::STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA, Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("video") })
            .subset_validators(std::slice::from_ref(validator_entry()))
            .inferences([semio_s_artifact_stdio_semio_video_inferences::schema::inferences::semio_video_artifact_inference_descriptor()]);
        builder
    }

    /// 💡️ Registers `s.stdio.semio.video.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::semio_framework_schema_registry::register_artifact_inference_descriptor(semio_s_artifact_stdio_semio_video_inferences::schema::inferences::semio_video_artifact_inference_descriptor()).expect("schema descriptor publication");
    }

    //#endregion 🔖️Register

    //#region 🧪️Tests
    #[cfg(test)]
    include!("🧪️tests/🔬️derived-composition-unit/🦀️.rs");
    //#endregion 🧪️Tests
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition




pub mod derived_construction {
    use semio_s_artifact_stdio_semio_video_model::schema::diff::SemioVideoDiff;
    use semio_s_artifact_stdio_semio_video_mutations::schema::mutations::SemioVideoMutation;
    use semio_s_artifact_stdio_semio_video_model::schema::snapshot::SemioVideoSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct SemioVideoBuilderConstruction {
        snapshot: SemioVideoSnapshot,
    }

    impl ArtifactBuilder for SemioVideoBuilderConstruction {
        type Snapshot = SemioVideoSnapshot;
        type Mutation = SemioVideoMutation;
        type Diff = SemioVideoDiff;
        fn empty() -> Self {
            Self { snapshot: SemioVideoSnapshot::default() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<SemioVideoSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioVideoSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <SemioVideoMutation as protocol::ColdMutation<SemioVideoSnapshot>>::diff(&mutation, &self.snapshot);
            match protocol::apply_diff_cold(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => (Self { snapshot, ..self }, outcome),
                Err(error) => (self, protocol::MutationOutcome::fatal(error.code, error.message, error.target)),
            }
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
    use semio_s_artifact_stdio_semio_video_model::schema::snapshot::SemioVideoSnapshot;
    use semio_s_artifact_stdio_semio_video_model::schema::snapshot::STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct SemioVideoParts {
        pub snapshot: Option<SemioVideoSnapshot>,
    }

    pub struct SemioVideoAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioVideoAnalyzerAnalysis {
        type Parts = SemioVideoParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("video") };

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA) {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioVideoParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioVideoSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioVideoSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes) {
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
    pub spec SemioVideoBuilderFacets {
        construction: SemioVideoBuilderConstruction,
        analysis: SemioVideoAnalyzerAnalysis,
        composition: SemioVideoComposerComposition,
    }
    builder: SemioVideoBuilder,
    analyzer: SemioVideoAnalyzer,
    composer: SemioVideoComposer,
);
