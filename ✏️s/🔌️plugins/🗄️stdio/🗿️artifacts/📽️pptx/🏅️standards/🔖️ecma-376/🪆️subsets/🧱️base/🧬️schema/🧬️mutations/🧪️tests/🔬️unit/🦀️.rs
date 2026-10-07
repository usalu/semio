use super::*;
use crate::schema::snapshot::PptxXmlPart;
use protocol::{command::DiffAlgebra, Mutation, MutationDiff, OpBinary, OpText};
use quick_xml::{events::Event, reader::Reader};
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::xml_document_from_text;
use semio_s_artifact_stdio_zip::opc::{OpcPackage, OpcRelationship, OpcTargetMode, RELS_CONTENT_TYPE, REL_TYPE_OFFICE_DOCUMENT};
use std::io::{Cursor, Read};

fn fixture_cases() -> Vec<semio_framework_pack_json::Value> {
    let fixture: semio_framework_pack_json::Value =
        semio_framework_pack_json::parse(include_str!("../../../../🧫️fixtures/🧬️canonical-addressed-edit/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("canonical addressed-edit fixture");
    fixture["cases"].as_array().expect("fixture cases").clone()
}

fn snapshot(case: &semio_framework_pack_json::Value) -> PptxSnapshot {
    let presentation_path = case["presentationPath"].as_str().expect("presentation path");
    let strict = case["dialect"].as_str() == Some("strict");
    let relationship_base = if strict { "http://purl.oclc.org/ooxml/officeDocument/relationships" } else { "http://schemas.openxmlformats.org/officeDocument/2006/relationships" };
    let mut opc = OpcPackage::empty();
    opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
    opc.content_types.set_default("xml", "application/xml");
    opc.content_types.set_override(presentation_path, "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml");
    opc.relationships.replace_owner(
        String::new(),
        vec![OpcRelationship {
            id: "office".into(),
            rel_type: if strict { "http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument".into() } else { REL_TYPE_OFFICE_DOCUMENT.into() },
            target: presentation_path.into(),
            target_mode: OpcTargetMode::Internal,
        }],
    );
    let relationships = case["relationships"]
        .as_array()
        .expect("relationships")
        .iter()
        .map(|relationship| OpcRelationship {
            id: relationship["id"].as_str().expect("relationship id").into(),
            rel_type: relationship["type"].as_str().expect("relationship type").into(),
            target: relationship["target"].as_str().expect("relationship target").into(),
            target_mode: OpcTargetMode::Internal,
        })
        .collect();
    opc.relationships.replace_owner(presentation_path.into(), relationships);
    let mut xml_parts = vec![PptxXmlPart {
        path: presentation_path.into(),
        content_type: "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml".into(),
        document: xml_document_from_text(case["presentationXml"].as_str().expect("presentation XML")).expect("valid presentation XML"),
    }];
    for slide in case["slides"].as_array().expect("slides") {
        let path = slide["path"].as_str().expect("slide path");
        let content_type = slide["contentType"].as_str().expect("slide content type");
        opc.content_types.set_override(path, content_type);
        xml_parts.push(PptxXmlPart { path: path.into(), content_type: content_type.into(), document: xml_document_from_text(slide["xml"].as_str().expect("slide XML")).expect("valid slide XML") });
    }
    PptxSnapshot::from_parts(opc, xml_parts)
}

fn text_at(snapshot: &PptxSnapshot, slide_path: &str, shape_id: &str) -> String {
    xml_address::pptx_slides(snapshot)
        .expect("canonical slides")
        .into_iter()
        .find(|slide| slide.address.slide_part_path == slide_path)
        .expect("target slide")
        .shapes
        .into_iter()
        .find(|shape| shape.address.shape_id == shape_id)
        .expect("target shape")
        .text
        .expect("text shape")
}

fn assert_quick_xml(text: &str) {
    let mut reader = Reader::from_str(text);
    loop {
        match reader.read_event().expect("QuickXML independently reopens saved XML") {
            Event::Eof => break,
            _ => {}
        }
    }
}

#[test]
fn canonical_addressed_edits_preserve_rich_unknown_xml_and_exact_inverse() {
    for case in fixture_cases() {
        let base = snapshot(&case);
        let slides = xml_address::pptx_slides(&base).expect("namespace-independent canonical projection");
        let expected = case["slides"].as_array().expect("fixture slides");
        assert_eq!(slides.len(), expected.len());
        for (slide, expected) in slides.iter().zip(expected) {
            assert_eq!(slide.address.slide_part_path, expected["path"].as_str().expect("expected path"));
            assert_eq!(slide.shapes.iter().map(|shape| shape.address.shape_id.as_str()).collect::<Vec<_>>(), expected["shapeIds"].as_array().expect("shape ids").iter().map(|id| id.as_str().expect("shape id")).collect::<Vec<_>>());
        }

        let edit = &case["edits"]["text"];
        let slide_path = edit["slidePath"].as_str().expect("text slide");
        let shape_id = edit["shapeId"].as_str().expect("text shape");
        let address = slides.iter().find(|slide| slide.address.slide_part_path == slide_path).and_then(|slide| slide.shapes.iter().find(|shape| shape.address.shape_id == shape_id)).expect("addressed text shape").address.clone();
        assert_eq!(text_at(&base, slide_path, shape_id), edit["from"].as_str().expect("source text"));
        let mutation = PptxMutation::SetShapeText(set_shape_text::SetShapeText { address, text: edit["to"].as_str().expect("target text").into() });
        let mut edited = base.clone();
        let outcome = apply_pptx_mutation(&mut edited, &mutation);
        assert_eq!(text_at(&edited, slide_path, shape_id), edit["to"].as_str().expect("target text"));
        let serialized = edited.part_text(slide_path).expect("edited slide XML");
        for token in edit["preserve"].as_array().expect("preserved text tokens") {
            assert!(serialized.contains(token.as_str().expect("preserved token")), "{token} was not preserved");
        }
        assert_quick_xml(&serialized);
        let restored = outcome.diff().inverse(&base).apply(&edited).expect("inverse diff");
        assert_eq!(restored, base);
        let mut inverse_restored = edited.clone();
        for inverse in Mutation::inverse(&mutation, &base).expect("valid retained mutation inverse fixture") {
            apply_pptx_mutation(&mut inverse_restored, &inverse);
        }
        assert_eq!(inverse_restored, base);

        let position_edit = &case["edits"]["position"];
        let position_address = slides.iter().find(|slide| slide.address.slide_part_path == slide_path).and_then(|slide| slide.shapes.iter().find(|shape| shape.address.shape_id == shape_id)).expect("addressed position shape").address.clone();
        let parse = |name: &str| position_edit["toPosition"][name].as_str().expect("position").parse().expect("i64 position");
        let position = PptxTransform { x: parse("x"), y: parse("y"), cx: parse("cx"), cy: parse("cy") };
        let mut positioned = base.clone();
        apply_pptx_mutation(&mut positioned, &PptxMutation::SetShapePosition(set_shape_position::SetShapePosition { address: position_address, position }));
        let shape = xml_address::pptx_slides(&positioned).expect("positioned slides").into_iter().flat_map(|slide| slide.shapes).find(|shape| shape.address.shape_id == shape_id).expect("positioned shape");
        assert_eq!(shape.position, Some(position));
        let serialized = positioned.part_text(slide_path).expect("positioned XML");
        for token in position_edit["preserve"].as_array().expect("preserved position tokens") {
            assert!(serialized.contains(token.as_str().expect("preserved token")), "{token} was not preserved");
        }

        let move_edit = &case["edits"]["moveSlide"];
        let moved_address = slides.iter().find(|slide| slide.address.relationship_id == move_edit["relationshipId"].as_str().expect("move relationship")).expect("addressed moved slide").address.clone();
        let mut moved = base.clone();
        apply_pptx_mutation(&mut moved, &PptxMutation::MoveSlide(move_slide::MoveSlide { address: moved_address, destination_index: move_edit["destinationIndex"].as_u64().expect("move index") as usize }));
        assert_eq!(xml_address::pptx_slides(&moved).expect("moved slides")[0].address.relationship_id, move_edit["relationshipId"].as_str().expect("move relationship"));
    }
}

#[test]
fn canonical_save_reopens_with_quick_xml_and_own_decoder() {
    for case in fixture_cases() {
        let base = snapshot(&case);
        let slides = xml_address::pptx_slides(&base).expect("slides");
        let edit = &case["edits"]["text"];
        let address = slides.iter().flat_map(|slide| &slide.shapes).find(|shape| shape.address.shape_id == edit["shapeId"].as_str().expect("shape id")).expect("shape").address.clone();
        let mut edited = base.clone();
        apply_pptx_mutation(&mut edited, &PptxMutation::SetShapeText(set_shape_text::SetShapeText { address, text: edit["to"].as_str().expect("target text").into() }));
        let bytes = crate::standards::v_ecma_376::subsets::base::io::export::serializers::encode_pptx(&edited).expect("canonical save");
        let mut archive = zip::ZipArchive::new(Cursor::new(&bytes)).expect("independent ZIP reopen");
        for part in &edited.xml_parts {
            let mut member = archive.by_name(&part.path).expect("saved XML member");
            let mut text = String::new();
            member.read_to_string(&mut text).expect("saved XML UTF-8");
            assert_quick_xml(&text);
        }
        drop(archive);
        let reopened = crate::standards::v_ecma_376::subsets::base::io::import::deserializers::decode_pptx(&bytes).expect("own canonical reopen");
        assert_eq!(text_at(&reopened, edit["slidePath"].as_str().expect("slide path"), edit["shapeId"].as_str().expect("shape id")), edit["to"].as_str().expect("target text"));
        assert_eq!(reopened.opc.parts, edited.opc.parts);
        assert_eq!(reopened.opc.relationships, edited.opc.relationships);
    }
}

#[test]
fn operation_codecs_and_kind_catalog_cover_canonical_payloads() {
    let cases = demo_mutation_cases();
    assert_eq!(cases.len(), KINDS.len());
    for mutation in cases {
        assert_eq!(PptxMutation::parse_op(&mutation.print_op()).expect("text operation roundtrip"), mutation);
        assert_eq!(PptxMutation::decode_op(&mutation.encode_op().expect("binary operation encode")).expect("binary operation decode"), mutation);
    }
}
