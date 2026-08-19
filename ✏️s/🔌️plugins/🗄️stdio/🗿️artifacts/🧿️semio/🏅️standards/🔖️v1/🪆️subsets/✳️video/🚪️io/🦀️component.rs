//! 🚪️ IO `s.stdio.semio` (v1/video) — real cross-format bridge leaves (W4): typed
//! `ArtifactDeserializer`/`ArtifactSerializer` impls, one pair per bridged format
//! (`video↔mp4`, `video↔avi` per the master plan's io lattice). Each leaf module is mounted here
//! (not in `📦️glue.rs`, a closer-only hot file) via `#[path=...]`, resolved relative to this
//! file's own directory — the same mechanism `📦️glue.rs` itself uses one level up. Registration
//! flows through `🎹️composer::register` (see that module), matching the repo-wide convention.

#[path = "📥️import/🧩️deserializers/🗿️artifacts/📼️avi/🔖️1.0/✳️any/🦀️component.rs"]
pub mod avi_deserializer;
#[path = "📤️export/🧵️serializers/🗿️artifacts/📼️avi/🔖️1.0/✳️any/🦀️component.rs"]
pub mod avi_serializer;
#[path = "📥️import/🧩️deserializers/🗿️artifacts/🎥️mp4/🔖️isobmff/✳️any/🦀️component.rs"]
pub mod mp4_deserializer;
#[path = "📤️export/🧵️serializers/🗿️artifacts/🎥️mp4/🔖️isobmff/✳️any/🦀️component.rs"]
pub mod mp4_serializer;
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::semio::standards::v1::subsets::video::io::{avi_deserializer::SemioVideoFromAvi, avi_serializer::SemioVideoToAvi, mp4_deserializer::SemioVideoFromMp4, mp4_serializer::SemioVideoToMp4};
    use crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::{SemioVideoSnapshot, SemioVideoStreamKind};
    use crate::artifacts::semio::standards::v1::subsets::video::schema::SemioVideoAnalyzer;
    use semio_framework_plugin::{
        deserializer_entry_of, register_composer_entries, register_subset_validator, serializer_entry_of, subset_validator_entry_of, AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect,
        IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry,
    };

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("video") };

    //#region 🔖️Composer
    pub struct SemioVideoComposerComposition;

    impl ArtifactComposition for SemioVideoComposerComposition {
        type Snapshot = SemioVideoSnapshot;
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
    pub async fn check_semio_video_invariants(snapshot: &SemioVideoSnapshot) -> Vec<dsl::Diagnostic> {
        let mut out = Vec::new();
        for (stream_index, stream) in snapshot.streams.iter().enumerate() {
            if stream.rate.den == 0 {
                out.push(dsl::Diagnostic::error("stdio.semio_video.rate-zero-denominator", dsl::TextSpan::at(1, 1), format!("stream {stream_index}: rate denominator is 0 (rate.num={})", stream.rate.num)));
            }
            if stream.kind == SemioVideoStreamKind::Video && (stream.width == 0 || stream.height == 0) {
                out.push(dsl::Diagnostic::error("stdio.semio_video.video-stream-zero-dimension", dsl::TextSpan::at(1, 1), format!("stream {stream_index}: kind=Video but width={} height={}", stream.width, stream.height)));
            }
            let mut prev_pts: Option<u64> = None;
            for (sample_index, sample) in stream.samples.iter().enumerate() {
                if let Some(prev) = prev_pts {
                    if sample.pts < prev {
                        out.push(dsl::Diagnostic {
                            code: dsl::FaultCode::new("stdio.semio_video.pts-non-monotonic"),
                            severity: dsl::Severity::Warning,
                            span: dsl::TextSpan::at(1, 1),
                            message: format!("stream {stream_index} sample {sample_index}: pts {} < previous pts {prev} (allowed — decode order may legitimately differ from presentation order)", sample.pts),
                            expected: None,
                            scope: dsl::FaultScope::default(),
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
        async fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioVideoSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioVideoSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_semio_video_invariants(&snapshot),
                None => vec![dsl::Diagnostic::error("stdio.semio_video.validate-decode-failed", dsl::TextSpan::at(1, 1), "SemioVideoValidator: payload did not decode as a SemioVideoSnapshot".to_string())],
            }
        }
    }

    static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
    async fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioVideoValidator>)
    }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec (`"s.stdio.semio.video"` — the
    /// repo-wide-unique id `policyDocumentCodecDuplicateIds` checks statically), and SubsetValidator.
    /// Called from this artifact's standard-level `engine::register()`.
    pub async fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::video::schema::semio_video_artifact_schema_descriptor());
        let _ = store::register_document_codec(store::ArtifactCodec::of::<SemioVideoSnapshot, crate::artifacts::semio::standards::v1::subsets::video::schema::mutations::SemioVideoMutation>(
            crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA,
        ));
        let _ = register_subset_validator(validator_entry());
        let _ = register_composer_entries(bridge_entries());
        register_artifact_inferences();
    }

    /// 💡️ Registers `s.stdio.semio.video.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    pub async fn register_artifact_inferences() {
        ::schema::register_artifact_inference_descriptor(crate::artifacts::semio::standards::v1::subsets::video::schema::inferences::semio_video_artifact_inference_descriptor());
    }

    /// 🌉️ video↔mp4 / video↔avi bridge entries (W4) -- forward (writes video, reads the format) +
    /// reverse (writes the format, reads video) rows, giving all 4 IoKeys per the master plan's io
    /// architecture note. Leaked to `'static` once, matching every other stdio composer's
    /// `OnceLock<Vec<ComposerEntry>>` entries-table convention (e.g. mp4/isobmff's own subset
    /// composer).
    async fn bridge_entries() -> &'static [semio_framework_plugin::ComposerEntry] {
        static ENTRIES: std::sync::OnceLock<Vec<semio_framework_plugin::ComposerEntry>> = std::sync::OnceLock::new();
        ENTRIES.get_or_init(|| vec![deserializer_entry_of::<SemioVideoFromMp4>(), serializer_entry_of::<SemioVideoToMp4>(), deserializer_entry_of::<SemioVideoFromAvi>(), serializer_entry_of::<SemioVideoToAvi>()]).as_slice()
    }
    //#endregion 🔖️Register

    //#region 🧪️Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::{SemioRational, SemioVideoSample, SemioVideoStream};

        async fn clean_snapshot() -> SemioVideoSnapshot {
            SemioVideoSnapshot {
                schema: crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(),
                streams: vec![SemioVideoStream {
                    kind: SemioVideoStreamKind::Video,
                    codec: "h264".into(),
                    width: 1920,
                    height: 1080,
                    rate: SemioRational { num: 30, den: 1 },
                    samples: vec![SemioVideoSample { pts: 0, key: true, data: vec![1] }, SemioVideoSample { pts: 33, key: false, data: vec![2] }],
                }],
            }
        }

        #[test]
        async fn clean_snapshot_has_no_diagnostics() {
            let diagnostics = check_semio_video_invariants(&clean_snapshot());
            assert!(diagnostics.is_empty(), "expected no diagnostics, got {diagnostics:?}");
        }

        #[test]
        async fn zero_denominator_rate_is_a_hard_error() {
            let mut snap = clean_snapshot();
            snap.streams[0].rate.den = 0;
            let diagnostics = check_semio_video_invariants(&snap);
            assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_video.rate-zero-denominator" && d.severity == dsl::Severity::Error));
        }

        #[test]
        async fn zero_dimension_video_stream_is_a_hard_error() {
            let mut snap = clean_snapshot();
            snap.streams[0].width = 0;
            let diagnostics = check_semio_video_invariants(&snap);
            assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_video.video-stream-zero-dimension" && d.severity == dsl::Severity::Error));
        }

        #[test]
        async fn zero_dimension_non_video_stream_is_not_flagged() {
            let mut snap = clean_snapshot();
            snap.streams[0].kind = SemioVideoStreamKind::Audio;
            snap.streams[0].width = 0;
            snap.streams[0].height = 0;
            let diagnostics = check_semio_video_invariants(&snap);
            assert!(diagnostics.iter().all(|d| d.code.0 != "stdio.semio_video.video-stream-zero-dimension"));
        }

        #[test]
        async fn non_monotonic_pts_is_a_soft_warning_not_a_hard_error() {
            let mut snap = clean_snapshot();
            // sample 0 has pts=0; force sample 1's pts BELOW it (a genuine decrease, not just equal).
            snap.streams[0].samples[1].pts = 0;
            snap.streams[0].samples.push(SemioVideoSample { pts: 33, key: false, data: vec![3] });
            snap.streams[0].samples[1].pts = snap.streams[0].samples[0].pts; // equal: allowed
            snap.streams[0].samples.push(SemioVideoSample { pts: 0, key: false, data: vec![4] }); // decrease vs prev (33)
            let diagnostics = check_semio_video_invariants(&snap);
            assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_video.pts-non-monotonic" && d.severity == dsl::Severity::Warning));
            assert!(diagnostics.iter().all(|d| d.severity != dsl::Severity::Error), "non-monotonic pts must never be a hard error: {diagnostics:?}");
        }

        #[test]
        async fn subset_validator_recheck_agrees_with_direct_invariant_check() {
            let snap = clean_snapshot();
            let bytes = <SemioVideoSnapshot as store::ArtifactPack>::encode_pack(&snap);
            let diagnostics = SemioVideoValidator::validate(&IoPayload::Binary(bytes));
            assert!(diagnostics.is_empty(), "wire recheck must agree with the direct check for a clean snapshot: {diagnostics:?}");
        }

        //#region 🔖️ConformanceLaws
        /// 🧪️ Per-artifact conformance laws (grammar recipe §4 item 8) for `s.stdio.semio.video`'s
        /// three facets. Lives in this composer's own test region: video has no per-standard `⚙️engine`
        /// dir the way json/csv/zip/png do, and v1's SHARED `🏅️standards/🔖️v1/⚙️engine/🦀️component.rs`
        /// only aggregates all 14 subsets' `register()` calls (no test module of its own, and out of
        /// this ticket's `✳️video/`-only edit scope anyway) — same home flow's/mesh's/image's own
        /// waves establish.
        mod conformance_laws {
            
            use crate::artifacts::semio::standards::v1::subsets::video::schema::{diff, mutations, snapshot};
            use protocol::{DiffCodec, OpBinary, OpText};

            /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
            /// parse under the real dialect — independent of, and cheaper than, the two `recognize`/
            /// `walk_protocol` laws below.
            #[test]
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
            #[test]
            async fn grammar_conformance_law() {
                let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
                let recognizer = dsl::Recognizer::compile(&grammar);
                let text = store::ArtifactDsl::print_dsl(&snapshot::demo_video_snapshot());
                let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
                let reconstructed = format!("{}\n{body}", envelope.envelope_id());
                assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
            }

            /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op` output
            /// for every `SemioVideoMutation` variant (`mutations::demo_mutation_cases()`).
            #[test]
            async fn ops_grammar_conformance_law() {
                let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
                let recognizer = dsl::Recognizer::compile(&grammar);
                for mutation in mutations::demo_mutation_cases() {
                    let printed = mutation.print_op();
                    assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
                }
            }

            /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output
            /// for every representative `SemioVideoDiff` (`diff::demo_diff_cases()`), incl. the empty
            /// (no-op) diff.
            #[test]
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
            #[test]
            async fn protocol_walk_law() {
                let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
                let packed = store::ArtifactPack::encode_pack(&snapshot::demo_video_snapshot());
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
            /// `print_dsl`/`encode_pack` output of `snapshot::demo_video_snapshot()` —
            /// `parse_dsl(fixture) == demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the
            /// pack twin — so the fixtures can never silently drift back to a fake.
            #[test]
            async fn fixture_honesty_law() {
                const FIXTURE_DSL: &str = include_str!("../../✳️any/📚️examples/🎥️clip/🖼️assets/🗣️example.dsl.semio");
                const FIXTURE_PACK: &[u8] = include_bytes!("../../✳️any/📚️examples/🎥️clip/🖼️assets/🎒️example.pack.semio");

                let demo = snapshot::demo_video_snapshot();

                let parsed = <snapshot::SemioVideoSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
                assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_video_snapshot()");
                assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_video_snapshot()) drifted from the shipped .dsl.semio fixture");

                let decoded = <snapshot::SemioVideoSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
                assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_video_snapshot()");
                assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_video_snapshot()) drifted from the shipped .pack.semio fixture");
            }
        }
        //#endregion 🔖️ConformanceLaws
    }
    //#endregion 🧪️Tests
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
