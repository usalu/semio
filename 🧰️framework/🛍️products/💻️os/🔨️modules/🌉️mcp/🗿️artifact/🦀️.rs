//! 🗿️ `artifact_create|open|validate|export|snapshot` — ticket 26/08/29/AI-MCP-END-TO-END packet
//! W4, closing 5 of the 9 `DECLARED_STUB_TOOL_NAMES` at `🦀️.rs:244`. Every tool is always
//! registered (present in `tools/list` unconditionally); only the RESULT of a call degrades across
//! the module-wide 3-tier contract (`📓️status.md` "Progressive-enhancement contract"): (1) no
//! `HeadlessWorkspace` bound → a retryable `PLUGIN_UNAVAILABLE` naming `--folder`/`--hub`; (2) a
//! workspace is bound but names zero registered plugins (`require_workspace_has_a_plugin`) → the
//! same real, retryable `PLUGIN_UNAVAILABLE`; (3) fully bound → a real answer built ONLY from
//! `🏠️workspace`'s own public API — this file duplicates none of its logic. Generic over artifact
//! kind throughout: no plugin id is ever hardcoded — `open`/`create`/`validate`/`snapshot` need no
//! specific plugin at all (ticket 26/08/29/AI-MCP-END-TO-END packet W8, `📓️w8-capability-routing.md`);
//! `artifact_export` alone needs one, resolved by `require_resolvable_export_plugin`, honest about
//! the one thing it cannot yet do — see that fn's own doc.
//!
//! 🆕️📤️ `artifact_create`'s `kind` and `artifact_export` are both routed for real (ticket 26/09/18
//! slice A1, closing `📓️g7-mcp-agent-and-collaboration-audit.md` §6 P1.4/P1.5): `kind` is validated
//! against the artifact kinds the INSTALLED plugins declare
//! ([`HeadlessWorkspace::installed_artifact_kinds`]) and, for a real plugin kind, the artifact is
//! seeded from that plugin's own freshly-opened document (`AppCommand::ReadArtifact`) and persisted
//! under its real schema id — which is also what finally gives `artifact_export` an artifact → plugin
//! mapping. `artifact_export` then drives the owning app's own media OUT port
//! (`AppCommand::ExportMedia` → the guest's `LoadDocument` + `MediaOut`, the identical pair `🏃️run`'s
//! workflow executor uses) and returns the guest's own bytes, never a synthesized export.
//!
//! 🚧️ **Known, honest gap** (not fabricated, not hidden): `artifact_validate` still forwards the real
//! "the wire protocol has no validate query command yet" gap `read_artifact_resource`'s `validation`
//! arm answers with — never a hardcoded `{"valid": true}`.

use crate::catalog::{CapabilityAudience, CapabilityDefinition, CapabilityKind, CapabilityOwner, CapabilityPresentation, CapabilityRef, CapabilitySource, ToolExposure};
use crate::errors::{GatewayError, GatewayErrorCode};
use crate::tool_from_capability;
use crate::protocol::{CallToolResult, ContentBlock, GatewayBackend, InMemoryToolRegistry};
use crate::schema::{
    artifact_create_input_schema, artifact_create_output_schema, artifact_export_input_schema, artifact_export_output_schema, artifact_open_input_schema, artifact_open_output_schema, artifact_snapshot_input_schema,
    artifact_snapshot_output_schema, artifact_validate_input_schema, artifact_validate_output_schema, RevisionStamp,
};
use crate::workspace::{find_plugin_entry, find_repo_root, load_package_descriptor, load_plugin_registry, HeadlessWorkspace};
use std::sync::Arc;

//#region 🔖️Schemas
// 📐️ Every `artifact_*` tool schema is a named export of `🧬️schema/🦀️.rs` (scope `os.mcp`), imported
// above — this facet stamps none of its own.
//#endregion 🔖️Schemas

