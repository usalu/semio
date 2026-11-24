//! 🔄️ LAW: detached Sync Attach and the unattached Task Manager remain real Shell-owned leaves.

use semio_renderer_wgpu_fixtures::shell::host_test_shell;
use super::*;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/🔄️shell-utility-leaves/🔣️.json")).expect("shell utility fixture")
}

fn find_id<'a>(value: &'a Value, id: &str) -> Option<&'a Value> {
    if value.get("id").and_then(Value::as_str) == Some(id) {
        return Some(value);
    }
    match value {
        Value::Array(items) => items.iter().find_map(|item| find_id(item, id)),
        Value::Object(fields) => fields.values().find_map(|item| find_id(item, id)),
        _ => None,
    }
}

#[test]
fn detached_sync_leaf_is_always_dockable_and_owns_all_three_choices() {
    let mut shell = host_test_shell();
    let fixture = fixture();
    assert!(shell.sync.sync_backbone_uri.is_none());
    let dock = SessionCtx::of(&mut shell).default_dock();
    let leaf = dock.tabs(PanelAnchor::BottomLeft).iter().find(|tab| tab.id == FRAMEWORK_SYNC_PANEL_TAB_ID).expect("detached Sync leaf");
    assert_eq!(leaf.label, fixture["sync"]["detachedLabel"]["en"].as_str().expect("detached label"));
    let body = serde_json::to_value(shell.sync.build_sync_attach_ui(&shell.prefs)).expect("sync body");
    for id in fixture["sync"]["choiceIds"].as_array().expect("choice ids").iter().filter_map(Value::as_str) {
        assert!(find_id(&body, id).is_some(), "missing detached choice {id}");
    }
    assert!(find_id(&body, "framework.sync.detach").is_none(), "detached state cannot offer detach");
}

#[test]
fn sync_body_routes_draft_attach_and_detach_through_the_existing_controller() {
    let mut shell = host_test_shell();
    semio_framework_async::block_on(ActionsCtx::of(&mut shell).handle_sync_action(ActionDescriptor { controller_id: "framework.sync".into(), action: "selectRemote".into(), args: None })).expect("select remote");
    semio_framework_async::block_on(ActionsCtx::of(&mut shell).handle_sync_action(ActionDescriptor { controller_id: "framework.sync".into(), action: "setSyncDraft".into(), args: semio_renderer_wgpu_host_base::action_args_json!({ "value": "https://hub.example" }) })).expect("draft route");
    assert_eq!(shell.sync.sync_card_draft, "https://hub.example");
    let remote = serde_json::to_value(shell.sync.build_sync_attach_ui(&shell.prefs)).expect("remote body");
    let attach = find_id(&remote, "framework.sync.attach").expect("attach action").to_string();
    assert!(attach.contains("framework.sync") && attach.contains("attach") && attach.contains("remote"));

    shell.sync.sync_backbone_uri = Some("remote://https://hub.example".into());
    let attached = serde_json::to_value(shell.sync.build_sync_attach_ui(&shell.prefs)).expect("attached body");
    let detach = find_id(&attached, "framework.sync.detach").expect("detach action").to_string();
    assert!(detach.contains("framework.sync") && detach.contains("detach"));
    semio_framework_async::block_on(ActionsCtx::of(&mut shell).handle_sync_action(ActionDescriptor { controller_id: "framework.sync".into(), action: "detach".into(), args: None })).expect("detach route");
    assert!(shell.sync.sync_backbone_uri.is_none());
}

