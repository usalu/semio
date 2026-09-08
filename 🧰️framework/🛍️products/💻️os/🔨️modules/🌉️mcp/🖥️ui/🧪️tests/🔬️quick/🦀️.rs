
use super::*;
use crate::protocol::ToolRegistry;

fn full_registry() -> InMemoryToolRegistry {
    let mut registry = InMemoryToolRegistry::new();
    register_ui_tools(&mut registry, None, None);
    registry
}

/// 🔌️ A `BridgeSlot` already filled with a live handle — the "bridge exists, no shell attached"
/// tier, which every `Option<BridgeSlot>` entry point now takes instead of a bare handle.
fn filled_slot() -> BridgeSlot {
    let slot: BridgeSlot = Arc::new(OnceLock::new());
    assert!(slot.set(Arc::new(BridgeHandle::new())).is_ok(), "a fresh slot is empty");
    slot
}

#[test]
fn all_four_tools_register_under_valid_mcp_names_with_object_top_level_schemas() {
    let registry = full_registry();
    let tools = registry.list();
    let names = ["ui_focus", "ui_reveal", "job_get", "job_cancel"];
    for name in names {
        let tool = tools.iter().find(|tool| tool.name == name).unwrap_or_else(|| panic!("missing tool {name}"));
        assert!(crate::protocol::is_valid_tool_name(&tool.name));
        assert_eq!(tool.input_schema["type"], "object", "{name} input_schema");
        let output_schema = tool.output_schema.as_ref().unwrap_or_else(|| panic!("{name} has no output_schema"));
        assert_eq!(output_schema["type"], "object", "{name} output_schema");
    }
    assert_eq!(tools.len(), names.len());
}

#[test]
fn ui_capabilities_expose_the_same_four_tool_names_with_the_right_kinds() {
    let capabilities = ui_capabilities();
    assert_eq!(capabilities.len(), 4);
    let expectations = [("ui.focus", "ui_focus", CapabilityKind::Ui), ("ui.reveal", "ui_reveal", CapabilityKind::Ui), ("job.get", "job_get", CapabilityKind::Job), ("job.cancel", "job_cancel", CapabilityKind::Job)];
    for (id, tool_name, kind) in expectations {
        let capability = capabilities.iter().find(|capability| capability.id.as_str() == id).unwrap_or_else(|| panic!("missing capability {id}"));
        assert_eq!(capability.kind, kind);
        assert_eq!(capability.exposure, ToolExposure::Direct { tool_name: tool_name.to_string() });
        assert_eq!(capability.input_schema["type"], "object");
        assert_eq!(capability.output_schema["type"], "object");
    }
}

#[test]
fn ui_focus_with_no_bridge_is_a_retryable_plugin_unavailable_not_a_panic_or_protocol_failure() {
    let registry = full_registry();
    let result = registry.call("ui_focus", serde_json::json!({ "windowId": "w1" })).expect("known tool name resolves");
    assert!(result.is_error);
    let structured = result.structured_content.expect("structured content");
    assert_eq!(structured["code"], "PLUGIN_UNAVAILABLE");
    assert_eq!(structured["retryable"], true);
}

#[test]
fn ui_reveal_with_no_bridge_is_a_retryable_plugin_unavailable() {
    let registry = full_registry();
    let result = registry.call("ui_reveal", serde_json::json!({ "anchor": "left", "path": ["a"] })).expect("known tool name resolves");
    assert!(result.is_error);
    let structured = result.structured_content.expect("structured content");
    assert_eq!(structured["code"], "PLUGIN_UNAVAILABLE");
    assert_eq!(structured["retryable"], true);
}

#[test]
fn ui_focus_with_a_bridge_but_no_shell_attached_is_a_normal_retryable_state() {
    let mut registry = InMemoryToolRegistry::new();
    register_ui_tools(&mut registry, Some(filled_slot()), None);
    let result = registry.call("ui_focus", serde_json::json!({ "windowId": "w1" })).expect("known tool name resolves");
    assert!(result.is_error);
    let structured = result.structured_content.expect("structured content");
    assert_eq!(structured["code"], "PLUGIN_UNAVAILABLE");
    assert_eq!(structured["retryable"], true);
    assert!(structured["message"].as_str().unwrap().contains("no shell is attached"));
}

