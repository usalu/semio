//! 🧭️ Catalog discovery tools: `capabilities_search` (paged BM25 search) and `capabilities_describe`.

use semio_os_mcp_catalog::catalog::{CapabilityKind, Catalog};
use semio_os_mcp_catalog::search::{SearchFilters, search};
use semio_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use semio_os_mcp_foundation::reports::SearchHit;
use semio_os_mcp_catalog::capability_tools::to_schema_search_hit;
use semio_os_mcp_wire::protocol::{CallToolResult, ContentBlock};
use semio_os_mcp_wire::tool_paging::page_window;

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

const SEARCH_PAGE_DEFAULT: usize = 20;

const SEARCH_PAGE_MAX: usize = 100;

pub fn capabilities_search_handler(catalog: &Catalog, arguments: serde_json::Value) -> CallToolResult {
    let window = match page_window(&arguments, SEARCH_PAGE_DEFAULT, SEARCH_PAGE_MAX) {
        Ok(window) => window,
        Err(error) => return CallToolResult::tool_error(&error),
    };
    let query = arguments.get("query").and_then(serde_json::Value::as_str).unwrap_or("").to_string();
    let filters = search_filters_from_arguments(&arguments);
    let hits = search(catalog, &query, &filters);
    let total = hits.len();
    let search_hits: Vec<SearchHit> = hits.iter().skip(window.offset).take(window.size).filter_map(|hit| catalog.get(&hit.capability_id).map(|capability| to_schema_search_hit(capability, hit.score))).collect();
    let structured = serde_json::json!({ "results": search_hits, "total": total, "nextCursor": window.next_cursor(total) });
    CallToolResult::ok(vec![ContentBlock::Text { text: format!("{} of {total} result(s) for {query:?}", search_hits.len()) }], Some(structured))
}

pub fn capabilities_describe_handler(catalog: &Catalog, arguments: serde_json::Value) -> CallToolResult {
    let id = arguments.get("capabilityId").and_then(serde_json::Value::as_str).unwrap_or("");
    match catalog.get(id) {
        Some(capability) => CallToolResult::ok(vec![ContentBlock::Text { text: capability.title.clone() }], Some(serde_json::to_value(capability).unwrap_or(serde_json::Value::Null))),
        None => CallToolResult::tool_error(&GatewayError::new(GatewayErrorCode::NotFound, format!("no such capability: {id}"))),
    }
}
