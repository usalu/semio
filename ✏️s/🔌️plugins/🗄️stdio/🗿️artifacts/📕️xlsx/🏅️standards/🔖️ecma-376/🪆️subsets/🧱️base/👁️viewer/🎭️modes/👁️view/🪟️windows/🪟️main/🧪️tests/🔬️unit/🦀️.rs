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
    let node = render(&document, semio_framework_ui_locale::Locale::En, &semio_framework_plugin::TreeWindows::unhosted()).expect("render");
    let Component::Table(props) = &node.component else { panic!("expected windowed table") };
    assert_eq!(props.columns.iter().map(|label| label.0.as_str()).collect::<Vec<_>>(), ["Sheet", "Row", "Column", "Value"]);
    let Component::TableRow(row) = &node.children[0].component else { panic!("expected cell row") };
    assert_eq!(row.cells.iter().map(|cell| cell.as_str()).collect::<Vec<_>>(), ["Sheet1", "1", "0", "1"]);
    assert!(node.children[0].children.iter().all(|cell| cell.bindings.is_empty()));
}

#[test]
fn viewer_projects_requested_cell_and_column_windows_without_edit_bindings() {
    use crate::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet, XlsxWorkbook};
    use crate::standards::v_ecma_376::subsets::base::io::export::serializers::{build_minimal_xlsx, encode_xlsx};
    use calamine::Reader;
    use semio_framework_ui_locale::Locale;
    use semio_framework_plugin::TreeWindowRequest;
    use semio_framework_plugin::TreeWindows;
    use semio_framework_plugin::ViewModel;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🧫️fixtures/🪟️viewer-cell-window/🔣️.json")).unwrap();
    let sheets = fixture["sheets"]
        .as_array()
        .unwrap()
        .iter()
        .map(|sheet| XlsxSheet {
            name: sheet["name"].as_str().unwrap().into(),
            cells: sheet["values"].as_array().unwrap().iter().enumerate().map(|(row, value)| XlsxCell { row: row as u32 + 1, col: 0, value: XlsxCellValue::InlineString(value.as_str().unwrap().into()) }).collect(),
        })
        .collect();
    let document = build_minimal_xlsx(XlsxWorkbook { sheets, ..Default::default() });
    let bytes = encode_xlsx(&document).unwrap();
    let mut reference: calamine::Xlsx<_> = calamine::open_workbook_from_rs(std::io::Cursor::new(bytes)).unwrap();
    let mut reference_rows = Vec::new();
    for name in reference.sheet_names() {
        let range = reference.worksheet_range(&name).unwrap();
        let (start_row, start_column) = range.start().unwrap_or_default();
        for (row, column, value) in range.used_cells() {
            reference_rows.push(vec![name.clone(), (start_row + row as u32 + 1).to_string(), (start_column + column as u32).to_string(), value.to_string()]);
        }
    }
    let row_offset = fixture["offset"].as_u64().unwrap() as usize;
    let row_count = fixture["rows"].as_u64().unwrap() as usize;
    let column_offset = fixture["columnOffset"].as_u64().unwrap() as usize;
    let column_count = fixture["columns"].as_u64().unwrap() as usize;
    let reference_window: Vec<Vec<String>> = reference_rows[row_offset..row_offset + row_count].iter().map(|row| row[column_offset..column_offset + column_count].to_vec()).collect();
    assert_eq!(serde_json::to_value(&reference_window).unwrap(), fixture["expectedCells"]);
    for (locale, language) in [(Locale::En, "en"), (Locale::De, "de")] {
        let view = ViewModel {
            locale,
            tree_windows: vec![
                TreeWindowRequest { body_key: BODY_KEY.into(), node_key: TableWindowKit::KIND_ID.into(), open: Some(true), offset: fixture["offset"].as_u64().unwrap() as u32, rows: fixture["rows"].as_u64().unwrap() as u32 },
                TreeWindowRequest {
                    body_key: BODY_KEY.into(),
                    node_key: semio_framework_plugin::app::table_column_window_key(TableWindowKit::KIND_ID),
                    open: Some(true),
                    offset: fixture["columnOffset"].as_u64().unwrap() as u32,
                    rows: fixture["columns"].as_u64().unwrap() as u32,
                },
            ],
            ..ViewModel::new(locale, semio_framework_ui_locale::Terminology::Native)
        };
        let node = render(&document, locale, &TreeWindows::for_body(&view, BODY_KEY)).unwrap();
        let Component::Table(props) = &node.component else { panic!("expected windowed table") };
        assert_eq!(props.window.map(|window| (window.total, window.offset)), Some((6, 2)));
        assert_eq!(props.column_window.map(|window| (window.total, window.offset)), Some((4, 1)));
        let labels: Vec<String> = serde_json::from_value(fixture["labels"][language].clone()).unwrap();
        assert_eq!(props.columns.iter().map(|label| label.0.as_str()).collect::<Vec<_>>(), labels[1..].iter().map(String::as_str).collect::<Vec<_>>());
        let keys: Vec<String> = serde_json::from_value(fixture["expectedKeys"].clone()).unwrap();
        assert_eq!(node.children.iter().map(|row| row.key.as_str()).collect::<Vec<_>>(), keys.iter().map(String::as_str).collect::<Vec<_>>());
        for (row, oracle) in node.children.iter().zip(&reference_window) {
            let Component::TableRow(props) = &row.component else { panic!("expected cell row") };
            assert_eq!(props.cells.iter().map(|cell| cell.as_str()).collect::<Vec<_>>(), oracle.iter().map(String::as_str).collect::<Vec<_>>());
            assert!(row.bindings.is_empty() && row.children.iter().all(|cell| cell.bindings.is_empty()));
        }
    }
}
