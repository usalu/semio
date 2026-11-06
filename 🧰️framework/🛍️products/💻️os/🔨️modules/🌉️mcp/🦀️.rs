//! 🌉️ Catalog-backed OS MCP gateway over stdio and Streamable HTTP.

use semio_os_mcp_bridge::session::AgentConversation;
use semio_os_mcp_bridge::shell_command_dispatch::BridgeSlot;
use semio_os_mcp_capabilities::core_capabilities::{build_catalog, gateway_only_catalog};
use semio_os_mcp_catalog::capability_tools::{to_schema_search_hit, tool_from_capability};
use semio_os_mcp_catalog::catalog::{CapabilityKind, Catalog};
use semio_os_mcp_catalog::search::{SearchFilters, search};
use semio_os_mcp_consent::approval_coordinator::ApprovalCoordinator;
use semio_os_mcp_consent::elicitation::ElicitationSlot;
use semio_os_mcp_dispatch::actions::ActionAdapter;
use semio_os_mcp_foundation::agent_credential::AgentCredentialSource;
use semio_os_mcp_foundation::audit::{AuditSinks, ClientInfo, FileAuditSink, default_audit_dir};
use semio_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use semio_os_mcp_foundation::handles::{HandleTable, IdempotencyStore, SessionHandle};
use semio_os_mcp_foundation::rendezvous::{BridgeOffer, BridgeOfferScope, PublishedBridgeOffer, RENDEZVOUS_SCHEMA_VERSION, live_os_sessions, mint_admission_proof, publish_offer};
use semio_os_mcp_foundation::reports::{RevisionStamp, SearchHit};
use semio_os_mcp_http_transport::transport::{HttpTransport, HttpTransportOptions, HttpTransportRun};
use semio_os_mcp_plugin_runtime::plugin_binding_and_activation::WorkspaceOrigin;
use semio_os_mcp_port::channel_port::{InvokeRequest, UnboundArtifactChannel};
use semio_os_mcp_port::policy::{AgentPrincipal, AutoApprovePolicy};
use semio_os_mcp_routing::routing_and_shell_routed::{ArtifactChannels, ShellRoutedArtifactChannel};
use semio_os_mcp_schema::schema::{action_invoke_input_schema, action_prepare_input_schema, capability_generic_output_schema, handle_input_schema, tool_output_schema, transaction_begin_input_schema};
use semio_os_mcp_server::mcp_server::McpServer;
use semio_os_mcp_server::stdio_transport::{McpTransport, StdioTransport};
use semio_os_mcp_shell_channel::context_resolve::{mint_session_id, resolve_context};
use semio_os_mcp_shell_channel::shell_channel::SessionChannelBinding;
use semio_os_mcp_tools_gateway::artifact::register_artifact_tools;
use semio_os_mcp_tools_gateway::context::WorkspaceResourceRegistry;
use semio_os_mcp_tools_gateway::prompts::build_prompt_registry;
use semio_os_mcp_tools_gateway::ui::{register_conversation_tools, register_ui_tools};
use semio_os_mcp_tools_inference::inference::{register_inference_job_tools, register_inference_tools};
use semio_os_mcp_wire::notify::notification_slot;
use semio_os_mcp_wire::protocol::{CallToolResult, ContentBlock, GatewayBackend, InMemoryToolRegistry, NullBackend, PromptRegistry, ResourceRegistry, Tool, ToolRegistry};
use semio_os_mcp_workspace::gateway_backend::{GatewayBackends, SharedWorkspace};
use semio_os_mcp_workspace::headless_workspace_state::HeadlessWorkspace;
#[cfg(not(target_arch = "wasm32"))]
use semio_os_mcp_hub_remote::remote::exchange_delegated_session_v1;
#[cfg(not(target_arch = "wasm32"))]
use semio_os_mcp_routing::routing_and_shell_routed::capability_instance_slot;


pub use semio_os_mcp_process_arena::ProcessRunFailure;

fn parse_capability_kind(raw: &str) -> Option<CapabilityKind> {
    match raw.to_ascii_lowercase().as_str() {
        "mutation" => Some(CapabilityKind::Mutation),
        "view" => Some(CapabilityKind::View),
        "history" => Some(CapabilityKind::History),
        "clipboard" => Some(CapabilityKind::Clipboard),
        "shell" => Some(CapabilityKind::Shell),
        "interaction" => Some(CapabilityKind::Interaction),
        "query" => Some(CapabilityKind::Query),
        "job" => Some(CapabilityKind::Job),
        "ui" => Some(CapabilityKind::Ui),
        "meta" => Some(CapabilityKind::Meta),
        _ => None,
    }
}

fn search_filters_from_arguments(arguments: &serde_json::Value) -> SearchFilters {
    let kind = arguments.get("kind").and_then(serde_json::Value::as_array).map(|values| values.iter().filter_map(serde_json::Value::as_str).filter_map(parse_capability_kind).collect()).unwrap_or_default();
    SearchFilters {
        kind,
        owner: arguments.get("owner").and_then(serde_json::Value::as_str).map(str::to_string),
        artifact_kind: arguments.get("artifactKind").and_then(serde_json::Value::as_str).map(str::to_string),
        requires_scope: arguments.get("requiresScope").and_then(serde_json::Value::as_str).map(str::to_string),
        audience: Vec::new(),
    }
}

/// 📄️ Requested page size clamped to the declared minimum and maximum.
const SEARCH_PAGE_DEFAULT: usize = 20;

const SEARCH_PAGE_MAX: usize = 100;

fn search_page_size(arguments: &serde_json::Value) -> usize {
    arguments.get("limit").and_then(serde_json::Value::as_u64).map(|limit| (limit as usize).clamp(1, SEARCH_PAGE_MAX)).unwrap_or(SEARCH_PAGE_DEFAULT)
}

/// 📄️ Decimal result offset; malformed cursors restart at the first page.
fn search_offset(arguments: &serde_json::Value) -> usize {
    arguments.get("cursor").and_then(serde_json::Value::as_str).and_then(|cursor| cursor.parse::<usize>().ok()).unwrap_or(0)
}

