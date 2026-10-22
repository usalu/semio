//! 🧭️ Core and mutation tool schemas of the os.mcp registry: the inference job wire limits, the schema dialect, the wire stamp and the context, capability, action and transaction shapes. See [`super`].


//#region 💡️InferenceJobWire
pub const INFERENCE_PROGRESS_MAX_CURSOR:u64=9_007_199_254_740_991;

pub const INFERENCE_PROPOSAL_HASH_HEX_LENGTH:usize=64;
//#endregion 💡️InferenceJobWire

//#region 🔖️ToolSchemas
/// 🌐️ The dialect every MCP `inputSchema`/`outputSchema` must declare — both SDKs validate
/// `tools/list` against 2020-12, so this is the ONE place in the crate that writes that URI.
pub const MCP_SCHEMA_DIALECT: &str = "https://json-schema.org/draft/2020-12/schema";

/// 🏷️ Stamps the MCP dialect plus the `semio://capability/{id}/{input|output}` `$id` onto one
/// catalogued shape. The shapes below stay `$id`-free so [`schemas`] can publish them verbatim as
/// `$defs` entries of `🧬️schema/🔣️.json`; only the wire copy a `Tool`/`CapabilityDefinition`
/// carries is stamped.
pub(super) fn wire(capability_id: &str, direction: &str, mut shape: serde_json::Value) -> serde_json::Value {
    if let Some(map) = shape.as_object_mut() {
        map.insert("$schema".to_string(), serde_json::Value::String(MCP_SCHEMA_DIALECT.to_string()));
        map.insert("$id".to_string(), serde_json::Value::String(format!("semio://capability/{capability_id}/{direction}")));
    }
    shape
}

/// 📐️ `RevisionStamp`-shaped but nullable — a sub-schema of `🗿️artifact`'s open/create/snapshot
/// envelopes, never a top-level tool schema, so it carries neither `$schema` nor `$id`.
pub fn nullable_revision_stamp_shape() -> serde_json::Value {
    serde_json::json!({
        "type": ["object", "null"],
        "properties": { "artifactId": { "type": "string" }, "headEditId": { "type": "string" }, "cursor": { "type": "string" } },
    })
}

//#region 🔖️CoreToolSchemas
pub fn capabilities_search_input_shape() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": {
            "query": { "type": "string" },
            "kind": { "type": "array", "items": { "type": "string" } },
            "owner": { "type": "string" },
            "artifactKind": { "type": "string" },
            "requiresScope": { "type": "string" },
            "limit": { "type": "integer", "minimum": 1, "maximum": 100, "default": 20 },
            "cursor": { "type": "string", "description": "The `nextCursor` of a previous page; omit for the first page." },
        },
        "required": ["query"],
        "additionalProperties": false,
    })
}

pub fn capabilities_search_input_schema() -> serde_json::Value {
    wire("capabilities.search", "input", capabilities_search_input_shape())
}

/// 📐️ MCP's `tools/list` requires an `outputSchema` (when present) to describe a JSON OBJECT at the
/// top level — a bare `type:"array"` fails the SDK client's own Zod validation of the `Tool` shape
/// (caught live running `repo run @semio-tech/framework-os-mcp:test -- quick`, not read off the spec).
/// The hits themselves are therefore wrapped under a `results` property.
pub fn capabilities_search_output_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "results": { "type": "array" }, "total": { "type": "integer" }, "nextCursor": { "type": ["string", "null"] } } })
}

pub fn capabilities_search_output_schema() -> serde_json::Value {
    wire("capabilities.search", "output", capabilities_search_output_shape())
}

pub fn capabilities_describe_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "capabilityId": { "type": "string" } }, "required": ["capabilityId"], "additionalProperties": false })
}

pub fn capabilities_describe_input_schema() -> serde_json::Value {
    wire("capabilities.describe", "input", capabilities_describe_input_shape())
}

pub fn capabilities_describe_output_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object" })
}

pub fn capabilities_describe_output_schema() -> serde_json::Value {
    wire("capabilities.describe", "output", capabilities_describe_output_shape())
}

pub fn context_resolve_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "principal": { "type": "string" }, "locale": { "type": "string" } }, "additionalProperties": false })
}

pub fn context_resolve_input_schema() -> serde_json::Value {
    wire("context.resolve", "input", context_resolve_input_shape())
}

pub fn context_resolve_output_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object" })
}

pub fn context_resolve_output_schema() -> serde_json::Value {
    wire("context.resolve", "output", context_resolve_output_shape())
}
//#endregion 🔖️CoreToolSchemas

//#region 🔖️MutationToolSchemas
pub fn action_prepare_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "capabilityId": { "type": "string" }, "input": { "type": "object" } }, "required": ["capabilityId"], "additionalProperties": false })
}

pub fn action_prepare_input_schema() -> serde_json::Value {
    wire("action.prepare", "input", action_prepare_input_shape())
}

pub fn action_invoke_input_shape() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": {
            "preparedActionHandle": { "type": "string" },
            "capabilityId": { "type": "string" },
            "input": { "type": "object" },
            "expectedRevision": { "type": "object" },
            "idempotencyKey": { "type": "string" },
            "approvalHandle": { "type": "string" },
        },
        "additionalProperties": false,
    })
}

pub fn action_invoke_input_schema() -> serde_json::Value {
    wire("action.invoke", "input", action_invoke_input_shape())
}

/// 🎫️ Every handle field the five one-handle tools draw from — `action.cancel`'s
/// `preparedActionHandle`, `transaction.commit`/`transaction.rollback`'s `transactionHandle`, and
/// `history.undo`/`history.redo`'s `undoToken`.
pub const HANDLE_INPUT_FIELDS: &[&str] = &["preparedActionHandle", "transactionHandle", "undoToken"];

/// 🎫️ The whole one-handle tool family as ONE contract: a call carries exactly one of
/// [`HANDLE_INPUT_FIELDS`] and nothing else. Each per-tool document [`handle_input_schema`] stamps is
/// literally one branch of this `oneOf`, so the family and its members cannot drift apart.
pub fn handle_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "oneOf": HANDLE_INPUT_FIELDS.iter().map(|field| handle_input_branch(field)).collect::<Vec<_>>() })
}

/// 🎫️ One closed single-field member of the [`handle_input_shape`] family.
fn handle_input_branch(field: &str) -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { field: { "type": "string" } }, "required": [field], "additionalProperties": false })
}

/// 🎫️ The wire document for the one tool that accepts `field`.
pub fn handle_input_schema(field: &str, capability_id: &str) -> serde_json::Value {
    assert!(HANDLE_INPUT_FIELDS.contains(&field), "`{field}` is not a declared handle field");
    wire(capability_id, "input", handle_input_branch(field))
}

pub fn transaction_begin_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "preparedHandles": { "type": "array", "items": { "type": "string" } } }, "required": ["preparedHandles"], "additionalProperties": false })
}

pub fn transaction_begin_input_schema() -> serde_json::Value {
    wire("transaction.begin", "input", transaction_begin_input_shape())
}
//#endregion 🔖️MutationToolSchemas
//#endregion 🔖️ToolSchemas
