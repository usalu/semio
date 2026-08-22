//! 🧭️ Dual-era JSON-RPC 2.0 + MCP protocol core (`📓️design-decisions.md` D1 is the controlling
//! requirement — read it before touching this file). A single [`McpServer`] dispatcher serves BOTH
//! eras through ONE handler layer: **modern** (`2026-07-28`+) is stateless — every request carries
//! its protocol version in `_meta.io.modelcontextprotocol/protocolVersion`, no `initialize`, no
//! session; **legacy** (`2025-11-25`, `2025-06-18`) negotiates once via `initialize`/
//! `notifications/initialized`. Which era a connection speaks is decided by its OPENING request
//! (`server/discover`/`_meta` ⇒ modern, `initialize` ⇒ legacy) and recorded on the dispatcher — every
//! later request on that connection is routed through the exact same `tools/*`/`resources/*`/
//! `prompts/*` handlers regardless of era. This is a spec interoperability requirement (every MCP
//! client installed in this repo today is legacy-era — see D1's SDK survey), not a compatibility
//! shim: CLAUDE.md's "no legacy support" rule does not apply to an external protocol we do not own.

use crate::workspace::GatewayBackends;
use schemars::JsonSchema;
use semio_framework_dispatch_macros::dyn_enum;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};

pub use crate::errors::{GatewayError, GatewayErrorCode};
pub use crate::schema::{ContextSummary, InvocationReport, PreparedActionReport, RevisionStamp, SearchHit};

//#region 🔖️JsonRpcCodes
/// 🔢️ The five JSON-RPC 2.0 standard error codes plus MCP's own `UnsupportedProtocolVersionError`
/// (`luna-mcpspec-audit.md` §"Error Codes (JSON-RPC)").
pub const PARSE_ERROR: i64 = -32700;
pub const INVALID_REQUEST: i64 = -32600;
pub const METHOD_NOT_FOUND: i64 = -32601;
pub const INVALID_PARAMS: i64 = -32602;
pub const INTERNAL_ERROR: i64 = -32603;
/// 🚧️ MCP `UnsupportedProtocolVersionError` — `data: {supported, requested}` always accompanies it.
pub const UNSUPPORTED_PROTOCOL_VERSION: i64 = -32022;
//#endregion 🔖️JsonRpcCodes

//#region 🔖️ProtocolVersions
/// 📚️ Supported protocol versions, newest first (D1) — modern era is index 0, legacy eras follow.
pub const SUPPORTED_PROTOCOL_VERSIONS: [&str; 3] = ["2026-07-28", "2025-11-25", "2025-06-18"];

/// 🔑️ The `_meta` key a modern-era request carries its protocol version under.
pub const META_PROTOCOL_VERSION_KEY: &str = "io.modelcontextprotocol/protocolVersion";

/// 🕵️ Reads `params._meta."io.modelcontextprotocol/protocolVersion"` if present — its presence is
/// what distinguishes a modern-era request from a legacy one at the framing level.
pub fn extract_meta_protocol_version(params: Option<&serde_json::Value>) -> Option<String> {
    params?.get("_meta")?.get(META_PROTOCOL_VERSION_KEY)?.as_str().map(str::to_string)
}
//#endregion 🔖️ProtocolVersions

//#region 🔖️ProtocolEra
/// 🕰️ Which MCP era a connection is speaking — decided once, from the opening request, and recorded
/// on the dispatcher (see this file's module doc).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolEra {
    Modern,
    Legacy,
}
//#endregion 🔖️ProtocolEra

//#region 🔖️JsonRpcId
/// 🆔️ A JSON-RPC request/response id — `Null` is distinct from an ABSENT id (which makes a request a
/// notification, modeled as `JsonRpcRequest.id: None`, not `Some(JsonRpcId::Null)`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonRpcId {
    Number(i64),
    String(String),
    Null,
}
//#endregion 🔖️JsonRpcId

/// 🕳️ Deserializes a PRESENT field as `Some(T::deserialize(..))`, including when its value is JSON
/// `null` — the plain `#[serde(default)]` path for an `Option<T>` field collapses an explicit `null`
/// down to `None` before `T::deserialize` ever runs (serde's blanket `Option<T>` impl treats `null`
/// as "absent"), which would make [`JsonRpcId::Null`] unreachable. Only reached when the field key IS
/// present in the object — `#[serde(default)]` on the same field covers the truly-absent case.
fn deserialize_some<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
}

//#region 🔖️JsonRpcRequest
/// 📨️ A JSON-RPC 2.0 request. `id: None` (the field entirely absent on the wire) makes it a
/// notification — no response is ever sent for one, success or failure. An explicit `"id":null`
/// deserializes to `Some(JsonRpcId::Null)`, distinct from absence (see [`deserialize_some`]).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    #[serde(default, deserialize_with = "deserialize_some", skip_serializing_if = "Option::is_none")]
    pub id: Option<JsonRpcId>,
    pub method: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

impl JsonRpcRequest {
    pub fn is_notification(&self) -> bool {
        self.id.is_none()
    }
}

