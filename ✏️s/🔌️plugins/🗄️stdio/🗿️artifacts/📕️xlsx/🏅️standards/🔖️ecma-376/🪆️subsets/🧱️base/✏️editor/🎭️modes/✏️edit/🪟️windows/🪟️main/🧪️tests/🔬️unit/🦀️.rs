use super::*;
use semio_framework_plugin::{plugin_app_close_prelude::UiMap, Component, Trigger};

fn descendants<'a>(node: &'a BuiltNode, out: &mut Vec<&'a BuiltNode>) {
    out.push(node);
    for child in &node.children {
        descendants(child, out);
    }
}

fn argument_number(arguments: &UiMap, name: &str) -> Option<f64> {
    arguments.iter().find_map(|(key, value)| (key.as_str() == name).then_some(value)).and_then(|value| match value {
        UiValue::Number(value) => Some(value),
        _ => None,
    })
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_table_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

#[semio_framework_async_macros::async_test]
async fn neutral_sparse_blank_and_formula_cases_render_as_real_editable_grids() {
    use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet, XlsxWorkbook};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🧫️fixtures/📊️sheet-grid/🔣️.json")).expect("third-party JSON parser reads neutral grid fixture");
    for case in fixture["cases"].as_array().unwrap() {
        let cells = case["cells"]
            .as_array()
            .unwrap()
            .iter()
            .map(|cell| XlsxCell {
                row: cell["row"].as_u64().unwrap() as u32,
                col: cell["column"].as_u64().unwrap() as u32,
                value: semio_framework_pack_json::from_json_str::<XlsxCellValue>(&cell["value"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap(),
            })
            .collect();
        let sheet_name = case["sheetName"].as_str().unwrap();
        let document = crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(XlsxWorkbook { sheets: vec![XlsxSheet { name: sheet_name.into(), cells }], ..Default::default() });
        let node = render(&document, Locale::En, &TreeWindows::unhosted(), semio_framework_plugin::UiPublicationRevision(23)).expect("grid render");
        let mut nodes = Vec::new();
        descendants(&node, &mut nodes);
        let table = nodes.iter().find(|node| matches!(node.component, Component::Table(_))).expect("materialized sheet table");
        let Component::Table(props) = &table.component else { unreachable!() };
        assert_eq!(props.window.map(|window| window.total), Some(case["expectedRows"].as_u64().unwrap() as u32), "{} row extent", case["id"]);
        assert_eq!(props.column_window.map(|window| window.total), Some(case["expectedColumns"].as_u64().unwrap() as u32), "{} column extent", case["id"]);
        assert_eq!(props.columns[0].0.as_str(), "A");
        let vacancy = &case["vacancy"];
        let vacancy_input = nodes
            .iter()
            .filter_map(|node| {
                let Component::Input(props) = &node.component else { return None };
                let binding = node.bindings.iter().find(|binding| binding.trigger == Trigger::Commit)?;
                let Some(UiValue::Map(arguments)) = &binding.args else { return None };
                (argument_number(arguments, "row") == vacancy["row"].as_f64() && argument_number(arguments, "column") == vacancy["column"].as_f64()).then_some((props, binding, arguments))
            })
            .next()
            .expect("fixture vacancy is a rendered editable cell");
        assert_eq!(vacancy_input.0.value.as_str(), "");
        assert_eq!(vacancy_input.1.action.scope.as_str(), BASE_CONTROLLER_ID);
        assert_eq!(vacancy_input.1.action.name.as_str(), "set-cell");
        let expected_revision = xlsx_worksheet_address(&document, sheet_name).unwrap().revision;
        assert!(matches!(vacancy_input.2.iter().find_map(|(key, value)| (key.as_str() == "revision").then_some(value)), Some(UiValue::Text(value)) if value.as_str() == expected_revision));
        if let Some(formula) = case["formulaDraft"].as_str() {
            assert!(nodes.iter().any(|node| matches!(&node.component, Component::Input(props) if props.value.as_str() == formula)));
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn german_grid_localizes_axes_without_rewriting_column_coordinates() {
    use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{XlsxSheet, XlsxWorkbook};
    let document = crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(XlsxWorkbook { sheets: vec![XlsxSheet { name: "Leer".into(), cells: Vec::new() }], ..Default::default() });
    let node = render(&document, Locale::De, &TreeWindows::unhosted(), semio_framework_plugin::UiPublicationRevision(23)).unwrap();
    let mut nodes = Vec::new();
    descendants(&node, &mut nodes);
    let table = nodes.iter().find(|node| matches!(node.component, Component::Table(_))).unwrap();
    let Component::Table(props) = &table.component else { unreachable!() };
    assert_eq!(props.row_label.as_ref().map(|label| label.0.as_str()), Some("Zeile"));
    assert_eq!(props.column_label.as_ref().map(|label| label.0.as_str()), Some("Spalte"));
    assert_eq!(props.columns[0].0.as_str(), "A");
}
