//! 🚪️ IO `s.stdio.semio` (v1/video) — real cross-format bridge leaves (W4): typed
//! `ArtifactDeserializer`/`ArtifactSerializer` impls, one pair per bridged format
//! (`video↔mp4`, `video↔avi` per the master plan's io lattice). Each leaf module is mounted here
//! (not in `🦀️.rs`, a closer-only hot file) via `#[path=...]`, resolved relative to this
//! file's own directory — the same mechanism `🦀️.rs` itself uses one level up. Registration
//! flows through `🎹️composer::register` (see that module), matching the repo-wide convention.

#[cfg(feature = "conversion-video-avi")]
#[path = "📥️import/🧩️deserializers/🗿️artifacts/📼️avi/🔖️1.0/✳️any/🦀️.rs"]
pub mod avi_deserializer;
#[cfg(feature = "conversion-video-avi")]
#[path = "📤️export/🧵️serializers/🗿️artifacts/📼️avi/🔖️1.0/✳️any/🦀️.rs"]
pub mod avi_serializer;
#[cfg(feature = "conversion-video-mp4")]
#[path = "📥️import/🧩️deserializers/🗿️artifacts/🎥️mp4/🔖️isobmff/✳️any/🦀️.rs"]
pub mod mp4_deserializer;
#[cfg(feature = "conversion-video-mp4")]
#[path = "📤️export/🧵️serializers/🗿️artifacts/🎥️mp4/🔖️isobmff/✳️any/🦀️.rs"]
pub mod mp4_serializer;
//#region 🎹️DerivedComposition
pub mod derived_composition {
    #[cfg(feature = "conversion-video-avi")]
    use crate::standards::v1::subsets::video::io::{avi_deserializer::SemioVideoFromAvi, avi_serializer::SemioVideoToAvi};
    #[cfg(feature = "conversion-video-mp4")]
    use crate::standards::v1::subsets::video::io::{mp4_deserializer::SemioVideoFromMp4, mp4_serializer::SemioVideoToMp4};
    use crate::standards::v1::subsets::video::schema::snapshot::{SemioVideoSnapshot, SemioVideoStreamKind};
    use crate::standards::v1::subsets::video::schema::SemioVideoAnalyzer;
    #[cfg(feature = "conversion-video")]
    use semio_framework_plugin::{deserializer_entry_of, register_composer_entries, serializer_entry_of};
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};

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
                IoPayload::Binary(bytes) => <SemioVideoSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioVideoSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
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
        ::semio_framework_schema_registry::register_artifact_schema_descriptor(crate::standards::v1::subsets::video::schema::semio_video_artifact_schema_descriptor()).expect("schema descriptor publication");
        semio_framework_plugin::io::register_native_snapshot_codec(semio_framework_plugin::Dialect { artifact_kind: "s.stdio.semio", standard: semio_framework_plugin::StandardId("v1"), subset: semio_framework_plugin::SubsetId("video") }, store::ArtifactCodec::of::<SemioVideoSnapshot, crate::standards::v1::subsets::video::schema::mutations::SemioVideoMutation>(
            crate::standards::v1::subsets::video::schema::snapshot::STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA,
        ))
        .expect("static Stdio registration must be available and conflict-free");
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
        #[cfg(feature = "conversion-video")]
        register_composer_entries(bridge_entries()).expect("static Stdio registration must be available and conflict-free");
        register_artifact_inferences();
    }

    /// 🧾️ The declarative twin of [`register`]: this subset's schema, document codec, `SubsetValidator`, composers
    /// (those writing semio, [`crate::semio_written`]) and inference descriptor as rows of [`crate::declaration`].
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn declare(builder: semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady>) -> semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady> {
        let builder = builder
            .schemas([crate::standards::v1::subsets::video::schema::semio_video_artifact_schema_descriptor()])
            .document_codec_bare::<SemioVideoSnapshot, crate::standards::v1::subsets::video::schema::mutations::SemioVideoMutation>(crate::standards::v1::subsets::video::schema::snapshot::STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA, semio_framework_plugin::Dialect { artifact_kind: "s.stdio.semio", standard: semio_framework_plugin::StandardId("v1"), subset: semio_framework_plugin::SubsetId("video") })
            .subset_validators(std::slice::from_ref(validator_entry()))
            .inferences([crate::standards::v1::subsets::video::schema::inferences::semio_video_artifact_inference_descriptor()]);
        #[cfg(feature = "conversion-video")]
        let builder = {
            static COMPOSERS: std::sync::OnceLock<Vec<semio_framework_plugin::ComposerEntry>> = std::sync::OnceLock::new();
            builder.composers(crate::semio_written(bridge_entries(), &COMPOSERS))
        };
        builder
    }

    /// 💡️ Registers `s.stdio.semio.video.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::standards::v1::subsets::video::schema::inferences::semio_video_artifact_inference_descriptor()).expect("schema descriptor publication");
    }

    /// 🌉️ video↔mp4 / video↔avi bridge entries (W4) -- forward (writes video, reads the format) +
    /// reverse (writes the format, reads video) rows, giving all 4 IoKeys per the master plan's io
    /// architecture note. Leaked to `'static` once, matching every other stdio composer's
    /// `OnceLock<Vec<ComposerEntry>>` entries-table convention (e.g. mp4/isobmff's own subset
    /// composer).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    #[cfg(feature = "conversion-video")]
    fn bridge_entries() -> &'static [semio_framework_plugin::ComposerEntry] {
        static ENTRIES: std::sync::OnceLock<Vec<semio_framework_plugin::ComposerEntry>> = std::sync::OnceLock::new();
        ENTRIES.get_or_init(|| {
            let mut entries = Vec::new();
            #[cfg(feature = "conversion-video-mp4")]
            { entries.push(deserializer_entry_of::<SemioVideoFromMp4>()); entries.push(serializer_entry_of::<SemioVideoToMp4>()); }
            #[cfg(feature = "conversion-video-avi")]
            { entries.push(deserializer_entry_of::<SemioVideoFromAvi>()); entries.push(serializer_entry_of::<SemioVideoToAvi>()); }
            entries
        }).as_slice()
    }
    //#endregion 🔖️Register

    //#region 🧪️Tests
    #[cfg(all(test, feature = "conversion-video"))]
    include!("🧪️tests/🔬️derived-composition-unit/🦀️.rs");
    //#endregion 🧪️Tests
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