//#region 🔖️Capabilities
fn artifact_capability(id: &str, tool_name: &str, kind: CapabilityKind, icon_id: &str, title: &str, description: &str, use_when: Vec<String>, input_schema: serde_json::Value, output_schema: serde_json::Value) -> CapabilityDefinition {
    CapabilityDefinition {
        id: CapabilityRef(id.to_string()),
        version: 1,
        owner: CapabilityOwner::Gateway,
        kind,
        audience: CapabilityAudience::Agent,
        title: title.to_string(),
        description: description.to_string(),
        artifact_kind: None,
        use_when,
        input_schema,
        output_schema,
        effects: match kind {
            CapabilityKind::Mutation => semio_framework::manifest::CapabilityEffects { writes: vec![semio_framework::manifest::ResourceSelector::new("artifact:{self}")], ..Default::default() },
            _ => Default::default(),
        },
        policy: Default::default(),
        execution: Default::default(),
        exposure: ToolExposure::Direct { tool_name: tool_name.to_string() },
        presentation: CapabilityPresentation { icon_id: Some(icon_id.to_string()), category: Some("gateway".to_string()), keys: None, in_palette: false, args: Vec::new() },
        examples: Vec::new(),
        source: CapabilitySource::Gateway,
    }
}

/// 🗿️ The five artifact capabilities, folded into `CatalogSource.gateway` exactly like the 3 core
/// gateway tools (`🦀️.rs`'s `core_tool_capabilities`) so they compile into the catalog for
/// real — searchable, describable, `semio://capability/{id}`-readable — never just a bare `Tool`
/// with a placeholder schema.
pub fn artifact_capabilities() -> Vec<CapabilityDefinition> {
    vec![
        artifact_capability(
            "artifact.open",
            "artifact_open",
            CapabilityKind::Query,
            "file",
            "Open Artifact",
            "Opens an existing artifact by id and returns its identity, kind, revision stamp and size — not its full body (that is the semio://artifact/{id} resource's job).",
            vec!["open an artifact".to_string(), "what artifacts exist".to_string()],
            artifact_open_input_schema(),
            artifact_open_output_schema(),
        ),
        artifact_capability(
            "artifact.create",
            "artifact_create",
            CapabilityKind::Mutation,
            "add",
            "Create Artifact",
            "Creates a new artifact of a given kind in the bound workspace through the ordinary envelope/store path.",
            vec!["create a new artifact".to_string(), "start a new document".to_string()],
            artifact_create_input_schema(),
            artifact_create_output_schema(),
        ),
        artifact_capability(
            "artifact.validate",
            "artifact_validate",
            CapabilityKind::Query,
            "check",
            "Validate Artifact",
            "Runs real plugin validation for an artifact — never a fabricated pass.",
            vec!["validate an artifact".to_string(), "is this artifact valid".to_string()],
            artifact_validate_input_schema(),
            artifact_validate_output_schema(),
        ),
        artifact_capability(
            "artifact.snapshot",
            "artifact_snapshot",
            CapabilityKind::Query,
            "camera",
            "Snapshot Artifact",
            "Returns a real content snapshot of an artifact at its current revision.",
            vec!["snapshot an artifact".to_string(), "read the current content".to_string()],
            artifact_snapshot_input_schema(),
            artifact_snapshot_output_schema(),
        ),
        artifact_capability(
            "artifact.export",
            "artifact_export",
            CapabilityKind::Query,
            "download",
            "Export Artifact",
            "Enumerates the real export formats the artifact's plugin declares and reports whether a live export command is reachable.",
            vec!["export an artifact".to_string(), "what formats can this export to".to_string()],
            artifact_export_input_schema(),
            artifact_export_output_schema(),
        ),
    ]
}
//#endregion 🔖️Capabilities

