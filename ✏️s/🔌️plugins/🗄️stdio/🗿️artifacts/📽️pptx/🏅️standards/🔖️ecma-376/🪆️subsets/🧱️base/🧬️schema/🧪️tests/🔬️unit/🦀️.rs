use super::*;
use crate::schema::snapshot::{PptxParagraph, PptxRun, PptxShape, PptxSlide, PptxTransform};
use crate::standards::v_ecma_376::subsets::base::schema::construction::minimal::build_minimal_pptx;
use crate::standards::v_ecma_376::subsets::base::io::export::serializers::encode_pptx;
use crate::standards::v_ecma_376::subsets::base::io::import::deserializers::{decode_pptx, sniff_pptx_bytes};
use crate::standards::v_ecma_376::subsets::base::{io::{PptxAnalyzerAnalysis,PptxBuilderConstruction,MINIMAL_SLIDE_MASTER_XML},schema::{refusal::{PptxError},vocabulary::{PRESENTATION_CONTENT_TYPE,PRESENTATION_PART,REL_TYPE_OFFICE_DOCUMENT_STRICT,REL_TYPE_SLIDE,REL_TYPE_SLIDE_LAYOUT,REL_TYPE_SLIDE_MASTER,SLIDE_CONTENT_TYPE,SLIDE_LAYOUT_PART,SLIDE_MASTER_CONTENT_TYPE,SLIDE_MASTER_PART,THEME_PART}}};
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::xml_document_from_text;
use semio_s_artifact_stdio_zip::opc::{self, OpcPackage, RELS_CONTENT_TYPE, REL_TYPE_OFFICE_DOCUMENT};

fn construction_fidelity_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../🧫️fixtures/🏗️typed-construction-fidelity/🔣️.json")).expect("schema-first typed-construction fixture")
}

fn construction_fidelity_snapshot(fixture: &serde_json::Value) -> PptxSnapshot {
    let text = |key: &str| fixture[key].as_str().expect("fixture text");
    let mut opc = OpcPackage::empty();
    opc.comment = text("archiveComment").into();
    opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
    opc.content_types.set_default("xml", "application/xml");
    opc.set_part(PRESENTATION_PART, PRESENTATION_CONTENT_TYPE, text("presentationXml").as_bytes().to_vec());
    opc.set_part("ppt/slides/slide3.xml", SLIDE_CONTENT_TYPE, text("slideXml").as_bytes().to_vec());
    opc.set_part("ppt/slideMasters/slideMaster9.xml", "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml", text("masterXml").as_bytes().to_vec());
    opc.set_part("ppt/slideLayouts/slideLayout9.xml", "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml", text("layoutXml").as_bytes().to_vec());
    opc.set_part("ppt/notesSlides/notesSlide1.xml", "application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml", text("notesXml").as_bytes().to_vec());
    opc.set_part("ppt/theme/theme9.xml", "application/vnd.openxmlformats-officedocument.theme+xml", text("themeXml").as_bytes().to_vec());
    opc.set_part(text("customPartPath"), "application/xml", text("customXml").as_bytes().to_vec());
    opc.add_relationship("", "rId3", REL_TYPE_OFFICE_DOCUMENT_STRICT, PRESENTATION_PART);
    opc.add_relationship(PRESENTATION_PART, "rId9", "http://purl.oclc.org/ooxml/officeDocument/relationships/slideMaster", "slideMasters/slideMaster9.xml");
    opc.add_relationship(PRESENTATION_PART, "rId7", "http://purl.oclc.org/ooxml/officeDocument/relationships/slide", "slides/slide3.xml");
    opc.add_relationship(PRESENTATION_PART, "rId2", "http://purl.oclc.org/ooxml/officeDocument/relationships/notesMaster", "notesMasters/notesMaster1.xml");
    opc.add_relationship("ppt/slides/slide3.xml", "rId12", "http://purl.oclc.org/ooxml/officeDocument/relationships/slideLayout", "../slideLayouts/slideLayout9.xml");
    opc.add_relationship("ppt/slides/slide3.xml", "rId13", "http://purl.oclc.org/ooxml/officeDocument/relationships/notesSlide", "../notesSlides/notesSlide1.xml");
    opc.add_relationship("ppt/slideMasters/slideMaster9.xml", "rId4", "http://purl.oclc.org/ooxml/officeDocument/relationships/slideLayout", "../slideLayouts/slideLayout9.xml");
    opc.add_relationship("ppt/slideMasters/slideMaster9.xml", "rId6", "http://purl.oclc.org/ooxml/officeDocument/relationships/theme", "../theme/theme9.xml");
    opc.add_relationship("ppt/slideLayouts/slideLayout9.xml", "rId8", "http://purl.oclc.org/ooxml/officeDocument/relationships/slideMaster", "../slideMasters/slideMaster9.xml");
    opc.add_relationship("ppt/notesSlides/notesSlide1.xml", "rId1", "http://purl.oclc.org/ooxml/officeDocument/relationships/slide", "../slides/slide3.xml");
    decode_pptx(&opc::encode_opc(&opc).expect("fixture OPC encode")).expect("fixture PPTX decode")
}

fn xml_part<'a>(snapshot: &'a PptxSnapshot, path: &str) -> &'a crate::schema::snapshot::PptxXmlPart {
    snapshot.xml_parts.iter().find(|part| part.path == path).unwrap_or_else(|| panic!("missing retained XML part {path}"))
}

fn descendant_local<'a>(node: &'a semio_s_artifact_stdio_xml::schema::snapshot::XmlNode, local: &str) -> Option<&'a semio_s_artifact_stdio_xml::schema::snapshot::XmlNode> {
    let semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element { name, children, .. } = node else { return None };
    if name.rsplit_once(':').map_or(name.as_str(), |(_, local)| local) == local {
        return Some(node);
    }
    children.iter().find_map(|child| descendant_local(child, local))
}

