//! 🧪️ Every details-pane pointer of the PPTX editor resolves to the concrete kinds of its gesture, and replaying them reaches the edited deck.

use super::*;
use semio_s_artifact_stdio_zip::opc::{OpcRelationship, OpcTargetMode};
use semio_framework_value::DslValue;
use semio_s_artifact_stdio_xml::schema::snapshot::XmlAttr;

fn slide_part(snapshot: &PptxSnapshot) -> usize {
    snapshot.xml_parts.iter().position(|part| part.path.starts_with("ppt/slides/slide")).expect("the fixture has a slide part")
}

fn resolve(snapshot: &PptxSnapshot, event: SnapshotEditEvent) -> Vec<PptxMutation> {
    match special(&event, snapshot).unwrap() {
        Some(mutations) => mutations,
        None => EDIT_RULES.resolve::<PptxSnapshot, PptxMutation>(snapshot, &event).unwrap(),
    }
}

fn replay(snapshot: &PptxSnapshot, mutations: &[PptxMutation]) -> PptxSnapshot {
    let mut edited = snapshot.clone();
    for mutation in mutations {
        assert!(apply_pptx_mutation(&mut edited, mutation).messages().is_empty());
    }
    edited
}

#[test]
fn an_attribute_insert_replaces_only_the_addressed_element() {
    let base = demo_fixture();
    let part = slide_part(&base);
    let attribute = XmlAttr { name: "xmlns:extra".into(), value: "urn:extra".into() };
    let mutations = resolve(&base, SnapshotEditEvent::InsertValue { path: format!("/xmlParts/{part}/document/root/attrs/-"), value: attribute.to_value() });
    assert!(matches!(mutations.as_slice(), [PptxMutation::ReplaceXmlNode(_)]));
    let XmlNode::Element { attrs, .. } = replay(&base, &mutations).xml_parts[part].document.root.clone().unwrap() else { panic!("an element root") };
    assert_eq!(attrs.last(), Some(&attribute));
}

#[test]
fn a_child_removal_replaces_the_parent_and_an_unchanged_edit_publishes_nothing() {
    let base = demo_fixture();
    let part = slide_part(&base);
    let XmlNode::Element { children, name, .. } = base.xml_parts[part].document.root.clone().unwrap() else { panic!("an element root") };
    let removed = resolve(&base, SnapshotEditEvent::RemoveValue { path: format!("/xmlParts/{part}/document/root/children/0") });
    assert!(matches!(removed.as_slice(), [PptxMutation::ReplaceXmlNode(_)]));
    let XmlNode::Element { children: after, .. } = replay(&base, &removed).xml_parts[part].document.root.clone().unwrap() else { panic!("an element root") };
    assert_eq!(after.len(), children.len() - 1);
    assert!(resolve(&base, SnapshotEditEvent::SetValue { path: format!("/xmlParts/{part}/document/root/name"), value: DslValue::String(name) }).is_empty());
}

#[test]
fn a_package_pointer_has_no_kind_and_is_refused() {
    let base = demo_fixture();
    let event = SnapshotEditEvent::RemoveValue { path: "/opc/parts/0".into() };
    assert_eq!(EDIT_RULES.resolve::<PptxSnapshot, PptxMutation>(&base, &event).unwrap_err().code, "snapshot-edit.unsupported-path");
}

#[test]
fn a_relationship_row_inserts_removes_and_changes_through_its_own_kinds() {
    let base = demo_fixture();
    let list = opc_layer::with_package(&base, |opc| opc.relationships.relationships("").cloned()).unwrap().expect("the root owns relationships");
    let added = OpcRelationship { id: "rIdPointer".into(), rel_type: "http://example.invalid/relationships/pointer".into(), target: "https://example.invalid/pointer".into(), target_mode: OpcTargetMode::External };
    let inserted = resolve(&base, SnapshotEditEvent::InsertValue { path: "/opc/relationships//0".into(), value: added.to_value() });
    assert!(matches!(inserted.as_slice(), [PptxMutation::SetRelationship(set_relationship::SetRelationship { index: Some(0), .. })]));
    let external = list.iter().position(|relationship| relationship.target_mode == OpcTargetMode::External).expect("the fixture carries an external demo relationship");
    let removed = resolve(&base, SnapshotEditEvent::RemoveValue { path: format!("/opc/relationships//{external}") });
    assert_eq!(removed, vec![PptxMutation::RemoveRelationship(remove_relationship::RemoveRelationship { owner: String::new(), id: list[external].id.clone() })]);
    let changed = resolve(&base, SnapshotEditEvent::SetValue { path: format!("/opc/relationships//{external}/target"), value: DslValue::String("https://example.invalid/changed".into()) });
    assert!(matches!(changed.as_slice(), [PptxMutation::SetRelationship(set_relationship::SetRelationship { index: None, .. })]));
    let renamed = resolve(&base, SnapshotEditEvent::SetValue { path: format!("/opc/relationships//{external}/id"), value: DslValue::String("rIdRenamed".into()) });
    assert!(matches!(renamed.as_slice(), [PptxMutation::RemoveRelationship(_), PptxMutation::SetRelationship(set_relationship::SetRelationship { index: Some(_), .. })]));
}

#[test]
fn a_content_type_row_inserts_removes_and_changes_through_its_own_kinds() {
    let base = demo_fixture();
    let defaults = opc_layer::with_package(&base, |opc| opc.content_types.defaults.clone()).unwrap();
    let demo = defaults.iter().position(|(extension, _)| extension == "zzdemo").expect("the fixture carries the demo default");
    let removed = resolve(&base, SnapshotEditEvent::RemoveValue { path: format!("/opc/contentTypes/defaults/{demo}") });
    assert_eq!(removed, vec![PptxMutation::RemoveContentType(remove_content_type::RemoveContentType { is_override: false, name: "zzdemo".into() })]);
    let changed = resolve(&base, SnapshotEditEvent::SetValue { path: format!("/opc/contentTypes/defaults/{demo}/1"), value: DslValue::String("application/x-semio-changed".into()) });
    assert_eq!(changed, vec![PptxMutation::SetContentType(set_content_type::SetContentType { is_override: false, name: "zzdemo".into(), content_type: "application/x-semio-changed".into(), index: None })]);
    let inserted = resolve(&base, SnapshotEditEvent::InsertValue { path: "/opc/contentTypes/defaults/-".into(), value: ("zzpointer".to_string(), "application/x-semio-demo".to_string()).to_value() });
    assert!(matches!(inserted.as_slice(), [PptxMutation::SetContentType(set_content_type::SetContentType { is_override: false, index: Some(_), .. })]));
}