fn capabilities_search_handler(catalog: &Catalog, arguments: serde_json::Value) -> CallToolResult {
    let query = arguments.get("query").and_then(serde_json::Value::as_str).unwrap_or("").to_string();
    let filters = search_filters_from_arguments(&arguments);
    let hits = search(catalog, &query, &filters);
    let total = hits.len();
    let offset = search_offset(&arguments).min(total);
    let page_size = search_page_size(&arguments);
    let search_hits: Vec<SearchHit> = hits.iter().skip(offset).take(page_size).filter_map(|hit| catalog.get(&hit.capability_id).map(|capability| to_schema_search_hit(capability, hit.score))).collect();
    let next_offset = offset + search_hits.len();
    let next_cursor = if next_offset < total { serde_json::Value::String(next_offset.to_string()) } else { serde_json::Value::Null };
    let structured = serde_json::json!({ "results": search_hits, "total": total, "nextCursor": next_cursor });
    CallToolResult::ok(vec![ContentBlock::Text { text: format!("{} of {total} result(s) for {query:?}", search_hits.len()) }], Some(structured))
}

fn capabilities_describe_handler(catalog: &Catalog, arguments: serde_json::Value) -> CallToolResult {
    let id = arguments.get("capabilityId").and_then(serde_json::Value::as_str).unwrap_or("");
    match catalog.get(id) {
        Some(capability) => CallToolResult::ok(vec![ContentBlock::Text { text: capability.title.clone() }], Some(serde_json::to_value(capability).unwrap_or(serde_json::Value::Null))),
        None => CallToolResult::tool_error(&GatewayError::new(GatewayErrorCode::NotFound, format!("no such capability: {id}"))),
    }
}

fn caller_locale(arguments: &serde_json::Value) -> Result<&str, GatewayError> {
    match arguments.get("locale").and_then(serde_json::Value::as_str) {
        Some(locale @ ("en" | "de")) => Ok(locale),
        _ => Err(GatewayError::new(GatewayErrorCode::InputInvalid, "locale must explicitly name en or de")),
    }
}

fn context_resolve_handler(catalog: &Catalog, counter: &std::sync::atomic::AtomicU64, arguments: serde_json::Value) -> CallToolResult {
    let principal = arguments.get("principal").and_then(serde_json::Value::as_str).unwrap_or("agent:local").to_string();
    let locale = match caller_locale(&arguments) { Ok(locale) => locale, Err(error) => return CallToolResult::tool_error(&error) };
    let count = counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let session_id = mint_session_id(&principal, count);
    let summary = resolve_context(catalog, session_id, &principal, Vec::new(), None, locale);
    CallToolResult::ok(vec![ContentBlock::Text { text: format!("session {} resolved", summary.session_id) }], Some(serde_json::to_value(&summary).unwrap_or(serde_json::Value::Null)))
}

/// 🎯️ The complete public gateway tool census in tools/list order.
pub const GATEWAY_TOOL_NAMES: [&str; 28] = [
    "capabilities_search",
    "capabilities_describe",
    "context_resolve",
    "action_prepare",
    "action_invoke",
    "action_cancel",
    "transaction_begin",
    "transaction_commit",
    "transaction_rollback",
    "history_undo",
    "history_redo",
    "artifact_open",
    "artifact_create",
    "artifact_validate",
    "artifact_snapshot",
    "artifact_export",
    "inference_list",
    "inference_get",
    "inference_run",
    "inference_submit",
    "inference_events",
    "inference_cancel",
    "inference_approve",
    "ui_focus",
    "ui_reveal",
    "conversation_reply",
    "job_get",
    "job_cancel",
];



/// 🕐️ Wall-clock milliseconds — every `ActionAdapter`/`HandleTable` call takes `now_ms` explicitly
/// (testability), this is the ONE call site that reads the real clock for the live server.
fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64
}

/// 🪪️ The gateway connection's mutation-protocol handle namespace.
pub(crate) const DEFAULT_SESSION_ID: &str = "sess_default";

pub(crate) fn default_session() -> SessionHandle {
    SessionHandle::new(DEFAULT_SESSION_ID)
}

fn parse_revision_stamp(value: &serde_json::Value) -> Option<RevisionStamp> {
    serde_json::from_value(value.clone()).ok()
}

/// 🎯️ Decodes the capability owner's catalog route before a capability-less history read.
#[cfg(not(target_arch = "wasm32"))]
fn action_instance_slot(catalog: &Catalog, capability_id: Option<&str>) -> u32 {
    capability_id.and_then(|id| capability_instance_slot(catalog, id)).unwrap_or(0)
}

#[cfg(target_arch = "wasm32")]
fn action_instance_slot(_catalog: &Catalog, _capability_id: Option<&str>) -> u32 {
    0
}

fn action_caller_context(arguments: &serde_json::Value) -> Result<Option<semio_framework::ViewModel>, GatewayError> {
    let context: Option<semio_framework::ViewModel> = arguments.get("context").cloned().map(serde_json::from_value).transpose().map_err(|error| GatewayError::new(GatewayErrorCode::InputInvalid, format!("invalid caller context: {error}")))?;
    if let Some(target) = arguments.get("target") {
        for (selector, actual) in [("windowId", context.as_ref().and_then(|context| context.window_id.as_deref())), ("modeId", context.as_ref().and_then(|context| context.active_mode_id.as_deref()))] {
            if target.get(selector).and_then(serde_json::Value::as_str).is_some_and(|requested| Some(requested) != actual) {
                return Err(GatewayError::new(GatewayErrorCode::InputInvalid, format!("target {selector} requires the identical explicit caller context")));
            }
        }
    }
    Ok(context)
}

fn action_target_instance(catalog: &Catalog, actions: &ActionAdapter, principal: &AgentPrincipal, capability_id: &str, arguments: &serde_json::Value) -> Result<u32, GatewayError> {
    match arguments.get("target") {
        Some(target) => {
            let target = serde_json::from_value(target.clone()).map_err(|error| GatewayError::new(GatewayErrorCode::InputInvalid, format!("invalid action target: {error}")))?;
            actions.target_instance(catalog, principal, capability_id, &target)
        }
        None => Ok(action_instance_slot(catalog, Some(capability_id))),
    }
}

