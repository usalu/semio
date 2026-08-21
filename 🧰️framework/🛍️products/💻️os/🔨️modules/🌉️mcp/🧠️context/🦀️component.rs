//! 🧠️ Context broker + resource projection — packet `P2-catalog`, `📋️master.md` §3.5. `context.resolve`
//! returns the token-cheap `ContextSummary` (P1a's `🧬️schema`); `semio://capability[/{id}]` and
//! `semio://workspace` are served for real from the compiled `Catalog` — with `NullBackend` there is
//! no live workspace yet, so the live-data resources return well-formed empty/`NOT_FOUND` rather than
//! fabricated content (this module's brief §2.3: "do not fake workspace data").

use crate::catalog::Catalog;
use crate::errors::{GatewayError, GatewayErrorCode};
use crate::protocol::{Resource, ResourceContent, ResourceRegistry, ResourceTemplate};
use crate::schema::ContextSummary;
use std::sync::Arc;

//#region 🔖️TokenBudget
pub const DEFAULT_MAX_TOKENS: u32 = 4096;
pub const HARD_MAX_TOKENS: u32 = 32768;

/// 🔢️ `ceil(bytes/4)` — the crate-wide token estimate every resource projection budgets against.
pub fn estimate_tokens(bytes: usize) -> u32 {
    ((bytes + 3) / 4) as u32
}

/// ✂️ Result of `truncate_to_budget` — `omitted` carries a JSON pointer per dropped element so a
/// caller can request the rest via a follow-up, narrower read.
#[derive(Clone, Debug, PartialEq)]
pub struct Truncated {
    pub value: serde_json::Value,
    pub omitted: Vec<String>,
    pub token_estimate: u32,
}

/// ✂️ Breadth-first truncation over the top-level `"entries"` array (the shape every projection in
/// this module produces) — drops elements from the END until the remaining JSON fits `max_tokens`
/// (clamped to `[1, HARD_MAX_TOKENS]`), recording each dropped index as `/entries/<index>`. A value
/// with no `"entries"` array, or one that already fits, passes through with `omitted: []`.
pub fn truncate_to_budget(value: serde_json::Value, max_tokens: u32) -> Truncated {
    let max_tokens = max_tokens.clamp(1, HARD_MAX_TOKENS);
    let byte_budget = max_tokens as usize * 4;
    let full_bytes = serde_json::to_vec(&value).unwrap_or_default().len();
    if full_bytes <= byte_budget {
        return Truncated { value: value.clone(), omitted: Vec::new(), token_estimate: estimate_tokens(full_bytes) };
    }
    let mut working = value;
    let mut omitted = Vec::new();
    if let Some(object) = working.as_object_mut() {
        if let Some(serde_json::Value::Array(entries)) = object.get_mut("entries") {
            while !entries.is_empty() {
                let current_bytes = serde_json::to_vec(entries).unwrap_or_default().len();
                if current_bytes <= byte_budget {
                    break;
                }
                let index = entries.len() - 1;
                entries.remove(index);
                omitted.push(format!("/entries/{index}"));
            }
        }
    }
    let final_bytes = serde_json::to_vec(&working).unwrap_or_default().len();
    omitted.reverse();
    Truncated { value: working, omitted, token_estimate: estimate_tokens(final_bytes) }
}
//#endregion 🔖️TokenBudget

//#region 🔖️ContextResolve
/// 🪪️ Mints a fresh, opaque session id — blake3-mixed `(principal, wall-clock ms, call counter)`, the
/// same "no new dependency, `blake3` already in-tree" precedent `🎫️handles/🦀️component.rs`'s own
/// `mint_id` set (`📓️terra-P1b-report.md` §2.2). A real `AgentSession` (handle-table-backed,
/// refreshed on every resolve) is P6/P7 work — this is `context.resolve`'s minimal, real-today
/// contract: a stable-looking id, not a persisted session record.
pub fn mint_session_id(principal: &str, counter: u64) -> String {
    let now_ms = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|duration| duration.as_millis()).unwrap_or(0);
    format!("sess_{}", blake3::hash(format!("{principal}:{now_ms}:{counter}").as_bytes()).to_hex())
}

/// 🪪️ `context.resolve` — the token-cheap summary every session opens with.
pub fn resolve_context(catalog: &Catalog, session_id: String, principal: &str, scopes: Vec<String>, active_artifact_id: Option<String>, locale: &str) -> ContextSummary {
    ContextSummary { session_id, principal: principal.to_string(), scopes, active_artifact_id, catalog_hash: catalog.hash.clone(), locale: locale.to_string() }
}
//#endregion 🔖️ContextResolve

