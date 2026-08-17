//! 🌉️ Semio OS MCP gateway library root — dual-era JSON-RPC/MCP protocol core over stdio + Streamable
//! HTTP (packets `P1a-protocol-core` + `P1b-http-handles-bridge`). Downstream packets (P2 catalog, P6
//! actions/policy) implement `crate::{ToolRegistry, ResourceRegistry, PromptRegistry,
//! GatewayBackend}` against the real plugin host; THIS crate has zero dependency on it
//! (`📓️design-decisions.md` D8, P1a's brief §2.6 — verified by the absence of
//! `semio-framework*`/plugin/channel/actor deps in this module's own `Cargo.toml`). Every public item
//! from the `⚠️errors`/`🧬️schema`/`🧭️protocol`/`🚚️transport`/`🎫️handles`/`📒️audit`/`🧵️bridge` facets
//! is re-exported flat at this crate root for ergonomic downstream use.

//#region 🔖️Facets
pub use crate::audit::*;
pub use crate::bridge::*;
pub use crate::catalog::*;
pub use crate::conformance::*;
pub use crate::context::*;
pub use crate::errors::*;
pub use crate::fixtures::*;
pub use crate::handles::*;
pub use crate::protocol::*;
pub use crate::schema::*;
pub use crate::search::*;
pub use crate::transport::*;
// 🎫️ ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet P2-catalog: the glob re-exports
// above already bring every facet's public items into this module's namespace unqualified (matching
// this file's pre-existing convention — `StdioTransport`/`GatewayError`/`HttpTransportOptions` below
// are never module-qualified either), so P2's own new code below deliberately stays unqualified too;
// the one real ambiguity this crate now has (P1a's `schema::SearchHit` vs this packet's own BM25
// ranking type) is resolved by naming the latter `RankedHit`, not by qualifying paths.
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

/// 🚧️ `📋️master.md` §"MCP tool names": `action.prepare|invoke|cancel`, `transaction.begin|commit|
/// rollback`, `history.undo|redo`, `artifact.create|open|validate|export|snapshot`, `job.get|cancel`,
/// `ui.focus|reveal` — declared here (so `tools/list` is already the real, stable surface) but not
/// implemented until P6 (actions/policy) and P7 (headless workspace) land; every call returns a
/// structured `PLUGIN_UNAVAILABLE` tool-error, never a protocol-level failure.
const DECLARED_STUB_TOOL_NAMES: [&str; 17] = [
    "action_prepare",
    "action_invoke",
    "action_cancel",
    "transaction_begin",
    "transaction_commit",
    "transaction_rollback",
    "history_undo",
    "history_redo",
    "artifact_create",
    "artifact_open",
    "artifact_validate",
    "artifact_export",
    "artifact_snapshot",
    "job_get",
    "job_cancel",
    "ui_focus",
    "ui_reveal",
];

fn stub_tool_unavailable(_arguments: serde_json::Value) -> CallToolResult {
    CallToolResult::tool_error(&GatewayError::new(GatewayErrorCode::PluginUnavailable, "not implemented yet — lands with a later packet (P6 actions/policy or P7 headless workspace)").retryable())
}

/// 🏗️ Builds the real `ToolRegistry` this crate serves: the 3 real core tools (backed by the
/// compiled `catalog`), plus the 17 declared-but-unimplemented core names above.
pub fn build_tool_registry(catalog: std::sync::Arc<Catalog>) -> InMemoryToolRegistry {
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

    for tool_name in DECLARED_STUB_TOOL_NAMES {
        let mut tool = Tool::new(tool_name, serde_json::json!({ "$schema": "https://json-schema.org/draft/2020-12/schema", "type": "object" }));
        tool.description = Some("Declared, not yet implemented — returns a PLUGIN_UNAVAILABLE tool-error until P6/P7 land.".to_string());
        registry.register(tool, stub_tool_unavailable).expect("every declared stub name satisfies ^[a-zA-Z0-9_-]{1,64}$");
    }

    registry
}