fn action_prepare_handler(catalog: &Catalog, actions: &ActionAdapter, principal: &AgentPrincipal, arguments: serde_json::Value) -> CallToolResult {
    let capability_id = match arguments.get("capabilityId").and_then(serde_json::Value::as_str) {
        Some(id) => id,
        None => return CallToolResult::tool_error(&GatewayError::new(GatewayErrorCode::InputInvalid, "capabilityId is required")),
    };
    let input = arguments.get("input").cloned().unwrap_or_else(|| serde_json::json!({}));
    let context = match action_caller_context(&arguments) { Ok(context) => context, Err(error) => return CallToolResult::tool_error(&error) };
    let instance = match action_target_instance(catalog, actions, principal, capability_id, &arguments) { Ok(instance) => instance, Err(error) => return CallToolResult::tool_error(&error) };
    match actions.prepare_in_context(catalog, principal, &default_session(), capability_id, input, instance, context, now_ms()) {
        Ok(report) => CallToolResult::ok(vec![ContentBlock::Text { text: format!("prepared {}", report.prepared_handle) }], Some(serde_json::to_value(&report).unwrap_or(serde_json::Value::Null))),
        Err(error) => CallToolResult::tool_error(&error),
    }
}

fn action_invoke_handler(catalog: &Catalog, actions: &ActionAdapter, principal: &AgentPrincipal, arguments: serde_json::Value) -> CallToolResult {
    if arguments.get("preparedActionHandle").is_some() && ["target", "context", "capabilityId", "input"].iter().any(|key| arguments.get(key).is_some()) {
        return CallToolResult::tool_error(&GatewayError::new(GatewayErrorCode::InputInvalid, "prepared actions retain their target, input and context; invoke the handle without overrides"));
    }
    let context = match action_caller_context(&arguments) { Ok(context) => context, Err(error) => return CallToolResult::tool_error(&error) };
    let request = InvokeRequest {
        context,
        prepared_handle: arguments.get("preparedActionHandle").and_then(serde_json::Value::as_str).map(str::to_string),
        capability_id: arguments.get("capabilityId").and_then(serde_json::Value::as_str).map(str::to_string),
        input: arguments.get("input").cloned(),
        expected_revision: arguments.get("expectedRevision").and_then(parse_revision_stamp),
        idempotency_key: arguments.get("idempotencyKey").and_then(serde_json::Value::as_str).map(str::to_string),
        approval_handle: arguments.get("approvalHandle").and_then(serde_json::Value::as_str).map(str::to_string),
    };
    let instance = match request.capability_id.as_deref() {
        Some(capability_id) => match action_target_instance(catalog, actions, principal, capability_id, &arguments) { Ok(instance) => instance, Err(error) => return CallToolResult::tool_error(&error) },
        None => 0,
    };
    match actions.invoke(catalog, principal, &default_session(), request, instance, now_ms()) {
        Ok(report) => CallToolResult::ok(vec![ContentBlock::Text { text: format!("invocation {} {:?}", report.invocation_id, report.status) }], Some(serde_json::to_value(&report).unwrap_or(serde_json::Value::Null))),
        Err(error) => CallToolResult::tool_error(&error),
    }
}

fn action_cancel_handler(actions: &ActionAdapter, arguments: serde_json::Value) -> CallToolResult {
    let handle = match arguments.get("preparedActionHandle").and_then(serde_json::Value::as_str) {
        Some(handle) => handle,
        None => return CallToolResult::tool_error(&GatewayError::new(GatewayErrorCode::InputInvalid, "preparedActionHandle is required")),
    };
    match actions.cancel(&default_session(), handle, now_ms()) {
        Ok(()) => CallToolResult::ok(vec![ContentBlock::Text { text: format!("cancelled {handle}") }], Some(serde_json::json!({ "cancelled": true }))),
        Err(error) => CallToolResult::tool_error(&error),
    }
}

fn transaction_begin_handler(actions: &ActionAdapter, arguments: serde_json::Value) -> CallToolResult {
    let handles: Vec<String> = arguments.get("preparedHandles").and_then(serde_json::Value::as_array).map(|values| values.iter().filter_map(serde_json::Value::as_str).map(str::to_string).collect()).unwrap_or_default();
    match actions.transaction_begin(&default_session(), &handles, now_ms()) {
        Ok(transaction_handle) => CallToolResult::ok(vec![ContentBlock::Text { text: format!("saga {transaction_handle} began with {} member(s)", handles.len()) }], Some(serde_json::json!({ "transactionHandle": transaction_handle }))),
        Err(error) => CallToolResult::tool_error(&error),
    }
}

fn transaction_commit_handler(actions: &ActionAdapter, principal: &AgentPrincipal, arguments: serde_json::Value) -> CallToolResult {
    let handle = match arguments.get("transactionHandle").and_then(serde_json::Value::as_str) {
        Some(handle) => handle,
        None => return CallToolResult::tool_error(&GatewayError::new(GatewayErrorCode::InputInvalid, "transactionHandle is required")),
    };
    match actions.transaction_commit(principal, &default_session(), handle, now_ms()) {
        Ok(report) => CallToolResult::ok(vec![ContentBlock::Text { text: format!("saga {} committed {} member(s)", report.transaction_handle, report.members.len()) }], Some(serde_json::to_value(&report).unwrap_or(serde_json::Value::Null))),
        Err(error) => CallToolResult::tool_error(&error),
    }
}

fn transaction_rollback_handler(actions: &ActionAdapter, arguments: serde_json::Value) -> CallToolResult {
    let handle = match arguments.get("transactionHandle").and_then(serde_json::Value::as_str) {
        Some(handle) => handle,
        None => return CallToolResult::tool_error(&GatewayError::new(GatewayErrorCode::InputInvalid, "transactionHandle is required")),
    };
    match actions.transaction_rollback(&default_session(), handle, now_ms()) {
        Ok(()) => CallToolResult::ok(vec![ContentBlock::Text { text: format!("saga {handle} rolled back") }], Some(serde_json::json!({ "transactionHandle": handle, "rolledBack": true }))),
        Err(error) => CallToolResult::tool_error(&error),
    }
}

fn history_undo_handler(actions: &ActionAdapter, arguments: serde_json::Value) -> CallToolResult {
    let token = match arguments.get("undoToken").and_then(serde_json::Value::as_str) {
        Some(token) => token,
        None => return CallToolResult::tool_error(&GatewayError::new(GatewayErrorCode::InputInvalid, "undoToken is required")),
    };
    match actions.history_undo(&default_session(), token, now_ms()) {
        Ok(report) => CallToolResult::ok(vec![ContentBlock::Text { text: format!("undid {} member(s)", report.members) }], Some(serde_json::to_value(&report).unwrap_or(serde_json::Value::Null))),
        Err(error) => CallToolResult::tool_error(&error),
    }
}

