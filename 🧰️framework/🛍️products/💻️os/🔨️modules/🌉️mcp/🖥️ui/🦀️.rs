//! 🖥️ UI information, UI actions, and jobs — ticket 26/08/29/AI-MCP-END-TO-END packet
//! W6-ui-and-jobs. `ui_focus`/`ui_reveal` forward a `ShellCommand` (JSON-encoded, matching
//! `💻️os/🔨️modules/🖥️shell`'s own wire vocabulary) to the attached `/bridge` shell as a
//! `GatewayToShell::ShellCommand` frame and block on its `ShellCommandResult` (bounded by
//! [`SHELL_COMMAND_TIMEOUT_MS`], never forever). `semio://window[/…]`/`semio://ui/active-context`/
//! `semio://ui/selection` project the SAME `ShellState` mirror `🧵️bridge::BridgeHandle::last_shell_state`
//! already holds — never a second, invented source of truth. `job_get`/`job_cancel` read/act through
//! [`job_registry`], a plugin-agnostic, process-wide job tracker any producer (W5's inference, a
//! future tessellation job, …) mints ids into via [`JobRegistry::begin`] and reports progress/result
//! into via [`JobRegistry::report_progress`]/[`JobRegistry::succeed`]/[`JobRegistry::fail`] — this
//! facet never reaches into the plugin-host reactor's own `job: u64` ids directly (those are
//! per-guest-instance and require a live `GuestInstance` this crate's tool layer never holds); a
//! producer that spawns one is the bridge between the two id spaces. Every tool this file registers
//! stays PRESENT in `tools/list` and every resource stays LISTED regardless of tier — only a `call`/
//! `read` result varies (bare: no bridge at all; headless: a bridge exists but no shell has dialed
//! it yet, a normal, retryable state; attached: a live shell). Never a panic, never a protocol-level
//! failure, never fabricated UI state — see `read_ui_resource`'s doc for the one deliberate honesty
//! trade-off (`semio://ui/selection` omits per-artifact object selection, which `ShellState` does
//! not model).

use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use semio_framework_os_mcp_bridge::session::BridgeHandle;
use semio_framework_os_mcp_bridge::shell_command_dispatch::{BridgeSlot, active_shell_connection, bridge_not_running_error, dispatch_shell_command, no_shell_attached_error, resolve_bridge};
use semio_framework_os_mcp_bridge_codec::bridge::{GatewayToShell, ShellToGateway, truncate_conversation_text};
use semio_framework_os_mcp_capabilities::capability_declarations::{conversation_reply_capability, ui_focus_capability, ui_reveal_capability};
use semio_framework_os_mcp_dispatch::actions::ActionAdapter;
use semio_framework_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use semio_framework_os_mcp_port::policy::{AgentPrincipal, PolicyEngine, authorize_capability_scopes};
use semio_framework_os_mcp_schema::schema::{conversation_reply_input_schema, conversation_reply_output_schema, job_cancel_input_schema, job_get_input_schema, job_snapshot_output_schema, ui_focus_input_schema, ui_focus_output_schema, ui_reveal_input_schema, ui_reveal_output_schema};
use semio_framework_os_mcp_wire::job_registry::job_registry;
use semio_framework_os_mcp_wire::protocol::{CallToolResult, ContentBlock, InMemoryToolRegistry, Resource, ResourceContent, ResourceTemplate, Tool};
use semio_framework_os_mcp_workspace::headless_workspace_state::HeadlessWorkspace;

//#region 🔖️ToolHandlers

fn input_invalid(message: impl Into<String>) -> CallToolResult {
    CallToolResult::tool_error(&GatewayError::new(GatewayErrorCode::InputInvalid, message.into()))
}

