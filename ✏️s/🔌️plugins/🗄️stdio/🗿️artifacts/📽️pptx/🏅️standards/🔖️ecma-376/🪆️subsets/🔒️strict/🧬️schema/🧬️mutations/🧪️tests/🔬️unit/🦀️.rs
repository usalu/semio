use super::*;

//#region 🔖️KindsConformanceLaw
/// 🧭️ `kind_of` is an EXHAUSTIVE match (no wildcard arm) — the compiler refuses this file if a
/// variant is added to `PptxStrictMutation` without a matching kebab-case spelling here, which is what keeps
/// `KINDS` honest against the enum. The second half reads the sibling oracle manifest's `kinds`
/// array as text (the framework never parses Rust, so this is the only side that can prove the
/// manifest matches) and asserts the same list, in the same order.
#[test]
fn kinds_match_enum_and_catalog() {
    fn kind_of(mutation: &PptxStrictMutation) -> &'static str {
        match mutation {
            PptxStrictMutation::SetMainNamespace(_) => "set-main-namespace",
            PptxStrictMutation::SetDrawingNamespace(_) => "set-drawing-namespace",
            PptxStrictMutation::SetRelationshipBase(_) => "set-relationship-base",
            PptxStrictMutation::SetConformanceAttribute(_) => "set-conformance-attribute",
            PptxStrictMutation::RemoveConformanceAttribute(_) => "remove-conformance-attribute",
            PptxStrictMutation::InsertVmlPart(_) => "insert-vml-part",
            PptxStrictMutation::RemoveVmlPart(_) => "remove-vml-part",
            PptxStrictMutation::InsertAlternateContent(_) => "insert-alternate-content",
            PptxStrictMutation::RemoveAlternateContent(_) => "remove-alternate-content",
        }
    }
    let samples = [
        PptxStrictMutation::SetMainNamespace(set_main_namespace::SetMainNamespace { namespace: String::new() }),
        PptxStrictMutation::SetDrawingNamespace(set_drawing_namespace::SetDrawingNamespace { namespace: String::new() }),
        PptxStrictMutation::SetRelationshipBase(set_relationship_base::SetRelationshipBase { base: String::new() }),
        PptxStrictMutation::SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute { value: String::new() }),
        PptxStrictMutation::RemoveConformanceAttribute(remove_conformance_attribute::RemoveConformanceAttribute {}),
        PptxStrictMutation::InsertVmlPart(insert_vml_part::InsertVmlPart { path: String::new(), document: XmlDocument::default(), index: None, override_index: None }),
        PptxStrictMutation::RemoveVmlPart(remove_vml_part::RemoveVmlPart { path: String::new() }),
        PptxStrictMutation::InsertAlternateContent(insert_alternate_content::InsertAlternateContent { path: String::new(), node: None, index: None }),
        PptxStrictMutation::RemoveAlternateContent(remove_alternate_content::RemoveAlternateContent { path: String::new(), index: None }),
    ];
    let from_enum: Vec<&'static str> = samples.iter().map(kind_of).collect();
    assert_eq!(from_enum, KINDS, "KINDS must list every PptxStrictMutation variant, in declaration order");

    let manifest = include_str!("../../../../🔮️oracles/🔣️.json");
    let needle = "\"kinds\": [";
    let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
    let end = start + manifest[start..].find(']').expect("kinds array is closed");
    let declared: Vec<String> = manifest[start..end].split(',').map(|entry| entry.trim().trim_matches('"').to_string()).filter(|entry| !entry.is_empty()).collect();
    assert_eq!(declared, KINDS, "the oracle manifest's kinds must match PptxStrictMutation exactly");
}
//#endregion 🔖️KindsConformanceLaw