fn history_redo_handler(actions: &ActionAdapter, arguments: serde_json::Value) -> CallToolResult {
    let token = match arguments.get("undoToken").and_then(serde_json::Value::as_str) {
        Some(token) => token,
        None => return CallToolResult::tool_error(&GatewayError::new(GatewayErrorCode::InputInvalid, "undoToken is required")),
    };
    match actions.history_redo(&default_session(), token, now_ms()) {
        Ok(report) => CallToolResult::ok(vec![ContentBlock::Text { text: format!("redid {} member(s)", report.members) }], Some(serde_json::to_value(&report).unwrap_or(serde_json::Value::Null))),
        Err(error) => CallToolResult::tool_error(&error),
    }
}

/// 🏗️ Registers catalog discovery, workspace, actions, inference and UI tools on their owned adapters.
pub fn build_tool_registry(
    catalog: std::sync::Arc<Catalog>,
    actions: std::sync::Arc<ActionAdapter>,
    principal: AgentPrincipal,
    workspace: Option<std::sync::Arc<HeadlessWorkspace>>,
    bridge: Option<BridgeSlot>,
) -> InMemoryToolRegistry {
    let mut registry = InMemoryToolRegistry::new();
    let (conversation_actions, conversation_principal) = (actions.clone(), principal.clone());

    let search_tool = tool_from_capability(catalog.get("capabilities.search").expect("capabilities.search compiled"), "capabilities_search");
    let search_catalog = catalog.clone();
    registry.register(search_tool, move |arguments| capabilities_search_handler(&search_catalog, arguments)).expect("capabilities_search is a valid tool name");

    let describe_tool = tool_from_capability(catalog.get("capabilities.describe").expect("capabilities.describe compiled"), "capabilities_describe");
    let describe_catalog = catalog.clone();
    registry.register(describe_tool, move |arguments| capabilities_describe_handler(&describe_catalog, arguments)).expect("capabilities_describe is a valid tool name");

    let context_tool = tool_from_capability(catalog.get("context.resolve").expect("context.resolve compiled"), "context_resolve");
    let context_catalog = catalog.clone();
    let session_counter = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    registry.register(context_tool, move |arguments| context_resolve_handler(&context_catalog, &session_counter, arguments)).expect("context_resolve is a valid tool name");

    let mut action_prepare = Tool::new("action_prepare", action_prepare_input_schema());
    action_prepare.title = Some("Prepare Action".to_string());
    action_prepare.description = Some("Validates input, checks policy, captures the current revision, and dry-runs the capability — returns a PreparedActionReport.".to_string());
    action_prepare.output_schema = Some(tool_output_schema("PreparedActionReport"));
    let (c, a, p) = (catalog.clone(), actions.clone(), principal.clone());
    registry.register(action_prepare, move |arguments| action_prepare_handler(&c, &a, &p, arguments)).expect("action_prepare is a valid tool name");

    let mut action_invoke = Tool::new("action_invoke", action_invoke_input_schema());
    action_invoke.title = Some("Invoke Action".to_string());
    action_invoke.description = Some("Commits a prepared (or freshly-prepared) action through the 2-phase transaction protocol — returns an InvocationReport.".to_string());
    action_invoke.output_schema = Some(tool_output_schema("InvocationReport"));
    let (c, a, p) = (catalog.clone(), actions.clone(), principal.clone());
    registry.register(action_invoke, move |arguments| action_invoke_handler(&c, &a, &p, arguments)).expect("action_invoke is a valid tool name");

    let mut action_cancel = Tool::new("action_cancel", handle_input_schema("preparedActionHandle", "action.cancel"));
    action_cancel.title = Some("Cancel Action".to_string());
    action_cancel.description = Some("Drops a prepared-action handle before it is invoked.".to_string());
    action_cancel.output_schema = Some(capability_generic_output_schema("action.cancel"));
    let a = actions.clone();
    registry.register(action_cancel, move |arguments| action_cancel_handler(&a, arguments)).expect("action_cancel is a valid tool name");

    let mut transaction_begin = Tool::new("transaction_begin", transaction_begin_input_schema());
    transaction_begin.title = Some("Begin Transaction".to_string());
    transaction_begin.description = Some("Binds several already-prepared action handles into one saga transaction handle.".to_string());
    transaction_begin.output_schema = Some(capability_generic_output_schema("transaction.begin"));
    let a = actions.clone();
    registry.register(transaction_begin, move |arguments| transaction_begin_handler(&a, arguments)).expect("transaction_begin is a valid tool name");

    let mut transaction_commit = Tool::new("transaction_commit", handle_input_schema("transactionHandle", "transaction.commit"));
    transaction_commit.title = Some("Commit Transaction".to_string());
    transaction_commit.description = Some("Commits every member of a saga transaction (2-phase, reverse-order commit, compensating undo on failure).".to_string());
    transaction_commit.output_schema = Some(capability_generic_output_schema("transaction.commit"));
    let (a, p) = (actions.clone(), principal.clone());
    registry.register(transaction_commit, move |arguments| transaction_commit_handler(&a, &p, arguments)).expect("transaction_commit is a valid tool name");

    let mut transaction_rollback = Tool::new("transaction_rollback", handle_input_schema("transactionHandle", "transaction.rollback"));
    transaction_rollback.title = Some("Rollback Transaction".to_string());
    transaction_rollback.description = Some("Abandons a saga transaction before it is committed.".to_string());
    transaction_rollback.output_schema = Some(capability_generic_output_schema("transaction.rollback"));
    let a = actions.clone();
    registry.register(transaction_rollback, move |arguments| transaction_rollback_handler(&a, arguments)).expect("transaction_rollback is a valid tool name");

    let mut history_undo = Tool::new("history_undo", handle_input_schema("undoToken", "history.undo"));
    history_undo.title = Some("Undo".to_string());
    history_undo.description = Some("Fans TransactionUndo out to every member a committed invocation or saga touched.".to_string());
    history_undo.output_schema = Some(capability_generic_output_schema("history.undo"));
    let a = actions.clone();
    registry.register(history_undo, move |arguments| history_undo_handler(&a, arguments)).expect("history_undo is a valid tool name");

    let mut history_redo = Tool::new("history_redo", handle_input_schema("undoToken", "history.redo"));
    history_redo.title = Some("Redo".to_string());
    history_redo.description = Some("Fans TransactionRedo out to every member a committed invocation or saga touched.".to_string());
    history_redo.output_schema = Some(capability_generic_output_schema("history.redo"));
    let a = actions.clone();
    registry.register(history_redo, move |arguments| history_redo_handler(&a, arguments)).expect("history_redo is a valid tool name");

    register_artifact_tools(&mut registry, workspace.clone(), principal.clone());
    register_inference_tools(&mut registry, workspace.clone());
    register_inference_job_tools(&mut registry, catalog.clone(), workspace.clone(), actions.clone(), principal.clone(), default_session());
    register_conversation_tools(&mut registry, bridge.clone(), conversation_actions, conversation_principal);
    register_ui_tools(&mut registry, bridge, workspace, principal);

    registry
}


