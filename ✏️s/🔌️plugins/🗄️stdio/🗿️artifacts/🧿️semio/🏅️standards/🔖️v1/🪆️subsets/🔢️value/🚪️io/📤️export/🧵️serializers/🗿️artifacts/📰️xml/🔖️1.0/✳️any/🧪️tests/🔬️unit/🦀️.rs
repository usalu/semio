
use super::*;
use crate::standards::v1::subsets::value::io::import::deserializers::artifacts::xml::v1_0::any::semio_value_from_xml_document;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn round_trip(doc: XmlDocument) -> XmlDocument {
    let value = semio_value_from_xml_document(&doc);
    let nodes = HashMap::new();
    let mut visiting = HashSet::new();
    xml_document_from_semio(&value, &nodes, &mut visiting).expect("value->xml")
}

/// 🧪️ Required proof: xml -> value -> xml -> value round trip preserves everything the
/// value subset can represent (the whole document, structurally — nothing is lossy in THIS
/// direction since the fixture already conforms to the tagged-map convention).
#[semio_framework_async_macros::async_test]
async fn xml_to_value_to_xml_round_trips_structurally() {
    let doc = XmlDocument {
        root: Some(XmlNode::Element {
            name: "svg".into(),
            attrs: vec![XmlAttr { name: "viewBox".into(), value: "0 0 10 10".into() }, XmlAttr { name: "xmlns".into(), value: "http://www.w3.org/2000/svg".into() }],
            children: vec![
                XmlNode::Element { name: "rect".into(), attrs: vec![XmlAttr { name: "width".into(), value: "5".into() }], children: vec![] },
                XmlNode::Comment { text: "a note".into() },
                XmlNode::CData { text: "raw <stuff>".into() },
                XmlNode::Text { text: "hello & goodbye".into() },
            ],
        }),
        doctype: Some("<!DOCTYPE svg>".into()),
        declaration: Some(XmlDeclaration { version: "1.0".into(), encoding: Some("UTF-8".into()), standalone: Some(false) }),
        prolog: vec![XmlNode::Comment { text: "generated".into() }],
    };
    assert_eq!(round_trip(doc.clone()), doc);
}

#[semio_framework_async_macros::async_test]
async fn empty_document_round_trips() {
    assert_eq!(round_trip(XmlDocument::default()), XmlDocument::default());
}

#[semio_framework_async_macros::async_test]
async fn non_conforming_shape_is_a_hard_error_not_a_silent_default() {
    let nodes = HashMap::new();
    let mut visiting = HashSet::new();
    let bogus = SemioValue::Int { lexeme: "1".into() };
    assert!(xml_document_from_semio(&bogus, &nodes, &mut visiting).is_err());
}

#[semio_framework_async_macros::async_test]
async fn ref_is_dereferenced_and_cycles_error() {
    let id = ValueId::new("self");
    let cyclic = SemioValue::Ref { id: id.clone() };
    let mut nodes_owned: HashMap<&ValueId, &SemioValue> = HashMap::new();
    nodes_owned.insert(&id, &cyclic);
    let mut visiting = HashSet::new();
    assert!(xml_document_from_semio(&cyclic, &nodes_owned, &mut visiting).is_err());
}