/// 📦️ One line of stdio input parses to either a single request or a JSON-RPC batch (an array of
/// requests/notifications) — untagged so the same wire bytes round-trip through whichever shape they
/// arrived as.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonRpcIncoming {
    Batch(Vec<JsonRpcRequest>),
    Single(JsonRpcRequest),
}
//#endregion 🔖️JsonRpcRequest

//#region 🔖️JsonRpcResponse
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonRpcErrorObject {
    pub code: i64,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonRpcOutcome {
    Result { result: serde_json::Value },
    Error { error: JsonRpcErrorObject },
}

/// 📨️ A JSON-RPC 2.0 response — always carries the id of the request it answers (notifications never
/// get one of these at all, see [`McpServer::dispatch`]).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: JsonRpcId,
    #[serde(flatten)]
    pub outcome: JsonRpcOutcome,
}

impl JsonRpcResponse {
    pub fn result(id: JsonRpcId, value: serde_json::Value) -> Self {
        Self { jsonrpc: "2.0".to_string(), id, outcome: JsonRpcOutcome::Result { result: value } }
    }

    pub fn error(id: JsonRpcId, code: i64, message: impl Into<String>, data: Option<serde_json::Value>) -> Self {
        Self { jsonrpc: "2.0".to_string(), id, outcome: JsonRpcOutcome::Error { error: JsonRpcErrorObject { code, message: message.into(), data } } }
    }

    pub fn is_error(&self) -> bool {
        matches!(self.outcome, JsonRpcOutcome::Error { .. })
    }
}

/// 📤️ A `JsonRpcNotification` the SERVER sends unprompted (`notifications/tools/list_changed`, …) —
/// never carries an id, matching [`JsonRpcRequest`]'s notification shape exactly.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonRpcNotification {
    pub jsonrpc: String,
    pub method: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

impl JsonRpcNotification {
    pub fn new(method: impl Into<String>, params: Option<serde_json::Value>) -> Self {
        Self { jsonrpc: "2.0".to_string(), method: method.into(), params }
    }
}

pub const NOTIFICATION_TOOLS_LIST_CHANGED: &str = "notifications/tools/list_changed";
pub const NOTIFICATION_RESOURCES_LIST_CHANGED: &str = "notifications/resources/list_changed";
pub const NOTIFICATION_RESOURCES_UPDATED: &str = "notifications/resources/updated";
//#endregion 🔖️JsonRpcResponse

//#region 🔖️Methods
pub const METHOD_SERVER_DISCOVER: &str = "server/discover";
pub const METHOD_INITIALIZE: &str = "initialize";
pub const METHOD_NOTIFICATIONS_INITIALIZED: &str = "notifications/initialized";
pub const METHOD_TOOLS_LIST: &str = "tools/list";
pub const METHOD_TOOLS_CALL: &str = "tools/call";
pub const METHOD_RESOURCES_LIST: &str = "resources/list";
pub const METHOD_RESOURCES_TEMPLATES_LIST: &str = "resources/templates/list";
pub const METHOD_RESOURCES_READ: &str = "resources/read";
pub const METHOD_RESOURCES_SUBSCRIBE: &str = "resources/subscribe";
pub const METHOD_RESOURCES_UNSUBSCRIBE: &str = "resources/unsubscribe";
pub const METHOD_PROMPTS_LIST: &str = "prompts/list";
pub const METHOD_PROMPTS_GET: &str = "prompts/get";
pub const METHOD_PING: &str = "ping";
pub const METHOD_NOTIFICATIONS_CANCELLED: &str = "notifications/cancelled";
//#endregion 🔖️Methods

//#region 🔖️ToolNameCharset
/// ✅️ `^[a-zA-Z0-9_-]{1,64}$` (`luna-mcpspec-audit.md` §"Tool Names") — enforced at registration
/// time by [`InMemoryToolRegistry::register`], not at call time (a caller invoking an already-invalid
/// name simply gets `NOT_FOUND`, since it was never accepted into the registry).
pub fn is_valid_tool_name(name: &str) -> bool {
    let length = name.chars().count();
    length >= 1 && length <= 64 && name.chars().all(|character| character.is_ascii_alphanumeric() || character == '_' || character == '-')
}
//#endregion 🔖️ToolNameCharset

//#region 🔖️Content
/// 📄️ One block of unstructured tool/prompt content (`luna-mcpspec-audit.md` §"Tools Protocol").
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    Text {
        text: String,
    },
    Image {
        data: String,
        #[serde(rename = "mimeType")]
        mime_type: String,
    },
    Audio {
        data: String,
        #[serde(rename = "mimeType")]
        mime_type: String,
    },
    ResourceLink {
        uri: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(default, rename = "mimeType", skip_serializing_if = "Option::is_none")]
        mime_type: Option<String>,
    },
    Resource {
        uri: String,
        #[serde(default, rename = "mimeType", skip_serializing_if = "Option::is_none")]
        mime_type: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        text: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        blob: Option<String>,
    },
}
//#endregion 🔖️Content

//#region 🔖️Tools
/// 🔧️ One registered tool's descriptor (`luna-mcpspec-audit.md` §"Tools Protocol" `tools/list`
/// shape) — `name` must satisfy [`is_valid_tool_name`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Tool {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub input_schema: serde_json::Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<serde_json::Value>,
    #[serde(default, rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