/// 🔌️ Late-bound bridge, elicitation and principal-scoped runtime services.
#[derive(Clone, Default)]
pub struct GatewayRuntime {
    pub bridge: Option<BridgeSlot>,
    pub elicitation: Option<ElicitationSlot>,
    pub auto_approve: AutoApprovePolicy,
    /// 🐚️ Sticky session channel shared by context resolution and artifact dispatch.
    pub channel_binding: Option<std::sync::Arc<SessionChannelBinding>>,
}

impl GatewayRuntime {
    /// 🐚️ Returns the runtime's binding or creates one from its bridge.
    pub fn channel_binding(&self) -> std::sync::Arc<SessionChannelBinding> {
        match &self.channel_binding {
            Some(binding) => std::sync::Arc::clone(binding),
            None => std::sync::Arc::new(SessionChannelBinding::new(self.bridge.clone())),
        }
    }

    /// ⛩️ Approval channels available to parked actions, ordered by runtime policy.
    fn approval_coordinator(&self) -> std::sync::Arc<ApprovalCoordinator> {
        std::sync::Arc::new(ApprovalCoordinator::new(self.elicitation.clone(), self.bridge.clone()))
    }
}


/// 🏗️ Builds the gateway with an explicit principal and owned artifact channel.
pub fn build_server_with_principal(principal: AgentPrincipal, audit: std::sync::Arc<AuditSinks>, channel: Box<ArtifactChannels>, runtime: GatewayRuntime) -> McpServer {
    build_server_from_catalog(std::sync::Arc::new(build_catalog()), principal, audit, channel, runtime)
}

/// 🗂️ Builds the same server over an explicitly supplied catalog.
pub fn build_server_from_catalog(catalog: std::sync::Arc<Catalog>, principal: AgentPrincipal, audit: std::sync::Arc<AuditSinks>, channel: Box<ArtifactChannels>, runtime: GatewayRuntime) -> McpServer {
    let handles = std::sync::Arc::new(HandleTable::new());
    let idempotency = std::sync::Arc::new(IdempotencyStore::new());
    let client = ClientInfo { name: "semio-os-mcp".to_string(), version: env!("CARGO_PKG_VERSION").to_string() };
    let actions = std::sync::Arc::new(ActionAdapter::new(channel, handles, idempotency, audit, runtime.auto_approve, client));
    actions.bind_approval_coordinator(runtime.approval_coordinator());
    let label = principal.label.clone();
    let tools = build_tool_registry(catalog.clone(), actions, principal, None, runtime.bridge.clone());
    let resources = WorkspaceResourceRegistry::new(catalog).with_bridge(runtime.bridge.clone());
    let server = McpServer::new(Box::new(tools), Box::new(resources), Box::new(build_prompt_registry()), Box::new(GatewayBackends::Null(NullBackend)));
    publishing_agent_conversation(server, runtime.bridge, &label)
}

/// 💬️ Publishes actual tool traffic to the attached shell bridge.
fn publishing_agent_conversation(server: McpServer, bridge: Option<BridgeSlot>, label: &str) -> McpServer {
    match bridge {
        Some(slot) => server.publishing_conversation_to(std::sync::Arc::new(AgentConversation::new(slot, label))),
        None => server,
    }
}

/// 🏠️ Builds the same gateway over a real workspace backend and its catalog.
pub fn build_server_with_workspace(principal: AgentPrincipal, audit: std::sync::Arc<AuditSinks>, workspace: std::sync::Arc<HeadlessWorkspace>, channel: Box<ArtifactChannels>, runtime: GatewayRuntime) -> McpServer {
    let catalog = workspace.discovery_catalog().unwrap_or_else(|_| gateway_only_catalog());
    let handles = std::sync::Arc::new(HandleTable::new());
    let idempotency = std::sync::Arc::new(IdempotencyStore::new());
    let client = ClientInfo { name: "semio-os-mcp".to_string(), version: env!("CARGO_PKG_VERSION").to_string() };
    let actions = std::sync::Arc::new(ActionAdapter::new(channel, handles, idempotency, audit, runtime.auto_approve, client));
    actions.bind_history_undo_port(workspace.clone());
    workspace.bind_root_action_adapter(actions.clone());
    actions.bind_approval_coordinator(runtime.approval_coordinator());
    let label = principal.label.clone();
    let tools = WorkspaceToolRegistry { workspace: workspace.clone(), actions, principal, bridge: runtime.bridge.clone(), channel_binding: runtime.channel_binding() };
    let resources = WorkspaceResourceRegistry::with_workspace(catalog, workspace.clone()).with_bridge(runtime.bridge.clone());
    let server = McpServer::new(Box::new(tools), Box::new(resources), Box::new(build_prompt_registry()), Box::new(GatewayBackends::WorkspaceArc(SharedWorkspace(workspace))));
    publishing_agent_conversation(server, runtime.bridge, &label)
}

/// 🔄️ Refreshes discovery from current authorized descriptors for every observation.
struct WorkspaceToolRegistry {
    pub(crate) workspace: std::sync::Arc<HeadlessWorkspace>,
    actions: std::sync::Arc<ActionAdapter>,
    pub(crate) principal: AgentPrincipal,
    bridge: Option<BridgeSlot>,
    channel_binding: std::sync::Arc<SessionChannelBinding>,
}