#[test]
fn task_manager_leaf_body_and_command_report_the_unattached_runtime_truthfully() {
    let mut shell = host_test_shell();
    let fixture = fixture();
    let dock = SessionCtx::of(&mut shell).default_dock();
    let roots: Vec<&str> = dock.tabs(PanelAnchor::BottomRight).iter().map(|tab| tab.id.as_str()).collect();
    assert!(roots.contains(&FRAMEWORK_TASK_MANAGER_PANEL_ID));
    let body = serde_json::to_value(shell.opening.build_task_manager_ui(&shell.prefs)).expect("task manager body");
    let unavailable = find_id(&body, "os.task-manager.no-runtime").expect("truthful no-runtime row").to_string();
    assert!(unavailable.contains(fixture["taskManager"]["message"]["en"].as_str().expect("message")));
    for action in ["suspend", "resume", "cancel"] {
        assert!(!body.to_string().contains(&format!("\"action\":\"{action}\"")), "no actor action without an actor runtime");
    }
    assert!(shell.docs.build_os_commands(&shell.chrome).iter().any(|command| command.id == fixture["taskManager"]["commandId"].as_str().expect("command id")));
    SessionCtx::of(&mut shell).sync_dock_tabs();
    semio_framework_async::block_on(ActionsCtx::of(&mut shell).apply_os_command("os.openTaskManager", None)).expect("open task manager");
    let state = shell.dock.anchor_state(PanelAnchor::BottomRight);
    assert!(state.visible);
    assert_eq!(state.active_tab(), Some(FRAMEWORK_TASK_MANAGER_PANEL_ID));
}

#[test]
fn shell_owned_retained_document_closure_includes_sync_and_task_manager() {
    let shell = host_test_shell();
    let leaves = shell.docs.shell_owned_panel_leaves(&shell.chrome, &shell.plugins, &shell.prefs);
    assert!(leaves.iter().any(|leaf| leaf == FRAMEWORK_SYNC_PANEL_TAB_ID));
    assert!(leaves.iter().any(|leaf| leaf == FRAMEWORK_TASK_MANAGER_PANEL_ID));
    for (id, body) in [(FRAMEWORK_SYNC_PANEL_TAB_ID, shell.sync.build_sync_attach_ui(&shell.prefs)), (FRAMEWORK_TASK_MANAGER_PANEL_ID, shell.opening.build_task_manager_ui(&shell.prefs))] {
        let records = panel_ui_records(id, &body).unwrap_or_else(|error| panic!("{id} retained projection: {error}"));
        assert!(!records.is_empty(), "{id} retained document");
    }
}

fn doc_open_fixture() -> Value {
    serde_json::from_str(include_str!("../../../🧵️TaskManager/🧪️tests/🧫️fixtures/🧵️actor-table.json")).expect("actor table fixture")
}

#[test]
fn task_manager_leaf_lists_the_runtime_actors_with_all_three_row_actions() {
    use semio_renderer_wgpu_shell_doc_open::{RuntimeActorSample, TaskManagerActionKind};
    let mut shell = host_test_shell();
    let actors: Vec<RuntimeActorSample> = doc_open_fixture()["snapshot"]["actors"].as_array().expect("actors").iter().map(|actor| serde_json::from_value(actor.clone()).expect("actor")).collect();
    shell.opening.set_runtime_actors(Some(actors));
    let body = serde_json::to_value(shell.opening.build_task_manager_ui(&shell.prefs)).expect("task manager body");
    assert!(find_id(&body, "os.task-manager.no-runtime").is_none(), "an attached runtime is not the no-runtime state");
    let text = body.to_string();
    for needle in ["\"type\":\"table\"", "\"cells\"", "actor-1", "actor-2", "os.task-manager"] {
        assert!(text.contains(needle), "retained table misses {needle}");
    }
    assert!(!text.contains("columnsJson"), "the actor table is a retained table, not a TableScene surface");
    for kind in TaskManagerActionKind::ALL {
        assert!(text.contains(kind.tag()), "row action {}", kind.tag());
    }
    for label in ["Actor", "Package", "Lane", "p95 wall (µs)", "Mailbox", "Maintenance", "Suspended", "healthy", "—"] {
        assert!(text.contains(label), "missing {label}");
    }
    shell.opening.set_runtime_actors(Some(Vec::new()));
    let empty = serde_json::to_value(shell.opening.build_task_manager_ui(&shell.prefs)).expect("empty body");
    assert!(find_id(&empty, "os.task-manager.no-actors").is_some() && find_id(&empty, "os.task-manager.no-runtime").is_none());
}