#[test]
fn ui_reveal_rejects_a_bad_anchor_as_input_invalid() {
    let registry = full_registry();
    let result = registry.call("ui_reveal", serde_json::json!({ "anchor": "middle", "path": [] })).expect("known tool name resolves");
    assert!(result.is_error);
    assert_eq!(result.structured_content.unwrap()["code"], "INPUT_INVALID");
}

#[test]
fn ui_resources_and_templates_never_depend_on_bridge_presence() {
    assert_eq!(ui_resources(None).len(), 3);
    assert_eq!(ui_resources(Some(&filled_slot())).len(), 3);
    let templates = ui_resource_templates();
    assert!(templates.iter().any(|template| template.uri_template == "semio://window/{windowId}"));
    assert!(templates.iter().any(|template| template.uri_template == "semio://job/{jobId}"));
}

#[test]
fn read_ui_resource_returns_none_for_a_non_ui_uri() {
    assert!(read_ui_resource("semio://not-a-resource", None, None).is_none());
    assert!(read_ui_resource("semio://capability", None, None).is_none());
}

#[test]
fn read_ui_resource_for_window_degrades_to_a_typed_error_without_a_shell() {
    let outcome = read_ui_resource("semio://window", None, None).expect("ours");
    let error = outcome.expect_err("no bridge means no window data");
    assert_eq!(error.code, GatewayErrorCode::PluginUnavailable);
    assert!(error.retryable);
}

#[test]
fn read_ui_resource_for_active_context_and_selection_also_degrade_cleanly() {
    assert!(read_ui_resource("semio://ui/active-context", None, None).expect("ours").is_err());
    assert!(read_ui_resource("semio://ui/selection", None, None).expect("ours").is_err());
}

#[test]
fn dispatch_shell_command_times_out_when_the_shell_never_replies() {
    let bridge = BridgeHandle::new();
    let (_connection_id, _outbox) = bridge.register();
    let result = dispatch_shell_command_with_timeout(&bridge, serde_json::json!({ "type": "focusWindow", "windowId": null }), Duration::from_millis(60));
    let error = result.expect_err("no reply was ever recorded");
    assert_eq!(error.code, GatewayErrorCode::PluginUnavailable);
    assert!(error.retryable);
}

#[test]
fn dispatch_shell_command_succeeds_once_the_matching_reply_arrives() {
    let bridge = Arc::new(BridgeHandle::new());
    let (connection_id, _outbox) = bridge.register();
    let expected_seq = SHELL_COMMAND_SEQ.load(Ordering::SeqCst);
    let reply_bridge = bridge.clone();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(10));
        reply_bridge.record(connection_id, ShellToGateway::ShellCommandResult { in_reply_to: expected_seq, ok: true, fault: None });
    });
    let result = dispatch_shell_command_with_timeout(&bridge, serde_json::json!({ "type": "focusWindow", "windowId": null }), Duration::from_millis(2_000));
    assert!(result.is_ok(), "{result:?}");
}

#[test]
fn dispatch_shell_command_surfaces_a_shell_fault_as_side_effect_rejected() {
    let bridge = Arc::new(BridgeHandle::new());
    let (connection_id, _outbox) = bridge.register();
    let expected_seq = SHELL_COMMAND_SEQ.load(Ordering::SeqCst);
    let reply_bridge = bridge.clone();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(10));
        reply_bridge.record(connection_id, ShellToGateway::ShellCommandResult { in_reply_to: expected_seq, ok: false, fault: Some("unknown window".to_string()) });
    });
    let result = dispatch_shell_command_with_timeout(&bridge, serde_json::json!({ "type": "focusWindow", "windowId": "nope" }), Duration::from_millis(2_000));
    let error = result.expect_err("the shell rejected the command");
    assert_eq!(error.code, GatewayErrorCode::SideEffectRejected);
}