//#region 🔖️Tiering
/// 🥉️ Tier 1: no workspace bound at all — retryable, names exactly what binding closes the gap.
fn require_workspace(workspace: &Option<Arc<HeadlessWorkspace>>) -> Result<&Arc<HeadlessWorkspace>, GatewayError> {
    workspace.as_ref().ok_or_else(|| GatewayError::new(GatewayErrorCode::PluginUnavailable, "no workspace is bound to this session — start the gateway with --folder <dir> or --hub <url> --space <id> to use artifact tools").retryable())
}

/// 🥈️ Tier 2, for `artifact_open`/`create`/`validate`/`snapshot`: at least one plugin must be
/// registered for there to be anything to act against. The OLD gate here (`resolve_default_plugin_id`,
/// deleted by ticket 26/08/29/AI-MCP-END-TO-END packet W8, `📓️w8-capability-routing.md`) demanded
/// EXACTLY one — a single-plugin-workspace artifact of the pre-routing era, not a real requirement of
/// these four handlers: none of them route through a specific plugin's `ArtifactChannel` at all (they
/// read/write this workspace's own generic, schema-agnostic probe documents — `🏠️workspace`'s own
/// module doc), so two-or-more registered plugins is no longer inherently blocking.
fn require_workspace_has_a_plugin(workspace: &HeadlessWorkspace) -> Result<(), GatewayError> {
    if workspace.catalog_plugin_ids().is_empty() {
        return Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, "no plugin-owned capability is registered in this workspace's catalog — nothing for artifact tools to act against").retryable());
    }
    Ok(())
}

/// 🥈️ `artifact_export` alone needs a SPECIFIC plugin id (to read its committed `export_formats`
/// via `resolve_plugin_export_formats`). This workspace has no `artifact_id` → `plugin_id` mapping —
/// every artifact it manages today is the generic, schema-agnostic probe document `🏠️workspace`'s own
/// module doc describes, never a plugin-typed one — so the only honest answer is the workspace's
/// plugin when there is EXACTLY one; two or more is a typed, named gap (never a guess at which one
/// owns `artifact_id`), same real, retryable `PLUGIN_UNAVAILABLE` shape the deleted
/// `resolve_default_plugin_id` used to build.
fn require_resolvable_export_plugin(workspace: &HeadlessWorkspace, artifact_id: &str) -> Result<String, GatewayError> {
    if let Some(binding) = workspace.plugin_artifact_binding(artifact_id) {
        return Ok(binding.plugin_id);
    }
    let plugin_ids = workspace.catalog_plugin_ids();
    match plugin_ids.len() {
        0 => Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, "no plugin-owned capability is registered in this workspace's catalog — nothing to export against").retryable()),
        1 => Ok(plugin_ids.into_iter().next().expect("exactly one plugin id checked above")),
        _ => Err(GatewayError::new(
            GatewayErrorCode::PluginUnavailable,
            format!("artifact `{artifact_id}` cannot be matched to one of this workspace's {} registered plugins ({}) — no artifact-to-plugin mapping exists yet", plugin_ids.len(), plugin_ids.join(", ")),
        )
        .retryable()),
    }
}

fn require_field<'a>(arguments: &'a serde_json::Value, field: &str) -> Result<&'a str, GatewayError> {
    arguments.get(field).and_then(serde_json::Value::as_str).filter(|value| !value.is_empty()).ok_or_else(|| GatewayError::new(GatewayErrorCode::InputInvalid, format!("{field} is required")))
}
//#endregion 🔖️Tiering

//#region 🔖️ArtifactMetadata
/// 🔎️ Best-effort real schema id for `artifact_id` — `None` when this workspace cannot answer (a
/// cold, disk-only artifact never opened this session, or the real wire protocol has no
/// schema-describe command for it yet). Built ONLY from the public `read_resource` API
/// (`semio://artifact/{id}/schema`) — never a second decoder for `🏠️workspace`'s own resource body.
fn resolve_artifact_schema_id(workspace: &Arc<HeadlessWorkspace>, artifact_id: &str) -> Option<String> {
    artifact_schema_resource_field(workspace, artifact_id, "schema")
}

