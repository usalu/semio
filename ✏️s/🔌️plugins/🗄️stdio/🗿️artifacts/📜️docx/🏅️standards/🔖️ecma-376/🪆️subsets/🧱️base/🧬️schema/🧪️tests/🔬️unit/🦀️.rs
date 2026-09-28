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
    assert_eq!(decoded.project_document().expect("project document"), sample_document().await);
}

#[semio_framework_async_macros::async_test]
async fn tables_and_styles_round_trip() {
    let snap = build_minimal_docx(sample_document_with_table_and_styles().await);
    let bytes = encode_docx(&snap).expect("encode");
    let decoded = decode_docx(&bytes).expect("decode");
    let document = decoded.project_document().expect("project document");
    assert_eq!(document, sample_document_with_table_and_styles().await);
    let DocxBlock::Table(table) = &document.body[1] else { panic!("expected table") };
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
    let document = decoded.project_document().expect("project document");
    assert_eq!(document.body.len(), 3);
    let DocxBlock::Paragraph(p0) = &document.body[0] else { panic!("paragraph") };
    assert!(p0.runs[0].bold);
    let DocxBlock::Paragraph(p1) = &document.body[1] else { panic!("paragraph") };
    assert!(p1.runs[0].italic);
    let DocxBlock::Paragraph(p2) = &document.body[2] else { panic!("paragraph") };
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
    let numbering = decoded.xml_part("word/numbering.xml").expect("an unmodeled XML part is authoritative XML, never an opaque byte part").document.clone();
    assert!(decoded.opc.part("word/numbering.xml").is_none(), "an XML part is never duplicated into the binary lane");
    assert_eq!(xml_document_to_text(&numbering), "<w:numbering/>", "the unmodeled part's text survives decode");
    let re_encoded = encode_docx(&decoded).expect("re-encode");
    let re_decoded = decode_docx(&re_encoded).expect("re-decode");
    assert_eq!(re_decoded.xml_part("word/numbering.xml").map(|part| &part.document), Some(&numbering), "the unmodeled part survives encode/decode exactly");
    assert_eq!(re_decoded.project_document().expect("project document"), sample_document().await);
}

#[semio_framework_async_macros::async_test]
async fn unmodeled_run_properties_survive_round_trip() {
    let mut run = DocxRun { text: "colored".into(), ..Default::default() };
    run.extra_run_properties.push(XmlNode::Element { name: "w:color".into(), attrs: vec![XmlAttr { name: "w:val".into(), value: "FF0000".into() }], children: vec![] });
    let doc = DocxDocument { body: vec![DocxBlock::Paragraph(DocxParagraph { runs: vec![run], style: None, extra_paragraph_properties: Vec::new() })], styles: Vec::new() };
    let snap = build_minimal_docx(doc.clone());
    let bytes = encode_docx(&snap).expect("encode");
    let decoded = decode_docx(&bytes).expect("decode");
    assert_eq!(decoded.project_document().expect("project document"), doc);
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
    let analyzed_document = analyzed.project_document().expect("project analyzed document");
    let rebuilt = build_minimal_docx(analyzed_document.clone());
    let rebuilt_bytes = encode_docx(&rebuilt).expect("encode rebuilt");
    let reanalyzed = decode_docx(&rebuilt_bytes).expect("decode rebuilt");
    assert_eq!(reanalyzed.project_document().expect("project rebuilt document"), analyzed_document);
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

    /// 🖊️ The ONLY way the three shipped assets are ever refreshed: real `encode_docx`/`print_dsl`/
    /// `encode_pack` output of the demo, never a hand edit (`fixture_honesty_law` above is what that
    /// honesty means). Run it deliberately after a codec change — `cargo test -p
    /// semio-s-artifact-stdio-docx --lib -- --ignored zzz_write` — then re-run the law.
    #[semio_framework_async_macros::async_test]
    #[ignore]
    async fn zzz_write_native_docx_fixture() {
        let demo = demo_docx_snapshot().await;
        let assets = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/📚️examples/🎬️demo/🖼️assets");
        std::fs::write(assets.join("📜️example.docx"), encode_docx(&demo).expect("encode")).expect("write 📜️example.docx");
        std::fs::write(assets.join("🗣️.dsl.semio"), store::ArtifactDsl::print_dsl(&demo)).expect("write 🗣️.dsl.semio");
        std::fs::write(assets.join("🎒️.pack.semio"), store::ArtifactPack::encode_pack(&demo)).expect("write 🎒️.pack.semio");
    }
}
//#endregion 🔖️ConformanceLaws

