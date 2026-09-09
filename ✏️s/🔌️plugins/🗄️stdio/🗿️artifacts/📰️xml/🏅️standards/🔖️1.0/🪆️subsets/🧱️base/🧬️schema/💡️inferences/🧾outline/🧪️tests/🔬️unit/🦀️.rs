use super::*;
use crate::schema::snapshot::XmlDocument;

#[semio_framework_async_macros::async_test]
async fn counts_elements_and_depth_over_nested_structure() {
    let root = XmlNode::Element { name: "root".into(), attrs: vec![], children: vec![XmlNode::Element { name: "child".into(), attrs: vec![], children: vec![XmlNode::Text { text: "hi".into() }] }] };
    let snapshot = XmlSnapshot { schema: "stdio.xml".into(), doc: XmlDocument { root: Some(root), doctype: Some("<!DOCTYPE root>".into()), declaration: None, prolog: Vec::new() } };
    let outline = XmlOutline::compute(&snapshot);
    assert_eq!(outline.element_count, 2);
    assert_eq!(outline.max_depth, 2);
    assert!(outline.has_doctype);
}

#[semio_framework_async_macros::async_test]
async fn empty_document_has_zero_elements_and_depth() {
    let outline = XmlOutline::compute(&XmlSnapshot::default());
    assert_eq!(outline.element_count, 0);
    assert_eq!(outline.max_depth, 0);
    assert!(!outline.has_doctype);
}

#[semio_framework_async_macros::async_test]
async fn outline_is_deterministic() {
    let snapshot = XmlSnapshot::default();
    assert_eq!(XmlOutline::compute(&snapshot), XmlOutline::compute(&snapshot));
}