async fn sample_presentation() -> PptxPresentation {
    PptxPresentation {
        slides: vec![
            PptxSlide {
                shapes: vec![PptxShape::Placeholder {
                    kind: "title".into(),
                    text_frame: vec![PptxParagraph { runs: vec![PptxRun { text: "Title Slide".into(), bold: true, italic: false, font_size: Some(44) }] }],
                    position: PptxTransform { x: 100, y: 200, cx: 300, cy: 400 },
                }],
            },
            PptxSlide {
                shapes: vec![
                    PptxShape::TextBox { text_frame: vec![PptxParagraph::text("Second slide, plain.")], position: PptxTransform::default() },
                    PptxShape::TextBox { text_frame: vec![PptxParagraph { runs: vec![PptxRun { text: "italic note".into(), bold: false, italic: true, font_size: None }] }], position: PptxTransform { x: 1, y: 2, cx: 3, cy: 4 } },
                    PptxShape::Picture { blip_rel_id: "rId5".into(), position: PptxTransform { x: 10, y: 20, cx: 30, cy: 40 } },
                ],
            },
        ],
    }
}

#[semio_framework_async_macros::async_test]
async fn builder_produces_minimal_valid_package_that_decodes_back() {
    let snap = build_minimal_pptx(sample_presentation().await);
    let bytes = encode_pptx(&snap).expect("encode minimal package");
    assert!(opc::sniff_opc_bytes(&bytes));
    assert!(sniff_pptx_bytes(&bytes));
    let decoded = decode_pptx(&bytes).expect("decode minimal package");
    assert_eq!(decoded.presentation().expect("derived PresentationML view"), sample_presentation().await);
    // The synthesized boilerplate chain must actually be present — a real reader needs it.
    assert!(decoded.xml_parts.iter().any(|part| part.path == SLIDE_MASTER_PART));
    assert!(decoded.xml_parts.iter().any(|part| part.path == SLIDE_LAYOUT_PART));
    assert!(decoded.xml_parts.iter().any(|part| part.path == THEME_PART));
}

