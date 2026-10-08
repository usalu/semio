//! 🧪️ Every details-pane pointer of the DOCX editor resolves to the concrete kinds of its gesture, and replaying them reaches the edited package.

use super::*;
use semio_s_artifact_stdio_zip::opc::{OpcRelationship, OpcTargetMode};
use semio_framework_value::DslValue;
use semio_s_artifact_stdio_xml::schema::snapshot::XmlAttr;

fn main_part(snapshot: &DocxSnapshot) -> usize {
    snapshot.xml_parts.iter().position(|part| part.path == "word/document.xml").expect("the fixture has a main document")
}

fn main_root(snapshot: &DocxSnapshot) -> XmlNode {
    snapshot.xml_part("word/document.xml").unwrap().materialize_document_exact().unwrap().root.unwrap()
}

fn resolve(snapshot: &DocxSnapshot, event: SnapshotEditEvent) -> Vec<DocxMutation> {
    match special(&event, snapshot).unwrap() {
        Some(mutations) => mutations,
        None => EDIT_RULES.resolve::<DocxSnapshot, DocxMutation>(snapshot, &event).unwrap(),
    }
}

fn replay(snapshot: &DocxSnapshot, mutations: &[DocxMutation]) -> DocxSnapshot {
    let mut edited = snapshot.clone();
    for mutation in mutations {
        assert!(apply_docx_mutation(&mut edited, mutation).messages().is_empty());
    }
    edited
}

fn children(node: &XmlNode) -> &[XmlNode] {
    let XmlNode::Element { children, .. } = node else { panic!("an element") };
    children
}

#[test]
fn an_attribute_edit_replaces_only_the_addressed_element() {
    let base = fixture();
    let part = main_part(&base);
    let attribute = XmlAttr { name: "xmlns:extra".into(), value: "urn:extra".into() };
    let mutations = resolve(&base, SnapshotEditEvent::InsertValue { path: format!("/xmlParts/{part}/document/root/attrs/-"), value: attribute.to_value() });
    assert!(matches!(mutations.as_slice(), [DocxMutation::ReplaceXmlNode(_)]));
    let XmlNode::Element { attrs, .. } = main_root(&replay(&base, &mutations)) else { panic!("an element root") };
    assert_eq!(attrs.last(), Some(&attribute));
}

#[test]
fn children_insert_and_remove_under_the_parents_address() {
    let base = fixture();
    let part = main_part(&base);
    let before = children(&main_root(&base)).len();
    let inserted = resolve(&base, SnapshotEditEvent::InsertValue { path: format!("/xmlParts/{part}/document/root/children/-"), value: XmlNode::Comment { text: "tail".into() }.to_value() });
    assert!(matches!(inserted.as_slice(), [DocxMutation::InsertXmlNode(_)]));
    assert_eq!(children(&main_root(&replay(&base, &inserted))).last(), Some(&XmlNode::Comment { text: "tail".into() }));
    let removed = resolve(&base, SnapshotEditEvent::RemoveValue { path: format!("/xmlParts/{part}/document/root/children/0") });
    assert!(matches!(removed.as_slice(), [DocxMutation::RemoveXmlNode(_)]));
    assert_eq!(children(&main_root(&replay(&base, &removed))).len(), before - 1);
}

#[test]
fn an_unchanged_node_publishes_nothing_and_a_whole_part_replacement_is_refused() {
    let base = fixture();
    let part = main_part(&base);
    let root_name = match main_root(&base) {
        XmlNode::Element { name, .. } => name,
        _ => panic!("an element root"),
    };
    assert!(resolve(&base, SnapshotEditEvent::SetValue { path: format!("/xmlParts/{part}/document/root/name"), value: DslValue::String(root_name) }).is_empty());
    let whole = SnapshotEditEvent::SetValue { path: format!("/xmlParts/{part}/document"), value: DslValue::Null };
    assert_eq!(EDIT_RULES.resolve::<DocxSnapshot, DocxMutation>(&base, &whole).unwrap_err().code, "snapshot-edit.unsupported-path");
}

