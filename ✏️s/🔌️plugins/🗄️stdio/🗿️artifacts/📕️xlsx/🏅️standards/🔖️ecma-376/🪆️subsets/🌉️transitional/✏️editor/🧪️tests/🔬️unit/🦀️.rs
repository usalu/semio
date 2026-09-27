use super::*;

#[semio_framework_async_macros::async_test]
async fn create_xlsx_transitional_editor_builds_a_definition_for_the_editor_role() {
    let def = create_xlsx_transitional_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, XLSX_TRANSITIONAL_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<XlsxTransitionalEditor as ArtifactEditor>::DIALECT, XLSX_TRANSITIONAL_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_main_window() {
    let def = create_xlsx_transitional_editor();
    assert!(def.window_kinds.iter().any(|w| w.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn flat_cells_orders_by_sheet_then_cell_storage_order() {
    use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{XlsxCell, XlsxSheet, XlsxWorkbook};
    let document = XlsxSnapshot {
        workbook: XlsxWorkbook {
            sheets: vec![XlsxSheet { name: "S1".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::Number(1.0) }] }, XlsxSheet { name: "S2".into(), cells: vec![XlsxCell { row: 2, col: 1, value: XlsxCellValue::Boolean(true) }] }],
            ..Default::default()
        },
        ..XlsxSnapshot::default()
    };
    let rows = xlsx_flat_cells(&document);
    assert_eq!(rows, vec![("S1".to_string(), 1, 0, XlsxCellValue::Number(1.0)), ("S2".to_string(), 2, 1, XlsxCellValue::Boolean(true))]);
}

#[semio_framework_async_macros::async_test]
async fn render_value_resolves_shared_strings_and_exposes_editable_formula_source() {
    let strings = vec!["hello".to_string()];
    assert_eq!(render_xlsx_cell_value(&XlsxCellValue::SharedString(0), &strings), "hello");
    assert_eq!(render_xlsx_cell_value(&XlsxCellValue::SharedString(9), &strings), "#9");
    assert_eq!(render_xlsx_cell_value(&XlsxCellValue::Formula { expr: "SUM(A1:A2)".into(), cached: Some(Box::new(XlsxCellValue::Number(3.0))) }, &strings), "=SUM(A1:A2)");
    assert_eq!(render_xlsx_cell_value(&XlsxCellValue::Empty, &strings), "");
}

#[semio_framework_async_macros::async_test]
async fn parse_cell_value_detects_bool_and_number_before_falling_back_to_inline_string() {
    assert_eq!(parse_xlsx_cell_value(""), XlsxCellValue::Empty);
    assert_eq!(parse_xlsx_cell_value("=SUM(A1:A2)"), XlsxCellValue::Formula { expr: "SUM(A1:A2)".into(), cached: None });
    assert_eq!(parse_xlsx_cell_value("'=literal"), XlsxCellValue::InlineString("=literal".into()));
    assert_eq!(parse_xlsx_cell_value("true"), XlsxCellValue::Boolean(true));
    assert_eq!(parse_xlsx_cell_value("false"), XlsxCellValue::Boolean(false));
    assert_eq!(parse_xlsx_cell_value("3.5"), XlsxCellValue::Number(3.5));
    assert_eq!(parse_xlsx_cell_value("NaN"), XlsxCellValue::InlineString("NaN".into()));
    assert_eq!(parse_xlsx_cell_value("hello"), XlsxCellValue::InlineString("hello".into()));
}

#[semio_framework_async_macros::async_test]
async fn command_parser_requires_the_stable_cell_address_and_codec_preserves_unicode() {
    let fixture = serde_json::json!({ "sheetName": "Tabelle %20", "row": 4, "column": 2, "revision": "0123456789abcdef", "value": "Grüße\n\\s %20" });
    let args = dsl::json::from_json_str::<dsl::DslValue>(&fixture.to_string()).expect("parse arguments");
    let command = XlsxTransitionalEditor::command_from_action("set-cell", Some(&args)).expect("stable cell action parses");
    let encoded = protocol::OpBinary::encode_op(&command).expect("encode command");
    let decoded = <semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<XlsxTransitionalEditorCommand> as protocol::OpBinary>::decode_op(&encoded).expect("decode command");
    assert_eq!(decoded, command);
    let incomplete = dsl::json::from_json_str::<dsl::DslValue>(r#"{"sheetName":"Sheet 1","row":4,"value":"x"}"#).unwrap();
    assert!(XlsxTransitionalEditor::command_from_action("set-cell", Some(&incomplete)).is_err());
}

#[semio_framework_async_macros::async_test]
async fn stable_cell_edit_rejects_address_and_revision_drift() {
    use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{XlsxCell, XlsxSheet, XlsxWorkbook};
    let snapshot = XlsxSnapshot {
        workbook: XlsxWorkbook { sheets: vec![XlsxSheet { name: "Sheet 1".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::Boolean(true) }] }], ..Default::default() },
        ..XlsxSnapshot::default()
    };
    let command = XlsxTransitionalEditorCommand::SetCell { sheet_name: "Sheet 1".into(), row: 1, column: 0, revision: xlsx_cell_revision(&XlsxCellValue::Boolean(true)), value: "false".into() };
    assert!(xlsx_set_cell_emit(&snapshot, &command).is_ok());
    assert!(xlsx_set_cell_emit(&snapshot, &XlsxTransitionalEditorCommand::SetCell { sheet_name: "Sheet 1".into(), row: 1, column: 0, revision: "stale".into(), value: "false".into() }).is_err());
    assert!(xlsx_set_cell_emit(&snapshot, &XlsxTransitionalEditorCommand::SetCell { sheet_name: "Sheet 1".into(), row: 99, column: 0, revision: command_revision(&command), value: "false".into() }).is_err());
}

fn command_revision(command: &XlsxTransitionalEditorCommand) -> String {
    let XlsxTransitionalEditorCommand::SetCell { revision, .. } = command;
    revision.clone()
}
