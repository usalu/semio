use super::*;
use semio_framework_plugin::Component;

fn descendant<'a>(node: &'a BuiltNode, key: &str) -> Option<&'a BuiltNode> {
    (node.key.as_str() == key).then_some(node).or_else(|| node.children.iter().find_map(|child| descendant(child, key)))
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_table_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

#[semio_framework_async_macros::async_test]
async fn render_projects_used_cells_and_a_vacant_edge_as_a_read_only_grid() {
    use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet, XlsxWorkbook};
    let document = crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx(XlsxWorkbook {
        sheets: vec![XlsxSheet { name: "Sheet1".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::Number(1.0) }] }],
        ..Default::default()
    });
    let node = render(&document, Locale::En, &TreeWindows::unhosted()).expect("render");
    let table = descendant(&node, "xlsx-viewer-sheet-0-grid").expect("worksheet grid");
    let Component::Table(props) = &table.component else { panic!("expected windowed table") };
    assert_eq!(props.columns.iter().map(|label| label.0.as_str()).collect::<Vec<_>>(), ["A", "B"]);
    assert_eq!(props.window.map(|window| window.total), Some(2));
    assert_eq!(props.column_window.map(|window| window.total), Some(2));
    let Component::TableRow(row) = &table.children[0].component else { panic!("expected grid row") };
    assert_eq!(row.cells.iter().map(|cell| cell.as_str()).collect::<Vec<_>>(), ["1", ""]);
    assert!(table.children.iter().all(|row| row.bindings.is_empty() && row.children.iter().all(|cell| cell.bindings.is_empty())));
}

#[test]
fn viewer_projects_requested_sparse_grid_windows_against_calamine() {
    use crate::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet, XlsxWorkbook};
    use crate::standards::v_ecma_376::subsets::base::io::export::serializers::{build_minimal_xlsx, encode_xlsx};
    use calamine::Reader;
    use semio_framework_plugin::{TreeWindowRequest, ViewModel, TREE_WINDOW_PATH_SEPARATOR};

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
    let sheet_index = fixture["sheetIndex"].as_u64().unwrap() as usize;
    let sheet_name = fixture["sheets"][sheet_index]["name"].as_str().unwrap();
    let range = reference.worksheet_range(sheet_name).unwrap();
    let start = range.start().unwrap_or_default();
    let mut oracle = std::collections::BTreeMap::new();
    for (row, column, value) in range.used_cells() {
        oracle.insert((start.0 + row as u32 + 1, start.1 + column as u32), value.to_string());
    }
    let row_offset = fixture["rowOffset"].as_u64().unwrap() as usize;
    let row_count = fixture["rows"].as_u64().unwrap() as usize;
    let column_offset = fixture["columnOffset"].as_u64().unwrap() as usize;
    let column_count = fixture["columns"].as_u64().unwrap() as usize;
    let reference_window = (row_offset..row_offset + row_count).map(|row| (column_offset..column_offset + column_count).map(|column| oracle.get(&(row as u32 + 1, column as u32)).cloned().unwrap_or_default()).collect::<Vec<_>>()).collect::<Vec<_>>();
    assert_eq!(serde_json::to_value(&reference_window).unwrap(), fixture["expectedCells"]);

    for (locale, language) in [(Locale::En, "en"), (Locale::De, "de")] {
        let section_id = "xlsx-viewer-sheets";
        let item_id = format!("xlsx-viewer-sheet-{sheet_index}");
        let table_id = format!("xlsx-viewer-sheet-{sheet_index}-grid");
        let item_path = format!("{section_id}{TREE_WINDOW_PATH_SEPARATOR}{item_id}");
        let table_path = format!("{item_path}{TREE_WINDOW_PATH_SEPARATOR}{table_id}");
        let column_path = format!("{item_path}{TREE_WINDOW_PATH_SEPARATOR}{}", semio_framework_plugin::app::table_column_window_key(&table_id));
        let view = ViewModel {
            locale,
            tree_windows: vec![
                TreeWindowRequest { body_key: BODY_KEY.into(), node_key: section_id.into(), open: Some(true), offset: sheet_index as u32, rows: 1 },
                TreeWindowRequest { body_key: BODY_KEY.into(), node_key: item_path, open: Some(true), offset: 0, rows: 1 },
                TreeWindowRequest { body_key: BODY_KEY.into(), node_key: table_path, open: Some(true), offset: row_offset as u32, rows: row_count as u32 },
                TreeWindowRequest { body_key: BODY_KEY.into(), node_key: column_path, open: Some(true), offset: column_offset as u32, rows: column_count as u32 },
            ],
            ..ViewModel::new(locale, semio_framework_ui_locale::Terminology::Native)
        };
        let node = render(&document, locale, &TreeWindows::for_body(&view, BODY_KEY)).unwrap();
        let table = descendant(&node, &table_id).expect("requested worksheet table");
        let Component::Table(props) = &table.component else { panic!("expected windowed table") };
        assert_eq!(props.window.map(|window| (window.total, window.offset)), Some((4, row_offset as u32)));
        assert_eq!(props.column_window.map(|window| (window.total, window.offset)), Some((2, column_offset as u32)));
        assert_eq!(table.children.len(), row_count);
        assert_eq!(props.columns.len(), column_count);
        assert_eq!(props.row_label.as_ref().unwrap().0.as_str(), fixture["labels"][language]["row"].as_str().unwrap());
        assert_eq!(props.column_label.as_ref().unwrap().0.as_str(), fixture["labels"][language]["column"].as_str().unwrap());
        let keys: Vec<String> = serde_json::from_value(fixture["expectedKeys"].clone()).unwrap();
        assert_eq!(table.children.iter().map(|row| row.key.as_str()).collect::<Vec<_>>(), keys.iter().map(String::as_str).collect::<Vec<_>>());
        for (row, expected) in table.children.iter().zip(&reference_window) {
            let Component::TableRow(props) = &row.component else { panic!("expected grid row") };
            assert_eq!(props.cells.iter().map(|cell| cell.as_str()).collect::<Vec<_>>(), expected.iter().map(String::as_str).collect::<Vec<_>>());
            assert!(row.bindings.is_empty() && row.children.iter().all(|cell| cell.bindings.is_empty()));
        }
    }
}
