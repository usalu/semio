
use super::*;
use crate::standards::v1_0::subsets::valid::schema::{CODE_DOCTYPE_MISSING, CODE_ROOT_NAME_MISMATCH};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn valid_document() -> XmlSnapshot {
    <XmlSnapshot as store::ArtifactDsl>::parse_dsl(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n<plist version=\"1.0\"><key>Name</key></plist>",
    )
    .expect("the fixture document parses")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn applied(base: &XmlSnapshot, mutation: &XmlValidMutation) -> (XmlSnapshot, protocol::MutationOutcome<XmlDiff>) {
    let mut next = base.clone();
    let outcome = apply_xml_valid_mutation(&mut next, mutation);
    (next, outcome)
}

#[test]
fn kinds_matches_enum_variants_in_declaration_order() {
    let one_per_variant = vec![
        XmlValidMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: XmlSnapshot::default() }),
        XmlValidMutation::DeclareDoctype(declare_doctype::DeclareDoctype { external_id: None }),
        XmlValidMutation::RenameDocumentElement(rename_document_element::RenameDocumentElement { name: "x".into() }),
        XmlValidMutation::SetExternalSubset(set_external_subset::SetExternalSubset { external_id: None }),
        XmlValidMutation::SetStandalone(set_standalone::SetStandalone { standalone: None }),
        XmlValidMutation::DeclareEntity(declare_entity::DeclareEntity { index: 0, parameter: false, name: "e".into(), value: "v".into() }),
        XmlValidMutation::SetInternalSubset(set_internal_subset::SetInternalSubset { declarations: Vec::new() }),
        XmlValidMutation::SetText(set_text::SetText { path: XmlNodePath(vec![0]), text: "t".into() }),
    ];
    assert_eq!(one_per_variant.len(), KINDS.len(), "one_per_variant must cover every KINDS entry exactly once");
    for (mutation, kind) in one_per_variant.iter().zip(KINDS.iter()) {
        assert_eq!(kind_of(mutation), *kind, "KINDS order must match the enum's own declaration order for {mutation:?}");
    }
}

#[test]
fn gate_agrees_with_the_subset_conformance_checker() {
    let no_doctype = <XmlSnapshot as store::ArtifactDsl>::parse_dsl("<plist/>").expect("parses");
    assert!(blocked_snapshot_violation(&no_doctype).expect("a document with no DOCTYPE is hard-invalid").contains(CODE_DOCTYPE_MISSING));
    let mismatched = <XmlSnapshot as store::ArtifactDsl>::parse_dsl("<!DOCTYPE book>\n<plist/>").expect("parses");
    assert!(blocked_snapshot_violation(&mismatched).expect("a desynchronised DOCTYPE Name is hard-invalid").contains(CODE_ROOT_NAME_MISMATCH));
    assert_eq!(blocked_snapshot_violation(&valid_document()), None, "the always-on advisory is a Warning and must never block");
}

#[test]
fn set_snapshot_refuses_a_replacement_that_is_not_valid() {
    let base = valid_document();
    let (next, outcome) = applied(&base, &XmlValidMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: <XmlSnapshot as store::ArtifactDsl>::parse_dsl("<plist/>").expect("parses") }));
    assert!(outcome.messages().iter().any(|message| message.code.0 == CODE_REJECTED), "got {:?}", outcome.messages());
    assert_eq!(next, base, "a rejected mutation must leave the document untouched");
}

#[test]
fn declare_doctype_derives_the_name_from_the_document_element() {
    let base = <XmlSnapshot as store::ArtifactDsl>::parse_dsl("<requests version=\"1.0\"/>").expect("parses");
    let (next, _) = applied(&base, &XmlValidMutation::DeclareDoctype(declare_doctype::DeclareDoctype { external_id: Some(XmlExternalId::System { system_id: "logreq.dtd".into() }) }));
    assert_eq!(next.doc.doctype.as_ref().map(|doctype| doctype.name.as_str()), Some("requests"), "§2.8 must hold by construction, not by the caller's care");
    assert_eq!(blocked_snapshot_violation(&next), None);
}

#[test]
fn rename_document_element_retags_the_doctype_in_the_same_step() {
    let base = valid_document();
    let (next, _) = applied(&base, &XmlValidMutation::RenameDocumentElement(rename_document_element::RenameDocumentElement { name: "propertyList".into() }));
    assert_eq!(document_element_name(&next), Some("propertyList"));
    assert_eq!(next.doc.doctype.as_ref().map(|doctype| doctype.name.as_str()), Some("propertyList"));
    assert_eq!(blocked_snapshot_violation(&next), None, "a rename that passes through an invalid state is exactly what this subset exists to prevent");
}

