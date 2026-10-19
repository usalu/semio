//! 📐️ Tool shapes of the wire crate: JSON-RPC and MCP wire types, registries, the backend port and the notification lane.

use schemars::schema_for;
use semio_framework_os_mcp_foundation::errors::GatewayErrorCode;

//#region 🔖️McpSchemaShape

/// 🧷️ Rewrites every boolean sub-schema into its object equivalent, in place and recursively.
///
/// JSON Schema 2020-12 allows `true`/`false` as complete schemas, and `schemars` emits `true` for a
/// free-form `serde_json::Value` field. The MCP SDKs, however, validate tool `inputSchema`/
/// `outputSchema` with a Zod model that requires every sub-schema to be an **object** — a bare
/// `true` makes the official client reject the whole `tools/list` response with a `$ZodError`
/// (observed against `@modelcontextprotocol/sdk` 1.30.0 on `action_prepare.outputSchema.properties
/// .preview`). `{}` and `{"not": {}}` are the semantically identical object forms, so this narrows
/// the encoding without changing what any schema accepts.
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

/// 🔁️ Converts a `schemars` 0.8 draft-07 document into JSON Schema 2020-12 in place.
///
/// MCP requires tool `inputSchema`/`outputSchema` to be 2020-12, but `schemars` 0.8 (the version
/// this workspace pins) emits draft-07. The two differ, for the struct shapes this crate mirrors,
/// only in where subschema definitions live and how they are referenced — so this is a real
/// conversion of those two things, not a relabelling of the `$schema` URI:
/// `definitions` → `$defs`, and every `#/definitions/X` reference → `#/$defs/X`.
/// Anything already declaring 2020-12 is left untouched.
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

/// 🧯️ Widens one tool's `outputSchema` to `anyOf [its success shape, the typed tool error, the result wrapper]`. The
/// official MCP SDK validates `structuredContent` against `outputSchema` on EVERY result, `isError`
/// ones included (`@modelcontextprotocol/sdk` 1.30.0 `Client.callTool`, measured 2026-09-26: an
/// `APPROVAL_REQUIRED` from `action_invoke` became a thrown `-32602 … must have required property
/// 'affectedResources'`), so a schema that names only the success shape turns every typed error into
/// a client-side exception the agent never sees. The dialect, `$id`, `title` and the `$defs` every
/// `#/$defs/…` reference resolves against stay at the root. Idempotent.
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
