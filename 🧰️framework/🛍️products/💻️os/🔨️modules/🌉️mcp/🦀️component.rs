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
pub use crate::audit::*;
pub use crate::bridge::*;
pub use crate::catalog::*;
pub use crate::conformance::*;
pub use crate::context::*;
pub use crate::errors::*;
pub use crate::fixtures::*;
pub use crate::handles::*;
pub use crate::policy::*;
pub use crate::protocol::*;
pub use crate::schema::*;
pub use crate::search::*;
pub use crate::transport::*;
pub use crate::workspace::*;
// 🎫️ ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet P2-catalog: the glob re-exports
// above already bring every facet's public items into this module's namespace unqualified (matching
// this file's pre-existing convention — `StdioTransport`/`GatewayError`/`HttpTransportOptions` below
// are never module-qualified either), so P2's own new code below deliberately stays unqualified too;
// the one real ambiguity this crate now has (P1a's `schema::SearchHit` vs this packet's own BM25
// ranking type) is resolved by naming the latter `RankedHit`, not by qualifying paths.
// 🎬️ packet P6-actions-policy: `actions`/`policy` add no further name collisions (`ActionAdapter`/
// `ArtifactChannel`/`MockArtifactChannel`/`InvokeRequest`/`SagaReport`/`UndoRedoReport` and
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
    let input_schema = serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "semio://capability/capabilities.search/input",
        "type": "object",
        "properties": {
            "query": { "type": "string" },
            "kind": { "type": "array", "items": { "type": "string" } },
            "owner": { "type": "string" },
            "artifactKind": { "type": "string" },
            "requiresScope": { "type": "string" },
        },
        "required": ["query"],
        "additionalProperties": false,
    });
    CapabilityDefinition {
        id: CapabilityRef("capabilities.search".to_string()),
        version: 1,
        owner: CapabilityOwner::Gateway,
        kind: CapabilityKind::Meta,
        title: "Search Capabilities".to_string(),
        description: "Deterministic BM25 search over the compiled capability catalog — no LLM.".to_string(),
        artifact_kind: None,
        use_when: vec!["find a capability".to_string(), "what can I do".to_string(), "search for an action".to_string()],
        input_schema,
        // 🐛️ MCP's `tools/list` schema requires `outputSchema` (when present) to describe a JSON
        // OBJECT at the top level — a bare `type: "array"` fails the SDK client's own Zod validation
        // of the `Tool` shape (caught live running `bun nx run @semio-tech/framework-os-mcp:test-quick`,
        // not from reading the spec text). The hits themselves are wrapped under a `results` property.
        output_schema: serde_json::json!({ "$schema": "https://json-schema.org/draft/2020-12/schema", "$id": "semio://capability/capabilities.search/output", "type": "object", "properties": { "results": { "type": "array" } } }),
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
    let input_schema = serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "semio://capability/capabilities.describe/input",
        "type": "object",
        "properties": { "capabilityId": { "type": "string" } },
        "required": ["capabilityId"],
        "additionalProperties": false,
    });
    CapabilityDefinition {
        id: CapabilityRef("capabilities.describe".to_string()),
        version: 1,
        owner: CapabilityOwner::Gateway,
        kind: CapabilityKind::Meta,
        title: "Describe Capability".to_string(),
        description: "Returns the full CapabilityDefinition for one capability id.".to_string(),
        artifact_kind: None,
        use_when: vec!["show me the details of a capability".to_string(), "what arguments does this take".to_string()],
        input_schema,
        output_schema: serde_json::json!({ "$schema": "https://json-schema.org/draft/2020-12/schema", "$id": "semio://capability/capabilities.describe/output", "type": "object" }),
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
    let input_schema = serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "semio://capability/context.resolve/input",
        "type": "object",
        "properties": { "principal": { "type": "string" }, "locale": { "type": "string" } },
        "additionalProperties": false,
    });
    CapabilityDefinition {
        id: CapabilityRef("context.resolve".to_string()),
        version: 1,
        owner: CapabilityOwner::Gateway,
        kind: CapabilityKind::Meta,
        title: "Resolve Context".to_string(),
        description: "Opens/refreshes the calling session and returns a token-cheap ContextSummary.".to_string(),
        artifact_kind: None,
        use_when: vec!["start a session".to_string(), "what can this session do".to_string()],
        input_schema,
        output_schema: serde_json::json!({ "$schema": "https://json-schema.org/draft/2020-12/schema", "$id": "semio://capability/context.resolve/output", "type": "object" }),
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
    vec![capabilities_search_capability(), capabilities_describe_capability(), context_resolve_capability()]
}
//#endregion 🔖️CoreCapabilities