impl WorkspaceToolRegistry {
    pub(crate) fn current(&self) -> InMemoryToolRegistry {
        let catalog = self.workspace.discovery_catalog().unwrap_or_else(|_| gateway_only_catalog());
        let mut tools = build_tool_registry(catalog.clone(), self.actions.clone(), self.principal.clone(), Some(self.workspace.clone()), self.bridge.clone());
        let context_tool = tool_from_capability(catalog.get("context.resolve").expect("context.resolve compiled"), "context_resolve");
        registry_override_context_resolve(&mut tools, context_tool, self.workspace.clone(), self.principal.id.clone(), std::sync::Arc::clone(&self.channel_binding));
        tools
    }
}

impl ToolRegistry for WorkspaceToolRegistry {
    fn list(&self) -> Vec<Tool> {
        let mut tools = self.current().list();
        if let Some(selection) = workspace_tool_catalog_meta(&self.workspace) {
            for tool in &mut tools {
                if matches!(tool.name.as_str(), "capabilities_search" | "capabilities_describe" | "inference_list" | "inference_get") {
                    tool.meta = Some(selection.clone());
                }
            }
        }
        tools
    }

    fn call(&self, name: &str, arguments: serde_json::Value) -> Result<CallToolResult, GatewayError> {
        self.current().call(name, arguments)
    }
}

fn workspace_tool_catalog_meta(workspace: &HeadlessWorkspace) -> Option<serde_json::Value> {
    if !matches!(workspace.origin(), WorkspaceOrigin::Hub { .. }) {
        return None;
    }
    let selected = workspace.verified_hub_catalog_selections().ok()?;
    Some(serde_json::json!({
        "semio": {
            "hubSelectedPackages": selected.selections.iter().map(|selection| serde_json::json!({
                "scope": {
                    "spaceId": selection.scope.space_id,
                    "documentId": selection.scope.document_id,
                },
                "descriptorDigestV1": selection.descriptor_digest_v1,
                "catalogGenerationId": selection.lease.catalog.generation_id,
                "package": {
                    "pluginId": selection.lease.package.plugin_id,
                    "packageId": selection.lease.package.package_id,
                    "version": selection.lease.package.version,
                    "componentSha256": selection.lease.package.component_sha256,
                    "componentBlake3": selection.lease.package.component_blake3,
                    "descriptorByteSha256": selection.lease.package.descriptor_byte_sha256,
                    "executionProtocol": {
                        "appChannelVersion": selection.lease.package.execution_protocol.app_channel_version,
                    },
                }
            })).collect::<Vec<_>>()
        }
    }))
}

/// 🔁️ Replaces context resolution with this workspace's sticky channel and authorized live catalog.
fn registry_override_context_resolve(tools: &mut InMemoryToolRegistry, context_tool: Tool, workspace: std::sync::Arc<HeadlessWorkspace>, principal_id: String, channel_binding: std::sync::Arc<SessionChannelBinding>) {
    tools
        .register(context_tool, move |arguments| {
            let locale = match caller_locale(&arguments) { Ok(locale) => locale, Err(error) => return CallToolResult::tool_error(&error) };
            match workspace.resolve_context(&principal_id, locale) {
            Ok(mut summary) => {
                summary.channel = channel_binding.resolve().label().to_string();
                CallToolResult::ok(vec![ContentBlock::Text { text: format!("session {} resolved on the {} channel", summary.session_id, summary.channel) }], Some(serde_json::to_value(&summary).unwrap_or(serde_json::Value::Null)))
            }
            Err(error) => CallToolResult::tool_error(&error),
        }})
        .expect("context_resolve is a valid tool name");
}



/// 🏠️ Authorized hub binding supplied by the gateway launch options.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct HubOptions {
    pub base_url: String,
    pub space_id: String,
    /// 🤖️ Agent credential source; absent uses the inherited human bootstrap envelope.
    pub credential: Option<AgentCredentialSource>,
}

impl std::fmt::Debug for HubOptions {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("HubOptions").field("base_url", &self.base_url).field("space_id", &self.space_id).field("credential", &self.credential).finish()
    }
}

/// ⏳️ Maximum retries and interval for transient hub authority refreshes.
pub(crate) const HUB_OPEN_RETRY_ATTEMPTS: u32 = 6;

const HUB_OPEN_RETRY_STEP_MS: u64 = 500;

/// 🔁️ Opens the hub workspace, re-attempting ONLY the states the hub marks retryable — a descriptor [Design notes](README.md#source-notes-571).
fn open_hub_workspace_with_retry(hub: &HubOptions, credential: std::sync::Arc<semio_os_kernel::os_directory::client::LocalHubCredential>, principal: &AgentPrincipal, host_driver_policy:semio_plugin_host_wake::GuestRelayDriverPolicy, operation_policy:semio_os_mcp_port::channel_port::HostOperationPolicy) -> Result<HeadlessWorkspace, GatewayError> {
    let scopes: Vec<String> = principal.scopes.iter().map(|scope| scope.0.clone()).collect();
    let mut attempts_made = 0;
    loop {
        match HeadlessWorkspace::open_hub(hub.base_url.clone(), hub.space_id.clone(), credential.clone(), principal.id.clone(), scopes.clone(), host_driver_policy, operation_policy) {
            Ok(workspace) => return Ok(workspace),
            Err(error) => match hub_open_retry_backoff_ms(&error, attempts_made) {
                Some(backoff_ms) => {
                    attempts_made += 1;
                    eprintln!("[semio-os-mcp] hub binding is not settled yet ({}); retry {attempts_made}/{HUB_OPEN_RETRY_ATTEMPTS} in {backoff_ms} ms", error.message);
                    std::thread::sleep(std::time::Duration::from_millis(backoff_ms));
                }
                None => return Err(error),
            },
        }
    }
}

/// ⏳️ Returns the next retry delay only for a retryable failure within the attempt budget.
pub fn hub_open_retry_backoff_ms(error: &GatewayError, attempts_made: u32) -> Option<u64> {
    if !error.retryable || attempts_made >= HUB_OPEN_RETRY_ATTEMPTS {
        return None;
    }
    Some(HUB_OPEN_RETRY_STEP_MS.saturating_mul(u64::from(attempts_made) + 1))
}

