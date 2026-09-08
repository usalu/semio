mod tests {
    use super::*;

    const PLIST: &[u8] = br#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>UTTypeIdentifier</key>
  <string>tech.semio.document</string>
</dict>
</plist>
"#;

    fn plist() -> MarkupDoc {
        parse_markup(PLIST).expect("the real fixture parses")
    }

    #[test]
    fn the_real_fixture_decomposes_into_apples_public_external_id() {
        let doctype = parse_doctype(plist().doctype.as_deref().expect("the fixture carries a DOCTYPE")).expect("parses");
        assert_eq!(doctype.name, "plist");
        assert_eq!(doctype.external_id, Some(ExternalId::Public { public_id: "-//Apple//DTD PLIST 1.0//EN".into(), system_id: "http://www.apple.com/DTDs/PropertyList-1.0.dtd".into() }));
        assert!(doctype.declarations.is_empty(), "the real fixture declares no internal subset");
    }

    #[test]
    fn the_doctype_grammar_round_trips_every_form_it_accepts() {
        for raw in [
            "plist",
            "plist SYSTEM \"plist.dtd\"",
            "plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\"",
            "requests [<!ENTITY semio \"Semio\"><!ENTITY % shared \"x\">]",
            "requests [<!ELEMENT requests (internal|external)*>]",
        ] {
            let parsed = parse_doctype(raw).unwrap_or_else(|error| panic!("{raw:?} must parse: {error}"));
            assert_eq!(render_doctype(&parsed), raw, "the grammar must be its own inverse for {raw:?}");
        }
    }

    #[test]
    fn an_opaque_content_model_declaration_is_reported_rather_than_dropped() {
        let doctype = parse_doctype("requests [<!ELEMENT requests (internal|external)*>]").expect("parses");
        assert_eq!(doctype.declarations, vec![Declaration::Opaque { raw: "<!ELEMENT requests (internal|external)*>".to_string() }]);
    }

    #[test]
    fn the_verdicts_read_the_two_hard_axes_and_the_standalone_pair() {
        let clean = verdicts(&plist()).expect("verdicts");
        assert_eq!(clean.get("doctypePresent"), Some(&Json::Bool(true)));
        assert_eq!(clean.get("doctypeNameMatchesDocumentElement"), Some(&Json::Bool(true)));
        assert_eq!(clean.get("standaloneBesideExternalSubset"), Some(&Json::Bool(false)));

        let mut standalone = plist();
        apply(&mut standalone, "set-standalone", &obj(vec![("standalone", Json::Bool(true))])).expect("applies");
        assert_eq!(verdicts(&standalone).expect("verdicts").get("standaloneBesideExternalSubset"), Some(&Json::Bool(true)), "§2.9 fires only once BOTH halves are set");

        let mismatched = parse_markup(b"<!DOCTYPE book>\n<plist/>").expect("parses");
        assert_eq!(verdicts(&mismatched).expect("verdicts").get("doctypeNameMatchesDocumentElement"), Some(&Json::Bool(false)));
    }

    #[test]
    fn rename_document_element_retags_the_doctype_in_the_same_step() {
        let mut doc = plist();
        apply(&mut doc, "rename-document-element", &obj(vec![("name", Json::String("propertyList".into()))])).expect("applies");
        assert_eq!(document_element_name(&doc), Some("propertyList"));
        assert_eq!(parse_doctype(doc.doctype.as_deref().expect("doctype")).expect("parses").name, "propertyList");
        assert_eq!(verdicts(&doc).expect("verdicts").get("doctypeNameMatchesDocumentElement"), Some(&Json::Bool(true)), "the rename must never pass through an invalid state");
    }

    #[test]
    fn set_snapshot_refuses_a_replacement_that_is_not_valid() {
        let mut doc = plist();
        assert!(apply(&mut doc, "set-snapshot", &obj(vec![("xml", Json::String("<plist/>".into()))])).is_err(), "a replacement with no DOCTYPE is not XML 1.0 valid");
        assert!(apply(&mut doc, "set-snapshot", &obj(vec![("xml", Json::String("<!DOCTYPE book>\n<plist/>".into()))])).is_err(), "§2.8 requires the DOCTYPE Name to be the document element's name");
    }

    #[test]
    fn declare_entity_places_at_the_index_and_refuses_a_duplicate() {
        let mut doc = plist();
        apply(&mut doc, "declare-entity", &obj(vec![("index", Json::Number(0.0)), ("name", Json::String("first".into())), ("value", Json::String("1".into()))])).expect("applies");
        apply(&mut doc, "declare-entity", &obj(vec![("index", Json::Number(0.0)), ("name", Json::String("zero".into())), ("value", Json::String("0".into()))])).expect("applies");
        let names: Vec<String> = parse_doctype(doc.doctype.as_deref().expect("doctype"))
            .expect("parses")
            .declarations
            .iter()
            .filter_map(|entry| match entry {
                Declaration::Entity { name, .. } => Some(name.clone()),
                Declaration::Opaque { .. } => None,
            })
            .collect();
        assert_eq!(names, vec!["zero".to_string(), "first".to_string()], "§4.2 makes position semantic");
        assert!(apply(&mut doc, "declare-entity", &obj(vec![("index", Json::Number(0.0)), ("name", Json::String("first".into())), ("value", Json::String("x".into()))])).is_err());
    }

    #[test]
    fn a_kind_this_subset_does_not_declare_is_an_error_not_a_no_op() {
        let mut doc = plist();
        assert!(apply(&mut doc, "set-doctype", &Json::Object(Vec::new())).is_err(), "`✳️any`'s own vocabulary must not be silently accepted here");
        assert!(apply(&mut doc, "undeclare-doctype", &Json::Object(Vec::new())).is_err());
    }
}