#[test]
fn job_registry_round_trips_pending_running_succeeded() {
    let registry = JobRegistry::new();
    let job_id = registry.begin("inference.demo");
    let pending = registry.snapshot(&job_id).expect("just began");
    assert_eq!(pending.status, JobStatus::Pending);
    assert!(registry.report_progress(&job_id, 0.5, Some("halfway".to_string())));
    let running = registry.snapshot(&job_id).expect("still tracked");
    assert_eq!(running.status, JobStatus::Running);
    assert_eq!(running.progress, Some(0.5));
    assert!(registry.succeed(&job_id, serde_json::json!({ "answer": 42 })));
    let done = registry.snapshot(&job_id).expect("still tracked");
    assert_eq!(done.status, JobStatus::Succeeded);
    assert_eq!(done.progress, Some(1.0));
    assert!(!registry.report_progress(&job_id, 0.1, None), "a terminal job never regresses");
}

#[test]
fn begin_with_id_lets_a_producer_reuse_an_id_it_minted_elsewhere() {
    let registry = JobRegistry::new();
    let returned = registry.begin_with_id("job_from_elsewhere", "inference.demo");
    assert_eq!(returned, "job_from_elsewhere");
    assert_eq!(registry.snapshot("job_from_elsewhere").unwrap().status, JobStatus::Pending);
}

#[test]
fn cancelling_a_pending_job_finishes_it_immediately() {
    let registry = JobRegistry::new();
    let job_id = registry.begin("inference.demo");
    let snapshot = registry.request_cancel(&job_id).expect("pending job is cancellable");
    assert_eq!(snapshot.status, JobStatus::Cancelled);
}

#[test]
fn cancelling_a_running_job_only_sets_the_cooperative_flag_until_the_producer_acknowledges() {
    let registry = JobRegistry::new();
    let job_id = registry.begin("inference.demo");
    registry.report_progress(&job_id, 0.1, None);
    let snapshot = registry.request_cancel(&job_id).expect("running job is cancellable");
    assert_eq!(snapshot.status, JobStatus::Running);
    assert!(registry.is_cancel_requested(&job_id));
    assert!(registry.mark_cancelled(&job_id));
    assert_eq!(registry.snapshot(&job_id).unwrap().status, JobStatus::Cancelled);
}

#[test]
fn cancelling_an_unknown_or_already_terminal_job_is_a_typed_error_not_a_silent_no_op() {
    let registry = JobRegistry::new();
    assert_eq!(registry.request_cancel("job_missing").unwrap_err().code, GatewayErrorCode::NotFound);
    let job_id = registry.begin("inference.demo");
    registry.succeed(&job_id, serde_json::Value::Null);
    assert_eq!(registry.request_cancel(&job_id).unwrap_err().code, GatewayErrorCode::PreconditionFailed);
}

#[test]
fn job_get_tool_reports_not_found_for_an_unknown_id() {
    let registry = full_registry();
    let result = registry.call("job_get", serde_json::json!({ "jobId": "job_missing_entirely" })).expect("known tool name resolves");
    assert!(result.is_error);
    assert_eq!(result.structured_content.unwrap()["code"], "NOT_FOUND");
}

#[test]
fn job_get_and_job_cancel_tools_round_trip_through_the_shared_registry() {
    let registry = full_registry();
    let job_id = job_registry().begin("ui.quick.test");
    let got = registry.call("job_get", serde_json::json!({ "jobId": job_id })).expect("known tool name resolves");
    assert!(!got.is_error, "{got:?}");
    assert_eq!(got.structured_content.as_ref().unwrap()["status"], "PENDING");
    let cancelled = registry.call("job_cancel", serde_json::json!({ "jobId": job_id })).expect("known tool name resolves");
    assert!(!cancelled.is_error, "{cancelled:?}");
    assert_eq!(cancelled.structured_content.unwrap()["status"], "CANCELLED");
}

#[test]
fn read_ui_resource_for_a_known_job_id_reflects_the_shared_registry() {
    let job_id = job_registry().begin("ui.quick.test.resource");
    let outcome = read_ui_resource(&format!("semio://job/{job_id}"), None, None).expect("ours").expect("job exists");
    assert_eq!(outcome[0].uri, format!("semio://job/{job_id}"));
    assert!(outcome[0].text.as_ref().unwrap().contains("PENDING"));
}