//#region 🔖️CapabilityResource
/// 📖️ `semio://capability` (list, token-budgeted) / `semio://capability/{id}` (one full
/// `CapabilityDefinition`) — served for real from the compiled catalog; an unknown `{id}` is a
/// well-formed `NOT_FOUND`, never fabricated content.
pub fn capability_resource_contents(catalog: &Catalog, id: Option<&str>) -> Result<Vec<ResourceContent>, GatewayError> {
    match id {
        Some(id) => {
            let capability = catalog.get(id).ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("no such capability: {id}")))?;
            let json = serde_json::to_value(capability).expect("CapabilityDefinition always serializes");
            Ok(vec![ResourceContent { uri: format!("semio://capability/{id}"), mime_type: Some("application/json".to_string()), text: Some(json.to_string()), blob: None }])
        }
        None => {
            let entries: Vec<serde_json::Value> = catalog.entries.iter().map(|capability| serde_json::json!({ "id": capability.id.as_str(), "title": capability.title, "kind": capability.kind })).collect();
            let value = serde_json::json!({ "catalogHash": catalog.hash, "entries": entries });
            let truncated = truncate_to_budget(value, DEFAULT_MAX_TOKENS);
            let mut payload = truncated.value;
            if !truncated.omitted.is_empty() {
                if let Some(object) = payload.as_object_mut() {
                    object.insert("omitted".to_string(), serde_json::json!(truncated.omitted));
                }
            }
            Ok(vec![ResourceContent { uri: "semio://capability".to_string(), mime_type: Some("application/json".to_string()), text: Some(payload.to_string()), blob: None }])
        }
    }
}
//#endregion 🔖️CapabilityResource

//#region 🔖️WorkspaceResource
/// 🏠️ `semio://workspace` — with `NullBackend` there is no live workspace (no open artifacts, no
/// session list); this resource honestly reports that instead of inventing placeholder workspace
/// data, while still surfacing the one thing that IS real today: the compiled capability catalog's
/// identity and size.
pub fn workspace_resource_contents(catalog: &Catalog) -> Vec<ResourceContent> {
    let value = serde_json::json!({
        "catalogHash": catalog.hash,
        "capabilityCount": catalog.entries.len(),
        "artifacts": [],
        "note": "no live workspace backend wired yet (NullBackend, packet P2-catalog) — artifact/session data arrives with P6/P7",
    });
    vec![ResourceContent { uri: "semio://workspace".to_string(), mime_type: Some("application/json".to_string()), text: Some(value.to_string()), blob: None }]
}
//#endregion 🔖️WorkspaceResource

//#region 🔖️CatalogResourceRegistry
/// 🗂️ The real `ResourceRegistry` this crate registers into `McpServer` — every method is served
/// from the compiled `Catalog` (`semio://capability`, `semio://capability/{id}`, `semio://workspace`);
/// `subscribe`/`unsubscribe` are accepted no-ops (there is no live change stream to subscribe to
/// until a real `GatewayBackend` lands — P6/P7).
pub struct CatalogResourceRegistry {
    catalog: Arc<Catalog>,
}

impl CatalogResourceRegistry {
    pub fn new(catalog: Arc<Catalog>) -> Self {
        Self { catalog }
    }
}

impl ResourceRegistry for CatalogResourceRegistry {
    fn list(&self) -> Vec<Resource> {
        vec![
            Resource {
                uri: "semio://capability".to_string(),
                name: "capabilities".to_string(),
                title: Some("Capability catalog".to_string()),
                description: Some("Every compiled capability, token-budgeted".to_string()),
                mime_type: Some("application/json".to_string()),
                size: None,
            },
            Resource {
                uri: "semio://workspace".to_string(),
                name: "workspace".to_string(),
                title: Some("Workspace".to_string()),
                description: Some("Live workspace summary (no backend wired yet)".to_string()),
                mime_type: Some("application/json".to_string()),
                size: None,
            },
        ]
    }

    fn templates(&self) -> Vec<ResourceTemplate> {
        vec![ResourceTemplate {
            uri_template: "semio://capability/{id}".to_string(),
            name: "capability".to_string(),
            title: Some("One capability".to_string()),
            description: Some("Full CapabilityDefinition by id".to_string()),
            mime_type: Some("application/json".to_string()),
        }]
    }

    fn read(&self, uri: &str) -> Result<Vec<ResourceContent>, GatewayError> {
        if uri == "semio://capability" {
            return capability_resource_contents(&self.catalog, None);
        }
        if let Some(id) = uri.strip_prefix("semio://capability/") {
            return capability_resource_contents(&self.catalog, Some(id));
        }
        if uri == "semio://workspace" {
            return Ok(workspace_resource_contents(&self.catalog));
        }
        Err(GatewayError::new(GatewayErrorCode::NotFound, format!("unknown resource: {uri}")))
    }

