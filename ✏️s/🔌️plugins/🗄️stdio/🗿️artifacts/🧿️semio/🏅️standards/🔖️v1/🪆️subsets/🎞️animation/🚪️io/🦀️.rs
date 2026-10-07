//! 🚪️ IO `s.stdio.semio` (v1/animation) — real cross-format bridge leaves (W4): typed
//! `ArtifactDeserializer`/`ArtifactSerializer` impls, one pair per bridged format
//! (`animation↔gltf`, `animation↔mp4`, `animation↔gif` per the master plan's io lattice). Mounted
//! here (not in `🦀️.rs`, a closer-only hot file) via `#[path=...]` relative to this file's own
//! directory. Registration flows through `🎹️composer::register`.

#[cfg(feature = "conversion-animation")]
#[path = "📥️import/🧩️deserializers/🗿️artifacts/🎞️gif/🔖️89a/✳️any/🦀️.rs"]
pub mod gif_deserializer;
#[cfg(feature = "conversion-animation")]
#[path = "📤️export/🧵️serializers/🗿️artifacts/🎞️gif/🔖️89a/✳️any/🦀️.rs"]
pub mod gif_serializer;
#[cfg(feature = "conversion-animation")]
#[path = "📥️import/🧩️deserializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️.rs"]
pub mod gltf_deserializer;
#[cfg(feature = "conversion-animation")]
#[path = "📤️export/🧵️serializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️.rs"]
pub mod gltf_serializer;
#[cfg(feature = "conversion-animation")]
#[path = "📥️import/🧩️deserializers/🗿️artifacts/🎥️mp4/🔖️isobmff/✳️any/🦀️.rs"]
pub mod mp4_deserializer;
#[cfg(feature = "conversion-animation")]
#[path = "📤️export/🧵️serializers/🗿️artifacts/🎥️mp4/🔖️isobmff/✳️any/🦀️.rs"]
pub mod mp4_serializer;
//#region 🎹️DerivedComposition
pub mod derived_composition {
    #[cfg(feature = "conversion-animation")]
    use crate::standards::v1::subsets::animation::io::{
        gif_deserializer::SemioAnimationFromGif, gif_serializer::SemioAnimationToGif, gltf_deserializer::SemioAnimationFromGltf, gltf_serializer::SemioAnimationToGltf, mp4_deserializer::SemioAnimationFromMp4, mp4_serializer::SemioAnimationToMp4,
    };
    use crate::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot;
    use crate::standards::v1::subsets::animation::io::SemioAnimationAnalyzer;
    #[cfg(feature = "conversion-animation")]
    use semio_framework_plugin::{deserializer_entry_of, register_composer_entries, serializer_entry_of, ComposerEntry};
    use {semio_framework_plugin::register_subset_validator,semio_framework_plugin::subset_validator_entry_of,semio_framework_plugin::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::ComposeError,semio_framework_plugin::ComposeSource,semio_framework_plugin::Composition,semio_framework_artifact_reference::Dialect,semio_framework_plugin::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_plugin::SubsetValidator,semio_framework_plugin::SubsetValidatorEntry};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("animation") };

    //#region 🔖️Composer
    pub struct SemioAnimationComposerComposition;

    impl ArtifactComposition for SemioAnimationComposerComposition {
        type Snapshot = SemioAnimationSnapshot;
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
                return Err(ComposeError { message: "SemioAnimationComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioAnimationAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "SemioAnimationComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ Decodes the payload, then runs real structural invariants gltf's own animation spec
    /// requires: every channel's `keyframes` must be non-empty and non-decreasing in `t` (glTF 2.0
    /// §5.20.1 `sampler.input` accessor — "the values MUST be non-decreasing"; gap-free coverage isn't
    /// spec-required so overlapping/duplicate `t` values are only flagged, not an error).
    pub struct SemioAnimationValidator;

    /// 🔍️ Real referential-invariant sweep over a decoded snapshot — separated from `validate` so both
    /// the registered `SubsetValidator` and this module's own tests exercise the exact same logic.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn check_semio_animation_invariants(snapshot: &SemioAnimationSnapshot) -> Vec<semio_framework_diagnostic::Diagnostic> {
        let mut diagnostics = Vec::new();
        for (ti, timeline) in snapshot.timelines.iter().enumerate() {
            for (ci, channel) in timeline.channels.iter().enumerate() {
                if channel.keyframes.is_empty() {
                    diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.semio_animation.empty-channel", semio_framework_diagnostic::TextSpan::at(1, 1), format!("timeline[{ti}] channel[{ci}] (node {:?}) has zero keyframes", channel.target.node)));
                    continue;
                }
                for w in channel.keyframes.windows(2) {
                    if w[1].t < w[0].t {
                        diagnostics.push(semio_framework_diagnostic::Diagnostic::error(
                            "stdio.semio_animation.non-monotonic-keyframes",
                            semio_framework_diagnostic::TextSpan::at(1, 1),
                            format!("timeline[{ti}] channel[{ci}] (node {:?}): keyframe t must be non-decreasing, got {} after {}", channel.target.node, w[1].t, w[0].t),
                        ));
                    }
                }
            }
        }
        diagnostics
    }

    impl SubsetValidator for SemioAnimationValidator {
        const DIALECT: Dialect = DIALECT;
        async fn validate(payload: &IoPayload) -> Vec<semio_framework_diagnostic::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioAnimationSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioAnimationSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_semio_animation_invariants(&snapshot),
                None => vec![semio_framework_diagnostic::Diagnostic::error("stdio.semio_animation.validate-decode-failed", semio_framework_diagnostic::TextSpan::at(1, 1), "SemioAnimationValidator: payload did not decode as a SemioAnimationSnapshot".to_string())],
            }
        }
    }

    static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioAnimationValidator>)
    }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec, and SubsetValidator. Called from
    /// this artifact's standard-level `engine::register()`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        ::semio_framework_schema_registry::register_artifact_schema_descriptor(crate::standards::v1::subsets::animation::schema::semio_animation_artifact_schema_descriptor()).expect("schema descriptor publication");
        semio_framework_plugin::io::register_native_snapshot_codec(semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.semio", standard: semio_framework_artifact_reference::StandardId("v1"), subset: semio_framework_artifact_reference::SubsetId("animation") }, store::ArtifactCodec::of::<SemioAnimationSnapshot, crate::standards::v1::subsets::animation::schema::mutations::SemioAnimationMutation>(
            crate::standards::v1::subsets::animation::schema::snapshot::STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA,
        ))
        .expect("static Stdio registration must be available and conflict-free");
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
        #[cfg(feature = "conversion-animation")]
        register_composer_entries(bridge_entries()).expect("static Stdio registration must be available and conflict-free");
        register_artifact_inferences();
    }

    /// 🧾️ The declarative twin of [`register`]: this subset's schema, document codec, `SubsetValidator`, composers
    /// (those writing semio, [`crate::semio_written`]) and inference descriptor as rows of [`crate::declaration`].
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn declare(builder: semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady>) -> semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady> {
        let builder = builder
            .schemas([crate::standards::v1::subsets::animation::schema::semio_animation_artifact_schema_descriptor()])
            .document_codec_bare::<SemioAnimationSnapshot, crate::standards::v1::subsets::animation::schema::mutations::SemioAnimationMutation>(crate::standards::v1::subsets::animation::schema::snapshot::STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.semio", standard: semio_framework_artifact_reference::StandardId("v1"), subset: semio_framework_artifact_reference::SubsetId("animation") })
            .subset_validators(std::slice::from_ref(validator_entry()))
            .inferences([crate::standards::v1::subsets::animation::schema::inferences::semio_animation_artifact_inference_descriptor()]);
        #[cfg(feature = "conversion-animation")]
        let builder = {
            static COMPOSERS: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
            builder.composers(crate::semio_written(bridge_entries(), &COMPOSERS))
        };
        builder
    }

    /// 💡️ Registers `s.stdio.semio.animation.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::standards::v1::subsets::animation::schema::inferences::semio_animation_artifact_inference_descriptor()).expect("schema descriptor publication");
    }

    /// 🌉️ animation↔gltf / animation↔mp4 / animation↔gif bridge entries (W4) -- forward + reverse rows
    /// per pair, giving all 4 IoKeys per pair per the master plan's io architecture note.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    #[cfg(feature = "conversion-animation")]
    fn bridge_entries() -> &'static [ComposerEntry] {
        static ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
        ENTRIES
            .get_or_init(|| {
                vec![
                    deserializer_entry_of::<SemioAnimationFromGltf>(),
                    serializer_entry_of::<SemioAnimationToGltf>(),
                    deserializer_entry_of::<SemioAnimationFromMp4>(),
                    serializer_entry_of::<SemioAnimationToMp4>(),
                    deserializer_entry_of::<SemioAnimationFromGif>(),
                    serializer_entry_of::<SemioAnimationToGif>(),
                ]
            })
            .as_slice()
    }
    //#endregion 🔖️Register

    //#region 🔖️Tests
    #[cfg(all(test, feature = "conversion-animation"))]
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
    use crate::standards::v1::subsets::animation::schema::diff::SemioAnimationDiff;
    use crate::standards::v1::subsets::animation::schema::mutations::{apply_semio_animation_mutation, SemioAnimationMutation};
    use crate::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct SemioAnimationBuilderConstruction {
        snapshot: SemioAnimationSnapshot,
    }

    impl ArtifactBuilder for SemioAnimationBuilderConstruction {
        type Snapshot = SemioAnimationSnapshot;
        type Mutation = SemioAnimationMutation;
        type Diff = SemioAnimationDiff;
        fn empty() -> Self {
            Self { snapshot: SemioAnimationSnapshot::default() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<SemioAnimationSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioAnimationSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_semio_animation_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <SemioAnimationDiff as protocol::MutationDiff<SemioAnimationSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            Ok(self.snapshot)
        }
    }
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::standards::v1::subsets::animation::schema::snapshot::{SemioAnimationSnapshot, STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA};
    use {semio_framework_plugin::Analysis,semio_framework_plugin::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_plugin::IoConfidence,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct SemioAnimationParts {
        pub snapshot: Option<SemioAnimationSnapshot>,
    }

    pub struct SemioAnimationAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioAnimationAnalyzerAnalysis {
        type Parts = SemioAnimationParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("animation") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioAnimationParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioAnimationSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioAnimationSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec SemioAnimationBuilderFacets {
        construction: SemioAnimationBuilderConstruction,
        analysis: SemioAnimationAnalyzerAnalysis,
        composition: crate::standards::v1::subsets::animation::io::derived_composition::SemioAnimationComposerComposition,
    }
    builder: SemioAnimationBuilder,
    analyzer: SemioAnimationAnalyzer,
    composer: SemioAnimationComposer,
);
