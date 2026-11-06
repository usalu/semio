//! 📐️ Tool shapes of the wire crate: JSON-RPC and MCP wire types, registries, the backend port and the notification lane.

use schemars::schema_for;
use semio_os_mcp_foundation::errors::GatewayErrorCode;

//#region 🔖️McpSchemaShape

/// 🧷️ Rewrites every boolean sub-schema into its object equivalent, in place and recursively. [Design notes](README.md#source-notes-8).
pub fn normalize_boolean_subschemas(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Bool(true) => *value = serde_json::json!({}),
        serde_json::Value::Bool(false) => *value = serde_json::json!({ "not": {} }),
        serde_json::Value::Object(map) => {
            for (key, entry) in map.iter_mut() {
                if SCHEMA_BOOLEAN_KEYWORDS.contains(&key.as_str()) {
                    continue;
                }
                normalize_boolean_subschemas(entry);
            }
        }
        serde_json::Value::Array(items) => {
            for item in items.iter_mut() {
                normalize_boolean_subschemas(item);
            }
        }
        _ => {}
    }
}

/// 🔑️ Keywords whose value is a genuine boolean **flag**, not a sub-schema — rewriting these would
/// corrupt the schema rather than normalize it.
pub const SCHEMA_BOOLEAN_KEYWORDS: &[&str] = &["additionalProperties", "unevaluatedProperties", "additionalItems", "unevaluatedItems", "readOnly", "writeOnly", "deprecated", "uniqueItems", "exclusiveMinimum", "exclusiveMaximum"];

/// 🔁️ Converts a `schemars` 0.8 draft-07 document into JSON Schema 2020-12 in place. [Design notes](README.md#source-notes-42).
pub fn convert_draft07_to_2020_12(value: &mut serde_json::Value) {
    const DRAFT_07: &str = "http://json-schema.org/draft-07/schema#";
    const DIALECT_2020_12: &str = "https://json-schema.org/draft/2020-12/schema";
    let is_draft_07 = value.get("$schema").and_then(serde_json::Value::as_str) == Some(DRAFT_07);
    if !is_draft_07 {
        return;
    }
    if let Some(map) = value.as_object_mut() {
        map.insert("$schema".to_string(), serde_json::Value::String(DIALECT_2020_12.to_string()));
        if let Some(definitions) = map.remove("definitions") {
            map.insert("$defs".to_string(), definitions);
        }
    }
    rewrite_definition_refs(value);
}

/// 🔗️ Repoints every `$ref` from draft-07's `#/definitions/` to 2020-12's `#/$defs/`.
fn rewrite_definition_refs(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            if let Some(serde_json::Value::String(reference)) = map.get_mut("$ref") {
                if let Some(name) = reference.strip_prefix("#/definitions/") {
                    *reference = format!("#/$defs/{name}");
                }
            }
            for entry in map.values_mut() {
                rewrite_definition_refs(entry);
            }
        }
        serde_json::Value::Array(items) => {
            for item in items.iter_mut() {
                rewrite_definition_refs(item);
            }
        }
        _ => {}
    }
}

//#endregion 🔖️McpSchemaShape

//#region 🔖️ToolSchemas

//#region 🔖️ToolErrors

/// ⚠️ `GatewayError` as a self-contained shape (codes inlined from [`GatewayErrorCode`]'s own derived
/// schema): what `structuredContent` IS when a tool answers `isError: true`.
pub fn tool_error_shape() -> serde_json::Value {
    let codes = serde_json::to_value(schema_for!(GatewayErrorCode)).expect("GatewayErrorCode schema")["enum"].clone();
    serde_json::json!({
        "type": "object",
        "description": "A typed tool error: the result's isError is true and this is the gateway's GatewayError.",
        "properties": { "code": { "type": "string", "enum": codes }, "message": { "type": "string" }, "details": {}, "retryable": { "type": "boolean" } },
        "required": ["code", "message", "details", "retryable"],
        "additionalProperties": false,
    })
}

/// 📦️ The wrapper `structuredContent` takes when a tool's data is an array or a scalar: MCP requires
/// `structuredContent` to be a JSON object, so `CallToolResult::wire_structured_content` wraps such data as
/// `{"result": <value>}`. Every widened `outputSchema` admits it, so a wrapped result still validates.
pub fn tool_result_wrapper_shape() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "description": "Array or scalar tool data wrapped as an object: the data is the value of `result`.",
        "properties": { "result": {} },
        "required": ["result"],
        "additionalProperties": false,
    })
}

/// 🧯️ Widens one tool's `outputSchema` to `anyOf [its success shape, the typed tool error, the result wrapper]`. The [Design notes](README.md#source-notes-120).
pub fn admit_tool_errors(output_schema: &mut serde_json::Value) {
    let error = tool_error_shape();
    let Some(map) = output_schema.as_object_mut() else { return };
    if map.get("anyOf").and_then(serde_json::Value::as_array).is_some_and(|branches| branches.contains(&error)) {
        return;
    }
    let root: Vec<(String, serde_json::Value)> = ["$schema", "$id", "$defs", "title"].iter().filter_map(|key| map.remove(*key).map(|value| (key.to_string(), value))).collect();
    let success = serde_json::Value::Object(std::mem::take(map));
    map.extend(root);
    map.insert("type".to_string(), serde_json::json!("object"));
    map.insert("anyOf".to_string(), serde_json::json!([success, error, tool_result_wrapper_shape()]));
}

//#endregion 🔖️ToolErrors

//#endregion 🔖️ToolSchemas
