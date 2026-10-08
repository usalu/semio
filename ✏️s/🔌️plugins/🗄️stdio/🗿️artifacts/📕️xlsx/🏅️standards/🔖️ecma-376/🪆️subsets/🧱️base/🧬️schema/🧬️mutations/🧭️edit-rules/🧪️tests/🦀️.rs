//! 🧪️ Every details-pane pointer of the XLSX editor resolves to the concrete kinds of its gesture, and replaying them reaches the edited workbook.

use super::*;
use semio_s_artifact_stdio_zip::opc::{OpcRelationship, OpcTargetMode};
use crate::schema::snapshot::XlsxCellValue;
use semio_s_artifact_stdio_xml::schema::snapshot::XmlAttr;

fn part_row(snapshot: &XlsxSnapshot, wanted: impl Fn(&PartRole) -> bool) -> usize {
    snapshot.xml_parts.iter().position(|part| part_role(snapshot, &part.path).is_ok_and(|role| wanted(&role))).expect("the fixture has the part")
}

fn pointer(part: usize, path: &[usize]) -> String {
    format!("/xmlParts/{part}/document/root{}", path.iter().map(|index| format!("/children/{index}")).collect::<String>())
}

fn resolve(snapshot: &XlsxSnapshot, event: SnapshotEditEvent) -> Vec<XlsxMutation> {
    match special(&event, snapshot).unwrap() {
        Some(mutations) => mutations,
        None => EDIT_RULES.resolve::<XlsxSnapshot, XlsxMutation>(snapshot, &event).unwrap(),
    }
}

fn replay(snapshot: &XlsxSnapshot, mutations: &[XlsxMutation]) -> XlsxSnapshot {
    let mut edited = snapshot.clone();
    for mutation in mutations {
        assert!(apply_xlsx_mutation(&mut edited, mutation).messages().is_empty(), "{mutation:?} was refused");
    }
    edited
}

#[test]
fn editing_the_value_text_of_a_cell_sets_that_cell() {
    let base = fixture();
    let sheet = part_row(&base, |role| matches!(role, PartRole::Worksheet(name) if name == "Sheet1"));
    let address = cell_address::xlsx_cell_address(&base, "Sheet1", 1, 0).unwrap();
    let text = format!("{}/children/0/children/0/text", pointer(sheet, &address.node_path));
    let mutations = resolve(&base, SnapshotEditEvent::SetValue { path: text, value: DslValue::String("2".into()) });
    assert!(matches!(mutations.as_slice(), [XlsxMutation::SetCell(set_cell::SetCell { node: Some(_), value: XlsxCellValue::Number(number), .. })] if *number == 2.0));
    let edited = replay(&base, &mutations).project_workbook().unwrap();
    assert_eq!(edited.sheets[0].cells[0].value, XlsxCellValue::Number(2.0));
}

#[test]
fn removing_a_cell_element_removes_that_cell() {
    let base = fixture();
    let sheet = part_row(&base, |role| matches!(role, PartRole::Worksheet(name) if name == "Sheet1"));
    let address = cell_address::xlsx_cell_address(&base, "Sheet1", 1, 0).unwrap();
    let mutations = resolve(&base, SnapshotEditEvent::RemoveValue { path: pointer(sheet, &address.node_path) });
    assert!(matches!(mutations.as_slice(), [XlsxMutation::RemoveCell(_)]));
    assert!(replay(&base, &mutations).project_workbook().unwrap().sheets[0].cells.is_empty());
}

#[test]
fn editing_a_shared_string_entry_sets_the_string_at_its_position() {
    let base = fixture();
    let strings = part_row(&base, |role| matches!(role, PartRole::SharedStrings));
    let text = format!("{}/children/0/children/0/text", pointer(strings, &[0]));
    let mutations = resolve(&base, SnapshotEditEvent::SetValue { path: text, value: DslValue::String("world".into()) });
    assert!(matches!(mutations.as_slice(), [XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index: 0, node: Some(_), .. })]));
    assert_eq!(replay(&base, &mutations).project_workbook().unwrap().shared_strings, vec!["world".to_string()]);
}

#[test]
fn renaming_a_sheet_element_renames_the_sheet_and_other_attributes_are_refused() {
    let base = fixture();
    let workbook = part_row(&base, |role| matches!(role, PartRole::Workbook));
    let root = base.xml_parts[workbook].document.root.clone().unwrap();
    let XmlNode::Element { children, .. } = &root else { panic!("workbook element") };
    let sheets = children.iter().position(|child| matches!(child, XmlNode::Element { name, .. } if name.ends_with("sheets"))).expect("sheets element");
    let XmlNode::Element { children: entries, .. } = &children[sheets] else { panic!("sheets element") };
    let XmlNode::Element { attrs, .. } = &entries[0] else { panic!("sheet element") };
    let name_attr = attrs.iter().position(|attr| attr.name == "name").unwrap();
    let sheet_id = attrs.iter().position(|attr| attr.name == "sheetId").unwrap();
    let renamed = resolve(&base, SnapshotEditEvent::SetValue { path: format!("{}/attrs/{name_attr}/value", pointer(workbook, &[sheets, 0])), value: DslValue::String("Renamed".into()) });
    assert_eq!(renamed, vec![XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name: "Sheet1".into(), new_name: "Renamed".into() })]);
    let refused = SnapshotEditEvent::SetValue { path: format!("{}/attrs/{sheet_id}/value", pointer(workbook, &[sheets, 0])), value: DslValue::String("99".into()) };
    assert_eq!(special(&refused, &base).unwrap_err().code, "snapshot-edit.unsupported-path");
}

