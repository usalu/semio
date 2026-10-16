//! 🎯️ Mcp server of the server crate: The MCP server dispatcher and the stdio transport.

use std::sync::Arc;
use semio_framework_os_mcp_bridge::session::AgentConversation;
use semio_framework_os_mcp_bridge_codec::bridge::SELF_PUBLISHING_CONVERSATION_TOOLS;
use semio_framework_os_mcp_wire::notify::{DEFAULT_PAGE_SIZE, NotificationSlot, ProgressBinding, ResourceSubscriptions, broadcast_tool_result_changes, cancel_request, decode_cursor, enter_progress_scope, enter_request_scope, paginate};
use semio_framework_os_mcp_wire::protocol::{CallToolResult, ClientFeatures, ContentBlock, INVALID_PARAMS, InMemoryPromptRegistry, InMemoryResourceRegistry, InMemoryToolRegistry, JsonRpcRequest, JsonRpcResponse, METHOD_INITIALIZE, METHOD_NOTIFICATIONS_CANCELLED, METHOD_NOTIFICATIONS_INITIALIZED, METHOD_NOT_FOUND, METHOD_PING, METHOD_PROMPTS_GET, METHOD_PROMPTS_LIST, METHOD_RESOURCES_LIST, METHOD_RESOURCES_READ, METHOD_RESOURCES_SUBSCRIBE, METHOD_RESOURCES_TEMPLATES_LIST, METHOD_RESOURCES_UNSUBSCRIBE, METHOD_SERVER_DISCOVER, METHOD_TOOLS_CALL, METHOD_TOOLS_LIST, NullBackend, PromptRegistry, ProtocolEra, ResourceRegistry, SERVER_INSTRUCTIONS, SUPPORTED_PROTOCOL_VERSIONS, ToolRegistry, UNSUPPORTED_PROTOCOL_VERSION, extract_meta_protocol_version};
use semio_framework_os_mcp_workspace::gateway_backend::GatewayBackends;

//#region 🔖️Server

fn server_capabilities() -> serde_json::Value {
    serde_json::json!({
        "tools": { "listChanged": true },
        "resources": { "listChanged": true, "subscribe": true },
        "prompts": { "listChanged": true },
    })
}

enum DispatchOutcome {
    Result(serde_json::Value),
    Error(i64, String, Option<serde_json::Value>),
    NoResponse,
}

/// 🗼️ The single dual-era dispatcher — one `McpServer` per connection (stdio: one per process
/// lifetime). See this file's module doc for the era-detection contract.
/// 🧾️ One line describing a tool result for the shell's agent panel: the first text block, or —
/// for a result that carries only structured content — that content rendered as JSON. Never the
/// whole payload: `AgentConversation` truncates it, and the panel is a view, not a transport.
fn tool_result_summary(result: &CallToolResult) -> String {
    if let Some(text) = result.content.iter().find_map(|block| match block {
        ContentBlock::Text { text } => Some(text.clone()),
        _ => None,
    }) {
        return text;
    }
    match result.structured_content.as_ref() {
        Some(value) => value.to_string(),
        None => String::new(),
    }
}

pub struct McpServer {
    pub tools: Box<dyn ToolRegistry>,
    pub resources: Box<dyn ResourceRegistry>,
    pub prompts: Box<dyn PromptRegistry>,
    pub backend: Box<GatewayBackends>,
    server_name: String,
    server_version: String,
    pub(crate) era: Option<ProtocolEra>,
    pub(crate) negotiated_version: Option<String>,
    initialized: bool,
    /// 💬️ Where this server publishes what the agent is doing, for an attached shell to render.
    /// `None` is the ordinary case for a gateway with no `/bridge` (stdio) and for every test that
    /// does not care; it is never an error, and a tool call behaves identically either way.
    conversation: Option<Arc<AgentConversation>>,
    /// 🧭️ The live mirror of what the connected client advertised, shared with the transport that
    /// owns the server→client request channel. Always present; empty until `initialize` lands.
    pub client_features: Arc<ClientFeatures>,
    /// 🔔️ THIS connection's live `resources/subscribe` set — always present and always registered
    /// with the process-wide broker, so `"resources": {"subscribe": true}` is a promise this server
    /// keeps rather than advertises. With no notification sink bound (every test tier, and stdio
    /// before a transport publishes one) the set still records faithfully and simply delivers
    /// nothing.
    pub subscriptions: Arc<ResourceSubscriptions>,
    /// 📤️ Where this connection's unprompted notifications go, filled by whichever transport owns
    /// the socket.
    pub(crate) notifications: Option<NotificationSlot>,
}

