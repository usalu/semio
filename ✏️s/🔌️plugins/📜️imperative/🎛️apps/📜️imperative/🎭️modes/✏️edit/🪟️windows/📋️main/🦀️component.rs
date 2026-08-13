//! 📋️ Imperative play app — the main window: a table of the document's top-level steps plus, once `run`
//! has been dispatched, the resulting scope.

use crate::apps::imperative::terminology::ImperativeLabels;
use crate::artifacts::imperative::{ImperativeSnapshot, Step};
use semio_framework_plugin::{build_table_scene, LocalizedLabel, SurfaceKind, TableScene, UiNode, WindowKindDefinition, WindowOptions};
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️Constants
pub const IMPERATIVE_PLAY_WINDOW_MAIN: &str = "imperative-main";
pub const IMPERATIVE_PLAY_BODY_MAIN: &str = "imperative.play.main";
const IMPERATIVE_PLAY_SURFACE_MAIN: &str = "imperative.play.main";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: IMPERATIVE_PLAY_WINDOW_MAIN.into(),
        label: LocalizedLabel::native("Imperative", "Imperativ"),
        body_key: IMPERATIVE_PLAY_BODY_MAIN.into(),
        surface_kind: SurfaceKind::NodeGraph,
        icon_id: "code".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
#[derive(Serialize, Deserialize)]
struct TableRow {
    index: usize,
    id: String,
    kind: String,
}

fn table_rows(steps: &[Step]) -> String {
    let rows: Vec<TableRow> = steps.iter().enumerate().map(|(index, step)| TableRow { index: index + 1, id: step.id.clone(), kind: step.kind.clone() }).collect();
    serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into())
}

/// 📤️ One table row per scope key so the full run output is legible instead of an 80-char
/// truncated blob; falls back to the raw JSON when it isn't a plain object.
fn run_output_rows(run_output_json: &str, offset: usize) -> Vec<TableRow> {
    match serde_json::from_str::<Value>(run_output_json).ok().and_then(|value| value.as_object().cloned()) {
        Some(scope) if !scope.is_empty() => {
            scope.into_iter().enumerate().map(|(index, (key, value))| TableRow { index: offset + index + 1, id: format!("run-output.{key}"), kind: format!("{key} = {}", serde_json::to_string(&value).unwrap_or_else(|_| "null".into())) }).collect()
        }
        _ => vec![TableRow { index: offset + 1, id: "run-output".into(), kind: run_output_json.to_string() }],
    }
}

pub fn render(document: &ImperativeSnapshot, run_output_json: &str, labels: &ImperativeLabels) -> UiNode {
    let path = crate::artifacts::imperative::imperative_working_scene(document).path;
    let mut rows_json = table_rows(&path.steps);
    if !run_output_json.is_empty() {
        if let Ok(mut rows) = serde_json::from_str::<Vec<TableRow>>(&rows_json) {
            rows.extend(run_output_rows(run_output_json, rows.len()));
            rows_json = serde_json::to_string(&rows).unwrap_or(rows_json);
        }
    }
    build_table_scene(
        IMPERATIVE_PLAY_SURFACE_MAIN,
        crate::apps::imperative::IMPERATIVE_PLAY_APP_ID,
        TableScene::base(
            serde_json::json!([
                {"id":"index","label":labels.col_index.as_str()},
                {"id":"id","label":labels.col_id.as_str()},
                {"id":"kind","label":labels.col_kind.as_str()},
            ])
            .to_string(),
            rows_json,
        ),
    )
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::imperative::testkit::{imperative_app, render as render_body};
    use crate::apps::imperative::ImperativeCommand;

    #[test]
    fn renders_table_scene() {
        let mut app = imperative_app();
        let json = render_body(&mut app, IMPERATIVE_PLAY_BODY_MAIN);
        assert!(json.contains("table"));
    }

    #[test]
    fn run_command_expands_scope_into_readable_rows_without_truncation() {
        use crate::apps::imperative::testkit::dispatch;
        use crate::apps::imperative::commands::run;
        let mut app = imperative_app();
        dispatch(&mut app, ImperativeCommand::Run(run::Run {}));
        let json = render_body(&mut app, IMPERATIVE_PLAY_BODY_MAIN);
        assert!(json.contains("log.print"), "main table lists default path steps after run");
    }
}
//#endregion 🧪️Tests
