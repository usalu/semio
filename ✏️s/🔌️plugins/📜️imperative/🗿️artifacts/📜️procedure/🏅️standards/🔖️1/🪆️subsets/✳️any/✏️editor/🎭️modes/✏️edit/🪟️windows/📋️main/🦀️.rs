//! 📋️ Imperative play app — the main window: a table of the document's top-level steps plus, once `run`
//! has been dispatched, the resulting scope.

use crate::artifacts::procedure::{ProcedureSnapshot, Step};
use crate::editor::procedure::terminology::ImperativeLabels;
use semio_framework_plugin::app::{TableView, TableWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, SurfaceKind, WindowKindDefinition, WindowOptions};
use dsl::os_pack::json::Value;

//#region 🔖️Constants
pub const IMPERATIVE_PLAY_WINDOW_MAIN: &str = "imperative-main";
pub const IMPERATIVE_PLAY_BODY_MAIN: &str = "imperative.play.main";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: IMPERATIVE_PLAY_WINDOW_MAIN.into(),
        label: LocalizedLabel::native("Imperative", "Imperativ"),
        body_key: IMPERATIVE_PLAY_BODY_MAIN.into(),
        surface_kind: SurfaceKind::Table,
        icon_id: "code".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
struct TableRow {
    index: usize,
    id: String,
    kind: String,
}

impl TableRow {
    fn cells(self) -> Vec<String> {
        vec![self.index.to_string(), self.id, self.kind]
    }
}

fn table_rows(steps: &[Step]) -> Vec<TableRow> {
    steps.iter().enumerate().map(|(index, step)| TableRow { index: index + 1, id: step.id.clone(), kind: step.kind.clone() }).collect()
}

/// 📤️ One table row per scope key so the full run output is legible instead of an 80-char
/// truncated blob; falls back to the raw JSON when it isn't a plain object.
fn run_output_rows(run_output_json: &str, offset: usize) -> Vec<TableRow> {
    let parsed = dsl::os_pack::json::from_json_str::<Value>(run_output_json).ok();
    match parsed.as_ref().and_then(|value| value.as_object()) {
        Some(scope) if !scope.is_empty() => {
            scope.iter().enumerate().map(|(index, (key, value))| TableRow { index: offset + index + 1, id: format!("run-output.{key}"), kind: format!("{key} = {}", dsl::os_pack::json::to_json_string(value)) }).collect()
        }
        _ => vec![TableRow { index: offset + 1, id: "run-output".into(), kind: run_output_json.to_string() }],
    }
}

pub fn render(document: &ProcedureSnapshot, run_output_json: &str, labels: &ImperativeLabels) -> BuiltNode {
    let path = crate::artifacts::procedure::procedure_working_scene(document).path;
    let mut rows = table_rows(&path.steps);
    if !run_output_json.is_empty() {
        rows.extend(run_output_rows(run_output_json, rows.len()));
    }
    TableWindowKit::render(&TableView { columns: vec![labels.col_index.as_str().into(), labels.col_id.as_str().into(), labels.col_kind.as_str().into()], rows: rows.into_iter().map(TableRow::cells).collect() })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::procedure::testkit::{imperative_app, render as render_body};
    use crate::editor::procedure::ImperativeCommand;

    #[semio_framework_async_macros::async_test]
    async fn renders_table_scene() {
        let mut app = imperative_app().await;
        let json = render_body(&mut app, IMPERATIVE_PLAY_BODY_MAIN).await;
        assert!(json.contains("table"));
    }

    #[semio_framework_async_macros::async_test]
    async fn run_command_expands_scope_into_readable_rows_without_truncation() {
        use crate::editor::procedure::commands::run;
        use crate::editor::procedure::testkit::dispatch;
        let mut app = imperative_app().await;
        dispatch(&mut app, ImperativeCommand::Run(run::Run {})).await;
        let json = render_body(&mut app, IMPERATIVE_PLAY_BODY_MAIN).await;
        assert!(json.contains("log.print"), "main table lists default path steps after run");
    }
}
//#endregion 🧪️Tests
