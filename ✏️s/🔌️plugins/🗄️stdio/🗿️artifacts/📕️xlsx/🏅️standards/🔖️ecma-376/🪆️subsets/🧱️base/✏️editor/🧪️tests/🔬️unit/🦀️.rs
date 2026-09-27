use super::*;

#[semio_framework_async_macros::async_test]
async fn create_xlsx_editor_builds_a_definition_for_the_editor_role() {
    let def = create_xlsx_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, XLSX_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<XlsxEditor as ArtifactEditor>::DIALECT, XLSX_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_main_window() {
    let def = create_xlsx_editor();
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
async fn stable_cell_action_fixture_roundtrips_exact_text_through_parser_and_binary_codec() {
    const FIXTURE: &str = include_str!("../../🧫️fixtures/📊️stable-cell-edit/🔣️.json");
    let oracle: serde_json::Value = serde_json::from_str(FIXTURE).expect("third-party JSON oracle parses the action fixture");
    let args = dsl::json::from_json_str::<dsl::DslValue>(FIXTURE).expect("DSL JSON parses the action fixture");
    let command = XlsxEditor::command_from_action("set-cell", Some(&args)).expect("stable cell action parses");
    let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(native) = &command else { panic!("expected native command") };
    assert_eq!(
        native,
        &XlsxEditorCommand::SetCell {
            sheet_name: oracle["sheetName"].as_str().unwrap().to_string(),
            row: oracle["row"].as_u64().unwrap() as u32,
            column: oracle["column"].as_u64().unwrap() as u32,
            revision: oracle["revision"].as_str().unwrap().to_string(),
            value: oracle["value"].as_str().unwrap().to_string(),
        }
    );
    let encoded = protocol::OpBinary::encode_op(&command).expect("encode command");
    let decoded = <semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<XlsxEditorCommand> as protocol::OpBinary>::decode_op(&encoded).expect("decode command");
    assert_eq!(decoded, command);
}

#[semio_framework_async_macros::async_test]
async fn stable_cell_edit_targets_identity_and_rejects_a_stale_revision() {
    use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{XlsxCell, XlsxSheet, XlsxWorkbook};
    let snapshot = XlsxSnapshot {
        workbook: XlsxWorkbook { sheets: vec![XlsxSheet { name: "Sheet 1".into(), cells: vec![XlsxCell { row: 41, col: 7, value: XlsxCellValue::Number(1.0) }] }], ..Default::default() },
        ..XlsxSnapshot::default()
    };
    let command = XlsxEditorCommand::SetCell { sheet_name: "Sheet 1".into(), row: 41, column: 7, revision: xlsx_cell_revision(&XlsxCellValue::Number(1.0)), value: "2".into() };
    let emit = xlsx_set_cell_emit(&snapshot, &command).expect("matching revision emits a mutation");
    assert_eq!(
        emit.artifact_mutations,
        vec![XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet 1".into(), row: 41, col: 7, value: XlsxCellValue::Number(2.0) })]
    );
    let stale = XlsxEditorCommand::SetCell { sheet_name: "Sheet 1".into(), row: 41, column: 7, revision: "stale".into(), value: "2".into() };
    assert!(xlsx_set_cell_emit(&snapshot, &stale).is_err());
}
