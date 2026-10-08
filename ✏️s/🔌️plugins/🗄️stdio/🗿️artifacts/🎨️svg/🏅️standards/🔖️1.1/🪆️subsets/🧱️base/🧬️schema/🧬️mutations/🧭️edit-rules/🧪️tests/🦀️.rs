//! 🧪️ Every details-pane pointer of the SVG editor resolves to the concrete kinds of its gesture, and replaying them reaches the edited document.

use super::*;
use crate::schema::snapshot::{SvgAttr, SvgDocument};
use semio_framework_value::DslValue;

fn element(name: &str, attrs: Vec<SvgAttr>, children: Vec<SvgNode>) -> SvgNode {
    SvgNode::Element { name: name.into(), attrs, children }
}

fn text_attribute(name: &str, value: &str) -> SvgAttr {
    SvgAttr { name: name.into(), value: SvgAttributeValue::Text(value.into()) }
}

fn base() -> SvgSnapshot {
    let group = element("g", vec![text_attribute("id", "a"), text_attribute("class", "b")], vec![SvgNode::Text { text: "hello".into() }, element("rect", vec![], vec![])]);
    SvgSnapshot { doc: SvgDocument { root: Some(element("svg", vec![text_attribute("id", "root")], vec![group, element("circle", vec![], vec![])])), ..Default::default() }, ..Default::default() }
}

fn text(value: &str) -> DslValue {
    DslValue::String(value.to_string())
}

fn resolve(event: SnapshotEditEvent) -> Vec<SvgMutation> {
    let snapshot = base();
    match special(&event, &snapshot).unwrap() {
        Some(mutations) => mutations,
        None => EDIT_RULES.resolve::<SvgSnapshot, SvgMutation>(&snapshot, &event).unwrap(),
    }
}

fn replay(mutations: &[SvgMutation]) -> SvgSnapshot {
    let mut snapshot = base();
    for mutation in mutations {
        assert!(apply_svg_mutation(&mut snapshot, mutation).messages().is_empty());
    }
    snapshot
}

fn root_children(snapshot: &SvgSnapshot) -> &[SvgNode] {
    let Some(SvgNode::Element { children, .. }) = &snapshot.doc.root else { panic!("the fixture has an element root") };
    children
}

#[test]
fn an_attribute_value_edit_raises_set_attribute_by_name_at_the_node_path() {
    let mutations = resolve(SnapshotEditEvent::SetValue { path: "/doc/root/children/0/attrs/1/value".into(), value: DslValue::object([("kind".to_string(), text("text")), ("value".to_string(), text("changed"))]) });
    assert_eq!(mutations, vec![SvgMutation::SetAttribute(SetAttributePayload { path: vec![0], name: "class".into(), value: Some(SvgAttributeValue::Text("changed".into())), index: None })]);
    let SvgNode::Element { attrs, .. } = &root_children(&replay(&mutations))[0] else { panic!("element") };
    assert_eq!(attrs[1], text_attribute("class", "changed"));
}

#[test]
fn a_text_node_and_an_element_name_have_their_own_kinds() {
    let text_mutations = resolve(SnapshotEditEvent::SetValue { path: "/doc/root/children/0/children/0/text".into(), value: text("bye") });
    assert_eq!(text_mutations, vec![SvgMutation::SetText(SetTextPayload { path: vec![0, 0], text: "bye".into() })]);
    let renamed = resolve(SnapshotEditEvent::SetValue { path: "/doc/root/children/1/name".into(), value: text("ellipse") });
    assert_eq!(renamed, vec![SvgMutation::SetElementName(SetElementNamePayload { path: vec![1], name: "ellipse".into() })]);
}

#[test]
fn children_insert_remove_and_move_by_position() {
    let removed = resolve(SnapshotEditEvent::RemoveValue { path: "/doc/root/children/0/children/1".into() });
    assert_eq!(removed, vec![SvgMutation::RemoveElement(RemoveElementPayload { parent: vec![0], index: 1 })]);
    let appended = resolve(SnapshotEditEvent::InsertValue { path: "/doc/root/children/-".into(), value: DslValue::object([("kind".to_string(), text("text")), ("text".to_string(), text("tail"))]) });
    assert_eq!(root_children(&replay(&appended)).last(), Some(&SvgNode::Text { text: "tail".into() }));
    let moved = resolve(SnapshotEditEvent::MoveValue { from: "/doc/root/children/0".into(), path: "/doc/root/children/1".into() });
    assert_eq!(root_children(&replay(&moved))[1], base().doc.root.map(|root| match root { SvgNode::Element { children, .. } => children[0].clone(), _ => unreachable!() }).unwrap());
}

#[test]
fn attributes_insert_remove_and_move_by_position() {
    let inserted = resolve(SnapshotEditEvent::InsertValue { path: "/doc/root/children/0/attrs/0".into(), value: DslValue::object([("name".to_string(), text("lang")), ("value".to_string(), DslValue::object([("kind".to_string(), text("text")), ("value".to_string(), text("en"))]))]) });
    let SvgNode::Element { attrs, .. } = &root_children(&replay(&inserted))[0] else { panic!("element") };
    assert_eq!(attrs.iter().map(|attribute| attribute.name.as_str()).collect::<Vec<_>>(), ["lang", "id", "class"]);
    let removed = resolve(SnapshotEditEvent::RemoveValue { path: "/doc/root/children/0/attrs/0".into() });
    let SvgNode::Element { attrs, .. } = &root_children(&replay(&removed))[0] else { panic!("element") };
    assert_eq!(attrs, &[text_attribute("class", "b")]);
    let moved = resolve(SnapshotEditEvent::MoveValue { from: "/doc/root/children/0/attrs/0".into(), path: "/doc/root/children/0/attrs/1".into() });
    let SvgNode::Element { attrs, .. } = &root_children(&replay(&moved))[0] else { panic!("element") };
    assert_eq!(attrs.iter().map(|attribute| attribute.name.as_str()).collect::<Vec<_>>(), ["class", "id"]);
}

#[test]
fn the_declaration_has_its_own_kind_and_a_prolog_edit_is_refused() {
    assert_eq!(resolve(SnapshotEditEvent::SetValue { path: "/doc/declaration".into(), value: DslValue::Null }), Vec::<SvgMutation>::new());
    let event = SnapshotEditEvent::SetValue { path: "/doc/prolog".into(), value: DslValue::Array(Vec::new()) };
    assert_eq!(EDIT_RULES.resolve::<SvgSnapshot, SvgMutation>(&base(), &event).unwrap_err().code, "snapshot-edit.unsupported-path");
}

#[test]
fn a_drawing_region_replaces_the_root_content_row_by_row() {
    let described = SvgSnapshot { doc: SvgDocument { root: Some(element("svg", vec![text_attribute("class", "new"), text_attribute("id", "root")], vec![element("path", vec![], vec![])])), ..Default::default() }, ..Default::default() };
    let rows = region(&base(), &described).unwrap();
    let edited = replay(&rows);
    assert_eq!(edited.doc.root, described.doc.root);
}
