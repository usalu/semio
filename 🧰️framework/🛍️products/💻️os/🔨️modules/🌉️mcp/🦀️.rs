//! 🌉️ Semio OS MCP gateway library root — dual-era JSON-RPC/MCP protocol core over stdio + Streamable
//! HTTP (packets `P1a-protocol-core` + `P1b-http-handles-bridge`). Downstream packets (P2 catalog, P6
//! actions/policy) implement `crate::{ToolRegistry, ResourceRegistry, PromptRegistry,
//! GatewayBackend}` against the real plugin host; THIS crate has zero dependency on it
//! (`📓️design-decisions.md` D8, P1a's brief §2.6 — verified by the absence of
//! `semio-framework*`/plugin/channel/actor deps in this module's own `Cargo.toml`). Every public item
//! from the `⚠️errors`/`🧬️schema`/`🧭️protocol`/`🚚️transport`/`🎫️handles`/`📒️audit`/`🧵️bridge` facets
//! is re-exported flat at this crate root for ergonomic downstream use.

//#region 🔖️Facets
pub use crate::actions::*;
pub use crate::artifact::*;
pub use crate::audit::*;
pub use crate::bridge::*;
pub use crate::catalog::*;
pub use crate::inference::*;
pub use crate::conformance::*;
pub use crate::context::*;
pub use crate::errors::*;
pub use crate::handles::*;
pub use crate::policy::*;
pub use crate::prompts::*;
pub use crate::protocol::*;
pub use crate::registry::*;
pub use crate::schema::*;
pub use crate::search::*;
pub use crate::transport::*;
pub use crate::ui::*;
pub use crate::workspace::*;
// 🧫️ Test-only: `🧪️tests/🧱️source-builders` is a `#[cfg(test)] pub(crate) mod` on the crate root
// (`📦️packages/🦀️rust/🦀️.rs`), and this crate's own test modules reach its builders unqualified
// through this root (`crate::note_descriptor()`, `note_and_cad_source()`) exactly as they reach every
// other facet above. Restored after a repo-wide sweep dropped it and left `📇️registry`/`🧪️tests`'
// own `#[cfg(test)]` modules unresolvable — not a new export, and invisible to any non-test build.
#[cfg(test)]
pub use crate::source_builders::*;
// 🎫️ ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet P2-catalog: the glob re-exports
// above already bring every facet's public items into this module's namespace unqualified (matching
// this file's pre-existing convention — `StdioTransport`/`GatewayError`/`HttpTransportOptions` below
// are never module-qualified either), so P2's own new code below deliberately stays unqualified too;
// the one real ambiguity this crate now has (P1a's `schema::SearchHit` vs this packet's own BM25
// ranking type) is resolved by naming the latter `RankedHit`, not by qualifying paths.
// 🎬️ packet P6-actions-policy: `actions`/`policy` add no further name collisions (`ActionAdapter`/
// `ArtifactChannel`/`UnboundArtifactChannel`/`InvokeRequest`/`SagaReport`/`UndoRedoReport` and
// `AgentPrincipal`/`PolicyEngine`/`AutoApprovePolicy`/`ApprovalGate` are all novel names crate-wide).
//#endregion 🔖️Facets

//#region 🔖️CoreCapabilities
// 🎫️ ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet P2-catalog: the small, stable
// direct-tool core `📋️master.md` §"MCP tool names" names (`context_resolve`, `capabilities_search`,
// `capabilities_describe`) — built as real `CapabilityDefinition`s (not just ad hoc `Tool`s)
// so they compile INTO the catalog like every other capability (searchable, describable,
// `semio://capability/{id}`-readable) and their `input_schema`/`output_schema`/`title`/`description`
// have exactly one source of truth, read back by `tool_from_capability` below.
fn capabilities_search_capability() -> CapabilityDefinition {
    let input_schema = capabilities_search_input_schema();
    CapabilityDefinition {
        id: CapabilityRef("capabilities.search".to_string()),
        version: 1,
        owner: CapabilityOwner::Gateway,
        kind: CapabilityKind::Meta,
        audience: CapabilityAudience::Agent,
        title: "Search Capabilities".to_string(),
        description: "Deterministic BM25 search over the compiled capability catalog — no LLM.".to_string(),
        artifact_kind: None,
        use_when: vec!["find a capability".to_string(), "what can I do".to_string(), "search for an action".to_string()],
        input_schema,
        output_schema: capabilities_search_output_schema(),
        effects: Default::default(),
        policy: Default::default(),
        execution: Default::default(),
        exposure: ToolExposure::Direct { tool_name: "capabilities_search".to_string() },
        presentation: CapabilityPresentation { icon_id: Some("search".to_string()), category: Some("gateway".to_string()), keys: None, in_palette: false, args: Vec::new() },
        examples: Vec::new(),
        source: CapabilitySource::Gateway,
    }
}

fn capabilities_describe_capability() -> CapabilityDefinition {
    let input_schema = capabilities_describe_input_schema();
    CapabilityDefinition {
        id: CapabilityRef("capabilities.describe".to_string()),
        version: 1,
        owner: CapabilityOwner::Gateway,
        kind: CapabilityKind::Meta,
        audience: CapabilityAudience::Agent,
        title: "Describe Capability".to_string(),
        description: "Returns the full CapabilityDefinition for one capability id.".to_string(),
        artifact_kind: None,
        use_when: vec!["show me the details of a capability".to_string(), "what arguments does this take".to_string()],
        input_schema,
        output_schema: capabilities_describe_output_schema(),
        effects: Default::default(),
        policy: Default::default(),
        execution: Default::default(),
        exposure: ToolExposure::Direct { tool_name: "capabilities_describe".to_string() },
        presentation: CapabilityPresentation { icon_id: Some("info".to_string()), category: Some("gateway".to_string()), keys: None, in_palette: false, args: Vec::new() },
        examples: Vec::new(),
        source: CapabilitySource::Gateway,
    }
}

