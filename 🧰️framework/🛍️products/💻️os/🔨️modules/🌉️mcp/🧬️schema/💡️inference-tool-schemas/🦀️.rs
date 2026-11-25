//! 💡️ Inference tool schemas of the os.mcp registry: the list, get, submit, job handle, approve, job output and run shapes with the original retained turn schema. See [`super`].

use super::*;

//#region 🔖️ToolSchemas
//#region 🔖️InferenceToolSchemas
pub fn inference_list_input_shape() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": {
            "artifactId": { "type": "string" },
            "inferenceSchema": { "type": "string" },
            "limit": { "type": "integer", "minimum": 1, "maximum": 100, "default": 20 },
            "cursor": { "type": "string", "description": "The `nextCursor` of a previous page; omit for the first page." },
        },
        "additionalProperties": false,
    })
}

pub fn inference_list_input_schema() -> serde_json::Value {
    wire("inference.list", "input", inference_list_input_shape())
}

pub fn inference_list_output_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "artifactId": {}, "artifactKind": {}, "declared": { "type": "array" }, "total": { "type": "integer" }, "nextCursor": { "type": ["string", "null"] } } })
}

pub fn inference_list_output_schema() -> serde_json::Value {
    wire("inference.list", "output", inference_list_output_shape())
}

pub fn inference_get_input_shape() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": { "artifactId": { "type": "string" }, "inferenceSchema": { "type": "string" } },
        "required": ["artifactId", "inferenceSchema"],
        "additionalProperties": false,
    })
}

pub fn inference_get_input_schema() -> serde_json::Value {
    wire("inference.get", "input", inference_get_input_shape())
}

pub fn inference_get_output_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object" })
}

pub fn inference_get_output_schema() -> serde_json::Value {
    wire("inference.get", "output", inference_get_output_shape())
}

/// 📏️ The canonical five-axis requested policy grants no native authority.
pub(super) fn inference_retained_policy_shape()->serde_json::Value{let mut schema:serde_json::Value=serde_json::from_str(include_str!("../🔣️.json")).expect("committed MCP schema is valid");schema["$defs"]["InferenceRetainedPolicy"].take()}

/// 🧷️ Makes the shape self-contained: its policy reference resolves against its own `definitions` bucket, which the mirror hoists.
fn with_retained_policy_definition(mut shape:serde_json::Value)->serde_json::Value{shape["properties"]["retainedPolicy"]=serde_json::json!({"$ref":"#/definitions/InferenceRetainedPolicy"});shape["definitions"]=serde_json::json!({"InferenceRetainedPolicy":inference_retained_policy_shape()});shape}

/// 📌️ Replaces the policy reference with the inline policy and drops the definitions bucket.
fn with_inline_retained_policy(mut input:serde_json::Value)->serde_json::Value{input["properties"]["retainedPolicy"]=inference_retained_policy_shape();if let Some(map)=input.as_object_mut(){map.remove("definitions");}input}

pub fn inference_submit_input_shape() -> serde_json::Value {
    let mut schema:serde_json::Value=serde_json::from_str(include_str!("../🔣️.json")).expect("committed MCP schema is valid");with_retained_policy_definition(schema["$defs"]["InferenceSubmitInput"].take())
}

pub fn inference_submit_input_schema() -> serde_json::Value {
    wire("inference.submit","input",with_inline_retained_policy(inference_submit_input_shape()))
}

/// 📐️ `inference.events`/`inference.cancel` — one session-owned job handle plus a bounded progress
/// cursor.
pub fn inference_job_handle_input_shape() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": { "jobHandle": { "type": "string" }, "after": { "type": "integer", "minimum": 0, "maximum": INFERENCE_PROGRESS_MAX_CURSOR } },
        "required": ["jobHandle"],
        "additionalProperties": false,
    })
}

pub fn inference_job_handle_input_schema(capability_id: &str) -> serde_json::Value {
    wire(capability_id, "input", inference_job_handle_input_shape())
}

pub fn inference_approve_input_shape() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": { "jobHandle": { "type": "string" }, "proposalHash": { "type": "string", "pattern": hex_pattern(INFERENCE_PROPOSAL_HASH_HEX_LENGTH) } },
        "required": ["jobHandle", "proposalHash"],
        "additionalProperties": false,
    })
}

pub fn inference_approve_input_schema() -> serde_json::Value {
    wire("inference.approve", "input", inference_approve_input_shape())
}

pub fn inference_job_output_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object" })
}

pub fn inference_job_output_schema(capability_id: &str) -> serde_json::Value {
    wire(capability_id, "output", inference_job_output_shape())
}

/// 📐️ `inference.run` — the general plugin-declared inference execution route. `payload` is the
/// caller's canonical request body for the plugin's own inference schema; `pluginId`/`artifactId`
/// are optional disambiguators when a kind declares rows from more than one contributor.
pub fn inference_run_input_shape() -> serde_json::Value {
    let mut schema:serde_json::Value=serde_json::from_str(include_str!("../🔣️.json")).expect("committed MCP schema is valid");with_retained_policy_definition(schema["$defs"]["InferenceRunInput"].take())
}

pub fn inference_run_input_schema() -> serde_json::Value {
    wire("inference.run","input",with_inline_retained_policy(inference_run_input_shape()))
}

pub fn inference_run_output_shape() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": {
            "jobId": { "type": "string" },
            "status": { "type": "string" },
            "artifactKind": { "type": "string" },
            "inferenceSchema": { "type": "string" },
            "pluginId": { "type": "string" },
            "cancellationId": { "type": "string" },
            "complete": { "type": "boolean" },
            "payload": {},
            "payloadBytes": { "type": "integer", "minimum": 0 },
        },
        "required": ["jobId", "status", "artifactKind", "inferenceSchema"],
    })
}

pub fn inference_run_output_schema() -> serde_json::Value {
    wire("inference.run", "output", inference_run_output_shape())
}
//#endregion 🔖️InferenceToolSchemas
//#endregion 🔖️ToolSchemas
