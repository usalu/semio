//! 🚪️ IO `s.stdio.semio` (v1/animation) — real cross-format bridge leaves (W4): typed
//! `ArtifactDeserializer`/`ArtifactSerializer` impls, one pair per bridged format
//! (`animation↔gltf`, `animation↔mp4`, `animation↔gif` per the master plan's io lattice). Mounted
//! here (not in `📦️glue.rs`, a closer-only hot file) via `#[path=...]` relative to this file's own
//! directory. Registration flows through `🎹️composer::register`.

#[path = "📥️import/🧩️deserializers/🗿️artifacts/🎞️gif/🔖️89a/✳️any/🦀️component.rs"]
pub mod gif_deserializer;
#[path = "📤️export/🧵️serializers/🗿️artifacts/🎞️gif/🔖️89a/✳️any/🦀️component.rs"]
pub mod gif_serializer;
#[path = "📥️import/🧩️deserializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️component.rs"]
pub mod gltf_deserializer;
#[path = "📤️export/🧵️serializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️component.rs"]
pub mod gltf_serializer;
#[path = "📥️import/🧩️deserializers/🗿️artifacts/🎥️mp4/🔖️isobmff/✳️any/🦀️component.rs"]
pub mod mp4_deserializer;
#[path = "📤️export/🧵️serializers/🗿️artifacts/🎥️mp4/🔖️isobmff/✳️any/🦀️component.rs"]
pub mod mp4_serializer;
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::semio::standards::v1::subsets::animation::io::{
        gif_deserializer::SemioAnimationFromGif, gif_serializer::SemioAnimationToGif, gltf_deserializer::SemioAnimationFromGltf, gltf_serializer::SemioAnimationToGltf, mp4_deserializer::SemioAnimationFromMp4, mp4_serializer::SemioAnimationToMp4,
    };
    use crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot;
    use crate::artifacts::semio::standards::v1::subsets::animation::schema::SemioAnimationAnalyzer;
    use semio_framework_plugin::{
        deserializer_entry_of, register_composer_entries, register_subset_validator, serializer_entry_of, subset_validator_entry_of, AnalyzeSource, ArtifactAnalyzer as _, ArtifactComposition, ComposeError, ComposeSource, ComposerEntry, Composition,
        Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry,
    };

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("animation") };

    //#region 🔖️Composer
    pub struct SemioAnimationComposerComposition;

    impl ArtifactComposition for SemioAnimationComposerComposition {
        type Snapshot = SemioAnimationSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT]
        }

        fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
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
        fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
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
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioAnimationValidator>)
    }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec, and SubsetValidator. Called from
    /// this artifact's standard-level `engine::register()`.
    pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::animation::schema::semio_animation_artifact_schema_descriptor());
        store::register_document_codec(store::ArtifactCodec::of::<SemioAnimationSnapshot, crate::artifacts::semio::standards::v1::subsets::animation::schema::mutations::SemioAnimationMutation>(
            crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA,
        ));
        register_subset_validator(validator_entry());
        register_composer_entries(bridge_entries());
        register_artifact_inferences();
    }

    /// 💡️ Registers `s.stdio.semio.animation.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    pub fn register_artifact_inferences() {
        ::schema::register_artifact_inference_descriptor(crate::artifacts::semio::standards::v1::subsets::animation::schema::inferences::semio_animation_artifact_inference_descriptor());
    }

    /// 🌉️ animation↔gltf / animation↔mp4 / animation↔gif bridge entries (W4) -- forward + reverse rows
    /// per pair, giving all 4 IoKeys per pair per the master plan's io architecture note.
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
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::{AnimChannel, AnimKeyframe, AnimTarget, AnimTargetProperty, AnimTimeline, AnimValue};

        fn snapshot_with_channel(keyframes: Vec<AnimKeyframe>) -> SemioAnimationSnapshot {
            SemioAnimationSnapshot {
                timelines: vec![AnimTimeline { name: None, channels: vec![AnimChannel { target: AnimTarget { node: "n".into(), property: AnimTargetProperty::Translation }, interpolation: Default::default(), keyframes }] }],
                ..SemioAnimationSnapshot::default()
            }
        }

        #[test]
        fn empty_channel_is_flagged() {
            let snap = snapshot_with_channel(vec![]);
            let diagnostics = check_semio_animation_invariants(&snap);
            assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_animation.empty-channel"), "got {diagnostics:?}");
        }

        #[test]
        fn non_monotonic_keyframes_are_flagged() {
            let snap = snapshot_with_channel(vec![AnimKeyframe { t: 1.0, value: AnimValue::Scalar { value: 0.0 } }, AnimKeyframe { t: 0.0, value: AnimValue::Scalar { value: 1.0 } }]);
            let diagnostics = check_semio_animation_invariants(&snap);
            assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_animation.non-monotonic-keyframes"), "got {diagnostics:?}");
        }

        #[test]
        fn well_formed_snapshot_has_no_diagnostics() {
            let snap = snapshot_with_channel(vec![AnimKeyframe { t: 0.0, value: AnimValue::Scalar { value: 0.0 } }, AnimKeyframe { t: 1.0, value: AnimValue::Scalar { value: 1.0 } }]);
            assert!(check_semio_animation_invariants(&snap).is_empty());
        }

        #[test]
        fn registered_validator_matches_direct_invariant_check_on_a_binary_payload() {
            let snap = snapshot_with_channel(vec![]);
            let bytes = <SemioAnimationSnapshot as store::ArtifactPack>::encode_pack(&snap);
            let via_validator = SemioAnimationValidator::validate(&IoPayload::Binary(bytes));
            assert!(via_validator.iter().any(|d| d.code.0 == "stdio.semio_animation.empty-channel"), "got {via_validator:?}");
        }

        //#region 🔖️ConformanceLaws
        /// 🧪️ ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION animation wave — the 6
        /// conformance-law tests every semio wave lands (`📖️grammar-recipe.md` §4), proving the real
        /// hand-rolled grammar/protocol files actually recognize/walk this facet's own real codec
        /// output. Lives here (not the shared 14-subset `🏅️standards/🔖️v1/⚙️engine/🦀️component.rs`
        /// aggregator, which has no test module of its own and is out of this wave's `✳️animation/`-only
        /// edit scope) — same home every prior semio wave's report identifies as correct.
        mod conformance_laws {
            use super::*;
            use crate::artifacts::semio::standards::v1::subsets::animation::schema::{diff, mutations, snapshot};
            use protocol::{DiffCodec, OpBinary, OpText};

            /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
            /// parse under the real dialect — independent of, and cheaper than, the two `recognize`/
            /// `walk_protocol` laws below.
            #[test]
            fn committed_facet_files_parse() {
                for (label, text) in [("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO), ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO), ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO)] {
                    let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
                    assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
                }
                for (label, text) in [("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO), ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO), ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO)] {
                    dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
                }
            }

            /// ✅️ `grammar_conformance_law`: the snapshot grammar recognizes real `print_dsl` output
            /// for the demo snapshot — same preamble-stripped body reconstruction the eventual
            /// `m5_handcrafted_grammar_conformance` harness uses (envelope id prepended as the bare
            /// `artifact-mark` token), so this is a direct proof this facet will pass that harness once
            /// graduated.
            #[test]
            fn grammar_conformance_law() {
                let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
                let recognizer = dsl::Recognizer::compile(&grammar);
                let text = store::ArtifactDsl::print_dsl(&snapshot::demo_animation_snapshot());
                let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
                let reconstructed = format!("{}\n{body}", envelope.envelope_id());
                assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
            }

            /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
            /// output for every `SemioAnimationMutation` variant (`mutations::demo_mutation_cases()`).
            #[test]
            fn ops_grammar_conformance_law() {
                let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
                let recognizer = dsl::Recognizer::compile(&grammar);
                for mutation in mutations::demo_mutation_cases() {
                    let printed = mutation.print_op();
                    assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
                }
            }

            /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output
            /// for every representative `SemioAnimationDiff` (`diff::demo_diff_cases()`), incl. the
            /// empty (no-op) diff.
            #[test]
            fn diff_grammar_conformance_law() {
                let grammar = dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
                let recognizer = dsl::Recognizer::compile(&grammar);
                for d in diff::demo_diff_cases() {
                    let printed = d.print_diff();
                    assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
                }
            }

            /// ✅️ `protocol_walk_law`: `walk_protocol` against REAL bytes for all three facets —
            /// snapshot pack (`encode_pack`, envelope-unwrapped first), every demo mutation's
            /// `encode_op`, and every demo diff's `encode_diff` — asserting `consumed == bytes.len()`.
            #[test]
            fn protocol_walk_law() {
                let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
                let packed = store::ArtifactPack::encode_pack(&snapshot::demo_animation_snapshot());
                let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
                let trace = dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, inner.len(), "pack walk did not consume every byte");

                let op_spec = dsl::parse_protocol(mutations::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
                for mutation in mutations::demo_mutation_cases() {
                    let bytes = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
                    let trace = dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
                    assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
                }

                let diff_spec = dsl::parse_protocol(diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
                for d in diff::demo_diff_cases() {
                    let bytes = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
                    let trace = dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
                    assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
                }
            }

            /// ✅️ `fixture_honesty_law`: the shipped `.dsl.semio`/`.pack.semio` fixtures are GENUINE
            /// `print_dsl`/`encode_pack` output of `snapshot::demo_animation_snapshot()` —
            /// `parse_dsl(fixture) == demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the
            /// pack twin — so the fixtures can never silently drift back to a fake.
            #[test]
            fn fixture_honesty_law() {
                const FIXTURE_DSL: &str = include_str!("../../✳️any/📚️examples/🚶️walk/🖼️assets/🗣️example.dsl.semio");
                const FIXTURE_PACK: &[u8] = include_bytes!("../../✳️any/📚️examples/🚶️walk/🖼️assets/🎒️example.pack.semio");

                let demo = snapshot::demo_animation_snapshot();

                let parsed = <snapshot::SemioAnimationSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
                assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_animation_snapshot()");
                assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_animation_snapshot()) drifted from the shipped .dsl.semio fixture");

                let decoded = <snapshot::SemioAnimationSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
                assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_animation_snapshot()");
                assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_animation_snapshot()) drifted from the shipped .pack.semio fixture");
            }
        }
        //#endregion 🔖️ConformanceLaws
    }
    //#endregion 🔖️Tests
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