#[test]
fn save_preserves_relationship_selected_part_paths_and_clears_the_last_style() {
    use crate::schema::diff::DocxBlockPath;
    use crate::schema::mutations::{apply_docx_mutation, remove_style, set_run_text, DocxMutation};
    use crate::standards::v_ecma_376::subsets::base::io::{MAIN_DOCUMENT_CONTENT_TYPE, MAIN_DOCUMENT_PART, REL_TYPE_STYLES, STYLES_CONTENT_TYPE};
    use semio_s_artifact_stdio_zip::opc::REL_TYPE_OFFICE_DOCUMENT;
    use std::io::Read;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/📦️save-part-identity/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let main = case["main"].as_str().unwrap();
        let styles = case["styles"].as_str().unwrap();
        let mut package = OpcPackage::empty();
        package.content_types.set_default("rels", RELS_CONTENT_TYPE);
        package.set_part(main, MAIN_DOCUMENT_CONTENT_TYPE, br#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body><w:p><w:r><w:t>Before</w:t></w:r></w:p></w:body></w:document>"#.to_vec());
        package.set_part(styles, STYLES_CONTENT_TYPE, br#"<w:styles xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:docDefaults/><w:style w:styleId="Normal"><w:name w:val="Normal"/></w:style></w:styles>"#.to_vec());
        package.add_relationship("", "main", REL_TYPE_OFFICE_DOCUMENT, main);
        package.add_relationship(main, "styles", REL_TYPE_STYLES, case["stylesTarget"].as_str().unwrap());
        if let Some(ids) = case["occupiedIds"].as_array() {
            for id in ids {
                package.add_relationship("", id.as_str().unwrap(), "urn:retained:root", main);
                package.add_relationship(main, id.as_str().unwrap(), "urn:retained:document", case["stylesTarget"].as_str().unwrap());
            }
        }
        let input = semio_s_artifact_stdio_zip::opc::encode_opc(&package).unwrap();
        assert!(sniff_docx_bytes(&input), "relationship-selected custom DOCX main part must be recognized");
        let mut snapshot = decode_docx(&input).unwrap();
        assert_eq!(snapshot.project_document().unwrap().styles.len(), 1);
        apply_docx_mutation(&mut snapshot, &DocxMutation::RemoveStyle(remove_style::RemoveStyle { id: "Normal".into() }));
        let address = crate::schema::mutations::docx_block_run_address(&snapshot, &DocxBlockPath { segments: Vec::new(), index: 0 }, 0).unwrap();
        apply_docx_mutation(&mut snapshot, &DocxMutation::SetRunText(set_run_text::SetRunText { address, text: fixture["text"].as_str().unwrap().into() }));
        if case["removeRequiredRelationships"].as_bool().unwrap_or(false) {
            snapshot.opc.relationships.get_mut("").unwrap().retain(|relationship| relationship.rel_type != REL_TYPE_OFFICE_DOCUMENT);
            snapshot.opc.relationships.get_mut(main).unwrap().retain(|relationship| relationship.rel_type != REL_TYPE_STYLES);
            let refused = snapshot.clone();
            assert!(encode_docx(&snapshot).is_err(), "missing required authored relationships must be refused before publication");
            assert_eq!(snapshot, refused, "refused publication must not mutate authored state");
            continue;
        }
        let before = snapshot.clone();
        let output = encode_docx(&snapshot).unwrap();
        assert_eq!(snapshot, before);
        let reopened = decode_docx(&output).unwrap();
        assert_eq!(reopened.project_document().unwrap(), snapshot.project_document().unwrap());
        assert_eq!(reopened.opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT).as_deref(), Some(main));
        assert_eq!(reopened.opc.resolve_relationship(main, REL_TYPE_STYLES).as_deref(), Some(styles));
        if let Some(ids) = case["occupiedIds"].as_array() {
            for owner in ["", main] {
                let relationships = reopened.opc.relationships_for(owner);
                assert_eq!(relationships.len(), ids.len() + 1);
                for id in ids {
                    assert!(relationships.iter().any(|relationship| relationship.id == id.as_str().unwrap() && relationship.rel_type.starts_with("urn:retained:")));
                }
                assert!(relationships.iter().any(|relationship| relationship.id == "rId3"));
            }
        }
        let mut archive = zip::ZipArchive::new(std::io::Cursor::new(&output)).unwrap();
        let mut main_xml = String::new();
        archive.by_name(main).unwrap().read_to_string(&mut main_xml).unwrap();
        assert!(main_xml.contains(fixture["text"].as_str().unwrap()));
        let mut styles_xml = String::new();
        archive.by_name(styles).unwrap().read_to_string(&mut styles_xml).unwrap();
        assert!(!styles_xml.contains("<w:style "));
        assert!(styles_xml.contains("w:docDefaults"));
        if main != MAIN_DOCUMENT_PART {
            assert!(archive.by_name(MAIN_DOCUMENT_PART).is_err());
        }
    }
    println!("[DEBUG] DOCX custom main/styles relationships and last-style removal survive save/reopen and independent ZIP inspection");
}