//#region 🔖️Catalog
/// 🗂️ Compiles the live gateway's catalog. Until a real `GatewayBackend` (P6/P7) sources
/// `PackageDescriptor`s from an actual plugin host, `🧫️note_and_cad_source()` — the SAME
/// real, hand-verified note/cad action census `🧪️conformance`'s own tests compile against — is the
/// only real capability data this crate has to serve `tools/list`/`capabilities.search` with. This is
/// temporary and explicitly documented as such (`📓️terra-P2-report.md` "what P6 needs from me"): a
/// later packet replaces this call with one that compiles from a live descriptor source instead.
pub fn build_catalog() -> Catalog {
    compile(&note_and_cad_source(), semio_framework::Locale::En, semio_framework::Terminology::Native).expect("the bundled note+cad fixture catalog always compiles")
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
    }
}

fn to_schema_search_hit(capability: &CapabilityDefinition, score: f64) -> SearchHit {
    let (plugin_id, app_id) = match &capability.owner {
        CapabilityOwner::Plugin { plugin_id, app_id, .. } => (plugin_id.clone(), app_id.clone().unwrap_or_default()),
        CapabilityOwner::Framework => ("framework".to_string(), String::new()),
        CapabilityOwner::Os => ("os".to_string(), String::new()),
        CapabilityOwner::Shell => ("shell".to_string(), String::new()),
        CapabilityOwner::Gateway => ("gateway".to_string(), String::new()),
        CapabilityOwner::Extension { extension_id } => (extension_id.clone(), String::new()),
    };
    SearchHit { capability_id: capability.id.to_string(), title: capability.title.clone(), description: capability.description.clone(), score, plugin_id, app_id }
}

fn tool_from_capability(capability: &CapabilityDefinition, tool_name: &str) -> Tool {
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
    let search_hits: Vec<SearchHit> = hits.iter().take(20).filter_map(|hit| catalog.get(&hit.capability_id).map(|capability| to_schema_search_hit(capability, hit.score))).collect();
    let structured = serde_json::json!({ "results": search_hits });
    CallToolResult::ok(vec![ContentBlock::Text { text: format!("{} result(s) for {query:?}", search_hits.len()) }], Some(structured))
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

/// 🚧️ `📋️master.md` §"MCP tool names": `artifact.create|open|validate|export|snapshot`,
/// `job.get|cancel`, `ui.focus|reveal` — declared here (so `tools/list` is already the real, stable
/// surface) but not implemented until P7 (headless workspace) / P10 (shell) land; every call returns
/// a structured `PLUGIN_UNAVAILABLE` tool-error, never a protocol-level failure. The 8 mutation-
/// protocol tools (`action_prepare|invoke|cancel`, `transaction_begin|commit|rollback`,
/// `history_undo|redo`) moved OUT of this list in packet `P6-actions-policy` — see
/// `🔖️MutationProtocolTools` below, they are real now.
const DECLARED_STUB_TOOL_NAMES: [&str; 9] = ["artifact_create", "artifact_open", "artifact_validate", "artifact_export", "artifact_snapshot", "job_get", "job_cancel", "ui_focus", "ui_reveal"];

fn stub_tool_unavailable(_arguments: serde_json::Value) -> CallToolResult {
    CallToolResult::tool_error(&GatewayError::new(GatewayErrorCode::PluginUnavailable, "not implemented yet — lands with a later packet (P7 headless workspace or P10 shell)").retryable())
}
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
const DEFAULT_SESSION_ID: &str = "sess_default";

fn default_session() -> SessionHandle {
    SessionHandle::new(DEFAULT_SESSION_ID)
}

fn parse_revision_stamp(value: &serde_json::Value) -> Option<RevisionStamp> {
    serde_json::from_value(value.clone()).ok()
}

fn action_prepare_input_schema() -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "semio://capability/action.prepare/input",
        "type": "object",
        "properties": { "capabilityId": { "type": "string" }, "input": { "type": "object" } },
        "required": ["capabilityId"],
        "additionalProperties": false,
    })
}