fn ui_focus_handler(bridge: Option<&Arc<BridgeHandle>>, arguments: serde_json::Value) -> CallToolResult {
    let window_id = match arguments.get("windowId") {
        None => None,
        Some(serde_json::Value::String(window_id)) => Some(window_id.clone()),
        Some(_) => return input_invalid("windowId must be a string when present"),
    };
    let Some(bridge) = bridge else {
        return CallToolResult::tool_error(&bridge_not_running_error());
    };
    let command = serde_json::json!({ "type": "focusWindow", "windowId": window_id });
    match dispatch_shell_command(bridge, command) {
        Ok(()) => CallToolResult::ok(vec![ContentBlock::Text { text: format!("focused {window_id:?}") }], Some(serde_json::json!({ "ok": true, "windowId": window_id }))),
        Err(error) => CallToolResult::tool_error(&error),
    }
}

fn ui_reveal_handler(bridge: Option<&Arc<BridgeHandle>>, arguments: serde_json::Value) -> CallToolResult {
    let anchor = match arguments.get("anchor").and_then(serde_json::Value::as_str) {
        Some(anchor) if ["left", "right", "top", "bottom"].contains(&anchor) => anchor.to_string(),
        _ => return input_invalid("anchor must be one of left|right|top|bottom"),
    };
    let path: Vec<String> = match arguments.get("path").and_then(serde_json::Value::as_array) {
        Some(items) => match items.iter().map(|item| item.as_str().map(str::to_string)).collect::<Option<Vec<_>>>() {
            Some(path) => path,
            None => return input_invalid("path must be an array of strings"),
        },
        None => return input_invalid("path is required"),
    };
    let Some(bridge) = bridge else {
        return CallToolResult::tool_error(&bridge_not_running_error());
    };
    if let Err(error) = dispatch_shell_command(bridge, serde_json::json!({ "type": "setPanelVisible", "anchor": anchor, "visible": true })) {
        return CallToolResult::tool_error(&error);
    }
    match dispatch_shell_command(bridge, serde_json::json!({ "type": "setPanelPath", "anchor": anchor, "path": path })) {
        Ok(()) => CallToolResult::ok(vec![ContentBlock::Text { text: format!("revealed {path:?} in {anchor}") }], Some(serde_json::json!({ "ok": true, "anchor": anchor, "path": path }))),
        Err(error) => CallToolResult::tool_error(&error),
    }
}

fn job_get_handler(arguments: serde_json::Value) -> CallToolResult {
    let job_id = match arguments.get("jobId").and_then(serde_json::Value::as_str) {
        Some(job_id) => job_id.to_string(),
        None => return input_invalid("jobId is required"),
    };
    match job_registry().snapshot(&job_id) {
        Some(snapshot) => CallToolResult::ok(vec![ContentBlock::Text { text: format!("job {job_id} is {:?}", snapshot.status) }], Some(snapshot.to_json())),
        None => CallToolResult::tool_error(&GatewayError::new(GatewayErrorCode::NotFound, format!("no such job: {job_id}"))),
    }
}

fn job_cancel_handler(arguments: serde_json::Value) -> CallToolResult {
    let job_id = match arguments.get("jobId").and_then(serde_json::Value::as_str) {
        Some(job_id) => job_id.to_string(),
        None => return input_invalid("jobId is required"),
    };
    match job_registry().request_cancel(&job_id) {
        Ok(snapshot) => CallToolResult::ok(vec![ContentBlock::Text { text: format!("job {job_id} cancellation requested ({:?})", snapshot.status) }], Some(snapshot.to_json())),
        Err(error) => CallToolResult::tool_error(&error),
    }
}