fn canonical_authority_fixture() -> (DocxSnapshot, serde_json::Value) {
    use crate::schema::snapshot::DocxXmlPart;
    use crate::standards::v_ecma_376::subsets::base::io::{MAIN_DOCUMENT_CONTENT_TYPE, REL_TYPE_STYLES, STYLES_CONTENT_TYPE};
    use semio_s_artifact_stdio_xml::schema::snapshot::xml_document_from_text;
    use semio_s_artifact_stdio_zip::opc::REL_TYPE_OFFICE_DOCUMENT;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🧬️canonical-xml-authority/🔣️.json")).unwrap();
    let main = &fixture["parts"][0];
    let styles = &fixture["parts"][1];
    let main_path = main["path"].as_str().unwrap();
    let styles_path = styles["path"].as_str().unwrap();
    let mut opc = OpcPackage::empty();
    opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
    opc.content_types.set_default("xml", "application/xml");
    opc.content_types.set_override(main_path, MAIN_DOCUMENT_CONTENT_TYPE);
    opc.content_types.set_override(styles_path, STYLES_CONTENT_TYPE);
    opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, main_path);
    opc.add_relationship(main_path, "rId2", REL_TYPE_STYLES, "../styles/styles.xml");
    let snapshot = DocxSnapshot::from_parts(
        opc,
        vec![
            DocxXmlPart { path: main_path.into(), content_type: MAIN_DOCUMENT_CONTENT_TYPE.into(), document: xml_document_from_text(main["xml"].as_str().unwrap()).unwrap() },
            DocxXmlPart { path: styles_path.into(), content_type: STYLES_CONTENT_TYPE.into(), document: xml_document_from_text(styles["xml"].as_str().unwrap()).unwrap() },
        ],
    );
    (snapshot, fixture)
}

fn canonical_marker_path(snapshot: &DocxSnapshot, part_path: &str, marker: &str) -> Vec<usize> {
    use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;
    fn locate(node: &XmlNode, marker: &str, path: &mut Vec<usize>) -> bool {
        let XmlNode::Element { attrs, children, .. } = node else { return false };
        if attrs.iter().any(|attr| attr.name.split_once(':').map_or(attr.name.as_str(), |(_, local)| local) == "marker" && attr.value == marker) {
            return true;
        }
        for (index, child) in children.iter().enumerate() {
            path.push(index);
            if locate(child, marker, path) {
                return true;
            }
            path.pop();
        }
        false
    }
    let root = snapshot.xml_part(part_path).unwrap().document.root.as_ref().unwrap();
    let mut path = Vec::new();
    assert!(locate(root, marker, &mut path), "missing marker {marker}");
    path
}

