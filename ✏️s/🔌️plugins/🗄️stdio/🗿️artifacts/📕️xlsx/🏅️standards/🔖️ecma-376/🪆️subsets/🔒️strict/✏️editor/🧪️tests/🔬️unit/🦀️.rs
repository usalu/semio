use super::*;

#[semio_framework_async_macros::async_test]
async fn create_xlsx_strict_editor_builds_a_definition_for_the_editor_role() {
    let def = create_xlsx_strict_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, XLSX_STRICT_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<XlsxStrictEditor as ArtifactEditor>::DIALECT, XLSX_STRICT_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_main_window() {
    let def = create_xlsx_strict_editor();
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
async fn render_value_resolves_shared_strings_and_shows_formula_cache() {
    let strings = vec!["hello".to_string()];
    assert_eq!(render_xlsx_cell_value(&XlsxCellValue::SharedString(0), &strings), "hello");
    assert_eq!(render_xlsx_cell_value(&XlsxCellValue::SharedString(9), &strings), "#9");
    assert_eq!(render_xlsx_cell_value(&XlsxCellValue::Formula { expr: "SUM(A1:A2)".into(), cached: Some(Box::new(XlsxCellValue::Number(3.0))) }, &strings), "=SUM(A1:A2) (3)");
    assert_eq!(render_xlsx_cell_value(&XlsxCellValue::Empty, &strings), "");
}

#[semio_framework_async_macros::async_test]
async fn parse_cell_value_detects_bool_and_number_before_falling_back_to_inline_string() {
    assert_eq!(parse_xlsx_cell_value("true"), XlsxCellValue::Boolean(true));
    assert_eq!(parse_xlsx_cell_value("false"), XlsxCellValue::Boolean(false));
    assert_eq!(parse_xlsx_cell_value("3.5"), XlsxCellValue::Number(3.5));
    assert_eq!(parse_xlsx_cell_value("hello"), XlsxCellValue::InlineString("hello".into()));
}