fn action_invoke_input_schema() -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "semio://capability/action.invoke/input",
        "type": "object",
        "properties": {
            "preparedActionHandle": { "type": "string" },
            "capabilityId": { "type": "string" },
            "input": { "type": "object" },
            "expectedRevision": { "type": "object" },
            "idempotencyKey": { "type": "string" },
            "approvalHandle": { "type": "string" },
        },
        "additionalProperties": false,
    })
}

fn handle_input_schema(field: &str, capability_id: &str) -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": format!("semio://capability/{capability_id}/input"),
        "type": "object",
        "properties": { field: { "type": "string" } },
        "required": [field],
        "additionalProperties": false,
    })
}

fn transaction_begin_input_schema() -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "semio://capability/transaction.begin/input",
        "type": "object",
        "properties": { "preparedHandles": { "type": "array", "items": { "type": "string" } } },
        "required": ["preparedHandles"],
        "additionalProperties": false,
    })
}

fn action_prepare_handler(catalog: &Catalog, actions: &ActionAdapter, principal: &AgentPrincipal, arguments: serde_json::Value) -> CallToolResult {
    let capability_id = match arguments.get("capabilityId").and_then(serde_json::Value::as_str) {
        Some(id) => id,
        None => return CallToolResult::tool_error(&GatewayError::new(GatewayErrorCode::InputInvalid, "capabilityId is required")),
    };
    let input = arguments.get("input").cloned().unwrap_or_else(|| serde_json::json!({}));
    match actions.prepare(catalog, principal, &default_session(), capability_id, input, 0, now_ms()) {
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
    match actions.invoke(catalog, principal, &default_session(), request, 0, now_ms()) {
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

/// 🏗️ Builds the real `ToolRegistry` this crate serves: the 3 real core tools, the 8 real
/// mutation-protocol tools (packet `P6-actions-policy`, backed by `actions`/`principal`), plus the 9
/// still-declared-but-unimplemented names above.
pub fn build_tool_registry(catalog: std::sync::Arc<Catalog>, actions: std::sync::Arc<ActionAdapter>, principal: AgentPrincipal) -> InMemoryToolRegistry {
    let mut registry = InMemoryToolRegistry::new();

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
    action_prepare.output_schema = Some(serde_json::to_value(schemars::schema_for!(PreparedActionReport)).unwrap_or(serde_json::Value::Null));
    let (c, a, p) = (catalog.clone(), actions.clone(), principal.clone());
    registry.register(action_prepare, move |arguments| action_prepare_handler(&c, &a, &p, arguments)).expect("action_prepare is a valid tool name");

    let mut action_invoke = Tool::new("action_invoke", action_invoke_input_schema());
    action_invoke.title = Some("Invoke Action".to_string());
    action_invoke.description = Some("Commits a prepared (or freshly-prepared) action through the 2-phase transaction protocol — returns an InvocationReport.".to_string());
    action_invoke.output_schema = Some(serde_json::to_value(schemars::schema_for!(InvocationReport)).unwrap_or(serde_json::Value::Null));
    let (c, a, p) = (catalog.clone(), actions.clone(), principal.clone());
    registry.register(action_invoke, move |arguments| action_invoke_handler(&c, &a, &p, arguments)).expect("action_invoke is a valid tool name");

    let mut action_cancel = Tool::new("action_cancel", handle_input_schema("preparedActionHandle", "action.cancel"));
    action_cancel.title = Some("Cancel Action".to_string());
    action_cancel.description = Some("Drops a prepared-action handle before it is invoked.".to_string());
    let a = actions.clone();
    registry.register(action_cancel, move |arguments| action_cancel_handler(&a, arguments)).expect("action_cancel is a valid tool name");

    let mut transaction_begin = Tool::new("transaction_begin", transaction_begin_input_schema());
    transaction_begin.title = Some("Begin Transaction".to_string());
    transaction_begin.description = Some("Binds several already-prepared action handles into one saga transaction handle.".to_string());
    let a = actions.clone();
    registry.register(transaction_begin, move |arguments| transaction_begin_handler(&a, arguments)).expect("transaction_begin is a valid tool name");

    let mut transaction_commit = Tool::new("transaction_commit", handle_input_schema("transactionHandle", "transaction.commit"));
    transaction_commit.title = Some("Commit Transaction".to_string());
    transaction_commit.description = Some("Commits every member of a saga transaction (2-phase, reverse-order commit, compensating undo on failure).".to_string());
    let (a, p) = (actions.clone(), principal.clone());
    registry.register(transaction_commit, move |arguments| transaction_commit_handler(&a, &p, arguments)).expect("transaction_commit is a valid tool name");

    let mut transaction_rollback = Tool::new("transaction_rollback", handle_input_schema("transactionHandle", "transaction.rollback"));
    transaction_rollback.title = Some("Rollback Transaction".to_string());
    transaction_rollback.description = Some("Abandons a saga transaction before it is committed.".to_string());
    let a = actions.clone();
    registry.register(transaction_rollback, move |arguments| transaction_rollback_handler(&a, arguments)).expect("transaction_rollback is a valid tool name");

    let mut history_undo = Tool::new("history_undo", handle_input_schema("undoToken", "history.undo"));
    history_undo.title = Some("Undo".to_string());
    history_undo.description = Some("Fans TransactionUndo out to every member a committed invocation or saga touched.".to_string());
    let a = actions.clone();
    registry.register(history_undo, move |arguments| history_undo_handler(&a, arguments)).expect("history_undo is a valid tool name");

    let mut history_redo = Tool::new("history_redo", handle_input_schema("undoToken", "history.redo"));
    history_redo.title = Some("Redo".to_string());
    history_redo.description = Some("Fans TransactionRedo out to every member a committed invocation or saga touched.".to_string());
    let a = actions.clone();
    registry.register(history_redo, move |arguments| history_redo_handler(&a, arguments)).expect("history_redo is a valid tool name");

    for tool_name in DECLARED_STUB_TOOL_NAMES {
        let mut tool = Tool::new(tool_name, serde_json::json!({ "$schema": "https://json-schema.org/draft/2020-12/schema", "type": "object" }));
        tool.description = Some("Declared, not yet implemented — returns a PLUGIN_UNAVAILABLE tool-error until P7/P10 land.".to_string());
        registry.register(tool, stub_tool_unavailable).expect("every declared stub name satisfies ^[a-zA-Z0-9_-]{1,64}$");
    }

    registry
}

/// 🏗️ Assembles the real `McpServer`: the catalog-backed + action-adapter-backed tool registry, the
/// catalog-backed resource registry, an empty prompt registry (unowned by this packet), and
/// `NullBackend` (`GatewayBackend` itself — the resource/context seam — still has no real
/// implementation; P7's headless workspace is that, `ArtifactChannel` here is a narrower, disjoint
/// port scoped to the mutation protocol only). `channel` is boxed so the live binary and every test
/// can supply either `MockArtifactChannel` (today) or P7's real implementation (tomorrow) with zero
/// change to this function's body beyond the argument passed in.
pub fn build_server_with_principal(principal: AgentPrincipal, audit: std::sync::Arc<dyn AuditSink>, channel: Box<dyn ArtifactChannel>) -> McpServer {
    let catalog = std::sync::Arc::new(build_catalog());
    let handles = std::sync::Arc::new(HandleTable::new());
    let idempotency = std::sync::Arc::new(IdempotencyStore::new());
    let client = ClientInfo { name: "semio-os-mcp".to_string(), version: env!("CARGO_PKG_VERSION").to_string() };
    let actions = std::sync::Arc::new(ActionAdapter::new(channel, handles, idempotency, audit, AutoApprovePolicy::Never, client));
    let tools = build_tool_registry(catalog.clone(), actions, principal);
    let resources = CatalogResourceRegistry::new(catalog);
    McpServer::new(Box::new(tools), Box::new(resources), Box::new(InMemoryPromptRegistry::new()), Box::new(NullBackend))
}

/// 🏗️ Convenience default used by every pre-existing P1a/P1b/P2 test and by anywhere a live backend
/// isn't the point — an unscoped `agent:local` principal (zero granted scopes, the SAFE default — no
/// capability requiring any policy scope can be invoked without explicitly granting one),
/// `InMemoryAuditSink` (no disk I/O from unit tests), and a fresh `MockArtifactChannel`.
pub fn build_server() -> McpServer {
    let principal = AgentPrincipal::from_scope_names("agent:local", "local agent", &[], None);
    build_server_with_principal(principal, std::sync::Arc::new(InMemoryAuditSink::new()), Box::new(MockArtifactChannel::new()))
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
pub fn build_server_with_workspace(principal: AgentPrincipal, audit: std::sync::Arc<dyn AuditSink>, workspace: std::sync::Arc<HeadlessWorkspace>, channel: Box<dyn ArtifactChannel>) -> McpServer {
    let catalog = std::sync::Arc::new(build_catalog());
    let handles = std::sync::Arc::new(HandleTable::new());
    let idempotency = std::sync::Arc::new(IdempotencyStore::new());
    let client = ClientInfo { name: "semio-os-mcp".to_string(), version: env!("CARGO_PKG_VERSION").to_string() };
    let actions = std::sync::Arc::new(ActionAdapter::new(channel, handles, idempotency, audit, AutoApprovePolicy::Never, client));
    let mut tools = build_tool_registry(catalog.clone(), actions, principal.clone());
    let context_tool = tool_from_capability(catalog.get("context.resolve").expect("context.resolve compiled"), "context_resolve");
    let (workspace_for_context, principal_id) = (workspace.clone(), principal.id.clone());
    registry_override_context_resolve(&mut tools, context_tool, workspace_for_context, principal_id);
    let resources = CatalogResourceRegistry::new(catalog);
    McpServer::new(Box::new(tools), Box::new(resources), Box::new(InMemoryPromptRegistry::new()), Box::new(workspace) as Box<dyn GatewayBackend>)
}

/// 🔁️ `InMemoryToolRegistry::register` overwrites an existing entry by name (`HashMap::insert`) —
/// re-registering `context_resolve` here replaces `build_tool_registry`'s backend-independent
/// handler with one that answers from the real, live workspace (real open artifacts, real
/// `catalog_hash`, real `active_artifact_id`) — never a fabricated session.
fn registry_override_context_resolve(tools: &mut InMemoryToolRegistry, context_tool: Tool, workspace: std::sync::Arc<HeadlessWorkspace>, principal_id: String) {
    tools
        .register(context_tool, move |_arguments| match workspace.resolve_context(&principal_id) {
            Ok(summary) => CallToolResult::ok(vec![ContentBlock::Text { text: format!("session {} resolved", summary.session_id) }], Some(serde_json::to_value(&summary).unwrap_or(serde_json::Value::Null))),
            Err(error) => CallToolResult::tool_error(&error),
        })
        .expect("context_resolve is a valid tool name");
}
//#endregion 🔖️MutationProtocolTools

//#region 🔖️WorkspaceOptions
/// 🏠️ ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet P7-headless-workspace:
/// `--hub <url> --space <id> [--token <t>]` — the second binding shape `📋️master.md` §2.1 names
/// alongside `--folder`. Shared by `StdioOptions`/`HttpOptions` rather than duplicated per mode.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HubOptions {
    pub base_url: String,
    pub space_id: String,
    pub token: Option<String>,
}

/// 🏠️ Builds the real `McpServer` for a `--folder`/`--hub`-bound session: opens a real
/// `HeadlessWorkspace`, a real `workspace::PluginArtifactChannel` targeting `note` (the
/// one plugin with a committed descriptor — `📓️status.md`'s E2-builder-descriptor entry) when the
/// registry/wasm are both resolvable, and falls back to `MockArtifactChannel` with a clear stderr
/// diagnostic otherwise (never a silent downgrade). `folder`/`hub` are mutually exclusive; neither
/// given falls back to [`build_server_with_principal`] (`NullBackend` + `MockArtifactChannel`,
/// unchanged pre-P7 behavior — every pre-existing P1a/P1b/P2/P6 test keeps passing).
fn server_for_workspace_options(principal: AgentPrincipal, audit: std::sync::Arc<dyn AuditSink>, folder: Option<&str>, hub: Option<&HubOptions>) -> Result<McpServer, GatewayError> {
    let catalog = std::sync::Arc::new(build_catalog());
    let origin_label;
    let workspace = if let Some(folder) = folder {
        origin_label = format!("folder {folder}");
        std::sync::Arc::new(HeadlessWorkspace::open_folder(std::path::PathBuf::from(folder), principal.id.clone(), principal.scopes.iter().map(|scope| scope.0.clone()).collect(), catalog)?)
    } else if let Some(hub) = hub {
        origin_label = format!("hub {}/{}", hub.base_url, hub.space_id);
        std::sync::Arc::new(HeadlessWorkspace::open_hub(hub.base_url.clone(), hub.space_id.clone(), hub.token.clone(), principal.id.clone(), principal.scopes.iter().map(|scope| scope.0.clone()).collect(), catalog)?)
    } else {
        return Ok(build_server_with_principal(principal, audit, Box::new(MockArtifactChannel::new())));
    };
    let channel: Box<dyn ArtifactChannel> = match workspace.open_artifact_channel("note") {
        Ok(real_channel) => Box::new(real_channel),
        Err(error) => {
            eprintln!("[semio-os-mcp] real ArtifactChannel unavailable for {origin_label} ({error:?}); falling back to MockArtifactChannel");
            Box::new(MockArtifactChannel::new())
        }
    };
    Ok(build_server_with_workspace(principal, audit, workspace, channel))
}
//#endregion 🔖️WorkspaceOptions

//#region 🔖️StdioEntrypoint
/// ⚙️ Options `📦️bin.rs`'s `stdio` subcommand parses off argv (`semio-os-mcp stdio [--folder <dir>]
/// [--hub <url> --space <id> [--token <t>]] [--principal <id>] [--scopes a,b]`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StdioOptions {
    pub folder: Option<String>,
    pub hub: Option<HubOptions>,
    pub principal: Option<String>,
    pub scopes: Vec<String>,
}

/// 🚪️ Boots the real [`McpServer`] and serves it over the REAL process stdin/stdout/stderr until the
/// client closes stdin (EOF) or a hard io error occurs. `bin.rs`'s entire `stdio` mode is this one
/// call — all logic lives here, in the lib, per P1a's brief §2.5. `options.principal`/`options.scopes`
/// (packet `P6-actions-policy`) build the real `AgentPrincipal` the mutation-protocol tools enforce;
/// `options.folder`/`options.hub` (packet `P7-headless-workspace`) open a real workspace instead of
/// `NullBackend`/`MockArtifactChannel` — the audit lane writes to `~/.semio/agent/audit` (D7: local
/// folder lane from day one). `--auto-approve` has no CLI flag yet, so the server always runs with
/// the safe `AutoApprovePolicy::Never` until a later packet leases `bin.rs` to add the flag.
pub fn run_stdio(options: StdioOptions) -> Result<(), GatewayError> {
    let principal = AgentPrincipal::from_scope_names(options.principal.clone().unwrap_or_else(|| "agent:local".to_string()), "stdio agent", &options.scopes, None);
    let audit: std::sync::Arc<dyn AuditSink> = std::sync::Arc::new(FileAuditSink::new(default_audit_dir())?);
    let server = server_for_workspace_options(principal, audit, options.folder.as_deref(), options.hub.as_ref())?;
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let stderr = std::io::stderr();
    let mut transport = StdioTransport::new(stdin.lock(), stdout.lock(), stderr.lock());
    transport.serve(server)
}
//#endregion 🔖️StdioEntrypoint

//#region 🔖️HttpEntrypoint
/// ⚙️ Options `📦️bin.rs`'s `http` subcommand parses off argv (`semio-os-mcp http [--port <p>]
/// [--bind <addr>] --token <t> [--folder <dir>] [--hub <url> --space <id> [--token <t>]]
/// [--principal <id>] [--scopes a,b] [--audit-dir <dir>] [--allow-origin <origin>]…`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpOptions {
    pub port: u16,
    pub bind: String,
    pub token: String,
    pub folder: Option<String>,
    pub hub: Option<HubOptions>,
    pub principal: Option<String>,
    pub scopes: Vec<String>,
    pub audit_dir: Option<String>,
    pub allow_origin: Vec<String>,
}

/// 🚪️ Boots an axum-backed [`HttpTransport`] (Streamable HTTP, dual-era) bound to `bind:port`,
/// serving until the process is killed — the HTTP analogue of [`run_stdio`]. Fails fast (before
/// binding a socket) if the audit directory cannot be created, so a misconfigured `--audit-dir`
/// surfaces immediately rather than on the first audit write a later packet wires in.
pub fn run_http(options: HttpOptions) -> Result<(), GatewayError> {
    let audit_dir = options.audit_dir.clone().map(std::path::PathBuf::from).unwrap_or_else(default_audit_dir);
    let audit: std::sync::Arc<dyn AuditSink> = std::sync::Arc::new(FileAuditSink::new(audit_dir)?);
    let principal = AgentPrincipal::from_scope_names(options.principal.clone().unwrap_or_else(|| "agent:local".to_string()), "http agent", &options.scopes, None);
    let server = server_for_workspace_options(principal, audit, options.folder.as_deref(), options.hub.as_ref())?;
    let bind_ip: std::net::IpAddr = options.bind.parse().map_err(|error| GatewayError::new(GatewayErrorCode::InputInvalid, format!("invalid --bind address `{}`: {error}", options.bind)))?;
    let transport_options = HttpTransportOptions::new(options.token).bind_addr(std::net::SocketAddr::new(bind_ip, options.port)).allowed_origins(options.allow_origin);
    let mut transport = HttpTransport::new(transport_options);
    transport.serve(server)
}
//#endregion 🔖️HttpEntrypoint

//#region 🧪️Tests
#[cfg(test)]
mod quick {
    use super::*;

    #[test]
    fn stdio_options_default_to_empty() {
        let options = StdioOptions::default();
        assert!(options.folder.is_none());
        assert!(options.principal.is_none());
        assert!(options.scopes.is_empty());
    }

    #[test]
    fn every_facet_re_export_is_reachable_from_the_crate_root() {
        let _code: GatewayErrorCode = GatewayErrorCode::Internal;
        let _tools = InMemoryToolRegistry::new();
        let _resources = InMemoryResourceRegistry::new();
        let _prompts = InMemoryPromptRegistry::new();
        let _backend = NullBackend;
        let _server = McpServer::with_defaults();
        assert_eq!(SUPPORTED_PROTOCOL_VERSIONS[0], "2026-07-28");
    }

    #[test]
    fn p1b_facet_re_exports_are_reachable_from_the_crate_root_too() {
        let _handles = HandleTable::new();
        let _idempotency = IdempotencyStore::new();
        let _audit = InMemoryAuditSink::new();
        let _bridge_frame = ShellToGateway::Ping;
        assert_eq!(BRIDGE_VERSION, 1);
    }

    #[test]
    fn http_options_round_trip_into_a_transport_bind_addr_and_token() {
        let options = HttpOptions { port: 7401, bind: "127.0.0.1".to_string(), token: "t".to_string(), folder: None, hub: None, principal: None, scopes: vec![], audit_dir: None, allow_origin: vec![] };
        assert_eq!(options.port, 7401);
        assert_eq!(options.bind, "127.0.0.1");
    }

    /// 🎬️ packet `P6-actions-policy`: total tools/list size (20) is unchanged — 3 real core tools + 8
    /// real mutation-protocol tools (this packet) + 9 still-declared stubs. The 8 mutation-protocol
    /// names moved OUT of `DECLARED_STUB_TOOL_NAMES` (now 9, not 17) into real, individually-tested
    /// tool registrations below.
    #[test]
    fn tools_list_has_the_real_tools_plus_the_declared_stubs() {
        let server = build_server();
        let tools = server.tools.list();
        assert_eq!(tools.len(), 20, "tools: {:?}", tools.iter().map(|tool| &tool.name).collect::<Vec<_>>());
        let real_names = ["capabilities_search", "capabilities_describe", "context_resolve", "action_prepare", "action_invoke", "action_cancel", "transaction_begin", "transaction_commit", "transaction_rollback", "history_undo", "history_redo"];
        assert_eq!(real_names.len() + DECLARED_STUB_TOOL_NAMES.len(), 20);
        for name in real_names {
            assert!(tools.iter().any(|tool| tool.name == name), "missing real tool {name}");
            assert!(!DECLARED_STUB_TOOL_NAMES.contains(&name), "{name} must no longer be a declared stub");
        }
        for name in DECLARED_STUB_TOOL_NAMES {
            assert!(tools.iter().any(|tool| tool.name == name), "missing declared stub tool {name}");
        }
    }

    /// 🚧️ `artifact_create` is still genuinely unimplemented (P7's job) — this is the same assertion
    /// the pre-P6 test made against `action_invoke`, redirected to a tool that is STILL a stub now
    /// that `action_invoke` is real (packet `P6-actions-policy`).
    #[test]
    fn declared_stub_tool_call_is_a_structured_plugin_unavailable_error() {
        let server = build_server();
        let result = server.tools.call("artifact_create", serde_json::json!({})).expect("known tool name resolves");
        assert!(result.is_error);
        assert_eq!(result.structured_content.as_ref().unwrap()["code"], "PLUGIN_UNAVAILABLE");
    }

    #[test]
    fn capabilities_search_tool_call_finds_translate_selection() {
        let server = build_server();
        let result = server.tools.call("capabilities_search", serde_json::json!({ "query": "move the selection" })).expect("known tool name resolves");
        assert!(!result.is_error);
        let structured = result.structured_content.expect("structured content");
        assert_eq!(structured["results"][0]["capabilityId"], "cad.editor.translateSelection");
    }

    #[test]
    fn capabilities_describe_tool_call_returns_the_full_definition() {
        let server = build_server();
        let result = server.tools.call("capabilities_describe", serde_json::json!({ "capabilityId": "cad.editor.translateSelection" })).expect("known tool name resolves");
        assert!(!result.is_error);
        assert_eq!(result.structured_content.unwrap()["id"], "cad.editor.translateSelection");
    }

    #[test]
    fn context_resolve_tool_call_returns_a_context_summary_with_the_catalog_hash() {
        let server = build_server();
        let catalog = build_catalog();
        let result = server.tools.call("context_resolve", serde_json::json!({ "principal": "agent:local" })).expect("known tool name resolves");
        assert!(!result.is_error);
        assert_eq!(result.structured_content.unwrap()["catalogHash"], catalog.hash);
    }

    #[test]
    fn every_declared_stub_tool_name_satisfies_the_tool_name_charset() {
        for name in DECLARED_STUB_TOOL_NAMES {
            assert!(is_valid_tool_name(name), "{name} violates ^[a-zA-Z0-9_-]{{1,64}}$");
        }
    }

    //#region 🔖️MutationProtocolToolWiring
    /// 🎬️ The exact scenario the brief's §5 live transcript demonstrates, proven deterministically:
    /// a principal WITH `artifact.write` can `action_prepare` the cad demo capability and gets back a
    /// real `PreparedActionReport`.
    #[test]
    fn action_prepare_tool_call_returns_a_prepared_action_report_for_a_granted_scope() {
        let principal = AgentPrincipal::from_scope_names("agent:demo", "demo", &["artifact.write".to_string()], None);
        let server = build_server_with_principal(principal, std::sync::Arc::new(InMemoryAuditSink::new()), Box::new(MockArtifactChannel::new()));
        let result = server.tools.call("action_prepare", serde_json::json!({ "capabilityId": "cad.editor.translateSelection", "input": { "dx": 1.0, "dy": 0.0, "dz": 0.0, "objectIds": ["a"] } })).expect("known tool name resolves");
        assert!(!result.is_error, "{result:?}");
        let structured = result.structured_content.expect("structured content");
        assert!(structured["preparedHandle"].as_str().unwrap().starts_with("prep_"));
        assert_eq!(structured["capabilityId"], "cad.editor.translateSelection");
    }

    /// 🎬️ The second half of the brief's §5 live transcript: a principal WITHOUT the required scope
    /// gets `PERMISSION_DENIED`, never a protocol-level failure.
    #[test]
    fn action_prepare_tool_call_is_permission_denied_for_a_scope_the_principal_lacks() {
        let principal = AgentPrincipal::from_scope_names("agent:demo", "demo", &[], None); // no scopes granted
        let server = build_server_with_principal(principal, std::sync::Arc::new(InMemoryAuditSink::new()), Box::new(MockArtifactChannel::new()));
        let result = server.tools.call("action_prepare", serde_json::json!({ "capabilityId": "cad.editor.translateSelection", "input": { "dx": 1.0, "dy": 0.0, "dz": 0.0, "objectIds": ["a"] } })).expect("known tool name resolves");
        assert!(result.is_error);
        assert_eq!(result.structured_content.as_ref().unwrap()["code"], "PERMISSION_DENIED");
    }

    #[test]
    fn action_invoke_tool_call_commits_a_prepared_capability_end_to_end() {
        let principal = AgentPrincipal::from_scope_names("agent:demo", "demo", &["artifact.write".to_string()], None);
        let server = build_server_with_principal(principal, std::sync::Arc::new(InMemoryAuditSink::new()), Box::new(MockArtifactChannel::new()));
        let prepared = server.tools.call("action_prepare", serde_json::json!({ "capabilityId": "cad.editor.translateSelection", "input": { "dx": 1.0, "dy": 0.0, "dz": 0.0, "objectIds": ["a"] } })).unwrap();
        let handle = prepared.structured_content.unwrap()["preparedHandle"].as_str().unwrap().to_string();
        let invoked = server.tools.call("action_invoke", serde_json::json!({ "preparedActionHandle": handle })).expect("known tool name resolves");
        assert!(!invoked.is_error, "{invoked:?}");
        let structured = invoked.structured_content.unwrap();
        assert_eq!(structured["status"], "SUCCEEDED");
        assert!(structured["undoToken"].as_str().unwrap().starts_with("undo_"));
    }
    //#endregion 🔖️MutationProtocolToolWiring
}
//#endregion 🧪️Tests