impl Tool {
    pub fn new(name: impl Into<String>, input_schema: serde_json::Value) -> Self {
        Self { name: name.into(), title: None, description: None, input_schema, output_schema: None, annotations: None, meta: None }
    }
}

/// 🧾️ Result of `tools/call` — `is_error: true` is a TOOL execution failure (actionable, for LLM
/// self-correction); a JSON-RPC error response is a PROTOCOL failure (unknown tool, malformed
/// request) instead. See [`McpServer::handle_tools_call`] for the single place this choice is made.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CallToolResult {
    pub content: Vec<ContentBlock>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub structured_content: Option<serde_json::Value>,
    pub is_error: bool,
}

impl CallToolResult {
    pub fn ok(content: Vec<ContentBlock>, structured_content: Option<serde_json::Value>) -> Self {
        Self { content, structured_content, is_error: false }
    }

    /// 🚧️ A TOOL-execution failure — the JSON-RPC envelope stays a success; only `is_error` says the
    /// tool itself failed. Content carries the human-readable message, `structuredContent` the full
    /// [`GatewayError`] payload.
    pub fn tool_error(error: &GatewayError) -> Self {
        Self { content: vec![ContentBlock::Text { text: error.message.clone() }], structured_content: Some(error.to_tool_error_payload()), is_error: true }
    }
}

/// 🗂️ The seam through which tool execution is provided — an in-memory default here, real
/// capability-catalog/action providers (P1b's `catalog`/P2/P6) implement this without touching
/// `protocol` at all.
pub trait ToolRegistry: Send + Sync {
    fn list(&self) -> Vec<Tool>;
    /// ↩️ `Err` here is a PROTOCOL failure (the name doesn't resolve to a registered tool at all) —
    /// a registered tool's own business failure is `Ok(CallToolResult::tool_error(..))` instead.
    fn call(&self, name: &str, arguments: serde_json::Value) -> Result<CallToolResult, GatewayError>;
}

type ToolHandler = Arc<dyn Fn(serde_json::Value) -> CallToolResult + Send + Sync>;

/// 🗃️ In-memory [`ToolRegistry`] — the default this crate ships; `register` enforces
/// [`is_valid_tool_name`] so an invalid name can never enter the catalog.
#[derive(Default)]
pub struct InMemoryToolRegistry {
    tools: BTreeMap<String, Tool>,
    handlers: BTreeMap<String, ToolHandler>,
}

impl InMemoryToolRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, tool: Tool, handler: impl Fn(serde_json::Value) -> CallToolResult + Send + Sync + 'static) -> Result<(), GatewayError> {
        if !is_valid_tool_name(&tool.name) {
            return Err(GatewayError::new(GatewayErrorCode::InputInvalid, format!("tool name `{}` violates ^[a-zA-Z0-9_-]{{1,64}}$", tool.name)));
        }
        // 🧷️ Normalize here rather than at each call site: this is the one choke point every tool
        // passes through, so no future registration can reintroduce a boolean sub-schema that makes
        // the official SDK reject the entire `tools/list` response. See `schema::normalize_boolean_subschemas`.
        let mut tool = tool;
        crate::schema::convert_draft07_to_2020_12(&mut tool.input_schema);
        crate::schema::normalize_boolean_subschemas(&mut tool.input_schema);
        if let Some(output_schema) = tool.output_schema.as_mut() {
            crate::schema::convert_draft07_to_2020_12(output_schema);
            crate::schema::normalize_boolean_subschemas(output_schema);
        }
        self.handlers.insert(tool.name.clone(), Arc::new(handler));
        self.tools.insert(tool.name.clone(), tool);
        Ok(())
    }
}

impl ToolRegistry for InMemoryToolRegistry {
    fn list(&self) -> Vec<Tool> {
        self.tools.values().cloned().collect()
    }

    fn call(&self, name: &str, arguments: serde_json::Value) -> Result<CallToolResult, GatewayError> {
        match self.handlers.get(name) {
            Some(handler) => Ok(handler(arguments)),
            None => Err(GatewayError::new(GatewayErrorCode::NotFound, format!("unknown tool: {name}"))),
        }
    }
}
//#endregion 🔖️Tools

//#region 🔖️Resources
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub uri: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResourceTemplate {
    #[serde(rename = "uriTemplate")]
    pub uri_template: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResourceContent {
    pub uri: String,
    #[serde(default, rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blob: Option<String>,
}

pub trait ResourceRegistry: Send + Sync {
    fn list(&self) -> Vec<Resource>;
    fn templates(&self) -> Vec<ResourceTemplate>;
    fn read(&self, uri: &str) -> Result<Vec<ResourceContent>, GatewayError>;
    fn subscribe(&self, uri: &str) -> Result<(), GatewayError>;
    fn unsubscribe(&self, uri: &str) -> Result<(), GatewayError>;
}

/// 🗃️ In-memory [`ResourceRegistry`] default — `semio://audit/*`/`semio://capability/*` providers
/// (P1b/P2/P6) implement the trait directly instead of extending this one.
#[derive(Default)]
pub struct InMemoryResourceRegistry {
    resources: BTreeMap<String, Resource>,
    templates: Vec<ResourceTemplate>,
    contents: BTreeMap<String, Vec<ResourceContent>>,
    subscriptions: Mutex<BTreeSet<String>>,
}

impl InMemoryResourceRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, resource: Resource, contents: Vec<ResourceContent>) {
        self.contents.insert(resource.uri.clone(), contents);
        self.resources.insert(resource.uri.clone(), resource);
    }

    pub fn register_template(&mut self, template: ResourceTemplate) {
        self.templates.push(template);
    }

    pub fn subscribed_uris(&self) -> Vec<String> {
        self.subscriptions.lock().expect("subscriptions lock poisoned").iter().cloned().collect()
    }
}