/// 🪢 The owning app's dialect coordinate (`s.gis.gismap`) — the id space `capabilities_search`'s
/// `artifactKind` filter and every capability row's own `artifactKind` are spelled in, which is NOT
/// the pack schema (`gis.map`) [`resolve_artifact_schema_id`] answers. They are two vocabularies of
/// one document; a workspace that knows only one of them answers `None` for the other rather than
/// passing the wrong one off as it.
fn resolve_artifact_kind_id(workspace: &Arc<HeadlessWorkspace>, artifact_id: &str) -> Option<String> {
    artifact_schema_resource_field(workspace, artifact_id, "artifactKind")
}

fn artifact_schema_resource_field(workspace: &Arc<HeadlessWorkspace>, artifact_id: &str, field: &str) -> Option<String> {
    let contents = workspace.read_resource(&format!("semio://artifact/{artifact_id}/schema")).ok()?;
    let text = contents.first()?.text.as_ref()?;
    let value: serde_json::Value = serde_json::from_str(text).ok()?;
    value.get(field)?.as_str().map(str::to_string)
}

/// 🔎️ Best-effort real `RevisionStamp` for `artifact_id`, derived from the same `appliedEditIds`
/// list `semio://artifact/{id}/history` already answers with — `None` under the same conditions as
/// [`resolve_artifact_schema_id`].
fn resolve_artifact_revision(workspace: &Arc<HeadlessWorkspace>, artifact_id: &str) -> Option<RevisionStamp> {
    let contents = workspace.read_resource(&format!("semio://artifact/{artifact_id}/history")).ok()?;
    let text = contents.first()?.text.as_ref()?;
    let value: serde_json::Value = serde_json::from_str(text).ok()?;
    let applied_edit_ids = value.get("appliedEditIds")?.as_array()?;
    let head_edit_id = applied_edit_ids.last()?.as_str()?.to_string();
    Some(RevisionStamp { artifact_id: artifact_id.to_string(), head_edit_id, cursor: applied_edit_ids.len().to_string() })
}

/// 🗂️ The resolved plugin's real, committed export formats (`ArtifactKindSpec.export_formats`,
/// unioned across every app it declares) — read straight from `🔣️.json` via
/// `🏠️workspace`'s own public plugin-path helpers, never invented.
fn resolve_plugin_export_formats(plugin_id: &str) -> Result<Vec<String>, GatewayError> {
    let repo_root = find_repo_root()?;
    let registry = load_plugin_registry(&repo_root)?;
    let entry = find_plugin_entry(&registry, plugin_id)?;
    let descriptor = load_package_descriptor(&entry.owner_root)?;
    let mut formats = std::collections::BTreeSet::new();
    for app in &descriptor.manifest.apps {
        for artifact_kind in &app.artifact_kinds {
            formats.extend(artifact_kind.export_formats.iter().cloned());
        }
    }
    Ok(formats.into_iter().collect())
}
//#endregion 🔖️ArtifactMetadata

//#region 🔖️Handlers
/// 🌎️ Projects the binding `artifact_open` just minted for a HUB document — see
/// `crate::schema::session_document_shape` for why `writePath` is a fact the agent is owed before it
/// spends a mutation rather than a detail it discovers when one goes nowhere.
fn session_document_report(workspace: &Arc<HeadlessWorkspace>, artifact_id: &str) -> Option<serde_json::Value> {
    let binding = workspace.plugin_artifact_binding(artifact_id)?;
    #[cfg(not(target_arch = "wasm32"))]
    let write_path = match (&binding.backbone, &binding.backbone_blocked_by) {
        (Some(_), _) => "open".to_string(),
        (None, Some(reason)) => reason.clone(),
        (None, None) => "this document has no open document actor".to_string(),
    };
    #[cfg(target_arch = "wasm32")]
    let write_path = binding.backbone_blocked_by.clone().unwrap_or_else(|| "this target opens no document actor".to_string());
    Some(serde_json::json!({
        "pluginId": binding.plugin_id,
        "appId": binding.app_id,
        "surfaceId": binding.surface_id,
        "packBytes": binding.document.as_ref().map_or(0, |pair| pair.pack.len()),
        "sprBytes": binding.document.as_ref().map_or(0, |pair| pair.spr.len()),
        "writePath": write_path,
        "relayedBatches": binding.relayed.load(std::sync::atomic::Ordering::Relaxed),
    }))
}