#[semio_framework_async_macros::async_test]
async fn typed_construction_appends_to_imported_canonical_package_without_rebuilding_it() {
    use quick_xml::{events::Event, reader::Reader, XmlVersion};
    use semio_framework_plugin::ArtifactBuilder;
    use std::io::{Cursor, Read};

    let fixture = construction_fidelity_fixture();
    let imported = construction_fidelity_snapshot(&fixture);
    let original_slide = xml_part(&imported, "ppt/slides/slide3.xml").document.clone();
    let original_opc = imported.opc.clone();
    let paragraph_text = fixture["paragraphText"].as_str().expect("paragraph text");
    let with_paragraph = PptxBuilderConstruction::from_snapshot(imported.clone()).add_text_paragraph(paragraph_text).await.build().expect("append paragraph to imported package");

    assert_eq!(with_paragraph.opc, original_opc, "paragraph construction must not rebuild OPC metadata or binary authority");
    for path in ["ppt/presentation.xml", "ppt/slideMasters/slideMaster9.xml", "ppt/slideLayouts/slideLayout9.xml", "ppt/notesSlides/notesSlide1.xml", "ppt/theme/theme9.xml", fixture["customPartPath"].as_str().unwrap()] {
        assert_eq!(xml_part(&with_paragraph, path), xml_part(&imported, path), "unrelated retained part changed: {path}");
    }
    let old_tx_body = descendant_local(original_slide.root.as_ref().expect("old slide root"), "txBody").expect("old text body");
    let new_tx_body = descendant_local(xml_part(&with_paragraph, "ppt/slides/slide3.xml").document.root.as_ref().expect("new slide root"), "txBody").expect("new text body");
    let (semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element { children: old_children, .. }, semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element { children: new_children, .. }) = (old_tx_body, new_tx_body) else { panic!("text body elements") };
    assert_eq!(&new_children[..old_children.len()], old_children, "existing text subtree must remain an exact structural prefix");
    assert_eq!(new_children.len(), old_children.len() + 1, "construction appends exactly one paragraph");
    let semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element { name: appended_paragraph_name, .. } = new_children.last().expect("appended paragraph") else { panic!("appended paragraph element") };
    assert_eq!(appended_paragraph_name, "ink:p", "paragraph must use the DrawingML prefix in scope at the text body");

    let with_slide = PptxBuilderConstruction::from_snapshot(with_paragraph.clone()).add_slide().await.build().expect("append slide to imported package");
    for path in ["ppt/slides/slide3.xml", "ppt/slideMasters/slideMaster9.xml", "ppt/slideLayouts/slideLayout9.xml", "ppt/notesSlides/notesSlide1.xml", "ppt/theme/theme9.xml", fixture["customPartPath"].as_str().unwrap()] {
        assert_eq!(xml_part(&with_slide, path), xml_part(&with_paragraph, path), "existing part changed while appending slide: {path}");
    }
    let presentation_relationships = with_slide.opc.relationships_for(PRESENTATION_PART);
    assert_eq!(&presentation_relationships[..original_opc.relationships_for(PRESENTATION_PART).len()], original_opc.relationships_for(PRESENTATION_PART), "existing presentation relationship order and identities must be a prefix");
    let added_relationship = presentation_relationships.last().expect("new slide relationship");
    assert_eq!(added_relationship.id, "rId1");
    assert_eq!(added_relationship.rel_type, "http://purl.oclc.org/ooxml/officeDocument/relationships/slide");
    assert_eq!(added_relationship.target, "slides/slide1.xml");
    let added_slide = xml_part(&with_slide, "ppt/slides/slide1.xml");
    let added_slide_text = semio_s_artifact_stdio_zip::opc::xml_document_to_opc_text(&added_slide.document);
    assert!(added_slide_text.starts_with("<deck:sld "));
    assert!(added_slide_text.contains("xmlns:draw=\"http://purl.oclc.org/ooxml/drawingml/main\""));
    assert_eq!(with_slide.opc.relationships_for("ppt/slides/slide1.xml"), &[semio_s_artifact_stdio_zip::opc::OpcRelationship { id: "rId1".into(), rel_type: "http://purl.oclc.org/ooxml/officeDocument/relationships/slideLayout".into(), target: "../slideLayouts/slideLayout9.xml".into(), target_mode: semio_s_artifact_stdio_zip::opc::OpcTargetMode::Internal }]);

    let bytes = encode_pptx(&with_slide).expect("save retained construction");
    let mut archive = zip::ZipArchive::new(Cursor::new(&bytes)).expect("independent ZIP reopen");
    assert_eq!(archive.comment(), fixture["archiveComment"].as_str().unwrap().as_bytes());
    let mut presentation_xml = String::new();
    archive.by_name(PRESENTATION_PART).expect("presentation member").read_to_string(&mut presentation_xml).expect("presentation UTF-8");
    assert!(presentation_xml.contains(r#"<slot:sldId id="256" link:id="rId1"/>"#), "slide identity must use the namespaces in scope at the locally shadowed slide list");
    let mut custom_xml = String::new();
    archive.by_name(fixture["customPartPath"].as_str().unwrap()).expect("custom member").read_to_string(&mut custom_xml).expect("custom UTF-8");
    assert_eq!(custom_xml, fixture["customXml"].as_str().unwrap());
    let mut reader = Reader::from_str(&presentation_xml);
    let mut slide_ids = Vec::new();
    let mut size = None;
    loop {
        match reader.read_event().expect("independent presentation XML parse") {
            Event::Start(event) | Event::Empty(event) if event.local_name().as_ref() == "sldId" => {
                let id = event.attributes().map(Result::unwrap).find(|attribute| attribute.key.local_name().as_ref() == "id").expect("slide id").normalized_value(XmlVersion::Explicit1_0).unwrap().into_owned();
                slide_ids.push(id);
            }
            Event::Start(event) | Event::Empty(event) if event.local_name().as_ref() == "sldSz" => {
                let attrs = event.attributes().map(Result::unwrap).map(|attribute| (attribute.key.local_name().as_ref().to_string(), attribute.normalized_value(XmlVersion::Explicit1_0).unwrap().into_owned())).collect::<std::collections::BTreeMap<_, _>>();
                size = Some(attrs);
            }
            Event::Eof => break,
            _ => {}
        }
    }
    assert_eq!(slide_ids, vec!["319", "256"]);
    let size = size.expect("retained presentation dimensions");
    assert_eq!(size.get("cx").map(String::as_str), fixture["presentationSize"]["cx"].as_str());
    assert_eq!(size.get("cy").map(String::as_str), fixture["presentationSize"]["cy"].as_str());
    assert_eq!(size.get("type").map(String::as_str), fixture["presentationSize"]["type"].as_str());
    drop(archive);
    let reopened = decode_pptx(&bytes).expect("own reopen after independent oracles");
    assert_eq!(reopened, with_slide);

    let fresh = PptxBuilderConstruction::empty()
        .add_slide()
        .await
        .add_text_paragraph("Fresh authority")
        .await
        .build()
        .expect("minimal construction from genuinely empty authority");
    let fresh_presentation = fresh.presentation().expect("fresh derived PresentationML view");
    assert_eq!(fresh_presentation.slides.len(), 1);
    assert_eq!(fresh_presentation.slides[0].shapes.len(), 1);
    let fresh_bytes = encode_pptx(&fresh).expect("save fresh construction");
    assert_eq!(decode_pptx(&fresh_bytes).expect("reopen fresh construction"), fresh);
}

#[semio_framework_async_macros::async_test]
async fn decode_resolves_real_hand_built_package_with_shape_boundaries_and_position() {
    // Hand-built OOXML: a slide with TWO real shapes -- a positioned placeholder title and a
    // positioned picture -- exercising real shape-BOUNDARY recovery (not flattened text) and
    // real `a:xfrm` position decoding.
    let mut opc = OpcPackage::empty();
    opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
    opc.content_types.set_default("xml", "application/xml");

    let slide_xml = concat!(
        r#"<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">"#,
        "<p:cSld><p:spTree>",
        r#"<p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr/>"#,
        "<p:sp><p:nvSpPr><p:cNvPr id=\"2\" name=\"Title\"/><p:cNvSpPr/><p:nvPr><p:ph type=\"title\"/></p:nvPr></p:nvSpPr>",
        r#"<p:spPr><a:xfrm><a:off x="111" y="222"/><a:ext cx="333" cy="444"/></a:xfrm></p:spPr>"#,
        r#"<p:txBody><a:bodyPr/><a:p><a:r><a:rPr b="1" i="1" sz="4400"/><a:t>Nested &amp; bold-italic</a:t></a:r></a:p></p:txBody>"#,
        "</p:sp>",
        r#"<p:pic><p:nvPicPr><p:cNvPr id="3" name="Pic"/><p:cNvPicPr/><p:nvPr/></p:nvPicPr>"#,
        r#"<p:blipFill><a:blip r:embed="rId9"/><a:stretch><a:fillRect/></a:stretch></p:blipFill>"#,
        r#"<p:spPr><a:xfrm><a:off x="5" y="6"/><a:ext cx="7" cy="8"/></a:xfrm></p:spPr></p:pic>"#,
        "</p:spTree></p:cSld></p:sld>",
    );
    opc.set_part("ppt/slides/slide1.xml", SLIDE_CONTENT_TYPE, slide_xml.as_bytes().to_vec());
    opc.add_relationship("ppt/slides/slide1.xml", "rId1", REL_TYPE_SLIDE_LAYOUT, "../slideLayouts/slideLayout1.xml");

    let presentation_xml = concat!(
        r#"<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">"#,
        r#"<p:sldMasterIdLst><p:sldMasterId id="2147483648" r:id="rId1"/></p:sldMasterIdLst>"#,
        r#"<p:sldIdLst><p:sldId id="256" r:id="rId2"/></p:sldIdLst>"#,
        "</p:presentation>",
    );
    opc.set_part(PRESENTATION_PART, PRESENTATION_CONTENT_TYPE, presentation_xml.as_bytes().to_vec());
    opc.add_relationship(PRESENTATION_PART, "rId1", REL_TYPE_SLIDE_MASTER, "slideMasters/slideMaster1.xml");
    opc.add_relationship(PRESENTATION_PART, "rId2", REL_TYPE_SLIDE, "slides/slide1.xml");
    opc.set_part(SLIDE_MASTER_PART, SLIDE_MASTER_CONTENT_TYPE, MINIMAL_SLIDE_MASTER_XML.as_bytes().to_vec());
    opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, PRESENTATION_PART);

    let bytes = opc::encode_opc(&opc).expect("encode hand-built package");
    let decoded = decode_pptx(&bytes).expect("decode hand-built pptx");

    assert_eq!(decoded.presentation().expect("derived PresentationML view").slides.len(), 1);
    let shapes = &decoded.presentation().expect("derived PresentationML view").slides[0].shapes;
    assert_eq!(shapes.len(), 2, "two DIRECT shapes must be recovered as two distinct PptxShape entries, not flattened");
    let PptxShape::Placeholder { kind, text_frame, position } = &shapes[0] else { panic!("expected placeholder shape") };
    assert_eq!(kind, "title");
    assert_eq!(*position, PptxTransform { x: 111, y: 222, cx: 333, cy: 444 });
    assert_eq!(text_frame[0].runs[0].text, "Nested & bold-italic");
    assert!(text_frame[0].runs[0].bold && text_frame[0].runs[0].italic);
    assert_eq!(text_frame[0].runs[0].font_size, Some(44));
    let PptxShape::Picture { blip_rel_id, position } = &shapes[1] else { panic!("expected picture shape") };
    assert_eq!(blip_rel_id, "rId9");
    assert_eq!(*position, PptxTransform { x: 5, y: 6, cx: 7, cy: 8 });
}

#[semio_framework_async_macros::async_test]
async fn decode_preserves_unmodeled_shape_kinds_as_logical_other() {
    // A `p:graphicFrame` (chart/table/SmartArt) direct child -- not typed by this layer --
    // must survive decode->encode->decode as a logical `PptxShape::Other` XML node.
    let mut opc = OpcPackage::empty();
    opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
    opc.content_types.set_default("xml", "application/xml");
    let graphic_frame = r#"<p:graphicFrame><p:nvGraphicFramePr><p:cNvPr id="9" name="Table 1"/></p:nvGraphicFramePr><a:graphic/></p:graphicFrame>"#;
    let slide_xml = format!(
        concat!(
            r#"<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">"#,
            "<p:cSld><p:spTree>",
            r#"<p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr/>"#,
            "{}",
            "</p:spTree></p:cSld></p:sld>",
        ),
        graphic_frame,
    );
    opc.set_part("ppt/slides/slide1.xml", SLIDE_CONTENT_TYPE, slide_xml.into_bytes());
    opc.add_relationship("ppt/slides/slide1.xml", "rId1", REL_TYPE_SLIDE_LAYOUT, "../slideLayouts/slideLayout1.xml");
    let presentation_xml = concat!(
        r#"<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">"#,
        r#"<p:sldMasterIdLst><p:sldMasterId id="2147483648" r:id="rId1"/></p:sldMasterIdLst>"#,
        r#"<p:sldIdLst><p:sldId id="256" r:id="rId2"/></p:sldIdLst>"#,
        "</p:presentation>",
    );
    opc.set_part(PRESENTATION_PART, PRESENTATION_CONTENT_TYPE, presentation_xml.as_bytes().to_vec());
    opc.add_relationship(PRESENTATION_PART, "rId1", REL_TYPE_SLIDE_MASTER, "slideMasters/slideMaster1.xml");
    opc.add_relationship(PRESENTATION_PART, "rId2", REL_TYPE_SLIDE, "slides/slide1.xml");
    opc.set_part(SLIDE_MASTER_PART, SLIDE_MASTER_CONTENT_TYPE, MINIMAL_SLIDE_MASTER_XML.as_bytes().to_vec());
    opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, PRESENTATION_PART);

    let bytes = opc::encode_opc(&opc).expect("encode");
    let decoded = decode_pptx(&bytes).expect("decode");
    assert_eq!(decoded.presentation().expect("derived PresentationML view").slides[0].shapes.len(), 1);
    let PptxShape::Other { node } = &decoded.presentation().expect("derived PresentationML view").slides[0].shapes[0] else { panic!("expected Other shape") };
    let node_model = format!("{node:?}");
    assert!(node_model.contains("p:graphicFrame") && node_model.contains("Table 1"));

    // Re-encode -> re-decode: the logical XML node must survive the round trip.
    let re_encoded = encode_pptx(&decoded).expect("re-encode");
    let re_decoded = decode_pptx(&re_encoded).expect("re-decode");
    assert_eq!(re_decoded.presentation().expect("derived PresentationML view"), decoded.presentation().expect("derived PresentationML view"));
}

