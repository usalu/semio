use super::*;

fn fixture() -> XlsxSnapshot {
    use crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx;
    use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{XlsxCell, XlsxSheet, XlsxWorkbook};
    build_minimal_xlsx(XlsxWorkbook {
        sheets: vec![XlsxSheet { name: "Sheet 1".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::Boolean(true) }] }],
        ..Default::default()
    })
}

#[semio_framework_async_macros::async_test]
async fn strict_editor_declares_its_dialect_and_main_window() {
    let definition = create_xlsx_strict_editor();
    assert_eq!(definition.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(definition.dialect, XLSX_STRICT_DIALECT.into());
    assert_eq!(<XlsxStrictEditor as ArtifactEditor>::DIALECT, XLSX_STRICT_DIALECT);
    assert!(definition.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn strict_cell_action_preserves_unicode_and_requires_a_complete_address() {
    let fixture = serde_json::json!({ "sheetName": "Tabelle %20", "row": 4, "column": 2, "revision": "0123456789abcdef", "value": "Grüße\n\\s %20" });
    let arguments = dsl::json::from_json_str::<dsl::DslValue>(&fixture.to_string()).unwrap();
    let command = XlsxStrictEditor::command_from_action("set-cell", Some(&arguments)).unwrap();
    let encoded = protocol::OpBinary::encode_op(&command).unwrap();
    assert_eq!(<semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<XlsxStrictEditorCommand> as protocol::OpBinary>::decode_op(&encoded).unwrap(), command);
    let incomplete = dsl::json::from_json_str::<dsl::DslValue>(r#"{"row":4,"column":2,"value":"x"}"#).unwrap();
    assert!(XlsxStrictEditor::command_from_action("set-cell", Some(&incomplete)).is_err());
}

#[semio_framework_async_macros::async_test]
async fn strict_cell_edit_uses_the_canonical_lineage_revision() {
    let snapshot = fixture();
    let address = crate::standards::v_ecma_376::subsets::base::schema::mutations::cell_address::xlsx_cell_address(&snapshot, "Sheet 1", 1, 0).unwrap();
    let command = XlsxStrictEditorCommand::SetCell { sheet_name: "Sheet 1".into(), row: 1, column: 0, revision: address.revision.clone(), value: "false".into() };
    let emit = xlsx_set_cell_emit(&snapshot, &command).unwrap();
    assert_eq!(emit.artifact_mutations, vec![XlsxMutation::SetCell(set_cell::SetCell { address, value: XlsxCellValue::Boolean(false) })]);
    assert!(xlsx_set_cell_emit(&snapshot, &XlsxStrictEditorCommand::SetCell { sheet_name: "Sheet 1".into(), row: 1, column: 0, revision: "stale".into(), value: "false".into() }).is_err());
    assert!(xlsx_set_cell_emit(&snapshot, &XlsxStrictEditorCommand::SetCell { sheet_name: "Sheet 1".into(), row: 99, column: 0, revision: command_revision(&command), value: "false".into() }).is_err());
}

fn command_revision(command: &XlsxStrictEditorCommand) -> String {
    let XlsxStrictEditorCommand::SetCell { revision, .. } = command;
    revision.clone()
}
