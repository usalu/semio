use super::*;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

const FIXTURE: &str = include_str!("../../../🧫️fixtures/🎮️registry/🏗️workspace.json");

fn materialise(name: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let root = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map_or_else(std::env::temp_dir, PathBuf::from).join(format!("semio-inventory-{name}-{nanos}"));
    let document: serde_json::Value = serde_json::from_str(FIXTURE).unwrap();
    for file in document["files"].as_array().unwrap() { write(&root, file["path"].as_str().unwrap(), &file.get("json").map_or_else(|| file["text"].as_str().unwrap().to_string(), |json| serde_json::to_string_pretty(json).unwrap())); }
    root
}

fn write(root: &Path, path: &str, text: &str) {
    let target = path.split('/').fold(root.to_path_buf(), |path, part| path.join(part));
    std::fs::create_dir_all(target.parent().unwrap()).unwrap();
    std::fs::write(target, text).unwrap();
}

fn ids(registry: &Registry) -> Vec<String> { registry.entries().iter().map(|entry| entry.id.clone()).collect() }

fn never() -> AtomicBool { AtomicBool::new(false) }

#[test]
fn discovery_reads_manifests_the_graph_the_catalog_and_the_open_tickets_and_nothing_else() {
    let root = materialise("sources");
    let registry = discover(&root, &never());
    assert_eq!(registry.problems(), &[] as &[Problem]);
    let ids = ids(&registry);
    for expected in ["workspace:verify", "hub:serve-docs", "@fixture/quiz:materialize-dev", "@fixture/artifact-wav:check", "@fixture/artifact-pdf:watch-tests", "playground:shell", "tool:workspace/inspector", "ticket:26/09/23/OPEN-PROBE/probe"] { assert!(ids.contains(&expected.to_string()), "{expected}"); }
    assert!(!ids.iter().any(|id| id.starts_with("script:") || id.contains("vanished") || id.contains("CLOSED-PROBE/stale")));
    let wav = registry.find("@fixture/artifact-wav:check").unwrap();
    assert_eq!(wav.source, ".nx/workspace-data/project-graph.json (project @fixture/artifact-wav)", "a graph-only target names the graph as its source");
    assert_eq!(registry.find("hub:dev").unwrap().source, "🌎️hub/📦️packages/🟦️typescript/📋️project.json");
    assert!(registry.find("@fixture/artifact-pdf:watch-tests").unwrap().long_running, "Nx `continuous` makes a target long-running");
    assert!(registry.find("hub:serve-docs").unwrap().long_running, "the graph's `continuous` reaches the registry");
    fs_remove(&root);
}

fn fs_remove(root: &Path) { std::fs::remove_dir_all(root).ok(); }

#[test]
fn the_catalog_facts_of_a_playground_are_read_with_the_row() {
    let root = materialise("catalog");
    let facts = read_playgrounds(&root).value;
    let shell = facts.iter().find(|row| row.variant == "shell").unwrap();
    assert_eq!((shell.hub.as_deref(), shell.data_dir.as_deref(), shell.user_data_dir.as_deref(), shell.local_only, shell.user_react.clone()), (Some("http://127.0.0.1:8787"), Some(".data/shell-base"), Some(".data/shell-user{N}"), true, vec![6072, 6073]));
    assert_eq!(facts.iter().find(|row| row.variant == "cad").unwrap().viewer_path.as_deref(), Some("/?plugin=cad&role=viewer"));
    fs_remove(&root);
}