/// 🖥️ Registers `ui_focus`, `ui_reveal`, `job_get`, `job_cancel` — always registered, always in
/// `tools/list`, regardless of tier (§ module doc). `_workspace` is accepted (not yet read) for
/// seam symmetry with `read_ui_resource`/a later packet that scopes jobs to one workspace; today's
/// job seam is the process-wide [`job_registry`], deliberately workspace-independent.
pub fn register_ui_tools(registry: &mut InMemoryToolRegistry, bridge: Option<BridgeSlot>, _workspace: Option<Arc<HeadlessWorkspace>>, principal: AgentPrincipal) {
    let mut ui_focus = Tool::new("ui_focus", ui_focus_input_schema());
    ui_focus.title = Some("Focus Window".to_string());
    ui_focus.description = Some("Focuses a window on the attached shell (omit windowId to clear focus).".to_string());
    ui_focus.output_schema = Some(ui_focus_output_schema());
    let (focus_bridge, focus_principal, focus_capability) = (bridge.clone(), principal.clone(), ui_focus_capability());
    registry
        .register(ui_focus, move |arguments| match authorize_capability_scopes(&focus_principal, &focus_capability) {
            Ok(()) => ui_focus_handler(resolve_bridge(focus_bridge.as_ref()), arguments),
            Err(error) => CallToolResult::tool_error(&error),
        })
        .expect("ui_focus is a valid tool name");

    let mut ui_reveal = Tool::new("ui_reveal", ui_reveal_input_schema());
    ui_reveal.title = Some("Reveal In Panel".to_string());
    ui_reveal.description = Some("Makes a panel visible and navigates it to the given path on the attached shell.".to_string());
    ui_reveal.output_schema = Some(ui_reveal_output_schema());
    let (reveal_bridge, reveal_principal, reveal_capability) = (bridge.clone(), principal, ui_reveal_capability());
    registry
        .register(ui_reveal, move |arguments| match authorize_capability_scopes(&reveal_principal, &reveal_capability) {
            Ok(()) => ui_reveal_handler(resolve_bridge(reveal_bridge.as_ref()), arguments),
            Err(error) => CallToolResult::tool_error(&error),
        })
        .expect("ui_reveal is a valid tool name");

    let mut job_get = Tool::new("job_get", job_get_input_schema());
    job_get.title = Some("Get Job".to_string());
    job_get.description = Some("Reads the status/progress/result of one plugin-agnostic job by id.".to_string());
    job_get.output_schema = Some(job_snapshot_output_schema("job.get"));
    registry.register(job_get, move |arguments| job_get_handler(arguments)).expect("job_get is a valid tool name");

    let mut job_cancel = Tool::new("job_cancel", job_cancel_input_schema());
    job_cancel.title = Some("Cancel Job".to_string());
    job_cancel.description = Some("Requests cooperative cancellation of one running or pending job by id.".to_string());
    job_cancel.output_schema = Some(job_snapshot_output_schema("job.cancel"));
    registry.register(job_cancel, move |arguments| job_cancel_handler(arguments)).expect("job_cancel is a valid tool name");
}

//#region 💬️ConversationReply

/// 🔢️ The gateway's own reply-turn counter, for a client that streams without minting ids of its
/// own. Process-wide like [`job_registry`], for the same reason: the shell keys rows by this id and
/// two connections must never hand it the same one.
static NEXT_REPLY_ID: AtomicU64 = AtomicU64::new(1);

