use super::*;
use crate::schema::snapshot::{DocxBlock, DocxParagraph, DocxRun, DocxStyle, DocxTable, DocxTableCell, DocxTableRow};
use crate::standards::v_ecma_376::subsets::base::io::export::serializers::{build_minimal_docx, document_to_xml, encode_docx};
use crate::standards::v_ecma_376::subsets::base::io::import::deserializers::{decode_docx, sniff_docx_bytes};
use crate::standards::v_ecma_376::subsets::base::io::DocxError;
use semio_s_artifact_stdio_xml::schema::snapshot::{xml_document_to_text, XmlAttr, XmlNode};
use semio_s_artifact_stdio_zip::opc::{OpcPackage, RELS_CONTENT_TYPE};

async fn sample_document() -> DocxDocument {
    DocxDocument {
        body: vec![
            DocxBlock::Paragraph(DocxParagraph {
                runs: vec![DocxRun { text: "Hello, ".into(), bold: true, ..Default::default() }, DocxRun { text: "world!".into(), italic: true, ..Default::default() }],
                style: None,
                extra_paragraph_properties: Vec::new(),
            }),
            DocxBlock::paragraph("Second paragraph, plain."),
        ],
        styles: Vec::new(),
    }
}

async fn sample_document_with_table_and_styles() -> DocxDocument {
    DocxDocument {
        body: vec![
            DocxBlock::Paragraph(DocxParagraph { style: Some("Heading1".into()), ..DocxParagraph::text("Title") }),
            DocxBlock::Table(DocxTable {
                rows: vec![DocxTableRow { cells: vec![DocxTableCell { blocks: vec![DocxBlock::paragraph("R1C1")], ..Default::default() }, DocxTableCell { blocks: vec![DocxBlock::paragraph("R1C2")], ..Default::default() }], ..Default::default() }],
                ..Default::default()
            }),
        ],
        styles: vec![DocxStyle { id: "Normal".into(), name: "Normal".into(), based_on: None }, DocxStyle { id: "Heading1".into(), name: "heading 1".into(), based_on: Some("Normal".into()) }],
    }
}

#[semio_framework_async_macros::async_test]
async fn builder_produces_minimal_valid_package_that_decodes_back() {
    let snap = build_minimal_docx(sample_document().await);
    let bytes = encode_docx(&snap).expect("encode minimal package");
    assert!(semio_s_artifact_stdio_zip::opc::sniff_opc_bytes(&bytes));
    assert!(sniff_docx_bytes(&bytes));
    let decoded = decode_docx(&bytes).expect("decode minimal package");
    assert_eq!(decoded.document, sample_document().await);
}

#[semio_framework_async_macros::async_test]
async fn tables_and_styles_round_trip() {
    let snap = build_minimal_docx(sample_document_with_table_and_styles().await);
    let bytes = encode_docx(&snap).expect("encode");
    let decoded = decode_docx(&bytes).expect("decode");
    assert_eq!(decoded.document, sample_document_with_table_and_styles().await);
    let DocxBlock::Table(table) = &decoded.document.body[1] else { panic!("expected table") };
    assert_eq!(table.rows[0].cells.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn decode_resolves_real_hand_built_package_with_formatting() {
    // Hand-built OOXML: correct Content_Types/.rels/part structure, not just "a zip with xml".
    let mut opc = OpcPackage::empty();
    opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
    opc.content_types.set_default("xml", "application/xml");
    let xml = concat!(
        r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">"#,
        "<w:body>",
        r#"<w:p><w:r><w:rPr><w:b/></w:rPr><w:t xml:space="preserve">Bold run</w:t></w:r></w:p>"#,
        r#"<w:p><w:r><w:rPr><w:i/></w:rPr><w:t xml:space="preserve">Italic run</w:t></w:r></w:p>"#,
        r#"<w:p><w:r><w:t xml:space="preserve">Plain &amp; escaped</w:t></w:r></w:p>"#,
        "</w:body>",
        "</w:document>",
    );
    const MAIN_DOCUMENT_PART: &str = "word/document.xml";
    const MAIN_DOCUMENT_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
    const REL_TYPE_OFFICE_DOCUMENT: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
    opc.set_part(MAIN_DOCUMENT_PART, MAIN_DOCUMENT_CONTENT_TYPE, xml.as_bytes().to_vec());
    opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, MAIN_DOCUMENT_PART);
    let bytes = semio_s_artifact_stdio_zip::opc::encode_opc(&opc).expect("encode opc");

    let decoded = decode_docx(&bytes).expect("decode hand-built docx");
    assert_eq!(decoded.document.body.len(), 3);
    let DocxBlock::Paragraph(p0) = &decoded.document.body[0] else { panic!("paragraph") };
    assert!(p0.runs[0].bold);
    let DocxBlock::Paragraph(p1) = &decoded.document.body[1] else { panic!("paragraph") };
    assert!(p1.runs[0].italic);
    let DocxBlock::Paragraph(p2) = &decoded.document.body[2] else { panic!("paragraph") };
    assert_eq!(p2.runs[0].text, "Plain & escaped");
}

#[semio_framework_async_macros::async_test]
async fn unmodeled_parts_survive_decode_encode_verbatim() {
    const MAIN_DOCUMENT_PART: &str = "word/document.xml";
    const MAIN_DOCUMENT_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
    const REL_TYPE_OFFICE_DOCUMENT: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
    let mut opc = OpcPackage::empty();
    opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
    opc.content_types.set_default("xml", "application/xml");
    opc.set_part(MAIN_DOCUMENT_PART, MAIN_DOCUMENT_CONTENT_TYPE, xml_document_to_text(&document_to_xml(&sample_document().await)).into_bytes());
    opc.set_part("word/numbering.xml", "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml", b"<w:numbering/>".to_vec());
    opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, MAIN_DOCUMENT_PART);
    let bytes = semio_s_artifact_stdio_zip::opc::encode_opc(&opc).expect("encode");

    let decoded = decode_docx(&bytes).expect("decode");
    assert_eq!(decoded.opc.part_bytes("word/numbering.xml"), Some(b"<w:numbering/>".as_slice()));
    let re_encoded = encode_docx(&decoded).expect("re-encode");
    let re_decoded = decode_docx(&re_encoded).expect("re-decode");
    assert_eq!(re_decoded.opc.part_bytes("word/numbering.xml"), Some(b"<w:numbering/>".as_slice()));
    assert_eq!(re_decoded.document, sample_document().await);
}