#[test]
fn a_snapshot_serves_an_unchanged_workspace_and_gives_way_to_any_changed_source() {
    let root = materialise("snapshot");
    let snapshot = root.join(".snapshot/commands.json");
    let first = registry_at(&root, &snapshot);
    assert!(snapshot.exists(), "the first call writes the snapshot");
    let written = std::fs::metadata(&snapshot).unwrap().modified().unwrap();
    let second = registry_at(&root, &snapshot);
    assert_eq!(ids(&first), ids(&second));
    assert_eq!(std::fs::metadata(&snapshot).unwrap().modified().unwrap(), written, "an unchanged workspace does not rewrite the snapshot");
    write(&root, "🎓️teaching/❓️quiz/📦️packages/🟦️typescript/📋️project.json", &serde_json::json!({ "name": "@fixture/quiz", "targets": { "dev": {}, "added-later": {} } }).to_string());
    let third = registry_at(&root, &snapshot);
    assert!(ids(&third).contains(&"@fixture/quiz:added-later".to_string()), "a changed manifest is read again");
    assert!(!ids(&first).contains(&"@fixture/quiz:added-later".to_string()));
    write(&root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/OPEN-PROBE/🎮️commands.json", r#"{ "tools": [{ "id": "second", "command": ["x"] }] }"#);
    assert!(ids(&registry_at(&root, &snapshot)).contains(&"ticket:26/09/23/OPEN-PROBE/second".to_string()), "a changed ticket command document is read again");
    fs_remove(&root);
}

#[test]
fn a_corrupt_snapshot_or_one_of_another_workspace_is_ignored() {
    let root = materialise("corrupt");
    let snapshot = root.join(".snapshot/commands.json");
    std::fs::create_dir_all(snapshot.parent().unwrap()).unwrap();
    std::fs::write(&snapshot, "{ not json").unwrap();
    assert!(ids(&registry_at(&root, &snapshot)).contains(&"hub:dev".to_string()));
    let other = materialise("other");
    registry_at(&other, &snapshot);
    assert_eq!(read_snapshot(&snapshot, &root), None, "a snapshot belongs to the workspace it was read from");
    fs_remove(&root);
    fs_remove(&other);
}

#[test]
fn the_walk_enters_neither_hidden_folders_build_output_nor_fixtures() {
    let root = materialise("walk");
    for folder in [".hidden", "node_modules/pkg", "target", "🧫️fixtures/sample", "📦️build"] { write(&root, &format!("{folder}/📋️project.json"), r#"{"name":"intruder","targets":{"build":{}}}"#); }
    write(&root, "🧰️tools/found/📋️project.json", r#"{"name":"found-by-walk","targets":{"build":{}}}"#);
    let ids = ids(&discover(&root, &never()));
    assert!(ids.contains(&"found-by-walk:build".to_string()));
    assert!(!ids.iter().any(|id| id.starts_with("intruder:")));
    fs_remove(&root);
}

#[test]
fn a_second_project_with_a_taken_name_is_reported_and_never_replaces_the_first() {
    let root = materialise("duplicate");
    write(&root, "🧰️tools/impostor/📋️project.json", &serde_json::json!({ "name": "workspace", "targets": { "alien": {} }, "metadata": { "semio": { "dashboard": { "tools": [{ "id": "alien-tool", "command": ["x"] }] } } } }).to_string());
    let registry = discover(&root, &never());
    assert!(registry.problems().iter().any(|problem| problem.file == "🧰️tools/impostor/📋️project.json" && problem.message.contains("already used by the project in \".\"")), "{:?}", registry.problems());
    let ids = ids(&registry);
    assert!(ids.contains(&"tool:workspace/inspector".to_string()) && !ids.contains(&"workspace:alien".to_string()) && !ids.contains(&"tool:workspace/alien-tool".to_string()));
    fs_remove(&root);
}

#[test]
fn a_broken_manifest_is_a_problem_naming_its_file_and_costs_nothing_else() {
    let root = materialise("broken");
    write(&root, "🧰️tools/broken/📋️project.json", "{ \"name\": ");
    write(&root, "🧰️tools/odd/📋️project.json", &serde_json::json!({ "name": "odd", "targets": { "dev": { "metadata": { "semio": { "dashboard": { "ready": { "port": 99999 } } } } } } }).to_string());
    let registry = discover(&root, &never());
    let problems: Vec<String> = registry.problems().iter().map(ToString::to_string).collect();
    assert!(problems.iter().any(|problem| problem.starts_with("🧰️tools/broken/📋️project.json")), "{problems:?}");
    assert!(problems.iter().any(|problem| problem.starts_with("🧰️tools/odd/📋️project.json: targets.dev.metadata.semio.dashboard.ready: `port` must be between 1 and 65535")), "{problems:?}");
    assert!(ids(&registry).contains(&"hub:dev".to_string()));
    fs_remove(&root);
}

#[test]
fn the_job_publishes_the_known_commands_first_then_the_result_of_the_walk_and_finishes() {
    let root = materialise("job");
    let snapshot = root.join(".snapshot/commands.json");
    let job = start_at(root.clone(), snapshot.clone(), false);
    let mut phases = Vec::new();
    let mut finished = false;
    while let Ok(update) = job.receiver.recv_timeout(Duration::from_secs(30)) {
        match update {
            Update::Ready(rows, phase) => { assert!(!rows.is_empty()); phases.push(phase); }
            Update::Finished => { finished = true; break; }
            Update::Failed(error) => panic!("{error}"),
        }
    }
    assert!(finished);
    assert_eq!((phases.first().map(String::as_str), phases.last().map(String::as_str)), (Some("known"), Some("complete")));
    assert_eq!(job.progress.phase(), Phase::Complete);
    assert!(job.registry().is_some_and(|registry| registry.find("hub:dev").is_some()));
    assert!(snapshot.exists());
    fs_remove(&root);
}

#[test]
fn a_partially_published_graph_is_a_problem_per_recorded_plugin_error() {
    let root = materialise("graph-errors");
    let path = root.join(".nx/workspace-data/project-graph.json");
    let mut graph: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    assert!(discover(&root, &never()).check().is_empty(), "a graph without an errors array is healthy");
    graph["errors"] = serde_json::json!([]);
    std::fs::write(&path, graph.to_string()).unwrap();
    assert!(discover(&root, &never()).check().is_empty(), "an empty errors array is healthy");
    graph["errors"] = serde_json::json!([{ "name": "AggregateCreateNodesError", "pluginIndex": 2, "message": "ignored" }, { "name": "ProcessDependenciesError", "pluginName": "@semio-tech/plugin" }, { "name": "Odd" }]);
    std::fs::write(&path, graph.to_string()).unwrap();
    let problems = discover(&root, &never()).check();
    let wanted = [("errors[0]", "AggregateCreateNodesError (plugin index 2)"), ("errors[1]", "ProcessDependenciesError (@semio-tech/plugin)"), ("errors[2]", "Odd (unknown plugin)")];
    assert_eq!(problems.len(), 3, "{problems:?}");
    for (problem, (at, what)) in problems.iter().zip(wanted) {
        assert_eq!((problem.file.as_str(), problem.at.as_str()), (".nx/workspace-data/project-graph.json", at));
        assert_eq!(problem.message, format!("the published project graph carries a plugin error: {what}; re-run bun nx show projects --json for the message"));
    }
    fs_remove(&root);
}

#[test]
fn a_cancelled_discovery_returns_nothing_and_leaves_no_snapshot() {
    let root = materialise("cancel");
    let cancelled = AtomicBool::new(true);
    assert!(gather(&root, None, &cancelled, &Progress::default(), &mut |_, _| {}).is_none());
    let seeded = discover(&root, &cancelled);
    assert!(seeded.find("workspace:build").is_some(), "a cancelled discovery falls back to the seed");
    assert!(seeded.find("hub:dev").is_none(), "the seed only knows the root manifest and the playgrounds");
    fs_remove(&root);
}

#[test]
fn a_task_label_tells_whether_it_starts_from_the_published_graph() {
    let root = materialise("graph-label");
    let registry = discover(&root, &never());
    let (build, dev) = (registry.find("workspace:build").unwrap().label.clone(), registry.find("workspace:dev").unwrap().label.clone());
    *PUBLISHED.lock().unwrap() = Some(Arc::new(registry));
    assert!(starts_from_published_graph(&build), "finite targets may reuse the graph");
    assert!(!starts_from_published_graph(&dev), "continuous targets build their own graph");
    assert!(!starts_from_published_graph("unheard of"));
    fs_remove(&root);
}