#[semio_framework_async_macros::async_test]
async fn decode_resolves_strict_office_document_relationship_too() {
    // 🏅️ A genuine ISO/IEC 29500-1 Strict package's root relationship carries
    // `REL_TYPE_OFFICE_DOCUMENT_STRICT`, never the Transitional type this engine's own writer
    // emits -- `decode_pptx`/`sniff_pptx_bytes` must still recognize it (ticket
    // 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES, so the `🔒️strict` subset's
    // analyzer can ever see real Strict bytes).
    let mut opc = OpcPackage::empty();
    opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
    opc.content_types.set_default("xml", "application/xml");
    let presentation_xml = concat!(
        r#"<p:presentation xmlns:a="http://purl.oclc.org/ooxml/drawingml/main" xmlns:p="http://purl.oclc.org/ooxml/presentationml/main" xmlns:r="http://purl.oclc.org/ooxml/officeDocument/relationships">"#,
        r#"<p:sldMasterIdLst><p:sldMasterId id="2147483648" r:id="rId1"/></p:sldMasterIdLst>"#,
        r#"<p:sldIdLst/>"#,
        "</p:presentation>",
    );
    opc.set_part(PRESENTATION_PART, PRESENTATION_CONTENT_TYPE, presentation_xml.as_bytes().to_vec());
    opc.set_part(SLIDE_MASTER_PART, SLIDE_MASTER_CONTENT_TYPE, MINIMAL_SLIDE_MASTER_XML.as_bytes().to_vec());
    opc.add_relationship(PRESENTATION_PART, "rId1", REL_TYPE_SLIDE_MASTER, "slideMasters/slideMaster1.xml");
    opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT_STRICT, PRESENTATION_PART);

    let bytes = opc::encode_opc(&opc).expect("encode hand-built Strict package");
    assert!(sniff_pptx_bytes(&bytes), "Strict-relationship-typed package must still sniff as pptx");
    let decoded = decode_pptx(&bytes).expect("decode Strict-relationship-typed package");
    assert_eq!(decoded.presentation().expect("derived PresentationML view").slides.len(), 0);
}

#[semio_framework_async_macros::async_test]
async fn decode_rejects_missing_presentation_relationship() {
    let mut opc = OpcPackage::empty();
    opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
    let bytes = opc::encode_opc(&opc).expect("encode");
    let err = decode_pptx(&bytes).expect_err("must reject a package with no officeDocument relationship");
    assert_eq!(err, PptxError::MissingPresentationRelationship);
}