/// 🏠️ Builds the real `McpServer` for a `--folder`/`--hub`-bound session: opens a real [Design notes](README.md#source-notes-604).
fn server_for_workspace_options(mut principal: AgentPrincipal, audit: std::sync::Arc<AuditSinks>, folder: Option<&str>, hub: Option<&HubOptions>, mut runtime: GatewayRuntime, host_driver_policy:semio_plugin_host_wake::GuestRelayDriverPolicy, operation_policy:semio_os_mcp_port::channel_port::HostOperationPolicy) -> Result<(McpServer, String), GatewayError> {
    let origin_label;
    let workspace = if let Some(folder) = folder {
        origin_label = format!("folder {folder}");
        let catalog = std::sync::Arc::new(build_catalog());
        std::sync::Arc::new(HeadlessWorkspace::open_folder(std::path::PathBuf::from(folder), principal.id.clone(), principal.scopes.iter().map(|scope| scope.0.clone()).collect(), catalog, host_driver_policy, operation_policy)?)
    } else if let Some(hub) = hub {
        origin_label = format!("hub {}/{}", hub.base_url, hub.space_id);
        let mut adopted: Option<(String, String)> = None;
        let credential = match &hub.credential {
            Some(source) => {
                let (protocol, delegation) = source.load()?;
                if delegation.scope_id() != hub.space_id {
                    return Err(GatewayError::new(GatewayErrorCode::PermissionDenied, format!("this agent credential is scoped to space `{}`, not `{}`", delegation.scope_id(), hub.space_id)));
                }
                let grant = exchange_delegated_session_v1(&hub.base_url, &delegation, protocol)?;
                eprintln!("[semio-os-mcp] acting as agent principal {} (\"{}\") in space {}", grant.agent_principal_id, grant.agent_label, grant.space_id);
                adopted = Some((grant.agent_principal_id.clone(), grant.agent_label.clone()));
                std::sync::Arc::new(
                    semio_os_kernel::os_directory::client::LocalHubCredential::adopt_session_capability(hub.base_url.trim_end_matches('/'), &grant.token)
                        .map_err(|_| GatewayError::new(GatewayErrorCode::PermissionDenied, "the hub returned an agent session capability this process cannot adopt"))?,
                )
            }
            None => semio_os_kernel::os_directory::identity::claimed_local_hub_credential("mcp")
                .ok_or_else(|| GatewayError::new(GatewayErrorCode::PermissionDenied, "hub workspace requires either --credential-file <delegated agent credential> or a protected process-entry MCP credential"))?,
        };
        if let Some((agent_principal_id, agent_label)) = adopted {
            principal.delegated_by = Some(principal.id.clone());
            principal.id = agent_principal_id;
            principal.label = agent_label;
        }
        std::sync::Arc::new(open_hub_workspace_with_retry(hub, credential, &principal, host_driver_policy, operation_policy)?)
    } else {
        let principal_id = principal.id.clone();
        return Ok((build_server_with_principal(principal, audit, Box::new(ArtifactChannels::Unbound(UnboundArtifactChannel)), runtime), principal_id));
    };
    eprintln!("[semio-os-mcp] real per-capability ArtifactChannel routing bound for {origin_label}");
    let binding = std::sync::Arc::new(SessionChannelBinding::for_principal(&principal, runtime.bridge.clone()));
    runtime.channel_binding = Some(std::sync::Arc::clone(&binding));
    let catalog = std::sync::Arc::new(build_catalog());
    workspace.bind_shell_route(std::sync::Arc::clone(&binding), std::sync::Arc::clone(&catalog));
    let channel: Box<ArtifactChannels> = Box::new(ArtifactChannels::Shell(ShellRoutedArtifactChannel::new(binding, catalog, workspace.open_routing_channel())));
    let principal_id = principal.id.clone();
    Ok((build_server_with_workspace(principal, audit, workspace, channel, runtime), principal_id))
}



/// ⚙️ Explicit stdio workspace, principal, audit and bridge launch options.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StdioOptions {
    pub host_driver_policy:semio_plugin_host_wake::GuestRelayDriverPolicy,
    pub operation_policy:semio_os_mcp_port::channel_port::HostOperationPolicy,
    pub folder: Option<String>,
    pub hub: Option<HubOptions>,
    pub principal: Option<String>,
    pub scopes: Vec<String>,
    pub auto_approve: AutoApprovePolicy,
    /// 🚫️ Disables the local bridge listener for an isolated stdio gateway.
    pub no_bridge: bool,
}

/// 🌉️ Starts the loopback bridge immediately; publishes its authenticated offer after the workspace opens.
fn start_stdio_bridge(options: &StdioOptions, bridge_slot: &BridgeSlot) -> Option<StdioBridgeListener> {
    if options.no_bridge {
        return None;
    }
    let proof = mint_admission_proof();
    let mut transport = HttpTransport::new(HttpTransportOptions::with_local_proof(&proof)).publishing_bridge_into(bridge_slot.clone());
    match transport.start_bridge_only() {
        Ok(run) => Some(StdioBridgeListener { run, proof }),
        Err(error) => {
            eprintln!("[semio-os-mcp] the loopback bridge listener could not bind ({}) — continuing over stdio with no live-shell surface", error.message);
            None
        }
    }
}

/// 🌉️ A started, not yet offered bridge listener.
struct StdioBridgeListener {
    run: HttpTransportRun,
    proof: String,
}

/// 📤️ Publishes the listener's principal-scoped authenticated bridge offer.
fn publish_stdio_bridge_offer(listener: StdioBridgeListener, principal_id: String, scope: BridgeOfferScope) -> Option<StdioBridgeAttachment> {
    let StdioBridgeListener { run, proof } = listener;
    let offered_to = match &scope {
        BridgeOfferScope::Hub { hub_origin, space_id } => format!("the shell of the human who delegated {principal_id} on {hub_origin} space {space_id}"),
        BridgeOfferScope::Local => format!("this user's local os sessions ({} live)", live_os_sessions().len()),
    };
    let offer = BridgeOffer {
        schema_version: RENDEZVOUS_SCHEMA_VERSION,
        url: format!("ws://{}/bridge", run.local_addr()),
        admission_proof: proof,
        principal: principal_id,
        pid: std::process::id(),
        published_at_ms: now_ms(),
        scope,
    };
    match publish_offer(offer) {
        Ok(published) => {
            eprintln!("[semio-os-mcp] bridge listening on ws://{}/bridge — offered to {offered_to} via {}", run.local_addr(), published.path().display());
            let offer = std::sync::Arc::new(std::sync::Mutex::new(Some(published)));
            let withdrawn = std::sync::Arc::clone(&offer);
            let completion = run.completion();
            std::thread::spawn(move || {
                completion.wait();
                withdrawn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            });
            Some(StdioBridgeAttachment { run, offer })
        }
        Err(error) => {
            eprintln!("[semio-os-mcp] the bridge offer could not be published ({}) — cancelling the listener rather than leaving an unreachable socket open", error.message);
            run.cancel();
            None
        }
    }
}

