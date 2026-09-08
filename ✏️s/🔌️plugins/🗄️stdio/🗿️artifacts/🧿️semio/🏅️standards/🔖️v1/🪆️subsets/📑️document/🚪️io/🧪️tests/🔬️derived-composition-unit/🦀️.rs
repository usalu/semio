mod tests {
    use super::*;
    use crate::standards::v1::subsets::document::schema::snapshot::{DocImage, DocStyle};
    use semio_framework_plugin::{ArtifactDeserializer, ArtifactSerializer};

    #[semio_framework_async_macros::async_test]
    async fn clean_document_validates_with_no_diagnostics() {
        let snapshot = SemioDocumentSnapshot {
            schema: crate::standards::v1::subsets::document::schema::snapshot::STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(),
            styles: vec![DocStyle { id: "base".into(), name: "Base".into(), based_on: None }, DocStyle { id: "child".into(), name: "Child".into(), based_on: Some("base".into()) }],
            images: vec![DocImage { id: "img1".into(), mime: "image/png".into(), bytes: vec![1] }],
            blocks: vec![DocBlock::Paragraph { style_id: Some("child".into()), runs: Vec::new() }, DocBlock::Image { image_id: "img1".into(), alt: "alt".into(), width: None, height: None }],
        };
        let bytes = store::ArtifactPack::encode_pack(&snapshot);
        let diagnostics = SemioDocumentValidator::validate(&IoPayload::Binary(bytes)).await;
        assert!(diagnostics.is_empty(), "expected no diagnostics, got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn unresolved_image_and_style_references_are_flagged() {
        let snapshot = SemioDocumentSnapshot {
            schema: crate::standards::v1::subsets::document::schema::snapshot::STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(),
            styles: Vec::new(),
            images: Vec::new(),
            blocks: vec![DocBlock::Paragraph { style_id: Some("missing-style".into()), runs: Vec::new() }, DocBlock::Image { image_id: "missing-image".into(), alt: String::new(), width: None, height: None }],
        };
        let bytes = store::ArtifactPack::encode_pack(&snapshot);
        let diagnostics = SemioDocumentValidator::validate(&IoPayload::Binary(bytes)).await;
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_document.unresolved-style-id"), "got {diagnostics:?}");
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_document.unresolved-image-id"), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn based_on_cycle_is_flagged() {
        let snapshot = SemioDocumentSnapshot {
            schema: crate::standards::v1::subsets::document::schema::snapshot::STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(),
            styles: vec![DocStyle { id: "a".into(), name: "A".into(), based_on: Some("b".into()) }, DocStyle { id: "b".into(), name: "B".into(), based_on: Some("a".into()) }],
            images: Vec::new(),
            blocks: Vec::new(),
        };
        let diagnostics = check_document_referential_integrity(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_document.based-on-cycle"), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn nested_table_cell_reference_is_checked() {
        let snapshot = SemioDocumentSnapshot {
            schema: crate::standards::v1::subsets::document::schema::snapshot::STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(),
            styles: Vec::new(),
            images: Vec::new(),
            blocks: vec![DocBlock::Table {
                rows: vec![crate::standards::v1::subsets::document::schema::snapshot::DocTableRow {
                    cells: vec![crate::standards::v1::subsets::document::schema::snapshot::DocTableCell { blocks: vec![DocBlock::Image { image_id: "nested-missing".into(), alt: String::new(), width: None, height: None }] }],
                }],
            }],
        };
        let diagnostics = check_document_referential_integrity(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_document.unresolved-image-id"), "nested reference must be checked too: {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn composer_reads_own_dialect_pack() {
        let snapshot = SemioDocumentSnapshot::default();
        let bytes = store::ArtifactPack::encode_pack(&snapshot);
        let sources = vec![ComposeSource { dialect: DIALECT, payload: AnalyzeSource::Binary(&bytes) }];
        let composed = SemioDocumentComposerComposition::compose(&sources).expect("compose from native dialect");
        assert_eq!(composed.snapshot, snapshot);
    }

    //#region 🔖️IoRoundTrips
    // 🔁️ W4 G6 fixture-backed round trips: format1 -(deserialize)-> semio1 -(serialize)->
    // format2 -(deserialize)-> semio2, asserting semio1 == semio2 — i.e. this pair's serializer is
    // a faithful inverse of what its deserializer captured (documented lossy fields, e.g. docx's
    // `extra_*_properties` or txt's formatting, never entering the comparison because
    // `SemioDocumentSnapshot` itself has no field for them).

    #[semio_framework_async_macros::async_test]
    async fn docx_round_trip_is_stable() {
        use semio_s_artifact_stdio_docx::DocxSnapshot;
        use semio_s_artifact_stdio_docx::schema::snapshot::{DocxBlock, DocxDocument, DocxParagraph, DocxRun, DocxStyle};
        use semio_s_artifact_stdio_zip::opc::OpcPackage;

        let docx1 = DocxSnapshot::from_parts(
            OpcPackage::default(),
            DocxDocument {
                styles: vec![DocxStyle { id: "Heading1".into(), name: "Heading 1".into(), based_on: None }],
                body: vec![
                    DocxBlock::Paragraph(DocxParagraph {
                        runs: vec![DocxRun { text: "Title".into(), bold: true, italic: false, underline: false, extra_run_properties: Vec::new() }],
                        style: Some("Heading1".into()),
                        extra_paragraph_properties: Vec::new(),
                    }),
                    DocxBlock::paragraph("Body."),
                ],
            },
        );
        let semio1 = semio_framework_plugin::resolve_ready(SemioDocumentFromDocx::deserialize(&docx1)).expect("deserialize");
        let docx2 = semio_framework_plugin::resolve_ready(SemioDocumentToDocx::serialize(&semio1)).expect("serialize");
        let semio2 = semio_framework_plugin::resolve_ready(SemioDocumentFromDocx::deserialize(&docx2)).expect("deserialize round 2");
        assert_eq!(semio1, semio2);
    }

    #[semio_framework_async_macros::async_test]
    async fn md_round_trip_is_stable() {
        use semio_s_artifact_stdio_md::MdSnapshot;
        use semio_s_artifact_stdio_md::schema::snapshot::{MdBlock, MdInline};

        let md1 = MdSnapshot {
            schema: semio_s_artifact_stdio_md::STDIO_MD_DOCUMENT_SCHEMA.into(),
            blocks: vec![
                MdBlock::Heading { level: 1, inlines: vec![MdInline::Strong { inlines: vec![MdInline::Text { text: "Title".into() }] }] },
                MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "Body".into() }] },
                MdBlock::CodeBlock { info: Some("rust".into()), literal: "fn main() {}".into() },
            ],
        };
        let semio1 = semio_framework_plugin::resolve_ready(SemioDocumentFromMd::deserialize(&md1)).expect("deserialize");
        let md2 = semio_framework_plugin::resolve_ready(SemioDocumentToMd::serialize(&semio1)).expect("serialize");
        let semio2 = semio_framework_plugin::resolve_ready(SemioDocumentFromMd::deserialize(&md2)).expect("deserialize round 2");
        assert_eq!(semio1, semio2);
    }

    #[semio_framework_async_macros::async_test]
    async fn txt_round_trip_is_stable() {
        use semio_s_artifact_stdio_txt::TxtSnapshot;
        use semio_s_artifact_stdio_txt::schema::snapshot::LineEnding;

        let txt1 = TxtSnapshot { schema: semio_s_artifact_stdio_txt::STDIO_TXT_DOCUMENT_SCHEMA.into(), lines: vec!["First line.".into(), String::new(), "Third line.".into()], trailing_newline: true, line_ending: LineEnding::Lf };
        let semio1 = semio_framework_plugin::resolve_ready(SemioDocumentFromTxt::deserialize(&txt1)).expect("deserialize");
        let txt2 = semio_framework_plugin::resolve_ready(SemioDocumentToTxt::serialize(&semio1)).expect("serialize");
        let semio2 = semio_framework_plugin::resolve_ready(SemioDocumentFromTxt::deserialize(&txt2)).expect("deserialize round 2");
        assert_eq!(semio1, semio2);
    }

    #[semio_framework_async_macros::async_test]
    async fn pdf_round_trip_is_stable() {
        use semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::schema::snapshot::{PdfPage, PdfSnapshot};

        let mut p1 = PdfPage::new(612.0, 792.0);
        p1.text = "Page one text.".into();
        let mut p2 = PdfPage::new(612.0, 792.0);
        p2.text = "Page two text.".into();
        let pdf1 = PdfSnapshot { pages: vec![p1, p2], ..Default::default() };
        let semio1 = semio_framework_plugin::resolve_ready(SemioDocumentFromPdf::deserialize(&pdf1)).expect("deserialize");
        let pdf2 = semio_framework_plugin::resolve_ready(SemioDocumentToPdf::serialize(&semio1)).expect("serialize");
        let semio2 = semio_framework_plugin::resolve_ready(SemioDocumentFromPdf::deserialize(&pdf2)).expect("deserialize round 2");
        assert_eq!(semio1, semio2);
    }
    //#endregion 🔖️IoRoundTrips

    //#region 🔖️ConformanceLaws
    /// 🧪️ Per-artifact conformance laws (grammar recipe §4 item 8) for `s.stdio.semio.document`'s
    /// three facets — ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION document wave,
    /// following the flow/model/brep pilots' proven pattern. Lives in this composer's own test
    /// region: document has no per-standard `⚙️engine` dir the way json/csv/zip/png do, and v1's
    /// SHARED `🏅️standards/🔖️v1/⚙️engine/🦀️.rs` only aggregates all 14 subsets'
    /// `register()` calls (no test module of its own, and out of this ticket's `📑️document/`-only
    /// edit scope anyway).
    mod conformance_laws {

        use crate::standards::v1::subsets::document::schema::{diff, mutations, snapshot};
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
            let text = store::ArtifactDsl::print_dsl(&snapshot::demo_semio_document_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op` output
        /// for every `SemioDocumentMutation` variant (`mutations::demo_mutation_cases()`).
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
        /// for every representative `SemioDocumentDiff` (`diff::demo_diff_cases()`), incl. the
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
            let packed = store::ArtifactPack::encode_pack(&snapshot::demo_semio_document_snapshot());
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
        /// `print_dsl`/`encode_pack` output of `snapshot::demo_semio_document_snapshot()` —
        /// `parse_dsl(fixture) == demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the
        /// pack twin — so the fixtures can never silently drift back to a fake.
        #[semio_framework_async_macros::async_test]
        async fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../../../../✉️base/📚️examples/🗒️memo/🖼️assets/🗣️.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../../../../✉️base/📚️examples/🗒️memo/🖼️assets/🎒️.pack.semio");

            let demo = snapshot::demo_semio_document_snapshot();

            let parsed = <snapshot::SemioDocumentSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_semio_document_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_semio_document_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <snapshot::SemioDocumentSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_semio_document_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_semio_document_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