#[semio_framework_async_macros::async_test]
async fn unmodeled_slide_master_survives_decode_encode_logically() {
    let mut snap = build_minimal_pptx(sample_presentation().await);
    // Replace the synthesized slide master with a distinguishable "real" one before encoding. The
    // slide master is an XML part, so its authority is `xml_parts` (since the logical split,
    // `opc.parts` carries only the parts `pptx_part_is_xml` rejects), and the package is written
    // by the real `encode_pptx` -- `opc::encode_opc` alone would emit a package with no
    // `ppt/presentation.xml` at all, because that part is materialized by the pptx encoder.
    let marker = "<p:sldMaster marker=\"real-file\"/>";
    let master = snap.xml_parts.iter_mut().find(|part| part.path == SLIDE_MASTER_PART).expect("synthesized slide master");
    assert_eq!(master.content_type, SLIDE_MASTER_CONTENT_TYPE);
    master.document = xml_document_from_text(marker).expect("parse marker slide master");
    let bytes = encode_pptx(&snap).expect("encode");

    let decoded = decode_pptx(&bytes).expect("decode");
    assert_eq!(decoded.part_text(SLIDE_MASTER_PART).as_deref(), Some(marker));
    let re_encoded = encode_pptx(&decoded).expect("re-encode must not clobber an already-present slide master");
    let re_decoded = decode_pptx(&re_encoded).expect("re-decode");
    assert_eq!(re_decoded.part_text(SLIDE_MASTER_PART).as_deref(), Some(marker));
    assert_eq!(re_decoded.presentation().expect("derived PresentationML view"), sample_presentation().await);
}

#[semio_framework_async_macros::async_test]
async fn analyzer_builder_round_trip() {
    let original = build_minimal_pptx(sample_presentation().await);
    let bytes = encode_pptx(&original).expect("encode");
    let analyzed = decode_pptx(&bytes).expect("decode");
    let rebuilt = build_minimal_pptx(analyzed.presentation().expect("derived PresentationML view").clone());
    let rebuilt_bytes = encode_pptx(&rebuilt).expect("encode rebuilt");
    let reanalyzed = decode_pptx(&rebuilt_bytes).expect("decode rebuilt");
    assert_eq!(reanalyzed.presentation().expect("derived PresentationML view"), analyzed.presentation().expect("derived PresentationML view"));
}