    fn subscribe(&self, _uri: &str) -> Result<(), GatewayError> {
        Ok(())
    }

    fn unsubscribe(&self, _uri: &str) -> Result<(), GatewayError> {
        Ok(())
    }
}
//#endregion 🔖️CatalogResourceRegistry

//#region 🧪️Tests
#[cfg(test)]
mod quick {
    use super::*;
    use crate::catalog::compile;
    use crate::fixtures;
    use semio_framework::{Locale, Terminology};

    fn test_catalog() -> Catalog {
        compile(&fixtures::note_and_cad_source(), Locale::En, Terminology::Native).expect("compiles")
    }

    #[test]
    fn estimate_tokens_rounds_up() {
        assert_eq!(estimate_tokens(0), 0);
        assert_eq!(estimate_tokens(1), 1);
        assert_eq!(estimate_tokens(4), 1);
        assert_eq!(estimate_tokens(5), 2);
    }

    #[test]
    fn small_value_is_never_truncated() {
        let value = serde_json::json!({ "entries": [1, 2, 3] });
        let truncated = truncate_to_budget(value.clone(), DEFAULT_MAX_TOKENS);
        assert_eq!(truncated.value, value);
        assert!(truncated.omitted.is_empty());
    }

    #[test]
    fn oversized_entries_array_is_truncated_with_recorded_pointers() {
        let entries: Vec<serde_json::Value> = (0..5000).map(|index| serde_json::json!({ "id": format!("capability-{index}"), "title": "x".repeat(50) })).collect();
        let value = serde_json::json!({ "entries": entries });
        let truncated = truncate_to_budget(value, 64);
        assert!(truncated.token_estimate <= 64 * 2, "truncation should bring the payload near budget, got {}", truncated.token_estimate);
        assert!(!truncated.omitted.is_empty());
        assert!(truncated.omitted.iter().all(|pointer| pointer.starts_with("/entries/")));
    }

    #[test]
    fn mint_session_id_is_unique_per_counter() {
        let a = mint_session_id("agent:local", 0);
        let b = mint_session_id("agent:local", 1);
        assert_ne!(a, b);
        assert!(a.starts_with("sess_"));
    }

    #[test]
    fn resolve_context_carries_the_catalog_hash() {
        let catalog = test_catalog();
        let summary = resolve_context(&catalog, "sess_1".to_string(), "agent:local", vec!["documents.read".to_string()], None, "en");
        assert_eq!(summary.catalog_hash, catalog.hash);
        assert_eq!(summary.session_id, "sess_1");
    }

    #[test]
    fn capability_resource_with_id_returns_the_full_definition() {
        let catalog = test_catalog();
        let contents = capability_resource_contents(&catalog, Some("cad.editor.translateSelection")).expect("known capability resolves");
        assert_eq!(contents.len(), 1);
        let text = contents[0].text.as_ref().expect("json text");
        assert!(text.contains("translateSelection"));
    }

    #[test]
    fn capability_resource_with_unknown_id_is_not_found() {
        let catalog = test_catalog();
        let result = capability_resource_contents(&catalog, Some("no.such.capability"));
        assert!(matches!(result, Err(error) if error.code == GatewayErrorCode::NotFound));
    }

    #[test]
    fn capability_resource_without_id_lists_every_entry() {
        let catalog = test_catalog();
        let contents = capability_resource_contents(&catalog, None).expect("list resolves");
        let value: serde_json::Value = serde_json::from_str(contents[0].text.as_ref().unwrap()).unwrap();
        assert_eq!(value["entries"].as_array().unwrap().len(), catalog.entries.len());
    }

    #[test]
    fn workspace_resource_is_well_formed_and_honest_about_having_no_backend() {
        let catalog = test_catalog();
        let contents = workspace_resource_contents(&catalog);
        let value: serde_json::Value = serde_json::from_str(contents[0].text.as_ref().unwrap()).unwrap();
        assert_eq!(value["catalogHash"], catalog.hash);
        assert!(value["note"].as_str().unwrap().contains("no live workspace"));
    }

    #[test]
    fn catalog_resource_registry_serves_list_read_and_templates() {
        let registry = CatalogResourceRegistry::new(Arc::new(test_catalog()));
        assert_eq!(registry.list().len(), 2);
        assert_eq!(registry.templates().len(), 1);
        assert!(registry.read("semio://capability").is_ok());
        assert!(registry.read("semio://capability/cad.editor.translateSelection").is_ok());
        assert!(registry.read("semio://workspace").is_ok());
        assert!(registry.read("semio://not-a-resource").is_err());
        assert!(registry.subscribe("semio://workspace").is_ok());
        assert!(registry.unsubscribe("semio://workspace").is_ok());
    }
}
//#endregion 🧪️Tests