impl McpServer {
    pub fn new(tools: Box<dyn ToolRegistry>, resources: Box<dyn ResourceRegistry>, prompts: Box<dyn PromptRegistry>, backend: Box<GatewayBackends>) -> Self {
        Self {
            tools,
            resources,
            prompts,
            backend,
            server_name: "semio".to_string(),
            server_version: env!("CARGO_PKG_VERSION").to_string(),
            era: None,
            negotiated_version: None,
            initialized: false,
            conversation: None,
            client_features: Arc::new(ClientFeatures::default()),
            subscriptions: ResourceSubscriptions::registered(),
            notifications: None,
        }
    }

    /// 📤️ Publishes this connection's server→client notification lane into the server, so
    /// `resources/updated`, `resources/list_changed` and `progress` reach the real client. Builder-
    /// shaped for the same reason [`McpServer::publishing_conversation_to`] is: every existing caller
    /// has no sink at all.
    #[must_use]
    pub fn publishing_notifications_into(mut self, slot: NotificationSlot) -> Self {
        self.subscriptions.bind_sink(slot.clone());
        self.notifications = Some(slot);
        self
    }

    /// 🔔️ This connection's live subscription set — what a test asserts `resources/subscribe`
    /// actually recorded.
    pub fn subscriptions(&self) -> &Arc<ResourceSubscriptions> {
        &self.subscriptions
    }

    /// 🧭️ The client-capability mirror this server fills at `initialize` — handed to the transport
    /// so an elicitation is attempted only against a client that advertised it.
    pub fn client_features(&self) -> Arc<ClientFeatures> {
        self.client_features.clone()
    }

    /// 💬️ Publishes this server's `tools/call` traffic onto the shell bridge as
    /// `AgentToolCall`/`AgentToolResult`/`AgentPresence` frames. Builder-shaped rather than a
    /// constructor argument because every existing caller (and the whole stdio path) has no bridge
    /// at all, and adding a parameter would force a `None` through dozens of call sites for a
    /// capability they never use.
    pub fn publishing_conversation_to(mut self, conversation: Arc<AgentConversation>) -> Self {
        self.conversation = Some(conversation);
        self
    }

    /// 💬️ The live conversation sink, if one was published into this server.
    pub fn conversation(&self) -> Option<&Arc<AgentConversation>> {
        self.conversation.as_ref()
    }

    /// 🏗️ `NullBackend` + empty in-memory registries — the default a bare `stdio` invocation boots
    /// with until a real backend is wired in.
    pub fn with_defaults() -> Self {
        Self::new(Box::new(InMemoryToolRegistry::new()), Box::new(InMemoryResourceRegistry::new()), Box::new(InMemoryPromptRegistry::new()), Box::new(GatewayBackends::Null(NullBackend)))
    }

    pub fn era(&self) -> Option<ProtocolEra> {
        self.era
    }