fn artifact_open_handler(workspace: &Option<Arc<HeadlessWorkspace>>, arguments: serde_json::Value) -> CallToolResult {
    let artifact_id = match require_field(&arguments, "artifactId") {
        Ok(value) => value.to_string(),
        Err(error) => return CallToolResult::tool_error(&error),
    };
    let workspace = match require_workspace(workspace) {
        Ok(workspace) => workspace,
        Err(error) => return CallToolResult::tool_error(&error),
    };
    if let Err(error) = require_workspace_has_a_plugin(workspace) {
        return CallToolResult::tool_error(&error);
    }
    match workspace.authenticated_probe_document_is_known(&artifact_id) {
        Ok(true) => {
            if let Err(error) = semio_framework::io::resolve_ready(workspace.ensure_probe_artifact(&artifact_id, serde_json::Value::Null)) {
                return CallToolResult::tool_error(&error);
            }
        }
        Ok(false) => {}
        Err(error) => return CallToolResult::tool_error(&error),
    }
    match workspace.read_artifact_bytes(&artifact_id) {
        Err(error) => CallToolResult::tool_error(&error),
        Ok(None) => CallToolResult::tool_error(&GatewayError::new(GatewayErrorCode::NotFound, format!("no such artifact: {artifact_id}"))),
        Ok(Some((pack, spr))) => {
            // 🌎️ Opening a HUB document is what binds it to the plugin, app and surface the hub's own
            // execution-target lease names, and hands that binding the canonical pair just read — so
            // every later `action_prepare` on this session runs the plugin's guest against THIS
            // document rather than against the plugin's genesis (ticket 26/09/18 slice M10, step 1 of
            // M8 §5.3's write path). A workspace that is not hub-bound, or a document the hub
            // authorizes no execution target for, binds nothing and still opens.
            let bound = match workspace.bind_hub_session_document(&artifact_id, &pack, &spr) {
                Ok(bound) => bound,
                Err(error) => return CallToolResult::tool_error(&error),
            };
            let structured = serde_json::json!({
                "artifactId": artifact_id,
                "kind": resolve_artifact_schema_id(workspace, &artifact_id),
                "artifactKind": resolve_artifact_kind_id(workspace, &artifact_id),
                "revision": resolve_artifact_revision(workspace, &artifact_id),
                "sizeBytes": pack.len() + spr.len(),
                "sessionDocument": bound.then(|| session_document_report(workspace, &artifact_id)).flatten(),
            });
            CallToolResult::ok(vec![ContentBlock::Text { text: format!("opened {artifact_id}") }], Some(structured))
        }
    }
}