fn context_resolve_capability() -> CapabilityDefinition {
    let input_schema = context_resolve_input_schema();
    CapabilityDefinition {
        id: CapabilityRef("context.resolve".to_string()),
        version: 1,
        owner: CapabilityOwner::Gateway,
        kind: CapabilityKind::Meta,
        audience: CapabilityAudience::Agent,
        title: "Resolve Context".to_string(),
        description: "Opens/refreshes the calling session and returns a token-cheap ContextSummary.".to_string(),
        artifact_kind: None,
        use_when: vec!["start a session".to_string(), "what can this session do".to_string()],
        input_schema,
        output_schema: context_resolve_output_schema(),
        effects: Default::default(),
        policy: Default::default(),
        execution: Default::default(),
        exposure: ToolExposure::Direct { tool_name: "context_resolve".to_string() },
        presentation: CapabilityPresentation { icon_id: Some("plug".to_string()), category: Some("gateway".to_string()), keys: None, in_palette: false, args: Vec::new() },
        examples: Vec::new(),
        source: CapabilitySource::Gateway,
    }
}

/// 🎯️ The three real, backend-free core tools this packet registers — folded into
/// `CatalogSource.gateway` so they appear in the compiled catalog exactly like every other
/// capability (D5: the catalog is the one source of truth `tools/list` and `capabilities.search`
/// both read from).
pub fn core_tool_capabilities() -> Vec<CapabilityDefinition> {
    let mut capabilities = vec![capabilities_search_capability(), capabilities_describe_capability(), context_resolve_capability()];
    capabilities.extend(artifact_capabilities());
    capabilities.extend(inference_capabilities());
    capabilities.extend(inference_job_capabilities());
    capabilities.extend(ui_capabilities());
    capabilities
}
//#endregion 🔖️CoreCapabilities

//#region 🔖️Catalog
/// 🗂️ Compiles the live gateway's catalog from the REAL installed plugin registry
/// (`registry::discover_catalog_source`, ticket 26/08/29/AI-MCP-END-TO-END packet W1) — never the
/// hand-written `🧫️note_and_cad_source()` fixture, which stays reserved for
/// `🗂️catalog`/`🔎️search`/`🧠️context`/`🧪️conformance`'s own tests. `discover_catalog_source(None)`
/// resolves the repo/space root itself and degrades to gateway-only capabilities (never a panic,
/// never a fallback to fixture data) when no plugin is installed or discoverable; if a real install
/// somehow yields a colliding capability id across two plugins, `compile()`'s `Err` degrades the same
/// way — a running gateway server with SOME broken plugin data must still start and serve its core
/// tools, never crash on launch.
pub fn build_catalog() -> Catalog {
    compile(&discover_catalog_source(None), semio_framework::Locale::En, semio_framework::Terminology::Native).unwrap_or_else(|error| {
        eprintln!("[mcp registry] catalog compile failed ({error}) — falling back to gateway-only capabilities");
        let gateway_only = CatalogSource { descriptors: Vec::new(), os_commands: Vec::new(), shell: Vec::new(), gateway: core_tool_capabilities() };
        compile(&gateway_only, semio_framework::Locale::En, semio_framework::Terminology::Native).expect("core gateway capabilities alone never collide with themselves")
    })
}

/// 🔐 Compiles a capability catalog from descriptors already verified by an external authority.
/// Unlike installed discovery, this function never reads a registry or descriptor path.
pub(crate) fn catalog_from_descriptors(descriptors: Vec<semio_framework::PackageDescriptor>) -> Result<Catalog, GatewayError> {
    let source = CatalogSource { descriptors, os_commands: Vec::new(), shell: Vec::new(), gateway: core_tool_capabilities() };
    compile(&source, semio_framework::Locale::En, semio_framework::Terminology::Native)
        .map_err(|error| GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("authenticated Hub descriptor catalog did not compile: {error}")).retryable())
}

fn gateway_only_catalog() -> std::sync::Arc<Catalog> {
    std::sync::Arc::new(catalog_from_descriptors(Vec::new()).expect("core gateway capabilities alone never collide with themselves"))
}
//#endregion 🔖️Catalog

//#region 🔖️Tools
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

/// 📄️ `capabilities.search`'s page size — the client's `limit`, clamped to `[1, SEARCH_PAGE_MAX]`,
/// defaulting to `SEARCH_PAGE_DEFAULT`. A client that asks for 500 hits gets 100 and a `nextCursor`,
/// never a silently truncated 20 (the pre-M5a behavior: a hardcoded `.take(20)` with no `total`, so
/// a caller could not tell a 20-hit answer from a 400-hit catalog).
const SEARCH_PAGE_DEFAULT: usize = 20;
const SEARCH_PAGE_MAX: usize = 100;

fn search_page_size(arguments: &serde_json::Value) -> usize {
    arguments.get("limit").and_then(serde_json::Value::as_u64).map(|limit| (limit as usize).clamp(1, SEARCH_PAGE_MAX)).unwrap_or(SEARCH_PAGE_DEFAULT)
}

/// 📄️ An opaque cursor is this result set's zero-based offset — opaque to the client by contract,
/// a decimal offset by implementation. A malformed cursor starts from the beginning rather than
/// erroring: a page boundary is never worth failing a discovery call over.
fn search_offset(arguments: &serde_json::Value) -> usize {
    arguments.get("cursor").and_then(serde_json::Value::as_str).and_then(|cursor| cursor.parse::<usize>().ok()).unwrap_or(0)
}

pub(crate) fn to_schema_search_hit(capability: &CapabilityDefinition, score: f64) -> SearchHit {
    let (plugin_id, app_id) = match &capability.owner {
        CapabilityOwner::Plugin { plugin_id, app_id, .. } => (plugin_id.clone(), app_id.clone().unwrap_or_default()),
        CapabilityOwner::Framework => ("framework".to_string(), String::new()),
        CapabilityOwner::Os => ("os".to_string(), String::new()),
        CapabilityOwner::Shell => ("shell".to_string(), String::new()),
        CapabilityOwner::Gateway => ("gateway".to_string(), String::new()),
        CapabilityOwner::Extension { extension_id } => (extension_id.clone(), String::new()),
    };
    let audience = match capability.audience {
        CapabilityAudience::Agent => "agent",
        CapabilityAudience::Input => "input",
        CapabilityAudience::Chrome => "chrome",
    };
    SearchHit { capability_id: capability.id.to_string(), title: capability.title.clone(), description: capability.description.clone(), score, plugin_id, app_id, audience: audience.to_string(), artifact_kind: capability.artifact_kind.clone().unwrap_or_default() }
}

