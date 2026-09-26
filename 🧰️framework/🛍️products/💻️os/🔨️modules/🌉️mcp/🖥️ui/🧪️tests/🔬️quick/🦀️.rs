
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
fn ui_capabilities_expose_the_same_five_tool_names_with_the_right_kinds() {
    let capabilities = ui_capabilities();
    assert_eq!(capabilities.len(), 5);
    let expectations = [
        ("ui.focus", "ui_focus", CapabilityKind::Ui),
        ("ui.reveal", "ui_reveal", CapabilityKind::Ui),
        ("conversation.reply", "conversation_reply", CapabilityKind::Ui),
        ("job.get", "job_get", CapabilityKind::Job),
        ("job.cancel", "job_cancel", CapabilityKind::Job),
    ];
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
    assert_eq!(ui_resources(None).len(), 4);
    assert_eq!(ui_resources(Some(&filled_slot())).len(), 4);
    for resources in [ui_resources(None), ui_resources(Some(&filled_slot()))] {
        assert!(resources.iter().any(|resource| resource.uri == "semio://ui/agent-messages"), "the agent's own inbox is listed at every tier, like every other UI resource");
    }
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

/// 🔁️ Both reply paths in ONE test, driven by a stand-in shell that answers the `seq` the command
/// ACTUALLY carried — read off this connection's own outbox — instead of predicting it from the
/// process-global `SHELL_COMMAND_SEQ`. Any other `#[test]` in this binary that dispatches a shell
/// command bumps that counter concurrently, so a predicted seq is only ever right by luck and the
/// unlucky run times out with `PLUGIN_UNAVAILABLE` instead of asserting what it meant to.
#[test]
fn dispatch_shell_command_resolves_a_matching_ok_reply_and_surfaces_a_shell_fault() {
    let bridge = Arc::new(BridgeHandle::new());
    let (connection_id, mut outbox) = bridge.register();

    let reply_bridge = bridge.clone();
    let replier = std::thread::spawn(move || {
        for (ok, fault) in [(true, None), (false, Some("unknown window".to_string()))] {
            let deadline = Instant::now() + Duration::from_millis(2_000);
            let seq = loop {
                match outbox.try_recv() {
                    Some(GatewayToShell::ShellCommand { seq, .. }) => break seq,
                    Some(other) => panic!("the gateway queued {other:?} instead of a ShellCommand"),
                    None => {
                        assert!(Instant::now() < deadline, "no ShellCommand was queued on the outbox");
                        std::thread::sleep(Duration::from_millis(1));
                    }
                }
            };
            reply_bridge.record(connection_id, ShellToGateway::ShellCommandResult { in_reply_to: seq, ok, fault });
        }
    });

    let result = dispatch_shell_command_with_timeout(&bridge, serde_json::json!({ "type": "focusWindow", "windowId": null }), Duration::from_millis(2_000));
    assert!(result.is_ok(), "{result:?}");

    let result = dispatch_shell_command_with_timeout(&bridge, serde_json::json!({ "type": "focusWindow", "windowId": "nope" }), Duration::from_millis(2_000));
    replier.join().expect("reply thread");
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

/// 🛑️ Replays `🧫️fixtures/🛑️job-cancel-hook-law.json`: the hook `inference_run` binds to reach a
/// guest blocked in a solve fires once per cancel, late binds fire immediately, terminal jobs never.
#[test]
fn a_bound_cancel_hook_obeys_the_language_agnostic_law() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🛑️job-cancel-hook-law.json")).expect("law fixture is JSON");
    for case in fixture["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("name");
        let registry = JobRegistry::new();
        let fired = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let mut job_id = String::new();
        let mut cancel_errors = Vec::new();
        for step in case["steps"].as_array().expect("steps") {
            match step.as_str().expect("step") {
                "begin" => job_id = registry.begin("inference.law"),
                "progress" => assert!(registry.report_progress(&job_id, 0.5, None), "{name}"),
                "succeed" => assert!(registry.succeed(&job_id, serde_json::Value::Null), "{name}"),
                "bind" => {
                    let fired = std::sync::Arc::clone(&fired);
                    registry.bind_cancel(&job_id, move || {
                        fired.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    });
                }
                "cancel" => {
                    if let Err(error) = registry.request_cancel(&job_id) {
                        cancel_errors.push(serde_json::to_value(error.code).expect("code"));
                    }
                }
                other => panic!("{name}: unknown step {other}"),
            }
        }
        assert_eq!(fired.load(std::sync::atomic::Ordering::SeqCst) as u64, case["fired"].as_u64().expect("fired"), "{name}: hook firings");
        assert_eq!(serde_json::to_value(registry.snapshot(&job_id).expect("job").status).expect("status"), case["status"], "{name}: status");
        assert_eq!(serde_json::Value::Array(cancel_errors), case["cancelErrors"], "{name}: cancel errors");
    }
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

//#region 💬️AgentConversation
/// 💬️ The receiving half of the in-shell agent panel: a human turn recorded on a live `/bridge`
/// connection is readable ONCE through `semio://ui/agent-messages`, then gone — MCP has no push, so
/// the agent polls this uri and must never be handed the same turn twice.
#[test]
fn agent_messages_resource_drains_the_shells_turns_exactly_once() {
    let slot: BridgeSlot = Arc::new(OnceLock::new());
    let handle = Arc::new(BridgeHandle::new());
    assert!(slot.set(handle.clone()).is_ok());
    let (connection, _outbox) = handle.register();
    handle.record(connection, ShellToGateway::AgentMessage { message_id: "msg_1".into(), text: "translate the selection".into() });
    handle.record(connection, ShellToGateway::AgentMessage { message_id: "msg_2".into(), text: "now undo it".into() });
    assert_eq!(handle.pending_agent_message_count(connection), 2);

    let contents = read_ui_resource("semio://ui/agent-messages", Some(&slot), None).expect("ours").expect("a live shell is attached");
    let body: serde_json::Value = serde_json::from_str(contents[0].text.as_deref().expect("a json body")).expect("valid json");
    let messages = body["messages"].as_array().expect("an array");
    assert_eq!(messages.len(), 2);
    assert_eq!(messages[0]["messageId"], "msg_1", "oldest first");
    assert_eq!(messages[1]["text"], "now undo it");

    let again = read_ui_resource("semio://ui/agent-messages", Some(&slot), None).expect("ours").expect("still attached");
    let body: serde_json::Value = serde_json::from_str(again[0].text.as_deref().expect("a json body")).expect("valid json");
    assert!(body["messages"].as_array().expect("an array").is_empty(), "a drained turn is never replayed");
}

/// 🕳️ Bare and headless tiers answer the SAME typed, retryable gap every other UI resource answers —
/// never an empty list that a caller could mistake for "the human said nothing".
#[test]
fn agent_messages_resource_is_retryable_plugin_unavailable_without_a_shell() {
    for slot in [None, Some(filled_slot())] {
        let error = read_ui_resource("semio://ui/agent-messages", slot.as_ref(), None).expect("ours").expect_err("no shell attached");
        assert_eq!(error.code, GatewayErrorCode::PluginUnavailable);
        assert!(error.retryable);
    }
}

/// 💬️ The emitting half: one tool call becomes a `AgentToolCall` + busy `AgentPresence` before the
/// handler runs, and a `AgentToolResult` + idle `AgentPresence` after — in that exact order, on the
/// live connection, with the arguments the agent really sent.
///
/// 📬️ `broadcast` admits onto the bridge's own worker pool, so the frames land asynchronously —
/// awaited (never polled in a spin loop) with a real deadline so a regression fails loudly instead
/// of hanging the suite.
#[tokio::test]
async fn agent_conversation_publishes_the_real_tool_call_and_its_result() {
    let slot: BridgeSlot = Arc::new(OnceLock::new());
    let handle = Arc::new(BridgeHandle::new());
    assert!(slot.set(handle.clone()).is_ok());
    let (_connection, mut outbox) = handle.register();
    let conversation = crate::bridge::AgentConversation::new(slot, "claude-code");

    let invocation = conversation.begin_tool_call("action_invoke", &serde_json::json!({ "capabilityId": "cad.viewport.translateSelection" }));
    conversation.finish_tool_call(&invocation, "action_invoke", true, "moved 1 object");

    let mut frames = Vec::new();
    for _ in 0..4 {
        let frame = tokio::time::timeout(Duration::from_secs(5), outbox.recv()).await.expect("the bridge delivers every admitted conversation frame").expect("the outbox stays open");
        frames.push(frame);
    }
    assert_eq!(
        frames,
        vec![
            GatewayToShell::AgentToolCall { invocation_id: invocation.clone(), tool_name: "action_invoke".into(), arguments: "{\"capabilityId\":\"cad.viewport.translateSelection\"}".into() },
            GatewayToShell::AgentPresence { active: true, label: "claude-code".into(), invocation_id: Some(invocation.clone()) },
            GatewayToShell::AgentToolResult { invocation_id: invocation.clone(), tool_name: "action_invoke".into(), ok: true, summary: "moved 1 object".into() },
            GatewayToShell::AgentPresence { active: false, label: "claude-code".into(), invocation_id: None },
        ]
    );
}

/// 🛑️ Ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice U1 (audit ranked item 2): the shell's
/// cancel control reaches the REAL cooperative cancellation, not a second mechanism. Every tool call
/// is a job in the one process-wide registry keyed by the invocation id the panel renders, an inbound
/// `ShellToGateway::AgentCancel` flips that job's flag, and the call then settles `Cancelled` rather
/// than as whatever the handler happened to return.
#[test]
fn an_agent_cancel_frame_flips_the_tool_calls_own_job_and_settles_it_cancelled() {
    let slot: BridgeSlot = Arc::new(OnceLock::new());
    let handle = Arc::new(BridgeHandle::new());
    assert!(slot.set(handle.clone()).is_ok());
    let (connection, _outbox) = handle.register();
    let conversation = crate::bridge::AgentConversation::new(slot, "claude-code");

    let invocation = conversation.begin_tool_call("inference_run", &serde_json::json!({ "artifactKind": "cad" }));
    let opened = crate::ui::job_registry().snapshot(&invocation).expect("a tool call is a real job keyed by its invocation id");
    assert_eq!(opened.kind, "toolCall");
    assert!(!opened.cancel_requested, "nothing has asked it to stop yet");
    assert!(!crate::ui::job_registry().is_cancel_requested(&invocation));

    handle.record(connection, crate::bridge::ShellToGateway::AgentCancel { invocation_id: invocation.clone() });
    assert!(crate::ui::job_registry().is_cancel_requested(&invocation), "the frame must reach the same flag `job_cancel` flips");

    conversation.finish_tool_call(&invocation, "inference_run", true, "done");
    let settled = crate::ui::job_registry().snapshot(&invocation).expect("the job outlives the call");
    assert_eq!(settled.status, crate::ui::JobStatus::Cancelled, "a cancelled call settles cancelled, not succeeded");
}

/// 🛑️ A cancel that arrives after its call already finished is the honest outcome of a late click,
/// never an error the gateway reports or a panic — and it must not resurrect a settled job.
#[test]
fn a_late_agent_cancel_frame_is_absorbed_without_disturbing_the_settled_job() {
    let slot: BridgeSlot = Arc::new(OnceLock::new());
    let handle = Arc::new(BridgeHandle::new());
    assert!(slot.set(handle.clone()).is_ok());
    let (connection, _outbox) = handle.register();
    let conversation = crate::bridge::AgentConversation::new(slot, "claude-code");

    let invocation = conversation.begin_tool_call("action_invoke", &serde_json::Value::Null);
    conversation.finish_tool_call(&invocation, "action_invoke", true, "moved 1 object");
    assert_eq!(crate::ui::job_registry().snapshot(&invocation).expect("job").status, crate::ui::JobStatus::Succeeded);

    handle.record(connection, crate::bridge::ShellToGateway::AgentCancel { invocation_id: invocation.clone() });
    let after = crate::ui::job_registry().snapshot(&invocation).expect("job");
    assert_eq!(after.status, crate::ui::JobStatus::Succeeded, "a late cancel must not rewrite a settled outcome");
    assert!(!after.cancel_requested);

    handle.record(connection, crate::bridge::ShellToGateway::AgentCancel { invocation_id: "inv_never_existed".into() });
}

/// ✂️ An agent calling a tool with an enormous argument blob must not be able to starve the bounded
/// bridge outbox: the rendered text is cut on a char boundary and marked, never sent whole.
#[test]
fn conversation_text_is_truncated_on_a_char_boundary() {
    let short = "ä".repeat(4);
    assert_eq!(crate::bridge::truncate_conversation_text(&short), short, "anything inside the cap is untouched");
    let long = "ä".repeat(crate::bridge::AGENT_CONVERSATION_MAX_TEXT);
    let truncated = crate::bridge::truncate_conversation_text(&long);
    assert!(truncated.len() <= crate::bridge::AGENT_CONVERSATION_MAX_TEXT + '…'.len_utf8());
    assert!(truncated.ends_with('…'));
    assert!(std::str::from_utf8(truncated.as_bytes()).is_ok(), "a cut inside a multi-byte char would not be utf-8");
}
//#endregion 💬️AgentConversation

//#region 💬️ConversationReply
/// ⚖️ AC1's local laws for `conversation_reply` — the frame the agent's own words travel on.
/// `PolicyEngine`/`AgentPrincipal` are built here rather than through a whole `ActionAdapter`,
/// because the only thing the handler asks the engine is the scope subset check.
fn reply_policy() -> crate::policy::PolicyEngine {
    crate::policy::PolicyEngine::new(Arc::new(crate::handles::HandleTable::new()), crate::policy::AutoApprovePolicy::Never)
}

fn reply_principal(scopes: &[&str]) -> crate::policy::AgentPrincipal {
    crate::policy::AgentPrincipal::from_scope_names("agent:test", "claude-code", &scopes.iter().map(|scope| scope.to_string()).collect::<Vec<_>>(), None)
}

/// 🔐️ `conversation.write` is the scope that decides it, and nothing else is a substitute: an agent
/// granted every window-moving grant `ui.control` expands to still may not put text in front of a
/// human, and the refusal is the typed `PERMISSION_DENIED` naming the missing scope.
#[test]
fn conversation_reply_is_refused_without_the_conversation_write_scope() {
    let (policy, slot) = (reply_policy(), filled_slot());
    let bridge = slot.get().cloned().expect("filled");
    let refused = conversation_reply_handler(&policy, &reply_principal(&["ui.control", "artifact.write"]), Some(&bridge), serde_json::json!({ "text": "hello" }));
    assert!(refused.is_error, "an unscoped principal must not reach the human's screen");
    let structured = refused.structured_content.as_ref().expect("a typed refusal");
    assert_eq!(structured["code"], "PERMISSION_DENIED");
    assert!(structured["message"].as_str().unwrap_or_default().contains("shell.converse"), "{structured}");

    let granted = conversation_reply_handler(&policy, &reply_principal(&["conversation.write"]), Some(&bridge), serde_json::json!({ "text": "hello" }));
    assert!(!granted.is_error, "the granted principal is admitted: {:?}", granted.structured_content);
}

/// 📭️ No shell attached is a tier, not a failure — but it is never a silent success either: with no
/// bridge at all the call is the same typed `PLUGIN_UNAVAILABLE` every other shell-dependent tool
/// answers, and with a bridge nobody dialed it answers honestly that it reached `0` shells.
#[test]
fn conversation_reply_reports_how_many_shells_it_reached() {
    let policy = reply_policy();
    let principal = reply_principal(&["conversation.write"]);
    let unbound = conversation_reply_handler(&policy, &principal, None, serde_json::json!({ "text": "hello" }));
    assert!(unbound.is_error);
    assert_eq!(unbound.structured_content.as_ref().expect("typed")["code"], "PLUGIN_UNAVAILABLE");

    let slot = filled_slot();
    let bridge = slot.get().cloned().expect("filled");
    let empty_room = conversation_reply_handler(&policy, &principal, Some(&bridge), serde_json::json!({ "text": "hello" }));
    assert!(!empty_room.is_error);
    assert_eq!(empty_room.structured_content.as_ref().expect("typed")["shells"], 0);
}

/// 🚫️ Every input the schema admits is checked, and an empty turn is refused rather than published:
/// a blank row in the transcript tells the human less than nothing.
#[test]
fn conversation_reply_refuses_malformed_input() {
    let (policy, slot) = (reply_policy(), filled_slot());
    let bridge = slot.get().cloned().expect("filled");
    let principal = reply_principal(&["conversation.write"]);
    for arguments in [
        serde_json::json!({}),
        serde_json::json!({ "text": "" }),
        serde_json::json!({ "text": "hi", "replyId": 7 }),
        serde_json::json!({ "text": "hi", "replyId": "" }),
        serde_json::json!({ "text": "hi", "inReplyTo": 7 }),
        serde_json::json!({ "text": "hi", "complete": "yes" }),
    ] {
        let refused = conversation_reply_handler(&policy, &principal, Some(&bridge), arguments.clone());
        assert!(refused.is_error, "{arguments} must be refused");
        assert_eq!(refused.structured_content.as_ref().expect("typed")["code"], "INPUT_INVALID", "{arguments}");
    }
}

/// 💬️ The streaming contract, on a real connection: two chunks of ONE turn carry the same
/// `reply_id`, only the last one is `complete`, and `in_reply_to` correlates the turn to the human
/// message that asked for it. A turn longer than the bridge's bounded text cap is cut, never
/// dropped.
#[tokio::test]
async fn conversation_reply_streams_chunks_of_one_turn_over_the_real_bridge() {
    let policy = reply_policy();
    let principal = reply_principal(&["conversation.write"]);
    let slot: BridgeSlot = Arc::new(OnceLock::new());
    let handle = Arc::new(BridgeHandle::new());
    assert!(slot.set(handle.clone()).is_ok());
    let (_connection, mut outbox) = handle.register();

    let first = conversation_reply_handler(&policy, &principal, Some(&handle), serde_json::json!({ "text": "Widening that wall means", "replyId": "rep_stream", "inReplyTo": "msg_1", "complete": false }));
    assert!(!first.is_error);
    assert_eq!(first.structured_content.as_ref().expect("typed")["shells"], 1);
    let second = conversation_reply_handler(&policy, &principal, Some(&handle), serde_json::json!({ "text": " the 300 mm variant.", "replyId": "rep_stream" }));
    assert!(!second.is_error);

    let mut frames = Vec::new();
    for _ in 0..2 {
        frames.push(tokio::time::timeout(Duration::from_secs(5), outbox.recv()).await.expect("the bridge delivers every admitted reply frame").expect("the outbox stays open"));
    }
    assert_eq!(
        frames,
        vec![
            GatewayToShell::AgentReply { reply_id: "rep_stream".into(), in_reply_to: Some("msg_1".into()), text: "Widening that wall means".into(), complete: false },
            GatewayToShell::AgentReply { reply_id: "rep_stream".into(), in_reply_to: None, text: " the 300 mm variant.".into(), complete: true },
        ]
    );

    let huge = "ä".repeat(crate::bridge::AGENT_CONVERSATION_MAX_TEXT);
    assert!(!conversation_reply_handler(&policy, &principal, Some(&handle), serde_json::json!({ "text": huge })).is_error);
    let cut = tokio::time::timeout(Duration::from_secs(5), outbox.recv()).await.expect("delivered").expect("open");
    let GatewayToShell::AgentReply { text, reply_id, .. } = cut else { panic!("the third frame is a reply") };
    assert!(text.ends_with('…'), "an oversized turn is cut and marked, never dropped");
    assert!(reply_id.starts_with("rep_"), "a client that mints no id gets one: {reply_id}");
}

/// 💬️ A `conversation_reply` call must NOT also appear as a tool-call row: the frame it publishes
/// is already the row, and `SELF_PUBLISHING_CONVERSATION_TOOLS` is the declared list that says so.
#[test]
fn conversation_reply_is_declared_self_publishing() {
    assert!(crate::bridge::SELF_PUBLISHING_CONVERSATION_TOOLS.contains(&"conversation_reply"));
    assert!(!crate::bridge::SELF_PUBLISHING_CONVERSATION_TOOLS.contains(&"action_invoke"));
}
//#endregion 💬️ConversationReply

//#region 💬️AgentMessagePush
/// 🔔️ The reverse direction: a turn the HUMAN types reaches the agent as a push, not only as a
/// poll. The inbox already existed (read-once through `semio://ui/agent-messages`); AC1 adds the
/// `notifications/resources/updated` that tells a subscribed client the question was asked, so the
/// channel `conversation_reply` answers on has no poll-interval latency floor of its own.
#[test]
fn a_typed_human_turn_pushes_a_resource_update_for_the_agent_inbox() {
    let handle = Arc::new(BridgeHandle::new());
    let (connection, _outbox) = handle.register();
    let subscriptions = crate::notify::ResourceSubscriptions::registered();
    let sink: crate::notify::NotificationSlot = crate::notify::notification_slot();
    let recorder = Arc::new(crate::notify::RecordingSink::new());
    assert!(sink.set(recorder.clone() as Arc<dyn crate::notify::NotificationSink>).is_ok());
    subscriptions.bind_sink(sink);
    assert!(subscriptions.subscribe("semio://ui/agent-messages"));

    handle.record(connection, crate::bridge::ShellToGateway::AgentMessage { message_id: "msg_push".into(), text: "widen that wall to 300".into() });

    let pushed = recorder.taken();
    assert!(
        pushed.iter().any(|notification| notification.method == "notifications/resources/updated" && notification.params.as_ref().map(|params| params["uri"] == "semio://ui/agent-messages").unwrap_or(false)),
        "a subscribed client is told at once: {pushed:?}"
    );
    assert_eq!(handle.pending_agent_message_count(connection), 1, "and the turn is still there to be read");
}
//#endregion 💬️AgentMessagePush
