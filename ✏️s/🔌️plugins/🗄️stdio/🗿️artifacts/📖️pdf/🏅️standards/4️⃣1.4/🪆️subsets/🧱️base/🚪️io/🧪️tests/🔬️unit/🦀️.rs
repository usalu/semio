use super::*;
use crate::standards::v1_4::subsets::base::schema::snapshot::demo_pdf_snapshot;

/// 🧫️ The real committed document every 1.4 test case runs on — 65 pages, a classic
/// cross-reference table, page 1 typeset at A4.
const THESIS: &[u8] = include_bytes!("../../../🖼️assets/🎓️bachelor-thesis/🎓️bachelor-thesis.pdf");

/// 🧪️ `codec_retention_law`: decode→encode→decode is stable on everything this standard's
/// snapshot carries — every page, its extent and its shown text.
#[semio_framework_async_macros::async_test]
async fn codec_retention_law_round_trips_every_page() {
    let original = PdfSnapshot {
        schema: STDIO_PDF_DOCUMENT_SCHEMA.into(),
        pages: vec![
            PageDoc { width: 612.0, height: 792.0, text: "Hello Semio".into() },
            PageDoc { width: 595.276, height: 841.89, text: "Zweite Seite (mit Klammern)".into() },
            PageDoc { width: 200.0, height: 300.0, text: String::new() },
            // 🔤 Non-ASCII, and a `U+FFFD` — what `shown_text`'s lossy decode leaves behind for
            // a glyph code with no Unicode reading, which the committed thesis carries on page 1.
            // Writing a literal string through `byte as char` re-encoded every byte ≥ 0x80 as
            // TWO bytes and broke exactly this round trip; the differential run caught it.
            PageDoc { width: 300.0, height: 400.0, text: "Grüße \u{fffd} 中文 — a\\b(c)d".into() },
        ],
    };
    let bytes = encode_pdf(&original).expect("encode");
    let redecoded = decode_pdf(&bytes).expect("decode");
    assert_eq!(redecoded.pages, original.pages);
}

/// 🧪️ The defect this standard's real codec exists to close: the committed 65-page thesis used
/// to come back as a one-page, 607-byte skeleton. Every page must survive decode AND encode.
#[semio_framework_async_macros::async_test]
async fn the_real_65_page_thesis_survives_decode_and_re_encode() {
    let decoded = decode_pdf(THESIS).expect("the committed thesis decodes");
    assert_eq!(decoded.pages.len(), 65, "the thesis has 65 pages and every one of them must be read");
    let first = &decoded.pages[0];
    assert!((first.width - 595.276).abs() < 1e-3 && (first.height - 841.89).abs() < 1e-3, "page 1 is typeset at A4, got {}x{}", first.width, first.height);
    assert!(first.text.contains("SemIO"), "page 1's shown text must be recovered, got {:?}", first.text);

    let bytes = encode_pdf(&decoded).expect("re-encode");
    assert_ne!(bytes, THESIS, "a re-encode from the model alone can never be a byte pass-through");
    let again = decode_pdf(&bytes).expect("the re-encoded document decodes");
    assert_eq!(again.pages.len(), 65, "re-encoding must not drop a single page");
    assert_eq!(again.pages[0].text, first.text, "page 1's shown text must survive the re-encode");
    assert!((again.pages[0].width - first.width).abs() < 1e-6);
}

/// 🧪️ A degenerate page extent is a real, writable state — `1.4/🖨️x`'s `collapse-page-size`
/// depends on it — so the writer must never clamp it away.
#[semio_framework_async_macros::async_test]
async fn a_collapsed_page_extent_is_written_as_it_stands() {
    let snapshot = PdfSnapshot { schema: STDIO_PDF_DOCUMENT_SCHEMA.into(), pages: vec![PageDoc { width: 0.0, height: 841.89, text: "x".into() }] };
    let bytes = encode_pdf(&snapshot).expect("encode");
    assert!(String::from_utf8_lossy(&bytes).contains("/MediaBox [0 0 0 841.89]"));
    assert_eq!(decode_pdf(&bytes).expect("decode").pages[0].width, 0.0);
}

#[semio_framework_async_macros::async_test]
async fn a_document_that_is_not_a_pdf_is_refused() {
    assert!(decode_pdf(b"not a pdf at all").is_err());
}

#[semio_framework_async_macros::async_test]
async fn demo_snapshot_round_trip() {
    let snap = demo_pdf_snapshot();
    let text = store::ArtifactDsl::print_dsl(&snap);
    let parsed = <PdfSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(parsed, snap);
    let bytes = store::ArtifactPack::encode_pack(&snap);
    let decoded = <PdfSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(decoded, snap);
}