#[test]
fn task_manager_actor_table_projects_to_retained_table_records_with_one_row_target() {
    let _arena = semio_ui_contract_values::testkit::UiArenaFixture::new();
    use semio_renderer_wgpu_shell_doc_open::{RuntimeActorSample, TaskManagerActionKind};
    let mut shell = host_test_shell();
    let actors: Vec<RuntimeActorSample> = doc_open_fixture()["snapshot"]["actors"].as_array().expect("actors").iter().map(|actor| serde_json::from_value(actor.clone()).expect("actor")).collect();
    let count = actors.len();
    shell.opening.set_runtime_actors(Some(actors));
    let body = shell.opening.build_task_manager_ui(&shell.prefs);
    let records = panel_ui_records(FRAMEWORK_TASK_MANAGER_PANEL_ID, &body).expect("task manager retained projection");
    let table = records.iter().find_map(|record| match &record.component { ui_contract::Component::Table(props) => Some((record, props)), _ => None }).expect("one retained Table record");
    assert_eq!(table.1.columns.len(), 11, "eleven text columns, the actions column is the table's actions label");
    assert!(table.1.actions_label.is_some());
    let rows: Vec<_> = records.iter().filter_map(|record| match &record.component { ui_contract::Component::TableRow(props) => Some((record, props)), _ => None }).collect();
    assert_eq!(rows.len(), count);
    assert_eq!(table.0.children.len(), count);
    for (record, props) in rows {
        assert_eq!(props.cells.len(), 11, "cells are positional to the columns");
        assert!(record.key.as_str().ends_with(props.cells.get(0).expect("actor id cell").as_str()), "row key is the actor id");
        let verbs: Vec<&str> = props.row_actions.iter().map(|action| action.verb.as_str()).collect();
        assert_eq!(verbs, TaskManagerActionKind::ALL.iter().map(|kind| kind.tag()).collect::<Vec<_>>());
        let target = props.target.as_ref().expect("the one row target");
        assert!(target.activation.is_none(), "an action-only row has no activation");
    }
}

#[test]
fn task_manager_actor_and_task_presses_queue_requests_for_the_runtime_bridge() {
    use semio_renderer_wgpu_shell_doc_open::{TaskManagerActionKind, TaskManagerRequest};
    let mut shell = host_test_shell();
    for (action, args) in [("suspend", json_args("actorId", "actor-1")), ("resume", json_args("task", "kernel:wfc#1")), ("resume", json_args("task", "toolRun:wfc#1"))] {
        semio_framework_async::block_on(ActionsCtx::of(&mut shell).dispatch_action(ActionDescriptor { controller_id: TASK_MANAGER_ID.into(), action: action.into(), args })).expect("task manager press");
    }
    assert_eq!(shell.opening.take_task_manager_requests(), vec![TaskManagerRequest { kind: TaskManagerActionKind::Resume, actor_id: None, task_id: Some("kernel:wfc#1".into()) }], "a tool run's press is answered with its own panel action, never left queued for the runtime bridge");
    let controls = semio_renderer_wgpu_host_base::take_actor_controls();
    assert!(controls.iter().any(|control| control.actor_id == "actor-1" && control.kind == semio_renderer_wgpu_host_base::ActorControlKind::Suspend), "the actor press reaches the kernel pool port");
}

fn json_args(key: &str, value: &str) -> Option<semio_framework::DslValue> {
    semio_renderer_wgpu_host_base::action_args_json!({ key: value })
}

const TASK_MANAGER_ID: &str = "os.task-manager";

#[test]
fn unknown_shell_route_is_recorded_and_known_routes_clear_it() {
    let _arena = semio_ui_contract_values::testkit::UiArenaFixture::new();
    let mut shell = host_test_shell();
    for (uri, not_found) in [("/nope", Some("/nope")), ("/hub", None), ("/", None), ("/spaces/s1/extra/1", Some("/spaces/s1/extra/1"))] {
        let _ = semio_framework_async::block_on(ActionsCtx::of(&mut shell).apply_shell_uri(uri));
        assert_eq!(shell.opening.route_not_found.as_deref(), not_found, "{uri}");
    }
    let fixture = doc_open_fixture();
    let page = serde_json::to_value(shell.opening.build_route_not_found_ui("/nope", &shell.prefs)).expect("route not found page");
    assert!(find_id(&page, "os.route-not-found").is_some());
    assert!(page.to_string().contains(fixture["routeNotFound"]["en"].as_str().expect("en")));
    let home = find_id(&page, "os.route-not-found.home").expect("home button").to_string();
    assert!(home.contains("navigateShellUri"));
    semio_renderer_wgpu_fixtures::shell::retire_shell_documents(&mut shell);
}
