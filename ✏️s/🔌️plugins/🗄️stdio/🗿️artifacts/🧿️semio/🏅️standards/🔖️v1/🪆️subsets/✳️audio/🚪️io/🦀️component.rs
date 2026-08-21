//! 🚪️ IO `s.stdio.semio` (v1/audio) — real cross-format bridge leaves (W4): typed
//! `ArtifactDeserializer`/`ArtifactSerializer` impls, one pair per bridged format
//! (`audio↔mp3`, `audio↔wav` per the master plan's io lattice). Mounted here (not in
//! `📦️glue.rs`, a closer-only hot file) via `#[path=...]` relative to this file's own directory.
//! Registration flows through `🎹️composer::register`.

#[path = "📥️import/🧩️deserializers/🗿️artifacts/🎵️mp3/🔖️mpeg1-layer3/✳️any/🦀️component.rs"]
pub mod mp3_deserializer;
#[path = "📤️export/🧵️serializers/🗿️artifacts/🎵️mp3/🔖️mpeg1-layer3/✳️any/🦀️component.rs"]
pub mod mp3_serializer;
#[path = "📥️import/🧩️deserializers/🗿️artifacts/🔊️wav/🔖️riff-pcm/✳️any/🦀️component.rs"]
pub mod wav_deserializer;
#[path = "📤️export/🧵️serializers/🗿️artifacts/🔊️wav/🔖️riff-pcm/✳️any/🦀️component.rs"]
pub mod wav_serializer;
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::semio::standards::v1::subsets::audio::io::{mp3_deserializer::SemioAudioFromMp3, mp3_serializer::SemioAudioToMp3, wav_deserializer::SemioAudioFromWav, wav_serializer::SemioAudioToWav};
    use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot;
    use crate::artifacts::semio::standards::v1::subsets::audio::schema::SemioAudioAnalyzer;
    use dsl::{Diagnostic, FaultScope, Severity, TextSpan};
    use semio_framework_plugin::{
        deserializer_entry_of, register_composer_entries, register_subset_validator, serializer_entry_of, subset_validator_entry_of, AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, ComposerEntry, Composition,
        Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry,
    };

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("audio") };

    //#region 🔖️Composer
    pub struct SemioAudioComposerComposition;

    impl ArtifactComposition for SemioAudioComposerComposition {
        type Snapshot = SemioAudioSnapshot;
        const WRITES: Dialect = DIALECT;

        async fn reads() -> &'static [Dialect] {
            &[DIALECT]
        }

        async fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
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
            let analysis = SemioAudioAnalyzer::analyze(&native).await;
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
        Diagnostic { code: dsl::FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
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
                    code: dsl::FaultCode::new("stdio.semio_audio.validate-decode-failed"),
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
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::audio::schema::semio_audio_artifact_schema_descriptor());
        let _ = store::register_document_codec(store::ArtifactCodec::of::<SemioAudioSnapshot, crate::artifacts::semio::standards::v1::subsets::audio::schema::mutations::SemioAudioMutation>(
            crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA,
        ));
        let _ = register_subset_validator(validator_entry());
        let _ = register_composer_entries(bridge_entries());
        register_artifact_inferences();
    }

    /// 💡️ Registers `s.stdio.semio.audio.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::schema::register_artifact_inference_descriptor(crate::artifacts::semio::standards::v1::subsets::audio::schema::inferences::semio_audio_artifact_inference_descriptor());
    }

    /// 🌉️ audio↔mp3 / audio↔wav bridge entries (W4) -- forward + reverse rows per pair, giving all 4
    /// IoKeys per pair per the master plan's io architecture note.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn bridge_entries() -> &'static [ComposerEntry] {
        static ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
        ENTRIES.get_or_init(|| vec![deserializer_entry_of::<SemioAudioFromMp3>(), serializer_entry_of::<SemioAudioToMp3>(), deserializer_entry_of::<SemioAudioFromWav>(), serializer_entry_of::<SemioAudioToWav>()]).as_slice()
    }
    //#endregion 🔖️Register

    //#region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::{SemioAudioChannel, SemioAudioTag};

        #[semio_framework_async_macros::async_test]
        async fn compose_decodes_a_real_binary_source_with_no_advisories() {
            let snapshot = SemioAudioSnapshot {
                sample_rate: 44_100,
                channels: vec![SemioAudioChannel { samples: vec![0.0, 1.0] }, SemioAudioChannel { samples: vec![0.0, -1.0] }],
                tags: vec![SemioAudioTag { key: "title".into(), value: "clean".into() }],
                ..SemioAudioSnapshot::default()
            };
            let bytes = <SemioAudioSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
            let sources = vec![ComposeSource { dialect: DIALECT, payload: AnalyzeSource::Binary(&bytes) }];
            let composed = SemioAudioComposerComposition::compose(&sources).expect("clean document must compose");
            assert_eq!(composed.snapshot, snapshot);
            assert!(composed.diagnostics.is_empty(), "got {:?}", composed.diagnostics);
        }

        #[semio_framework_async_macros::async_test]
        async fn zero_sample_rate_surfaces_a_real_warning_not_silently() {
            let snapshot = SemioAudioSnapshot { sample_rate: 0, ..SemioAudioSnapshot::default() };
            let diagnostics = check_semio_audio_invariants(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_audio.zero-sample-rate" && d.severity == Severity::Warning), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn mismatched_channel_lengths_surface_a_real_warning() {
            let snapshot = SemioAudioSnapshot { sample_rate: 44_100, channels: vec![SemioAudioChannel { samples: vec![0.0, 1.0, 2.0] }, SemioAudioChannel { samples: vec![0.0] }], ..SemioAudioSnapshot::default() };
            let diagnostics = check_semio_audio_invariants(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_audio.channel-length-mismatch"), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn empty_tag_key_surfaces_a_real_warning() {
            let snapshot = SemioAudioSnapshot { sample_rate: 44_100, tags: vec![SemioAudioTag { key: String::new(), value: "orphaned".into() }], ..SemioAudioSnapshot::default() };
            let diagnostics = check_semio_audio_invariants(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_audio.empty-tag-key"), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn subset_validator_recheck_matches_the_composer_side_invariants() {
            let snapshot = SemioAudioSnapshot { sample_rate: 0, ..SemioAudioSnapshot::default() };
            let bytes = <SemioAudioSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
            let diagnostics = SemioAudioValidator::validate(&IoPayload::Binary(bytes));
            assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_audio.zero-sample-rate"), "got {diagnostics:?}");
        }

        //#region 🔖️ConformanceLaws
        /// 🧪️ The 6 real-codec conformance-law tests (ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-
        /// REUSE-EVOLUTION's audio wave), mirroring `✳️flow`'s/`✳️mesh`'s/`✳️image`'s own proven,
        /// fully-verified template (`ws-codec-workflow-report.md`/`ws-codec-mesh-report.md`/
        /// `ws-codec-image-report.md`) — same 6 test names, same shape, only the facet modules and
        /// demo-case helpers differ.
        mod conformance_laws {
            use crate::artifacts::semio::standards::v1::subsets::audio::schema::{diff, mutations, snapshot};
            use protocol::{DiffCodec, OpBinary, OpText};

            /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
            /// parse under the real dialect — independent of, and cheaper than, the two `recognize`/
            /// `walk_protocol` laws below.
            #[semio_framework_async_macros::async_test]
            async fn committed_facet_files_parse() {
                for (label, text) in [("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO), ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO), ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO)] {
                    let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
                    assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
                }
                for (label, text) in [("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO), ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO), ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO)] {
                    dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
                }
            }

            /// ✅️ `grammar_conformance_law`: the snapshot grammar recognizes real `print_dsl` output for
            /// the demo snapshot — same preamble-stripped body reconstruction the eventual
            /// `m5_handcrafted_grammar_conformance` harness uses (envelope id prepended as the bare
            /// `artifact-mark` token), so this is a direct proof this facet will pass that harness once
            /// graduated.
            #[semio_framework_async_macros::async_test]
            async fn grammar_conformance_law() {
                let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
                let recognizer = dsl::Recognizer::compile(&grammar);
                let text = store::ArtifactDsl::print_dsl(&snapshot::demo_audio_snapshot());
                let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
                let reconstructed = format!("{}\n{body}", envelope.envelope_id());
                assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
            }

            /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op` output
            /// for every `SemioAudioMutation` variant (`mutations::demo_mutation_cases()`).
            #[semio_framework_async_macros::async_test]
            async fn ops_grammar_conformance_law() {
                let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
                let recognizer = dsl::Recognizer::compile(&grammar);
                for mutation in mutations::demo_mutation_cases() {
                    let printed = mutation.print_op();
                    assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
                }
            }

            /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output
            /// for every representative `SemioAudioDiff` (`diff::demo_diff_cases()`), incl. the empty
            /// (no-op) diff.
            #[semio_framework_async_macros::async_test]
            async fn diff_grammar_conformance_law() {
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
            #[semio_framework_async_macros::async_test]
            async fn protocol_walk_law() {
                let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
                let packed = store::ArtifactPack::encode_pack(&snapshot::demo_audio_snapshot());
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
            /// `print_dsl`/`encode_pack` output of `snapshot::demo_audio_snapshot()` —
            /// `parse_dsl(fixture) == demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the
            /// pack twin — so the fixtures can never silently drift back to a fake.
            #[semio_framework_async_macros::async_test]
            async fn fixture_honesty_law() {
                const FIXTURE_DSL: &str = include_str!("../../✳️any/📚️examples/🎵️tone/🖼️assets/🗣️example.dsl.semio");
                const FIXTURE_PACK: &[u8] = include_bytes!("../../✳️any/📚️examples/🎵️tone/🖼️assets/🎒️example.pack.semio");

                let demo = snapshot::demo_audio_snapshot();

                let parsed = <snapshot::SemioAudioSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
                assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_audio_snapshot()");
                assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_audio_snapshot()) drifted from the shipped .dsl.semio fixture");

                let decoded = <snapshot::SemioAudioSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
                assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_audio_snapshot()");
                assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_audio_snapshot()) drifted from the shipped .pack.semio fixture");
            }
        }
        //#endregion 🔖️ConformanceLaws
    }
    //#endregion 🔖️Tests
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