#[test]
fn declare_entity_refuses_a_duplicate_name() {
    let base = valid_document();
    let (with_entity, _) = applied(&base, &XmlValidMutation::DeclareEntity(declare_entity::DeclareEntity { index: 0, parameter: false, name: "semio".into(), value: "Semio".into() }));
    let (_, duplicate) = applied(&with_entity, &XmlValidMutation::DeclareEntity(declare_entity::DeclareEntity { index: 0, parameter: false, name: "semio".into(), value: "Other".into() }));
    assert!(duplicate.messages().iter().any(|message| message.code.0 == CODE_REJECTED), "§4.2 binds the first declaration, so a second one is dead markup");
    let (_, no_doctype) = applied(&<XmlSnapshot as store::ArtifactDsl>::parse_dsl("<plist/>").expect("parses"), &XmlValidMutation::SetInternalSubset(set_internal_subset::SetInternalSubset { declarations: Vec::new() }));
    assert!(no_doctype.messages().iter().any(|message| message.code.0 == CODE_REJECTED), "there is no internal subset without a DOCTYPE to hold it");
}

#[test]
fn declare_entity_inserts_at_the_declared_index_and_inverts_to_the_prior_list() {
    let base = valid_document();
    let seeded = XmlValidMutation::SetInternalSubset(set_internal_subset::SetInternalSubset {
        declarations: vec![XmlDtdDeclaration::Entity { parameter: false, name: "first".into(), value: "1".into() }, XmlDtdDeclaration::Entity { parameter: false, name: "third".into(), value: "3".into() }],
    });
    let (with_two, _) = applied(&base, &seeded);
    let insertion = XmlValidMutation::DeclareEntity(declare_entity::DeclareEntity { index: 1, parameter: false, name: "second".into(), value: "2".into() });
    let (mut with_three, _) = applied(&with_two, &insertion);
    let names: Vec<&str> = with_three
        .doc
        .doctype
        .as_ref()
        .expect("doctype")
        .declarations
        .iter()
        .map(|declaration| match declaration {
            XmlDtdDeclaration::Entity { name, .. } => name.as_str(),
        })
        .collect();
    assert_eq!(names, vec!["first", "second", "third"], "position is semantic under §4.2");
    for step in inverse_xml_valid_mutation(&insertion, &with_two) {
        let (undone, _) = applied(&with_three, &step);
        with_three = undone;
    }
    assert_eq!(with_three, with_two, "the inverse must restore the prior declaration LIST, order included");
}

#[test]
fn every_kind_round_trips_through_its_own_inverse() {
    let base = valid_document();
    let cases = vec![
        XmlValidMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: <XmlSnapshot as store::ArtifactDsl>::parse_dsl("<!DOCTYPE root>\n<root><child>text</child></root>").expect("parses") }),
        XmlValidMutation::DeclareDoctype(declare_doctype::DeclareDoctype { external_id: Some(XmlExternalId::System { system_id: "plist.dtd".into() }) }),
        XmlValidMutation::RenameDocumentElement(rename_document_element::RenameDocumentElement { name: "propertyList".into() }),
        XmlValidMutation::SetExternalSubset(set_external_subset::SetExternalSubset { external_id: None }),
        XmlValidMutation::SetStandalone(set_standalone::SetStandalone { standalone: Some(true) }),
        XmlValidMutation::DeclareEntity(declare_entity::DeclareEntity { index: 0, parameter: false, name: "semio".into(), value: "Semio".into() }),
        XmlValidMutation::SetInternalSubset(set_internal_subset::SetInternalSubset { declarations: vec![XmlDtdDeclaration::Entity { parameter: true, name: "shared".into(), value: "<!ELEMENT dummy EMPTY>".into() }] }),
        XmlValidMutation::SetText(set_text::SetText { path: XmlNodePath(vec![0, 0]), text: "Renamed".into() }),
    ];
    for mutation in cases {
        let (mut next, outcome) = applied(&base, &mutation);
        assert!(!outcome.messages().iter().any(|message| message.code.0 == CODE_REJECTED), "{mutation:?} must apply against the fixture: {:?}", outcome.messages());
        for step in inverse_xml_valid_mutation(&mutation, &base) {
            let (undone, _) = applied(&next, &step);
            next = undone;
        }
        assert_eq!(next, base, "applying {mutation:?} and then its own inverse must land back on the original document");
    }
}

#[test]
fn set_standalone_is_exact_in_every_declaration_combination() {
    for source in ["<!DOCTYPE root>\n<root/>", "<?xml version=\"1.0\"?>\n<!DOCTYPE root>\n<root/>", "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<!DOCTYPE root>\n<root/>"] {
        let base = <XmlSnapshot as store::ArtifactDsl>::parse_dsl(source).expect("parses");
        for target in [None, Some(true), Some(false)] {
            let mutation = XmlValidMutation::SetStandalone(set_standalone::SetStandalone { standalone: target });
            let (mut next, _) = applied(&base, &mutation);
            for step in inverse_xml_valid_mutation(&mutation, &base) {
                let (undone, _) = applied(&next, &step);
                next = undone;
            }
            assert_eq!(next, base, "set-standalone({target:?}) on {source:?} must invert exactly");
        }
    }
}
