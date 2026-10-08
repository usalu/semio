//! 🚪️ IO `s.stdio.semio` (v1/audio) — real cross-format bridge leaves (W4): typed
//! `ArtifactDeserializer`/`ArtifactSerializer` impls, one pair per bridged format
//! (`audio↔mp3`, `audio↔wav` per the master plan's io lattice). Mounted here (not in
//! `🦀️.rs`, a closer-only hot file) via `#[path=...]` relative to this file's own directory.
//! Registration flows through `🎹️composer::register`.

#[cfg(feature = "conversion-audio")]
#[path = "📥️import/🧩️deserializers/🗿️artifacts/🎵️mp3/🔖️mpeg1-layer3/✳️any/🦀️.rs"]
pub mod mp3_deserializer;
#[cfg(feature = "conversion-audio")]
#[path = "📤️export/🧵️serializers/🗿️artifacts/🎵️mp3/🔖️mpeg1-layer3/✳️any/🦀️.rs"]
pub mod mp3_serializer;
#[cfg(feature = "conversion-audio")]
#[path = "📥️import/🧩️deserializers/🗿️artifacts/🔊️wav/🔖️riff-pcm/✳️any/🦀️.rs"]
pub mod wav_deserializer;
#[cfg(feature = "conversion-audio")]
#[path = "📤️export/🧵️serializers/🗿️artifacts/🔊️wav/🔖️riff-pcm/✳️any/🦀️.rs"]
pub mod wav_serializer;
//#region 🎹️DerivedComposition
pub mod derived_composition {
    #[cfg(feature = "conversion-audio")]
    use crate::standards::v1::subsets::audio::io::{mp3_deserializer::SemioAudioFromMp3, mp3_serializer::SemioAudioToMp3, wav_deserializer::SemioAudioFromWav, wav_serializer::SemioAudioToWav};
    use crate::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot;
    use crate::standards::v1::subsets::audio::io::SemioAudioAnalyzer;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultScope;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    #[cfg(feature = "conversion-audio")]
    use semio_framework_plugin::{deserializer_entry_of, register_composer_entries, serializer_entry_of, ComposerEntry};
    use {semio_framework_plugin::register_subset_validator,semio_framework_plugin::subset_validator_entry_of,semio_framework_plugin::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::ComposeError,semio_framework_plugin::ComposeSource,semio_framework_plugin::Composition,semio_framework_artifact_reference::Dialect,semio_framework_plugin::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_plugin::SubsetValidator,semio_framework_plugin::SubsetValidatorEntry};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("audio") };

    //#region 🔖️Composer
    pub struct SemioAudioComposerComposition;

    impl ArtifactComposition for SemioAudioComposerComposition {
        type Snapshot = SemioAudioSnapshot;
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
                return Err(ComposeError { message: "SemioAudioComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioAudioAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "SemioAudioComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            let mut diagnostics = analysis.diagnostics;
            diagnostics.extend(check_semio_audio_invariants(&snapshot));
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️Invariants
    /// 🛡️ Real referential/structural invariant checks over a decoded `SemioAudioSnapshot` — backs
    /// both the composer's advisory diagnostics above and the registered `SubsetValidator` below (same
    /// function, two call sites, matching pdf/a's `check_pdf_a_conformance` precedent). None of these
    /// are hard compose-failures (audio has no PDF/A-style conformance gate) — every finding is
    /// advisory, surfaced as a real `Diagnostic` rather than silently dropped.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_semio_audio_invariants(snapshot: &SemioAudioSnapshot) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        if snapshot.sample_rate == 0 {
            diagnostics.push(warning("stdio.semio_audio.zero-sample-rate", "sample_rate is 0 -- no real audio can play back at this rate".to_string()));
        }
        if let Some(first) = snapshot.channels.first() {
            let expected = first.samples.len();
            for (i, channel) in snapshot.channels.iter().enumerate().skip(1) {
                if channel.samples.len() != expected {
                    diagnostics.push(warning("stdio.semio_audio.channel-length-mismatch", format!("channel {i} has {} samples, channel 0 has {expected} -- channels are expected to be the same length", channel.samples.len())));
                }
            }
        }
        for (i, tag) in snapshot.tags.iter().enumerate() {
            if tag.key.is_empty() {
                diagnostics.push(warning("stdio.semio_audio.empty-tag-key", format!("tag {i} has an empty key")));
            }
        }
        diagnostics
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn warning(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: semio_framework_diagnostic::FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }
    //#endregion 🔖️Invariants

    //#region 🔖️SubsetValidator
    /// 🛡️ The registered `SubsetValidator` for `s.stdio.semio/v1/audio` — decodes the wire payload as
    /// this subset's own snapshot and re-runs the SAME `check_semio_audio_invariants` the composer
    /// runs pre-serialization (matching pdf/a's own composer/validator split).
    pub struct SemioAudioValidator;

    impl SubsetValidator for SemioAudioValidator {
        const DIALECT: Dialect = DIALECT;
        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioAudioSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioAudioSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_semio_audio_invariants(&snapshot),
                None => vec![Diagnostic {
                    code: semio_framework_diagnostic::FaultCode::new("stdio.semio_audio.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "SemioAudioValidator: payload did not decode as a SemioAudioSnapshot -- skipped".into(),
                    expected: None,
                    scope: FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioAudioValidator>)
    }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec (`"s.stdio.semio.audio"` — the
    /// document-codec id, repo-wide unique per the ticket's static policy check, distinct from every
    /// other artifact's own document schema string), and `SubsetValidator`. Called from this
    /// artifact's standard-level `engine::register()`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        ::semio_framework_schema_registry::register_artifact_schema_descriptor(crate::standards::v1::subsets::audio::schema::semio_audio_artifact_schema_descriptor()).expect("schema descriptor publication");
        semio_framework_plugin::io::register_native_snapshot_codec(semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.semio", standard: semio_framework_artifact_reference::StandardId("v1"), subset: semio_framework_artifact_reference::SubsetId("audio") }, store::ArtifactCodec::of::<SemioAudioSnapshot, crate::standards::v1::subsets::audio::schema::mutations::SemioAudioMutation>(
            crate::standards::v1::subsets::audio::schema::snapshot::STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA,
        ))
        .expect("static Stdio registration must be available and conflict-free");
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
        #[cfg(feature = "conversion-audio")]
        register_composer_entries(bridge_entries()).expect("static Stdio registration must be available and conflict-free");
        register_artifact_inferences();
    }

    /// 🧾️ The declarative twin of [`register`]: this subset's schema, document codec, `SubsetValidator`, composers
    /// (those writing semio, [`crate::semio_written`]) and inference descriptor as rows of [`crate::declaration`].
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn declare(builder: semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady>) -> semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady> {
        let builder = builder
            .schemas([crate::standards::v1::subsets::audio::schema::semio_audio_artifact_schema_descriptor()])
            .document_codec_bare::<SemioAudioSnapshot, crate::standards::v1::subsets::audio::schema::mutations::SemioAudioMutation>(crate::standards::v1::subsets::audio::schema::snapshot::STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.semio", standard: semio_framework_artifact_reference::StandardId("v1"), subset: semio_framework_artifact_reference::SubsetId("audio") })
            .subset_validators(std::slice::from_ref(validator_entry()))
            .inferences([crate::standards::v1::subsets::audio::schema::inferences::semio_audio_artifact_inference_descriptor()]);
        #[cfg(feature = "conversion-audio")]
        let builder = {
            static COMPOSERS: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
            builder.composers(crate::semio_written(bridge_entries(), &COMPOSERS))
        };
        builder
    }

    /// 💡️ Registers `s.stdio.semio.audio.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::standards::v1::subsets::audio::schema::inferences::semio_audio_artifact_inference_descriptor()).expect("schema descriptor publication");
    }

    /// 🌉️ audio↔mp3 / audio↔wav bridge entries (W4) -- forward + reverse rows per pair, giving all 4
    /// IoKeys per pair per the master plan's io architecture note.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    #[cfg(feature = "conversion-audio")]
    fn bridge_entries() -> &'static [ComposerEntry] {
        static ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
        ENTRIES.get_or_init(|| vec![deserializer_entry_of::<SemioAudioFromMp3>(), serializer_entry_of::<SemioAudioToMp3>(), deserializer_entry_of::<SemioAudioFromWav>(), serializer_entry_of::<SemioAudioToWav>()]).as_slice()
    }
    //#endregion 🔖️Register

    //#region 🔖️Tests
    #[cfg(all(test, feature = "conversion-audio"))]
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
    use crate::standards::v1::subsets::audio::schema::diff::SemioAudioDiff;
    use crate::standards::v1::subsets::audio::schema::mutations::{SemioAudioMutation};
    #[cfg(test)]
    use crate::standards::v1::subsets::audio::schema::mutations::{insert_channel, set_format, set_sample_rate};
    use crate::standards::v1::subsets::audio::schema::snapshot::{SemioAudioChannel, SemioAudioFormat, SemioAudioSnapshot, SemioAudioTag};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct SemioAudioBuilderConstruction {
        snapshot: SemioAudioSnapshot,
    }

    //#region 🔖️TypedConstructors
    impl SemioAudioBuilderConstruction {
        /// 🏗️ Starts a fresh document at the given sample rate/format.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn new(sample_rate: u32, format: SemioAudioFormat) -> Self {
            Self { snapshot: SemioAudioSnapshot { sample_rate, format, ..SemioAudioSnapshot::default() } }
        }
        /// 🏗️ Appends one channel's decoded samples, in channel order.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_channel(mut self, channel: SemioAudioChannel) -> Self {
            self.snapshot.channels.push(channel);
            self
        }
        /// 🏗️ Appends one metadata key/value pair.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_tag(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
            self.snapshot.tags.push(SemioAudioTag { key: key.into(), value: value.into() });
            self
        }
        /// 🏗️ Sets the sample rate.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn set_sample_rate(mut self, sample_rate: u32) -> Self {
            self.snapshot.sample_rate = sample_rate;
            self
        }
        /// 🏗️ Sets the original-encoding sample format.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn set_format(mut self, format: SemioAudioFormat) -> Self {
            self.snapshot.format = format;
            self
        }
    }
    //#endregion 🔖️TypedConstructors

    impl ArtifactBuilder for SemioAudioBuilderConstruction {
        type Snapshot = SemioAudioSnapshot;
        type Mutation = SemioAudioMutation;
        type Diff = SemioAudioDiff;
        fn empty() -> Self {
            Self { snapshot: SemioAudioSnapshot::default() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<SemioAudioSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioAudioSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <SemioAudioMutation as protocol::Mutation<SemioAudioSnapshot>>::diff(&mutation, &self.snapshot);
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
    //#endregion 🔖️Builder

    //#region 🔖️Tests
    #[cfg(test)]
    include!("../🧬️schema/🧪️tests/🔬️derived-construction-unit/🦀️.rs");
    //#endregion 🔖️Tests
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::standards::v1::subsets::audio::schema::snapshot::{SemioAudioSnapshot, STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA};
    use {semio_framework_plugin::Analysis,semio_framework_plugin::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_plugin::IoConfidence,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    //#region 🔖️Parts
    #[derive(Clone, Debug, Default)]
    pub struct SemioAudioParts {
        pub snapshot: Option<SemioAudioSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    pub struct SemioAudioAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioAudioAnalyzerAnalysis {
        type Parts = SemioAudioParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("audio") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioAudioParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioAudioSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioAudioSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    //#endregion 🔖️Analyzer

    //#region 🔖️Tests
    #[cfg(test)]
    include!("../🧬️schema/🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
    //#endregion 🔖️Tests
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec SemioAudioBuilderFacets {
        construction: SemioAudioBuilderConstruction,
        analysis: SemioAudioAnalyzerAnalysis,
        composition: crate::standards::v1::subsets::audio::io::derived_composition::SemioAudioComposerComposition,
    }
    builder: SemioAudioBuilder,
    analyzer: SemioAudioAnalyzer,
    composer: SemioAudioComposer,
);
