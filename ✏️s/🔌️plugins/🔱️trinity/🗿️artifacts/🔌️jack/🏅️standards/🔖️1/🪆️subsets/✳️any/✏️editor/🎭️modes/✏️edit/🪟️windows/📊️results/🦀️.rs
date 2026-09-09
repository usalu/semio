//! 📊️ Trinity Jack app — Results window (jack-query table/graph render).

use crate::ast::{QueryResult, QueryResultKind};
use crate::PropertyValue;
use semio_framework_plugin::{scene_surface, BuiltNode, NodeGraphScene, TableScene, UiAssemblyResult};
use semio_framework_ui_contract::SurfaceKind;

fn property_value_to_string(value: &PropertyValue) -> String {
    match value {
        PropertyValue::String(text) => text.clone(),
        PropertyValue::Number(number) => number.to_string(),
        PropertyValue::Bool(flag) => flag.to_string(),
        PropertyValue::Null => "null".into(),
        PropertyValue::Array(items) => pack::to_json_string(items),
        PropertyValue::Object(map) => pack::to_json_string(map),
    }
}

fn result_to_table(parsed: &QueryResult) -> (String, String) {
    let columns: Vec<pack::JsonValue> = parsed.columns.iter().map(|column| pack::json!({ "id": column, "label": column })).collect();
    let rows: Vec<pack::JsonValue> = parsed
        .rows
        .iter()
        .enumerate()
        .map(|(index, row)| {
            let mut record = pack::JsonObject::new();
            record.insert("index", pack::json!(index + 1));
            for (column, value) in parsed.columns.iter().zip(row.iter()) {
                record.insert(column.clone(), pack::json!(property_value_to_string(value)));
            }
            pack::JsonValue::Object(record)
        })
        .collect();
    (pack::to_json_string(&columns), pack::to_json_string(&rows))
}

pub(crate) fn render(surface_id: &str, _controller_id: &str, result: Option<&QueryResult>, error: Option<&str>) -> UiAssemblyResult<BuiltNode> {
    if let Some(error) = error {
        return semio_framework_plugin::built_text_node(semio_framework_plugin::Label::data(error)).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("trinity.query.error-label", "the query error exceeds its UI label bound"));
    }
    let empty = QueryResult::table(Vec::new(), Vec::new());
    let result = result.unwrap_or(&empty);
    if result.kind == QueryResultKind::Graph {
        if let Some(fixture) = &result.graph_fixture {
            let (nodes, edges, viewport) = crate::editor::jack::fixture_to_workflow(fixture);
            return scene_surface(surface_id, SurfaceKind::NodeGraph, &NodeGraphScene::base(nodes, edges, viewport));
        }
    }
    let (columns_json, rows_json) = result_to_table(result);
    scene_surface(surface_id, SurfaceKind::Table, &TableScene::base(columns_json, rows_json))
}
