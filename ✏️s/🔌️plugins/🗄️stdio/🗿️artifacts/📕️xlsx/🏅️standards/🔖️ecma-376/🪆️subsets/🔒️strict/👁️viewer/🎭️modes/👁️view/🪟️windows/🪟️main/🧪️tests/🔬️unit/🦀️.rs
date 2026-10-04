use super::*;
use semio_framework_plugin::Component;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_table_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

#[semio_framework_async_macros::async_test]
async fn render_delegates_to_the_read_only_worksheet_grid() {
    use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet, XlsxWorkbook};
    let document = crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx(XlsxWorkbook {
        sheets: vec![XlsxSheet { name: "Sheet1".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::Number(1.0) }] }],
        ..Default::default()
    });
    let node = render(&document, semio_framework_ui_locale::Locale::En, &semio_framework_plugin::TreeWindows::unhosted()).expect("render");
    fn table(node: &BuiltNode) -> Option<&BuiltNode> {
        matches!(node.component, Component::Table(_)).then_some(node).or_else(|| node.children.iter().find_map(table))
    }
    let table = table(&node).expect("worksheet grid");
    let Component::Table(props) = &table.component else { unreachable!() };
    assert_eq!(props.columns.iter().map(|label| label.0.as_str()).collect::<Vec<_>>(), ["A", "B"]);
    let Component::TableRow(row) = &table.children[0].component else { panic!("expected grid row") };
    assert_eq!(row.cells.iter().map(|cell| cell.as_str()).collect::<Vec<_>>(), ["1", ""]);
    assert!(table.children[0].children.iter().all(|cell| cell.bindings.is_empty()));
}