fn artifact_create_handler(workspace: &Option<Arc<HeadlessWorkspace>>, arguments: serde_json::Value) -> CallToolResult {
    let artifact_id = match require_field(&arguments, "artifactId") {
        Ok(value) => value.to_string(),
        Err(error) => return CallToolResult::tool_error(&error),
    };
    let kind = match require_field(&arguments, "kind") {
        Ok(value) => value.to_string(),
        Err(error) => return CallToolResult::tool_error(&error),
    };
    let workspace = match require_workspace(workspace) {
        Ok(workspace) => workspace,
        Err(error) => return CallToolResult::tool_error(&error),
    };
    if let Err(error) = require_workspace_has_a_plugin(workspace) {
        return CallToolResult::tool_error(&error);
    }
    match workspace.workspace_artifact_ids() {
        Err(error) => return CallToolResult::tool_error(&error),
        Ok(existing) if existing.iter().any(|id| id == &artifact_id) => {
            return CallToolResult::tool_error(&GatewayError::new(GatewayErrorCode::PreconditionFailed, format!("artifact `{artifact_id}` already exists")));
        }
        Ok(_) => {}
    }
    if kind != crate::workspace::PROBE_SCHEMA {
        let installed = match workspace.installed_artifact_kinds() {
            Ok(kinds) => kinds,
            Err(error) => return CallToolResult::tool_error(&error),
        };
        let Some(declared) = installed.iter().find(|row| row.schema == kind) else {
            let known: Vec<&str> = std::iter::once(crate::workspace::PROBE_SCHEMA).chain(installed.iter().map(|row| row.schema.as_str())).collect();
            return CallToolResult::tool_error(
                &GatewayError::new(GatewayErrorCode::InputInvalid, format!("no installed plugin declares artifact kind `{kind}`"))
                    .with_details(serde_json::json!({ "requestedKind": kind, "installedKinds": known })),
            );
        };
        return match workspace.create_plugin_artifact(&artifact_id, declared) {
            Ok((pack_bytes, spr_bytes)) => CallToolResult::ok(
                vec![ContentBlock::Text { text: format!("created {artifact_id} as {kind}") }],
                Some(serde_json::json!({ "artifactId": artifact_id, "kind": kind, "pluginId": declared.plugin_id, "appId": declared.app_id, "sizeBytes": pack_bytes + spr_bytes, "revision": resolve_artifact_revision(workspace, &artifact_id) })),
            ),
            Err(error) => CallToolResult::tool_error(&error),
        };
    }
    let initial = arguments.get("initial").cloned().unwrap_or_else(|| serde_json::json!({}));
    match semio_framework::io::resolve_ready(workspace.ensure_probe_artifact(&artifact_id, initial)) {
        Ok(revision) => CallToolResult::ok(vec![ContentBlock::Text { text: format!("created {artifact_id}") }], Some(serde_json::json!({ "artifactId": artifact_id, "kind": kind, "revision": revision }))),
        Err(error) => CallToolResult::tool_error(&error),
    }
}

fn artifact_validate_handler(workspace: &Option<Arc<HeadlessWorkspace>>, arguments: serde_json::Value) -> CallToolResult {
    let artifact_id = match require_field(&arguments, "artifactId") {
        Ok(value) => value.to_string(),
        Err(error) => return CallToolResult::tool_error(&error),
    };
    let workspace = match require_workspace(workspace) {
        Ok(workspace) => workspace,
        Err(error) => return CallToolResult::tool_error(&error),
    };
    if let Err(error) = require_workspace_has_a_plugin(workspace) {
        return CallToolResult::tool_error(&error);
    }
    match workspace.read_resource(&format!("semio://artifact/{artifact_id}/validation")) {
        Ok(contents) => {
            let structured = contents.first().and_then(|content| content.text.as_deref()).and_then(|text| serde_json::from_str(text).ok()).unwrap_or_else(|| serde_json::json!({}));
            CallToolResult::ok(vec![ContentBlock::Text { text: format!("validated {artifact_id}") }], Some(structured))
        }
        Err(error) => CallToolResult::tool_error(&error),
    }
}