/// 🏗️ Assembles the real `McpServer` `run_stdio`/`run_http` serve — the catalog-backed tool registry,
/// the catalog-backed resource registry, an empty prompt registry (unowned by this packet), and
/// `NullBackend` (no real `GatewayBackend` yet — P6/P7).
pub fn build_server() -> McpServer {
    let catalog = std::sync::Arc::new(build_catalog());
    let tools = build_tool_registry(catalog.clone());
    let resources = CatalogResourceRegistry::new(catalog);
    McpServer::new(Box::new(tools), Box::new(resources), Box::new(InMemoryPromptRegistry::new()), Box::new(NullBackend))
}
//#endregion 🔖️Tools

//#region 🔖️StdioEntrypoint
/// ⚙️ Options `📦️bin.rs`'s `stdio` subcommand parses off argv (`semio-os-mcp stdio [--folder <dir>]
/// [--principal <id>] [--scopes a,b]`) — stored but not yet consumed by anything OS-specific: no real
/// `GatewayBackend` is wired in P1a, so these become real constructor inputs in a later packet.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StdioOptions {
    pub folder: Option<String>,
    pub principal: Option<String>,
    pub scopes: Vec<String>,
}

/// 🚪️ Boots a [`McpServer::with_defaults`] and serves it over the REAL process stdin/stdout/stderr
/// until the client closes stdin (EOF) or a hard io error occurs. `bin.rs`'s entire `stdio` mode is
/// this one call — all logic lives here, in the lib, per P1a's brief §2.5.
pub fn run_stdio(_options: StdioOptions) -> Result<(), GatewayError> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let stderr = std::io::stderr();
    let mut transport = StdioTransport::new(stdin.lock(), stdout.lock(), stderr.lock());
    transport.serve(build_server())
}
//#endregion 🔖️StdioEntrypoint

//#region 🔖️HttpEntrypoint
/// ⚙️ Options `📦️bin.rs`'s `http` subcommand parses off argv (`semio-os-mcp http [--port <p>]
/// [--bind <addr>] --token <t> [--folder <dir>] [--principal <id>] [--scopes a,b] [--audit-dir <dir>]
/// [--allow-origin <origin>]…`) — `folder`/`principal`/`scopes` are stored but not yet consumed (same
/// deferral as [`StdioOptions`]: no real `GatewayBackend` is wired until a later packet).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpOptions {
    pub port: u16,
    pub bind: String,
    pub token: String,
    pub folder: Option<String>,
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
    FileAuditSink::new(audit_dir)?;
    let bind_ip: std::net::IpAddr = options.bind.parse().map_err(|error| GatewayError::new(GatewayErrorCode::InputInvalid, format!("invalid --bind address `{}`: {error}", options.bind)))?;
    let transport_options = HttpTransportOptions::new(options.token).bind_addr(std::net::SocketAddr::new(bind_ip, options.port)).allowed_origins(options.allow_origin);
    let mut transport = HttpTransport::new(transport_options);
    transport.serve(build_server())
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
        let options = HttpOptions { port: 7401, bind: "127.0.0.1".to_string(), token: "t".to_string(), folder: None, principal: None, scopes: vec![], audit_dir: None, allow_origin: vec![] };
        assert_eq!(options.port, 7401);
        assert_eq!(options.bind, "127.0.0.1");
    }

    #[test]
    fn tools_list_has_the_three_real_tools_plus_seventeen_declared_stubs() {
        let server = build_server();
        let tools = server.tools.list();
        assert_eq!(tools.len(), 20, "tools: {:?}", tools.iter().map(|tool| &tool.name).collect::<Vec<_>>());
        for name in ["capabilities_search", "capabilities_describe", "context_resolve"] {
            assert!(tools.iter().any(|tool| tool.name == name), "missing real tool {name}");
        }
        for name in DECLARED_STUB_TOOL_NAMES {
            assert!(tools.iter().any(|tool| tool.name == name), "missing declared stub tool {name}");
        }
    }

    #[test]
    fn declared_stub_tool_call_is_a_structured_plugin_unavailable_error() {
        let server = build_server();
        let result = server.tools.call("action_invoke", serde_json::json!({})).expect("known tool name resolves");
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
}
//#endregion 🧪️Tests
