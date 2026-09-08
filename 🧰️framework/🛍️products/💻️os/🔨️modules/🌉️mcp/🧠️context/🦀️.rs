//! 🧠️ Context broker + resource projection — packet `P2-catalog`/`W2-workspace-resources`,
//! `📋️master.md` §3.5. `context.resolve` returns the token-cheap `ContextSummary` (P1a's `🧬️schema`);
//! `WorkspaceResourceRegistry` (this file's `resources/list`/`resources/read` implementor) serves the
//! UNION of `semio://capability[/{id}]` (real, from the compiled `Catalog`, always) and
//! `semio://workspace[/artifacts]`/`semio://artifact/{id}[/…]` (real, delegated to the bound
//! `GatewayBackend` — progressive: with no workspace bound the URIs are still LISTED, but a `read`
//! against one is a structured, retryable `PLUGIN_UNAVAILABLE` naming the binding it needs, never
//! fabricated content — this module's brief §2.3: "do not fake workspace data").

use crate::catalog::Catalog;
use crate::errors::{GatewayError, GatewayErrorCode};
use crate::protocol::{GatewayBackend, Resource, ResourceContent, ResourceRegistry, ResourceTemplate};
use crate::schema::ContextSummary;
use crate::workspace::HeadlessWorkspace;
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
    let mut empty = working.clone();
    if let Some(entries) = empty.get_mut("entries").and_then(serde_json::Value::as_array_mut) {
        entries.clear();
    }
    let empty_bytes = serde_json::to_vec(&empty).unwrap_or_default().len();
    if let Some(object) = working.as_object_mut() {
        if let Some(serde_json::Value::Array(entries)) = object.get_mut("entries") {
            let original_len = entries.len();
            let mut current_bytes = empty_bytes;
            let mut retained = 0;
            for entry in entries.iter() {
                let entry_bytes = serde_json::to_vec(entry).unwrap_or_default().len();
                let separator_bytes = usize::from(retained > 0);
                if current_bytes.saturating_add(separator_bytes).saturating_add(entry_bytes) > byte_budget {
                    break;
                }
                current_bytes += separator_bytes + entry_bytes;
                retained += 1;
            }
            entries.truncate(retained);
            omitted.extend((retained..original_len).map(|index| format!("/entries/{index}")));
        }
    }
    let final_bytes = serde_json::to_vec(&working).unwrap_or_default().len();
    Truncated { value: working, omitted, token_estimate: estimate_tokens(final_bytes) }
}
//#endregion 🔖️TokenBudget

//#region 🔖️ContextResolve
/// 🪪️ Mints a fresh, opaque session id — blake3-mixed `(principal, wall-clock ms, call counter)`, the
/// same "no new dependency, `blake3` already in-tree" precedent `🎫️handles/🦀️.rs`'s own
/// `mint_id` set (`📓️terra-P1b-report.md` §2.2). A real `AgentSession` (handle-table-backed,
/// refreshed on every resolve) is P6/P7 work — this is `context.resolve`'s minimal, real-today
/// contract: a stable-looking id, not a persisted session record.
pub fn mint_session_id(principal: &str, counter: u64) -> String {
    let now_ms = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|duration| duration.as_millis()).unwrap_or(0);
    format!("sess_{}", framework_hash::hash_bytes(format!("{principal}:{now_ms}:{counter}").as_bytes()))
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

//#region 🔖️WorkspaceResourceRegistry
/// 🗂️ The real `ResourceRegistry` this crate registers into `McpServer` — the UNION of the compiled
/// `Catalog`'s resources (`semio://capability`[/{id}], token-budgeted via `truncate_to_budget`,
/// unchanged from this file's earlier `CatalogResourceRegistry`) and, when a workspace is bound,
/// everything the live `GatewayBackend` reports (`semio://workspace`, `semio://workspace/artifacts`,
/// `semio://artifact/{id}[/…]`). Holds the SAME `Arc<HeadlessWorkspace>` `McpServer::backend` wraps
/// (`build_server_with_workspace` clones it once more — a cheap refcount bump, never a second,
/// divergent `HeadlessWorkspace` instance answering for the same folder/hub).
///
/// `list`/`templates` never depend on whether a workspace is bound — mirrors `🦀️.rs`'s
/// `DECLARED_STUB_TOOL_NAMES` convention: a resource's PRESENCE never depends on tier, only its
/// `read` RESULT does. With no workspace bound, a read against a workspace URI is a structured,
/// retryable `PLUGIN_UNAVAILABLE` naming the binding it needs (`--folder`/`--hub`) — never the
/// fabricated body this region used to synthesize. `subscribe`/`unsubscribe` stay accepted no-ops:
/// `GatewayBackend` itself declares no change-stream method for either registry to delegate to.
pub struct WorkspaceResourceRegistry {
    catalog: Arc<Catalog>,
    workspace: Option<Arc<HeadlessWorkspace>>,
    bridge: Option<crate::ui::BridgeSlot>,
}

