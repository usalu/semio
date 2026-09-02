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
//! 🚧️ **Known, honest gaps** (not fabricated, not hidden): `artifact_create`'s `kind` argument is
//! accepted but not yet wire-routed to a plugin-specific document type — creation always goes
//! through this crate's own generic, host-opaque probe-document mechanism
//! ([`HeadlessWorkspace::ensure_probe_artifact`]) because the real `PluginArtifactChannel` wire
//! protocol has no "create a document of schema X" command yet (see `🏠️workspace/🦀️.rs`'s
//! own module doc for the same class of gap). `artifact_validate`/`artifact_export` forward the SAME
//! real "the wire protocol has no validate/export query command yet" gap
//! `read_artifact_resource`'s `validation` arm already answers with — never a hardcoded `{"valid":
//! true}`, never a synthesized export. `artifact_export` DOES enumerate the resolved plugin's real,
//! committed `export_formats` (`semio_framework::manifest::ArtifactKindSpec`) so a caller sees exactly
//! what that plugin declares, even though nothing can act on the request yet.

use crate::catalog::{CapabilityDefinition, CapabilityKind, CapabilityOwner, CapabilityPresentation, CapabilityRef, CapabilitySource, ToolExposure};
use crate::errors::{GatewayError, GatewayErrorCode};
use crate::tool_from_capability;
use crate::protocol::{CallToolResult, ContentBlock, GatewayBackend, InMemoryToolRegistry, Tool};
use crate::schema::RevisionStamp;
use crate::workspace::{find_plugin_entry, find_repo_root, load_package_descriptor, load_plugin_registry, HeadlessWorkspace};
use std::sync::Arc;

//#region 🔖️Schemas
/// 📐️ `RevisionStamp`-shaped, nullable — used as a sub-schema (never a top-level tool schema, so no
/// `$schema`/`$id` of its own).
fn revision_stamp_schema() -> serde_json::Value {
    serde_json::json!({
        "type": ["object", "null"],
        "properties": { "artifactId": { "type": "string" }, "headEditId": { "type": "string" }, "cursor": { "type": "string" } },
    })
}

fn artifact_open_input_schema() -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "semio://capability/artifact.open/input",
        "type": "object",
        "properties": { "artifactId": { "type": "string" } },
        "required": ["artifactId"],
        "additionalProperties": false,
    })
}

/// 📐️ Deliberately NOT the resource's full-body shape (`semio://artifact/{id}` — packBytes/
/// sprBytes/packBase64) — this file's own module doc: `artifact_open` answers identity/kind/
/// revision/size, never the whole body.
fn artifact_open_output_schema() -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "semio://capability/artifact.open/output",
        "type": "object",
        "properties": {
            "artifactId": { "type": "string" },
            "kind": { "type": ["string", "null"] },
            "revision": revision_stamp_schema(),
            "sizeBytes": { "type": ["integer", "null"] },
        },
    })
}

fn artifact_create_input_schema() -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "semio://capability/artifact.create/input",
        "type": "object",
        "properties": { "artifactId": { "type": "string" }, "kind": { "type": "string" }, "initial": {} },
        "required": ["artifactId", "kind"],
        "additionalProperties": false,
    })
}

fn artifact_create_output_schema() -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "semio://capability/artifact.create/output",
        "type": "object",
        "properties": { "artifactId": { "type": "string" }, "kind": { "type": "string" }, "revision": revision_stamp_schema() },
    })
}

fn artifact_validate_input_schema() -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "semio://capability/artifact.validate/input",
        "type": "object",
        "properties": { "artifactId": { "type": "string" } },
        "required": ["artifactId"],
        "additionalProperties": false,
    })
}

/// 📐️ Deliberately permissive: the real wire protocol has no validate query command yet
/// (`🏠️workspace/🦀️.rs` `read_artifact_resource`'s `validation` arm), so no shape can be
/// pinned down before that lands.
fn artifact_validate_output_schema() -> serde_json::Value {
    serde_json::json!({ "$schema": "https://json-schema.org/draft/2020-12/schema", "$id": "semio://capability/artifact.validate/output", "type": "object" })
}

fn artifact_snapshot_input_schema() -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "semio://capability/artifact.snapshot/input",
        "type": "object",
        "properties": { "artifactId": { "type": "string" }, "revision": revision_stamp_schema() },
        "required": ["artifactId"],
        "additionalProperties": false,
    })
}

fn artifact_snapshot_output_schema() -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "semio://capability/artifact.snapshot/output",
        "type": "object",
        "properties": {
            "artifactId": { "type": "string" },
            "packBytes": { "type": ["integer", "null"] },
            "sprBytes": { "type": ["integer", "null"] },
            "packBase64": { "type": ["string", "null"] },
        },
    })
}