impl ResourceRegistry for InMemoryResourceRegistry {
    fn list(&self) -> Vec<Resource> {
        self.resources.values().cloned().collect()
    }

    fn templates(&self) -> Vec<ResourceTemplate> {
        self.templates.clone()
    }

    fn read(&self, uri: &str) -> Result<Vec<ResourceContent>, GatewayError> {
        self.contents.get(uri).cloned().ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("unknown resource: {uri}")))
    }

    fn subscribe(&self, uri: &str) -> Result<(), GatewayError> {
        self.subscriptions.lock().expect("subscriptions lock poisoned").insert(uri.to_string());
        Ok(())
    }

    fn unsubscribe(&self, uri: &str) -> Result<(), GatewayError> {
        self.subscriptions.lock().expect("subscriptions lock poisoned").remove(uri);
        Ok(())
    }
}
//#endregion 🔖️Resources

//#region 🔖️Prompts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PromptArgument {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub required: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Prompt {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub arguments: Vec<PromptArgument>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct PromptMessage {
    pub role: String,
    pub content: ContentBlock,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct PromptGetResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub messages: Vec<PromptMessage>,
}

pub trait PromptRegistry: Send + Sync {
    fn list(&self) -> Vec<Prompt>;
    fn get(&self, name: &str, arguments: Option<serde_json::Value>) -> Result<PromptGetResult, GatewayError>;
}

type PromptHandler = Arc<dyn Fn(Option<serde_json::Value>) -> Result<PromptGetResult, GatewayError> + Send + Sync>;

#[derive(Default)]
pub struct InMemoryPromptRegistry {
    prompts: BTreeMap<String, Prompt>,
    handlers: BTreeMap<String, PromptHandler>,
}

impl InMemoryPromptRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, prompt: Prompt, handler: impl Fn(Option<serde_json::Value>) -> Result<PromptGetResult, GatewayError> + Send + Sync + 'static) {
        self.handlers.insert(prompt.name.clone(), Arc::new(handler));
        self.prompts.insert(prompt.name.clone(), prompt);
    }
}

impl PromptRegistry for InMemoryPromptRegistry {
    fn list(&self) -> Vec<Prompt> {
        self.prompts.values().cloned().collect()
    }

    fn get(&self, name: &str, arguments: Option<serde_json::Value>) -> Result<PromptGetResult, GatewayError> {
        match self.handlers.get(name) {
            Some(handler) => handler(arguments),
            None => Err(GatewayError::new(GatewayErrorCode::NotFound, format!("unknown prompt: {name}"))),
        }
    }
}
//#endregion 🔖️Prompts

//#region 🔖️Backend
/// 🔌️ Everything a real OS/plugin integration provides — later packets (P1b/P2/P6) implement this
/// against the live plugin host; THIS crate never references `semio-framework`,
/// `semio-framework-os-kernel`, the plugin host, the channel, or the actor crate (they are mid-rewrite
/// by the peer `MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME` ticket — §2.6 of this packet's brief).
// 🔀️ dedyn-fw-os-misc, O1/R11: closed 3-implementor set (`NullBackend`, `HeadlessWorkspace`, its
// `Arc<HeadlessWorkspace>` delegation impl) — `#[dyn_enum]` here + `dyn_enum_close!` at
// `🏠️workspace`'s `GatewayBackends` (defined alongside `HeadlessWorkspace`, the module both non-Null
// variants live in) closes it into an enum instead of `Box<dyn GatewayBackend>`.
#[dyn_enum]
pub trait GatewayBackend: Send + Sync {
    fn resolve_context(&self, principal: &str) -> Result<ContextSummary, GatewayError>;
    fn search_capabilities(&self, query: &str) -> Result<Vec<SearchHit>, GatewayError>;
    fn describe_capabilities(&self, capability_id: &str) -> Result<serde_json::Value, GatewayError>;
    fn prepare_action(&self, capability_id: &str, input: serde_json::Value, expected_revision: Option<RevisionStamp>) -> Result<PreparedActionReport, GatewayError>;
    fn invoke_action(&self, prepared_handle: &str, idempotency_key: Option<&str>) -> Result<InvocationReport, GatewayError>;
    fn read_resource(&self, uri: &str) -> Result<Vec<ResourceContent>, GatewayError>;
    fn list_resources(&self) -> Result<Vec<Resource>, GatewayError>;
}

/// 🕳️ Answers `PLUGIN_UNAVAILABLE` for every single-target operation and empty for every list
/// operation — the default backend until a real one (P1b+) is wired in.
pub struct NullBackend;

