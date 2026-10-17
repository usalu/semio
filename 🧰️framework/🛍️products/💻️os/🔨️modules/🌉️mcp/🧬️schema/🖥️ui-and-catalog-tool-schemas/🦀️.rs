//! 🖥️ UI and catalog tool schemas of the os.mcp registry: the focus, reveal, conversation and job shapes and the capability action, dialog and artifact create template shapes. See [`super`].

use super::*;

//#region 🔖️ToolSchemas
//#region 🔖️UiToolSchemas
pub fn ui_focus_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "windowId": { "type": "string" } }, "additionalProperties": false })
}

pub fn ui_focus_input_schema() -> serde_json::Value {
    wire("ui.focus", "input", ui_focus_input_shape())
}

pub fn ui_focus_output_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "ok": { "type": "boolean" }, "windowId": {} } })
}

pub fn ui_focus_output_schema() -> serde_json::Value {
    wire("ui.focus", "output", ui_focus_output_shape())
}

pub fn ui_reveal_input_shape() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": { "anchor": { "type": "string", "enum": ["left", "right", "top", "bottom"] }, "path": { "type": "array", "items": { "type": "string" } } },
        "required": ["anchor", "path"],
        "additionalProperties": false,
    })
}

pub fn ui_reveal_input_schema() -> serde_json::Value {
    wire("ui.reveal", "input", ui_reveal_input_shape())
}

pub fn ui_reveal_output_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "ok": { "type": "boolean" }, "anchor": { "type": "string" }, "path": { "type": "array", "items": { "type": "string" } } } })
}

pub fn ui_reveal_output_schema() -> serde_json::Value {
    wire("ui.reveal", "output", ui_reveal_output_shape())
}

/// 📐️ `conversation.reply` — the agent's own free-text turn. `text` is the only required field:
/// a client that streams passes the same `replyId` on every chunk and `complete: false` until the
/// last one, and a client that answers in one shot passes neither.
pub fn conversation_reply_input_shape() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": {
            "text": { "type": "string" },
            "replyId": { "type": "string" },
            "inReplyTo": { "type": "string" },
            "complete": { "type": "boolean" },
        },
        "required": ["text"],
        "additionalProperties": false,
    })
}

pub fn conversation_reply_input_schema() -> serde_json::Value {
    wire("conversation.reply", "input", conversation_reply_input_shape())
}

/// 📐️ `shells` is how many attached shells the chunk actually reached — `0` is an honest answer
/// (nobody is looking) rather than a failure, and the agent can stop narrating to an empty room.
pub fn conversation_reply_output_shape() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": { "ok": { "type": "boolean" }, "replyId": { "type": "string" }, "complete": { "type": "boolean" }, "shells": { "type": "integer" } },
    })
}

pub fn conversation_reply_output_schema() -> serde_json::Value {
    wire("conversation.reply", "output", conversation_reply_output_shape())
}

pub fn job_get_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "jobId": { "type": "string" } }, "required": ["jobId"], "additionalProperties": false })
}

pub fn job_get_input_schema() -> serde_json::Value {
    wire("job.get", "input", job_get_input_shape())
}

pub fn job_cancel_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": { "jobId": { "type": "string" } }, "required": ["jobId"], "additionalProperties": false })
}

pub fn job_cancel_input_schema() -> serde_json::Value {
    wire("job.cancel", "input", job_cancel_input_shape())
}

/// 📐️ `job.get` and `job.cancel` answer the SAME snapshot — one shape, stamped with whichever
/// capability id asked for it.
pub fn job_snapshot_output_shape() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": {
            "jobId": { "type": "string" },
            "kind": { "type": "string" },
            "status": { "type": "string" },
            "progress": {},
            "message": {},
            "result": {},
            "error": {},
            "cancelRequested": { "type": "boolean" },
        },
    })
}

pub fn job_snapshot_output_schema(capability_id: &str) -> serde_json::Value {
    wire(capability_id, "output", job_snapshot_output_shape())
}
//#endregion 🔖️UiToolSchemas

//#region 🔖️CatalogToolSchemas
/// 📐️ `📋️master.md` §3.2 step 1's input envelope — `{type:"object", properties, required,
/// additionalProperties:false}`. The catalogued shape is the zero-argument member of the family
/// (`additionalProperties:false` with nothing declared accepts exactly `{}`, which IS the contract of
/// an argument-less action); [`capability_action_input_schema`] folds one capability's declared
/// `ActionArgDef` leaves in.
pub fn capability_action_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object", "properties": {}, "additionalProperties": false })
}