/// 🔧️ Projects one `CapabilityDefinition` onto the MCP `Tool` shape — the single place a tool's
/// title/description/schemas are read back off the catalog, shared by every facet that registers
/// direct tools (`🗿️artifact`, `💡️inference`, `🖥️ui`) so no facet re-derives them.
pub(crate) fn tool_from_capability(capability: &CapabilityDefinition, tool_name: &str) -> Tool {
    let mut tool = Tool::new(tool_name, capability.input_schema.clone());
    tool.title = Some(capability.title.clone());
    tool.description = Some(capability.description.clone());
    tool.output_schema = Some(capability.output_schema.clone());
    tool
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

fn context_resolve_handler(catalog: &Catalog, counter: &std::sync::atomic::AtomicU64, arguments: serde_json::Value) -> CallToolResult {
    let principal = arguments.get("principal").and_then(serde_json::Value::as_str).unwrap_or("agent:local").to_string();
    let locale = arguments.get("locale").and_then(serde_json::Value::as_str).unwrap_or("en").to_string();
    let count = counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let session_id = mint_session_id(&principal, count);
    let summary = resolve_context(catalog, session_id, &principal, Vec::new(), None, &locale);
    CallToolResult::ok(vec![ContentBlock::Text { text: format!("session {} resolved", summary.session_id) }], Some(serde_json::to_value(&summary).unwrap_or(serde_json::Value::Null)))
}

/// 🎯️ Every MCP tool name this gateway serves, in `tools/list` order — the stable surface
/// `📋️master.md` §"MCP tool names" names. Ticket 26/08/29/AI-MCP-END-TO-END closed the last 9
/// stubs (`artifact.*`, `job.*`, `ui.*`), so this is now purely a census: there is no such thing
/// as a declared-but-unimplemented tool here any more. A tool's PRESENCE never depends on which
/// progressive-enhancement tier the server is running in; only a call's RESULT does.
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
//#endregion 🔖️Tools

//#region 🔖️MutationProtocolTools
/// 🕐️ Wall-clock milliseconds — every `ActionAdapter`/`HandleTable` call takes `now_ms` explicitly
/// (testability), this is the ONE call site that reads the real clock for the live server.
fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64
}

/// 🪪️ Every mutation-protocol tool call runs as this one fixed session, until a later packet makes
/// `McpServer` connection/session-aware (`📓️terra-P1b-report.md` §7.2 already documents this as a
/// P1b-only simplification every downstream facet inherits, not something this packet narrows).
pub(crate) const DEFAULT_SESSION_ID: &str = "sess_default";

pub(crate) fn default_session() -> SessionHandle {
    SessionHandle::new(DEFAULT_SESSION_ID)
}

fn parse_revision_stamp(value: &serde_json::Value) -> Option<RevisionStamp> {
    serde_json::from_value(value.clone()).ok()
}

/// 🎯️ The `instance` slot the mutation-protocol adapter must address for `capability_id` — the
/// SAME derivation `RoutingArtifactChannel` decodes an instance back with, so `prepare`'s leading
/// capability-less `ReadHistory` lands on the capability's own plugin instead of whichever plugin
/// sorts first in the catalog. `None`/unknown keeps the historical `0`, which is exactly the
/// single-plugin case where the first slot IS the right one.
#[cfg(not(target_arch = "wasm32"))]
fn action_instance_slot(catalog: &Catalog, capability_id: Option<&str>) -> u32 {
    capability_id.and_then(|id| capability_instance_slot(catalog, id)).unwrap_or(0)
}

#[cfg(target_arch = "wasm32")]
fn action_instance_slot(_catalog: &Catalog, _capability_id: Option<&str>) -> u32 {
    0
}

fn action_prepare_handler(catalog: &Catalog, actions: &ActionAdapter, principal: &AgentPrincipal, arguments: serde_json::Value) -> CallToolResult {
    let capability_id = match arguments.get("capabilityId").and_then(serde_json::Value::as_str) {
        Some(id) => id,
        None => return CallToolResult::tool_error(&GatewayError::new(GatewayErrorCode::InputInvalid, "capabilityId is required")),
    };
    let input = arguments.get("input").cloned().unwrap_or_else(|| serde_json::json!({}));
    let instance = action_instance_slot(catalog, Some(capability_id));
    match actions.prepare(catalog, principal, &default_session(), capability_id, input, instance, now_ms()) {
        Ok(report) => CallToolResult::ok(vec![ContentBlock::Text { text: format!("prepared {}", report.prepared_handle) }], Some(serde_json::to_value(&report).unwrap_or(serde_json::Value::Null))),
        Err(error) => CallToolResult::tool_error(&error),
    }
}