fn plugin_unavailable(what: &str) -> GatewayError {
    GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("{what}: no backend wired yet")).retryable()
}

impl GatewayBackend for NullBackend {
    fn resolve_context(&self, _principal: &str) -> Result<ContextSummary, GatewayError> {
        Err(plugin_unavailable("resolve_context"))
    }

    fn search_capabilities(&self, _query: &str) -> Result<Vec<SearchHit>, GatewayError> {
        Ok(Vec::new())
    }

    fn describe_capabilities(&self, _capability_id: &str) -> Result<serde_json::Value, GatewayError> {
        Err(plugin_unavailable("describe_capabilities"))
    }

    fn prepare_action(&self, _capability_id: &str, _input: serde_json::Value, _expected_revision: Option<RevisionStamp>) -> Result<PreparedActionReport, GatewayError> {
        Err(plugin_unavailable("prepare_action"))
    }

    fn invoke_action(&self, _prepared_handle: &str, _idempotency_key: Option<&str>) -> Result<InvocationReport, GatewayError> {
        Err(plugin_unavailable("invoke_action"))
    }

    fn read_resource(&self, _uri: &str) -> Result<Vec<ResourceContent>, GatewayError> {
        Err(plugin_unavailable("read_resource"))
    }

    fn list_resources(&self) -> Result<Vec<Resource>, GatewayError> {
        Ok(Vec::new())
    }
}
//#endregion 🔖️Backend

//#region 🔖️CatalogHash
/// #️⃣️ Deterministic hash of a tool catalog's names — `context.resolve`'s published `catalogHash`
/// (`📋️master.md` §"MCP tool names": "catalog hash published in `context.resolve`") changes if and
/// only if the promoted/core tool SET changes, independent of description/schema edits to an
/// unchanged name set.
pub fn compute_catalog_hash(tools: &[Tool]) -> String {
    let mut names: Vec<&str> = tools.iter().map(|tool| tool.name.as_str()).collect();
    names.sort_unstable();
    let joined = names.join("\u{0}");
    framework_hash::hash_bytes(joined.as_bytes())
}
//#endregion 🔖️CatalogHash

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
pub struct McpServer {
    pub tools: Box<dyn ToolRegistry>,
    pub resources: Box<dyn ResourceRegistry>,
    pub prompts: Box<dyn PromptRegistry>,
    pub backend: Box<GatewayBackends>,
    server_name: String,
    server_version: String,
    era: Option<ProtocolEra>,
    negotiated_version: Option<String>,
    initialized: bool,
}