/// 🧷️ Retains the bridge listener and removes its offer when stdio serving ends.
struct StdioBridgeAttachment {
    run: HttpTransportRun,
    offer: std::sync::Arc<std::sync::Mutex<Option<PublishedBridgeOffer>>>,
}

impl Drop for StdioBridgeAttachment {
    fn drop(&mut self) {
        self.offer.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        self.run.cancel();
    }
}

/// 🚪️ Serves real stdio until EOF or an I/O failure, recording diagnostics only on stderr.
pub fn run_stdio(options:StdioOptions)->Result<(),ProcessRunFailure>{semio_os_mcp_process_arena::serve(options.host_driver_policy,options.operation_policy,||run_stdio_serving(options))}
fn run_stdio_serving(options: StdioOptions) -> Result<(), GatewayError> {
    let principal = AgentPrincipal::from_scope_names(options.principal.clone().unwrap_or_else(|| "agent:local".to_string()), "stdio agent", &options.scopes, None);
    if principal.scopes.is_empty() {
        eprintln!(
            "[semio-os-mcp] principal `{}` was launched with no --scopes: every policy-gated tool (action_prepare, action_invoke, inference_run, …) will answer PERMISSION_DENIED. Pass e.g. `--scopes workspace.read,artifact.write,inference.execute`.",
            principal.id
        );
    }
    let audit: std::sync::Arc<AuditSinks> = std::sync::Arc::new(AuditSinks::File(FileAuditSink::new(default_audit_dir())?));
    let bridge_slot: BridgeSlot = std::sync::Arc::new(std::sync::OnceLock::new());
    let elicitation: ElicitationSlot = std::sync::Arc::new(std::sync::OnceLock::new());
    let listener = start_stdio_bridge(&options, &bridge_slot);
    let runtime = GatewayRuntime { bridge: listener.as_ref().map(|_| bridge_slot.clone()), elicitation: Some(elicitation.clone()), auto_approve: options.auto_approve, channel_binding: None };
    let (server, principal_id) = server_for_workspace_options(principal, audit, options.folder.as_deref(), options.hub.as_ref(), runtime, options.host_driver_policy, options.operation_policy)?;
    let scope = options.hub.as_ref().map_or(BridgeOfferScope::Local, |hub| BridgeOfferScope::Hub { hub_origin: hub.base_url.clone(), space_id: hub.space_id.clone() });
    let attachment = listener.and_then(|listener| publish_stdio_bridge_offer(listener, principal_id, scope));
    let features = server.client_features();
    let notifications = notification_slot();
    let server = server.publishing_notifications_into(notifications.clone());
    let mut transport = StdioTransport::new(std::io::BufReader::new(std::io::stdin()), std::io::stdout(), std::io::stderr())
        .publishing_elicitation_into(elicitation, features)
        .publishing_notifications_into(notifications);
    let result = transport.serve(server);
    if let Some(attachment)=attachment.as_ref(){attachment.run.cancel();attachment.run.completion().wait();}
    drop(attachment);
    result
}



/// ⚙️ Explicit HTTP listener, workspace, principal, audit and runtime options.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpOptions {
    pub host_driver_policy:semio_plugin_host_wake::GuestRelayDriverPolicy,
    pub operation_policy:semio_os_mcp_port::channel_port::HostOperationPolicy,
    pub port: u16,
    pub bind: String,
    pub folder: Option<String>,
    pub hub: Option<HubOptions>,
    pub principal: Option<String>,
    pub scopes: Vec<String>,
    pub audit_dir: Option<String>,
    pub allow_origin: Vec<String>,
    pub auto_approve: AutoApprovePolicy,
}

/// 🌐️ Serves MCP and its bridge on the same retained HTTP transport until cancellation.
pub fn run_http(options:HttpOptions)->Result<(),ProcessRunFailure>{semio_os_mcp_process_arena::serve(options.host_driver_policy,options.operation_policy,||run_http_serving(options))}
fn run_http_serving(options: HttpOptions) -> Result<(), GatewayError> {
    let audit_dir = options.audit_dir.clone().map(std::path::PathBuf::from).unwrap_or_else(default_audit_dir);
    let audit: std::sync::Arc<AuditSinks> = std::sync::Arc::new(AuditSinks::File(FileAuditSink::new(audit_dir)?));
    let principal = AgentPrincipal::from_scope_names(options.principal.clone().unwrap_or_else(|| "agent:local".to_string()), "http agent", &options.scopes, None);
    let bridge_slot: BridgeSlot = std::sync::Arc::new(std::sync::OnceLock::new());
    let runtime = GatewayRuntime { bridge: Some(bridge_slot.clone()), elicitation: None, auto_approve: options.auto_approve, channel_binding: None };
    let (server, _principal_id) = server_for_workspace_options(principal, audit, options.folder.as_deref(), options.hub.as_ref(), runtime, options.host_driver_policy, options.operation_policy)?;
    let bind_ip: std::net::IpAddr = options.bind.parse().map_err(|error| GatewayError::new(GatewayErrorCode::InputInvalid, format!("invalid --bind address `{}`: {error}", options.bind)))?;
    let credential = semio_os_kernel::os_directory::identity::claimed_local_hub_credential("mcp")
        .ok_or_else(|| GatewayError::new(GatewayErrorCode::PermissionDenied, "HTTP mode requires a protected process-entry MCP credential"))?;
    eprintln!("[semio-os-mcp] bridge listening on ws://{bind_ip}:{}/bridge", options.port);
    let transport_options = HttpTransportOptions::new(credential).bind_addr(std::net::SocketAddr::new(bind_ip, options.port)).allowed_origins(options.allow_origin);
    let notifications = notification_slot();
    let server = server.publishing_notifications_into(notifications.clone());
    let mut transport = HttpTransport::new(transport_options).publishing_bridge_into(bridge_slot).publishing_notifications_into(notifications);
    transport.start(server)?.wait()
}