#[semio_framework_async_macros::async_test]
async fn unmodeled_run_properties_survive_round_trip() {
    let mut run = DocxRun { text: "colored".into(), ..Default::default() };
    run.extra_run_properties.push(XmlNode::Element { name: "w:color".into(), attrs: vec![XmlAttr { name: "w:val".into(), value: "FF0000".into() }], children: vec![] });
    let doc = DocxDocument { body: vec![DocxBlock::Paragraph(DocxParagraph { runs: vec![run], style: None, extra_paragraph_properties: Vec::new() })], styles: Vec::new() };
    let snap = build_minimal_docx(doc.clone());
    let bytes = encode_docx(&snap).expect("encode");
    let decoded = decode_docx(&bytes).expect("decode");
    assert_eq!(decoded.document, doc);
}

#[semio_framework_async_macros::async_test]
async fn decode_rejects_missing_main_document_relationship() {
    let mut opc = OpcPackage::empty();
    opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
    let bytes = semio_s_artifact_stdio_zip::opc::encode_opc(&opc).expect("encode");
    let err = decode_docx(&bytes).expect_err("must reject a package with no officeDocument relationship");
    assert_eq!(err, DocxError::MissingMainDocumentRelationship);
}

#[semio_framework_async_macros::async_test]
async fn analyzer_builder_round_trip() {
    let original = build_minimal_docx(sample_document_with_table_and_styles().await);
    // Analyzer: real decode of the encoded bytes.
    let bytes = encode_docx(&original).expect("encode");
    let analyzed = decode_docx(&bytes).expect("decode");
    // Builder: reconstruct an equivalent document from the analyzed parts.
    let rebuilt = build_minimal_docx(analyzed.document.clone());
    let rebuilt_bytes = encode_docx(&rebuilt).expect("encode rebuilt");
    let reanalyzed = decode_docx(&rebuilt_bytes).expect("decode rebuilt");
    assert_eq!(reanalyzed.document, analyzed.document);
}

//#region 🔖️ConformanceLaws
/// 🧪️ FG-wave: per-artifact conformance laws (`📖️grammar-recipe.md` §4's checklist item) --
/// grammar/protocol parseability, `Recognizer` against real fixtures AND real `print_op`/
/// `print_diff` output, `walk_protocol` against real `encode_pack`/`encode_op`/`encode_diff`
/// bytes, and the fixture-honesty round-trip. Lives beside the rest of this artifact's schema
/// tests (moved out of `⚙️engine`, ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) --
/// this artifact's OWN early-warning, plus direct coverage of the mutations/diff facets the
/// framework's `m5` auto-discovery does not reach at all.
mod conformance_laws {
    use super::*;
    use crate::schema::{diff, mutations, snapshot};
    use protocol::{DiffCodec, OpBinary, OpText};

    /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
    /// parse under the real dialect -- independent of, and cheaper than, the two
    /// `recognize`/`walk_protocol` laws below (a parse failure here fails fast with a clearer
    /// message).
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

