use super::*;
use semio_framework_plugin::Component;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_table_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

#[semio_framework_async_macros::async_test]
async fn render_lists_one_row_per_cell() {
    use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet, XlsxWorkbook};
    let document = crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx(XlsxWorkbook { sheets: vec![XlsxSheet { name: "Sheet1".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::Number(1.0) }] }], ..Default::default() });
    let node = render(&document, semio_framework_plugin::Locale::En, &semio_framework_plugin::TreeWindows::unhosted()).expect("render");
    let Component::Table(props) = &node.component else { panic!("expected windowed table") };
    assert_eq!(props.columns.iter().map(|label| label.0.as_str()).collect::<Vec<_>>(), ["Sheet", "Row", "Column", "Value"]);
    let Component::TableRow(row) = &node.children[0].component else { panic!("expected cell row") };
    assert_eq!(row.cells.iter().map(|cell| cell.as_str()).collect::<Vec<_>>(), ["Sheet1", "1", "0", "1"]);
    assert!(node.children[0].children.iter().all(|cell| cell.bindings.is_empty()));
}