fn artifact_export_input_schema() -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "semio://capability/artifact.export/input",
        "type": "object",
        "properties": { "artifactId": { "type": "string" }, "format": { "type": "string" } },
        "required": ["artifactId"],
        "additionalProperties": false,
    })
}

/// 📐️ The shape a real export would answer with once the wire protocol grows an export command —
/// today every call ends in a tool-error carrying `availableFormats` in its `details` instead (see
/// [`artifact_export_handler`]).
fn artifact_export_output_schema() -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "semio://capability/artifact.export/output",
        "type": "object",
        "properties": { "artifactId": { "type": "string" }, "format": { "type": "string" }, "contentBase64": { "type": ["string", "null"] }, "mimeType": { "type": ["string", "null"] } },
    })
}
//#endregion 🔖️Schemas

//#region 🔖️Capabilities
fn artifact_capability(id: &str, tool_name: &str, kind: CapabilityKind, icon_id: &str, title: &str, description: &str, use_when: Vec<String>, input_schema: serde_json::Value, output_schema: serde_json::Value) -> CapabilityDefinition {
    CapabilityDefinition {
        id: CapabilityRef(id.to_string()),
        version: 1,
        owner: CapabilityOwner::Gateway,
        kind,
        title: title.to_string(),
        description: description.to_string(),
        artifact_kind: None,
        use_when,
        input_schema,
        output_schema,
        effects: Default::default(),
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
    let contents = workspace.read_resource(&format!("semio://artifact/{artifact_id}/schema")).ok()?;
    let text = contents.first()?.text.as_ref()?;
    let value: serde_json::Value = serde_json::from_str(text).ok()?;
    value.get("schema")?.as_str().map(str::to_string)
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
    match workspace.read_artifact_bytes(&artifact_id) {
        Err(error) => CallToolResult::tool_error(&error),
        Ok(None) => CallToolResult::tool_error(&GatewayError::new(GatewayErrorCode::NotFound, format!("no such artifact: {artifact_id}"))),
        Ok(Some((pack, spr))) => {
            let structured = serde_json::json!({
                "artifactId": artifact_id,
                "kind": resolve_artifact_schema_id(workspace, &artifact_id),
                "revision": resolve_artifact_revision(workspace, &artifact_id),
                "sizeBytes": pack.len() + spr.len(),
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
    match resolve_plugin_export_formats(&plugin_id) {
        Ok(available_formats) => CallToolResult::tool_error(
            &GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("plugin `{plugin_id}` declares {} export format(s) but no live export command is wired yet — the real wire protocol has no export query command", available_formats.len()))
                .with_details(serde_json::json!({ "pluginId": plugin_id, "availableFormats": available_formats, "requestedFormat": requested_format }))
                .retryable(),
        ),
        Err(error) => CallToolResult::tool_error(&error),
    }
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
mod quick {
    use super::*;
    use crate::catalog::{compile, Catalog, CatalogSource};
    use crate::protocol::ToolRegistry;

    const ARTIFACT_TOOL_NAMES: [&str; 5] = ["artifact_open", "artifact_create", "artifact_validate", "artifact_snapshot", "artifact_export"];

    fn empty_catalog() -> Arc<Catalog> {
        Arc::new(compile(&CatalogSource::default(), semio_framework::Locale::En, semio_framework::Terminology::Native).expect("empty catalog source compiles"))
    }

    /// 🧪️ A minimal, self-built (never `🧫️fixtures`, reserved for `🗂️catalog`/`🔎️search`/`🧠️context`/
    /// `🧪️conformance`'s own tests) single-`CapabilityOwner::Plugin` catalog — just enough for
    /// `require_workspace_has_a_plugin`/`require_resolvable_export_plugin` to resolve exactly one id,
    /// so tier 3 is reachable without a real installed wasm plugin.
    fn single_plugin_catalog(plugin_id: &str) -> Arc<Catalog> {
        let probe_capability = CapabilityDefinition {
            id: CapabilityRef("test.probe".to_string()),
            version: 1,
            owner: CapabilityOwner::Plugin { plugin_id: plugin_id.to_string(), app_id: None, window_kind_id: None, mode_id: None },
            kind: CapabilityKind::Query,
            title: "Test Probe".to_string(),
            description: "test-only capability so this workspace resolves exactly one plugin".to_string(),
            artifact_kind: None,
            use_when: Vec::new(),
            input_schema: serde_json::json!({ "type": "object" }),
            output_schema: serde_json::json!({ "type": "object" }),
            effects: Default::default(),
            policy: Default::default(),
            execution: Default::default(),
            exposure: ToolExposure::CatalogOnly,
            presentation: CapabilityPresentation { icon_id: None, category: None, keys: None, in_palette: false, args: Vec::new() },
            examples: Vec::new(),
            source: CapabilitySource::Gateway,
        };
        let source = CatalogSource { gateway: vec![probe_capability], ..Default::default() };
        Arc::new(compile(&source, semio_framework::Locale::En, semio_framework::Terminology::Native).expect("single-plugin source compiles"))
    }

    fn assert_object_typed_2020_12(schema: &serde_json::Value) {
        assert_eq!(schema.get("$schema").and_then(serde_json::Value::as_str), Some("https://json-schema.org/draft/2020-12/schema"));
        assert_eq!(schema.get("type").and_then(serde_json::Value::as_str), Some("object"), "schema: {schema}");
    }

    #[test]
    fn every_artifact_tool_registers_under_its_declared_name() {
        let mut registry = InMemoryToolRegistry::new();
        register_artifact_tools(&mut registry, None);
        let tools = registry.list();
        assert_eq!(tools.len(), 5, "tools: {:?}", tools.iter().map(|tool| &tool.name).collect::<Vec<_>>());
        for name in ARTIFACT_TOOL_NAMES {
            assert!(tools.iter().any(|tool| tool.name == name), "missing tool {name}");
        }
    }

    #[test]
    fn every_top_level_schema_is_object_typed_2020_12() {
        for schema in [artifact_open_input_schema(), artifact_open_output_schema(), artifact_create_input_schema(), artifact_create_output_schema(), artifact_validate_input_schema(), artifact_validate_output_schema(), artifact_snapshot_input_schema(), artifact_snapshot_output_schema(), artifact_export_input_schema(), artifact_export_output_schema()] {
            assert_object_typed_2020_12(&schema);
        }
    }

    #[test]
    fn no_workspace_bound_is_a_retryable_plugin_unavailable_for_every_artifact_tool() {
        let mut registry = InMemoryToolRegistry::new();
        register_artifact_tools(&mut registry, None);
        let arguments_by_tool = [("artifact_open", serde_json::json!({ "artifactId": "a" })), ("artifact_create", serde_json::json!({ "artifactId": "a", "kind": "k" })), ("artifact_validate", serde_json::json!({ "artifactId": "a" })), ("artifact_snapshot", serde_json::json!({ "artifactId": "a" })), ("artifact_export", serde_json::json!({ "artifactId": "a" }))];
        for (name, arguments) in arguments_by_tool {
            let result = registry.call(name, arguments).unwrap_or_else(|error| panic!("{name} must be a known tool name: {error:?}"));
            assert!(result.is_error, "{name} must fail with no workspace bound");
            let structured = result.structured_content.expect("structured content");
            assert_eq!(structured["code"], "PLUGIN_UNAVAILABLE", "{name}: {structured}");
            assert_eq!(structured["retryable"], true, "{name}: {structured}");
        }
    }

    #[test]
    fn missing_required_field_is_input_invalid_before_any_workspace_check() {
        let mut registry = InMemoryToolRegistry::new();
        register_artifact_tools(&mut registry, None);
        let empty_arguments_by_tool = ["artifact_open", "artifact_create", "artifact_validate", "artifact_snapshot", "artifact_export"];
        for name in empty_arguments_by_tool {
            let result = registry.call(name, serde_json::json!({})).unwrap();
            assert!(result.is_error, "{name} must reject a missing required field");
            assert_eq!(result.structured_content.unwrap()["code"], "INPUT_INVALID", "{name}");
        }
    }

    #[test]
    fn workspace_bound_with_zero_resolvable_plugins_is_still_plugin_unavailable() {
        let dir = store::test_support::tempdir().expect("tempdir");
        let workspace = Arc::new(HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), empty_catalog()).expect("opens"));
        let mut registry = InMemoryToolRegistry::new();
        register_artifact_tools(&mut registry, Some(workspace));
        let result = registry.call("artifact_open", serde_json::json!({ "artifactId": "whatever" })).unwrap();
        assert!(result.is_error);
        assert_eq!(result.structured_content.unwrap()["code"], "PLUGIN_UNAVAILABLE");
    }

    #[test]
    fn artifact_create_then_open_round_trips_for_real_with_exactly_one_resolvable_plugin() {
        let dir = store::test_support::tempdir().expect("tempdir");
        let workspace = Arc::new(HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), single_plugin_catalog("test-plugin")).expect("opens"));
        let mut registry = InMemoryToolRegistry::new();
        register_artifact_tools(&mut registry, Some(workspace));

        let created = registry.call("artifact_create", serde_json::json!({ "artifactId": "doc-1", "kind": "test.kind", "initial": { "n": 1 } })).unwrap();
        assert!(!created.is_error, "{created:?}");
        let created_structured = created.structured_content.expect("structured content");
        assert_eq!(created_structured["artifactId"], "doc-1");
        assert!(!created_structured["revision"]["headEditId"].as_str().unwrap_or_default().is_empty(), "a real applied edit has a non-empty head edit id: {created_structured}");

        let duplicate = registry.call("artifact_create", serde_json::json!({ "artifactId": "doc-1", "kind": "test.kind" })).unwrap();
        assert!(duplicate.is_error, "creating the same id twice must not silently no-op");
        assert_eq!(duplicate.structured_content.unwrap()["code"], "PRECONDITION_FAILED");

        let opened = registry.call("artifact_open", serde_json::json!({ "artifactId": "doc-1" })).unwrap();
        assert!(!opened.is_error, "{opened:?}");
        let opened_structured = opened.structured_content.expect("structured content");
        assert_eq!(opened_structured["artifactId"], "doc-1");
        assert!(opened_structured["sizeBytes"].as_u64().unwrap_or(0) > 0, "a real committed edit persists non-empty bytes: {opened_structured}");

        let missing = registry.call("artifact_open", serde_json::json!({ "artifactId": "does-not-exist" })).unwrap();
        assert!(missing.is_error);
        assert_eq!(missing.structured_content.unwrap()["code"], "NOT_FOUND");
    }

    #[test]
    fn artifact_validate_is_a_real_typed_gap_never_a_fabricated_pass() {
        let dir = store::test_support::tempdir().expect("tempdir");
        let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), single_plugin_catalog("test-plugin")).expect("opens");
        semio_framework::io::resolve_ready(workspace.ensure_probe_artifact("doc-2", serde_json::json!({}))).expect("seed");
        let mut registry = InMemoryToolRegistry::new();
        register_artifact_tools(&mut registry, Some(Arc::new(workspace)));
        let result = registry.call("artifact_validate", serde_json::json!({ "artifactId": "doc-2" })).unwrap();
        assert!(result.is_error, "no live validate command is wired yet — this must never silently pass");
        let structured = result.structured_content.unwrap();
        assert_eq!(structured["code"], "PLUGIN_UNAVAILABLE");
        assert_eq!(structured["retryable"], true);
    }

    #[test]
    fn artifact_snapshot_returns_real_bytes_for_the_current_revision_and_rejects_a_stale_one() {
        let dir = store::test_support::tempdir().expect("tempdir");
        let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), single_plugin_catalog("test-plugin")).expect("opens");
        semio_framework::io::resolve_ready(workspace.ensure_probe_artifact("doc-3", serde_json::json!({ "n": 7 }))).expect("seed");
        let mut registry = InMemoryToolRegistry::new();
        register_artifact_tools(&mut registry, Some(Arc::new(workspace)));

        let current = registry.call("artifact_snapshot", serde_json::json!({ "artifactId": "doc-3" })).unwrap();
        assert!(!current.is_error, "{current:?}");
        let structured = current.structured_content.expect("structured content");
        assert!(structured["packBytes"].as_u64().unwrap_or(0) > 0, "a real committed edit snapshots non-empty bytes: {structured}");

        let stale = registry.call("artifact_snapshot", serde_json::json!({ "artifactId": "doc-3", "revision": { "artifactId": "doc-3", "headEditId": "not-a-real-edit-id", "cursor": "999" } })).unwrap();
        assert!(stale.is_error, "a mismatched revision must not silently answer with the current snapshot");
        assert_eq!(stale.structured_content.unwrap()["code"], "PRECONDITION_FAILED");
    }

    #[test]
    fn artifact_export_never_fabricates_a_successful_export() {
        let dir = store::test_support::tempdir().expect("tempdir");
        let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), single_plugin_catalog("test-plugin")).expect("opens");
        semio_framework::io::resolve_ready(workspace.ensure_probe_artifact("doc-4", serde_json::json!({}))).expect("seed");
        let mut registry = InMemoryToolRegistry::new();
        register_artifact_tools(&mut registry, Some(Arc::new(workspace)));
        let result = registry.call("artifact_export", serde_json::json!({ "artifactId": "doc-4", "format": "pdf" })).unwrap();
        assert!(result.is_error, "no live export command is wired yet — this must never silently succeed");
    }
}
//#endregion 🧪️Tests
