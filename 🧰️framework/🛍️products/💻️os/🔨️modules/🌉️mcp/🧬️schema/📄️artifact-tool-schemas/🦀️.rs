//! 📄️ Artifact tool schemas of the os.mcp registry: the open, create, validate, snapshot and export input and output shapes. See [`super`].

use super::*;

//#region 🔖️ToolSchemas
//#region 🔖️ArtifactToolSchemas
pub fn artifact_open_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "artifactId": { "type": "string" } }, "required": ["artifactId"], "additionalProperties": false })
}

pub fn artifact_open_input_schema() -> serde_json::Value {
    wire("artifact.open", "input", artifact_open_input_shape())
}

/// 📐️ Deliberately NOT the resource's full-body shape (`semio://artifact/{id}` — packBytes/sprBytes/
/// packBase64/sprBase64): `artifact_open` answers identity/kind/revision/size, never the whole body.
pub fn artifact_open_output_shape() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": {
            "artifactId": { "type": "string" },
            "kind": { "type": ["string", "null"] },
            "artifactKind": { "type": ["string", "null"] },
            "revision": nullable_revision_stamp_shape(),
            "sizeBytes": { "type": ["integer", "null"] },
            "sessionDocument": session_document_shape(),
        },
    })
}

/// 🌎️ What `artifact_open` binding a HUB document reports back: which plugin's guest now holds this [Design notes](README.md#source-notes-31).
pub fn session_document_shape() -> serde_json::Value {
    serde_json::json!({
        "type": ["object", "null"],
        "properties": {
            "pluginId": { "type": "string" },
            "appId": { "type": "string" },
            "surfaceId": { "type": ["string", "null"] },
            "packBytes": { "type": "integer" },
            "sprBytes": { "type": "integer" },
            "writePath": { "type": "string" },
            "relayedBatches": { "type": "integer" },
            "sync": {
                "type": "object",
                "description": "What the hub document's own actor last reported: its link, the local mutations the hub has not acknowledged, whether the head is acknowledged, and the last coded fault (e.g. a link that expired).",
                "properties": { "remote": { "type": "string" }, "pendingMutations": { "type": "integer" }, "acknowledged": { "type": "boolean" }, "lastFault": { "type": ["string", "null"] } },
            },
        },
    })
}

pub fn artifact_open_output_schema() -> serde_json::Value {
    wire("artifact.open", "output", artifact_open_output_shape())
}

/// 📐️ `artifact_create`'s DIRECT-tool input (`🗿️artifact`), distinct from
/// [`artifact_create_template_input_shape`] — the catalog-compiled `artifact.create` capability that
/// enumerates every plugin's declared example templates.
pub fn artifact_create_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "artifactId": { "type": "string" }, "kind": { "type": "string" }, "initial": {} }, "required": ["artifactId", "kind"], "additionalProperties": false })
}

pub fn artifact_create_input_schema() -> serde_json::Value {
    wire("artifact.create", "input", artifact_create_input_shape())
}

/// 📐️ `artifact_create` runs as a job (`jobId`, readable with `job_get`, cancellable with
/// `job_cancel` or `notifications/cancelled`); a cancelled create answers `CANCELLED` and creates
/// nothing.
pub fn artifact_create_output_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "jobId": { "type": "string" }, "status": { "enum": ["SUCCEEDED", "CANCELLED"] }, "artifactId": { "type": "string" }, "kind": { "type": "string" }, "pluginId": { "type": "string" }, "appId": { "type": "string" }, "sizeBytes": { "type": "integer", "minimum": 0 }, "revision": nullable_revision_stamp_shape() }, "required": ["jobId", "status", "artifactId", "kind"] })
}

pub fn artifact_create_output_schema() -> serde_json::Value {
    wire("artifact.create", "output", artifact_create_output_shape())
}

pub fn artifact_validate_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "artifactId": { "type": "string" } }, "required": ["artifactId"], "additionalProperties": false })
}

pub fn artifact_validate_input_schema() -> serde_json::Value {
    wire("artifact.validate", "input", artifact_validate_input_shape())
}

/// 🔎️ Owner-codec validation of the complete canonical pair, with untrusted diagnostics.
pub fn artifact_validate_output_shape() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": {
            "artifactId": { "type": "string" }, "schema": { "type": "string" },
            "validation": { "const": "canonical-pair" }, "valid": { "type": "boolean" },
            "jobId": { "type": "string" },
            "untrusted": untrusted_content_shape_with(serde_json::json!({ "type": "object", "properties": { "diagnostics": { "type": "array", "items": { "type": "string" } } }, "required": ["diagnostics"], "additionalProperties": false })),
        },
        "required": ["artifactId", "schema", "validation", "valid", "untrusted"],
        "additionalProperties": false,
    })
}

pub fn artifact_validate_output_schema() -> serde_json::Value {
    wire("artifact.validate", "output", artifact_validate_output_shape())
}

pub fn artifact_snapshot_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "artifactId": { "type": "string" }, "revision": nullable_revision_stamp_shape() }, "required": ["artifactId"], "additionalProperties": false })
}

pub fn artifact_snapshot_input_schema() -> serde_json::Value {
    wire("artifact.snapshot", "input", artifact_snapshot_input_shape())
}

/// 🧊️ Both halves of the document, because one of them can never move. A `.spk` pack is [Design notes](README.md#source-notes-121).
pub fn artifact_snapshot_output_shape() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": { "artifactId": { "type": "string" }, "packBytes": { "type": "integer" }, "sprBytes": { "type": "integer" }, "untrusted": untrusted_content_shape_with(untrusted_artifact_body_shape()) },
        "required": ["artifactId", "packBytes", "sprBytes", "untrusted"],
    })
}

pub fn artifact_snapshot_output_schema() -> serde_json::Value {
    wire("artifact.snapshot", "output", artifact_snapshot_output_shape())
}

pub fn artifact_export_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "artifactId": { "type": "string" }, "format": { "type": "string" } }, "required": ["artifactId"], "additionalProperties": false })
}

pub fn artifact_export_input_schema() -> serde_json::Value {
    wire("artifact.export", "input", artifact_export_input_shape())
}

/// 📐️ The owning app's own media-out bytes for the artifact. They render the document, so they
/// travel only inside `untrusted` (`content.contentBase64`); `contentBytes` is the gateway's count.
pub fn artifact_export_output_shape() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": {
            "artifactId": { "type": "string" },
            "format": { "type": "string" },
            "mimeType": { "type": ["string", "null"] },
            "pluginId": { "type": "string" },
            "contentBytes": { "type": "integer" },
            "descriptorBytes": { "type": "integer" },
            "availablePorts": { "type": "array", "items": { "type": "string" } },
            "declaredExportFormats": { "type": "array", "items": { "type": "string" } },
            "untrusted": untrusted_content_shape_with(serde_json::json!({ "type": "object", "properties": { "contentBase64": { "type": "string" } }, "required": ["contentBase64"], "additionalProperties": false })),
        },
        "required": ["artifactId", "format", "contentBytes", "untrusted"],
    })
}

pub fn artifact_export_output_schema() -> serde_json::Value {
    wire("artifact.export", "output", artifact_export_output_shape())
}
//#endregion 🔖️ArtifactToolSchemas
//#endregion 🔖️ToolSchemas
