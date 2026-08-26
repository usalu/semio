//! 📊️ Trinity Jack app — Results window (jack-query table/graph render).

use crate::artifacts::jack::PropertyValue;
use crate::ast::{QueryResult, QueryResultKind};
use crate::editor::jack::config::JackConfig;
use semio_framework_plugin::{scene_surface, BuiltNode, NodeGraphScene, TableScene, UiAssemblyResult};
use semio_framework_ui_contract::SurfaceKind;
use serde_json::json;

fn property_value_to_string(value: &PropertyValue) -> String {
    match value {
        PropertyValue::String(text) => text.clone(),
        PropertyValue::Number(number) => number.to_string(),
        PropertyValue::Bool(flag) => flag.to_string(),
        PropertyValue::Null => "null".into(),
        PropertyValue::Array(items) => serde_json::to_string(items).unwrap_or_else(|_| "[]".into()),
        PropertyValue::Object(map) => serde_json::to_string(map).unwrap_or_else(|_| "{}".into()),
    }
}

fn result_to_table(result_json: &str) -> (String, String) {
    let parsed: QueryResult = serde_json::from_str(result_json).unwrap_or(QueryResult::table(vec![], vec![]));
    let columns: Vec<serde_json::Value> = parsed.columns.iter().map(|column| json!({ "id": column, "label": column })).collect();
    let rows: Vec<serde_json::Value> = parsed
        .rows
        .iter()
        .enumerate()
        .map(|(index, row)| {
            let mut record = serde_json::Map::new();
            record.insert("index".into(), json!(index + 1));
            for (column, value) in parsed.columns.iter().zip(row.iter()) {
                record.insert(column.clone(), json!(property_value_to_string(value)));
            }
            serde_json::Value::Object(record)
        })
        .collect();
    (serde_json::to_string(&columns).unwrap_or_else(|_| "[]".into()), serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into()))
}

pub(crate) fn render(surface_id: &str, _controller_id: &str, cfg: &JackConfig) -> UiAssemblyResult<BuiltNode> {
    let result: QueryResult = serde_json::from_str(&cfg.jack_result_json).unwrap_or(QueryResult::table(vec![], vec![]));
    if result.kind == QueryResultKind::Graph {
        if let Some(fixture) = &result.graph_fixture {
            let (nodes, edges, viewport) = crate::editor::jack::fixture_to_workflow(fixture);
            return scene_surface(surface_id, SurfaceKind::NodeGraph, &NodeGraphScene::base(nodes, edges, viewport));
        }
    }
    let (columns_json, rows_json) = result_to_table(&cfg.jack_result_json);
    scene_surface(surface_id, SurfaceKind::Table, &TableScene::base(columns_json, rows_json))
}