impl WorkspaceResourceRegistry {
    /// 🕳️ Bare tier — no workspace bound.
    pub fn new(catalog: Arc<Catalog>) -> Self {
        Self { catalog, workspace: None, bridge: None }
    }

    /// 🏠️ Headless/attached tier — `workspace` MUST be the exact `Arc<HeadlessWorkspace>`
    /// `McpServer::backend` (`GatewayBackends::WorkspaceArc`) also holds.
    pub fn with_workspace(catalog: Arc<Catalog>, workspace: Arc<HeadlessWorkspace>) -> Self {
        Self { catalog, workspace: Some(workspace), bridge: None }
    }

    /// 🔌️ Binds the late-filled `/bridge` slot so `semio://window…`, `semio://ui/…` and
    /// `semio://job/{id}` read through to the attached shell once one dials in. An unset or unfilled
    /// slot is the normal headless tier: those URIs still LIST, their reads degrade to a typed,
    /// retryable error.
    #[must_use]
    pub fn with_bridge(mut self, bridge: Option<crate::ui::BridgeSlot>) -> Self {
        self.bridge = bridge;
        self
    }

    fn is_workspace_uri(uri: &str) -> bool {
        uri == "semio://workspace" || uri == "semio://workspace/artifacts" || uri.starts_with("semio://workspace/scopes/") || uri.starts_with("semio://artifact/")
    }

    /// 🕳️ Structured, retryable `PLUGIN_UNAVAILABLE` naming the binding a workspace URI needs —
    /// never a protocol-level failure, never a fabricated body.
    fn workspace_binding_required(uri: &str) -> GatewayError {
        GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("`{uri}` needs a live workspace — bind one with `--folder <path>` or `--hub <url> --space <id>`"))
            .with_details(serde_json::json!({ "uri": uri, "bindWith": ["--folder", "--hub"] }))
            .retryable()
    }
}

impl ResourceRegistry for WorkspaceResourceRegistry {
    fn list(&self) -> Vec<Resource> {
        let mut resources = vec![
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
                description: Some("Active space and its artifacts (PLUGIN_UNAVAILABLE until --folder/--hub is bound)".to_string()),
                mime_type: Some("application/json".to_string()),
                size: None,
            },
            Resource {
                uri: "semio://workspace/artifacts".to_string(),
                name: "workspace-artifacts".to_string(),
                title: Some("Workspace artifact ids".to_string()),
                description: Some("Every artifact id open in this workspace (PLUGIN_UNAVAILABLE until --folder/--hub is bound)".to_string()),
                mime_type: Some("application/json".to_string()),
                size: None,
            },
        ];
        if let Some(workspace) = &self.workspace {
            if let Ok(live) = workspace.list_resources() {
                resources.extend(live.into_iter().filter(|resource| resource.uri != "semio://workspace"));
            }
        }
        resources.extend(crate::ui::ui_resources(self.bridge.as_ref()));
        resources.extend(crate::inference::inference_resources(self.workspace.as_ref()));
        resources
    }

    fn templates(&self) -> Vec<ResourceTemplate> {
        vec![
            ResourceTemplate {
                uri_template: "semio://capability/{id}".to_string(),
                name: "capability".to_string(),
                title: Some("One capability".to_string()),
                description: Some("Full CapabilityDefinition by id".to_string()),
                mime_type: Some("application/json".to_string()),
            },
            ResourceTemplate {
                uri_template: "semio://artifact/{artifactId}".to_string(),
                name: "artifact".to_string(),
                title: Some("One artifact".to_string()),
                description: Some("Real pack+spr bytes for one open artifact — /history and /validation are readable sub-resources of the same id".to_string()),
                mime_type: Some("application/octet-stream".to_string()),
            },
            ResourceTemplate {
                uri_template: "semio://artifact/{artifactId}/inference/{field}".to_string(),
                name: "artifact-inference".to_string(),
                title: Some("One declared inference of one artifact".to_string()),
                description: Some("A single inference field declared by the artifact's own plugin — /inference alone lists the roster".to_string()),
                mime_type: Some("application/json".to_string()),
            },
        ]
        .into_iter()
        .chain(crate::ui::ui_resource_templates())
        .collect()
    }

    fn read(&self, uri: &str) -> Result<Vec<ResourceContent>, GatewayError> {
        if uri == "semio://capability" {
            return capability_resource_contents(&self.catalog, None);
        }
        if let Some(id) = uri.strip_prefix("semio://capability/") {
            return capability_resource_contents(&self.catalog, Some(id));
        }
        if let Some(outcome) = crate::inference::read_inference_resource(uri, self.workspace.as_ref()) {
            return outcome;
        }
        if let Some(outcome) = crate::ui::read_ui_resource(uri, self.bridge.as_ref(), self.workspace.as_ref()) {
            return outcome;
        }
        if Self::is_workspace_uri(uri) {
            return match &self.workspace {
                Some(workspace) => workspace.read_resource(uri),
                None => Err(Self::workspace_binding_required(uri)),
            };
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
//#endregion 🔖️WorkspaceResourceRegistry

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️quick/🦀️.rs"]
mod quick;
//#endregion 🧪️Tests
