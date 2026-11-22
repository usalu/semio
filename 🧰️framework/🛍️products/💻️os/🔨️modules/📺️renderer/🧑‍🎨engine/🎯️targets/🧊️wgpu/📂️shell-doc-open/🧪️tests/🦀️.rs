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

fn claims(rows: serde_json::Value) -> BrowserCatalog {
    BrowserCatalog::from_claims_json(&json!({ "formats": rows }).to_string()).expect("claim document")
}

fn claim(kind_id: &str, extension: &str, mime: &str, artifact_ref: &str) -> Value {
    json!({ "kindId": kind_id, "shortId": extension.trim_start_matches('.'), "aliases": [], "mimes": [mime], "extensions": [extension], "artifactRef": artifact_ref, "package": null, "revisions": [] })
}

fn dropped(name: &str, mime: &str) -> BrowserPickedFile {
    BrowserPickedFile { token: 7, name: name.into(), size: 12, mime: mime.into() }
}

fn route(file: BrowserPickedFile, catalog: &BrowserCatalog, cancelled: bool) -> BrowserOpenRouting {
    route_browser_file(file, catalog, cancelled, None, &|_| Vec::new())
}

/// 🛬️ LAW: a dropped or picked file routes by its extension to the dialect of the editor that declares the format, the same resolution React's `routePickedFileV1` runs.
#[test]
fn a_dropped_file_routes_to_the_editor_whose_format_claims_its_extension() {
    let catalog = claims(json!([claim("s.stdio.pdf@1.7", ".pdf", "application/pdf", "s.stdio.pdf@1.7/*"), claim("s.gis.geojson@1", ".geojson", "application/geo+json", "s.gis.geojson@1/*")]));
    assert_eq!(catalog.accept(), ".pdf,.geojson");
    match route(dropped("Map.GEOJSON", ""), &catalog, false) {
        BrowserOpenRouting::Routed { file, artifact_ref } => assert_eq!((file.token, artifact_ref.as_str()), (7, "s.gis.geojson@1/*")),
        _ => panic!("a claimed extension routes"),
    }
    match route(dropped("noext", "application/pdf"), &catalog, false) {
        BrowserOpenRouting::Routed { artifact_ref, .. } => assert_eq!(artifact_ref, "s.stdio.pdf@1.7/*"),
        _ => panic!("a claimed MIME routes"),
    }
}

/// 🚫️ LAW: a name no editor claims and a name two editors claim are named refusals that keep the page's token for release, never a guess.
#[test]
fn an_unclaimed_or_ambiguous_file_is_a_named_refusal_holding_its_token() {
    let catalog = claims(json!([claim("a@1", ".dat", "application/a", "a@1/*"), claim("b@1", ".dat", "application/b", "b@1/*")]));
    for (name, reason) in [("x.png", "format-unresolved"), ("x.dat", "format-ambiguous")] {
        let routing = route(dropped(name, ""), &catalog, false);
        assert_eq!(routing.token(), Some(7));
        assert!(matches!(routing, BrowserOpenRouting::Refused { reason: found, .. } if found == reason), "{name}");
    }
    assert!(BrowserCatalog::from_claims_json(r#"{"formats":[]}"#).is_none() && BrowserCatalog::from_claims_json("nope").is_none());
}

/// 🛑️ LAW: a cancel settles the routing as no pick, still holding the token, and refuses nothing.
#[test]
fn a_cancelled_routing_is_not_a_refusal() {
    let catalog = claims(json!([claim("a@1", ".dat", "application/a", "a@1/*")]));
    assert!(matches!(route(dropped("x.dat", ""), &catalog, true), BrowserOpenRouting::Cancelled(Some(7))));
}

/// 🔬️ LAW: a format's revisions are told apart by the declared needle in the file head, and each revision routes to its own editor, as React's `pickRevisionV1` does; the head is read through the range cache the route fills.
#[test]
fn the_head_sniff_routes_a_file_to_the_revision_it_declares() {
    let mut row = claim("s.doc@1", ".doc", "application/x-doc", "s.doc@1/*");
    row["revisions"] = json!([{ "kindId": "s.doc@2", "artifactRef": "s.doc@2/*", "package": null, "window": 16, "contains": ["REV2"] }]);
    let catalog = claims(json!([row]));
    let file = BrowserPickedFile { token: 7, name: "a.doc".into(), size: 20, mime: String::new() };
    let cache = RangeCache::new(file.size, || false);
    let routed = |bytes: &[u8]| {
        let cache = RangeCache::new(20, || false);
        cache.insert(0, 20, bytes.to_vec());
        match route_browser_file(file.clone(), &catalog, false, Some(&cache), &|count| cache.head(count)) {
            BrowserOpenRouting::Routed { artifact_ref, .. } => artifact_ref,
            _ => panic!("routes"),
        }
    };
    assert_eq!(routed(b"xxREV2xxxxxxxxxxxxxx"), "s.doc@2/*");
    assert_eq!(routed(b"plain head, no need"), "s.doc@1/*");
    assert_eq!(cache.head(4), Vec::<u8>::new());
    assert_eq!(cache.take_misses(), vec![(0, 4)], "an unread range is recorded for the page to read");
}

/// 🗜️ LAW: a format that claims a package check refuses a file whose content is not that package (`content-mismatch`), and asks the page for the ranges it needs before it decides.
#[test]
fn a_package_claim_refuses_content_that_is_not_the_package() {
    let mut row = claim("s.pkg@1", ".pkg", "application/x-pkg", "s.pkg@1/*");
    row["package"] = json!({ "mainPartContentTypes": ["application/x-main+xml"], "dialects": [{ "artifactRef": "s.pkg.a@1/*", "namespace": "urn:a" }] });
    let catalog = claims(json!([row]));
    let file = BrowserPickedFile { token: 7, name: "a.pkg".into(), size: 40, mime: String::new() };
    let cache = RangeCache::new(file.size, || false);
    let first = route_browser_file(file.clone(), &catalog, false, Some(&cache), &|count| cache.head(count));
    assert!(!cache.take_misses().is_empty(), "the check asks for the package tail before it decides");
    assert!(matches!(first, BrowserOpenRouting::Refused { .. } | BrowserOpenRouting::Cancelled(_)));
    cache.insert(0, 40, vec![0; 40]);
    let settled = route_browser_file(file, &catalog, false, Some(&cache), &|count| cache.head(count));
    assert!(matches!(settled, BrowserOpenRouting::Refused { reason: "content-mismatch", token: 7, .. }));
}

/// 🗣️ LAW: the refusal notice is the shared vocabulary's text in the shell's own language, every language joined for any other, under React's fault code.
#[test]
fn refusal_notices_are_the_shared_vocabulary_in_both_languages() {
    assert_eq!(picked_file_refusal_text("owner-unavailable", "a.pdf", Some("en")), "No editor for “a.pdf” is available in this workspace.");
    assert_eq!(picked_file_refusal_text("owner-unavailable", "a.pdf", Some("de")), "Für „a.pdf“ ist in diesem Arbeitsbereich kein Editor verfügbar.");
    let joined = picked_file_refusal_text("owner-unavailable", "a.pdf", Some("fr"));
    assert!(joined.contains("No editor for") && joined.contains("kein Editor") && joined.contains(" | "));
    assert_eq!(picked_file_refusal_code("content-mismatch"), "shell.openFile.content-mismatch");
}