//#region 🔖️ConformanceLaws
/// 🧪️ P2-FG3: per-artifact conformance laws — grammar/protocol parseability, `Recognizer`
/// against real fixtures AND real `print_op`/`print_diff` output, `walk_protocol` against real
/// `encode_pack`/`encode_op`/`encode_diff` bytes, and the fixture-honesty round-trip. Lives
/// here (the engine's own test region), not any framework file.
mod conformance_laws {
    use super::*;
    #[cfg(test)]
    use crate::standards::v1_4::subsets::base::schema::diff::PdfDiff;
    use crate::standards::v1_4::subsets::base::schema::mutations::PdfMutation;
    use crate::standards::v1_4::subsets::base::schema::{diff, mutations, snapshot};
    use protocol::{DiffCodec, OpBinary, OpText};

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn demo_mutation_cases() -> Vec<PdfMutation> {
        vec![
            PdfMutation::InsertPage(mutations::InsertPage { index: 0, page: PageDoc::default() }),
            PdfMutation::RemovePage(mutations::RemovePage { index: 0 }),
            PdfMutation::MovePage(mutations::MovePage { from: 0, to: 1 }),
            PdfMutation::ResizePage(mutations::ResizePage { index: 1, width: 300.5, height: 400.25 }),
            PdfMutation::ReplacePageText(mutations::ReplacePageText { index: 0, text: "hello world".into() }),
        ]
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn demo_diff_cases() -> Vec<PdfDiff> {
        let a = demo_pdf_snapshot();
        let b = PdfSnapshot { schema: STDIO_PDF_DOCUMENT_SCHEMA.into(), pages: vec![PageDoc { width: 300.5, height: 400.25, text: "changed text".into() }, PageDoc { width: 100.0, height: 100.0, text: "second".into() }] };
        let c = PdfSnapshot { schema: STDIO_PDF_DOCUMENT_SCHEMA.into(), pages: Vec::new() };
        vec![<PdfDiff as protocol::command::DiffAlgebra<PdfSnapshot>>::between(&a, &b), <PdfDiff as protocol::command::DiffAlgebra<PdfSnapshot>>::between(&b, &c), PdfDiff::default()]
    }

    /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
    /// parse under the real dialect.
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

    /// ✅️ `grammar_conformance_law`: the snapshot grammar recognizes real `print_dsl` output.
    #[semio_framework_async_macros::async_test]
    async fn grammar_conformance_law() {
        let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
        let recognizer = dsl::Recognizer::compile(&grammar);
        let text = store::ArtifactDsl::print_dsl(&demo_pdf_snapshot());
        let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
        let reconstructed = format!("{}\n{body}", envelope.envelope_id());
        assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
    }

    /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
    /// output for every demo `PdfMutation`.
    #[semio_framework_async_macros::async_test]
    async fn ops_grammar_conformance_law() {
        let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
        let recognizer = dsl::Recognizer::compile(&grammar);
        for mutation in demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
        }
    }

    /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output.
    #[semio_framework_async_macros::async_test]
    async fn diff_grammar_conformance_law() {
        let grammar = dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
        let recognizer = dsl::Recognizer::compile(&grammar);
        for d in demo_diff_cases() {
            let printed = d.print_diff();
            assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
        }
    }

    /// ✅️ `protocol_walk_law`: `walk_protocol` against REAL bytes for all three facets.
    #[semio_framework_async_macros::async_test]
    async fn protocol_walk_law() {
        let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
        let packed = store::ArtifactPack::encode_pack(&demo_pdf_snapshot());
        let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
        let trace = dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
        assert_eq!(trace.consumed, inner.len(), "pack walk did not consume every byte");

        let op_spec = dsl::parse_protocol(mutations::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
        for mutation in demo_mutation_cases() {
            let bytes = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
            let trace = dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
        }

        let diff_spec = dsl::parse_protocol(diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
        for d in demo_diff_cases() {
            let bytes = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
            let trace = dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
        }
    }

    /// ✅️ `fixture_honesty_law`: the shipped `.dsl.semio`/`.pack.semio` fixtures are GENUINE
    /// `print_dsl`/`encode_pack` output of `demo_pdf_snapshot()`.
    #[semio_framework_async_macros::async_test]
    async fn fixture_honesty_law() {
        const FIXTURE_DSL: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");
        const FIXTURE_PACK: &[u8] = include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio");

        let demo = demo_pdf_snapshot();

        let parsed = <PdfSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
        assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_pdf_snapshot()");
        assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_pdf_snapshot()) drifted from the shipped .dsl.semio fixture");

        let decoded = <PdfSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
        assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_pdf_snapshot()");
        assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_pdf_snapshot()) drifted from the shipped .pack.semio fixture");
    }
}
//#endregion 🔖️ConformanceLaws