#[test]
fn canonical_xml_authority_edits_nested_run_without_losing_unknown_markup() {
    use crate::schema::mutations::{apply_docx_mutation, insert_table_row, set_paragraph_style, set_run_formatting, set_run_text, DocxMutation};
    use protocol::Mutation;
    use quick_xml::events::Event;
    use quick_xml::reader::Reader;
    use quick_xml::XmlVersion;
    use semio_s_artifact_stdio_xml::schema::snapshot::xml_document_to_text;
    use std::io::Read;
    let (mut snapshot, fixture) = canonical_authority_fixture();
    let original = snapshot.clone();
    snapshot.validate_authority().expect("fixture authority is valid");
    let projected = snapshot.project_document().expect("fixture projects");
    let DocxBlock::Paragraph(paragraph) = &projected.body[0] else { panic!("opening block is a paragraph") };
    assert_eq!(paragraph.runs[0].text, "Hello 🌍", "hyperlink-contained runs remain addressable");

    let main_path = fixture["parts"][0]["path"].as_str().unwrap();
    let edit = |id: &str| fixture["edits"].as_array().unwrap().iter().find(|edit| edit["id"] == id).unwrap();
    let mut inverses = Vec::new();
    fn commit(snapshot: &mut DocxSnapshot, mutation: DocxMutation, inverses: &mut Vec<DocxMutation>) {
        let inverse = Mutation::inverse(&mutation, &*snapshot);
        assert_eq!(inverse.len(), 1, "canonical edit has one compact inverse");
        assert!(!matches!(inverse[0], DocxMutation::SetSnapshot(_)), "canonical edit inverse is not a whole snapshot");
        let outcome = apply_docx_mutation(snapshot, &mutation);
        assert!(outcome.messages().is_empty(), "canonical edit refused: {:?}", outcome.messages());
        inverses.push(inverse.into_iter().next().unwrap());
    }
    let text_edit = edit("hyperlink-run-text");
    let address = crate::schema::mutations::docx_xml_address(&snapshot, main_path, canonical_marker_path(&snapshot, main_path, text_edit["targetMarker"].as_str().unwrap())).unwrap();
    commit(&mut snapshot, DocxMutation::SetRunText(set_run_text::SetRunText { address, text: text_edit["value"].as_str().unwrap().into() }), &mut inverses);
    let formatting_edit = edit("hyperlink-run-bold");
    let address = crate::schema::mutations::docx_xml_address(&snapshot, main_path, canonical_marker_path(&snapshot, main_path, formatting_edit["targetMarker"].as_str().unwrap())).unwrap();
    commit(&mut snapshot, DocxMutation::SetRunFormatting(set_run_formatting::SetRunFormatting { address, bold: formatting_edit["value"].as_bool().unwrap(), italic: true, underline: true }), &mut inverses);
    let style_edit = edit("paragraph-style");
    let address = crate::schema::mutations::docx_xml_address(&snapshot, main_path, canonical_marker_path(&snapshot, main_path, style_edit["targetMarker"].as_str().unwrap())).unwrap();
    commit(&mut snapshot, DocxMutation::SetParagraphStyle(set_paragraph_style::SetParagraphStyle { address, style_id: Some(style_edit["value"].as_str().unwrap().into()) }), &mut inverses);
    let row_edit = edit("table-row");
    let address = crate::schema::mutations::docx_xml_address(&snapshot, main_path, canonical_marker_path(&snapshot, main_path, row_edit["targetMarker"].as_str().unwrap())).unwrap();
    let cells = row_edit["value"]["cells"].as_array().unwrap().iter().map(|cell| cell.as_str().unwrap().to_string()).collect();
    commit(&mut snapshot, DocxMutation::InsertTableRow(insert_table_row::InsertTableRow { address, index: row_edit["value"]["index"].as_u64().unwrap() as usize, cells }), &mut inverses);

    let document_xml = xml_document_to_text(&snapshot.xml_part(main_path).unwrap().document);
    let mut reader = Reader::from_str(&document_xml);
    let mut rows = 0usize;
    let mut paragraph_styles = Vec::new();
    loop {
        match reader.read_event().unwrap() {
            Event::Start(event) | Event::Empty(event) if event.local_name().as_ref() == "tr" => rows += 1,
            Event::Start(event) | Event::Empty(event) if event.local_name().as_ref() == "pStyle" => {
                let value = event.attributes().map(|attr| attr.unwrap()).find(|attr| attr.key.local_name().as_ref() == "val").map(|attr| attr.normalized_value(XmlVersion::Explicit1_0).unwrap().into_owned());
                paragraph_styles.push(value);
            }
            Event::Eof => break,
            _ => {}
        }
    }
    assert_eq!(rows, 2, "quick-xml independently observes the inserted table row");
    assert_eq!(paragraph_styles, vec![Some("Quote".to_string())], "quick-xml independently observes the one paragraph style, now Quote");
    assert!(document_xml.contains("Edited 🚀") && document_xml.contains("New ßeta"));

    let changed = snapshot.clone();
    for inverse in inverses.into_iter().rev() {
        let outcome = apply_docx_mutation(&mut snapshot, &inverse);
        assert!(outcome.messages().is_empty(), "compact inverse refused: {:?}", outcome.messages());
    }
    assert_eq!(snapshot, original, "composed compact inverses restore exact authored authority");
    snapshot = changed;

    let authored_before_save = snapshot.clone();
    let bytes = encode_docx(&snapshot).expect("canonical authority publishes");
    assert_eq!(snapshot, authored_before_save, "publication never rewrites authored state");
    let reopened = decode_docx(&bytes).expect("published package reopens");
    assert_eq!(reopened, snapshot, "save/reopen retains exact logical XML/OPC authority");

    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(&bytes)).unwrap();
    for part in fixture["parts"].as_array().unwrap() {
        let mut xml = String::new();
        archive.by_name(part["path"].as_str().unwrap()).unwrap().read_to_string(&mut xml).unwrap();
        for marker in fixture["preservedMarkers"].as_array().unwrap() {
            let marker = marker.as_str().unwrap();
            if part["xml"].as_str().unwrap().contains(marker) {
                assert!(xml.contains(marker), "third-party ZIP inspection lost {marker}");
            }
        }
    }
    let document = reopened.project_document().unwrap();
    let DocxBlock::Paragraph(paragraph) = &document.body[0] else { panic!("opening block is a paragraph") };
    assert_eq!(paragraph.runs[0].text, "Edited 🚀");
    assert!(!paragraph.runs[0].bold && paragraph.runs[0].italic && paragraph.runs[0].underline);
}