#[test]
fn a_package_pointer_has_no_kind_and_is_refused() {
    let base = fixture();
    let event = SnapshotEditEvent::SetValue { path: "/opc/comment".into(), value: DslValue::String("note".into()) };
    assert_eq!(EDIT_RULES.resolve::<XlsxSnapshot, XlsxMutation>(&base, &event).unwrap_err().code, "snapshot-edit.unsupported-path");
}

#[test]
fn inserting_a_named_sheet_element_inserts_an_empty_sheet_at_that_position() {
    let base = fixture();
    let workbook = part_row(&base, |role| matches!(role, PartRole::Workbook));
    let root = base.xml_parts[workbook].document.root.clone().unwrap();
    let XmlNode::Element { children, .. } = &root else { panic!("workbook element") };
    let sheets = children.iter().position(|child| matches!(child, XmlNode::Element { name, .. } if name.ends_with("sheets"))).expect("sheets element");
    let entry = XmlNode::Element { name: "sheet".into(), attrs: vec![XmlAttr { name: "name".into(), value: "Middle".into() }], children: Vec::new() };
    let mutations = resolve(&base, SnapshotEditEvent::InsertValue { path: format!("{}/children/1", pointer(workbook, &[sheets])), value: entry.to_value() });
    assert!(matches!(mutations.as_slice(), [XlsxMutation::InsertSheet(insert_sheet::InsertSheet { index: Some(1), slot: Some(_), .. })]));
    let names: Vec<String> = replay(&base, &mutations).project_workbook().unwrap().sheets.into_iter().map(|sheet| sheet.name).collect();
    assert_eq!(names, vec!["Sheet1", "Middle", "Sheet2"]);
}

#[test]
fn a_relationship_row_inserts_removes_and_changes_through_its_own_kinds() {
    let base = fixture();
    let list = opc_layer::with_package(&base, |opc| opc.relationships.relationships("").cloned()).unwrap().expect("the root owns relationships");
    let added = OpcRelationship { id: "rIdPointer".into(), rel_type: "http://example.invalid/relationships/pointer".into(), target: "https://example.invalid/pointer".into(), target_mode: OpcTargetMode::External };
    let inserted = resolve(&base, SnapshotEditEvent::InsertValue { path: "/opc/relationships//0".into(), value: added.to_value() });
    assert!(matches!(inserted.as_slice(), [XlsxMutation::SetRelationship(set_relationship::SetRelationship { index: Some(0), .. })]));
    let external = list.iter().position(|relationship| relationship.target_mode == OpcTargetMode::External).expect("the fixture carries an external demo relationship");
    let removed = resolve(&base, SnapshotEditEvent::RemoveValue { path: format!("/opc/relationships//{external}") });
    assert_eq!(removed, vec![XlsxMutation::RemoveRelationship(remove_relationship::RemoveRelationship { owner: String::new(), id: list[external].id.clone() })]);
    let changed = resolve(&base, SnapshotEditEvent::SetValue { path: format!("/opc/relationships//{external}/target"), value: DslValue::String("https://example.invalid/changed".into()) });
    assert!(matches!(changed.as_slice(), [XlsxMutation::SetRelationship(set_relationship::SetRelationship { index: None, .. })]));
    let renamed = resolve(&base, SnapshotEditEvent::SetValue { path: format!("/opc/relationships//{external}/id"), value: DslValue::String("rIdRenamed".into()) });
    assert!(matches!(renamed.as_slice(), [XlsxMutation::RemoveRelationship(_), XlsxMutation::SetRelationship(set_relationship::SetRelationship { index: Some(_), .. })]));
}

#[test]
fn a_content_type_row_inserts_removes_and_changes_through_its_own_kinds() {
    let base = fixture();
    let defaults = opc_layer::with_package(&base, |opc| opc.content_types.defaults.clone()).unwrap();
    let demo = defaults.iter().position(|(extension, _)| extension == "zzdemo").expect("the fixture carries the demo default");
    let removed = resolve(&base, SnapshotEditEvent::RemoveValue { path: format!("/opc/contentTypes/defaults/{demo}") });
    assert_eq!(removed, vec![XlsxMutation::RemoveContentType(remove_content_type::RemoveContentType { is_override: false, name: "zzdemo".into() })]);
    let changed = resolve(&base, SnapshotEditEvent::SetValue { path: format!("/opc/contentTypes/defaults/{demo}/1"), value: DslValue::String("application/x-semio-changed".into()) });
    assert_eq!(changed, vec![XlsxMutation::SetContentType(set_content_type::SetContentType { is_override: false, name: "zzdemo".into(), content_type: "application/x-semio-changed".into(), index: None })]);
    let inserted = resolve(&base, SnapshotEditEvent::InsertValue { path: "/opc/contentTypes/defaults/-".into(), value: ("zzpointer".to_string(), "application/x-semio-demo".to_string()).to_value() });
    assert!(matches!(inserted.as_slice(), [XlsxMutation::SetContentType(set_content_type::SetContentType { is_override: false, index: Some(_), .. })]));
}