impl McpServer {
    pub fn new(tools: Box<dyn ToolRegistry>, resources: Box<dyn ResourceRegistry>, prompts: Box<dyn PromptRegistry>, backend: Box<GatewayBackends>) -> Self {
        Self { tools, resources, prompts, backend, server_name: "semio-os-mcp".to_string(), server_version: env!("CARGO_PKG_VERSION").to_string(), era: None, negotiated_version: None, initialized: false }
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
            METHOD_NOTIFICATIONS_CANCELLED => DispatchOutcome::NoResponse,
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
            METHOD_TOOLS_LIST => self.handle_tools_list(),
            METHOD_TOOLS_CALL => self.handle_tools_call(request),
            METHOD_RESOURCES_LIST => self.handle_resources_list(),
            METHOD_RESOURCES_TEMPLATES_LIST => self.handle_resources_templates_list(),
            METHOD_RESOURCES_READ => self.handle_resources_read(request),
            METHOD_RESOURCES_SUBSCRIBE => self.handle_resources_subscribe(request),
            METHOD_RESOURCES_UNSUBSCRIBE => self.handle_resources_unsubscribe(request),
            METHOD_PROMPTS_LIST => self.handle_prompts_list(),
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
        }))
    }

    fn handle_initialize(&mut self, request: &JsonRpcRequest) -> DispatchOutcome {
        let requested = request.params.as_ref().and_then(|params| params.get("protocolVersion")).and_then(|value| value.as_str()).unwrap_or("");
        let negotiated = if SUPPORTED_PROTOCOL_VERSIONS.contains(&requested) { requested.to_string() } else { SUPPORTED_PROTOCOL_VERSIONS[0].to_string() };
        self.era = Some(ProtocolEra::Legacy);
        self.negotiated_version = Some(negotiated.clone());
        DispatchOutcome::Result(serde_json::json!({
            "protocolVersion": negotiated,
            "capabilities": server_capabilities(),
            "serverInfo": { "name": self.server_name, "version": self.server_version },
        }))
    }

    fn handle_tools_list(&self) -> DispatchOutcome {
        let mut tools = self.tools.list();
        tools.sort_by(|a, b| a.name.cmp(&b.name));
        DispatchOutcome::Result(serde_json::json!({ "resultType": "complete", "tools": tools, "ttlMs": 300_000, "cacheScope": "public" }))
    }

    fn handle_tools_call(&self, request: &JsonRpcRequest) -> DispatchOutcome {
        let Some(params) = request.params.as_ref() else { return DispatchOutcome::Error(INVALID_PARAMS, "tools/call requires params".to_string(), None) };
        let Some(name) = params.get("name").and_then(|value| value.as_str()) else { return DispatchOutcome::Error(INVALID_PARAMS, "tools/call requires params.name".to_string(), None) };
        let arguments = params.get("arguments").cloned().unwrap_or(serde_json::Value::Null);
        match self.tools.call(name, arguments) {
            Ok(result) => DispatchOutcome::Result(serde_json::json!({
                "resultType": "complete",
                "content": result.content,
                "structuredContent": result.structured_content,
                "isError": result.is_error,
            })),
            Err(error) => {
                let (code, message, data) = error.to_json_rpc_parts();
                DispatchOutcome::Error(code, message, Some(data))
            }
        }
    }

    fn handle_resources_list(&self) -> DispatchOutcome {
        DispatchOutcome::Result(serde_json::json!({ "resultType": "complete", "resources": self.resources.list(), "ttlMs": 300_000, "cacheScope": "public" }))
    }

    fn handle_resources_templates_list(&self) -> DispatchOutcome {
        DispatchOutcome::Result(serde_json::json!({ "resultType": "complete", "resourceTemplates": self.resources.templates(), "ttlMs": 300_000, "cacheScope": "public" }))
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

    fn handle_resources_subscribe(&self, request: &JsonRpcRequest) -> DispatchOutcome {
        let Some(uri) = request.params.as_ref().and_then(|params| params.get("uri")).and_then(|value| value.as_str()) else {
            return DispatchOutcome::Error(INVALID_PARAMS, "resources/subscribe requires params.uri".to_string(), None);
        };
        match self.resources.subscribe(uri) {
            Ok(()) => DispatchOutcome::Result(serde_json::json!({})),
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
            Ok(()) => DispatchOutcome::Result(serde_json::json!({})),
            Err(error) => {
                let (code, message, data) = error.to_json_rpc_parts();
                DispatchOutcome::Error(code, message, Some(data))
            }
        }
    }

    fn handle_prompts_list(&self) -> DispatchOutcome {
        DispatchOutcome::Result(serde_json::json!({ "resultType": "complete", "prompts": self.prompts.list(), "ttlMs": 600_000, "cacheScope": "public" }))
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

//#region 🧪️Tests
#[cfg(test)]
mod quick {
    use super::*;

    fn request(id: Option<i64>, method: &str, params: Option<serde_json::Value>) -> JsonRpcRequest {
        JsonRpcRequest { jsonrpc: "2.0".to_string(), id: id.map(JsonRpcId::Number), method: method.to_string(), params }
    }

    fn modern_meta(version: &str) -> serde_json::Value {
        serde_json::json!({ "_meta": { META_PROTOCOL_VERSION_KEY: version } })
    }

    //#region 🔖️FramingRoundTrips
    #[test]
    fn single_request_round_trips_through_json() {
        let original = request(Some(1), "ping", None);
        let json = serde_json::to_string(&original).unwrap();
        let parsed: JsonRpcIncoming = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, JsonRpcIncoming::Single(original));
    }

    #[test]
    fn batch_round_trips_through_json() {
        let batch = vec![request(Some(1), "ping", None), request(Some(2), "server/discover", Some(modern_meta("2026-07-28")))];
        let json = serde_json::to_string(&batch).unwrap();
        let parsed: JsonRpcIncoming = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, JsonRpcIncoming::Batch(batch));
    }

    #[test]
    fn absent_id_field_parses_as_a_notification() {
        let json = r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#;
        let parsed: JsonRpcRequest = serde_json::from_str(json).unwrap();
        assert!(parsed.is_notification());
        assert_eq!(parsed.id, None);
    }

    #[test]
    fn explicit_null_id_is_not_a_notification() {
        let json = r#"{"jsonrpc":"2.0","id":null,"method":"ping"}"#;
        let parsed: JsonRpcRequest = serde_json::from_str(json).unwrap();
        assert!(!parsed.is_notification());
        assert_eq!(parsed.id, Some(JsonRpcId::Null));
    }
    //#endregion 🔖️FramingRoundTrips

    //#region 🔖️EraDetection
    #[test]
    fn modern_request_is_routed_via_meta_version_and_recorded_as_modern_era() {
        let mut server = McpServer::with_defaults();
        let response = server.dispatch(&request(Some(1), METHOD_TOOLS_LIST, Some(modern_meta("2026-07-28")))).unwrap();
        assert!(!response.is_error());
        assert_eq!(server.era(), Some(ProtocolEra::Modern));
        assert_eq!(server.negotiated_version(), Some("2026-07-28"));
    }

    #[test]
    fn legacy_initialize_handshake_echoes_a_supported_client_version() {
        let mut server = McpServer::with_defaults();
        let response = server.dispatch(&request(Some(1), METHOD_INITIALIZE, Some(serde_json::json!({ "protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": { "name": "t", "version": "0" } })))).unwrap();
        assert!(!response.is_error());
        let JsonRpcOutcome::Result { result } = response.outcome else { panic!("expected a result") };
        assert_eq!(result["protocolVersion"], "2025-06-18");
        assert_eq!(server.era(), Some(ProtocolEra::Legacy));
        assert_eq!(server.negotiated_version(), Some("2025-06-18"));

        let notified = server.dispatch(&JsonRpcRequest { jsonrpc: "2.0".to_string(), id: None, method: METHOD_NOTIFICATIONS_INITIALIZED.to_string(), params: None });
        assert!(notified.is_none(), "a notification never gets a response");
        assert!(server.is_initialized());
    }

    #[test]
    fn legacy_initialize_with_unknown_version_falls_back_to_latest_rather_than_erroring() {
        let mut server = McpServer::with_defaults();
        let response = server.dispatch(&request(Some(1), METHOD_INITIALIZE, Some(serde_json::json!({ "protocolVersion": "1999-01-01", "capabilities": {}, "clientInfo": { "name": "t", "version": "0" } })))).unwrap();
        let JsonRpcOutcome::Result { result } = response.outcome else { panic!("expected a result") };
        assert_eq!(result["protocolVersion"], SUPPORTED_PROTOCOL_VERSIONS[0]);
    }

    #[test]
    fn era_is_decided_by_the_opening_request_of_the_connection() {
        let mut modern_first = McpServer::with_defaults();
        assert!(modern_first.era().is_none());
        modern_first.dispatch(&request(Some(1), METHOD_SERVER_DISCOVER, None));
        assert_eq!(modern_first.era(), Some(ProtocolEra::Modern));

        let mut legacy_first = McpServer::with_defaults();
        legacy_first.dispatch(&request(Some(1), METHOD_INITIALIZE, Some(serde_json::json!({ "protocolVersion": "2025-11-25", "capabilities": {}, "clientInfo": { "name": "t", "version": "0" } }))));
        assert_eq!(legacy_first.era(), Some(ProtocolEra::Legacy));
    }
    //#endregion 🔖️EraDetection

    //#region 🔖️VersionRejection
    #[test]
    fn unsupported_meta_version_returns_dash_32022_with_supported_list() {
        let mut server = McpServer::with_defaults();
        let response = server.dispatch(&request(Some(1), METHOD_TOOLS_LIST, Some(modern_meta("2020-01-01")))).unwrap();
        let JsonRpcOutcome::Error { error } = response.outcome else { panic!("expected an error") };
        assert_eq!(error.code, UNSUPPORTED_PROTOCOL_VERSION);
        let data = error.data.expect("data must carry supported/requested");
        assert_eq!(data["requested"], "2020-01-01");
        assert_eq!(data["supported"], serde_json::json!(SUPPORTED_PROTOCOL_VERSIONS));
    }
    //#endregion 🔖️VersionRejection

    //#region 🔖️ServerDiscover
    #[test]
    fn server_discover_shape_carries_protocol_version_capabilities_and_server_info() {
        let mut server = McpServer::with_defaults();
        let response = server.dispatch(&request(Some(1), METHOD_SERVER_DISCOVER, Some(modern_meta("2025-11-25")))).unwrap();
        let JsonRpcOutcome::Result { result } = response.outcome else { panic!("expected a result") };
        assert_eq!(result["resultType"], "complete");
        assert_eq!(result["protocolVersion"], "2025-11-25");
        assert_eq!(result["serverInfo"]["name"], "semio-os-mcp");
        assert!(result["capabilities"]["tools"]["listChanged"].as_bool().unwrap());
    }

    #[test]
    fn server_discover_without_meta_defaults_to_the_newest_supported_version() {
        let mut server = McpServer::with_defaults();
        let response = server.dispatch(&request(Some(1), METHOD_SERVER_DISCOVER, None)).unwrap();
        let JsonRpcOutcome::Result { result } = response.outcome else { panic!("expected a result") };
        assert_eq!(result["protocolVersion"], SUPPORTED_PROTOCOL_VERSIONS[0]);
    }
    //#endregion 🔖️ServerDiscover

    //#region 🔖️ToolNameCharset
    #[test]
    fn tool_name_charset_accepts_and_rejects_correctly() {
        assert!(is_valid_tool_name("context_resolve"));
        assert!(is_valid_tool_name("cad__translateSelection"));
        assert!(is_valid_tool_name("a"));
        assert!(!is_valid_tool_name(""));
        assert!(!is_valid_tool_name("has spaces"));
        assert!(!is_valid_tool_name("has.dots"));
        assert!(!is_valid_tool_name(&"x".repeat(65)));
    }

    #[test]
    fn registry_rejects_registration_of_an_invalid_tool_name() {
        let mut registry = InMemoryToolRegistry::new();
        let result = registry.register(Tool::new("bad name!", serde_json::json!({"type": "object"})), |_arguments| CallToolResult::ok(vec![], None));
        let error = result.unwrap_err();
        assert_eq!(error.code, GatewayErrorCode::InputInvalid);
    }
    //#endregion 🔖️ToolNameCharset

    //#region 🔖️ToolVsProtocolError
    #[test]
    fn calling_an_unregistered_tool_is_a_protocol_error() {
        let mut server = McpServer::with_defaults();
        let response = server.dispatch(&request(Some(1), METHOD_TOOLS_CALL, Some(serde_json::json!({ "name": "does_not_exist", "arguments": {} })))).unwrap();
        assert!(response.is_error(), "unknown tool must be a JSON-RPC protocol error, not a successful isError result");
    }

    #[test]
    fn a_registered_tool_reporting_failure_is_a_successful_response_with_is_error_true() {
        let mut tools = InMemoryToolRegistry::new();
        tools.register(Tool::new("flaky_tool", serde_json::json!({"type": "object"})), |_arguments| CallToolResult::tool_error(&GatewayError::new(GatewayErrorCode::PreconditionFailed, "not ready"))).unwrap();
        let mut server = McpServer::new(Box::new(tools), Box::new(InMemoryResourceRegistry::new()), Box::new(InMemoryPromptRegistry::new()), Box::new(GatewayBackends::Null(NullBackend)));
        let response = server.dispatch(&request(Some(1), METHOD_TOOLS_CALL, Some(serde_json::json!({ "name": "flaky_tool", "arguments": {} })))).unwrap();
        assert!(!response.is_error(), "a tool's own failure must stay a JSON-RPC success envelope");
        let JsonRpcOutcome::Result { result } = response.outcome else { panic!("expected a result") };
        assert_eq!(result["isError"], true);
        assert_eq!(result["structuredContent"]["code"], "PRECONDITION_FAILED");
    }
    //#endregion 🔖️ToolVsProtocolError

    #[test]
    fn unknown_method_is_method_not_found() {
        let mut server = McpServer::with_defaults();
        let response = server.dispatch(&request(Some(1), "nonexistent/method", None)).unwrap();
        let JsonRpcOutcome::Error { error } = response.outcome else { panic!("expected an error") };
        assert_eq!(error.code, METHOD_NOT_FOUND);
    }

    #[test]
    fn catalog_hash_is_stable_under_reordering_and_changes_when_the_name_set_changes() {
        let a = vec![Tool::new("b_tool", serde_json::json!({})), Tool::new("a_tool", serde_json::json!({}))];
        let b = vec![Tool::new("a_tool", serde_json::json!({})), Tool::new("b_tool", serde_json::json!({}))];
        assert_eq!(compute_catalog_hash(&a), compute_catalog_hash(&b));
        let c = vec![Tool::new("a_tool", serde_json::json!({}))];
        assert_ne!(compute_catalog_hash(&a), compute_catalog_hash(&c));
    }
}

