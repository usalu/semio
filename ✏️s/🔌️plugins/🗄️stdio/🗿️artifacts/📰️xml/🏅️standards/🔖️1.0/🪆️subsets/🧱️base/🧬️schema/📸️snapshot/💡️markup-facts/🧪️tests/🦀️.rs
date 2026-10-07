//! 🧫️ Namespace evidence shared with the independent quick-xml reader.
use super::*;
#[test]
fn markup_facts_resolve_aliases_and_ignore_inert_payload_with_quick_xml() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let document: XmlDocument = semio_framework_pack_json::from_json_str(&case["document"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let facts = XmlMarkupFacts::from_document(&document);
        let expected: Vec<String> = case["namespaces"].as_array().unwrap().iter().map(|uri| uri.as_str().unwrap().into()).collect();
        assert_eq!(facts.namespaces, expected);
        assert_eq!(facts.alternate_content, case["alternateContent"].as_bool().unwrap());
        assert_eq!(facts.root_attribute_is("conformance", "strict"), case["strictRoot"].as_bool().unwrap());
        let retained = RetainedXmlDocument::try_from_document(&document).unwrap();
        assert_eq!(XmlMarkupFacts::from_retained_document(&retained), facts);
        let mut reader = quick_xml::reader::NsReader::from_str(case["xml"].as_str().unwrap());
        let mut namespaces = Vec::new();
        let mut alternate = false;
        let decoder = reader.decoder();
        loop {
            let (namespace, event) = reader.read_resolved_event().unwrap();
            match event {
                quick_xml::events::Event::Start(start) | quick_xml::events::Event::Empty(start) => {
                    if start.local_name().as_ref() == b"AlternateContent" && matches!(namespace, quick_xml::name::ResolveResult::Bound(uri) if uri.as_ref() == MC_NS.as_bytes()) { alternate = true; }
                    for attribute in start.attributes() {
                        let attribute = attribute.unwrap();
                        let name = attribute.key.as_ref();
                        if name == b"xmlns" || name.starts_with(b"xmlns:") {
                            let value = attribute.decode_and_unescape_value(decoder).unwrap().to_string();
                            if !namespaces.contains(&value) { namespaces.push(value); }
                        }
                    }
                }
                quick_xml::events::Event::Eof => break,
                _ => {}
            }
        }
        assert_eq!(namespaces, facts.namespaces);
        assert_eq!(alternate, facts.alternate_content);
    }
}
