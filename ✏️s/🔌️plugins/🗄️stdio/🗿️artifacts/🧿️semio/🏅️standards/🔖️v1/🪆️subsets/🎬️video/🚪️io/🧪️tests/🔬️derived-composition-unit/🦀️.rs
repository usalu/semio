mod tests {
    use super::*;
    use crate::standards::v1::subsets::video::schema::snapshot::{SemioRational, SemioVideoSample, SemioVideoStream};

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn clean_snapshot() -> SemioVideoSnapshot {
        SemioVideoSnapshot {
            schema: crate::standards::v1::subsets::video::schema::snapshot::STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(),
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

    #[semio_framework_async_macros::async_test]
    async fn clean_snapshot_has_no_diagnostics() {
        let diagnostics = check_semio_video_invariants(&clean_snapshot());
        assert!(diagnostics.is_empty(), "expected no diagnostics, got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn zero_denominator_rate_is_a_hard_error() {
        let mut snap = clean_snapshot();
        snap.streams[0].rate.den = 0;
        let diagnostics = check_semio_video_invariants(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_video.rate-zero-denominator" && d.severity == dsl::Severity::Error));
    }

    #[semio_framework_async_macros::async_test]
    async fn zero_dimension_video_stream_is_a_hard_error() {
        let mut snap = clean_snapshot();
        snap.streams[0].width = 0;
        let diagnostics = check_semio_video_invariants(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_video.video-stream-zero-dimension" && d.severity == dsl::Severity::Error));
    }

    #[semio_framework_async_macros::async_test]
    async fn zero_dimension_non_video_stream_is_not_flagged() {
        let mut snap = clean_snapshot();
        snap.streams[0].kind = SemioVideoStreamKind::Audio;
        snap.streams[0].width = 0;
        snap.streams[0].height = 0;
        let diagnostics = check_semio_video_invariants(&snap);
        assert!(diagnostics.iter().all(|d| d.code.0 != "stdio.semio_video.video-stream-zero-dimension"));
    }

    #[semio_framework_async_macros::async_test]
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

    #[semio_framework_async_macros::async_test]
    async fn subset_validator_recheck_agrees_with_direct_invariant_check() {
        let snap = clean_snapshot();
        let bytes = <SemioVideoSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let diagnostics = SemioVideoValidator::validate(&IoPayload::Binary(bytes)).await;
        assert!(diagnostics.is_empty(), "wire recheck must agree with the direct check for a clean snapshot: {diagnostics:?}");
    }

    //#region 🔖️ConformanceLaws
    /// 🧪️ Per-artifact conformance laws (grammar recipe §4 item 8) for `s.stdio.semio.video`'s
    /// three facets. Lives in this composer's own test region: video has no per-standard `⚙️engine`
    /// dir the way json/csv/zip/png do, and v1's SHARED `🏅️standards/🔖️v1/⚙️engine/🦀️.rs`
    /// only aggregates all 14 subsets' `register()` calls (no test module of its own, and out of
    /// this ticket's `🎬️video/`-only edit scope anyway) — same home flow's/mesh's/image's own
    /// waves establish.
    mod conformance_laws {

        use crate::standards::v1::subsets::video::schema::{diff, mutations, snapshot};
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
            let text = store::ArtifactDsl::print_dsl(&snapshot::demo_video_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op` output
        /// for every `SemioVideoMutation` variant (`mutations::demo_mutation_cases()`).
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
        /// for every representative `SemioVideoDiff` (`diff::demo_diff_cases()`), incl. the empty
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
        #[semio_framework_async_macros::async_test]
        async fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../../../../✉️base/📚️examples/🎥️clip/🖼️assets/🗣️.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../../../../✉️base/📚️examples/🎥️clip/🖼️assets/🎒️.pack.semio");

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