fn action_invoke_handler(catalog: &Catalog, actions: &ActionAdapter, principal: &AgentPrincipal, arguments: serde_json::Value) -> CallToolResult {
    let request = InvokeRequest {
        prepared_handle: arguments.get("preparedActionHandle").and_then(serde_json::Value::as_str).map(str::to_string),
        capability_id: arguments.get("capabilityId").and_then(serde_json::Value::as_str).map(str::to_string),
        input: arguments.get("input").cloned(),
        expected_revision: arguments.get("expectedRevision").and_then(parse_revision_stamp),
        idempotency_key: arguments.get("idempotencyKey").and_then(serde_json::Value::as_str).map(str::to_string),
        approval_handle: arguments.get("approvalHandle").and_then(serde_json::Value::as_str).map(str::to_string),
    };
    let instance = action_instance_slot(catalog, request.capability_id.as_deref());
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

/// 🏗️ Builds the real `ToolRegistry` this crate serves — 28 tools, none of them a stub: the 3 core
/// gateway tools, the 8 mutation-protocol tools (`P6-actions-policy`, backed by `actions`/
/// `principal`), the 5 `🗿️artifact` tools, the 2 `💡️inference` discovery tools, the 4 `💡️inference`
/// job tools (`inference_submit`/`inference_events`/`inference_cancel`/`inference_approve`, over any
/// declared service, guest- or hub-executed), the 4 `🖥️ui` tools (`ui_focus`/`ui_reveal`/`job_get`/`job_cancel`) and
/// `conversation_reply` (the agent's own free-text turn, ticket 26/09/18 slice AC1). Ticket
/// 26/08/29/AI-MCP-END-TO-END retired the last of these stubs entirely.
///
/// `workspace` and `bridge` carry the progressive-enhancement tier: a tool's PRESENCE in
/// `tools/list` never depends on either being bound — only a call's RESULT does, as a structured,
/// retryable `PLUGIN_UNAVAILABLE` naming exactly which binding is missing.
pub fn build_tool_registry(
    catalog: std::sync::Arc<Catalog>,
    actions: std::sync::Arc<ActionAdapter>,
    principal: AgentPrincipal,
    workspace: Option<std::sync::Arc<HeadlessWorkspace>>,
    bridge: Option<BridgeSlot>,
) -> InMemoryToolRegistry {
    let mut registry = InMemoryToolRegistry::new();
    // 💬️ Cloned up front: `principal` is moved into the inference job tools further down, and the
    // conversation tool is the only other one that gates on it.
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

    register_artifact_tools(&mut registry, workspace.clone());
    register_inference_tools(&mut registry, workspace.clone());
    //#region 💡️Inference
    register_inference_job_tools(&mut registry, catalog.clone(), workspace.clone(), actions.clone(), principal.clone(), default_session());
    //#endregion 💡️Inference
    //#region 💬️Conversation
    // 💬️ The agent's own voice. Registered from the SAME principal the mutation tools are gated on,
    // because it is the one UI tool a scope decides.
    register_conversation_tools(&mut registry, bridge.clone(), conversation_actions, conversation_principal);
    //#endregion 💬️Conversation
    register_ui_tools(&mut registry, bridge, workspace);

    registry
}

//#region 🔖️GatewayRuntime
/// 🔌️ Everything a live gateway binds LATE, after the tool registry already exists: the `/bridge`
/// slot a shell attaches through, the server→client request channel an `elicitation`-capable client
/// is asked through, and the launch-time `--auto-approve` policy. One struct rather than three more
/// positional parameters, because all three travel together through every constructor and every one
/// of them is absent in the ordinary test tier (`GatewayRuntime::default()`).
#[derive(Clone, Default)]
pub struct GatewayRuntime {
    pub bridge: Option<BridgeSlot>,
    pub elicitation: Option<ElicitationSlot>,
    pub auto_approve: AutoApprovePolicy,
    /// 🐚️ The session's artifact-channel decision (shell vs. headless), shared by the channel that
    /// executes it and the `context_resolve` handler that reports it. `None` in the ordinary test
    /// tier and on every gateway that never built a shell-routed channel — `channel_binding()`
    /// mints a headless-only one on demand so `context_resolve` always has an answer.
    pub channel_binding: Option<std::sync::Arc<crate::shell_channel::SessionChannelBinding>>,
}

impl GatewayRuntime {
    /// 🐚️ The binding this runtime reports and routes through — the one it was built with, or a
    /// fresh one over this runtime's own bridge slot (which decides `headless` whenever no shell
    /// with `relayAppCommands` is attached).
    pub fn channel_binding(&self) -> std::sync::Arc<crate::shell_channel::SessionChannelBinding> {
        match &self.channel_binding {
            Some(binding) => std::sync::Arc::clone(binding),
            None => std::sync::Arc::new(crate::shell_channel::SessionChannelBinding::new(self.bridge.clone())),
        }
    }

    /// ⛩️ The approval resolution chain this runtime can offer a parked approval — always built,
    /// even with both lanes empty, so `ActionAdapter` answers "nobody could be asked, here is why"
    /// rather than the older "APPROVAL_REQUIRED" with no explanation at all.
    fn approval_coordinator(&self) -> std::sync::Arc<ApprovalCoordinator> {
        std::sync::Arc::new(ApprovalCoordinator::new(self.elicitation.clone(), self.bridge.clone()))
    }
}
//#endregion 🔖️GatewayRuntime

/// 🏗️ Assembles the real `McpServer`: the catalog-backed + action-adapter-backed tool registry, the
/// catalog-backed resource registry, an empty prompt registry (unowned by this packet), and
/// `NullBackend` (`GatewayBackend` itself — the resource/context seam — still has no real
/// implementation; P7's headless workspace is that, `ArtifactChannel` here is a narrower, disjoint
/// port scoped to the mutation protocol only). `channel` is boxed so the live binary and every test
/// can supply either `UnboundArtifactChannel` (no `--folder`/`--hub`) or the real routing channel
/// with zero change to this function's body beyond the argument passed in.
pub fn build_server_with_principal(principal: AgentPrincipal, audit: std::sync::Arc<AuditSinks>, channel: Box<ArtifactChannels>, runtime: GatewayRuntime) -> McpServer {
    build_server_from_catalog(std::sync::Arc::new(build_catalog()), principal, audit, channel, runtime)
}

/// 🗂️ [`build_server_with_principal`] with the catalog injected rather than discovered. The live
/// binary always discovers (`build_catalog`); a test that asserts against a KNOWN capability census
/// injects `🧫️note_and_cad_source()`'s compiled catalog instead, so its assertions never depend on
/// which plugins happen to be installed in the tree it runs from.
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

/// 💬️ Publishes this server's real `tools/call` traffic onto the shell bridge, when there IS a
/// bridge. Factored out because both server constructors need the identical two lines, and because
/// "no bridge slot" must stay a silent, ordinary tier rather than a branch each caller re-invents.
fn publishing_agent_conversation(server: McpServer, bridge: Option<BridgeSlot>, label: &str) -> McpServer {
    match bridge {
        Some(slot) => server.publishing_conversation_to(std::sync::Arc::new(AgentConversation::new(slot, label))),
        None => server,
    }
}

/// 🏠️ ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet P7-headless-workspace:
/// additive twin of [`build_server_with_principal`] that plugs a real `GatewayBackend` in (a real
/// `HeadlessWorkspace`, via its `Arc<HeadlessWorkspace>: GatewayBackend` delegation
/// impl) instead of `NullBackend`, and overrides the `context_resolve` tool to answer from that SAME
/// live workspace instead of the backend-independent handler `build_tool_registry` wires by default
/// — everything else (catalog compile, the mutation-protocol tools, the action adapter) is the exact
/// same shared construction, called through unchanged. A separate function rather than a new
/// parameter on `build_server_with_principal` itself: that function's 3-argument shape has live
/// callers in this same in-flight packet's own tests (`P6-actions-policy`) this packet must not
/// disturb mid-flight.
pub fn build_server_with_workspace(principal: AgentPrincipal, audit: std::sync::Arc<AuditSinks>, workspace: std::sync::Arc<HeadlessWorkspace>, channel: Box<ArtifactChannels>, runtime: GatewayRuntime) -> McpServer {
    let catalog = workspace.discovery_catalog().unwrap_or_else(|_| gateway_only_catalog());
    let handles = std::sync::Arc::new(HandleTable::new());
    let idempotency = std::sync::Arc::new(IdempotencyStore::new());
    let client = ClientInfo { name: "semio-os-mcp".to_string(), version: env!("CARGO_PKG_VERSION").to_string() };
    let actions = std::sync::Arc::new(ActionAdapter::new(channel, handles, idempotency, audit, runtime.auto_approve, client));
    actions.bind_history_undo_port(workspace.clone());
    // 🗿️ The workspace reads this adapter's own guest instances back for `artifact_snapshot`, so a
    // snapshot shows the document as the agent just left it rather than as it was created.
    workspace.bind_root_action_adapter(actions.clone());
    actions.bind_approval_coordinator(runtime.approval_coordinator());
    let label = principal.label.clone();
    let tools = WorkspaceToolRegistry { workspace: workspace.clone(), actions, principal, bridge: runtime.bridge.clone(), channel_binding: runtime.channel_binding() };
    let resources = WorkspaceResourceRegistry::with_workspace(catalog, workspace.clone()).with_bridge(runtime.bridge.clone());
    let server = McpServer::new(Box::new(tools), Box::new(resources), Box::new(build_prompt_registry()), Box::new(GatewayBackends::WorkspaceArc(workspace)));
    publishing_agent_conversation(server, runtime.bridge, &label)
}

/// 🔄 Rebuilds the discovery projection for every list/call observation. A Hub binding that is
/// refreshing or revoked gets only gateway-owned tools; installed plugin descriptors are never a
/// fallback. Folder workspaces retain their installed catalog.
struct WorkspaceToolRegistry {
    workspace: std::sync::Arc<HeadlessWorkspace>,
    actions: std::sync::Arc<ActionAdapter>,
    principal: AgentPrincipal,
    bridge: Option<BridgeSlot>,
    channel_binding: std::sync::Arc<crate::shell_channel::SessionChannelBinding>,
}

impl WorkspaceToolRegistry {
    fn current(&self) -> InMemoryToolRegistry {
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

/// 🔁️ `InMemoryToolRegistry::register` overwrites an existing entry by name (`HashMap::insert`) —
/// re-registering `context_resolve` here replaces `build_tool_registry`'s backend-independent
/// handler with one that answers from the real, live workspace (real open artifacts, real
/// `catalog_hash`, real `active_artifact_id`) — never a fabricated session.
fn registry_override_context_resolve(tools: &mut InMemoryToolRegistry, context_tool: Tool, workspace: std::sync::Arc<HeadlessWorkspace>, principal_id: String, channel_binding: std::sync::Arc<crate::shell_channel::SessionChannelBinding>) {
    tools
        .register(context_tool, move |_arguments| match workspace.resolve_context(&principal_id) {
            Ok(mut summary) => {
                // 🐚️ THIS is where a session's document owner is chosen — once, and reported in the
                // same breath. `SessionChannelBinding::resolve` is sticky, so the `channel` a client
                // reads here is the channel every later `action_invoke`/`history_undo` on this
                // session executes through.
                summary.channel = channel_binding.resolve().label().to_string();
                CallToolResult::ok(vec![ContentBlock::Text { text: format!("session {} resolved on the {} channel", summary.session_id, summary.channel) }], Some(serde_json::to_value(&summary).unwrap_or(serde_json::Value::Null)))
            }
            Err(error) => CallToolResult::tool_error(&error),
        })
        .expect("context_resolve is a valid tool name");
}
//#endregion 🔖️MutationProtocolTools

//#region 🔖️WorkspaceOptions
/// 🏠️ ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet P7-headless-workspace:
/// `--hub <url> --space <id>` — the second binding shape `📋️master.md` §2.1 names
/// alongside `--folder`. Shared by `StdioOptions`/`HttpOptions` rather than duplicated per mode.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct HubOptions {
    pub base_url: String,
    pub space_id: String,
    /// 🤖️ Where this process reads its **delegated agent credential**, when it is an agent rather
    /// than a `dev s` child. `None` keeps the pre-existing behaviour: authenticate with the
    /// inherited fd-3 local-bootstrap envelope, i.e. as the human who launched the session.
    pub credential: Option<AgentCredentialSource>,
}

/// 🤖️ Where the delegated credential comes from. Only ever a *location*: the secret itself never
/// enters argv, the environment, or any `Debug` output.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AgentCredentialSource {
    File(String),
    Descriptor(i32),
}

impl AgentCredentialSource {
    /// 📥️ Loads and bounds-checks the credential this source names.
    pub fn load(&self) -> Result<crate::agent_credential::AgentCredentialV1, GatewayError> {
        match self {
            Self::File(path) => crate::agent_credential::AgentCredentialV1::read_file(std::path::Path::new(path)),
            #[cfg(unix)]
            Self::Descriptor(descriptor) => crate::agent_credential::AgentCredentialV1::read_fd(*descriptor),
            #[cfg(not(unix))]
            Self::Descriptor(_) => Err(GatewayError::new(GatewayErrorCode::InputInvalid, "--credential-fd is unavailable on this platform; use --credential-file")),
        }
    }
}

impl std::fmt::Debug for HubOptions {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("HubOptions").field("base_url", &self.base_url).field("space_id", &self.space_id).field("credential", &self.credential).finish()
    }
}

