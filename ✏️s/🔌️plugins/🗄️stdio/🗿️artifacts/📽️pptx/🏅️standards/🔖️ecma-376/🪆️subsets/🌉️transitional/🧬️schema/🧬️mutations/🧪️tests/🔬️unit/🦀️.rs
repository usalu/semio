use super::*;

//#region 🔖️KindsConformanceLaw
/// 🧭️ `kind_of` is an EXHAUSTIVE match (no wildcard arm) — the compiler refuses this file if a
/// variant is added to `PptxTransitionalMutation` without a matching kebab-case spelling here, which is what keeps
/// `KINDS` honest against the enum. The second half reads the sibling oracle manifest's `kinds`
/// array as text (the framework never parses Rust, so this is the only side that can prove the
/// manifest matches) and asserts the same list, in the same order.
#[test]
fn kinds_match_enum_and_catalog() {
    fn kind_of(mutation: &PptxTransitionalMutation) -> &'static str {
        match mutation {
            PptxTransitionalMutation::SetMainNamespace(_) => "set-main-namespace",
            PptxTransitionalMutation::SetDrawingNamespace(_) => "set-drawing-namespace",
            PptxTransitionalMutation::SetRelationshipBase(_) => "set-relationship-base",
            PptxTransitionalMutation::SetConformanceAttribute(_) => "set-conformance-attribute",
            PptxTransitionalMutation::RemoveConformanceAttribute(_) => "remove-conformance-attribute",
        }
    }
    let samples = [
        PptxTransitionalMutation::SetMainNamespace(set_main_namespace::SetMainNamespace { namespace: String::new() }),
        PptxTransitionalMutation::SetDrawingNamespace(set_drawing_namespace::SetDrawingNamespace { namespace: String::new() }),
        PptxTransitionalMutation::SetRelationshipBase(set_relationship_base::SetRelationshipBase { base: String::new() }),
        PptxTransitionalMutation::SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute { value: String::new() }),
        PptxTransitionalMutation::RemoveConformanceAttribute(remove_conformance_attribute::RemoveConformanceAttribute {}),
    ];
    let from_enum: Vec<&'static str> = samples.iter().map(kind_of).collect();
    assert_eq!(from_enum, KINDS, "KINDS must list every PptxTransitionalMutation variant, in declaration order");

    let manifest = include_str!("../../../../🔮️oracles/🔣️.json");
    let needle = "\"kinds\": [";
    let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
    let end = start + manifest[start..].find(']').expect("kinds array is closed");
    let declared: Vec<String> = manifest[start..end].split(',').map(|entry| entry.trim().trim_matches('"').to_string()).filter(|entry| !entry.is_empty()).collect();
    assert_eq!(declared, KINDS, "the oracle manifest's kinds must match PptxTransitionalMutation exactly");
}
//#endregion 🔖️KindsConformanceLaw

//#region 🔖️StampLaw
/// 🏗️ A deck declaring the strict families, so every stamp kind has something to retarget.
fn stampable() -> PptxSnapshot {
    use semio_s_artifact_stdio_zip::opc::{OpcPackage, RELS_CONTENT_TYPE};
    let (main_ns, drawing_ns, rel_ns) = (STRICT_MAIN_NS, STRICT_DRAWING_NS, STRICT_REL);
    let presentation = format!(r#"<p:presentation xmlns:a="{drawing_ns}" xmlns:p="{main_ns}" xmlns:r="{rel_ns}" conformance="strict"><p:sldIdLst/></p:presentation>"#);
    let slide = format!(r#"<p:sld xmlns:a="{drawing_ns}" xmlns:p="{main_ns}"><p:cSld/></p:sld>"#);
    let mut opc = OpcPackage::empty();
    opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
    opc.content_types.set_default("xml", "application/xml");
    opc.add_relationship("", "rId1", &format!("{rel_ns}/officeDocument"), "ppt/presentation.xml");
    let slide_type = "application/vnd.openxmlformats-officedocument.presentationml.slide+xml";
    let mut parts: Vec<(&str, &str, String)> = vec![
        ("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", presentation),
        ("ppt/slides/slide1.xml", slide_type, slide.clone()),
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
    for mutation in stamp_conformance_class_mutations(false) {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
    let mut stamped = base.clone();
    for mutation in stamp_conformance_class_mutations(false) {
        apply_pptx_transitional_mutation(&mut stamped, &mutation);
    }
    assert_ne!(stamped, base);
    for mutation in stamp_conformance_class_mutations(true) {
        apply_pptx_transitional_mutation(&mut stamped, &mutation);
    }
    assert_eq!(stamped, base);
}
//#endregion 🔖️StampLaw
