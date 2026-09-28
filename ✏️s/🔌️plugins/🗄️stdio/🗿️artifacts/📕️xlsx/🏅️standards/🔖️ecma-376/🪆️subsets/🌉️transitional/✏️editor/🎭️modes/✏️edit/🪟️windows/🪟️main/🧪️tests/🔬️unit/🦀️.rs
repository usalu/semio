use super::*;
use semio_framework_plugin::{Component, Trigger};

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_table_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

#[semio_framework_async_macros::async_test]
async fn render_projects_localized_cells_with_stable_native_addresses() {
    use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet, XlsxWorkbook};
    let document = crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx(XlsxWorkbook { sheets: vec![XlsxSheet { name: "Sheet1".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::Number(1.0) }] }], ..Default::default() });
    let node = render(&document, Locale::En, &TreeWindows::unhosted()).expect("render");
    let Component::Table(props) = &node.component else { panic!("table") };
    assert_eq!(props.columns.iter().map(|label| label.0.as_str()).collect::<Vec<_>>(), ["Sheet", "Row", "Column", "Value"]);
    assert_eq!(props.window.map(|window| (window.total, window.offset)), Some((1, 0)));
    assert_eq!(props.column_window.map(|window| (window.total, window.offset)), Some((4, 0)));
    assert_eq!(node.children[0].key.as_str(), "workbook-cell-0");
    assert_eq!(node.children[0].children.iter().map(|child| child.key.as_str()).collect::<Vec<_>>(), ["cell-0", "cell-1", "cell-2", "cell-3"]);
    let input = &node.children[0].children[3];
    let Component::Input(input_props) = &input.component else { panic!("value input") };
    assert_eq!(input_props.value.as_str(), "1");
    let binding = input.bindings.iter().find(|binding| binding.trigger == Trigger::Commit).expect("commit binding");
    assert_eq!(binding.action.scope.as_str(), "s.stdio.xlsx@ecma-376/transitional#editor");
    assert_eq!(binding.action.name.as_str(), "set-cell");
    let Some(UiValue::Map(arguments)) = &binding.args else { panic!("arguments") };
    assert!(matches!(arguments.iter().find_map(|(key, value)| (key.as_str() == "sheetName").then_some(value)), Some(UiValue::Text(value)) if value.as_str() == "Sheet1"));
    assert!(matches!(arguments.iter().find_map(|(key, value)| (key.as_str() == "row").then_some(value)), Some(UiValue::Number(1.0))));
    assert!(matches!(arguments.iter().find_map(|(key, value)| (key.as_str() == "column").then_some(value)), Some(UiValue::Number(0.0))));
    assert!(matches!(arguments.iter().find_map(|(key, value)| (key.as_str() == "revision").then_some(value)), Some(UiValue::Text(value)) if value.as_str() == xlsx_cell_revision(&XlsxCellValue::Number(1.0), &[])));
    let german = render(&document, Locale::De, &TreeWindows::unhosted()).expect("German render");
    let Component::Table(props) = &german.component else { panic!("German table") };
    assert_eq!(props.columns.iter().map(|label| label.0.as_str()).collect::<Vec<_>>(), ["Arbeitsblatt", "Zeile", "Spalte", "Wert"]);
}
