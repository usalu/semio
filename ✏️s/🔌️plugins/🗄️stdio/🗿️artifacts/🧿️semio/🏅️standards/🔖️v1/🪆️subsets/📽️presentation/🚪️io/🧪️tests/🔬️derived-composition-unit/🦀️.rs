mod tests {
    use super::*;
    use crate::standards::v1::subsets::presentation::schema::snapshot::{Slide, SlideLayout, SlideMaster};
    use semio_framework_plugin::{ArtifactDeserializer, ArtifactSerializer};

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn clean_snapshot() -> SemioPresentationSnapshot {
        SemioPresentationSnapshot {
            schema: "s.stdio.semio.presentation".into(),
            masters: vec![SlideMaster { id: "m1".into(), shapes: Vec::new() }],
            layouts: vec![SlideLayout { id: "l1".into(), master_id: "m1".into(), shapes: Vec::new() }],
            slides: vec![Slide { id: "s1".into(), layout_id: Some("l1".into()), shapes: Vec::new(), notes: Vec::new() }],
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn clean_snapshot_has_no_diagnostics() {
        assert!(check_presentation_referential_integrity(&clean_snapshot()).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn dangling_layout_master_is_flagged() {
        let mut snap = clean_snapshot();
        snap.layouts[0].master_id = "missing".into();
        let diagnostics = check_presentation_referential_integrity(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_presentation.dangling-layout-master"), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn dangling_slide_layout_is_flagged() {
        let mut snap = clean_snapshot();
        snap.slides[0].layout_id = Some("missing".into());
        let diagnostics = check_presentation_referential_integrity(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_presentation.dangling-slide-layout"), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn duplicate_master_id_is_flagged() {
        let mut snap = clean_snapshot();
        snap.masters.push(SlideMaster { id: "m1".into(), shapes: Vec::new() });
        let diagnostics = check_presentation_referential_integrity(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_presentation.duplicate-master-id"), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn slide_with_no_layout_is_never_flagged() {
        let mut snap = clean_snapshot();
        snap.slides[0].layout_id = None;
        assert!(check_presentation_referential_integrity(&snap).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn validator_roundtrips_through_pack_payload() {
        let bytes = <SemioPresentationSnapshot as store::ArtifactPack>::encode_pack(&clean_snapshot());
        let diagnostics = SemioPresentationValidator::validate(&IoPayload::Binary(bytes)).await;
        assert!(diagnostics.is_empty(), "got {diagnostics:?}");
    }

    /// 🔁️ W4 G6 fixture-backed round trip: pptx1 -(deserialize)-> semio1 -(serialize)-> pptx2
    /// -(deserialize)-> semio2, asserting semio1 == semio2 (masters/layouts/slide id/notes/Table
    /// shapes are the documented lossy fields — this fixture avoids them by construction, so the
    /// comparison exercises TextBox/Picture/Placeholder shape fidelity end to end).
    #[semio_framework_async_macros::async_test]
    async fn pptx_round_trip_is_stable() {
        use semio_s_artifact_stdio_pptx::PptxSnapshot;
        use semio_s_artifact_stdio_pptx::schema::snapshot::{PptxParagraph, PptxPresentation, PptxRun, PptxShape, PptxSlide, PptxTransform};
        use semio_s_artifact_stdio_zip::opc::OpcPackage;

        let pptx1 = PptxSnapshot::from_parts(
            OpcPackage::default(),
            Vec::new(),
            PptxPresentation {
                slides: vec![PptxSlide {
                    shapes: vec![
                        PptxShape::TextBox { text_frame: vec![PptxParagraph { runs: vec![PptxRun { text: "Hello".into(), bold: true, italic: false, font_size: Some(24) }] }], position: PptxTransform { x: 0, y: 0, cx: 100, cy: 20 } },
                        PptxShape::Picture { blip_rel_id: "rId2".into(), position: PptxTransform { x: 0, y: 30, cx: 50, cy: 50 } },
                        PptxShape::Placeholder { kind: "title".into(), text_frame: Vec::new(), position: PptxTransform { x: 0, y: 0, cx: 200, cy: 40 } },
                    ],
                }],
            },
        );
        let semio1 = semio_framework_plugin::resolve_ready(SemioPresentationFromPptx::deserialize(&pptx1)).expect("deserialize");
        let pptx2 = semio_framework_plugin::resolve_ready(SemioPresentationToPptx::serialize(&semio1)).expect("serialize");
        let semio2 = semio_framework_plugin::resolve_ready(SemioPresentationFromPptx::deserialize(&pptx2)).expect("deserialize round 2");
        assert_eq!(semio1, semio2);
    }

    //#region 🔖️ConformanceLaws
    /// 🧪️ Per-artifact conformance laws (grammar recipe §4 item 8) for `s.stdio.semio.presentation`'s
    /// three facets — ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION presentation
    /// wave, following the flow/model/brep/drawing/document pilots' proven pattern. Lives in
    /// this composer's own test region: presentation has no per-standard `⚙️engine` dir the way
    /// json/csv/zip/png do (only `📸️snapshot`/`🔺️diff`/`🧬️mutations`/`🎹️composer`/`🏗️builder`/
    /// `🚪️io`/`🧐️analyzer`), and v1's SHARED `🏅️standards/🔖️v1/⚙️engine/🦀️.rs` only
    /// aggregates all 14 subsets' `register()` calls (no test module of its own, and out of this
    /// ticket's `📽️presentation/`-only edit scope anyway).
    mod conformance_laws {

        use crate::standards::v1::subsets::presentation::schema::{diff, mutations, snapshot};
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
            let text = store::ArtifactDsl::print_dsl(&snapshot::demo_semio_presentation_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op` output
        /// for every `SemioPresentationMutation` variant (`mutations::demo_mutation_cases()`).
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
        /// for every representative `SemioPresentationDiff` (`diff::demo_diff_cases()`), incl. the
        /// empty (no-op) diff.
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
            let packed = store::ArtifactPack::encode_pack(&snapshot::demo_semio_presentation_snapshot());
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
        /// `print_dsl`/`encode_pack` output of `snapshot::demo_semio_presentation_snapshot()` —
        /// `parse_dsl(fixture) == demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the
        /// pack twin — so the fixtures can never silently drift back to a fake.
        #[semio_framework_async_macros::async_test]
        async fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../../../../✉️base/📚️examples/📽️deck/🖼️assets/🗣️.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../../../../✉️base/📚️examples/📽️deck/🖼️assets/🎒️.pack.semio");

            let demo = snapshot::demo_semio_presentation_snapshot();

            let parsed = <snapshot::SemioPresentationSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_semio_presentation_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_semio_presentation_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <snapshot::SemioPresentationSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_semio_presentation_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_semio_presentation_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
