
use super::*;
use semio_s_artifact_stdio_xml::schema::snapshot::XmlAttr;

#[semio_framework_async_macros::async_test]
async fn element_with_attrs_and_children_maps_to_a_kind_tagged_structure() {
    let doc = XmlDocument {
        root: Some(XmlNode::Element { name: "svg".into(), attrs: vec![XmlAttr { name: "viewBox".into(), value: "0 0 10 10".into() }], children: vec![XmlNode::Text { text: "hi".into() }, XmlNode::Comment { text: "note".into() }] }),
        doctype: None,
        declaration: Some(XmlDeclaration { version: "1.0".into(), encoding: Some("UTF-8".into()), standalone: Some(true) }),
        prolog: vec![XmlNode::Comment { text: "generated".into() }],
    };
    let value = semio_value_from_xml_document(&doc);
    match &value {
        SemioValue::Map { entries } => {
            assert_eq!(entries[0], SemioValueEntry { key: "kind".into(), value: str_value("document") });
            let root = entries.iter().find(|e| e.key == "root").expect("root entry").value.clone();
            match root {
                SemioValue::Map { entries } => {
                    assert!(entries.contains(&SemioValueEntry { key: "tag".into(), value: str_value("svg") }));
                    let children = entries.iter().find(|e| e.key == "children").unwrap().value.clone();
                    match children {
                        SemioValue::List { items } => assert_eq!(items.len(), 2),
                        other => panic!("expected list, got {other:?}"),
                    }
                }
                other => panic!("expected map, got {other:?}"),
            }
        }
        other => panic!("expected map, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn empty_document_still_produces_a_document_shaped_map() {
    let value = semio_value_from_xml_document(&XmlDocument::default());
    match value {
        SemioValue::Map { entries } => {
            assert_eq!(entries.iter().find(|e| e.key == "root").unwrap().value, SemioValue::Null);
            assert_eq!(entries.iter().find(|e| e.key == "doctype").unwrap().value, SemioValue::Null);
        }
        other => panic!("expected map, got {other:?}"),
    }
}