/// 💬️ `conversation_reply` — publishes one chunk of the agent's free-text turn to every attached
/// shell. It answers `shells: 0` (never an error) when nobody is looking, because a gateway with no
/// shell attached is an ordinary tier, not a failure; it refuses only when the principal was never
/// granted `shell.converse`, when `text` is missing, or when no `/bridge` exists at all.
pub(crate) fn conversation_reply_handler(policy: &PolicyEngine, principal: &AgentPrincipal, bridge: Option<&Arc<BridgeHandle>>, arguments: serde_json::Value) -> CallToolResult {
    let capability = conversation_reply_capability();
    if let Err(error) = policy.authorize_scopes(principal, &capability) {
        return CallToolResult::tool_error(&error);
    }
    let Some(text) = arguments.get("text").and_then(serde_json::Value::as_str) else {
        return input_invalid("text is required");
    };
    if text.is_empty() {
        return input_invalid("text must not be empty — an empty turn tells the human nothing");
    }
    let reply_id = match arguments.get("replyId") {
        None => format!("rep_{}", NEXT_REPLY_ID.fetch_add(1, Ordering::Relaxed)),
        Some(serde_json::Value::String(reply_id)) if !reply_id.is_empty() => reply_id.clone(),
        Some(_) => return input_invalid("replyId must be a non-empty string when present"),
    };
    let in_reply_to = match arguments.get("inReplyTo") {
        None => None,
        Some(serde_json::Value::String(message_id)) => Some(message_id.clone()),
        Some(_) => return input_invalid("inReplyTo must be a string when present"),
    };
    let complete = match arguments.get("complete") {
        None => true,
        Some(serde_json::Value::Bool(complete)) => *complete,
        Some(_) => return input_invalid("complete must be a boolean when present"),
    };
    let Some(bridge) = bridge else {
        return CallToolResult::tool_error(&bridge_not_running_error());
    };
    let frame = GatewayToShell::AgentReply { reply_id: reply_id.clone(), in_reply_to, text: truncate_conversation_text(text), complete };
    let shells = match bridge.broadcast(frame) {
        Ok(shells) => shells,
        Err(_) => return CallToolResult::tool_error(&GatewayError::new(GatewayErrorCode::Internal, "the shell bridge refused this reply — it exceeds the bridge's per-broadcast admission budget").retryable()),
    };
    CallToolResult::ok(
        vec![ContentBlock::Text { text: format!("published {} chars of reply {reply_id} to {shells} shell(s)", text.chars().count()) }],
        Some(serde_json::json!({ "ok": true, "replyId": reply_id, "complete": complete, "shells": shells })),
    )
}

/// 💬️ Registers `conversation_reply`. Separate from [`register_ui_tools`] because it is the one
/// tool here that is scope-gated, and therefore the one that needs the connection's own principal
/// and policy engine rather than only the bridge slot.
pub fn register_conversation_tools(registry: &mut InMemoryToolRegistry, bridge: Option<BridgeSlot>, actions: Arc<ActionAdapter>, principal: AgentPrincipal) {
    let mut conversation_reply = Tool::new("conversation_reply", conversation_reply_input_schema());
    conversation_reply.title = Some("Reply In Chat".to_string());
    conversation_reply.description = Some("Publishes one chunk of your own free-text turn into the shell's agent panel, so the human reads your answer where they typed their question. Pass the same replyId with complete=false to stream.".to_string());
    conversation_reply.output_schema = Some(conversation_reply_output_schema());
    registry
        .register(conversation_reply, move |arguments| conversation_reply_handler(actions.policy(), &principal, resolve_bridge(bridge.as_ref()), arguments))
        .expect("conversation_reply is a valid tool name");
}

//#endregion 💬️ConversationReply

//#endregion 🔖️ToolHandlers

//#region 🔖️ShellStateProjection

/// 🧭️ The attached shell's last full `ShellState` snapshot, decoded as raw JSON — `🧵️bridge`'s own
/// design note says `state`/`command` bytes are opaque, AgentBridge encodes/decodes them as JSON on
/// the renderer side, so this facet mirrors that instead of taking a new compile-time dependency on
/// `semio-framework-os-shell`'s Rust types (see this file's W6 report for the tradeoff). A
/// `ShellStatePatch`-only connection (no full snapshot received yet) is a typed, retryable
/// `PLUGIN_UNAVAILABLE`, never a guess at merging a patch onto nothing.
fn full_shell_state(bridge: Option<&Arc<BridgeHandle>>) -> Result<serde_json::Value, GatewayError> {
    let bridge = bridge.ok_or_else(bridge_not_running_error)?;
    let connection_id = active_shell_connection(bridge).ok_or_else(no_shell_attached_error)?;
    match bridge.last_shell_state(connection_id) {
        Some(ShellToGateway::ShellState { state, .. }) => serde_json::from_slice(&state).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("failed to decode the attached shell's ShellState JSON: {error}"))),
        Some(ShellToGateway::ShellStatePatch { .. }) => Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, "the attached shell has only sent incremental ShellStatePatch frames so far — no full ShellState snapshot to project yet; this resolves once the shell publishes its next full snapshot").retryable()),
        _ => Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, "the attached shell has not published a ShellState snapshot yet").retryable()),
    }
}