#[test]
fn malformed_authored_authority_is_refused_atomically() {
    use crate::schema::diff::DocxDiff;
    use crate::schema::mutations::{apply_docx_mutation, set_snapshot, DocxMutation};
    use protocol::command::DiffAlgebra;
    use protocol::MutationDiff;
    use semio_s_artifact_stdio_zip::opc::{OpcPart, OpcTargetMode, REL_TYPE_OFFICE_DOCUMENT};
    let (valid, _) = canonical_authority_fixture();
    let main_path = valid.opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT).unwrap();

    let mut invalid = Vec::new();
    let mut duplicate = valid.clone();
    duplicate.xml_parts.push(duplicate.xml_parts[0].clone());
    invalid.push(duplicate);
    let mut overlap = valid.clone();
    overlap.opc.parts.push(OpcPart { path: overlap.xml_parts[0].path.clone(), content_type: "application/octet-stream".into(), bytes: vec![0] });
    invalid.push(overlap);
    let mut metadata = valid.clone();
    metadata.xml_parts[0].path = "[Content_Types].xml".into();
    invalid.push(metadata);
    let mut content_type_drift = valid.clone();
    content_type_drift.xml_parts[0].content_type = "application/xml".into();
    invalid.push(content_type_drift.clone());
    let mut external_main = valid.clone();
    external_main.opc.relationships.get_mut("").unwrap()[0].target_mode = OpcTargetMode::External;
    invalid.push(external_main);
    let mut duplicate_main = valid.clone();
    duplicate_main.opc.add_relationship("", "rId9", REL_TYPE_OFFICE_DOCUMENT, &main_path);
    invalid.push(duplicate_main);
    let mut missing_styles = valid.clone();
    missing_styles.opc.relationships.get_mut(&main_path).unwrap()[0].target = "missing/styles.xml".into();
    invalid.push(missing_styles);

    for candidate in invalid {
        assert!(candidate.validate_authority().is_err());
        assert!(encode_docx(&candidate).is_err());
    }

    let diff = DocxDiff::between(&valid, &content_type_drift);
    assert!(diff.apply(&valid).is_err(), "invalid candidate diff must be refused");
    let mut source = valid.clone();
    let outcome = apply_docx_mutation(&mut source, &DocxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: content_type_drift }));
    assert!(!outcome.messages().is_empty());
    assert_eq!(source, valid, "refused mutation must preserve the exact source snapshot");
}