/// ⏳️ How many times the hub workspace open is re-attempted while the hub itself says "retry", and
/// the fixed step between attempts. The bound is what makes this a retry rather than a wait: a hub
/// that stays mid-refresh for longer than `HUB_OPEN_RETRY_ATTEMPTS × HUB_OPEN_RETRY_STEP_MS` still
/// fails, with the hub's own last message, instead of hanging a client's `initialize` forever.
const HUB_OPEN_RETRY_ATTEMPTS: u32 = 6;
const HUB_OPEN_RETRY_STEP_MS: u64 = 500;

/// 🔁️ Opens the hub workspace, re-attempting ONLY the states the hub marks retryable — a descriptor
/// index that is `refreshing`, a directory stream that has not re-dialled yet. Those are ordinary
/// mid-refresh states of a live hub, not faults, and before this a client whose hub happened to be
/// refreshing at `initialize` time saw the gateway process EXIT (measured 2026-09-21, M8 §5.4(2):
/// `PluginUnavailable: authenticated hub descriptor index is refreshing; retry after authority
/// refresh`, exit 1, before `initialize`). A non-retryable error — a bad credential, a space the
/// principal is not a member of, a version boundary — is returned on its first occurrence untouched.
fn open_hub_workspace_with_retry(hub: &HubOptions, credential: std::sync::Arc<semio_framework_os_kernel::os_directory::client::LocalHubCredential>, principal: &AgentPrincipal) -> Result<HeadlessWorkspace, GatewayError> {
    let scopes: Vec<String> = principal.scopes.iter().map(|scope| scope.0.clone()).collect();
    let mut attempts_made = 0;
    loop {
        match HeadlessWorkspace::open_hub(hub.base_url.clone(), hub.space_id.clone(), credential.clone(), principal.id.clone(), scopes.clone()) {
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

/// ⏳️ How long to wait before re-attempting a hub open, or `None` when this failure must be
/// returned instead — the whole retry decision as one pure predicate, so its bound is a law rather
/// than a loop a reader has to simulate. Only the hub's OWN `retryable` marking is retried; the
/// backoff grows linearly so the attempts spread rather than hammer, and it ends.
pub fn hub_open_retry_backoff_ms(error: &GatewayError, attempts_made: u32) -> Option<u64> {
    if !error.retryable || attempts_made >= HUB_OPEN_RETRY_ATTEMPTS {
        return None;
    }
    Some(HUB_OPEN_RETRY_STEP_MS.saturating_mul(u64::from(attempts_made) + 1))
}

/// 🏠️ Builds the real `McpServer` for a `--folder`/`--hub`-bound session: opens a real
/// `HeadlessWorkspace` and a `workspace::RoutingArtifactChannel`, which resolves the owning plugin
/// per capability from the compiled catalog and lazily opens one `PluginArtifactChannel` per plugin
/// (ticket 26/08/29/AI-MCP-END-TO-END packet W8 — the predecessor pinned the single plugin `note`,
/// then a single-plugin-only `resolve_default_plugin_id`, both of which break the moment more than
/// one plugin is installed). `folder`/`hub` are mutually exclusive; with NEITHER given the server is
/// built on [`UnboundArtifactChannel`] — every mutation-protocol call then answers the typed,
/// retryable `PLUGIN_UNAVAILABLE` naming both flags, the same answer the `🗿️artifact` tools' own
/// tier-1 gate gives. There is no scripted stand-in on any production path any more.
fn server_for_workspace_options(mut principal: AgentPrincipal, audit: std::sync::Arc<AuditSinks>, folder: Option<&str>, hub: Option<&HubOptions>, mut runtime: GatewayRuntime) -> Result<McpServer, GatewayError> {
    let origin_label;
    let workspace = if let Some(folder) = folder {
        origin_label = format!("folder {folder}");
        let catalog = std::sync::Arc::new(build_catalog());
        std::sync::Arc::new(HeadlessWorkspace::open_folder(std::path::PathBuf::from(folder), principal.id.clone(), principal.scopes.iter().map(|scope| scope.0.clone()).collect(), catalog)?)
    } else if let Some(hub) = hub {
        origin_label = format!("hub {}/{}", hub.base_url, hub.space_id);
        // 🤖️ Filled by the agent branch below and applied before the workspace is opened: a process
        // that exchanged a delegation IS that agent principal, and every surface that names a
        // principal — `context_resolve`, the audit sink, the policy gate — must say so rather than
        // the `--principal` default the launcher happened to pass (observed live 2026-09-20:
        // `context_resolve` reported `agent:local` while the process was acting as
        // `agent:<delegation id>`).
        let mut adopted: Option<(String, String)> = None;
        let credential = match &hub.credential {
            // 🤖️ Agent mode: a delegation a human minted for this agent, read from a 0600 file (or
            // an inherited descriptor), exchanged once at `POST /auth/agent-sessions`. The session
            // it returns is an ordinary hub session whose *kind* is `agent`, so everything below
            // this line is identical to a human's — and everything above the hub's presence
            // normalization now knows this peer is an agent principal, not the delegating human.
            Some(source) => {
                let delegation = source.load()?;
                if delegation.space_id() != hub.space_id {
                    return Err(GatewayError::new(GatewayErrorCode::PermissionDenied, format!("this agent credential is scoped to space `{}`, not `{}`", delegation.space_id(), hub.space_id)));
                }
                let grant = crate::workspace::remote::exchange_agent_session(&hub.base_url, &delegation)?;
                eprintln!("[semio-os-mcp] acting as agent principal {} (\"{}\") in space {}", grant.agent_principal_id, grant.agent_label, grant.space_id);
                adopted = Some((grant.agent_principal_id.clone(), grant.agent_label.clone()));
                std::sync::Arc::new(
                    semio_framework_os_kernel::os_directory::client::LocalHubCredential::adopt_session_capability(hub.base_url.trim_end_matches('/'), &grant.token)
                        .map_err(|_| GatewayError::new(GatewayErrorCode::PermissionDenied, "the hub returned an agent session capability this process cannot adopt"))?,
                )
            }
            None => semio_framework_os_kernel::os_directory::identity::claimed_local_hub_credential("mcp")
                .ok_or_else(|| GatewayError::new(GatewayErrorCode::PermissionDenied, "hub workspace requires either --credential-file <delegated agent credential> or a protected process-entry MCP credential"))?,
        };
        if let Some((agent_principal_id, agent_label)) = adopted {
            principal.delegated_by = Some(principal.id.clone());
            principal.id = agent_principal_id;
            principal.label = agent_label;
        }
        std::sync::Arc::new(open_hub_workspace_with_retry(hub, credential, &principal)?)
    } else {
        return Ok(build_server_with_principal(principal, audit, Box::new(ArtifactChannels::Unbound(UnboundArtifactChannel)), runtime));
    };
    eprintln!("[semio-os-mcp] real per-capability ArtifactChannel routing bound for {origin_label}");
    // 🐚️ Both routes are built; `SessionChannelBinding` picks one per session at `context_resolve`
    // time and `ContextSummary.channel` reports which. With no shell attached this behaves exactly
    // as the pre-LB1 gateway did — the headless workspace, unchanged.
    let binding = std::sync::Arc::new(crate::shell_channel::SessionChannelBinding::new(runtime.bridge.clone()));
    runtime.channel_binding = Some(std::sync::Arc::clone(&binding));
    let catalog = std::sync::Arc::new(build_catalog());
    // 🐚️ The artifact-level verbs (`artifact_create`, `artifact_export`) open a channel of their own
    // inside the workspace; publishing the binding there is what keeps a `shell` session from having
    // two document owners (`📓️lb1…` §7.3 step 1).
    workspace.bind_shell_route(std::sync::Arc::clone(&binding), std::sync::Arc::clone(&catalog));
    let channel: Box<ArtifactChannels> = Box::new(ArtifactChannels::Shell(crate::workspace::ShellRoutedArtifactChannel::new(binding, catalog, workspace.open_routing_channel())));
    Ok(build_server_with_workspace(principal, audit, workspace, channel, runtime))
}
//#endregion 🔖️WorkspaceOptions

//#region 🔖️StdioEntrypoint
/// ⚙️ Options `🏗️bootstrap/🦀️.rs`'s `stdio` subcommand parses off argv (`semio-os-mcp stdio [--folder <dir>]
/// [--hub <url> --space <id>] [--principal <id>] [--scopes a,b] [--auto-approve never|readonly|all]
/// [--no-bridge]`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StdioOptions {
    pub folder: Option<String>,
    pub hub: Option<HubOptions>,
    pub principal: Option<String>,
    pub scopes: Vec<String>,
    pub auto_approve: AutoApprovePolicy,
    /// 🚫️ Opts out of the loopback `/bridge` listener entirely — for a client that wants a pure,
    /// socket-free stdio process. Without it, stdio serving attaches to a live os session exactly
    /// like `http` mode does (`🛰️rendezvous`).
    pub no_bridge: bool,
}

/// 🌉️ Starts the loopback `/bridge` listener a stdio gateway attaches a live os session through, and
/// publishes its address as a `🛰️rendezvous` offer — whether or not a session is live yet. A shell
/// polls for offers (`useDiscoveredAgentBridgeConfig`), so a `dev s` started after the client launched
/// this gateway still dials it: the order in which a person opens their MCP client and their shell is
/// never a manual step. Returns `None` only when the user opted out (`--no-bridge`) or when nothing
/// can be bound or published; every bridge-dependent tool then answers the typed
/// `bridge_not_running_error`, and with a bridge but no shell yet, `no_shell_attached_error`.
///
/// The listener is bridge-ONLY: this process's MCP surface is stdin/stdout, so `/mcp` on that socket
/// is genuinely absent (404). Admission is a per-process proof published only through the owner-only
/// offer file — a stdio gateway inherits no hub fd-3 credential and must never fabricate one.
fn attach_stdio_bridge(options: &StdioOptions, principal: &AgentPrincipal, bridge_slot: &BridgeSlot) -> Option<StdioBridgeAttachment> {
    if options.no_bridge {
        return None;
    }
    let sessions = crate::rendezvous::live_os_sessions();
    let proof = crate::rendezvous::mint_admission_proof();
    let mut transport = HttpTransport::new(HttpTransportOptions::with_local_proof(&proof)).publishing_bridge_into(bridge_slot.clone());
    let run = match transport.start_bridge_only() {
        Ok(run) => run,
        Err(error) => {
            eprintln!("[semio-os-mcp] the loopback bridge listener could not bind ({}) — continuing over stdio with no live-shell surface", error.message);
            return None;
        }
    };
    let offer = crate::rendezvous::BridgeOffer {
        schema_version: crate::rendezvous::RENDEZVOUS_SCHEMA_VERSION,
        url: format!("ws://{}/bridge", run.local_addr()),
        admission_proof: proof,
        principal: principal.id.clone(),
        pid: std::process::id(),
        published_at_ms: now_ms(),
    };
    match crate::rendezvous::publish_offer(offer) {
        Ok(published) => {
            eprintln!("[semio-os-mcp] bridge listening on ws://{}/bridge — offered to {} live os session(s) via {}", run.local_addr(), sessions.len(), published.path().display());
            Some(StdioBridgeAttachment { run, _offer: published })
        }
        Err(error) => {
            eprintln!("[semio-os-mcp] the bridge offer could not be published ({}) — cancelling the listener rather than leaving an unreachable socket open", error.message);
            run.cancel();
            None
        }
    }
}

/// 🧷️ Keeps the bridge-only listener and its published offer alive for exactly as long as stdio
/// serving runs — dropping it removes the offer file and cancels the listener.
struct StdioBridgeAttachment {
    run: HttpTransportRun,
    _offer: crate::rendezvous::PublishedBridgeOffer,
}

impl Drop for StdioBridgeAttachment {
    fn drop(&mut self) {
        self.run.cancel();
    }
}

/// 🚪️ Boots the real [`McpServer`] and serves it over the REAL process stdin/stdout/stderr until the
/// client closes stdin (EOF) or a hard io error occurs. `bin.rs`'s entire `stdio` mode is this one
/// call — all logic lives here, in the lib, per P1a's brief §2.5. `options.principal`/`options.scopes`
/// (packet `P6-actions-policy`) build the real `AgentPrincipal` the mutation-protocol tools enforce;
/// `options.folder`/`options.hub` (packet `P7-headless-workspace`) open a real workspace instead of
/// `NullBackend`/`UnboundArtifactChannel` — the audit lane writes to `~/.semio/agent/audit` (D7:
/// local folder lane from day one). `options.auto_approve` carries the parsed `--auto-approve
/// never|readonly|all` policy; `Never` stays the default.
pub fn run_stdio(options: StdioOptions) -> Result<(), GatewayError> {
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
    let attachment = attach_stdio_bridge(&options, &principal, &bridge_slot);
    let runtime = GatewayRuntime { bridge: attachment.as_ref().map(|_| bridge_slot.clone()), elicitation: Some(elicitation.clone()), auto_approve: options.auto_approve, channel_binding: None };
    let server = server_for_workspace_options(principal, audit, options.folder.as_deref(), options.hub.as_ref(), runtime)?;
    let features = server.client_features();
    // 📤️ The server→client notification lane: `resources/updated` for a subscribed artifact,
    // `resources/list_changed` for a changed roster, `progress` for a `_meta.progressToken` call.
    // It rides the same single-owner `StdioLines` channel the elicitation request does, so a
    // notification emitted from inside a tool call reaches the client mid-call.
    let notifications = crate::notify::notification_slot();
    let server = server.publishing_notifications_into(notifications.clone());
    let mut transport = StdioTransport::new(std::io::BufReader::new(std::io::stdin()), std::io::stdout(), std::io::stderr())
        .publishing_elicitation_into(elicitation, features)
        .publishing_notifications_into(notifications);
    let result = transport.serve(server);
    drop(attachment);
    result
}
//#endregion 🔖️StdioEntrypoint

//#region 🔖️HttpEntrypoint
/// ⚙️ Options `🏗️bootstrap/🦀️.rs`'s `http` subcommand parses off argv (`semio-os-mcp http [--port <p>]
/// [--bind <addr>] [--folder <dir>] [--hub <url> --space <id>] [--principal <id>] [--scopes a,b]
/// [--audit-dir <dir>] [--allow-origin <origin>]…`). HTTP and bridge admission are both authorized
/// by the protected process-entry credential and never by argv, URL, environment, or disk state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpOptions {
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

/// 🚪️ Boots the retained nonblocking [`HttpTransport`] (Streamable HTTP, dual-era, `/mcp` +
/// `/bridge` on the SAME socket) bound to `bind:port`, serving until cancellation or process exit —
/// [`run_stdio`]. Fails fast (before binding a socket) if the audit directory cannot be created, so a
/// misconfigured `--audit-dir` surfaces immediately rather than on the first audit write a later
/// packet wires in. The logged bridge URL is credential-free; callers receive the admission proof
/// only through the supervisor-owned in-memory boundary.
pub fn run_http(options: HttpOptions) -> Result<(), GatewayError> {
    let audit_dir = options.audit_dir.clone().map(std::path::PathBuf::from).unwrap_or_else(default_audit_dir);
    let audit: std::sync::Arc<AuditSinks> = std::sync::Arc::new(AuditSinks::File(FileAuditSink::new(audit_dir)?));
    let principal = AgentPrincipal::from_scope_names(options.principal.clone().unwrap_or_else(|| "agent:local".to_string()), "http agent", &options.scopes, None);
    let bridge_slot: BridgeSlot = std::sync::Arc::new(std::sync::OnceLock::new());
    let runtime = GatewayRuntime { bridge: Some(bridge_slot.clone()), elicitation: None, auto_approve: options.auto_approve, channel_binding: None };
    let server = server_for_workspace_options(principal, audit, options.folder.as_deref(), options.hub.as_ref(), runtime)?;
    let bind_ip: std::net::IpAddr = options.bind.parse().map_err(|error| GatewayError::new(GatewayErrorCode::InputInvalid, format!("invalid --bind address `{}`: {error}", options.bind)))?;
    let credential = semio_framework_os_kernel::os_directory::identity::claimed_local_hub_credential("mcp")
        .ok_or_else(|| GatewayError::new(GatewayErrorCode::PermissionDenied, "HTTP mode requires a protected process-entry MCP credential"))?;
    eprintln!("[semio-os-mcp] bridge listening on ws://{bind_ip}:{}/bridge", options.port);
    let transport_options = HttpTransportOptions::new(credential).bind_addr(std::net::SocketAddr::new(bind_ip, options.port)).allowed_origins(options.allow_origin);
    let notifications = crate::notify::notification_slot();
    let server = server.publishing_notifications_into(notifications.clone());
    let mut transport = HttpTransport::new(transport_options).publishing_bridge_into(bridge_slot).publishing_notifications_into(notifications);
    transport.start(server)?.wait()
}
//#endregion 🔖️HttpEntrypoint

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️quick/🦀️.rs"]
mod quick;
//#endregion 🧪️Tests