#[semio_framework_async_macros::async_test]
async fn shrinking_slide_count_drops_stale_slide_parts_and_relationships() {
    let mut wide = sample_presentation().await;
    let snap_wide = build_minimal_pptx(wide.clone());
    assert!(!snap_wide.opc.relationships_for("ppt/slides/slide2.xml").is_empty());

    wide.slides.truncate(1);
    let bytes = encode_pptx(&build_minimal_pptx(wide)).expect("encode freshly authored narrower presentation");
    let decoded = decode_pptx(&bytes).expect("decode");
    assert!(decoded.opc.relationships_for("ppt/slides/slide2.xml").is_empty(), "stale second slide's relationships must be dropped too");
    assert_eq!(decoded.presentation().expect("derived PresentationML view").slides.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn repeated_materialization_is_deterministic() {
    let snap = build_minimal_pptx(sample_presentation().await);
    let once = encode_pptx(&snap).expect("first materialization");
    let decoded = decode_pptx(&once).expect("decode first materialization");
    let twice = encode_pptx(&decoded).expect("second materialization");
    assert_eq!(once, twice);
}

//#region 🔖️ExactSourceRoundtrip
/// 📽️ The committed 7-slide package the exact-export laws run on. Its ZIP framing (no Office
/// alignment padding, stored timestamps) is not the writer's, so every pipeline is held byte-exact to
/// the export of its import, and that export to re-import as the same logical presentation.
async fn exact_pptx_bytes() -> Vec<u8> {
    include_bytes!("../../../🧫️fixtures/📽️.pptx").to_vec()
}

async fn local_member_names(bytes: &[u8]) -> Vec<String> {
    let mut names = Vec::new();
    let mut offset = 0usize;
    while bytes.get(offset..offset + 4) == Some(b"PK\x03\x04") {
        let compressed = u32::from_le_bytes(bytes[offset + 18..offset + 22].try_into().expect("compressed size")) as usize;
        let name_len = u16::from_le_bytes(bytes[offset + 26..offset + 28].try_into().expect("name length")) as usize;
        let extra_len = u16::from_le_bytes(bytes[offset + 28..offset + 30].try_into().expect("extra length")) as usize;
        let name_start = offset + 30;
        names.push(String::from_utf8(bytes[name_start..name_start + name_len].to_vec()).expect("UTF-8 member name"));
        offset = name_start + name_len + extra_len + compressed;
    }
    names
}

async fn local_compressed_members(bytes: &[u8]) -> Vec<(String, u16, u16, u16, u16, u16, u32, u32, Vec<u8>, Vec<u8>)> {
    let mut members = Vec::new();
    let mut offset = 0usize;
    while bytes.get(offset..offset + 4) == Some(b"PK\x03\x04") {
        let flags = u16::from_le_bytes(bytes[offset + 6..offset + 8].try_into().expect("flags"));
        let method = u16::from_le_bytes(bytes[offset + 8..offset + 10].try_into().expect("method"));
        let version = u16::from_le_bytes(bytes[offset + 4..offset + 6].try_into().expect("version"));
        let time = u16::from_le_bytes(bytes[offset + 10..offset + 12].try_into().expect("time"));
        let date = u16::from_le_bytes(bytes[offset + 12..offset + 14].try_into().expect("date"));
        let crc = u32::from_le_bytes(bytes[offset + 14..offset + 18].try_into().expect("crc"));
        let compressed = u32::from_le_bytes(bytes[offset + 18..offset + 22].try_into().expect("compressed size")) as usize;
        let uncompressed = u32::from_le_bytes(bytes[offset + 22..offset + 26].try_into().expect("uncompressed size"));
        let name_len = u16::from_le_bytes(bytes[offset + 26..offset + 28].try_into().expect("name length")) as usize;
        let extra_len = u16::from_le_bytes(bytes[offset + 28..offset + 30].try_into().expect("extra length")) as usize;
        let name_start = offset + 30;
        let payload_start = name_start + name_len + extra_len;
        members.push((
            String::from_utf8(bytes[name_start..name_start + name_len].to_vec()).expect("UTF-8 member name"),
            version,
            flags,
            method,
            time,
            date,
            crc,
            uncompressed,
            bytes[name_start + name_len..payload_start].to_vec(),
            bytes[payload_start..payload_start + compressed].to_vec(),
        ));
        offset = payload_start + compressed;
    }
    members
}

async fn assert_exact_export(snapshot: &PptxSnapshot, expected: &[u8]) {
    let actual = encode_pptx(snapshot).expect("export exact fixture");
    if actual != expected {
        let first_diff = actual.iter().zip(expected).position(|(left, right)| left != right).unwrap_or(actual.len().min(expected.len()));
        let actual_names = local_member_names(&actual).await;
        let expected_names = local_member_names(expected).await;
        let first_order_diff = actual_names.iter().zip(&expected_names).position(|(left, right)| left != right).unwrap_or(actual_names.len().min(expected_names.len()));
        let actual_compressed = local_compressed_members(&actual).await;
        let expected_compressed = local_compressed_members(expected).await;
        let first_compressed = actual_compressed.iter().zip(&expected_compressed).position(|(left, right)| left != right).unwrap_or(actual_compressed.len().min(expected_compressed.len()));
        let compressed_detail = actual_compressed
                .get(first_compressed)
                .zip(expected_compressed.get(first_compressed))
                .map(|(left, right)| {
                    let prefix = left.9.iter().zip(&right.9).position(|(actual, expected)| actual != expected).unwrap_or(left.9.len().min(right.9.len()));
                    format!(" compressed_name={} actual_version={} expected_version={} actual_flags={} expected_flags={} actual_method={} expected_method={} actual_time={} expected_time={} actual_date={} expected_date={} actual_crc={} expected_crc={} actual_uncompressed={} expected_uncompressed={} actual_extra={:?} expected_extra={:?} actual_compressed={} expected_compressed={} compressed_prefix={prefix}", left.0, left.1, right.1, left.2, right.2, left.3, right.3, left.4, right.4, left.5, right.5, left.6, right.6, left.7, right.7, left.8, right.8, left.9.len(), right.9.len())
                })
                .unwrap_or_default();
        let actual_zip = semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::io::decode_zip(&actual).expect("decode actual mismatch");
        let expected_zip = semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::io::decode_zip(expected).expect("decode expected mismatch");
        let first_entry = actual_zip.entries.iter().zip(&expected_zip.entries).position(|(left, right)| left != right).unwrap_or(actual_zip.entries.len().min(expected_zip.entries.len()));
        let logical_mismatch_separator = ",";
        let logical_mismatches = actual_zip
            .entries
            .iter()
            .zip(&expected_zip.entries)
            .filter(|(left, right)| left != right)
            .take(16)
            .map(|(left, right)| {
                let offset = left.data.iter().zip(&right.data).position(|(a, b)| a != b).unwrap_or(left.data.len().min(right.data.len()));
                let start = offset.saturating_sub(12);
                let actual_end = (offset + 44).min(left.data.len());
                let expected_end = (offset + 44).min(right.data.len());
                format!("{}:{}/{}@{}:{:?}!={:?}", left.name, left.data.len(), right.data.len(), offset, String::from_utf8_lossy(&left.data[start..actual_end]), String::from_utf8_lossy(&right.data[start..expected_end]),)
            })
            .collect::<Vec<_>>()
            .join(logical_mismatch_separator);
        let entry_detail = actual_zip
            .entries
            .get(first_entry)
            .zip(expected_zip.entries.get(first_entry))
            .map(|(left, right)| {
                let data_diff = left.data.iter().zip(&right.data).position(|(a, b)| a != b).unwrap_or(left.data.len().min(right.data.len()));
                let start = data_diff.saturating_sub(24);
                let actual_end = (data_diff + 72).min(left.data.len());
                let expected_end = (data_diff + 72).min(right.data.len());
                let actual_snippet = String::from_utf8_lossy(&left.data[start..actual_end]);
                let expected_snippet = String::from_utf8_lossy(&right.data[start..expected_end]);
                format!(" name={} actual_data_len={} expected_data_len={} data_diff={data_diff} actual_snippet={actual_snippet:?} expected_snippet={expected_snippet:?}", left.name, left.data.len(), right.data.len())
            })
            .unwrap_or_default();
        panic!(
            "PPTX exact export mismatch: actual_len={} expected_len={} first_diff={first_diff} first_order_diff={first_order_diff} first_compressed={first_compressed}{compressed_detail} first_entry={first_entry}{entry_detail} logical_mismatches=[{logical_mismatches}]",
            actual.len(),
            expected.len()
        );
    }
}

fn first_positioned_shape(snapshot: &PptxSnapshot) -> (crate::schema::mutations::PptxShapeAddress, PptxTransform) {
    for slide in crate::schema::mutations::xml_address::pptx_slides(snapshot).expect("retained slide authorities") {
        for shape in slide.shapes {
            if let Some(position) = shape.position {
                return (shape.address, position);
            }
        }
    }
    panic!("exact fixture has no positioned shape");
}

#[semio_framework_async_macros::async_test]
async fn fixture_survives_logical_io_persistence_diff_and_mutation_pipelines() {
    use crate::schema::mutations::{set_shape_position, set_shape_text};
    use crate::{PptxDiff, PptxMutation};
    use protocol::{DiffAlgebra, DiffBinary,DiffCodec,DiffText, Mutation, MutationDiff, OpBinary, OpText};
    use semio_framework_plugin::{io::AnalyzeSource, ArtifactAnalysis, ArtifactComposition, io::ComposeSource};

    let snapshot = decode_pptx(&exact_pptx_bytes().await).expect("import exact fixture");
    assert_eq!(snapshot.presentation().expect("derived fixture presentation").slides.len(), 7);
    assert!(!snapshot.xml_parts.is_empty());
    let xml_paths: std::collections::HashSet<&str> = snapshot.xml_parts.iter().map(|part| part.path.as_str()).collect();
    assert_eq!(xml_paths.len(), snapshot.xml_parts.len(), "every logical XML part must have one authority");
    assert!(crate::schema::snapshot::pptx_part_is_xml("ppt/drawings/vmlDrawing1.vml", "application/vnd.openxmlformats-officedocument.vmlDrawing"), "VML must be parsed as logical XML");
    assert!(snapshot.opc.parts.iter().all(|part| !crate::schema::snapshot::pptx_part_is_xml(&part.path, &part.content_type)), "OPC byte parts must contain no XML");
    assert!(snapshot.opc.parts.iter().all(|part| !xml_paths.contains(part.path.as_str())), "XML and binary authorities must be disjoint");
    assert!(
        snapshot.opc.parts.iter().all(|part| {
            let path = part.path.to_ascii_lowercase();
            [".png", ".jpg", ".jpeg", ".emf", ".bin"].iter().any(|extension| path.ends_with(extension))
        }),
        "fixture binary parts must be genuine media or embedded object payloads"
    );
    let relationship_parts = snapshot.opc.relationships.groups().map(|(_, relationships)| relationships).filter(|relationships| !relationships.is_empty()).count();
    assert_eq!(1 + relationship_parts + snapshot.xml_parts.len() + snapshot.opc.parts.len(), 55, "every native member must map to exactly one logical authority");
    let mut duplicate_authority = snapshot.clone();
    let duplicated = duplicate_authority.xml_parts.first().expect("fixture XML part");
    duplicate_authority.opc.parts.push(opc::OpcPart { path: duplicated.path.clone(), content_type: duplicated.content_type.clone(), bytes: b"<shadow/>".to_vec() });
    assert!(encode_pptx(&duplicate_authority).is_err(), "export must reject XML stored as opaque OPC bytes");
    let exact_bytes = encode_pptx(&snapshot).expect("export imported fixture");
    assert_eq!(decode_pptx(&exact_bytes).expect("re-import own export"), snapshot, "the export of an imported package re-imports to the same logical presentation");
    let source_bytes = exact_pptx_bytes().await;

    let analysis = PptxAnalyzerAnalysis::analyze(&[AnalyzeSource::Binary(&source_bytes)]);
    let analyzed = analysis.parts.snapshot.expect("analyze native PPTX fixture");
    assert_eq!(analyzed, snapshot);
    assert_exact_export(&analyzed, &exact_bytes).await;

    let dialect = <PptxAnalyzerAnalysis as ArtifactAnalysis>::DIALECT;
    let composition = crate::standards::v_ecma_376::subsets::base::io::PptxComposerComposition::compose(&[ComposeSource { dialect, payload: AnalyzeSource::Binary(&source_bytes) }]).expect("compose native PPTX fixture");
    assert_eq!(composition.snapshot, snapshot);
    assert_exact_export(&composition.snapshot, &exact_bytes).await;

    let zip = semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::io::decode_zip(&source_bytes).expect("decode exact zip");
    assert_eq!(zip.entries.len(), 55);
    assert_eq!(zip.entries.iter().filter(|entry| entry.name.starts_with("ppt/slides/slide") && entry.name.ends_with(".xml")).count(), 7);
    assert_eq!(zip.entries.iter().filter(|entry| entry.name.ends_with(".rels")).count(), 22);

    let packed = store::ArtifactPack::encode_pack(&snapshot);
    let unpacked = <PptxSnapshot as store::ArtifactPack>::decode_pack(&packed).expect("unpack exact fixture");
    assert_eq!(unpacked, snapshot);
    assert_exact_export(&unpacked, &exact_bytes).await;

    let binary = crate::standards::v_ecma_376::subsets::base::io::export::serializers::artifacts::zip::v2_0::base::serialize(&snapshot).expect("serialize exact fixture to binary");
    let from_binary = crate::standards::v_ecma_376::subsets::base::io::import::deserializers::artifacts::zip::v2_0::base::deserialize(&binary).expect("deserialize exact fixture from binary");
    assert_eq!(from_binary, snapshot);
    assert_exact_export(&from_binary, &exact_bytes).await;

    let dsl = store::ArtifactDsl::print_dsl(&snapshot);
    let parsed = <PptxSnapshot as store::ArtifactDsl>::parse_dsl(&dsl).expect("parse exact fixture dsl");
    assert_eq!(parsed, snapshot);
    assert_exact_export(&parsed, &exact_bytes).await;

    let self_diff = PptxDiff::between(&snapshot, &snapshot);
    assert!(self_diff.is_empty());
    assert_exact_export(&protocol::apply_diff(&self_diff, &snapshot).unwrap(), &exact_bytes).await;

    let (address, position) = first_positioned_shape(&snapshot);
    let mut missing = address.clone();
    missing.node.part_path = "ppt/slides/missing.xml".into();
    let mut rejected = snapshot.clone();
    let refusal = crate::schema::mutations::apply_pptx_mutation(&mut rejected, &PptxMutation::SetShapeText(set_shape_text::SetShapeText { address: missing, text: String::new() }));
    assert!(refusal.diff().is_empty());
    assert!(!refusal.messages().is_empty());
    assert_eq!(rejected, snapshot);
    assert_exact_export(&rejected, &exact_bytes).await;

    let changed_x = if position.x == i64::MAX { position.x - 1 } else { position.x + 1 };
    let mutation = PptxMutation::SetShapePosition(set_shape_position::SetShapePosition { address, position: PptxTransform { x: changed_x, ..position } });
    let mut changed = snapshot.clone();
    let forward = crate::schema::mutations::apply_pptx_mutation(&mut changed, &mutation);
    assert_ne!(changed, snapshot);
    assert_ne!(encode_pptx(&changed).expect("encode mutated presentation"), exact_bytes);
    for inverse in mutation.inverse(&snapshot).expect("valid retained mutation inverse fixture") {
        crate::schema::mutations::apply_pptx_mutation(&mut changed, &inverse);
    }
    assert_eq!(changed, snapshot);
    assert_exact_export(&changed, &exact_bytes).await;

    let mid = protocol::apply_diff(forward.diff(), &snapshot).unwrap();
    let inverse = forward.diff().inverse(&snapshot);
    let mut absorbed = forward.diff().clone();
    absorbed.absorb(inverse.clone());
    let restored = protocol::apply_diff(&inverse, &mid).unwrap();
    assert_eq!(restored, snapshot);
    assert_eq!(protocol::apply_diff(&absorbed, &snapshot).unwrap(), snapshot);
    assert_exact_export(&restored, &exact_bytes).await;
    assert_exact_export(&protocol::apply_diff(&absorbed, &snapshot).unwrap(), &exact_bytes).await;

    let mut without_xml_parts = snapshot.clone();
    without_xml_parts.xml_parts.clear();
    let xml_parts_diff = PptxDiff::between(&without_xml_parts, &snapshot);
    let printed_diff = xml_parts_diff.print_diff();
    let parsed_diff = PptxDiff::parse_diff(&printed_diff).expect("parse logical XML parts diff");
    assert_exact_export(&protocol::apply_diff(&parsed_diff, &without_xml_parts).unwrap(), &exact_bytes).await;
    let encoded_diff = xml_parts_diff.encode_diff().expect("encode logical XML parts diff");
    let decoded_diff = PptxDiff::decode_diff(&encoded_diff).expect("decode logical XML parts diff");
    assert_exact_export(&protocol::apply_diff(&decoded_diff, &without_xml_parts).unwrap(), &exact_bytes).await;

    let part = &snapshot.xml_parts[0];
    let replace = PptxMutation::ReplaceXmlNode(crate::schema::mutations::replace_xml_node::ReplaceXmlNode {
        address: crate::schema::mutations::xml_address::pptx_xml_address(&snapshot, &part.path, Vec::new()).expect("root address"),
        node: part.document.root.clone().expect("root"),
    });
    assert_eq!(PptxMutation::parse_op(&replace.print_op()).expect("parse replace-xml-node"), replace);
    assert_eq!(PptxMutation::decode_op(&replace.encode_op().expect("encode replace-xml-node")).expect("decode replace-xml-node"), replace);
}
//#endregion 🔖️ExactSourceRoundtrip

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
    use protocol::{DiffBinary,DiffCodec,DiffText, OpBinary, OpText};

    /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
    /// parse under the real dialect -- independent of, and cheaper than, the two
    /// `recognize`/`walk_protocol` laws below (a parse failure here fails fast with a clearer
    /// message).
    #[semio_framework_async_macros::async_test]
    async fn committed_facet_files_parse() {
        for (label, text) in [("snapshot grammar", crate::standards::v_ecma_376::subsets::base::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO), ("mutations grammar", crate::standards::v_ecma_376::subsets::base::io::text::mutations::COMPONENT_GRAMMAR_SEMIO), ("diff grammar", crate::standards::v_ecma_376::subsets::base::io::text::diff::COMPONENT_GRAMMAR_SEMIO)] {
            let grammar = semio_framework_dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
            assert_eq!(grammar.dialect, semio_framework_dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
        }
        for (label, text) in [("snapshot protocol", crate::standards::v_ecma_376::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO), ("mutations protocol", crate::standards::v_ecma_376::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO), ("diff protocol", crate::standards::v_ecma_376::subsets::base::io::binary::diff::COMPONENT_PROTOCOL_SEMIO)] {
            semio_framework_dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
        }
    }

    /// 🧾️ The native Text grammar recognizes the complete owned logical presentation.
    #[semio_framework_async_macros::async_test]
    async fn grammar_conformance_law() {
        let grammar = semio_framework_dsl::parse_grammar(crate::standards::v_ecma_376::subsets::base::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO).unwrap();
        let text = store::ArtifactDsl::print_dsl(&demo_pptx_snapshot().await);
        let (envelope, body) = store::semio_format::split_text_preamble(&text).unwrap();
        assert!(semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros())
            .expect("selected grammar fragments")
            .recognize(&format!("{}\n{body}", envelope.envelope_id()))
            .unwrap());
    }

    /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
    /// output for every `PptxMutation` variant (`mutations::demo_mutation_cases()`).
    #[semio_framework_async_macros::async_test]
    async fn ops_grammar_conformance_law() {
        let grammar = semio_framework_dsl::parse_grammar(crate::standards::v_ecma_376::subsets::base::io::text::mutations::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
        let recognizer = semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros())
            .expect("selected grammar fragments");
        for mutation in mutations::demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
        }
    }

    /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output
    /// for every representative `PptxDiff` (`diff::demo_diff_cases()`).
    #[semio_framework_async_macros::async_test]
    async fn diff_grammar_conformance_law() {
        let grammar = semio_framework_dsl::parse_grammar(crate::standards::v_ecma_376::subsets::base::io::text::diff::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
        let recognizer = semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros())
            .expect("selected grammar fragments");
        for d in diff::demo_diff_cases() {
            let printed = d.print_diff();
            assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
        }
    }

    /// 🧭️ The literal native protocol consumes every snapshot, mutation and diff byte.
    #[semio_framework_async_macros::async_test]
    async fn protocol_walk_law() {
        let pack_spec = semio_framework_dsl::parse_protocol(crate::standards::v_ecma_376::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
        let demo = demo_pptx_snapshot().await;
        let packed = store::ArtifactPack::encode_pack(&demo);
        let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
        let trace = semio_framework_dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
        assert_eq!(trace.consumed, inner.len(), "pack walk must consume every literal snapshot byte");

        let op_spec = semio_framework_dsl::parse_protocol(crate::standards::v_ecma_376::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
        for mutation in mutations::demo_mutation_cases() {
            let bytes = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
            let trace = semio_framework_dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
        }

        let diff_spec = semio_framework_dsl::parse_protocol(crate::standards::v_ecma_376::subsets::base::io::binary::diff::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
        for d in diff::demo_diff_cases() {
            let bytes = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
            let trace = semio_framework_dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
        }
    }

    /// ✅️ `fixture_honesty_law`: the shipped `.dsl.semio`/`.pack.semio` fixtures are GENUINE
    /// `print_dsl`/`encode_pack` output of `demo_pptx_snapshot().await` -- `parse_dsl(fixture) ==
    /// demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the pack twin -- so the
    /// fixtures can never silently drift back to a fake `"68656c6c6f"`-style placeholder again
    /// (see this ticket's own recon note on the pre-FG-wave state of these two files).
    #[semio_framework_async_macros::async_test]
    async fn fixture_honesty_law() {
        const FIXTURE_DSL: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");
        const FIXTURE_PACK: &[u8] = include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio");

        let demo = demo_pptx_snapshot().await;

        let parsed = <PptxSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
        assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_pptx_snapshot().await");
        assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_pptx_snapshot().await) drifted from the shipped .dsl.semio fixture");

        let decoded = <PptxSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
        assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_pptx_snapshot().await");
        assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_pptx_snapshot().await) drifted from the shipped .pack.semio fixture");
    }
}
//#endregion 🔖️ConformanceLaws