fn panel_summary(state: &serde_json::Value) -> serde_json::Value {
    let mut panels = serde_json::Map::new();
    for anchor in ["left", "right", "top", "bottom"] {
        panels.insert(
            anchor.to_string(),
            serde_json::json!({
                "visible": state.pointer(&format!("/panelsVisible/{anchor}")).cloned().unwrap_or(serde_json::Value::Null),
                "size": state.pointer(&format!("/panelsSize/{anchor}")).cloned().unwrap_or(serde_json::Value::Null),
                "path": state.pointer(&format!("/panelsPath/{anchor}")).cloned().unwrap_or(serde_json::Value::Null),
            }),
        );
    }
    serde_json::Value::Object(panels)
}

fn window_entries(state: &serde_json::Value) -> Vec<serde_json::Value> {
    let extra_windows = state.get("extraWindows").and_then(serde_json::Value::as_array).cloned().unwrap_or_default();
    let titles = state.get("windowTitlesById").cloned().unwrap_or_else(|| serde_json::json!({}));
    let icons = state.get("windowIconsById").cloned().unwrap_or_else(|| serde_json::json!({}));
    let active_window_id = state.get("activeWindowId").and_then(serde_json::Value::as_str);
    extra_windows
        .iter()
        .map(|window| {
            let window_id = window.get("windowId").and_then(serde_json::Value::as_str).unwrap_or_default();
            serde_json::json!({
                "windowId": window_id,
                "kind": window.get("kind").cloned().unwrap_or(serde_json::Value::Null),
                "title": titles.get(window_id).cloned().unwrap_or(serde_json::Value::Null),
                "icon": icons.get(window_id).cloned().unwrap_or(serde_json::Value::Null),
                "focused": active_window_id == Some(window_id),
            })
        })
        .collect()
}

fn read_window_resource(bridge: Option<&Arc<BridgeHandle>>, window_id: Option<&str>) -> Result<Vec<ResourceContent>, GatewayError> {
    let state = full_shell_state(bridge)?;
    let windows = window_entries(&state);
    match window_id {
        Some(window_id) => match windows.into_iter().find(|window| window.get("windowId").and_then(serde_json::Value::as_str) == Some(window_id)) {
            Some(window) => Ok(vec![ResourceContent { uri: format!("semio://window/{window_id}"), mime_type: Some("application/json".to_string()), text: Some(window.to_string()), blob: None }]),
            None => Err(GatewayError::new(GatewayErrorCode::NotFound, format!("no such window: {window_id}"))),
        },
        None => {
            let payload = serde_json::json!({ "activeWindowId": state.get("activeWindowId").cloned().unwrap_or(serde_json::Value::Null), "windows": windows, "panels": panel_summary(&state) });
            Ok(vec![ResourceContent { uri: "semio://window".to_string(), mime_type: Some("application/json".to_string()), text: Some(payload.to_string()), blob: None }])
        }
    }
}

fn read_active_context_resource(bridge: Option<&Arc<BridgeHandle>>) -> Result<Vec<ResourceContent>, GatewayError> {
    let state = full_shell_state(bridge)?;
    let payload = serde_json::json!({
        "revision": state.get("revision").cloned().unwrap_or(serde_json::Value::Null),
        "activeWindowId": state.get("activeWindowId").cloned().unwrap_or(serde_json::Value::Null),
        "activeToolId": state.get("activeToolId").cloned().unwrap_or(serde_json::Value::Null),
        "activeUtilityByWindow": state.get("activeUtilityByWindow").cloned().unwrap_or_else(|| serde_json::json!({})),
        "activeExampleId": state.get("activeExampleId").cloned().unwrap_or(serde_json::Value::Null),
        "openWithFocusRole": state.get("openWithFocusRole").cloned().unwrap_or(serde_json::Value::Null),
        "activeTutorialId": state.get("activeTutorialId").cloned().unwrap_or(serde_json::Value::Null),
        "uiLocale": state.get("uiLocale").cloned().unwrap_or(serde_json::Value::Null),
        "uiThemeId": state.get("uiThemeId").cloned().unwrap_or(serde_json::Value::Null),
    });
    Ok(vec![ResourceContent { uri: "semio://ui/active-context".to_string(), mime_type: Some("application/json".to_string()), text: Some(payload.to_string()), blob: None }])
}