//#region 🔖️StampLaw
/// 🏗️ A deck declaring the transitional families, so every stamp kind has something to retarget, with the VML part between two slides.
fn stampable() -> PptxSnapshot {
    use semio_s_artifact_stdio_zip::opc::{OpcPackage, RELS_CONTENT_TYPE};
    let (main_ns, drawing_ns, rel_ns) = (TRANSITIONAL_MAIN_NS, TRANSITIONAL_DRAWING_NS, TRANSITIONAL_REL);
    let presentation = format!(r#"<p:presentation xmlns:a="{drawing_ns}" xmlns:p="{main_ns}" xmlns:r="{rel_ns}"><p:sldIdLst/></p:presentation>"#);
    let slide = format!(r#"<p:sld xmlns:a="{drawing_ns}" xmlns:p="{main_ns}"><p:cSld/></p:sld>"#);
    let mut opc = OpcPackage::empty();
    opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
    opc.content_types.set_default("xml", "application/xml");
    opc.add_relationship("", "rId1", &format!("{rel_ns}/officeDocument"), "ppt/presentation.xml");
    let slide_type = "application/vnd.openxmlformats-officedocument.presentationml.slide+xml";
    let mut parts: Vec<(&str, &str, String)> = vec![
        ("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", presentation),
        ("ppt/slides/slide1.xml", slide_type, slide.clone()),
        ("ppt/drawings/vmlDrawing1.vml", VML_CONTENT_TYPE, r#"<xml xmlns:v="urn:schemas-microsoft-com:vml"><v:shape/></xml>"#.to_string()),
        ("ppt/slides/slide2.xml", slide_type, slide),
    ];
    for (path, content_type, _) in &parts {
        opc.content_types.set_override(path, content_type);
    }
    let xml_parts = parts
        .drain(..)
        .map(|(path, content_type, xml)| crate::schema::snapshot::PptxXmlPart {
            path: path.into(),
            content_type: content_type.into(),
            document: semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::xml_document_from_text(&xml).expect("valid XML"),
        })
        .collect();
    PptxSnapshot::from_parts(opc, xml_parts)
}

/// 🏅️ Every stamp kind satisfies the inverse sum law on a deck declaring the opposite class, and stamping into the class and back out restores the deck.
#[semio_framework_async_macros::async_test]
async fn every_stamp_kind_satisfies_the_inverse_sum_law_and_stamping_round_trips() {
    let base = stampable();
    for mutation in stamp_conformance_class_mutations(true) {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
    let mut stamped = base.clone();
    for mutation in stamp_conformance_class_mutations(true) {
        apply_pptx_strict_mutation(&mut stamped, &mutation);
    }
    assert_ne!(stamped, base);
    for mutation in stamp_conformance_class_mutations(false) {
        apply_pptx_strict_mutation(&mut stamped, &mutation);
    }
    assert_eq!(stamped, base);
}

#[semio_framework_async_macros::async_test]
async fn the_vml_and_alternate_content_kinds_satisfy_the_inverse_sum_law_at_a_middle_position() {
    let base = stampable();
    let document: XmlDocument = semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::xml_document_from_text(r#"<xml xmlns:v="urn:schemas-microsoft-com:vml"><v:shape/></xml>"#).expect("valid XML");
    let middle = PptxStrictMutation::RemoveVmlPart(remove_vml_part::RemoveVmlPart { path: "ppt/drawings/vmlDrawing1.vml".into() });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&middle, &base).await;
    let mut without = base.clone();
    apply_pptx_strict_mutation(&mut without, &middle);
    let insert = PptxStrictMutation::InsertVmlPart(insert_vml_part::InsertVmlPart { path: "ppt/drawings/vmlDrawing1.vml".into(), document, index: Some(2), override_index: Some(2) });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&insert, &without).await;
    let add = PptxStrictMutation::InsertAlternateContent(insert_alternate_content::InsertAlternateContent { path: "ppt/slides/slide1.xml".into(), node: None, index: None });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&add, &base).await;
    let mut with_fallback = base.clone();
    apply_pptx_strict_mutation(&mut with_fallback, &add);
    apply_pptx_strict_mutation(&mut with_fallback, &PptxStrictMutation::InsertAlternateContent(insert_alternate_content::InsertAlternateContent { path: "ppt/slides/slide1.xml".into(), node: None, index: Some(0) }));
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&PptxStrictMutation::RemoveAlternateContent(remove_alternate_content::RemoveAlternateContent { path: "ppt/slides/slide1.xml".into(), index: None }), &with_fallback).await;
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&PptxStrictMutation::RemoveAlternateContent(remove_alternate_content::RemoveAlternateContent { path: "ppt/slides/slide1.xml".into(), index: Some(0) }), &with_fallback).await;
}
//#endregion 🔖️StampLaw

#[test]
fn vml_owned_document_fixture_round_trips_native_codecs_and_inverse() {
    use protocol::{OpText, OpBinary};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🧬️mutations/🖼️insert-vml-part/🧾️owned-document/🔣️.json")).unwrap();
    let path = fixture["path"].as_str().unwrap().to_string();
    let document: XmlDocument = semio_framework_pack_json::from_json_str(&fixture["document"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mutation = PptxStrictMutation::InsertVmlPart(insert_vml_part::InsertVmlPart { path: path.clone(), document: document.clone(), index: None, override_index: None });
    assert_eq!(PptxStrictMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
    let encoded = mutation.encode_op().unwrap();
    assert_ne!(encoded, mutation.print_op().into_bytes());
    assert_eq!(PptxStrictMutation::decode_op(&encoded).unwrap(), mutation);
    let mut invalid = encoded.clone();
    invalid[0] = 255;
    assert!(PptxStrictMutation::decode_op(&invalid).is_err());
    let before = PptxSnapshot::default();
    let outcome = mutation.diff(&before);
    let inserted = protocol::apply_diff(outcome.diff(), &before).unwrap();
    let part = inserted.xml_parts.iter().find(|part| part.path == path).unwrap();
    assert_eq!(part.document, document);
    let physical = semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::xml_document_to_text(&document);
    assert_eq!(physical, fixture["xml"].as_str().unwrap());
    let mut reference = quick_xml::Reader::from_str(&physical);
    let mut element_names = Vec::new();
    loop {
        match reference.read_event().unwrap() {
            quick_xml::events::Event::Start(start) | quick_xml::events::Event::Empty(start) => element_names.push(start.name().as_ref().to_string()),
            quick_xml::events::Event::Eof => break,
            _ => {}
        }
    }
    assert_eq!(element_names, ["xml", "v:shape"]);
    let carrier: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&document)).unwrap();
    assert_eq!(carrier, fixture["document"]);
    let removal = PptxStrictMutation::RemoveVmlPart(remove_vml_part::RemoveVmlPart { path: path.clone() });
    let inverse = removal.inverse(&inserted).unwrap();
    let (index, override_index) = crate::standards::v_ecma_376::subsets::base::schema::mutations::xml_part_positions(&inserted, &path).unwrap();
    assert_eq!(inverse, vec![PptxStrictMutation::InsertVmlPart(insert_vml_part::InsertVmlPart { path: path.clone(), document: document.clone(), index: Some(index), override_index })]);
    let removed = protocol::apply_diff(removal.diff(&inserted).diff(), &inserted).unwrap();
    assert_eq!(removed, before);
}
