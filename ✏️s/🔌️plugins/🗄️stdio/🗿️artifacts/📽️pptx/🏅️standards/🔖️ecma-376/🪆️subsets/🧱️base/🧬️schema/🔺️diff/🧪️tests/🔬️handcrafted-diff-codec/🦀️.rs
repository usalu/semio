
use super::*;
use protocol::DiffCodec;
use semio_s_artifact_stdio_zip::opc::{OpcPackage, REL_TYPE_OFFICE_DOCUMENT, RELS_CONTENT_TYPE};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn elem_snapshot(slides: Vec<PptxSlide>) -> PptxSnapshot {
    let mut opc = OpcPackage::empty();
    opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
    opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", b"<p:presentation/>".to_vec());
    opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, "ppt/presentation.xml");
    PptxSnapshot::from_parts(opc, Vec::new(), PptxPresentation { slides })
}

/// 🧪️ `DiffCodec` round-trip laws over the hand-rolled `PptxDiff` grammar — exercises the
/// `PptxShapeDiff` enum tree (`TextBox`/`Picture`/`Placeholder`/`Replace`), the `font_size`
/// tri-state (`Some(None)` AND `Some(Some(_))`), nested `IndexedTripleDiff` collections
/// (slides/shapes/paragraphs/runs) AND `NamedTripleDiff` collections (content types/parts/
/// relationships, incl. the doubly-nested relationships-by-owner triple).
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let a = elem_snapshot(vec![PptxSlide {
        shapes: vec![
            PptxShape::TextBox { text_frame: vec![PptxParagraph { runs: vec![PptxRun { text: "old".into(), bold: false, italic: false, font_size: Some(10) }] }], position: PptxTransform { x: 1, y: 1, cx: 1, cy: 1 } },
            PptxShape::Picture { blip_rel_id: "rIdOld".into(), position: PptxTransform::default() },
        ],
    }]);
    let mut b_opc_snapshot = elem_snapshot(vec![PptxSlide {
        shapes: vec![
            PptxShape::TextBox { text_frame: vec![PptxParagraph { runs: vec![PptxRun { text: "new".into(), bold: true, italic: true, font_size: None }] }], position: PptxTransform { x: 9, y: 9, cx: 9, cy: 9 } },
            PptxShape::Placeholder { kind: "body".into(), text_frame: vec![PptxParagraph::text("ph")], position: PptxTransform::default() },
        ],
    }]);
    b_opc_snapshot.opc.set_part("ppt/added.xml", "application/xml", b"fresh".to_vec());
    b_opc_snapshot.opc.content_types.set_override("ppt/added.xml", "application/xml");
    b_opc_snapshot.opc.add_relationship("ppt/added.xml", "rId9", "http://example/added", "media/added.png");
    let b = b_opc_snapshot;

    let c = elem_snapshot(vec![]);

    let cases = vec![PptxDiff::default(), PptxDiff::between(&a, &b), PptxDiff::between(&b, &a), PptxDiff::between(&a, &c), PptxDiff::between(&c, &a)];
    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = PptxDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = PptxDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}

#[semio_framework_async_macros::async_test]
async fn logical_xml_parts_diff_apply_inverse_absorb_between_and_codecs() {
    let base = elem_snapshot(Vec::new());
    let mut sourced = base.clone();
    sourced.xml_parts = vec![PptxXmlPart { path: "docProps/core.xml".into(), content_type: "application/vnd.openxmlformats-package.core-properties+xml".into(), document: semio_s_artifact_stdio_xml::schema::snapshot::XmlDocument::default() }];
    let diff = PptxDiff::between(&base, &sourced);
    assert_eq!(diff.xml_parts, Some(sourced.xml_parts.clone()));
    assert_eq!(diff.apply(&base).unwrap(), sourced);

    let printed = diff.print_diff();
    assert_eq!(PptxDiff::parse_diff(&printed).expect("parse logical XML parts diff"), diff);
    let encoded = diff.encode_diff().expect("encode logical XML parts diff");
    assert_eq!(PptxDiff::decode_diff(&encoded).expect("decode logical XML parts diff"), diff);

    let inverse = diff.inverse(&base);
    assert_eq!(inverse.apply(&sourced).unwrap(), base);
    let mut absorbed = diff.clone();
    absorbed.absorb(inverse);
    assert_eq!(absorbed.apply(&base).unwrap(), base);
    assert!(PptxDiff::between(&sourced, &sourced).is_empty());
}
