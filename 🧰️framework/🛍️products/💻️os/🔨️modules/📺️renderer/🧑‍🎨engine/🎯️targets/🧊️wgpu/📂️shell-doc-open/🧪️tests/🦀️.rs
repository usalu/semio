//! 📂️ Unit tests of shell-doc-open; the work package moves the tests of its sources here.

use super::*;
use serde_json::{Value, json};
use std::collections::HashMap;

fn actor_table() -> Value {
    serde_json::from_str(include_str!("../../../../🧱️elements/🧵️TaskManager/🧪️tests/🧫️fixtures/🧵️actor-table.json")).expect("actor table fixture")
}

fn running_tasks() -> Value {
    serde_json::from_str(include_str!("../../../../🧱️elements/🧵️TaskManager/🧪️tests/🧫️fixtures/🏃️running-tasks.json")).expect("running tasks fixture")
}

fn typed<T: serde::de::DeserializeOwned>(value: &Value) -> T {
    serde_json::from_value(value.clone()).expect("typed fixture value")
}

/// 🧵️ LAW: a runtime metrics snapshot maps to one honest row per actor, and the table scene is the fixture's byte shape in both locales.
#[test]
fn actor_table_replays_the_fixture_in_both_locales() {
    let fixture = actor_table();
    let actors: Vec<RuntimeActorSample> = typed(&fixture["snapshot"]["actors"]);
    let rows = runtime_metrics_rows(&actors);
    assert_eq!(serde_json::to_value(&rows).expect("rows"), fixture["rows"]);
    for (locale, is_de) in [("en", false), ("de", true)] {
        assert_eq!(Value::Array(task_manager_columns(is_de)), fixture["columns"][locale], "{locale} columns");
        assert_eq!(Value::Array(task_manager_rows(&rows, is_de)), fixture["tableRows"][locale], "{locale} rows");
        let (columns_json, rows_json) = build_task_manager_table_scene(&rows, is_de);
        assert_eq!(serde_json::from_str::<Value>(&columns_json).expect("columns json"), fixture["columns"][locale]);
        assert_eq!(serde_json::from_str::<Value>(&rows_json).expect("rows json"), fixture["tableRows"][locale]);
    }
}

#[test]
fn actor_row_cells_print_the_em_dash_for_every_unobserved_counter() {
    let fixture = actor_table();
    let actors: Vec<RuntimeActorSample> = typed(&fixture["snapshot"]["actors"]);
    let cells = task_manager_row_cells(&runtime_metrics_rows(&actors)[1], false);
    assert_eq!(cells.as_slice(), ["actor-2", "s.stdio", "Maintenance", "Suspended", "healthy", "0", "—", "—", "—", "—", "—"]);
}

/// 🏃️ LAW: every live job, installation, tool call and tool run becomes exactly one task, and elapsed time never goes below zero.
#[test]
fn running_tasks_replay_the_fixture() {
    let fixture = running_tasks();
    let jobs: Vec<SpawnedJobRow> = typed(&fixture["jobs"]);
    let installs: Vec<String> = typed(&fixture["installs"]["pluginIds"]);
    let started: HashMap<String, u64> = typed(&fixture["installs"]["startedAtMs"]);
    let conversation: Vec<AgentConversationRow> = typed(&fixture["conversation"]);
    let tasks: Vec<TaskManagerTask> = [spawned_job_tasks(&jobs), install_tasks(&installs, &started, 0), tool_call_tasks(&conversation)].concat();
    assert_eq!(serde_json::to_value(&tasks).expect("tasks"), fixture["expected"]);
    for row in fixture["elapsed"].as_array().expect("elapsed") {
        assert_eq!(task_manager_elapsed_seconds(row["startedAtMs"].as_u64().expect("start"), row["nowMs"].as_u64().expect("now")), row["seconds"].as_u64().expect("seconds"));
    }
    let runs: Vec<TaskManagerToolRun> = typed(&fixture["toolRuns"]["runs"]);
    let program = &fixture["toolRuns"]["program"];
    let started: HashMap<String, u64> = typed(&fixture["toolRuns"]["startedAtMs"]);
    let tool_runs = tool_run_tasks(&runs, program["pluginId"].as_str().expect("plugin"), program["spawnedId"].as_str().expect("spawned"), &started, fixture["toolRuns"]["nowMs"].as_u64().expect("now"));
    assert_eq!(serde_json::to_value(&tool_runs).expect("tool runs"), fixture["toolRuns"]["expected"]);
}

#[test]
fn task_progress_line_and_percent_follow_the_fixture_words() {
    for case in actor_table()["progress"].as_array().expect("progress cases") {
        let task: TaskManagerTask = typed(&case["task"]);
        let now = case["nowMs"].as_u64().expect("now");
        assert_eq!(task_manager_progress_text(&task, now, false), case["text"]["en"].as_str().expect("en"));
        assert_eq!(task_manager_progress_text(&task, now, true), case["text"]["de"].as_str().expect("de"));
        assert_eq!(task_manager_determinate_percent(&task), case["percent"].as_u64());
    }
}

/// 🧭️ LAW: shell paths classify exactly as React's `parseShellRoute`.
#[test]
fn shell_routes_replay_the_fixture() {
    for case in actor_table()["routes"].as_array().expect("routes") {
        assert_eq!(serde_json::to_value(parse_shell_route(case["path"].as_str().expect("path"))).expect("route"), case["route"], "{}", case["path"]);
    }
}

#[test]
fn route_not_found_message_is_the_shared_en_de_label() {
    let fixture = actor_table();
    let expected = &fixture["routeNotFound"];
    let path = expected["path"].as_str().expect("path");
    assert_eq!(semio_renderer_wgpu_shell_vocab::shell_chrome_format("common.routeNotFound", false, &[("path", path)]), expected["en"].as_str().expect("en"));
    assert_eq!(semio_renderer_wgpu_shell_vocab::shell_chrome_format("common.routeNotFound", true, &[("path", path)]), expected["de"].as_str().expect("de"));
    assert_eq!(json!({ "controllerId": "framework", "action": NAVIGATE_SHELL_URI_ACTION, "args": { "uri": "/" } }), expected["homeAction"]);
}

#[test]
fn an_icon_import_stores_an_svg_as_text_and_any_other_image_as_its_data_url() {
    let svg = semio_framework_io_base64::base64_standard_encode(b"  <svg viewBox=\"0 0 1 1\"/>\n");
    assert_eq!(icon_import_value("Mark.SVG", &format!("data:application/svg;base64,{svg}")).as_deref(), Some("<svg viewBox=\"0 0 1 1\"/>"));
    assert_eq!(icon_import_value("a.jpg", "data:application/jpg;base64,QUJD").as_deref(), Some("data:image/jpeg;base64,QUJD"));
    assert_eq!(icon_import_value("a.png", "data:image/png;base64,QUJD").as_deref(), Some("data:image/png;base64,QUJD"));
    assert_eq!(icon_import_value("a.txt", "data:text/plain;base64,QUJD"), None);
    assert_eq!(icon_import_value("a.png", "not-a-data-url"), None);
}