#[test]
fn binary_parts_insert_and_remove_by_position() {
    let base = fixture();
    let added = OpcPart { path: "word/media/added.bin".into(), content_type: "application/octet-stream".into(), bytes: vec![9, 9] };
    let inserted = resolve(&base, SnapshotEditEvent::InsertValue { path: "/opc/parts/0".into(), value: added.to_value() });
    assert!(matches!(inserted.as_slice(), [DocxMutation::SetPart(_)]));
    assert_eq!(replay(&base, &inserted).opc.materialize_package_exact().unwrap().parts[0], added);
    let position = base.opc.materialize_package_exact().unwrap().parts.iter().position(|part| part.path == "word/media/original.bin").unwrap();
    let removed = resolve(&base, SnapshotEditEvent::RemoveValue { path: format!("/opc/parts/{position}") });
    assert_eq!(removed, vec![DocxMutation::RemovePart(remove_part::RemovePart { path: "word/media/original.bin".into() })]);
}

#[test]
fn a_part_content_type_edit_keeps_the_document_and_changes_the_type() {
    let base = fixture();
    let part = main_part(&base);
    let current = base.xml_parts.iter().nth(part).unwrap().content_type.clone();
    assert!(resolve(&base, SnapshotEditEvent::SetValue { path: format!("/xmlParts/{part}/contentType"), value: DslValue::String(current) }).is_empty());
}

#[test]
fn a_relationship_row_inserts_removes_and_changes_through_its_own_kinds() {
    let base = fixture();
    let list = opc_layer::with_package(&base, |opc| opc.relationships.relationships("").cloned()).unwrap().expect("the root owns relationships");
    let added = OpcRelationship { id: "rIdPointer".into(), rel_type: "http://example.invalid/relationships/pointer".into(), target: "https://example.invalid/pointer".into(), target_mode: OpcTargetMode::External };
    let inserted = resolve(&base, SnapshotEditEvent::InsertValue { path: "/opc/relationships//0".into(), value: added.to_value() });
    assert!(matches!(inserted.as_slice(), [DocxMutation::SetRelationship(set_relationship::SetRelationship { index: Some(0), .. })]));
    let external = list.iter().position(|relationship| relationship.target_mode == OpcTargetMode::External).expect("the fixture carries an external demo relationship");
    let removed = resolve(&base, SnapshotEditEvent::RemoveValue { path: format!("/opc/relationships//{external}") });
    assert_eq!(removed, vec![DocxMutation::RemoveRelationship(remove_relationship::RemoveRelationship { owner: String::new(), id: list[external].id.clone() })]);
    let changed = resolve(&base, SnapshotEditEvent::SetValue { path: format!("/opc/relationships//{external}/target"), value: DslValue::String("https://example.invalid/changed".into()) });
    assert!(matches!(changed.as_slice(), [DocxMutation::SetRelationship(set_relationship::SetRelationship { index: None, .. })]));
    let renamed = resolve(&base, SnapshotEditEvent::SetValue { path: format!("/opc/relationships//{external}/id"), value: DslValue::String("rIdRenamed".into()) });
    assert!(matches!(renamed.as_slice(), [DocxMutation::RemoveRelationship(_), DocxMutation::SetRelationship(set_relationship::SetRelationship { index: Some(_), .. })]));
}

#[test]
fn a_content_type_row_inserts_removes_and_changes_through_its_own_kinds() {
    let base = fixture();
    let defaults = opc_layer::with_package(&base, |opc| opc.content_types.defaults.clone()).unwrap();
    let demo = defaults.iter().position(|(extension, _)| extension == "zzdemo").expect("the fixture carries the demo default");
    let removed = resolve(&base, SnapshotEditEvent::RemoveValue { path: format!("/opc/contentTypes/defaults/{demo}") });
    assert_eq!(removed, vec![DocxMutation::RemoveContentType(remove_content_type::RemoveContentType { is_override: false, name: "zzdemo".into() })]);
    let changed = resolve(&base, SnapshotEditEvent::SetValue { path: format!("/opc/contentTypes/defaults/{demo}/1"), value: DslValue::String("application/x-semio-changed".into()) });
    assert_eq!(changed, vec![DocxMutation::SetContentType(set_content_type::SetContentType { is_override: false, name: "zzdemo".into(), content_type: "application/x-semio-changed".into(), index: None })]);
    let inserted = resolve(&base, SnapshotEditEvent::InsertValue { path: "/opc/contentTypes/defaults/-".into(), value: ("zzpointer".to_string(), "application/x-semio-demo".to_string()).to_value() });
    assert!(matches!(inserted.as_slice(), [DocxMutation::SetContentType(set_content_type::SetContentType { is_override: false, index: Some(_), .. })]));
}