    pub fn negotiated_version(&self) -> Option<&str> {
        self.negotiated_version.as_deref()
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// 🚪️ Dispatches one request/notification, returning `None` for notifications (per JSON-RPC,
    /// they never get a response — success OR failure) and `Some` for everything with an id.
    pub fn dispatch(&mut self, request: &JsonRpcRequest) -> Option<JsonRpcResponse> {
        let id = request.id.clone();
        match self.dispatch_inner(request) {
            DispatchOutcome::NoResponse => None,
            DispatchOutcome::Result(value) => id.map(|id| JsonRpcResponse::result(id, value)),
            DispatchOutcome::Error(code, message, data) => id.map(|id| JsonRpcResponse::error(id, code, message, data)),
        }
    }

    /// 📦️ Dispatches a batch, dropping every notification's (non-)response — the JSON-RPC batch
    /// contract: an all-notification batch yields an empty vec.
    pub fn dispatch_batch(&mut self, requests: &[JsonRpcRequest]) -> Vec<JsonRpcResponse> {
        requests.iter().filter_map(|request| self.dispatch(request)).collect()
    }

    fn dispatch_inner(&mut self, request: &JsonRpcRequest) -> DispatchOutcome {
        match request.method.as_str() {
            METHOD_SERVER_DISCOVER => self.handle_server_discover(request),
            METHOD_INITIALIZE => self.handle_initialize(request),
            METHOD_NOTIFICATIONS_INITIALIZED => {
                self.initialized = true;
                DispatchOutcome::NoResponse
            }
            METHOD_PING => DispatchOutcome::Result(serde_json::json!({})),
            METHOD_NOTIFICATIONS_CANCELLED => self.handle_notifications_cancelled(request),
            other => self.dispatch_versioned(other, request),
        }
    }

    /// 🕵️ Every method past the handshake trio re-validates a per-request modern `_meta` version
    /// (present or not — a legacy client that already completed `initialize` sends none, and that is
    /// fine) before routing.
    fn dispatch_versioned(&mut self, method: &str, request: &JsonRpcRequest) -> DispatchOutcome {
        if let Some(requested) = extract_meta_protocol_version(request.params.as_ref()) {
            if let Some(outcome) = self.reject_unsupported_version(&requested) {
                return outcome;
            }
            self.era = Some(ProtocolEra::Modern);
            self.negotiated_version = Some(requested);
        }
        match method {
            METHOD_TOOLS_LIST => self.handle_tools_list(request),
            METHOD_TOOLS_CALL => self.handle_tools_call(request),
            METHOD_RESOURCES_LIST => self.handle_resources_list(request),
            METHOD_RESOURCES_TEMPLATES_LIST => self.handle_resources_templates_list(request),
            METHOD_RESOURCES_READ => self.handle_resources_read(request),
            METHOD_RESOURCES_SUBSCRIBE => self.handle_resources_subscribe(request),
            METHOD_RESOURCES_UNSUBSCRIBE => self.handle_resources_unsubscribe(request),
            METHOD_PROMPTS_LIST => self.handle_prompts_list(request),
            METHOD_PROMPTS_GET => self.handle_prompts_get(request),
            other => DispatchOutcome::Error(METHOD_NOT_FOUND, format!("method not found: {other}"), None),
        }
    }

    fn reject_unsupported_version(&self, requested: &str) -> Option<DispatchOutcome> {
        if SUPPORTED_PROTOCOL_VERSIONS.contains(&requested) {
            return None;
        }
        Some(DispatchOutcome::Error(UNSUPPORTED_PROTOCOL_VERSION, format!("unsupported protocol version: {requested}"), Some(serde_json::json!({ "supported": SUPPORTED_PROTOCOL_VERSIONS, "requested": requested }))))
    }

    fn handle_server_discover(&mut self, request: &JsonRpcRequest) -> DispatchOutcome {
        self.client_features.record(request.params.as_ref().and_then(|params| params.get("capabilities")));
        let negotiated = match extract_meta_protocol_version(request.params.as_ref()) {
            Some(requested) => {
                if let Some(outcome) = self.reject_unsupported_version(&requested) {
                    return outcome;
                }
                requested
            }
            None => SUPPORTED_PROTOCOL_VERSIONS[0].to_string(),
        };
        self.era = Some(ProtocolEra::Modern);
        self.negotiated_version = Some(negotiated.clone());
        DispatchOutcome::Result(serde_json::json!({
            "resultType": "complete",
            "protocolVersion": negotiated,
            "capabilities": server_capabilities(),
            "serverInfo": { "name": self.server_name, "version": self.server_version },
            "instructions": SERVER_INSTRUCTIONS,
        }))
    }

    fn handle_initialize(&mut self, request: &JsonRpcRequest) -> DispatchOutcome {
        self.client_features.record(request.params.as_ref().and_then(|params| params.get("capabilities")));
        let requested = request.params.as_ref().and_then(|params| params.get("protocolVersion")).and_then(|value| value.as_str()).unwrap_or("");
        let negotiated = if SUPPORTED_PROTOCOL_VERSIONS.contains(&requested) { requested.to_string() } else { SUPPORTED_PROTOCOL_VERSIONS[0].to_string() };
        self.era = Some(ProtocolEra::Legacy);
        self.negotiated_version = Some(negotiated.clone());
        DispatchOutcome::Result(serde_json::json!({
            "protocolVersion": negotiated,
            "capabilities": server_capabilities(),
            "serverInfo": { "name": self.server_name, "version": self.server_version },
            "instructions": SERVER_INSTRUCTIONS,
        }))
    }

    /// 📄️ The page offset a `*/list` request asked for. An absent `cursor` is page one; a cursor this
    /// server did not mint is `INVALID_PARAMS` (the spec's own requirement), never a silently-wrong
    /// page of results.
    fn requested_offset(request: &JsonRpcRequest) -> Result<usize, DispatchOutcome> {
        match request.params.as_ref().and_then(|params| params.get("cursor")) {
            None | Some(serde_json::Value::Null) => Ok(0),
            Some(serde_json::Value::String(cursor)) => decode_cursor(cursor).ok_or_else(|| DispatchOutcome::Error(INVALID_PARAMS, format!("unknown cursor: {cursor}"), Some(serde_json::json!({ "cursor": cursor })))),
            Some(other) => Err(DispatchOutcome::Error(INVALID_PARAMS, "cursor must be a string".to_string(), Some(serde_json::json!({ "cursor": other })))),
        }
    }

    /// 📄️ One page of an already-stably-ordered list, rendered into the result object `key` names,
    /// with `nextCursor` present only when there IS a next page.
    fn paged_result(key: &str, items: Vec<serde_json::Value>, offset: usize, ttl_ms: u64) -> DispatchOutcome {
        let page = paginate(items, offset, DEFAULT_PAGE_SIZE);
        let mut result = serde_json::json!({ "resultType": "complete", key: page.items, "ttlMs": ttl_ms, "cacheScope": "public" });
        if let (Some(object), Some(next)) = (result.as_object_mut(), page.next_cursor) {
            object.insert("nextCursor".to_string(), serde_json::json!(next));
        }
        DispatchOutcome::Result(result)
    }

    fn handle_tools_list(&self, request: &JsonRpcRequest) -> DispatchOutcome {
        let offset = match Self::requested_offset(request) {
            Ok(offset) => offset,
            Err(outcome) => return outcome,
        };
        let mut tools = self.tools.list();
        tools.sort_by(|a, b| a.name.cmp(&b.name));
        Self::paged_result("tools", tools.into_iter().map(|tool| serde_json::to_value(tool).unwrap_or(serde_json::Value::Null)).collect(), offset, 300_000)
    }

    /// 📈️ The client's own opaque `_meta.progressToken`, bound to this connection's notification
    /// lane. Absent token, or no sink bound, is the ordinary call: no scope is entered and the job
    /// registry's poll-based surface is unchanged.
    fn progress_binding(&self, request: &JsonRpcRequest, params: &serde_json::Value) -> Option<ProgressBinding> {
        let token = params.get("_meta").and_then(|meta| meta.get("progressToken")).cloned()?;
        if token.is_null() {
            return None;
        }
        let slot = self.notifications.clone()?;
        Some(ProgressBinding { token, slot })
    }

    /// 📈️ Held across the whole call: every job the tool mints while this guard is alive is bound
    /// to the client's token, so its progress is PUSHED as `notifications/progress` instead of
    /// only being readable by polling `job_get`.
    ///
    /// 💬️ The ONE real dispatch point every tool call passes through — so the shell's agent
    /// panel shows the agent's actual calls, in order, with their real arguments and outcomes,
    /// rather than a second bookkeeping path that could drift from what ran.
    /// 💬️ …except for a tool whose own result IS a conversation frame: `conversation_reply`
    /// publishes the agent's prose itself, and a tool-call row carrying the same sentence as its
    /// `arguments` would print every turn twice (`SELF_PUBLISHING_CONVERSATION_TOOLS`).
    ///
    /// 🔔️ A committed mutation, an undo/redo, a created artifact — the declared table in
    /// `📣️notify` says which tool changed which resource, and every connection subscribed
    /// to one of those URIs is told. This is the one call site that makes
    /// `"resources": {"subscribe": true}` real.
    fn handle_tools_call(&self, request: &JsonRpcRequest) -> DispatchOutcome {
        let Some(params) = request.params.as_ref() else { return DispatchOutcome::Error(INVALID_PARAMS, "tools/call requires params".to_string(), None) };
        let Some(name) = params.get("name").and_then(|value| value.as_str()) else { return DispatchOutcome::Error(INVALID_PARAMS, "tools/call requires params.name".to_string(), None) };
        let arguments = params.get("arguments").cloned().unwrap_or(serde_json::Value::Null);
        let _request = enter_request_scope(request.id.as_ref().map(|id| serde_json::to_value(id).unwrap_or(serde_json::Value::Null)).as_ref());
        let _progress = enter_progress_scope(self.progress_binding(request, params));
        let publishes_itself = SELF_PUBLISHING_CONVERSATION_TOOLS.contains(&name);
        let invocation = self.conversation.as_ref().filter(|_| !publishes_itself).map(|conversation| conversation.begin_tool_call(name, &arguments));
        let outcome = self.tools.call(name, arguments);
        if let (Some(conversation), Some(invocation_id)) = (self.conversation.as_ref(), invocation.as_deref()) {
            let (ok, summary) = match &outcome {
                Ok(result) => (!result.is_error, tool_result_summary(result)),
                Err(error) => (false, error.message.clone()),
            };
            conversation.finish_tool_call(invocation_id, name, ok, &summary);
        }
        match outcome {
            Ok(result) => {
                broadcast_tool_result_changes(name, result.is_error, result.structured_content.as_ref());
                DispatchOutcome::Result(serde_json::json!({
                    "resultType": "complete",
                    "content": result.content,
                    "structuredContent": result.structured_content,
                    "isError": result.is_error,
                }))
            }
            Err(error) => {
                let (code, message, data) = error.to_json_rpc_parts();
                DispatchOutcome::Error(code, message, Some(data))
            }
        }
    }

    /// 🛑️ MCP's own request-scoped cancellation, mapped onto the very `job_registry().request_cancel`
    /// path the `job_cancel` tool uses: every job minted while the named request was running is
    /// asked to stop. A request that minted no job, or one already finished, is a silent no-op —
    /// the spec forbids answering a notification either way.
    fn handle_notifications_cancelled(&self, request: &JsonRpcRequest) -> DispatchOutcome {
        if let Some(request_id) = request.params.as_ref().and_then(|params| params.get("requestId")) {
            cancel_request(request_id);
        }
        DispatchOutcome::NoResponse
    }

    fn handle_resources_list(&self, request: &JsonRpcRequest) -> DispatchOutcome {
        let offset = match Self::requested_offset(request) {
            Ok(offset) => offset,
            Err(outcome) => return outcome,
        };
        Self::paged_result("resources", self.resources.list().into_iter().map(|resource| serde_json::to_value(resource).unwrap_or(serde_json::Value::Null)).collect(), offset, 300_000)
    }

    fn handle_resources_templates_list(&self, request: &JsonRpcRequest) -> DispatchOutcome {
        let offset = match Self::requested_offset(request) {
            Ok(offset) => offset,
            Err(outcome) => return outcome,
        };
        Self::paged_result("resourceTemplates", self.resources.templates().into_iter().map(|template| serde_json::to_value(template).unwrap_or(serde_json::Value::Null)).collect(), offset, 300_000)
    }

    fn handle_resources_read(&self, request: &JsonRpcRequest) -> DispatchOutcome {
        let Some(uri) = request.params.as_ref().and_then(|params| params.get("uri")).and_then(|value| value.as_str()) else {
            return DispatchOutcome::Error(INVALID_PARAMS, "resources/read requires params.uri".to_string(), None);
        };
        match self.resources.read(uri) {
            Ok(contents) => DispatchOutcome::Result(serde_json::json!({ "resultType": "complete", "contents": contents, "ttlMs": 60_000, "cacheScope": "private" })),
            Err(error) => {
                let (code, message, data) = error.to_json_rpc_parts();
                DispatchOutcome::Error(code, message, Some(data))
            }
        }
    }

    /// 🔔️ The registry validated that this URI is one it can serve; recording it here is what
    /// makes a later `notifications/resources/updated` actually reach THIS connection.
    fn handle_resources_subscribe(&self, request: &JsonRpcRequest) -> DispatchOutcome {
        let Some(uri) = request.params.as_ref().and_then(|params| params.get("uri")).and_then(|value| value.as_str()) else {
            return DispatchOutcome::Error(INVALID_PARAMS, "resources/subscribe requires params.uri".to_string(), None);
        };
        match self.resources.subscribe(uri) {
            Ok(()) => {
                self.subscriptions.subscribe(uri);
                DispatchOutcome::Result(serde_json::json!({}))
            }
            Err(error) => {
                let (code, message, data) = error.to_json_rpc_parts();
                DispatchOutcome::Error(code, message, Some(data))
            }
        }
    }

    fn handle_resources_unsubscribe(&self, request: &JsonRpcRequest) -> DispatchOutcome {
        let Some(uri) = request.params.as_ref().and_then(|params| params.get("uri")).and_then(|value| value.as_str()) else {
            return DispatchOutcome::Error(INVALID_PARAMS, "resources/unsubscribe requires params.uri".to_string(), None);
        };
        match self.resources.unsubscribe(uri) {
            Ok(()) => {
                self.subscriptions.unsubscribe(uri);
                DispatchOutcome::Result(serde_json::json!({}))
            }
            Err(error) => {
                let (code, message, data) = error.to_json_rpc_parts();
                DispatchOutcome::Error(code, message, Some(data))
            }
        }
    }

    fn handle_prompts_list(&self, request: &JsonRpcRequest) -> DispatchOutcome {
        let offset = match Self::requested_offset(request) {
            Ok(offset) => offset,
            Err(outcome) => return outcome,
        };
        Self::paged_result("prompts", self.prompts.list().into_iter().map(|prompt| serde_json::to_value(prompt).unwrap_or(serde_json::Value::Null)).collect(), offset, 600_000)
    }

    fn handle_prompts_get(&self, request: &JsonRpcRequest) -> DispatchOutcome {
        let Some(params) = request.params.as_ref() else { return DispatchOutcome::Error(INVALID_PARAMS, "prompts/get requires params".to_string(), None) };
        let Some(name) = params.get("name").and_then(|value| value.as_str()) else { return DispatchOutcome::Error(INVALID_PARAMS, "prompts/get requires params.name".to_string(), None) };
        let arguments = params.get("arguments").cloned();
        match self.prompts.get(name, arguments) {
            Ok(result) => DispatchOutcome::Result(serde_json::json!({ "resultType": "complete", "description": result.description, "messages": result.messages })),
            Err(error) => {
                let (code, message, data) = error.to_json_rpc_parts();
                DispatchOutcome::Error(code, message, Some(data))
            }
        }
    }
}

//#endregion 🔖️Server
