//! 🔄️ LAW: detached Sync Attach and the unattached Task Manager remain real Shell-owned leaves.

use super::*;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../../../../🧫️fixtures/🔄️shell-utility-leaves/🔣️.json")).expect("shell utility fixture")
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
    let shell = super::panel_anchor_model_tests::host_test_shell();
    let fixture = fixture();
    assert!(shell.sync_backbone_uri.is_none());
    let dock = shell.default_dock();
    let leaf = dock.tabs(PanelAnchor::BottomLeft).iter().find(|tab| tab.id == FRAMEWORK_SYNC_PANEL_TAB_ID).expect("detached Sync leaf");
    assert_eq!(leaf.label, fixture["sync"]["detachedLabel"]["en"].as_str().expect("detached label"));
    let body = serde_json::to_value(shell.build_sync_attach_ui()).expect("sync body");
    for id in fixture["sync"]["choiceIds"].as_array().expect("choice ids").iter().filter_map(Value::as_str) {
        assert!(find_id(&body, id).is_some(), "missing detached choice {id}");
    }
    assert!(find_id(&body, "framework.sync.detach").is_none(), "detached state cannot offer detach");
}

#[test]
fn sync_body_routes_draft_attach_and_detach_through_the_existing_controller() {
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    semio_framework_async::block_on(shell.handle_sync_action(ActionDescriptor {
        controller_id: "framework.sync".into(),
        action: "selectRemote".into(),
        args: None,
    }))
    .expect("select remote");
    semio_framework_async::block_on(shell.handle_sync_action(ActionDescriptor {
        controller_id: "framework.sync".into(),
        action: "setSyncDraft".into(),
        args: crate::action_args_json!({ "value": "https://hub.example" }),
    }))
    .expect("draft route");
    assert_eq!(shell.sync_card_draft, "https://hub.example");
    let remote = serde_json::to_value(shell.build_sync_attach_ui()).expect("remote body");
    let attach = find_id(&remote, "framework.sync.attach").expect("attach action").to_string();
    assert!(attach.contains("framework.sync") && attach.contains("attach") && attach.contains("remote"));

    shell.sync_backbone_uri = Some("remote://https://hub.example".into());
    let attached = serde_json::to_value(shell.build_sync_attach_ui()).expect("attached body");
    let detach = find_id(&attached, "framework.sync.detach").expect("detach action").to_string();
    assert!(detach.contains("framework.sync") && detach.contains("detach"));
    semio_framework_async::block_on(shell.handle_sync_action(ActionDescriptor {
        controller_id: "framework.sync".into(),
        action: "detach".into(),
        args: None,
    }))
    .expect("detach route");
    assert!(shell.sync_backbone_uri.is_none());
}

#[test]
fn task_manager_leaf_body_and_command_report_the_unattached_runtime_truthfully() {
    let mut shell = super::panel_anchor_model_tests::host_test_shell();
    let fixture = fixture();
    let dock = shell.default_dock();
    let roots: Vec<&str> = dock.tabs(PanelAnchor::BottomRight).iter().map(|tab| tab.id.as_str()).collect();
    assert!(roots.contains(&FRAMEWORK_TASK_MANAGER_PANEL_ID));
    let body = serde_json::to_value(shell.build_task_manager_ui()).expect("task manager body");
    let unavailable = find_id(&body, "os.task-manager.no-runtime").expect("truthful no-runtime row").to_string();
    assert!(unavailable.contains(fixture["taskManager"]["message"]["en"].as_str().expect("message")));
    for action in ["suspend", "resume", "cancel"] {
        assert!(!body.to_string().contains(&format!("\"action\":\"{action}\"")), "no actor action without an actor runtime");
    }
    assert!(shell.build_os_commands().iter().any(|command| command.id == fixture["taskManager"]["commandId"].as_str().expect("command id")));
    shell.sync_dock_tabs();
    semio_framework_async::block_on(shell.apply_os_command("os.openTaskManager", None)).expect("open task manager");
    let state = shell.anchor_state(PanelAnchor::BottomRight);
    assert!(state.visible);
    assert_eq!(state.active_tab(), Some(FRAMEWORK_TASK_MANAGER_PANEL_ID));
}

#[test]
fn shell_owned_retained_document_closure_includes_sync_and_task_manager() {
    let shell = super::panel_anchor_model_tests::host_test_shell();
    let leaves = shell.shell_owned_panel_leaves();
    assert!(leaves.iter().any(|leaf| leaf == FRAMEWORK_SYNC_PANEL_TAB_ID));
    assert!(leaves.iter().any(|leaf| leaf == FRAMEWORK_TASK_MANAGER_PANEL_ID));
    for (id, body) in [
        (FRAMEWORK_SYNC_PANEL_TAB_ID, shell.build_sync_attach_ui()),
        (FRAMEWORK_TASK_MANAGER_PANEL_ID, shell.build_task_manager_ui()),
    ] {
        let records = panel_ui_records(id, &body).unwrap_or_else(|error| panic!("{id} retained projection: {error}"));
        assert!(!records.is_empty(), "{id} retained document");
    }
}