/// 🎯️ `ShellState` models exactly ONE selection-shaped field — `selectedConflictId` (merge conflict
/// preview). Per-artifact object selection (e.g. a CAD scene's selected entities) is app-instance
/// state carried over `AppFrames`/`Instances`, not shell state, and this facet does not reach into
/// it (design requirement: never a second, invented source of truth) — `activeWindowId`/
/// `activeToolId` are included as the closest real "what's focused" signals `ShellState` has.
fn read_selection_resource(bridge: Option<&Arc<BridgeHandle>>) -> Result<Vec<ResourceContent>, GatewayError> {
    let state = full_shell_state(bridge)?;
    let payload = serde_json::json!({
        "revision": state.get("revision").cloned().unwrap_or(serde_json::Value::Null),
        "selectedConflictId": state.get("selectedConflictId").cloned().unwrap_or(serde_json::Value::Null),
        "activeWindowId": state.get("activeWindowId").cloned().unwrap_or(serde_json::Value::Null),
        "activeToolId": state.get("activeToolId").cloned().unwrap_or(serde_json::Value::Null),
    });
    Ok(vec![ResourceContent { uri: "semio://ui/selection".to_string(), mime_type: Some("application/json".to_string()), text: Some(payload.to_string()), blob: None }])
}

fn read_job_resource(job_id: &str) -> Result<Vec<ResourceContent>, GatewayError> {
    match job_registry().snapshot(job_id) {
        Some(snapshot) => Ok(vec![ResourceContent { uri: format!("semio://job/{job_id}"), mime_type: Some("application/json".to_string()), text: Some(snapshot.to_json().to_string()), blob: None }]),
        None => Err(GatewayError::new(GatewayErrorCode::NotFound, format!("no such job: {job_id}"))),
    }
}

/// 🖥️ The UI/job resource entries to advertise in `resources/list` — presence never depends on
/// tier (§ module doc); `bridge` is accepted for signature symmetry with `read_ui_resource`, not
/// read (the returned list is identical bare/headless/attached, matching
/// `WorkspaceResourceRegistry::list`'s own convention for `semio://workspace`).
pub fn ui_resources(_bridge: Option<&BridgeSlot>) -> Vec<Resource> {
    vec![
        Resource {
            uri: "semio://window".to_string(),
            name: "ui-windows".to_string(),
            title: Some("Windows & panels".to_string()),
            description: Some("Window/panel inventory projected from the attached shell's ShellState (PLUGIN_UNAVAILABLE with no shell attached)".to_string()),
            mime_type: Some("application/json".to_string()),
            size: None,
        },
        Resource {
            uri: "semio://ui/active-context".to_string(),
            name: "ui-active-context".to_string(),
            title: Some("Active UI context".to_string()),
            description: Some("Active window/tool/utility/example on the attached shell".to_string()),
            mime_type: Some("application/json".to_string()),
            size: None,
        },
        Resource {
            uri: "semio://ui/agent-messages".to_string(),
            name: "ui-agent-messages".to_string(),
            title: Some("Messages from the human".to_string()),
            description: Some("Unread turns the human typed into the shell's agent chat panel, oldest first. Reading DRAINS them, so each turn is delivered exactly once.".to_string()),
            mime_type: Some("application/json".to_string()),
            size: None,
        },
        Resource {
            uri: "semio://ui/selection".to_string(),
            name: "ui-selection".to_string(),
            title: Some("UI selection".to_string()),
            description: Some("Shell-level selection only (selected conflict, active window/tool) — per-artifact object selection lives in app-instance state and is not projected here".to_string()),
            mime_type: Some("application/json".to_string()),
            size: None,
        },
    ]
}