#[cfg(test)]
mod long {
    use super::*;

    #[test]
    fn a_full_modern_session_lists_reads_and_subscribes_resources_end_to_end() {
        let mut resources = InMemoryResourceRegistry::new();
        resources.register(
            Resource { uri: "semio://audit/log".to_string(), name: "audit-log".to_string(), title: None, description: None, mime_type: Some("text/plain".to_string()), size: None },
            vec![ResourceContent { uri: "semio://audit/log".to_string(), mime_type: Some("text/plain".to_string()), text: Some("hello".to_string()), blob: None }],
        );
        let mut server = McpServer::new(Box::new(InMemoryToolRegistry::new()), Box::new(resources), Box::new(InMemoryPromptRegistry::new()), Box::new(GatewayBackends::Null(NullBackend)));

        let meta = serde_json::json!({ "_meta": { META_PROTOCOL_VERSION_KEY: "2026-07-28" } });
        let list = server.dispatch(&tests_support::request_with(1, METHOD_RESOURCES_LIST, meta.clone())).unwrap();
        let JsonRpcOutcome::Result { result } = list.outcome else { panic!("expected a result") };
        assert_eq!(result["resources"].as_array().unwrap().len(), 1);

        let read = server.dispatch(&tests_support::request_with(2, METHOD_RESOURCES_READ, serde_json::json!({ "uri": "semio://audit/log", "_meta": meta["_meta"] }))).unwrap();
        let JsonRpcOutcome::Result { result } = read.outcome else { panic!("expected a result") };
        assert_eq!(result["contents"][0]["text"], "hello");

        let subscribed = server.dispatch(&tests_support::request_with(3, METHOD_RESOURCES_SUBSCRIBE, serde_json::json!({ "uri": "semio://audit/log", "_meta": meta["_meta"] }))).unwrap();
        assert!(!subscribed.is_error());
        assert_eq!(server.era(), Some(ProtocolEra::Modern));
    }

    #[test]
    fn batch_of_all_notifications_yields_no_responses() {
        let mut server = McpServer::with_defaults();
        let notifications = vec![
            JsonRpcRequest { jsonrpc: "2.0".to_string(), id: None, method: METHOD_NOTIFICATIONS_INITIALIZED.to_string(), params: None },
            JsonRpcRequest { jsonrpc: "2.0".to_string(), id: None, method: METHOD_NOTIFICATIONS_CANCELLED.to_string(), params: None },
        ];
        assert!(server.dispatch_batch(&notifications).is_empty());
    }
}

#[cfg(test)]
mod tests_support {
    use super::JsonRpcId;
    use super::JsonRpcRequest;

    pub fn request_with(id: i64, method: &str, params: serde_json::Value) -> JsonRpcRequest {
        JsonRpcRequest { jsonrpc: "2.0".to_string(), id: Some(JsonRpcId::Number(id)), method: method.to_string(), params: Some(params) }
    }
}
//#endregion 🧪️Tests