fn artifact_snapshot_handler(workspace: &Option<Arc<HeadlessWorkspace>>, arguments: serde_json::Value) -> CallToolResult {
    let artifact_id = match require_field(&arguments, "artifactId") {
        Ok(value) => value.to_string(),
        Err(error) => return CallToolResult::tool_error(&error),
    };
    let workspace = match require_workspace(workspace) {
        Ok(workspace) => workspace,
        Err(error) => return CallToolResult::tool_error(&error),
    };
    if let Err(error) = require_workspace_has_a_plugin(workspace) {
        return CallToolResult::tool_error(&error);
    }
    if let Some(requested) = arguments.get("revision") {
        let requested_stamp: Option<RevisionStamp> = serde_json::from_value(requested.clone()).ok();
        let current = resolve_artifact_revision(workspace, &artifact_id);
        if requested_stamp != current {
            return CallToolResult::tool_error(
                &GatewayError::new(GatewayErrorCode::PreconditionFailed, "only the artifact's current revision can be snapshotted — historical revision snapshots are not retrievable yet").with_details(serde_json::json!({ "requested": requested, "current": current })),
            );
        }
    }
    match workspace.read_resource(&format!("semio://artifact/{artifact_id}")) {
        Ok(contents) => {
            let structured = contents.first().and_then(|content| content.text.as_deref()).and_then(|text| serde_json::from_str(text).ok()).unwrap_or_else(|| serde_json::json!({}));
            CallToolResult::ok(vec![ContentBlock::Text { text: format!("snapshot of {artifact_id}") }], Some(structured))
        }
        Err(error) => CallToolResult::tool_error(&error),
    }
}

fn artifact_export_handler(workspace: &Option<Arc<HeadlessWorkspace>>, arguments: serde_json::Value) -> CallToolResult {
    let artifact_id = match require_field(&arguments, "artifactId") {
        Ok(value) => value.to_string(),
        Err(error) => return CallToolResult::tool_error(&error),
    };
    let workspace = match require_workspace(workspace) {
        Ok(workspace) => workspace,
        Err(error) => return CallToolResult::tool_error(&error),
    };
    let plugin_id = match require_resolvable_export_plugin(workspace, &artifact_id) {
        Ok(plugin_id) => plugin_id,
        Err(error) => return CallToolResult::tool_error(&error),
    };
    match workspace.read_artifact_bytes(&artifact_id) {
        Err(error) => return CallToolResult::tool_error(&error),
        Ok(None) => return CallToolResult::tool_error(&GatewayError::new(GatewayErrorCode::NotFound, format!("no such artifact: {artifact_id}"))),
        Ok(Some(_)) => {}
    }
    let requested_format = arguments.get("format").and_then(serde_json::Value::as_str).map(str::to_string);
    let ports = match export_ports_for(workspace, &artifact_id, &plugin_id) {
        Ok(ports) => ports,
        Err(error) => return CallToolResult::tool_error(&error),
    };
    let declared_formats = resolve_plugin_export_formats(&plugin_id).unwrap_or_default();
    let port = match &requested_format {
        Some(format) => match ports.iter().find(|port| *port == format) {
            Some(port) => port.clone(),
            None => {
                return CallToolResult::tool_error(
                    &GatewayError::new(GatewayErrorCode::InputInvalid, format!("plugin `{plugin_id}` exposes no output port `{format}` for `{artifact_id}`"))
                        .with_details(serde_json::json!({ "pluginId": plugin_id, "availablePorts": ports, "declaredExportFormats": declared_formats, "requestedFormat": format })),
                )
            }
        },
        None => match ports.first() {
            Some(port) => port.clone(),
            None => {
                return CallToolResult::tool_error(
                    &GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("plugin `{plugin_id}` declares no media output port to export `{artifact_id}` through"))
                        .with_details(serde_json::json!({ "pluginId": plugin_id, "declaredExportFormats": declared_formats }))
                        .retryable(),
                )
            }
        },
    };
    match workspace.export_artifact_media(&plugin_id, &artifact_id, &port) {
        Ok((descriptor, data, answered_port)) => CallToolResult::ok(
            vec![ContentBlock::Text { text: format!("exported {artifact_id} through `{answered_port}` ({} byte(s))", data.len()) }],
            Some(serde_json::json!({
                "artifactId": artifact_id,
                "format": answered_port,
                "mimeType": serde_json::Value::Null,
                "contentBase64": base64_encode(&data),
                "pluginId": plugin_id,
                "descriptorBytes": descriptor.len(),
                "availablePorts": ports,
                "declaredExportFormats": declared_formats,
            })),
        ),
        Err(error) => CallToolResult::tool_error(&error),
    }
}