/// 🖥️ The UI/job resource TEMPLATES to advertise in `resources/templates/list` — split from
/// [`ui_resources`] because `semio://window/{windowId}`/`semio://job/{jobId}` are parametrized
/// (`ResourceTemplate`, not `Resource`); not in the W6 contract's original shape, added because
/// `semio://job/{id}` has nowhere else to be listed (see this file's W6 report).
pub fn ui_resource_templates() -> Vec<ResourceTemplate> {
    vec![
        ResourceTemplate { uri_template: "semio://window/{windowId}".to_string(), name: "ui-window".to_string(), title: Some("One window".to_string()), description: Some("One extra window's title/icon/focus state".to_string()), mime_type: Some("application/json".to_string()) },
        ResourceTemplate { uri_template: "semio://job/{jobId}".to_string(), name: "job".to_string(), title: Some("One job".to_string()), description: Some("Status/progress/result for one plugin-agnostic job id".to_string()), mime_type: Some("application/json".to_string()) },
    ]
}

/// 🖥️ Answers `semio://window[/…]`, `semio://ui/active-context`, `semio://ui/selection`,
/// `semio://job/{id}`; returns `None` when `uri` is not one of ours so the composing registry can
/// fall through to its own `NOT_FOUND`. `_workspace` is accepted for signature symmetry (today's UI
/// projections read only the bridge's `ShellState` mirror, per design requirement #1).
pub fn read_ui_resource(uri: &str, bridge: Option<&BridgeSlot>, _workspace: Option<&Arc<HeadlessWorkspace>>) -> Option<Result<Vec<ResourceContent>, GatewayError>> {
    let bridge = resolve_bridge(bridge);
    if let Some(job_id) = uri.strip_prefix("semio://job/") {
        return Some(read_job_resource(job_id));
    }
    if let Some(window_id) = uri.strip_prefix("semio://window/") {
        return Some(read_window_resource(bridge, Some(window_id)));
    }
    match uri {
        "semio://window" => Some(read_window_resource(bridge, None)),
        "semio://ui/active-context" => Some(read_active_context_resource(bridge)),
        "semio://ui/selection" => Some(read_selection_resource(bridge)),
        "semio://ui/agent-messages" => Some(read_agent_messages_resource(bridge)),
        _ => None,
    }
}

/// 💬️ The receiving half of the agent chat panel: every turn the human typed at the agent since the
/// last read, oldest first. A READ DRAINS the queue — MCP resources have no server-push, so the
/// agent polls this uri, and a turn it has already been handed must not be handed to it twice.
/// Bare/headless tiers answer the same typed, retryable `PLUGIN_UNAVAILABLE` every other UI resource
/// does; an attached shell that has simply said nothing answers an empty list, which is a real
/// answer, not a gap.
fn read_agent_messages_resource(bridge: Option<&Arc<BridgeHandle>>) -> Result<Vec<ResourceContent>, GatewayError> {
    let bridge = bridge.ok_or_else(bridge_not_running_error)?;
    let connection = active_shell_connection(bridge).ok_or_else(no_shell_attached_error)?;
    let messages: Vec<serde_json::Value> = bridge
        .take_agent_messages(connection)
        .into_iter()
        .map(|message| serde_json::json!({ "messageId": message.message_id, "text": message.text, "receivedAtMs": message.received_at_ms }))
        .collect();
    let body = serde_json::json!({ "connection": connection.to_string(), "messages": messages, "drained": true });
    Ok(vec![ResourceContent { uri: "semio://ui/agent-messages".to_string(), mime_type: Some("application/json".to_string()), text: Some(body.to_string()), blob: None }])
}

//#endregion 🔖️ShellStateProjection

//#region 🔖️Schemas
//#endregion 🔖️Schemas

//#region 🧪️Tests
//#endregion 🧪️Tests