    /// ✅️ `grammar_conformance_law`: the snapshot grammar models the real TEXT syntax of the
    /// XML parts a docx OPC package carries (`📸️snapshot/📝️text/📖️.grammar.semio`'s
    /// own doc comment explains why -- this artifact's `ArtifactDsl::print_dsl` hex-dumps the
    /// WHOLE binary OPC package, matching this facet's SIBLING binary protocol, not this text
    /// grammar; the two facets describe different LAYERS of the same real artifact, same as
    /// every OPC-family member's own container/contained-parts split). So, UNLIKE a
    /// binary-native pilot's `grammar_conformance_law` (which feeds `print_dsl` output
    /// straight to the recognizer), this law decodes the REAL zip entries `encode_docx`
    /// genuinely produces (via `zip::engine::decode_zip`, the same real codec `opc::decode_opc`
    /// itself delegates to) and recognizes EACH real part's own text against the grammar --
    /// direct proof the grammar matches this artifact's own real per-part XML bytes, not an
    /// invented approximation.
    #[semio_framework_async_macros::async_test]
    async fn grammar_conformance_law() {
        let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
        let recognizer = dsl::Recognizer::compile(&grammar);

        let demo = demo_docx_snapshot().await;
        let bytes = encode_docx(&demo).expect("encode demo docx");
        let zip = semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::io::decode_zip(&bytes).expect("decode zip");

        let modeled_parts = ["[Content_Types].xml", "_rels/.rels", "word/document.xml", "word/styles.xml"];
        let mut checked = 0;
        for entry in &zip.entries {
            if !modeled_parts.contains(&entry.name.as_str()) {
                continue;
            }
            let text = String::from_utf8(entry.data.clone()).unwrap_or_else(|e| panic!("part {:?}: not valid utf-8: {e}", entry.name));
            assert!(recognizer.recognize(&text).unwrap_or(false), "grammar did not recognize real part {:?}:\n{text}", entry.name);
            checked += 1;
        }
        assert_eq!(checked, modeled_parts.len(), "not every modeled part was present in the real zip entries");
    }

    /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
    /// output for every `DocxMutation` variant (`mutations::demo_mutation_cases()`).
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
    /// for every representative `DocxDiff` (`diff::demo_diff_cases()`).
    #[semio_framework_async_macros::async_test]
    async fn diff_grammar_conformance_law() {
        let grammar = dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
        let recognizer = dsl::Recognizer::compile(&grammar);
        for d in diff::demo_diff_cases() {
            let printed = d.print_diff();
            assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
        }
    }

    /// ✅️ `protocol_walk_law`: `walk_protocol` against REAL bytes for all three facets --
    /// snapshot pack (`encode_pack`, envelope-unwrapped first, matching how
    /// `m5_handcrafted_protocol_conformance` itself feeds `walk_protocol`), every demo
    /// mutation's `encode_op`, and every demo diff's `encode_diff`. The snapshot protocol
    /// declares `backward`/`jump` (restated from zip's own real ZIP layout), so `walk_protocol`
    /// correctly does NOT require landing on exactly `bytes.len()` (M2's own documented
    /// exception, `📖️grammar-recipe.md` §2.3) -- assert a sane in-range `consumed` there
    /// instead, same as zip's own `protocol_walk_law` does; the op/diff protocols have no such
    /// exception and must consume every byte.
    #[semio_framework_async_macros::async_test]
    async fn protocol_walk_law() {
        let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
        let demo = demo_docx_snapshot().await;
        let packed = store::ArtifactPack::encode_pack(&demo);
        let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
        let trace = dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
        assert!(trace.consumed > 0 && trace.consumed <= inner.len(), "pack walk consumed an out-of-range span");

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
    /// `print_dsl`/`encode_pack` output of `demo_docx_snapshot().await` -- `parse_dsl(fixture) ==
    /// demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the pack twin -- so the
    /// fixtures can never silently drift back to a fake `"68656c6c6f"`-style placeholder again
    /// (see this ticket's own recon note on the pre-FG-wave state of these two files).
    #[semio_framework_async_macros::async_test]
    async fn fixture_honesty_law() {
        const FIXTURE_DSL: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");
        const FIXTURE_PACK: &[u8] = include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio");

        let demo = demo_docx_snapshot().await;

        let parsed = <DocxSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
        assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_docx_snapshot().await");
        assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_docx_snapshot().await) drifted from the shipped .dsl.semio fixture");

        let decoded = <DocxSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
        assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_docx_snapshot().await");
        assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_docx_snapshot().await) drifted from the shipped .pack.semio fixture");

        let native = encode_docx(&demo).expect("encode native docx");
        assert_eq!(native.as_slice(), include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/📜️example.docx"), "encode_docx(demo) drifted from 📜️example.docx");
    }

    #[semio_framework_async_macros::async_test]
    #[ignore]
    async fn zzz_write_native_docx_fixture() {
        let demo = demo_docx_snapshot().await;
        let native = encode_docx(&demo).expect("encode");
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/📚️examples/🎬️demo/🖼️assets/📜️example.docx");
        std::fs::write(path, native).expect("write 📜️example.docx");
    }
}
//#endregion 🔖️ConformanceLaws
