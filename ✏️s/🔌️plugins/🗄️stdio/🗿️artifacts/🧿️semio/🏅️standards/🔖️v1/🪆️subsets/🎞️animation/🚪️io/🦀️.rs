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
    use crate::standards::v1::subsets::animation::schema::SemioAnimationAnalyzer;
    #[cfg(feature = "conversion-animation")]
    use semio_framework_plugin::{deserializer_entry_of, register_composer_entries, serializer_entry_of, ComposerEntry};
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};

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
    fn check_semio_animation_invariants(snapshot: &SemioAnimationSnapshot) -> Vec<dsl::Diagnostic> {
        let mut diagnostics = Vec::new();
        for (ti, timeline) in snapshot.timelines.iter().enumerate() {
            for (ci, channel) in timeline.channels.iter().enumerate() {
                if channel.keyframes.is_empty() {
                    diagnostics.push(dsl::Diagnostic::error("stdio.semio_animation.empty-channel", dsl::TextSpan::at(1, 1), format!("timeline[{ti}] channel[{ci}] (node {:?}) has zero keyframes", channel.target.node)));
                    continue;
                }
                for w in channel.keyframes.windows(2) {
                    if w[1].t < w[0].t {
                        diagnostics.push(dsl::Diagnostic::error(
                            "stdio.semio_animation.non-monotonic-keyframes",
                            dsl::TextSpan::at(1, 1),
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
        async fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioAnimationSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioAnimationSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_semio_animation_invariants(&snapshot),
                None => vec![dsl::Diagnostic::error("stdio.semio_animation.validate-decode-failed", dsl::TextSpan::at(1, 1), "SemioAnimationValidator: payload did not decode as a SemioAnimationSnapshot".to_string())],
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
        ::framework_schema::register_artifact_schema_descriptor(crate::standards::v1::subsets::animation::schema::semio_animation_artifact_schema_descriptor());
        store::register_document_codec(store::ArtifactCodec::of::<SemioAnimationSnapshot, crate::standards::v1::subsets::animation::schema::mutations::SemioAnimationMutation>(
            crate::standards::v1::subsets::animation::schema::snapshot::STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA,
        ))
        .expect("static Stdio registration must be available and conflict-free");
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
        #[cfg(feature = "conversion-animation")]
        register_composer_entries(bridge_entries()).expect("static Stdio registration must be available and conflict-free");
        register_artifact_inferences();
    }

    /// 💡️ Registers `s.stdio.semio.animation.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::framework_schema::register_artifact_inference_descriptor(crate::standards::v1::subsets::animation::schema::inferences::semio_animation_artifact_inference_descriptor());
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