pub fn capability_action_input_schema(capability_id: &str, properties: serde_json::Map<String, serde_json::Value>, required: Vec<String>) -> serde_json::Value {
    let mut schema = capability_action_input_shape();
    let map = schema.as_object_mut().expect("object schema");
    map.insert("properties".to_string(), serde_json::Value::Object(properties));
    if !required.is_empty() {
        map.insert("required".to_string(), serde_json::Value::Array(required.into_iter().map(serde_json::Value::String).collect()));
    }
    wire(capability_id, "input", schema)
}

/// 📐️ The permissive input envelope a descriptor-derived catalog row publishes before its arguments
/// are typed.
pub fn capability_generic_input_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object" })
}

pub fn capability_generic_input_schema(capability_id: &str) -> serde_json::Value {
    wire(capability_id, "input", capability_generic_input_shape())
}

/// 📐️ No `manifest::ActionDefinition` carries a typed output shape yet (the bridge's
/// `AppFrame::Emit`/`DispatchReport` payload is dynamic), so every capability's `output_schema` is
/// this envelope tagged with its own `$id` until a later packet types individual results.
pub fn capability_generic_output_shape() -> serde_json::Value {
    serde_json::json!({ "type": "object" })
}

pub fn capability_generic_output_schema(capability_id: &str) -> serde_json::Value {
    wire(capability_id, "output", capability_generic_output_shape())
}

/// 📐️ `📋️master.md` §3.2: "`dialogs` → `ui.dialog.open`". The catalogued shape is the real value
/// space (any declared dialog id, plus an opaque argument bag);
/// [`ui_dialog_open_input_schema`] narrows `dialogId` to the ids one compiled workspace actually
/// declares.
pub fn ui_dialog_open_input_shape() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": { "dialogId": { "type": "string", "minLength": 1 }, "args": { "type": "object" } },
        "required": ["dialogId"],
        "additionalProperties": false,
    })
}

pub fn ui_dialog_open_input_schema(dialog_ids: &[String]) -> serde_json::Value {
    let mut schema = ui_dialog_open_input_shape();
    schema["properties"]["dialogId"] = serde_json::json!({ "type": "string", "enum": dialog_ids.iter().map(|id| serde_json::Value::String(id.clone())).collect::<Vec<_>>() });
    wire("ui.dialog.open", "input", schema)
}

/// 📐️ `📋️master.md` §3.2: "`examples` → the `artifact.create` template enum". The catalogued shape
/// is the real value space (`kind` plus any `<plugin_id>:<example_id>` template);
/// [`artifact_create_template_input_schema`] narrows `template` to the templates one compiled
/// workspace actually declares.
pub fn artifact_create_template_input_shape() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": { "kind": { "type": "string" }, "template": { "type": "string", "minLength": 1 } },
        "required": ["kind"],
        "additionalProperties": false,
    })
}

pub fn artifact_create_template_input_schema(template_ids: &[String]) -> serde_json::Value {
    let mut schema = artifact_create_template_input_shape();
    schema["properties"]["template"] = artifact_create_template_property(template_ids);
    wire("artifact.create", "input", schema)
}

/// 🌱️ The enumerated `template` argument every declared `ExampleDefinition` in the installed
/// registry folds into — `<plugin_id>:<example_id>`, `📋️master.md` §3.2's "`examples` → the
/// `artifact.create` template enum".
pub fn artifact_create_template_property(template_ids: &[String]) -> serde_json::Value {
    serde_json::json!({ "type": "string", "enum": template_ids.iter().map(|id| serde_json::Value::String(id.clone())).collect::<Vec<_>>() })
}

/// 🌱️ Grafts [`artifact_create_template_property`] onto an existing `artifact.create` input schema,
/// so the ONE canonical `artifact.create` capability (the `artifact_create` tool's own, with its
/// real `artifactId`/`kind`/`initial` arguments) carries the declared template enum instead of a
/// second capability claiming the same id with a different shape.
pub fn with_artifact_create_templates(mut schema: serde_json::Value, template_ids: &[String]) -> serde_json::Value {
    if let Some(properties) = schema.get_mut("properties").and_then(serde_json::Value::as_object_mut) {
        properties.insert("template".to_string(), artifact_create_template_property(template_ids));
    }
    schema
}
//#endregion 🔖️CatalogToolSchemas
//#endregion 🔖️ToolSchemas