/// 📤️ The output ports `artifact_id` can actually be exported through: the ports declared by the app
/// that owns this artifact's kind when this workspace minted it (`plugin_artifact_binding`), else
/// every port `plugin_id`'s editor apps declare. Real declared data either way — never a guess at a
/// port name.
fn export_ports_for(workspace: &Arc<HeadlessWorkspace>, artifact_id: &str, plugin_id: &str) -> Result<Vec<String>, GatewayError> {
    let installed = workspace.installed_artifact_kinds()?;
    if let Some(binding) = workspace.plugin_artifact_binding(artifact_id) {
        if let Some(row) = installed.iter().find(|row| row.schema == binding.schema) {
            return Ok(row.media_out_ports.clone());
        }
    }
    let mut ports: Vec<String> = installed.iter().filter(|row| row.plugin_id == plugin_id).flat_map(|row| row.media_out_ports.iter().cloned()).collect();
    ports.dedup();
    Ok(ports)
}

/// 🔢️ Standard base64 of the exported bytes — the `contentBase64` field `artifact.export`'s own
/// output schema declares.
fn base64_encode(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let triple = [chunk[0], *chunk.get(1).unwrap_or(&0), *chunk.get(2).unwrap_or(&0)];
        let packed = (u32::from(triple[0]) << 16) | (u32::from(triple[1]) << 8) | u32::from(triple[2]);
        out.push(ALPHABET[(packed >> 18) as usize & 63] as char);
        out.push(ALPHABET[(packed >> 12) as usize & 63] as char);
        out.push(if chunk.len() > 1 { ALPHABET[(packed >> 6) as usize & 63] as char } else { '=' });
        out.push(if chunk.len() > 2 { ALPHABET[packed as usize & 63] as char } else { '=' });
    }
    out
}
//#endregion 🔖️Handlers

//#region 🔖️Registration
/// 🗿️ Registers the five real artifact tools against `registry` — every tool is present regardless
/// of `workspace`; only a call's RESULT depends on the 3-tier contract (this file's own module doc).
pub fn register_artifact_tools(registry: &mut InMemoryToolRegistry, workspace: Option<Arc<HeadlessWorkspace>>) {
    let capabilities = artifact_capabilities();
    let capability = |id: &str| capabilities.iter().find(|capability| capability.id.as_str() == id).cloned().expect("artifact_capabilities defines this id");

    let open_tool = tool_from_capability(&capability("artifact.open"), "artifact_open");
    let open_workspace = workspace.clone();
    registry.register(open_tool, move |arguments| artifact_open_handler(&open_workspace, arguments)).expect("artifact_open is a valid tool name");

    let create_tool = tool_from_capability(&capability("artifact.create"), "artifact_create");
    let create_workspace = workspace.clone();
    registry.register(create_tool, move |arguments| artifact_create_handler(&create_workspace, arguments)).expect("artifact_create is a valid tool name");

    let validate_tool = tool_from_capability(&capability("artifact.validate"), "artifact_validate");
    let validate_workspace = workspace.clone();
    registry.register(validate_tool, move |arguments| artifact_validate_handler(&validate_workspace, arguments)).expect("artifact_validate is a valid tool name");

    let snapshot_tool = tool_from_capability(&capability("artifact.snapshot"), "artifact_snapshot");
    let snapshot_workspace = workspace.clone();
    registry.register(snapshot_tool, move |arguments| artifact_snapshot_handler(&snapshot_workspace, arguments)).expect("artifact_snapshot is a valid tool name");

    let export_tool = tool_from_capability(&capability("artifact.export"), "artifact_export");
    registry.register(export_tool, move |arguments| artifact_export_handler(&workspace, arguments)).expect("artifact_export is a valid tool name");
}
//#endregion 🔖️Registration

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️quick/🦀️.rs"]
mod quick;
//#endregion 🧪️Tests
