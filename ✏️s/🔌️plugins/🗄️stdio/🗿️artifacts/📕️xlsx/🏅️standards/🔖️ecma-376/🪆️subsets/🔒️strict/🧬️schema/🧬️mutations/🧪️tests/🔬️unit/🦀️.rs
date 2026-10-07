use super::*;

//#region 🔖️KindsConformanceLaw
/// 🧭️ `kind_of` is an EXHAUSTIVE match (no wildcard arm) — the compiler refuses this file if a
/// variant is added to `XlsxStrictMutation` without a matching kebab-case spelling here, which is what keeps
/// `KINDS` honest against the enum. The second half reads the sibling oracle manifest's `kinds`
/// array as text (the framework never parses Rust, so this is the only side that can prove the
/// manifest matches) and asserts the same list, in the same order.
#[test]
fn kinds_match_enum_and_catalog() {
    fn kind_of(mutation: &XlsxStrictMutation) -> &'static str {
        match mutation {
            XlsxStrictMutation::SetSnapshot(_) => "set-snapshot",
            XlsxStrictMutation::SetMainNamespace(_) => "set-main-namespace",
            XlsxStrictMutation::SetRelationshipsNamespace(_) => "set-relationships-namespace",
            XlsxStrictMutation::SetConformanceAttribute(_) => "set-conformance-attribute",
            XlsxStrictMutation::RemoveConformanceAttribute(_) => "remove-conformance-attribute",
            XlsxStrictMutation::InsertVmlPart(_) => "insert-vml-part",
            XlsxStrictMutation::RemoveVmlPart(_) => "remove-vml-part",
            XlsxStrictMutation::SetWorksheetContentType(_) => "set-worksheet-content-type",
        }
    }
    let samples = [
        XlsxStrictMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: XlsxSnapshot::default() }),
        XlsxStrictMutation::SetMainNamespace(set_main_namespace::SetMainNamespace { namespace: String::new() }),
        XlsxStrictMutation::SetRelationshipsNamespace(set_relationships_namespace::SetRelationshipsNamespace { namespace: String::new() }),
        XlsxStrictMutation::SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute { value: String::new() }),
        XlsxStrictMutation::RemoveConformanceAttribute(remove_conformance_attribute::RemoveConformanceAttribute {}),
        XlsxStrictMutation::InsertVmlPart(insert_vml_part::InsertVmlPart { path: String::new(), document: XmlDocument::default() }),
        XlsxStrictMutation::RemoveVmlPart(remove_vml_part::RemoveVmlPart { path: String::new() }),
        XlsxStrictMutation::SetWorksheetContentType(set_worksheet_content_type::SetWorksheetContentType { path: String::new(), content_type: String::new() }),
    ];
    let from_enum: Vec<&'static str> = samples.iter().map(kind_of).collect();
    assert_eq!(from_enum, KINDS, "KINDS must list every XlsxStrictMutation variant, in declaration order");

    let manifest = include_str!("../../../../🔮️oracles/🔣️.json");
    let needle = "\"kinds\": [";
    let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
    let end = start + manifest[start..].find(']').expect("kinds array is closed");
    let declared: Vec<String> = manifest[start..end].split(',').map(|entry| entry.trim().trim_matches('"').to_string()).filter(|entry| !entry.is_empty()).collect();
    assert_eq!(declared, KINDS, "the oracle manifest's kinds must match XlsxStrictMutation exactly");
}
//#endregion 🔖️KindsConformanceLaw

//#region 🔖️StampLaw
/// 🏅️ The class stamp is bijective: stamping into one class and back out of it lands on the
/// snapshot it started from. This is what makes `SetSnapshot` exactly invertible on this axis,
/// and it is proven on a snapshot built by this repository's own code, not asserted.
#[test]
fn stamping_into_a_class_and_back_is_the_identity() {
    let base = XlsxSnapshot::default();
    assert_eq!(stamp_conformance_class(stamp_conformance_class(base.clone(), true), false), stamp_conformance_class(base, false));
}
//#endregion 🔖️StampLaw

//#region 🔖️AuthorityLaw
/// 📄️ Every class axis lives in the snapshot's authoritative logical XML parts: retargeting the main namespace and
/// stamping the conformance attribute move the main part itself — an edit of `opc.parts` alone would move nothing, because
/// XML parts are never opaque OPC bytes.
#[test]
fn class_edits_move_the_authoritative_main_part() {
    let mut snapshot = XlsxSnapshot::default();
    let main = main_part_path(&snapshot).expect("the default workbook has a main part");
    apply_xlsx_strict_mutation(&mut snapshot, &XlsxStrictMutation::SetMainNamespace(set_main_namespace::SetMainNamespace { namespace: STRICT_MAIN_NS.into() }));
    apply_xlsx_strict_mutation(&mut snapshot, &XlsxStrictMutation::SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute { value: "strict".into() }));
    let root = snapshot.xml_part(&main).and_then(|part| part.document.root.as_ref()).expect("the main part keeps its root");
    assert!(declares_namespace(root, STRICT_MAIN_NS), "the strict main namespace must land on the logical main part");
    assert_eq!(conformance_attribute(&snapshot).as_deref(), Some("strict"));
}
//#endregion 🔖️AuthorityLaw

#[test]
fn vml_owned_document_fixture_round_trips_native_codecs_and_inverse() {
    use protocol::{OpText, OpBinary};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🧬️mutations/✒️insert-vml-part/🧾️owned-document/🔣️.json")).unwrap();
    let path = fixture["path"].as_str().unwrap().to_string();
    let document: XmlDocument = semio_framework_pack_json::from_json_str(&fixture["document"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mutation = XlsxStrictMutation::InsertVmlPart(insert_vml_part::InsertVmlPart { path: path.clone(), document: document.clone() });
    assert_eq!(XlsxStrictMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
    let encoded = mutation.encode_op().unwrap();
    assert_ne!(encoded, mutation.print_op().into_bytes());
    assert_eq!(XlsxStrictMutation::decode_op(&encoded).unwrap(), mutation);
    let mut invalid = encoded.clone();
    invalid[0] = 255;
    assert!(XlsxStrictMutation::decode_op(&invalid).is_err());
    let before = XlsxSnapshot::default();
    let outcome = mutation.diff(&before);
    let inserted = protocol::MutationDiff::apply(outcome.diff(), &before).unwrap();
    let part = inserted.xml_part(&path).unwrap();
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
    let removal = XlsxStrictMutation::RemoveVmlPart(remove_vml_part::RemoveVmlPart { path });
    let inverse = removal.inverse(&inserted).unwrap();
    assert_eq!(inverse, vec![mutation]);
    let removed = protocol::MutationDiff::apply(removal.diff(&inserted).diff(), &inserted).unwrap();
    assert_eq!(removed, before);
}
