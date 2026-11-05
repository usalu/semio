//! 💡️ Inference tool schemas of the os.mcp registry: the list, get, submit, job handle, approve, job output and run shapes with the original retained turn schema. See [`super`].

use super::*;

//#region 🔖️ToolSchemas
//#region 🔖️InferenceToolSchemas
pub fn inference_list_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "artifactId": { "type": "string" } }, "additionalProperties": false })
}

pub fn inference_list_input_schema() -> serde_json::Value {
    wire("inference.list", "input", inference_list_input_shape())
}

pub fn inference_list_output_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "artifactId": {}, "artifactKind": {}, "declared": { "type": "array" } } })
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

pub fn inference_submit_input_shape() -> serde_json::Value {
    let mut schema:serde_json::Value=serde_json::from_str(include_str!("../🔣️.json")).expect("committed MCP schema is valid");schema["$defs"]["InferenceSubmitInput"].take()
}

pub fn inference_submit_input_schema() -> serde_json::Value {
    let mut input=inference_submit_input_shape();input["properties"]["retainedPolicy"]=inference_retained_policy_shape();wire("inference.submit","input",input)
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
    let mut schema:serde_json::Value=serde_json::from_str(include_str!("../🔣️.json")).expect("committed MCP schema is valid");schema["$defs"]["InferenceRunInput"].take()
}

pub fn inference_run_input_schema() -> serde_json::Value {
    let mut input=inference_run_input_shape();input["properties"]["retainedPolicy"]=inference_retained_policy_shape();wire("inference.run","input",input)
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
